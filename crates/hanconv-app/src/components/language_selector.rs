use gpui::prelude::FluentBuilder;
use gpui::{App, Corner, IntoElement, RenderOnce, Window};
use gpui_component::button::Button;
use gpui_component::menu::{DropdownMenu, PopupMenuItem};
use icu_locale::Locale;
use std::rc::Rc;

#[derive(IntoElement)]
pub struct LanguageSelector {
    button: Button,
    selected: Option<Locale>,
    on_change: Option<Rc<dyn Fn(&Locale, &mut Window, &mut App) + 'static>>,
    anchor: Corner,
}

impl LanguageSelector {
    pub fn new(button: Button, selected: Option<Locale>) -> Self {
        LanguageSelector {
            button,
            selected,
            on_change: None,
            anchor: Corner::TopRight,
        }
    }

    pub fn on_change(mut self, handler: impl Fn(&Locale, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));

        self
    }

    pub fn anchor(mut self, anchor: impl Into<Corner>) -> Self {
        self.anchor = anchor.into();

        self
    }
}

impl RenderOnce for LanguageSelector {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.button
            .dropdown_menu({
                move |mut menu, _, _| {
                    for (key, locale) in available_locales!()
                        .into_iter()
                        .filter_map(|locale| Some((locale, locale.parse::<Locale>().ok()?)))
                    {
                        menu = menu.item(
                            PopupMenuItem::new(t!(key))
                                .checked(self.selected.as_ref() == Some(&locale))
                                .when_some(self.on_change.clone(), |this, on_change| {
                                    this.on_click(move |_, window, cx| {
                                        on_change(&locale, window, cx);
                                    })
                                }),
                        )
                    }

                    menu
                }
            })
            .anchor(self.anchor)
    }
}
