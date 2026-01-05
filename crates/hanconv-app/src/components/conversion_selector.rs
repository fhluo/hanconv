use crate::conversion::Conversion;
use gpui::{App, IntoElement, RenderOnce, Window};
use gpui_component::button::Button;
use gpui_component::menu::DropdownMenu;
use strum::VariantArray;

#[derive(IntoElement)]
pub struct ConversionSelector {
    button: Button,
    selected: Conversion,
}

impl ConversionSelector {
    pub fn new(button: Button, selected: Conversion) -> Self {
        ConversionSelector { button, selected }
    }
}

impl RenderOnce for ConversionSelector {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.button.dropdown_menu({
            move |mut menu, _, _| {
                for (i, conversions) in Conversion::VARIANTS.chunks(2).enumerate() {
                    if i > 0 {
                        menu = menu.separator();
                    }

                    for &conversion in conversions {
                        menu = menu.menu_with_check(
                            conversion.title(),
                            self.selected == conversion,
                            Box::new(conversion),
                        );
                    }
                }

                menu
            }
        })
    }
}
