use std::{collections::HashMap, fs, path::PathBuf};

use log::{debug, info};
use serde::{Deserialize, Serialize};

use crate::cli::LanguageCode;

#[derive(Debug, Deserialize, Serialize)]
pub struct Localizer {
    pub hostname: String,

    pub colors: HashMap<String, String>,
    pub attributes: HashMap<String, String>,
    pub categories: HashMap<String, String>,
    pub rarities: HashMap<String, String>,
}

// pub enum Locale {
//     Japanese,
//     ChineseSimp,
//     ChineseHK,
//     ChineseTW,
//     Thai,
//     EnglishAsia,
//     English,
// }

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

    pub fn load(language: LanguageCode) -> Result<Localizer, anyhow::Error> {
        match language {
            LanguageCode::English => Self::load_from_file("en"),
            LanguageCode::Japanese => Self::load_from_file("jp"),
        }
    }

    pub fn load_from_file(locale: &str) -> Result<Localizer, anyhow::Error> {
        let path = format!("./locales/{}.toml", locale);
        let path = PathBuf::from(path);
        info!("load {} locale from: {}", locale, path.to_string_lossy());

        let locale_data = fs::read_to_string(path)?;
        debug!("loaded {}", locale_data);

        let localizer: Localizer = toml::from_str(&locale_data)?;
        Ok(localizer)
    }
}
