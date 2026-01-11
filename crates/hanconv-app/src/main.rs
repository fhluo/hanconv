#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate rust_i18n;

mod assets;
mod components;
mod config;
mod conversion;

use crate::assets::{Assets, Icons};
use crate::components::{toolbar, LanguageSelector, Toolbar};
use crate::config::Config;
use crate::conversion::Conversion;
use gpui::prelude::*;
use gpui::{
    actions, div, px, size, App, Application, Bounds, ClipboardItem, Entity,
    ExternalPaths, Focusable, Menu, MenuItem, MouseButton, PathPromptOptions, Window, WindowBounds, WindowOptions,
};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::description_list::DescriptionList;
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::label::Label;
use gpui_component::link::Link;
use gpui_component::menu::AppMenuBar;
use gpui_component::{
    gray_500, ActiveTheme, Icon, IconName, Root, Sizable, StyledExt, TitleBar, WindowExt,
};
use icu_locale::Locale;
use rust_i18n::set_locale;
use std::path::{Path, PathBuf};
use std::{fs, io};
use strum::{EnumCount, VariantArray};

i18n!("locales", fallback = "en");

actions!([About]);

struct Hanconv {
    config: Config,

    menu_bar: Entity<AppMenuBar>,
    input_editor: Entity<InputState>,
    output_editor: Entity<InputState>,
}

impl Hanconv {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input_editor = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .placeholder(t!("input.placeholder"))
        });
        let output_editor = cx.new(|cx| {
            InputState::new(window, cx)
                .multi_line(true)
                .placeholder(t!("output.placeholder"))
        });

        cx.subscribe_in(&input_editor, window, Self::on_input_event)
            .detach();

        cx.observe_new(Self::init_config).detach();
        cx.on_release(Self::store_config).detach();

        Hanconv {
            config: Config::load("hanconv"),
            input_editor,
            output_editor,
            menu_bar: AppMenuBar::new(cx),
        }
    }

    fn on_input_event(
        &mut self,
        _: &Entity<InputState>,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(event, InputEvent::Change) {
            self.run_conversion(&self.config.conversion.clone(), window, cx);
        }
    }

    fn init_config(&mut self, window: Option<&mut Window>, cx: &mut Context<Self>) {
        self.config.init();
        self.update_menu_bar(cx);

        if let Some(window) = window {
            self.update_editors(window, cx);
        }
    }

    fn store_config(&mut self, _: &mut App) {
        self.config.store()
    }

    fn run_conversion(
        &mut self,
        conversion: &Conversion,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.config.conversion = *conversion;

        let content = self.input_editor.read(cx).value();
        let result = conversion.run(content);

        self.output_editor.update(cx, |state, cx| {
            state.set_value(result, window, cx);
        });

        self.update_menu_bar(cx);
    }

    fn change_locale(&mut self, locale: &Locale, window: &mut Window, cx: &mut Context<Self>) {
        set_locale(&locale.to_string());
        self.config.locale = Some(locale.to_owned());

        self.update_menu_bar(cx);
        self.update_editors(window, cx);
    }

    fn update_editors(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.input_editor.update(cx, |this, cx| {
            this.set_placeholder(t!("input.placeholder"), window, cx);
        });

        self.output_editor.update(cx, |this, cx| {
            this.set_placeholder(t!("output.placeholder"), window, cx);
        });
    }

    fn update_menu_bar(&mut self, cx: &mut Context<Self>) {
        let chunks = Conversion::VARIANTS.chunks(2);
        let mut items = Vec::with_capacity(Conversion::COUNT + (Conversion::COUNT - 1) / 2);

        for conversions in chunks {
            items.extend(conversions.iter().cloned().map(|conversion| {
                MenuItem::action(conversion.title(), conversion)
                    .checked(self.config.conversion == conversion)
            }));
            items.push(MenuItem::Separator);
        }

        cx.set_menus(vec![
            Menu {
                name: t!("Conversion").into(),
                items,
            },
            Menu {
                name: t!("Help").into(),
                items: vec![MenuItem::action(t!("About"), About)],
            },
        ]);

        self.menu_bar.update(cx, |menu_bar, cx| {
            menu_bar.reload(cx);
        });
    }

    fn update_last_directory(&mut self, directory: impl AsRef<Path>) {
        self.config.last_directory = Some(directory.as_ref().to_path_buf());
    }

    fn open_io_error_dialog(
        window: &mut Window,
        cx: &mut App,
        path: impl AsRef<Path>,
        err: io::Error,
    ) {
        let path = path.as_ref().to_owned();

        window.open_dialog(cx, move |dialog, _, cx| {
            dialog
                .alert()
                .title(
                    div()
                        .flex()
                        .flex_row()
                        .gap_3()
                        .items_center()
                        .child(
                            Icon::new(IconName::CircleX)
                                .text_color(cx.theme().red)
                                .size_6(),
                        )
                        .child(Label::new(t!("error.read-file")).font_semibold().text_lg()),
                )
                .child(
                    div().child(
                        DescriptionList::vertical()
                            .columns(1)
                            .item(
                                t!("Path").to_string(),
                                Link::new("path")
                                    .child(path.display().to_string())
                                    .on_click({
                                        let path = path.clone();
                                        move |_, _, cx| cx.reveal_path(path.as_path())
                                    })
                                    .into_any_element(),
                                1,
                            )
                            .item(t!("Error").to_string(), err.to_string(), 1),
                    ),
                )
        });
    }

    fn open(&mut self, _: &toolbar::Open, window: &mut Window, cx: &mut Context<Self>) {
        let path = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: None,
        });

        let input_editor = self.input_editor.clone();

        cx.spawn_in(window, async move |this, window| {
            let path = path.await.ok()?.ok()??.into_iter().next()?;

            this.update_in(window, |this, _, _| {
                if let Some(path) = path.parent() {
                    this.update_last_directory(path);
                }
            })
            .ok()?;

            window
                .update(|window, cx| match fs::read_to_string(&path) {
                    Ok(text) => {
                        input_editor.update(cx, |this, cx| {
                            this.set_value(text, window, cx);
                        });
                    }
                    Err(err) => {
                        Self::open_io_error_dialog(window, cx, path, err);
                    }
                })
                .ok()
        })
        .detach();
    }

    fn save_input(&mut self, _: &toolbar::Save, _: &mut Window, cx: &mut Context<Self>) {
        let path = cx.prompt_for_new_path(
            self.config
                .last_directory
                .clone()
                .unwrap_or_else(|| PathBuf::from("."))
                .as_path(),
            None,
        );

        cx.spawn(async move |this, cx| {
            let path = path.await.ok()?.ok()??;

            this.update(cx, |this, cx| {
                if let Some(path) = path.parent() {
                    this.update_last_directory(path);
                }

                fs::write(path, this.input_editor.read(cx).value().as_ref()).ok()
            })
            .ok()?
        })
        .detach();
    }

    fn save_output(&mut self, _: &toolbar::Save, _: &mut Window, cx: &mut Context<Self>) {
        let path = cx.prompt_for_new_path(
            self.config
                .last_directory
                .clone()
                .unwrap_or_else(|| PathBuf::from("."))
                .as_path(),
            None,
        );

        cx.spawn(async move |this, cx| {
            let path = path.await.ok()?.ok()??;

            this.update(cx, |this, cx| {
                if let Some(path) = path.parent() {
                    this.update_last_directory(path);
                }

                fs::write(path, this.output_editor.read(cx).value().as_ref()).ok()
            })
            .ok()?
        })
        .detach();
    }

    fn clear(&mut self, _: &toolbar::Clear, window: &mut Window, cx: &mut Context<Self>) {
        self.input_editor.update(cx, |this, cx| {
            this.set_value("", window, cx);
        })
    }

    fn copy_input(&mut self, _: &toolbar::Copy, _: &mut Window, cx: &mut Context<Self>) {
        cx.write_to_clipboard(ClipboardItem::new_string(
            self.input_editor.read(cx).value().to_string(),
        ));
    }

    fn copy_output(&mut self, _: &toolbar::Copy, _: &mut Window, cx: &mut Context<Self>) {
        cx.write_to_clipboard(ClipboardItem::new_string(
            self.output_editor.read(cx).value().to_string(),
        ))
    }

    fn paste(&mut self, _: &toolbar::Paste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(item) = cx.read_from_clipboard()
            && let Some(text) = item.text()
        {
            self.input_editor.update(cx, |this, cx| {
                this.insert(text, window, cx);
            })
        }
    }

    fn open_about_dialog(&mut self, _: &About, window: &mut Window, cx: &mut Context<Self>) {
        window.open_dialog(cx, |dialog, _, _| {
            dialog.alert().title(t!("About").to_string()).child(
                div().child(
                    DescriptionList::horizontal()
                        .columns(1)
                        .item(t!("about.Name").to_string(), "Hanconv", 1)
                        .item(
                            t!("about.Version").to_string(),
                            env!("CARGO_PKG_VERSION"),
                            1,
                        )
                        .item(
                            t!("about.Description").to_string(),
                            t!("about.DescriptionText").to_string(),
                            1,
                        )
                        .item(
                            t!("about.Repository").to_string(),
                            Link::new("repository")
                                .href("https://github.com/fhluo/hanconv")
                                .child("https://github.com/fhluo/hanconv")
                                .into_any_element(),
                            1,
                        )
                        .item(t!("about.License").to_string(), "MIT", 1),
                ),
            )
        });
    }
}

impl Render for Hanconv {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let language_selector = LanguageSelector::new(
            Button::new("language-button")
                .small()
                .ghost()
                .icon(Icons::Languages)
                .text_color(gray_500())
                .tooltip(t!("language")),
            self.config.locale.clone(),
        )
        .on_change(cx.listener(Self::change_locale));

        let dialog_layer = Root::render_dialog_layer(window, cx);

        div()
            .on_action(cx.listener(Self::run_conversion))
            .on_action(cx.listener(Self::open_about_dialog))
            .w_full()
            .h_full()
            .flex()
            .flex_col()
            .child(
                TitleBar::new()
                    .items_center()
                    .child(self.menu_bar.clone())
                    .child(
                        div().flex().flex_row().flex_1().child(
                            div()
                                .flex()
                                .flex_row()
                                .h_full()
                                .ml_auto()
                                .mr_3()
                                .child(language_selector),
                        ),
                    ),
            )
            .child(
                div()
                    .w_full()
                    .h_full()
                    .flex()
                    .flex_row()
                    .child(
                        div()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, window, cx| {
                                    this.input_editor.focus_handle(cx).focus(window, cx);
                                }),
                            )
                            .on_drop(cx.listener(|this, paths: &ExternalPaths, window, cx| {
                                if let Some(path) = paths.paths().first()
                                    && let Ok(text) = fs::read_to_string(path)
                                {
                                    if let Some(path) = path.parent() {
                                        this.update_last_directory(path);
                                    }

                                    this.input_editor.update(cx, |this, cx| {
                                        this.set_value(text, window, cx);
                                    });
                                }
                            }))
                            .on_action(cx.listener(Self::open))
                            .on_action(cx.listener(Self::save_input))
                            .on_action(cx.listener(Self::clear))
                            .on_action(cx.listener(Self::copy_input))
                            .on_action(cx.listener(Self::paste))
                            .flex_1()
                            .flex()
                            .flex_col()
                            .child({
                                let is_empty = self.input_editor.read(cx).value().is_empty();
                                let paste_disabled = !cx
                                    .read_from_clipboard()
                                    .is_some_and(|item| item.text().is_some());

                                Toolbar::new("source", t!("Source"))
                                    .open(Some(false))
                                    .save(Some(is_empty))
                                    .clear(Some(is_empty))
                                    .copy(Some(is_empty))
                                    .paste(Some(paste_disabled))
                            })
                            .child(
                                Input::new(&self.input_editor)
                                    .flex_1()
                                    .appearance(false)
                                    .border_r_1()
                                    .border_color(cx.theme().border),
                            ),
                    )
                    .child(
                        div()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _, window, cx| {
                                    this.output_editor.focus_handle(cx).focus(window, cx);
                                }),
                            )
                            .on_action(cx.listener(Self::save_output))
                            .on_action(cx.listener(Self::copy_output))
                            .flex_1()
                            .flex()
                            .flex_col()
                            .child({
                                let is_empty = self.output_editor.read(cx).value().is_empty();
                                Toolbar::new("target", t!("Target"))
                                    .save(Some(is_empty))
                                    .copy(Some(is_empty))
                            })
                            .child(Input::new(&self.output_editor).flex_1().appearance(false)),
                    ),
            )
            .children(dialog_layer)
    }
}

fn main() -> anyhow::Result<()> {
    let app = Application::new().with_assets(Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        let bounds = Bounds::centered(None, size(px(800.), px(600.)), cx);

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions {
                    titlebar: Some(TitleBar::title_bar_options()),
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| Hanconv::new(window, cx));

                    cx.new(|cx| Root::new(view, window, cx))
                },
            )?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });

    Ok(())
}
