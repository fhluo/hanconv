#[cfg(target_os = "windows")]
use {tauri::Manager, window_vibrancy::apply_mica};
#[cfg(target_os = "macos")]
use {
    tauri::Manager,
    window_vibrancy::{NSGlassEffectViewStyle, apply_liquid_glass},
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
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
            s2t, t2s, s2tw, tw2s, s2twp, tw2sp, t2tw, tw2t, s2hk, hk2s, t2hk, hk2t, t2jp, jp2t
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
