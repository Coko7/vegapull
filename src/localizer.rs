use anyhow::{ensure, Context, Result};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

use crate::{cli::LanguageCode, config};

pub const EN_LOCALE_RAW: &str = include_str!("../config/en.toml");
pub const JP_LOCALE_RAW: &str = include_str!("../config/jp.toml");

#[derive(Debug, Deserialize, Serialize)]
pub struct Localizer {
    pub hostname: String,

    pub colors: HashMap<String, String>,
    pub attributes: HashMap<String, String>,
    pub categories: HashMap<String, String>,
    pub rarities: HashMap<String, String>,
}

impl Localizer {
    fn reverse_search(hash_map: &HashMap<String, String>, value: &str) -> Option<String> {
        hash_map.iter().find_map(|(key, val)| {
            if val == value {
                Some(key.to_string())
            } else {
                None
            }
        })
    }

    pub fn match_color(&self, value: &str) -> Option<String> {
        Self::reverse_search(&self.colors, value)
    }

    pub fn match_attribute(&self, value: &str) -> Option<String> {
        Self::reverse_search(&self.attributes, value)
    }

    pub fn match_category(&self, value: &str) -> Option<String> {
        Self::reverse_search(&self.categories, value)
    }

    pub fn match_rarity(&self, value: &str) -> Option<String> {
        Self::reverse_search(&self.rarities, value)
    }

    pub fn find_locales() -> Result<()> {
        let config_dir = config::get_config_dir()?;
        ensure!(
            config_dir.exists(),
            format!("config directory not found: {}", config_dir.display())
        );

        let entries = fs::read_dir(&config_dir)?;
        println!("config directory: {}", config_dir.display());

        for entry in entries {
            let entry = entry?;
            let file_name = entry.file_name();
            println!("- {}", file_name.to_string_lossy());
        }

        Ok(())
    }

    pub fn load(language: LanguageCode) -> Result<Localizer> {
        match language {
            LanguageCode::ChineseHongKong => Self::load_from_file("zh_hk"),
            LanguageCode::ChineseSimplified => Self::load_from_file("zh_cn"),
            LanguageCode::ChineseTaiwan => Self::load_from_file("zh_tw"),
            LanguageCode::English => Self::load_from_file("en"),
            LanguageCode::EnglishAsia => Self::load_from_file("en_asia"),
            LanguageCode::Japanese => Self::load_from_file("jp"),
            LanguageCode::Thai => Self::load_from_file("th"),
        }
    }

    pub fn load_from_file(locale: &str) -> Result<Localizer> {
        let config_dir = config::get_config_dir()?;

        ensure!(
            config_dir.exists(),
            format!("config directory not found: {}", config_dir.display())
        );

        let locale_path = config_dir.join(format!("{}.toml", locale));
        ensure!(
            locale_path.exists(),
            format!("locale file not found: {}", locale_path.display())
        );

        info!("load {} locale from: {}", locale, locale_path.display());

        let locale_data = fs::read_to_string(&locale_path)
            .with_context(|| format!("Failed to open file: {}", locale_path.display()))?;
        debug!("loaded {}", locale_data);

        let localizer: Localizer = toml::from_str(&locale_data)?;
        Ok(localizer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_map() -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert(String::from("foo"), String::from("Toto"));
        map.insert(String::from("bar"), String::from("Tata"));
        map.insert(String::from("baz"), String::from("Tutu"));

        map
    }

    #[test]
    fn reverse_search_returns_some() {
        let map = get_test_map();

        let actual = Localizer::reverse_search(&map, "Toto");
        let expected = Some(String::from("foo"));

        assert_eq!(actual, expected);
    }

    #[test]
    fn reverse_search_returns_none() {
        let map = get_test_map();

        let actual = Localizer::reverse_search(&map, "Titi");
        let expected = None;

        assert_eq!(actual, expected);
    }
}
