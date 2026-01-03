#[macro_use]
extern crate rust_i18n;

mod components;
mod config;
mod conversion;

use crate::components::conversion_menu::ConversionMenu;
use crate::config::Config;
use crate::conversion::Conversion;
use gpui::prelude::*;
use gpui::{div, px, size, Application, Bounds, Entity, Window, WindowBounds, WindowOptions};
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::{Root, TitleBar};

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
