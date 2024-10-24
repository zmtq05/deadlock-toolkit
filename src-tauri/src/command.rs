use std::{
    fs,
    io::Cursor,
    path::{Path, PathBuf},
    sync::RwLock,
};

use anyhow::anyhow;
use reqwest::header::LAST_MODIFIED;
use tauri::{command, Manager, State};
use time::{format_description::well_known::Rfc2822, OffsetDateTime};

use crate::config::{DownloadTarget, MyConfig};

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
pub fn record_download_time(
    config: State<'_, RwLock<MyConfig>>,
    target: DownloadTarget,
) -> tauri::Result<()> {
    let mut config = config.write().unwrap();
    config
        .downloaded_at
        .insert(target, OffsetDateTime::now_utc());
    Ok(())
}

#[command]
pub fn get_download_time(
    config: State<'_, RwLock<MyConfig>>,
    target: DownloadTarget,
) -> Option<OffsetDateTime> {
    let config = config.read().unwrap();
    config.downloaded_at.get(&target).copied()
}

#[command]
pub fn is_exists(handle: tauri::AppHandle, target: DownloadTarget) -> bool {
    let cache_dir = handle.path().app_cache_dir().unwrap();
    cache_dir.join(target.to_file_name()).exists()
}

#[command]
pub async fn check_update(
    config: State<'_, RwLock<MyConfig>>,
    target: DownloadTarget,
) -> tauri::Result<bool> {
    let response = reqwest::Client::new()
        .head(target.download_url())
        .send()
        .await
        .map_err(|e| anyhow!(e))?;

    if !response.status().is_success() {
        return Err(anyhow!(response.status().as_u16()))?;
    }

    let config = config.read().unwrap();
    match response.headers().get(LAST_MODIFIED) {
        Some(last_modified) => {
            let last_modified =
                OffsetDateTime::parse(last_modified.to_str().unwrap(), &Rfc2822).unwrap();
            Ok(config
                .downloaded_at
                .get(&target)
                .is_some_and(|download_time| {
                    #[cfg(debug_assertions)]
                    println!(
                        "{} - last: {} | downloaded: {}",
                        target.to_file_name(),
                        last_modified.format(&Rfc2822).unwrap(),
                        download_time.format(&Rfc2822).unwrap(),
                    );
                    &last_modified > download_time // NOTE: not verified
                }))
        }
        None => Ok(false),
    }
}
