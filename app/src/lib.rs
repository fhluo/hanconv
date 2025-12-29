use encoding_rs::Encoding;
use serde::Serialize;
use std::fs::File;
use std::io::Read;
#[cfg(target_os = "windows")]
use {tauri::Manager, window_vibrancy::apply_mica};
#[cfg(target_os = "macos")]
use {
    tauri::Manager,
    window_vibrancy::{apply_liquid_glass, NSGlassEffectViewStyle},
};

#[tauri::command]
async fn s2t(s: String) -> String {
    hanconv::s2t(s)
}

#[tauri::command]
async fn t2s(s: String) -> String {
    hanconv::t2s(s)
}

#[tauri::command]
async fn s2tw(s: String) -> String {
    hanconv::s2tw(s)
}

#[tauri::command]
async fn tw2s(s: String) -> String {
    hanconv::tw2s(s)
}

#[tauri::command]
async fn s2twp(s: String) -> String {
    hanconv::s2twp(s)
}

#[tauri::command]
async fn tw2sp(s: String) -> String {
    hanconv::tw2sp(s)
}

#[tauri::command]
async fn t2tw(s: String) -> String {
    hanconv::t2tw(s)
}

#[tauri::command]
async fn tw2t(s: String) -> String {
    hanconv::tw2t(s)
}

#[tauri::command]
async fn s2hk(s: String) -> String {
    hanconv::s2hk(s)
}

#[tauri::command]
async fn hk2s(s: String) -> String {
    hanconv::hk2s(s)
}

#[tauri::command]
async fn t2hk(s: String) -> String {
    hanconv::t2hk(s)
}

#[tauri::command]
async fn hk2t(s: String) -> String {
    hanconv::hk2t(s)
}

#[tauri::command]
async fn t2jp(s: String) -> String {
    hanconv::t2jp(s)
}

#[tauri::command]
async fn jp2t(s: String) -> String {
    hanconv::jp2t(s)
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Encoding Error: {0}")]
    Encoding(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[tauri::command]
async fn read_text_file(path: String, encoding: Option<String>) -> Result<(String, String), Error> {
    let mut file = File::open(&path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    let mut label = encoding.unwrap_or("auto".to_string());

    if label.eq_ignore_ascii_case("auto") {
        if let Some(best) = charset_normalizer_rs::from_bytes(&buffer, None)
            .map_err(Error::Encoding)?
            .get_best()
        {
            label = best.encoding().to_string();
        }
    }

    let encoding = Encoding::for_label(label.as_bytes())
        .ok_or_else(|| Error::Encoding(format!("Unknown encoding: {}", label)))?;
    let (cow, _, _) = encoding.decode(&buffer);

    Ok((cow.into_owned(), encoding.name().to_string()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "windows")]
            apply_mica(&window, Some(false))?;

            #[cfg(target_os = "macos")]
            apply_liquid_glass(&window, NSGlassEffectViewStyle::Clear, None, Some(16.0))?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            s2t,
            t2s,
            s2tw,
            tw2s,
            s2twp,
            tw2sp,
            t2tw,
            tw2t,
            s2hk,
            hk2s,
            t2hk,
            hk2t,
            t2jp,
            jp2t,
            read_text_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
