use gpui::prelude::*;
use gpui::{div, px, size, Application, Bounds, Window, WindowBounds, WindowOptions};
use gpui_component::{Root, TitleBar};

struct Hanconv {}

impl Render for Hanconv {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child(TitleBar::new().child("Hanconv"))
    }
}

fn main() {
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
                    let view = cx.new(|_| Hanconv {});

                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    })
}
