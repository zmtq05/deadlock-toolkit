use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use steamlocate::SteamDir;
use tauri::Result;
use time::OffsetDateTime;

#[derive(Clone, Serialize, Deserialize)]
pub struct MyConfig {
    pub game_path: Option<PathBuf>,
    pub downloaded_at: HashMap<DownloadTarget, OffsetDateTime>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DownloadTarget {
    Translation,
    BuiltinFont,
    ExternalFont,
}

impl DownloadTarget {
    pub fn to_file_name(self) -> &'static str {
        match self {
            DownloadTarget::Translation => "translation.zip",
            DownloadTarget::BuiltinFont => "builtin_font.zip",
            DownloadTarget::ExternalFont => "external_font.zip",
        }
    }
}

impl MyConfig {
    pub fn new() -> Self {
        const DEADLOCK_APP_ID: &u32 = &1422450;
        let game_path = (|| {
            SteamDir::locate()?
                .app(DEADLOCK_APP_ID)
                .map(|a| a.path.clone())
        })();
        Self {
            game_path,
            downloaded_at: Default::default(),
        }
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn save(&self, path: impl AsRef<Path>) -> Result<()> {
        let f = fs::File::create(path)?;
        serde_json::to_writer(f, self)?;

        Ok(())
    }
}
