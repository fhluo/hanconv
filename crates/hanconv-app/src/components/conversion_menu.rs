use crate::conversion::Conversion;
use gpui::{App, IntoElement, RenderOnce, Window};
use gpui_component::button::Button;
use gpui_component::menu::DropdownMenu;

#[derive(IntoElement)]
pub struct ConversionMenu {
    button: Button,
    checked_conversion: Conversion,
}

impl ConversionMenu {
    pub fn new(button: Button, checked_conversion: Conversion) -> Self {
        ConversionMenu {
            button,
            checked_conversion,
        }
    }
}

impl RenderOnce for ConversionMenu {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.button.dropdown_menu({
            move |mut menu, _, _| {
                for (i, conversions) in Conversion::all().chunks(2).enumerate() {
                    if i > 0 {
                        menu = menu.separator();
                    }

                    for &conversion in conversions {
                        menu = menu.menu_with_check(
                            conversion.title(),
                            self.checked_conversion == conversion,
                            Box::new(conversion),
                        );
                    }
                }

                menu
            }
        })
    }
}
