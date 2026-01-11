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
    div, px, size, App, Application, Bounds, ClipboardItem, Entity, ExternalPaths,
    Focusable, Menu, MenuItem, MouseButton, PathPromptOptions, Window, WindowBounds, WindowOptions,
};
use gpui_component::button::{Button, ButtonVariants};
use gpui_component::input::{Input, InputEvent, InputState};
use gpui_component::menu::AppMenuBar;
use gpui_component::{gray_500, ActiveTheme, Root, Sizable, TitleBar};
use icu_locale::Locale;
use rust_i18n::set_locale;
use std::fs;
use std::path::{Path, PathBuf};
use strum::{EnumCount, VariantArray};

i18n!("locales", fallback = "en");

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

        cx.set_menus(vec![Menu {
            name: t!("conversion").into(),
            items,
        }]);

        self.menu_bar.update(cx, |menu_bar, cx| {
            menu_bar.reload(cx);
        });
    }

    fn update_last_directory(&mut self, directory: impl AsRef<Path>) {
        self.config.last_directory = Some(directory.as_ref().to_path_buf());
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

            let text = fs::read_to_string(path).ok()?;

            window
                .update(|window, cx| {
                    input_editor.update(cx, |this, cx| {
                        this.set_value(text, window, cx);
                    });
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
}

impl Render for Hanconv {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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

        div()
            .on_action(cx.listener(Self::run_conversion))
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
