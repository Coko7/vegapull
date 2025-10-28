use anyhow::{bail, Result};
use directories::ProjectDirs;
use log::info;
use std::{env, fs, path::PathBuf};

use crate::localizer::{EN_LOCALE_RAW, EN_ASIA_LOCALE_RAW, JP_LOCALE_RAW, FR_LOCALE_RAW, TH_LOCALE_RAW, ZH_HK_LOCALE_RAW, ZH_TW_LOCALE_RAW};

pub const APP_NAME: &str = "vegapull";
pub const CONFIG_VAR: &str = "VEGAPULL_CONFIG";

pub fn get_config_dir() -> Result<PathBuf> {
    if let Ok(config_var) = env::var(CONFIG_VAR) {
        let val = PathBuf::from(config_var);
        info!(
            "get config from env: {} = {}",
            CONFIG_VAR,
            val.to_string_lossy()
        );

        return Ok(val);
    }

    if let Some(proj_dirs) = ProjectDirs::from("", "", APP_NAME) {
        let config_dir = proj_dirs.config_dir();
        info!("get config dir from proj dirs: {}", config_dir.display());
        return Ok(config_dir.to_path_buf());
    }

    bail!("could not get config directory")
}

pub fn initialize_configs() -> Result<()> {
    let config_dir = get_config_dir()?;
    if !config_dir.exists() {
        info!("creating config dir at: {}", config_dir.display());
        fs::create_dir_all(&config_dir)?;
    }

    let locale_files = [
        ("en.toml", EN_LOCALE_RAW),
        ("en_asia.toml", EN_ASIA_LOCALE_RAW),
        ("jp.toml", JP_LOCALE_RAW),
        ("zh_hk.toml", ZH_HK_LOCALE_RAW),
        ("zh_tw.toml", ZH_TW_LOCALE_RAW),
        ("th.toml", TH_LOCALE_RAW),
        ("fr.toml", FR_LOCALE_RAW),
    ];

    for (filename, content) in &locale_files {
        let file_path = config_dir.join(filename);
        if !file_path.exists() {
            info!("creating locale file: {}", file_path.display());
            fs::write(file_path, content)?;
        }
    }

    Ok(())
}
