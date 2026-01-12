use crate::components::ConversionSelector;
use crate::conversion::Conversion;
use gpui::prelude::*;
use gpui::{div, App, IntoElement, RenderOnce, Window};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::{gray_600, ActiveTheme, Sizable};

#[derive(IntoElement)]
pub struct StatusBar {
    input_graphemes: usize,
    output_graphemes: usize,
    conversion: Conversion,
}

impl StatusBar {
    pub fn new(
        input_graphemes: usize,
        output_graphemes: usize,
        conversion: Conversion,
    ) -> Self {
        StatusBar {
            input_graphemes,
            output_graphemes,
            conversion,
        }
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
            .child(div().flex().flex_row().gap_1().items_center().when(
                self.input_graphemes != 0,
                |this| {
                    this.child(
                        Button::new("characters")
                            .label(format!(
                                "{} {}",
                                self.input_graphemes,
                                t!("status.characters")
                            ))
                            .ghost()
                            .xsmall()
                            .tooltip(format!(
                                "{}{} {}",
                                t!("status.result"),
                                self.output_graphemes,
                                t!("status.characters")
                            ))
                            .text_color(gray_600()),
                    )
                },
            ))
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
