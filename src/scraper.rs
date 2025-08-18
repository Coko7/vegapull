use anyhow::{bail, Context, Result};
use log::{debug, info};
use reqwest::blocking::{Client, Response};
use std::collections::HashMap;

use crate::{
    card::{Card, CardScraper},
    localizer::Localizer,
    pack::Pack,
};

pub struct OpTcgScraper<'a> {
    base_url: String,
    localizer: &'a Localizer,
}

impl<'a> OpTcgScraper<'a> {
    pub fn new(localizer: &Localizer) -> OpTcgScraper {
        OpTcgScraper {
            base_url: localizer.hostname.clone(),
            localizer,
        }
    }

    fn cardlist_endpoint(&self) -> String {
        format!("{}/{}", self.base_url, "cardlist")
    }

    fn get_img_full_url(&self, img_url: &str) -> String {
        let short_img_url = &img_url[3..];
        let full_url = format!("{}/{}", self.base_url, short_img_url);
        debug!("full url: {}", full_url);

        full_url
    }

    pub fn fetch_all_packs(&self) -> Result<Vec<Pack>> {
        let url = self.cardlist_endpoint();
        info!("GET `{}`", url);

        let response = reqwest::blocking::get(url)?.text()?;

        info!("parsing HTML document");
        let document = scraper::Html::parse_document(&response);

        let sel = "div.seriesCol>select#series>option";
        info!("fetching series (packs) ({})...", sel);

        let series_selector = scraper::Selector::parse(sel).unwrap();

        let mut packs = Vec::new();
        for element in document.select(&series_selector) {
            match Pack::new(element) {
                Ok(pack) => {
                    if !pack.id.is_empty() {
                        packs.push(pack);
                    }
                }
                Err(e) => bail!("failed to scrape data about packs: {}", e),
            }
        }

        info!("processed packs");
        Ok(packs)
    }

    pub fn fetch_all_cards(&self, pack_id: &str) -> Result<Vec<Card>> {
        let url = self.cardlist_endpoint();
        info!("GET `{}`", url);

        let mut params = HashMap::new();
        params.insert("series", pack_id);

        let client = Client::new();
        let response = client
            .get(self.cardlist_endpoint())
            .query(&params)
            .send()?
            .text()?;

        info!("parsing HTML document");
        let document = scraper::Html::parse_document(&response);

        let sel = "div.resultCol>a";
        info!("fetching cards for pack `{}` ({})...", pack_id, sel);

        let card_ids_selector = scraper::Selector::parse(sel).unwrap();

        let mut cards = Vec::new();
        for element in document.select(&card_ids_selector) {
            let card_id = element
                .attr("data-src")
                .context("expected `data-src` attr on <a>")?
                .to_string();

            let card_id = &card_id[1..];

            match CardScraper::create_card(self.localizer, &document, card_id, pack_id) {
                Ok(mut card) => {
                    debug!("computing img_full_url for card: {}", card);
                    card.img_full_url = Some(self.get_img_full_url(&card.img_url));
                    cards.push(card);
                }
                Err(e) => {
                    bail!("failed to scrape data about card `{}`: {}", &card_id, e)
                }
            };
        }

        info!("processed cards for pack `{}`", pack_id);
        Ok(cards)
    }

    pub fn download_card_image(&self, card: &Card) -> Result<Response> {
        let full_url = self.get_img_full_url(&card.img_url);

        debug!("downloading image `{}`...", full_url);
        let response = reqwest::blocking::get(full_url)?;
        Ok(response)
    }
}
