#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate rust_i18n;

mod assets;
mod components;
mod config;
mod conversion;

use crate::assets::{Assets, Icons};
use crate::components::{open_about_dialog, toolbar, LanguageSelector, StatusBar, Toolbar};
use crate::config::{Config, ConfigEvent};
use crate::conversion::Conversion;
use gpui::prelude::*;
use gpui::{
    actions, div, px, size, Action, App, Application, Bounds, ClipboardItem,
    Entity, ExternalPaths, FocusHandle, Focusable, Menu, MenuItem, MouseButton,
    PathPromptOptions, SharedString, Window, WindowBounds, WindowOptions,
};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::description_list::DescriptionList;
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::label::Label;
use gpui_component::link::Link;
use gpui_component::menu::AppMenuBar;
use gpui_component::{
    gray_500, gray_900, ActiveTheme, Icon, IconName, Root, Sizable, StyledExt, ThemeRegistry,
    TitleBar, WindowExt,
};
use icu_locale::Locale;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{fs, io};
use strum::{EnumCount, VariantArray};
use unicode_segmentation::UnicodeSegmentation;

i18n!("locales", fallback = "en");

actions!([About, Repository]);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Action)]
struct ChangeTheme(pub SharedString);

struct Hanconv {
    config: Entity<Config>,

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

        cx.on_focus_lost(window, |this, window, cx| {
            this.input_editor
                .update(cx, |this, cx| this.focus(window, cx));
        })
        .detach();

        Hanconv {
            config: Self::setup_config(window, cx),
            input_editor,
            output_editor,
            menu_bar: AppMenuBar::new(cx),
        }
    }

    fn setup_config(window: &mut Window, cx: &mut Context<Self>) -> Entity<Config> {
        let config = cx.new(|_| Config::load("Hanconv"));

        cx.observe_new(|this: &mut Self, _, cx| {
            this.config.update(cx, |this, cx| {
                this.init(cx);
            });
        })
        .detach();

        cx.on_release(|this, cx| {
            this.config.update(cx, |this, _| {
                this.store();
            });
        })
        .detach();

        cx.subscribe_in(&config, window, |this, _, event, window, cx| match event {
            ConfigEvent::LocaleChange => {
                this.update_menu_bar(cx);
                this.update_editors(window, cx);
                cx.notify();
            }
            ConfigEvent::ThemeChange => {
                this.update_menu_bar(cx);
                cx.refresh_windows();
            }
            _ => {}
        })
        .detach();

        config
    }

    fn on_input_event(
        &mut self,
        _: &Entity<InputState>,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if matches!(event, InputEvent::Change) {
            self.run_conversion(&self.config.read(cx).conversion(), window, cx);
        }
    }

    fn run_conversion(
        &mut self,
        conversion: &Conversion,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.config.update(cx, |this, cx| {
            this.set_conversion(*conversion, cx);
        });

        let content = self.input_editor.read(cx).value();
        let result = conversion.run(content);

        self.output_editor.update(cx, |state, cx| {
            state.set_value(result, window, cx);
        });

        self.update_menu_bar(cx);
    }

    fn change_locale(&mut self, locale: &Locale, _: &mut Window, cx: &mut Context<Self>) {
        self.config.update(cx, |this, cx| {
            this.set_locale(locale, cx);
        });
    }

    fn change_theme(
        &mut self,
        ChangeTheme(theme): &ChangeTheme,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.config.update(cx, |this, cx| {
            this.set_theme(theme, cx);
        });
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
        let mut conversion_menu_items =
            Vec::with_capacity(Conversion::COUNT + (Conversion::COUNT - 1) / 2);

        for conversions in chunks {
            conversion_menu_items.extend(conversions.iter().cloned().map(|conversion| {
                MenuItem::action(conversion.title(), conversion)
                    .checked(self.config.read(cx).conversion() == conversion)
            }));
            conversion_menu_items.push(MenuItem::Separator);
        }

        let theme_menu_items = ThemeRegistry::global(cx)
            .sorted_themes()
            .into_iter()
            .map(|theme_config| theme_config.name.clone())
            .map(|theme_name| {
                MenuItem::action(theme_name.clone(), ChangeTheme(theme_name.clone()))
                    .checked(self.config.read(cx).theme() == Some(&theme_name))
            })
            .collect::<Vec<_>>();

        cx.set_menus(vec![
            Menu {
                name: t!("Conversion").into(),
                items: conversion_menu_items,
            },
            Menu {
                name: t!("Theme").into(),
                items: theme_menu_items,
            },
            Menu {
                name: t!("Help").into(),
                items: vec![
                    MenuItem::action(t!("help.Repository"), Repository),
                    MenuItem::Separator,
                    MenuItem::action(t!("About"), About),
                ],
            },
        ]);

        self.menu_bar.update(cx, |menu_bar, cx| {
            menu_bar.reload(cx);
        });
    }

    fn update_last_directory(&mut self, directory: impl AsRef<Path>, cx: &mut Context<Self>) {
        self.config.update(cx, |this, cx| {
            this.set_last_directory(directory, cx);
        });
    }

    fn input_graphemes(&mut self, cx: &mut Context<Self>) -> usize {
        self.input_editor.read(cx).value().graphemes(true).count()
    }

    fn output_graphemes(&mut self, cx: &mut Context<Self>) -> usize {
        self.output_editor.read(cx).value().graphemes(true).count()
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

            this.update_in(window, |this, _, cx| {
                if let Some(path) = path.parent() {
                    this.update_last_directory(path, cx);
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

    fn last_directory(&mut self, cx: &mut Context<Self>) -> PathBuf {
        self.config
            .read(cx)
            .last_directory()
            .cloned()
            .unwrap_or_else(|| PathBuf::from("."))
    }

    fn save_input(&mut self, _: &toolbar::Save, _: &mut Window, cx: &mut Context<Self>) {
        let dir = self.last_directory(cx);
        let path = cx.prompt_for_new_path(dir.as_path(), None);

        cx.spawn(async move |this, cx| {
            let path = path.await.ok()?.ok()??;

            this.update(cx, |this, cx| {
                if let Some(path) = path.parent() {
                    this.update_last_directory(path, cx);
                }

                fs::write(path, this.input_editor.read(cx).value().as_ref()).ok()
            })
            .ok()?
        })
        .detach();
    }

    fn save_output(&mut self, _: &toolbar::Save, _: &mut Window, cx: &mut Context<Self>) {
        let dir = self.last_directory(cx);
        let path = cx.prompt_for_new_path(dir.as_path(), None);

        cx.spawn(async move |this, cx| {
            let path = path.await.ok()?.ok()??;

            this.update(cx, |this, cx| {
                if let Some(path) = path.parent() {
                    this.update_last_directory(path, cx);
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

    fn open_repository(&mut self, _: &Repository, _: &mut Window, cx: &mut Context<Self>) {
        cx.open_url("https://github.com/fhluo/hanconv");
    }

    fn on_action_about(&mut self, _: &About, window: &mut Window, cx: &mut Context<Self>) {
        open_about_dialog(window, cx);
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
            self.config.read(cx).locale().cloned(),
        )
        .on_change(cx.listener(Self::change_locale))
        .action_context(self.input_editor.focus_handle(cx));

        let dialog_layer = Root::render_dialog_layer(window, cx);

        div()
            .on_action(cx.listener(Self::run_conversion))
            .on_action(cx.listener(Self::change_theme))
            .on_action(cx.listener(Self::on_action_about))
            .on_action(cx.listener(Self::open_repository))
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
                                    this.input_editor
                                        .update(cx, |this, cx| this.focus(window, cx));
                                }),
                            )
                            .on_drop(cx.listener(|this, paths: &ExternalPaths, window, cx| {
                                if let Some(path) = paths.paths().first()
                                    && let Ok(text) = fs::read_to_string(path)
                                {
                                    if let Some(path) = path.parent() {
                                        this.update_last_directory(path, cx);
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
                                let paste_disabled = cx
                                    .read_from_clipboard()
                                    .is_none_or(|item| item.text().is_none());

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
                                    this.output_editor
                                        .update(cx, |this, cx| this.focus(window, cx));
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
            .child(
                StatusBar::new(
                    self.input_graphemes(cx),
                    self.output_graphemes(cx),
                    self.config.read(cx).conversion(),
                )
                .action_context(self.input_editor.focus_handle(cx)),
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
