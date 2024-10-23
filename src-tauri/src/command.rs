use std::{
    fs,
    io::Cursor,
    path::{Path, PathBuf},
    sync::RwLock,
};

use tauri::{command, Manager, State};
use time::OffsetDateTime;

use crate::config::MyConfig;

#[command]
pub fn get_game_path(config: State<'_, RwLock<MyConfig>>) -> Option<PathBuf> {
    let config = config.read().unwrap();
    config.game_path.clone()
}

#[command]
pub fn change_game_path(path: &str, config: State<'_, RwLock<MyConfig>>) -> Result<String, String> {
    let path = PathBuf::from(path);
    if !path.join("game").exists() {
        Err(format!("{}에 game 폴더가 없습니다.", path.display()))
    } else {
        let mut config = config.write().unwrap();
        config.game_path.replace(path);

        Ok("설정 완료".to_string())
    }
}

async fn extract(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> tauri::Result<()> {
    let archive = fs::read(src)?;
    zip_extract::extract(Cursor::new(archive), dest.as_ref(), false).unwrap();
    Ok(())
}

#[command]
pub async fn extract_translation(
    handle: tauri::AppHandle,
    config: State<'_, RwLock<MyConfig>>,
) -> tauri::Result<()> {
    let dest = {
        let config = config.read().unwrap();
        config
            .game_path
            .clone()
            .ok_or("path doesn't exists")
            .unwrap()
    };
    let src = handle
        .path()
        .app_cache_dir()
        .unwrap()
        .join("translation.zip");
    extract(src, dest).await?;
    Ok(())
}

#[command]
pub async fn extract_builtin_font(
    handle: tauri::AppHandle,
    config: State<'_, RwLock<MyConfig>>,
) -> tauri::Result<()> {
    let dest = {
        let config = config.read().unwrap();
        config
            .game_path
            .clone()
            .ok_or("path doesn't exists")
            .unwrap()
    };
    let src = handle
        .path()
        .app_cache_dir()
        .unwrap()
        .join("builtin_font.zip");
    extract(src, dest).await?;
    Ok(())
}

#[command]
pub async fn extract_external_font(
    handle: tauri::AppHandle,
    config: State<'_, RwLock<MyConfig>>,
) -> tauri::Result<()> {
    let dest = {
        let config = config.read().unwrap();
        config
            .game_path
            .clone()
            .ok_or("path doesn't exists")
            .unwrap()
    };
    let src = handle
        .path()
        .app_cache_dir()
        .unwrap()
        .join("external_font.zip");
    extract(src, dest).await?;
    Ok(())
}

#[command]
pub fn record_download_time(config: State<'_, RwLock<MyConfig>>, obj: String) -> tauri::Result<()> {
    let mut config = config.write().unwrap();
    config.downloaded_at.insert(obj, OffsetDateTime::now_utc());
    Ok(())
}

#[command]
pub fn get_download_time(
    config: State<'_, RwLock<MyConfig>>,
    obj: String,
) -> Option<OffsetDateTime> {
    let config = config.read().unwrap();
    config.downloaded_at.get(&obj).copied()
}
