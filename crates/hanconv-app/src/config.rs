use crate::conversion::Conversion;
use icu_locale::fallback::{LocaleFallbackConfig, LocaleFallbackPriority};
use icu_locale::{locale, DataLocale, Locale, LocaleFallbacker};
use rust_i18n::set_locale;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(skip)]
    app_name: String,

    pub locale: Option<Locale>,
    pub conversion: Conversion,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            app_name: env!("CARGO_PKG_NAME").to_string(),
            locale: None,
            conversion: Conversion::S2T,
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

    pub fn init(&mut self) {
        self.init_locale();
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
}
