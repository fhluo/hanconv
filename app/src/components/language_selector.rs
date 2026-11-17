use gpui_kit::component::button::Button;
use gpui_kit::component::menu::{DropdownMenu, PopupMenuItem};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{Anchor, App, FocusHandle, IntoElement, RenderOnce, Window};
use icu_locale::Locale;
use std::rc::Rc;

#[derive(IntoElement)]
pub struct LanguageSelector {
    button: Button,
    selected: Option<Locale>,
    on_change: Option<Rc<dyn Fn(&Locale, &mut Window, &mut App) + 'static>>,
    anchor: Anchor,
    action_context: Option<FocusHandle>,
}

impl LanguageSelector {
    pub fn new(button: Button, selected: Option<Locale>) -> Self {
        LanguageSelector {
            button,
            selected,
            on_change: None,
            anchor: Anchor::TopRight,
            action_context: None,
        }
    }

    pub fn on_change(mut self, handler: impl Fn(&Locale, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));

        self
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

impl RenderOnce for LanguageSelector {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        self.button
            .dropdown_menu({
                move |mut menu, _, _| {
                    if let Some(action_context) = self.action_context.clone() {
                        menu = menu.action_context(action_context);
                    }

                    for key in available_locales!() {
                        let Ok(locale) = key.parse::<Locale>() else {
                            continue;
                        };

                        menu = menu.item(
                            PopupMenuItem::new(t!(key.as_ref()))
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
