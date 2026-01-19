use crate::components::ConversionSelector;
use crate::conversion::Conversion;
use gpui::prelude::*;
use gpui::{div, App, FocusHandle, IntoElement, RenderOnce, Window};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::{gray_600, ActiveTheme, Sizable};

#[derive(IntoElement)]
pub struct StatusBar {
    input_graphemes: usize,
    output_graphemes: usize,
    conversion: Conversion,
    action_context: Option<FocusHandle>,
}

impl StatusBar {
    pub fn new(input_graphemes: usize, output_graphemes: usize, conversion: Conversion) -> Self {
        StatusBar {
            input_graphemes,
            output_graphemes,
            conversion,
            action_context: None,
        }
    }
    
    pub fn action_context(mut self, handle: FocusHandle) -> Self {
        self.action_context = Some(handle);
        
        self
    }
}

impl RenderOnce for StatusBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let mut conversion_selector = ConversionSelector::new(
            Button::new("conversion-button")
                .label(t!(self.conversion.title()))
                .xsmall()
                .ghost()
                .ml_auto()
                .text_color(gray_600()),
            self.conversion,
        );
        
        if let Some(action_context) = self.action_context {
            conversion_selector = conversion_selector.action_context(action_context);
        }

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
