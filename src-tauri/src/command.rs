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
use zip_extract::ZipExtractError;

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

fn extract(src: impl AsRef<Path>, dest: impl AsRef<Path>) -> tauri::Result<()> {
    let archive = fs::read(src)?;
    zip_extract::extract(Cursor::new(archive), dest.as_ref(), false).map_err(|e| match e {
        ZipExtractError::Io(error) => error.into(),
        ZipExtractError::Zip(zip_error) => std::io::Error::from(zip_error).into(),
        ZipExtractError::StripToplevel { .. } => unreachable!(),
    })
}

#[command]
pub fn apply(
    handle: tauri::AppHandle,
    config: State<'_, RwLock<MyConfig>>,
    target: DownloadTarget,
) -> tauri::Result<()> {
    let src = handle.path().app_cache_dir()?.join(target.file_name());
    let config = config.read().unwrap();
    let dest = config
        .game_path
        .as_deref()
        .ok_or(anyhow!("path doesn't exists"))?;

    extract(src, dest)
}

#[command]
pub fn record_download_time(
    config: State<'_, RwLock<MyConfig>>,
    target: DownloadTarget,
) -> tauri::Result<()> {
    config
        .write()
        .unwrap()
        .downloaded_at
        .insert(target, OffsetDateTime::now_utc());
    Ok(())
}

#[command]
pub fn get_download_time(
    config: State<'_, RwLock<MyConfig>>,
    target: DownloadTarget,
) -> Option<OffsetDateTime> {
    config.read().unwrap().downloaded_at.get(&target).copied()
}

#[command]
pub fn is_exists(handle: tauri::AppHandle, target: DownloadTarget) -> bool {
    match handle.path().app_cache_dir() {
        Ok(app_cache_dir) => app_cache_dir.join(target.file_name()).exists(),
        Err(_) => false,
    }
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
        return Err(anyhow!(response.status().as_u16()).into());
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
                        target.file_name(),
                        last_modified.format(&Rfc2822).unwrap(),
                        download_time.format(&Rfc2822).unwrap(),
                    );
                    &last_modified > download_time // NOTE: not verified
                }))
        }
        None => Ok(false),
    }
}
