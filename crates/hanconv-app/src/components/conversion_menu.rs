use crate::conversion::Conversion;
use gpui::prelude::FluentBuilder;
use gpui::{App, ElementId, IntoElement, RenderOnce, SharedString, Window};
use gpui_component::button::Button;
use gpui_component::menu::DropdownMenu;

#[derive(IntoElement)]
pub struct ConversionMenu {
    id: ElementId,
    label: Option<SharedString>,
    conversion: Conversion,
}

impl ConversionMenu {
    pub fn new(id: impl Into<ElementId>, conversion: Conversion) -> Self {
        ConversionMenu {
            id: id.into(),
            label: None,
            conversion,
        }
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl RenderOnce for ConversionMenu {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        Button::new(self.id)
            .when_some(self.label, |this, label| this.label(label))
            .dropdown_menu({
                move |mut menu, _, _| {
                    for (i, conversions) in Conversion::all().chunks(2).enumerate() {
                        if i > 0 {
                            menu = menu.separator();
                        }

                        for &conversion in conversions {
                            menu = menu.menu_with_check(
                                conversion.title(),
                                self.conversion == conversion,
                                Box::new(conversion),
                            );
                        }
                    }

                    menu
                }
            })
    }
}
