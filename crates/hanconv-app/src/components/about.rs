use gpui::{div, prelude::*, App, IntoElement, ParentElement, RenderOnce, Window};
use gpui_component::description_list::DescriptionList;
use gpui_component::label::Label;
use gpui_component::{gray_900, ActiveTheme, Sizable, StyledExt, WindowExt};

#[derive(IntoElement)]
pub struct About;

impl RenderOnce for About {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .py_3()
            .flex()
            .flex_col()
            .gap_1()
            .items_center()
            .child(Label::new("Hanconv").text_lg().font_semibold())
            .child(
                Label::new(t!("about.Description").to_string())
                    .text_sm()
                    .text_color(cx.theme().description_list_label_foreground)
                    .mt_1()
                    .mb_3(),
            )
            .child(
                DescriptionList::horizontal()
                    .bordered(false)
                    .columns(1)
                    .xsmall()
                    .item(
                        Label::new(t!("about.License"))
                            .text_xs()
                            .text_color(cx.theme().description_list_label_foreground)
                            .into_any_element(),
                        Label::new("MIT").text_xs().into_any_element(),
                        1,
                    )
                    .item(
                        Label::new(t!("about.Version"))
                            .text_xs()
                            .text_color(cx.theme().description_list_label_foreground)
                            .into_any_element(),
                        Label::new(env!("CARGO_PKG_VERSION"))
                            .text_xs()
                            .into_any_element(),
                        1,
                    ),
            )
            .child(
                Label::new("Copyright © 2022 fhluo")
                    .text_color(gray_900())
                    .text_xs()
                    .mt_3(),
            )
    }
}

pub fn open_about_dialog(window: &mut Window, cx: &mut App) {
    window.open_dialog(cx, |dialog, _, _| {
        dialog.alert().title(t!("About").to_string()).child(About)
    });
}
