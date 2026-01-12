use crate::conversion::Conversion;
use gpui::{App, Corner, IntoElement, RenderOnce, Window};
use gpui_component::button::Button;
use gpui_component::menu::DropdownMenu;
use strum::VariantArray;

#[derive(IntoElement)]
pub struct ConversionSelector {
    button: Button,
    selected: Conversion,
    anchor: Corner,
}

impl ConversionSelector {
    pub fn new(button: Button, selected: Conversion) -> Self {
        ConversionSelector {
            button,
            selected,
            anchor: Corner::BottomRight,
        }
    }

    pub fn anchor(mut self, anchor: impl Into<Corner>) -> Self {
        self.anchor = anchor.into();

        self
    }
}

impl RenderOnce for ConversionSelector {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.button
            .dropdown_menu({
                move |mut menu, _, _| {
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
