use anyhow::{bail, Context, Result};
use log::{info, trace};
use reqwest::blocking::Response;
use std::{fs, path::PathBuf};

use crate::{card::Card, cli::LanguageCode, pack::Pack};

pub struct DataStore {
    root_dir: PathBuf,
    locale: LanguageCode,
}

pub enum StoreLocation<'a> {
    RootDir,
    LocaleDir,
    ImagesDir,
    JsonDir,
    PacksListFile,
    CardsFile(&'a str),
    ImageFile(&'a Card),
}

impl DataStore {
    pub fn new(root_dir: &PathBuf, locale: LanguageCode) -> Self {
        DataStore {
            root_dir: root_dir.to_path_buf(),
            locale,
        }
    }

    pub fn get_path(&self, location: StoreLocation) -> Result<PathBuf> {
        let path = match location {
            StoreLocation::RootDir => &self.root_dir,
            StoreLocation::LocaleDir => {
                let locale_path = self.locale.to_path()?;
                &self.get_path(StoreLocation::RootDir)?.join(locale_path)
            }
            StoreLocation::ImagesDir => &self.get_path(StoreLocation::LocaleDir)?.join("images/"),
            StoreLocation::JsonDir => &self.get_path(StoreLocation::LocaleDir)?.join("json/"),
            StoreLocation::PacksListFile => {
                &self.get_path(StoreLocation::JsonDir)?.join("packs.json")
            }
            StoreLocation::CardsFile(pack_id) => &self.get_cards_filename(&pack_id)?,
            StoreLocation::ImageFile(card) => {
                let filename = Self::get_img_filename(card)?;
                &self.get_path(StoreLocation::ImagesDir)?.join(filename)
            }
        };

        Ok(path.to_path_buf())
    }

    fn get_cards_filename(&self, card_id: &str) -> Result<PathBuf> {
        let parent_dir = self.get_path(StoreLocation::JsonDir)?;
        let filename = format!("cards_{}.json", card_id);
        let path = parent_dir.join(filename);
        Ok(path)
    }

    fn get_img_filename(card: &Card) -> Result<String> {
        let last_slash_pos = card.img_url.rfind('/').context("expected to find `/`")?;

        let img_file_name = match card.img_url.find('?') {
            Some(quest_mark_pos) => &card.img_url[last_slash_pos + 1..quest_mark_pos],
            None => &card.img_url[last_slash_pos + 1..],
        };

        info!("filename for `{}` is: {}", card.id, img_file_name);
        Ok(img_file_name.to_string())
    }

    fn ensure_created(&self, location: StoreLocation) -> Result<()> {
        let root_dir = self.get_path(location)?;
        if root_dir.exists() {
            info!("data dir already exists at `{}`", root_dir.display());
            return Ok(());
        }

        match fs::create_dir_all(&root_dir) {
            Ok(_) => info!("successfully created `{}`", root_dir.display()),
            Err(e) => bail!("failed to create `{}`: {}", root_dir.display(), e),
        }

        Ok(())
    }

    pub fn write_packs(&self, packs: &Vec<Pack>) -> Result<()> {
        self.ensure_created(StoreLocation::JsonDir)?;

        let path = self.get_path(StoreLocation::PacksListFile)?;
        info!(
            "about to write {} packs to file: `{}`",
            packs.len(),
            path.display()
        );

        let json = serde_json::to_string(&packs)?;
        trace!("serialize data: `{:?} -> {}`", packs, json);

        fs::write(path, json)?;
        info!("wrote packs data to file");

        Ok(())
    }

    pub fn write_cards(&self, pack_id: &str, cards: &Vec<Card>) -> Result<()> {
        self.ensure_created(StoreLocation::JsonDir)?;

        let path = self.get_path(StoreLocation::CardsFile(&pack_id))?;
        info!(
            "about to write {} cards from `{}` to file: `{}`",
            cards.len(),
            &pack_id,
            path.display()
        );

        let json = serde_json::to_string(&cards)?;
        trace!("serialize data: `{:?} -> {}`", cards, json);

        fs::write(path, json)?;
        info!("wrote cards data to file");

        Ok(())
    }

    pub fn write_image(&self, card: &Card, mut img_data: Response) -> Result<()> {
        self.ensure_created(StoreLocation::ImagesDir)?;

        let path = self.get_path(StoreLocation::ImageFile(card))?;
        info!(
            "about to save image `{}` packs to file: `{}`",
            card.img_url,
            path.display()
        );

        let mut file = std::fs::File::create(path)?;

        img_data.copy_to(&mut file)?;
        info!("saved image to file");

        Ok(())
    }
}

// pub fn load_data() -> Result<OnePieceTcgData> {
//     let path = data_file_path();
//     info!("load data from: {}", path.to_string_lossy());
//
//     let json = fs::read_to_string(path)?;
//     let data: OnePieceTcgData = serde_json::from_str(&json)?;
//     trace!("deserialize data: `{} -> {:?}`", json, data);
//     debug!(
//         "loaded {} card sets ({} cards)",
//         data.card_sets.len(),
//         data.total_cards()
//     );
//
//     Ok(data)
// }
