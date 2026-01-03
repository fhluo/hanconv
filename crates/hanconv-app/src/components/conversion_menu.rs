use crate::conversion::Conversion;
use gpui::{App, IntoElement, RenderOnce, Window};
use gpui_component::button::Button;
use gpui_component::menu::DropdownMenu;

#[derive(IntoElement)]
pub struct ConversionMenu {
    button: Button,
    checked: Conversion,
}

impl ConversionMenu {
    pub fn new(button: Button, checked: Conversion) -> Self {
        ConversionMenu { button, checked }
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
                            self.checked == conversion,
                            Box::new(conversion),
                        );
                    }
                }

                menu
            }
        })
    }
}
