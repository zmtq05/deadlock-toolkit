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
            let mut config = if app_config_dir.exists() && config_path.exists() {
                MyConfig::load(config_path).expect("Failed to load config")
            } else {
                fs::create_dir_all(&app_config_dir).expect("failed to create directory");
                MyConfig::new()
            };
            let app_cache_dir = path_resolver.app_cache_dir().unwrap();
            fs::create_dir_all(&app_cache_dir)?;
            config.sync(&app_cache_dir);
            app.manage(RwLock::new(config));
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
            apply,
            record_download_time,
            get_download_time,
            is_exists,
            check_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
