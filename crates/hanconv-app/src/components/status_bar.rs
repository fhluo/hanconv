use crate::components::ConversionSelector;
use crate::conversion::Conversion;
use gpui::prelude::*;
use gpui::{div, App, IntoElement, RenderOnce, Window};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::{gray_600, ActiveTheme, Sizable};

#[derive(IntoElement)]
pub struct StatusBar {
    conversion: Conversion,
}

impl StatusBar {
    pub fn new(conversion: Conversion) -> Self {
        StatusBar { conversion }
    }
}

impl RenderOnce for StatusBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let conversion_selector = ConversionSelector::new(
            Button::new("conversion-button")
                .label(t!(self.conversion.title()))
                .xsmall()
                .ghost()
                .ml_auto()
                .text_color(gray_600()),
            self.conversion,
        );

        div()
            .h_8()
            .w_full()
            .flex()
            .flex_row()
            .items_center()
            .px_3()
            .bg(cx.theme().title_bar)
            .border_t_1()
            .border_color(cx.theme().title_bar_border)
            .child(
                div()
                    .ml_auto()
                    .flex()
                    .flex_row()
                    .gap_1()
                    .items_center()
                    .child(conversion_selector),
            )
    }
}
