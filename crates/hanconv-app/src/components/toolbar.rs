use crate::assets::Icons;
use gpui::prelude::*;
use gpui::{
    actions, div, Action, App, ElementId, IntoElement, RenderOnce, StyleRefinement, Window,
};
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
                            .tooltip(t!("tooltip.Open"))
                            .disabled(disabled),
                        )
                    })
                    .when_some(self.save, |this, disabled| {
                        this.child(
                            ToolbarItem::new(format!("{}-save", self.id), Icons::Save, Save)
                                .tooltip(t!("tooltip.Save"))
                                .disabled(disabled),
                        )
                    })
                    .when_some(self.clear, |this, disabled| {
                        this.child(
                            ToolbarItem::new(format!("{}-clear", self.id), Icons::Trash2, Clear)
                                .tooltip(t!("tooltip.Clear"))
                                .disabled(disabled),
                        )
                    })
                    .when_some(self.copy, |this, disabled| {
                        this.child(
                            ToolbarItem::new(format!("{}-copy", self.id), IconName::Copy, Copy)
                                .tooltip(t!("tooltip.Copy"))
                                .disabled(disabled),
                        )
                    })
                    .when_some(self.paste, |this, disabled| {
                        this.child(
                            ToolbarItem::new(format!("{}-paste", self.id), Icons::Clipboard, Paste)
                                .tooltip(t!("tooltip.Paste"))
                                .disabled(disabled),
                        )
                    }),
            )
    }
}

#[derive(IntoElement)]
struct ToolbarItem {
    id: ElementId,
    style: StyleRefinement,
    icon: Icon,
    action: Box<dyn Action>,
    disabled: bool,
    tooltip: Option<String>,
}

impl Styled for ToolbarItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl Disableable for ToolbarItem {
    fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;

        self
    }
}

impl ToolbarItem {
    fn new(id: impl Into<ElementId>, icon: impl Into<Icon>, action: impl Action) -> Self {
        ToolbarItem {
            id: id.into(),
            style: Default::default(),
            icon: icon.into(),
            action: Box::new(action),
            disabled: false,
            tooltip: None,
        }
    }

    fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());

        self
    }
}

impl RenderOnce for ToolbarItem {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let Self {
            id,
            style,
            icon,
            action,
            disabled,
            tooltip,
        } = self;

        Button::new(id)
            .icon(icon)
            .text_color(gray_500())
            .small()
            .ghost()
            .disabled(disabled)
            .refine_style(&style)
            .when_some(tooltip, |this, tooltip| this.tooltip(tooltip))
            .when(disabled, |this| this.text_color(gray_200()))
            .on_click(move |_, window, cx| window.dispatch_action(action.boxed_clone(), cx))
    }
}
