#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate rust_i18n;

mod assets;
mod components;
mod config;
mod conversion;

use crate::assets::Assets;
use crate::components::{ConversionSelector, LanguageSelector};
use crate::config::Config;
use crate::conversion::Conversion;
use gpui::prelude::*;
use gpui::{
    div, px, size, Application, Bounds, Entity, StyleRefinement, Window, WindowBounds,
    WindowOptions,
};
use gpui_component::button::Button;
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::{gray_500, ActiveTheme, Icon, Root, Sizable, StyledExt, TitleBar};
use icu_locale::Locale;
use rust_i18n::set_locale;

i18n!("locales", fallback = "en");

struct Hanconv {
    config: Config,

    input_editor: Entity<InputState>,
    output_editor: Entity<InputState>,
}

impl Hanconv {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_editor = cx.new(|cx| InputState::new(window, cx).multi_line(true));
        let output_editor = cx.new(|cx| InputState::new(window, cx).multi_line(true));

        cx.subscribe_in(
            &input_editor,
            window,
            |view, _, event, window, cx| match event {
                InputEvent::Change => {
                    let conversion = view.config.conversion;
                    view.run_conversion(&conversion, window, cx);
                }
                _ => {}
            },
        )
        .detach();

        cx.spawn(async |view, cx| {
            view.update(cx, |view, _| {
                view.config.init();
            })
        })
        .detach();

        cx.on_release(|view, _| {
            view.config.store();
        })
        .detach();

        Hanconv {
            config: Config::load("hanconv"),
            input_editor,
            output_editor,
        }
    }

    fn run_conversion(
        &mut self,
        conversion: &Conversion,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.config.conversion = *conversion;

        let content = self.input_editor.read(cx).value();
        let result = conversion.run(content);

        self.output_editor.update(cx, |state, cx| {
            state.set_value(result, window, cx);
        });
    }

    fn change_locale(&mut self, locale: &Locale, _: &mut Window, _: &mut Context<Self>) {
        set_locale(&locale.to_string());
        self.config.locale = Some(locale.to_owned())
    }
}

impl Render for Hanconv {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let menu_button_style = StyleRefinement::default()
            .h(gpui_component::TITLE_BAR_HEIGHT)
            .bg(cx.theme().title_bar)
            .border_0()
            .rounded_none();

        let conversion_selector = ConversionSelector::new(
            Button::new("conversion-button")
                .label(t!("conversion"))
                .refine_style(&menu_button_style),
            self.config.conversion,
        );

        let language_selector = LanguageSelector::new(
            Button::new("language-button")
                .icon(
                    Icon::empty()
                        .small()
                        .path("icons/languages.svg")
                        .text_color(gray_500()),
                )
                .tooltip(t!("language"))
                .w(gpui_component::TITLE_BAR_HEIGHT)
                .refine_style(&menu_button_style),
            self.config.locale.clone(),
        )
        .on_change(cx.listener(Self::change_locale));

        div()
            .on_action(cx.listener(Self::run_conversion))
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                TitleBar::new().child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_1()
                        .child(conversion_selector)
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .h_full()
                                .ml_auto()
                                .child(language_selector),
                        ),
                ),
            )
            .child(
                div()
                    .w_full()
                    .h_full()
                    .flex()
                    .flex_row()
                    .child(Input::new(&self.input_editor).flex_1())
                    .child(Input::new(&self.output_editor).flex_1()),
            )
    }
}

fn main() -> anyhow::Result<()> {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(800.), px(600.)), cx);

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitleBar::title_bar_options()),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| Hanconv::new(window, cx));

                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });

    Ok(())
}
