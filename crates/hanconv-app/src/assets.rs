use gpui::{AssetSource, SharedString};
use gpui_component::IconNamed;
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "assets"]
#[include = "icons/**/*.svg"]
pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<Cow<'static, [u8]>>> {
        if let Some(file) = Self::get(path) {
            Ok(Some(file.data))
        } else {
            gpui_component_assets::Assets.load(path)
        }
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        let mut paths = gpui_component_assets::Assets.list(path).unwrap_or_default();

        paths.extend(Self::iter().filter_map(|p| p.starts_with(path).then(|| p.into())));

        Ok(paths)
    }
}

#[derive(Copy, Clone)]
pub enum Icons {
    Clipboard,
    Languages,
    Save,
    Trash,
    Trash2,
}

impl IconNamed for Icons {
    fn path(self) -> SharedString {
        match self {
            Icons::Clipboard => "icons/clipboard.svg",
            Icons::Languages => "icons/languages.svg",
            Icons::Save => "icons/save.svg",
            Icons::Trash => "icons/trash.svg",
            Icons::Trash2 => "icons/trash-2.svg",
        }
        .into()
    }
}
