mod command;
mod config;

use std::{fs, sync::RwLock};

use command::*;
use config::MyConfig;
use tauri::{Manager, WindowEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_upload::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let path_resolver = app.handle().path();
            let app_config_dir = path_resolver.app_config_dir().unwrap();
            let config_path = app_config_dir.join("config.json");
            let config = if app_config_dir.exists() && config_path.exists() {
                MyConfig::load(config_path).expect("Failed to load config")
            } else {
                fs::create_dir_all(&app_config_dir).expect("failed to create directory");
                MyConfig::new()
            };
            app.manage(RwLock::new(config));
            fs::create_dir_all(path_resolver.app_cache_dir().unwrap())?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                let config = window.app_handle().state::<RwLock<MyConfig>>();
                config
                    .read()
                    .unwrap()
                    .save(
                        window
                            .app_handle()
                            .path()
                            .app_config_dir()
                            .unwrap()
                            .join("config.json"),
                    )
                    .unwrap();
            }
        })
        .invoke_handler(tauri::generate_handler![
            get_game_path,
            change_game_path,
            extract_translation,
            extract_builtin_font,
            extract_external_font,
            record_download_time,
            get_download_time,
            is_exists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
