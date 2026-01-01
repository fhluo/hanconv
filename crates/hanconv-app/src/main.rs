#[macro_use]
extern crate rust_i18n;

use gpui::prelude::*;
use gpui::{div, px, size, Application, Bounds, Entity, Window, WindowBounds, WindowOptions};
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::{Root, TitleBar};
use icu_locale::{Locale, LocaleExpander};
use icu_locale::fallback::{LocaleFallbackConfig, LocaleFallbackPriority};
use icu_locale::{DataLocale, Locale, LocaleFallbacker};
use rust_i18n::set_locale;

i18n!("locales", fallback = "en");

struct Hanconv {
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
            |view, state, event, window, cx| match event {
                InputEvent::Change => {
                    let content = state.read(cx).value();
                    let result = hanconv::s2t(content);

                    view.output_editor.update(cx, |state, cx| {
                        state.set_value(result, window, cx);
                    });
                }
                _ => {}
            },
        )
        .detach();

        Hanconv {
            input_editor,
            output_editor,
        }
    }
}

impl Render for Hanconv {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(TitleBar::new().child("Hanconv"))
            .child(
                div()
                    .w_full()
                    .h_full()
                    .px_6()
                    .py_6()
                    .gap_3()
                    .flex()
                    .flex_row()
                    .child(Input::new(&self.input_editor).flex_1())
                    .child(Input::new(&self.output_editor).flex_1()),
            )
    }
}

fn get_app_locale() -> Option<String> {
    let mut fallback_iter = LocaleFallbacker::new()
        .for_config({
            let mut config = LocaleFallbackConfig::default();
            config.priority = LocaleFallbackPriority::Language;
            config
        })
        .fallback_for(sys_locale::get_locale()?.parse::<Locale>().ok()?.into());

    let locales = available_locales!()
        .into_iter()
        .filter_map(|locale| locale.parse::<Locale>().map(DataLocale::from).ok())
        .collect::<Vec<_>>();

    loop {
        let locale = fallback_iter.get();
        if locale.is_unknown() {
            break None;
        }

        if locales.contains(locale) {
            break Some(locale.to_string());
        }

        fallback_iter.step();
    }
}

fn main() {
    let locale = get_app_locale();

    set_locale(locale.as_deref().unwrap_or("en"));

    let app = Application::new().with_assets(gpui_component_assets::Assets);

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
    })
}
