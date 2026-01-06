#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate rust_i18n;

mod assets;
mod components;
mod config;
mod conversion;

use crate::assets::Assets;
use crate::components::LanguageSelector;
use crate::config::Config;
use crate::conversion::Conversion;
use gpui::prelude::*;
use gpui::{
    div, px, size, App, Application, Bounds, Entity, Menu, MenuItem,
    StyleRefinement, Window, WindowBounds, WindowOptions,
};
use gpui_component::button::Button;
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::menu::AppMenuBar;
use gpui_component::{gray_500, ActiveTheme, Icon, Root, Sizable, StyledExt, TitleBar};
use icu_locale::Locale;
use rust_i18n::set_locale;
use strum::{EnumCount, VariantArray};

i18n!("locales", fallback = "en");

struct Hanconv {
    config: Config,

    menu_bar: Entity<AppMenuBar>,
    input_editor: Entity<InputState>,
    output_editor: Entity<InputState>,
}

impl Hanconv {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_editor = cx.new(|cx| InputState::new(window, cx).multi_line(true));
        let output_editor = cx.new(|cx| InputState::new(window, cx).multi_line(true));

        cx.subscribe_in(&input_editor, window, Self::on_input_event)
            .detach();

        cx.spawn(async |view, cx| view.update(cx, Self::init_config))
            .detach();

        cx.on_release(Self::store_config).detach();

        Hanconv {
            config: Config::load("hanconv"),
            input_editor,
            output_editor,
            menu_bar: AppMenuBar::new(cx),
        }
    }

    fn on_input_event(
        &mut self,
        _: &Entity<InputState>,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(event, InputEvent::Change) {
            self.run_conversion(&self.config.conversion.clone(), window, cx);
        }
    }

    fn init_config(&mut self, cx: &mut Context<Self>) {
        self.config.init();
        self.update_menu_bar(cx);
    }

    fn store_config(&mut self, _: &mut App) {
        self.config.store()
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

        self.update_menu_bar(cx);
    }

    fn change_locale(&mut self, locale: &Locale, _: &mut Window, cx: &mut Context<Self>) {
        set_locale(&locale.to_string());
        self.config.locale = Some(locale.to_owned());

        self.update_menu_bar(cx);
    }

    fn update_menu_bar(&mut self, cx: &mut Context<Self>) {
        let chunks = Conversion::VARIANTS.chunks(2);
        let mut items = Vec::with_capacity(Conversion::COUNT + (Conversion::COUNT - 1) / 2);

        for conversions in chunks {
            items.extend(conversions.iter().cloned().map(|conversion| {
                MenuItem::action(conversion.title(), conversion)
                    .checked(self.config.conversion == conversion)
            }));
            items.push(MenuItem::Separator);
        }

        cx.set_menus(vec![Menu {
            name: t!("conversion").into(),
            items,
        }]);

        self.menu_bar.update(cx, |menu_bar, cx| {
            menu_bar.reload(cx);
        });
    }
}

impl Render for Hanconv {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let menu_button_style = StyleRefinement::default()
            .bg(cx.theme().title_bar)
            .border_0();

        let language_selector = LanguageSelector::new(
            Button::new("language-button")
                .small()
                .icon(
                    Icon::empty()
                        .small()
                        .path("icons/languages.svg")
                        .text_color(gray_500()),
                )
                .tooltip(t!("language"))
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
                TitleBar::new()
                    .items_center()
                    .child(self.menu_bar.clone())
                    .child(
                        div().flex().flex_row().flex_1().child(
                            div()
                                .flex()
                                .flex_row()
                                .h_full()
                                .ml_auto()
                                .mr_3()
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
