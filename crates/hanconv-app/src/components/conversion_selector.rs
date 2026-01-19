use crate::conversion::Conversion;
use gpui::{App, Corner, FocusHandle, IntoElement, RenderOnce, Window};
use gpui_component::button::Button;
use gpui_component::menu::DropdownMenu;
use strum::VariantArray;

#[derive(IntoElement)]
pub struct ConversionSelector {
    button: Button,
    selected: Conversion,
    anchor: Corner,
    action_context: Option<FocusHandle>,
}

impl ConversionSelector {
    pub fn new(button: Button, selected: Conversion) -> Self {
        ConversionSelector {
            button,
            selected,
            anchor: Corner::BottomRight,
            action_context: None,
        }
    }

    #[allow(dead_code)]
    pub fn anchor(mut self, anchor: impl Into<Corner>) -> Self {
        self.anchor = anchor.into();

        self
    }

    pub fn action_context(mut self, handle: FocusHandle) -> Self {
        self.action_context = Some(handle);

        self
    }
}

impl RenderOnce for ConversionSelector {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.button
            .dropdown_menu({
                move |mut menu, _, _| {
                    if let Some(action_context) = self.action_context.clone() {
                        menu = menu.action_context(action_context);
                    }

                    for conversions in Conversion::VARIANTS.chunks(2) {
                        for &conversion in conversions {
                            menu = menu.menu_with_check(
                                conversion.title(),
                                self.selected == conversion,
                                Box::new(conversion),
                            );
                        }

                        menu = menu.separator();
                    }

                    menu
                }
            })
            .anchor(self.anchor)
    }
}
