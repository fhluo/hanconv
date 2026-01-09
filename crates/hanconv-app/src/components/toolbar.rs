use crate::assets::Icons;
use gpui::prelude::*;
use gpui::{actions, div, Action, App, ElementId, IntoElement, RenderOnce, Window};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::label::Label;
use gpui_component::{gray_400, gray_500, ActiveTheme, Icon, IconName, Sizable, StyledExt};

actions!([Open, Save, Clear, Copy, Paste]);

#[derive(Default, IntoElement)]
pub struct Toolbar {
    id: String,
    title: String,
    open: bool,
    save: bool,
    clear: bool,
    copy: bool,
    paste: bool,
}

impl Toolbar {
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Toolbar {
        Toolbar {
            id: id.into(),
            title: title.into(),
            ..Default::default()
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn save(mut self, save: bool) -> Self {
        self.save = save;
        self
    }

    pub fn clear(mut self, clear: bool) -> Self {
        self.clear = clear;
        self
    }

    pub fn copy(mut self, copy: bool) -> Self {
        self.copy = copy;
        self
    }

    pub fn paste(mut self, paste: bool) -> Self {
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
                    .when(self.open, |this| {
                        this.child(ToolbarItem::new(
                            format!("{}-open", self.id),
                            IconName::FolderOpen,
                            Open,
                        ))
                    })
                    .when(self.save, |this| {
                        this.child(ToolbarItem::new(
                            format!("{}-save", self.id),
                            Icons::Save,
                            Save,
                        ))
                    })
                    .when(self.clear, |this| {
                        this.child(ToolbarItem::new(
                            format!("{}-clear", self.id),
                            Icons::Trash2,
                            Clear,
                        ))
                    })
                    .when(self.copy, |this| {
                        this.child(ToolbarItem::new(
                            format!("{}-copy", self.id),
                            IconName::Copy,
                            Copy,
                        ))
                    })
                    .when(self.paste, |this| {
                        this.child(ToolbarItem::new(
                            format!("{}-paste", self.id),
                            Icons::Clipboard,
                            Paste,
                        ))
                    }),
            )
    }
}

#[derive(IntoElement)]
struct ToolbarItem {
    id: ElementId,
    icon: Icon,
    action: Box<dyn Action>,
}

impl ToolbarItem {
    fn new(id: impl Into<ElementId>, icon: impl Into<Icon>, action: impl Action) -> Self {
        ToolbarItem {
            id: id.into(),
            icon: icon.into(),
            action: Box::new(action),
        }
    }
}

impl RenderOnce for ToolbarItem {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        Button::new(self.id)
            .icon(self.icon)
            .text_color(gray_500())
            .small()
            .ghost()
            .on_click(move |_, _, cx| cx.dispatch_action(self.action.as_ref()))
    }
}
