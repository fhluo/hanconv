use crate::conversion::Conversion;
use dirs::document_dir;
use gpui::{Context, EventEmitter};
use icu_locale::fallback::{LocaleFallbackConfig, LocaleFallbackPriority};
use icu_locale::{locale, DataLocale, Locale, LocaleFallbacker};
use rust_i18n::set_locale;
use serde::{Deserialize, Serialize};
use std::env::home_dir;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)]
    app_name: String,

    locale: Option<Locale>,
    conversion: Conversion,
    last_directory: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            app_name: env!("CARGO_PKG_NAME").to_string(),
            locale: None,
            conversion: Conversion::S2T,
            last_directory: None,
        }
    }
}

impl Config {
    pub fn load(app_name: impl Into<String>) -> Config {
        let app_name = app_name.into();
        if let Ok(mut config) = confy::load::<Config>(&app_name, None) {
            config.app_name = app_name;
            config
        } else {
            Default::default()
        }
    }

    pub fn store(&self) {
        if let Err(err) = confy::store(&self.app_name, None, self) {
            eprintln!("{err}")
        }
    }

    pub fn init(&mut self, cx: &mut Context<Self>) {
        self.init_locale();
        cx.emit(ConfigEvent::LocaleChange);

        self.init_path();
        cx.emit(ConfigEvent::LastDirectoryChange);
    }

    fn init_locale(&mut self) {
        let mut fallback_iter = LocaleFallbacker::new()
            .for_config({
                let mut config = LocaleFallbackConfig::default();
                config.priority = LocaleFallbackPriority::Language;
                config
            })
            .fallback_for({
                if let Some(locale) = self.locale.clone() {
                    locale.into()
                } else {
                    sys_locale::get_locale()
                        .unwrap_or_else(|| "en".to_string())
                        .parse::<Locale>()
                        .ok()
                        .unwrap_or_else(|| locale!("en"))
                        .into()
                }
            });

        let locales = available_locales!()
            .into_iter()
            .filter_map(|locale| locale.parse::<Locale>().map(DataLocale::from).ok())
            .collect::<Vec<_>>();

        let locale = loop {
            let locale = fallback_iter.get();
            if locale.is_unknown() {
                break locale!("en");
            }

            if locales.contains(locale) {
                break locale.into_locale();
            }

            fallback_iter.step();
        };

        set_locale(locale.to_string().as_str());
        self.locale = Some(locale);
    }

    fn init_path(&mut self) {
        if self.last_directory.is_none() {
            self.last_directory = document_dir().or_else(home_dir)
        }
    }

    pub fn locale(&self) -> Option<&Locale> {
        self.locale.as_ref()
    }

    pub fn set_locale(&mut self, locale: &Locale, cx: &mut Context<Self>) {
        set_locale(&locale.to_string());
        self.locale = Some(locale.to_owned());

        cx.emit(ConfigEvent::LocaleChange);
    }

    pub fn conversion(&self) -> Conversion {
        self.conversion
    }

    pub fn set_conversion(&mut self, conversion: Conversion, cx: &mut Context<Self>) {
        self.conversion = conversion;

        cx.emit(ConfigEvent::ConversionChange);
    }

    pub fn last_directory(&self) -> Option<&PathBuf> {
        self.last_directory.as_ref()
    }

    pub fn set_last_directory(&mut self, path: impl AsRef<Path>, cx: &mut Context<Self>) {
        self.last_directory = Some(path.as_ref().to_path_buf());

        cx.emit(ConfigEvent::LastDirectoryChange);
    }
}

pub enum ConfigEvent {
    LocaleChange,
    ConversionChange,
    LastDirectoryChange,
}

impl EventEmitter<ConfigEvent> for Config {}
