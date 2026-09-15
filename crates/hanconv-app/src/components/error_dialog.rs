use gpui_kit::component::description_list::DescriptionList;
use gpui_kit::component::label::Label;
use gpui_kit::component::link::Link;
use gpui_kit::component::{ActiveTheme, Icon, IconName, StyledExt, WindowExt};
use gpui_kit::{div, prelude::*, App, IntoElement, RenderOnce, SharedString, Window};
use std::path::PathBuf;

#[derive(IntoElement)]
struct Title(SharedString);

impl RenderOnce for Title {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .gap_3()
            .items_center()
            .child(
                Icon::new(IconName::CircleX)
                    .text_color(cx.theme().red)
                    .size_6(),
            )
            .child(Label::new(self.0).font_semibold().text_lg())
    }
}

#[derive(Debug, Clone, IntoElement)]
struct IOError {
    path: PathBuf,
    message: String,
}

impl RenderOnce for IOError {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div().child(
            DescriptionList::vertical()
                .columns(1)
                .bordered(false)
                .item(
                    t!("Path").to_string(),
                    Link::new("path")
                        .text_sm()
                        .child(self.path.display().to_string())
                        .on_click(move |_, _, cx| cx.reveal_path(self.path.clone().as_path()))
                        .into_any_element(),
                    1,
                )
                .item(
                    t!("Error").to_string(),
                    Label::new(self.message).text_sm().into_any_element(),
                    1,
                ),
        )
    }
}

pub fn open_io_error_dialog(
    path: impl Into<PathBuf>,
    err_message: impl Into<String>,
    window: &mut Window,
    cx: &mut App,
) {
    let err = IOError {
        path: path.into(),
        message: err_message.into(),
    };

    window.open_alert_dialog(cx, move |alert, _, _| {
        alert
            .title(Title(t!("error.read-file").into()))
            .child(err.clone())
    });
}
