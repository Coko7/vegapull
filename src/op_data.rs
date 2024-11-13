use anyhow::Context;
use chrono::{DateTime, Utc};
use core::panic;
use log::{debug, error, info, trace};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

use crate::{card::Card, pack::Pack};

#[derive(Debug, Deserialize, Serialize)]
pub struct OnePieceTcgData {
    pub base_url: String,
    pub fetch_start_date: DateTime<Utc>,
    pub fetch_end_date: DateTime<Utc>,
    pub card_sets: Vec<Pack>,
}

impl OnePieceTcgData {
    pub fn total_cards(&self) -> usize {
        panic!("not reimplemented")
        // self.card_sets
        //     .iter()
        //     .map(|card_set| card_set.cards.len())
        //     .sum()
    }
}

fn data_dir_path() -> PathBuf {
    PathBuf::from("data/")
}

fn img_dir_path() -> PathBuf {
    data_dir_path().join("images/")
}

fn data_file_path() -> PathBuf {
    data_dir_path().join("cards.json")
}

pub fn create_data_dir() -> Result<(), anyhow::Error> {
    let dir = data_dir_path();
    if dir.exists() {
        info!("data dir already exists at `{}`", dir.to_string_lossy());
        return Ok(());
    }

    match fs::create_dir_all(&dir) {
        Ok(_) => info!("successfully created `{}`", dir.to_string_lossy()),
        Err(e) => error!("failed to create `{}`: {}", dir.to_string_lossy(), e),
    }

    Ok(())
}

pub fn get_img_filename(card: &Card) -> Result<String, anyhow::Error> {
    let last_slash_pos = card.img_url.rfind('/').context("expected to find `/`")?;

    let img_file_name = match card.img_url.find('?') {
        Some(quest_mark_pos) => &card.img_url[last_slash_pos + 1..quest_mark_pos],
        None => &card.img_url[last_slash_pos + 1..],
    };

    info!("filename for `{}` is: {}", card.id, img_file_name);
    Ok(img_file_name.to_string())
}

pub fn compute_img_file_path(card: &Card) -> Result<PathBuf, anyhow::Error> {
    let img_filename = get_img_filename(card)?;
    let path = img_dir_path().join(img_filename);

    info!(
        "img file path for `{}` is: {}",
        card.id,
        path.to_string_lossy()
    );
    Ok(path)
}

pub fn load_data() -> Result<OnePieceTcgData, anyhow::Error> {
    let path = data_file_path();
    info!("load data from: {}", path.to_string_lossy());

    let json = fs::read_to_string(path)?;
    let data: OnePieceTcgData = serde_json::from_str(&json)?;
    trace!("deserialize data: `{} -> {:?}`", json, data);
    debug!(
        "loaded {} card sets ({} cards)",
        data.card_sets.len(),
        data.total_cards()
    );

    Ok(data)
}

pub fn write_data(data: &OnePieceTcgData) -> Result<(), anyhow::Error> {
    create_data_dir()?;

    let path = data_file_path();
    info!(
        "write {} cards sets ({} cards) to file: `{}`",
        data.card_sets.len(),
        data.total_cards(),
        path.to_string_lossy()
    );

    let json = serde_json::to_string(data)?;
    trace!("serialize data: `{:?} -> {}`", data, json);

    fs::write(path, json)?;
    info!("wrote data to file");

    Ok(())
}
