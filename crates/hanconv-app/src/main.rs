#[macro_use]
extern crate rust_i18n;

mod components;
mod conversion;

use crate::components::conversion_menu::ConversionMenu;
use crate::conversion::Conversion;
use gpui::prelude::*;
use gpui::{div, px, size, Application, Bounds, Entity, Window, WindowBounds, WindowOptions};
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::{Root, TitleBar};
use icu_locale::fallback::{LocaleFallbackConfig, LocaleFallbackPriority};
use icu_locale::{locale, DataLocale, Locale, LocaleFallbacker};
use rust_i18n::set_locale;
use serde::{Deserialize, Serialize};

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
                    view.conv(&conversion, window, cx);
                }
                _ => {}
            },
        )
        .detach();

        cx.spawn(async |view, cx| {
            view.update(cx, |view, _| {
                view.init_locale();
            })
        })
        .detach();

        cx.on_release(|view, _| {
            let config = view.config.clone();
            if let Err(err) = confy::store("hanconv", None, config) {
                eprintln!("{err}")
            }
        })
        .detach();

        Hanconv {
            config: confy::load::<Config>("hanconv", None).unwrap_or_default(),
            input_editor,
            output_editor,
        }
    }

    fn conv(&mut self, conversion: &Conversion, window: &mut Window, cx: &mut Context<Self>) {
        self.config.conversion = *conversion;

        let content = self.input_editor.read(cx).value();
        let result = conversion.run(content);

        self.output_editor.update(cx, |state, cx| {
            state.set_value(result, window, cx);
        });
    }
}

impl Render for Hanconv {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .on_action(cx.listener(Self::conv))
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                TitleBar::new().child(
                    div().flex().flex_row().child(
                        ConversionMenu::new("conversion-menu-button", self.config.conversion)
                            .label(t!("conversion")),
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    locale: Option<Locale>,
    conversion: Conversion,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            locale: None,
            conversion: Conversion::S2T,
        }
    }
}

impl Hanconv {
    fn init_locale(&mut self) {
        let mut fallback_iter = LocaleFallbacker::new()
            .for_config({
                let mut config = LocaleFallbackConfig::default();
                config.priority = LocaleFallbackPriority::Language;
                config
            })
            .fallback_for({
                if let Some(locale) = self.config.locale.clone() {
                    locale.into()
                } else {
                    sys_locale::get_locale()
                        .unwrap_or_else(|| "en".to_string())
                        .parse::<Locale>()
                        .ok()
                        .unwrap_or_else(|| locale!("en"))
                        .into()
                }
            });

        let locales = available_locales!()
            .into_iter()
            .filter_map(|locale| locale.parse::<Locale>().map(DataLocale::from).ok())
            .collect::<Vec<_>>();

        let locale = loop {
            let locale = fallback_iter.get();
            if locale.is_unknown() {
                break locale!("en");
            }

            if locales.contains(locale) {
                break locale.into_locale();
            }

            fallback_iter.step();
        };

        set_locale(locale.to_string().as_str());
        self.config.locale = Some(locale);
    }
}

fn main() -> anyhow::Result<()> {
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
    });

    Ok(())
}
