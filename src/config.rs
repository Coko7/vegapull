use anyhow::{bail, Result};
use log::info;
use std::{env, fs, path::PathBuf};
use xdg::BaseDirectories;

use crate::localizer::{EN_LOCALE_RAW, JP_LOCALE_RAW};

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

    if let Ok(xdg_dirs) = BaseDirectories::new() {
        let config_home = xdg_dirs.get_config_home();
        let val = config_home.join(APP_NAME);
        info!("get config from XDG: {}", val.to_string_lossy());

        return Ok(val);
    }

    if let Ok(home_dir) = env::var("HOME") {
        let val = PathBuf::from(home_dir).join(".config").join(APP_NAME);
        info!("get config from HOME: {}", val.to_string_lossy());

        return Ok(val);
    }

    bail!("could not get config directory")
}

pub fn initialize_configs() -> Result<()> {
    let config_dir = get_config_dir()?;
    if !config_dir.exists() {
        info!("creating config dir at: {}", config_dir.display());
        fs::create_dir_all(&config_dir)?;
    }

    let locale_files = [("en.toml", EN_LOCALE_RAW), ("jp.toml", JP_LOCALE_RAW)];

    for (filename, content) in &locale_files {
        let file_path = config_dir.join(filename);
        if !file_path.exists() {
            info!("creating locale file: {}", file_path.display());
            fs::write(file_path, content)?;
        }
    }

    Ok(())
}
