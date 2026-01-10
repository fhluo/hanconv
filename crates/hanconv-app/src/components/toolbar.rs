use crate::assets::Icons;
use gpui::prelude::*;
use gpui::{actions, div, Action, App, ElementId, IntoElement, RenderOnce, Window};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::label::Label;
use gpui_component::{
    gray_200, gray_400, gray_500, ActiveTheme, Disableable, Icon, IconName, Sizable, StyledExt,
};

actions!([Open, Save, Clear, Copy, Paste]);

#[derive(Default, IntoElement)]
pub struct Toolbar {
    id: String,
    title: String,
    open: Option<bool>,
    save: Option<bool>,
    clear: Option<bool>,
    copy: Option<bool>,
    paste: Option<bool>,
}

impl Toolbar {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Toolbar {
        Toolbar {
            id: id.into(),
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn open(mut self, open: Option<bool>) -> Self {
        self.open = open;
        self
    }

    pub fn save(mut self, save: Option<bool>) -> Self {
        self.save = save;
        self
    }

    pub fn clear(mut self, clear: Option<bool>) -> Self {
        self.clear = clear;
        self
    }

    pub fn copy(mut self, copy: Option<bool>) -> Self {
        self.copy = copy;
        self
    }

    pub fn paste(mut self, paste: Option<bool>) -> Self {
        self.paste = paste;
        self
    }
}

impl RenderOnce for Toolbar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .h_8()
            .flex()
            .flex_row()
            .px_3()
            .border_color(cx.theme().border)
            .border_r_1()
            .border_b_1()
            .items_center()
            .child(
                Label::new(t!(self.title))
                    .text_color(gray_400())
                    .text_xs()
                    .font_semibold(),
            )
            .child(
                div()
                    .ml_auto()
                    .h_full()
                    .flex()
                    .flex_row()
                    .gap_1()
                    .items_center()
                    .when_some(self.open, |this, disabled| {
                        this.child(
                            ToolbarItem::new(
                                format!("{}-open", self.id),
                                IconName::FolderOpen,
                                Open,
                            )
                            .disabled(disabled),
                        )
                    })
                    .when_some(self.save, |this, disabled| {
                        this.child(
                            ToolbarItem::new(format!("{}-save", self.id), Icons::Save, Save)
                                .disabled(disabled),
                        )
                    })
                    .when_some(self.clear, |this, disabled| {
                        this.child(
                            ToolbarItem::new(format!("{}-clear", self.id), Icons::Trash2, Clear)
                                .disabled(disabled),
                        )
                    })
                    .when_some(self.copy, |this, disabled| {
                        this.child(
                            ToolbarItem::new(format!("{}-copy", self.id), IconName::Copy, Copy)
                                .disabled(disabled),
                        )
                    })
                    .when_some(self.paste, |this, disabled| {
                        this.child(
                            ToolbarItem::new(format!("{}-paste", self.id), Icons::Clipboard, Paste)
                                .disabled(disabled),
                        )
                    }),
            )
    }
}

#[derive(IntoElement)]
struct ToolbarItem {
    id: ElementId,
    icon: Icon,
    action: Box<dyn Action>,
    disabled: bool,
}

impl ToolbarItem {
    fn new(id: impl Into<ElementId>, icon: impl Into<Icon>, action: impl Action) -> Self {
        ToolbarItem {
            id: id.into(),
            icon: icon.into(),
            action: Box::new(action),
            disabled: false,
        }
    }

    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl RenderOnce for ToolbarItem {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let Self {
            id,
            icon,
            action,
            disabled,
        } = self;

        Button::new(id)
            .icon(icon)
            .text_color(gray_500())
            .small()
            .ghost()
            .disabled(disabled)
            .when(disabled, |this| this.text_color(gray_200()))
            .on_click(move |_, window, cx| window.dispatch_action(action.boxed_clone(), cx))
    }
}
