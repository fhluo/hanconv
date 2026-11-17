use crate::conversion::Conversion;
use gpui_kit::component::button::Button;
use gpui_kit::component::menu::DropdownMenu;
use gpui_kit::{Anchor, App, FocusHandle, IntoElement, RenderOnce, Window};
use strum::VariantArray;

#[derive(IntoElement)]
pub struct ConversionSelector {
    button: Button,
    selected: Conversion,
    anchor: Anchor,
    action_context: Option<FocusHandle>,
}

impl ConversionSelector {
    pub fn new(button: Button, selected: Conversion) -> Self {
        ConversionSelector {
            button,
            selected,
            anchor: Anchor::BottomRight,
            action_context: None,
        }
    }

    #[allow(dead_code)]
    pub fn anchor(mut self, anchor: impl Into<Anchor>) -> Self {
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
