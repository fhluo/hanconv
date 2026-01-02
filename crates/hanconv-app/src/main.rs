#[macro_use]
extern crate rust_i18n;

use gpui::prelude::*;
use gpui::{
    div, px, size, Action, Application, Bounds, Entity, Window, WindowBounds, WindowOptions,
};
use gpui_component::button::Button;
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::menu::DropdownMenu;
use gpui_component::{Root, TitleBar};
use icu_locale::fallback::{LocaleFallbackConfig, LocaleFallbackPriority};
use icu_locale::{locale, DataLocale, Locale, LocaleFallbacker};
use rust_i18n::set_locale;
use schemars::JsonSchema;
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

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Action)]
#[serde(rename_all = "lowercase")]
enum Conversion {
    S2T,
    T2S,
    S2TW,
    TW2S,
    T2TW,
    TW2T,
    S2HK,
    HK2S,
    T2HK,
    HK2T,
    T2JP,
    JP2T,
}

impl Conversion {
    fn name(&self) -> &'static str {
        match self {
            Conversion::S2T => "s2t",
            Conversion::T2S => "t2s",
            Conversion::S2TW => "s2tw",
            Conversion::TW2S => "tw2s",
            Conversion::T2TW => "t2tw",
            Conversion::TW2T => "tw2t",
            Conversion::S2HK => "s2hk",
            Conversion::HK2S => "hk2s",
            Conversion::T2HK => "t2hk",
            Conversion::HK2T => "hk2t",
            Conversion::T2JP => "t2jp",
            Conversion::JP2T => "jp2t",
        }
    }

    fn title(&self) -> String {
        format!(
            "{} → {}",
            t!(format!("{}.source", self.name())),
            t!(format!("{}.target", self.name()))
        )
    }

    fn run(&self, content: impl AsRef<str>) -> String {
        match self {
            Conversion::S2T => hanconv::s2t(content),
            Conversion::T2S => hanconv::t2s(content),
            Conversion::S2TW => hanconv::s2tw(content),
            Conversion::TW2S => hanconv::tw2s(content),
            Conversion::T2TW => hanconv::t2tw(content),
            Conversion::TW2T => hanconv::tw2t(content),
            Conversion::S2HK => hanconv::s2hk(content),
            Conversion::HK2S => hanconv::hk2s(content),
            Conversion::T2HK => hanconv::t2hk(content),
            Conversion::HK2T => hanconv::hk2t(content),
            Conversion::T2JP => hanconv::t2jp(content),
            Conversion::JP2T => hanconv::jp2t(content),
        }
    }

    #[allow(dead_code)]
    fn all() -> &'static [Conversion] {
        &[
            Conversion::S2T,
            Conversion::T2S,
            Conversion::S2TW,
            Conversion::TW2S,
            Conversion::T2TW,
            Conversion::TW2T,
            Conversion::S2HK,
            Conversion::HK2S,
            Conversion::T2HK,
            Conversion::HK2T,
            Conversion::T2JP,
            Conversion::JP2T,
        ]
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
                        Button::new("conversion-menu-button")
                            .label(t!("conversion"))
                            .dropdown_menu({
                                let conversion = self.config.conversion.clone();
                                move |menu, _, _| {
                                    menu.menu_with_check(
                                        Conversion::S2T.title(),
                                        conversion == Conversion::S2T,
                                        Box::new(Conversion::S2T),
                                    )
                                    .menu_with_check(
                                        Conversion::T2S.title(),
                                        conversion == Conversion::T2S,
                                        Box::new(Conversion::T2S),
                                    )
                                    .separator()
                                    .menu_with_check(
                                        Conversion::S2TW.title(),
                                        conversion == Conversion::S2TW,
                                        Box::new(Conversion::S2TW),
                                    )
                                    .menu_with_check(
                                        Conversion::TW2S.title(),
                                        conversion == Conversion::TW2S,
                                        Box::new(Conversion::TW2S),
                                    )
                                    .separator()
                                    .menu_with_check(
                                        Conversion::T2TW.title(),
                                        conversion == Conversion::T2TW,
                                        Box::new(Conversion::T2TW),
                                    )
                                    .menu_with_check(
                                        Conversion::TW2T.title(),
                                        conversion == Conversion::TW2T,
                                        Box::new(Conversion::TW2T),
                                    )
                                    .separator()
                                    .menu_with_check(
                                        Conversion::S2HK.title(),
                                        conversion == Conversion::S2HK,
                                        Box::new(Conversion::S2HK),
                                    )
                                    .menu_with_check(
                                        Conversion::HK2S.title(),
                                        conversion == Conversion::HK2S,
                                        Box::new(Conversion::HK2S),
                                    )
                                    .separator()
                                    .menu_with_check(
                                        Conversion::T2HK.title(),
                                        conversion == Conversion::T2HK,
                                        Box::new(Conversion::T2HK),
                                    )
                                    .menu_with_check(
                                        Conversion::HK2T.title(),
                                        conversion == Conversion::HK2T,
                                        Box::new(Conversion::HK2T),
                                    )
                                    .separator()
                                    .menu_with_check(
                                        Conversion::T2JP.title(),
                                        conversion == Conversion::T2JP,
                                        Box::new(Conversion::T2JP),
                                    )
                                    .menu_with_check(
                                        Conversion::JP2T.title(),
                                        conversion == Conversion::JP2T,
                                        Box::new(Conversion::JP2T),
                                    )
                                }
                            }),
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
