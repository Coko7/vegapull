use anyhow::{anyhow, Context};
use log::{debug, trace};
use regex::Regex;
use scraper::{ElementRef, Html};

use crate::card::{Card, CardAttribute, CardCategory, CardColor, CardRarity};

pub struct CardScraper {}

impl CardScraper {
    pub fn create_card(document: &Html, card_id: &str) -> Result<Card, anyhow::Error> {
        trace!("start create card: `{}`", card_id);
        let dl_elem = Self::get_dl_node(document, card_id.to_string())?;

        let id = Self::fetch_id(dl_elem)?;
        let name = Self::fetch_name(dl_elem)?;
        let rarity = Self::fetch_rarity(dl_elem)?;
        let category = Self::fetch_category(dl_elem)?;
        let img_url = Self::fetch_img_url(dl_elem)?;
        let colors = Self::fetch_colors(dl_elem)?;
        let cost = Self::fetch_cost(dl_elem)?;
        let attributes = Self::fetch_attributes(dl_elem)?;
        let power = Self::fetch_power(dl_elem)?;
        let counter = Self::fetch_counter(dl_elem)?;
        let types = Self::fetch_types(dl_elem)?;
        let effect = Self::fetch_effect(dl_elem)?;
        let trigger = Self::fetch_trigger(dl_elem)?;

        let card = Card {
            id,
            name,
            rarity,
            category,
            img_url,
            colors,
            cost,
            attributes,
            power,
            counter,
            types,
            effect,
            trigger,
        };

        debug!("processed card: `{}`", card);
        Ok(card)
    }

    // element is top level <dl> tag
    pub fn fetch_id(element: ElementRef) -> Result<String, anyhow::Error> {
        trace!("fetching card.id...");
        let id = element
            .attr("id")
            .context("expected to find id attr on <dl>")?
            .to_string();

        trace!("fetched card.id: {}", id);
        Ok(id)
    }

    pub fn fetch_name(element: ElementRef) -> Result<String, anyhow::Error> {
        let sel = "dt>div.cardName";
        trace!("fetching card.name ({})...", sel);

        let name = Self::get_child_node(element, sel.to_string())?.inner_html();

        trace!("fetched card.name: {}", name);
        Ok(name)
    }

    pub fn fetch_rarity(element: ElementRef) -> Result<CardRarity, anyhow::Error> {
        let sel = "dt>div.infoCol>span:nth-child(2)";
        trace!("fetching card.rarity ({})...", sel);

        let raw_rarity = Self::get_child_node(element, sel.to_string())?.inner_html();

        trace!("fetched card.rarity: {}", raw_rarity);
        let rarity = CardRarity::from_str(&raw_rarity)?;

        trace!("processed card.rarity");
        Ok(rarity)
    }

    pub fn fetch_category(element: ElementRef) -> Result<CardCategory, anyhow::Error> {
        let sel = "dt>div.infoCol>span:nth-child(3)";
        trace!("fetching card.category ({})...", sel);

        let raw_category = Self::get_child_node(element, sel.to_string())?.inner_html();

        trace!("fetched card.category: {}", raw_category);
        let category = CardCategory::from_str(&raw_category)?;

        trace!("processed card.category");
        Ok(category)
    }

    pub fn fetch_img_url(element: ElementRef) -> Result<String, anyhow::Error> {
        let sel = "dd>div.frontCol>img";
        trace!("fetching card.img_url ({})...", sel);

        let img_elem = Self::get_child_node(element, sel.to_string())?;
        let img_url = img_elem
            .attr("data-src")
            .context("no data-src attr")?
            .to_string();

        trace!("fetched card.img_url: {}", img_url);
        Ok(img_url)
    }

    pub fn fetch_colors(element: ElementRef) -> Result<Vec<CardColor>, anyhow::Error> {
        let sel = "dd>div.backCol>div.color";
        trace!("fetching card.colors ({})...", sel);

        let raw_colors = Self::get_child_node(element, sel.to_string())?.inner_html();
        let raw_colors = Self::strip_html_tags(&raw_colors)?;
        trace!("fetched card.colors: {}", raw_colors);

        let raw_colors: Vec<&str> = raw_colors.split('/').collect();

        let mut colors = Vec::new();
        for (index, raw_color) in raw_colors.iter().enumerate() {
            trace!("processing card.colors[{}]: {}", index, raw_color);
            let color = CardColor::from_str(&raw_color)?;
            colors.push(color);
        }

        trace!("prcessed card.colors");
        Ok(colors)
    }

    pub fn fetch_cost(element: ElementRef) -> Result<Option<i32>, anyhow::Error> {
        let sel = "dd>div.backCol>div.col2>div.cost";
        trace!("fetching card.cost ({})...", sel);

        let raw_cost = Self::get_child_node(element, sel.to_string())?.inner_html();
        let raw_cost = Self::strip_html_tags(&raw_cost)?;
        trace!("fetched card.cost: {}", raw_cost);

        if raw_cost == "-" {
            trace!("card.cost unset");
            return Ok(None);
        }

        let cost: i32 = raw_cost.parse()?;

        trace!("processed card.cost");
        Ok(Some(cost))
    }

    pub fn fetch_attributes(element: ElementRef) -> Result<Vec<CardAttribute>, anyhow::Error> {
        let sel = "dd>div.backCol>div.col2>div.attribute>i";
        trace!("fetching card.attributes ({})...", sel);

        let raw_attributes = Self::get_child_node(element, sel.to_string())?.inner_html();
        trace!("fetched card.attributes: {}", raw_attributes);

        if raw_attributes.is_empty() {
            trace!("card.attributes empty");
            return Ok(Vec::new());
        }

        let raw_attributes: Vec<&str> = raw_attributes.split('/').collect();

        let mut attributes = Vec::new();
        for (index, raw_attribute) in raw_attributes.iter().enumerate() {
            trace!("processing card.attributes[{}]: {}", index, raw_attribute);
            let attribute = CardAttribute::from_str(&raw_attribute)?;
            attributes.push(attribute);
        }

        trace!("processed card.attributes");
        Ok(attributes)
    }

    pub fn fetch_power(element: ElementRef) -> Result<Option<i32>, anyhow::Error> {
        let sel = "dd>div.backCol>div.col2>div.power";
        trace!("fetching card.power ({})...", sel);

        let raw_power = Self::get_child_node(element, sel.to_string())?.inner_html();
        let raw_power = Self::strip_html_tags(&raw_power)?;
        trace!("fetched card.power: {}", raw_power);

        if raw_power == "-" {
            trace!("card.power unset");
            return Ok(None);
        }

        let power: i32 = raw_power.parse()?;

        trace!("processed card.power");
        Ok(Some(power))
    }

    pub fn fetch_counter(element: ElementRef) -> Result<Option<i32>, anyhow::Error> {
        let sel = "dd>div.backCol>div.col2>div.counter";
        trace!("fetching card.counter ({})...", sel);

        let raw_counter = Self::get_child_node(element, sel.to_string())?.inner_html();
        let raw_counter = Self::strip_html_tags(&raw_counter)?;
        trace!("fetched card.counter: {}", raw_counter);

        if raw_counter == "-" {
            trace!("card.counter unset");
            return Ok(None);
        }

        let counter: i32 = raw_counter.parse()?;

        trace!("processed card.counter");
        Ok(Some(counter))
    }

    pub fn fetch_types(element: ElementRef) -> Result<Vec<String>, anyhow::Error> {
        let sel = "dd>div.backCol>div.feature";
        trace!("fetching card.types ({})...", sel);

        let types = Self::get_child_node(element, sel.to_string())?.inner_html();
        let types = Self::strip_html_tags(&types)?;
        trace!("fetched card.types: {}", types);

        let types: Vec<String> = types.split('/').map(str::to_owned).collect();

        trace!("processed card.types");
        Ok(types)
    }

    pub fn fetch_effect(element: ElementRef) -> Result<String, anyhow::Error> {
        let sel = "dd>div.backCol>div.text";
        trace!("fetching card.effect ({})...", sel);

        let effect = Self::get_child_node(element, sel.to_string())?.inner_html();
        let effect = Self::strip_html_tags(&effect)?;
        trace!("fetched card.effect: {}", effect);

        Ok(effect)
    }

    pub fn fetch_trigger(element: ElementRef) -> Result<Option<String>, anyhow::Error> {
        let sel = "dd>div.backCol>div.trigger";
        trace!("fetching card.trigger ({})...", sel);

        if let Ok(trigger_div) = Self::get_child_node(element, sel.to_string()) {
            let trigger = trigger_div.inner_html();
            let trigger = Self::strip_html_tags(&trigger)?;
            trace!("fetched card.trigger: {}", trigger);

            return Ok(Some(trigger));
        }

        trace!("card.trigger no html found");
        Ok(None)
    }

    fn strip_html_tags(value: &str) -> Result<String, anyhow::Error> {
        let reg = Regex::new(r"<[^>]*>.*?</[^>]*>")?;
        let result = reg.replace_all(&value, "").trim().to_string();
        Ok(result)
    }

    fn get_child_node(element: ElementRef, selector: String) -> Result<ElementRef, anyhow::Error> {
        let node_sel = scraper::Selector::parse(&selector).unwrap();
        let results: Vec<_> = element.select(&node_sel).collect();

        match results.len() {
            0 => Err(anyhow!("Expected `{}` but got nothing", selector)),
            1 => Ok(*results.iter().next().unwrap()),
            _ => Err(anyhow!("Expected single `{}` but got many", selector)),
        }
    }

    pub fn get_dl_node(document: &Html, card_id: String) -> Result<ElementRef, anyhow::Error> {
        let dl_sel = format!("dl#{}", card_id);
        let dl_sel = scraper::Selector::parse(&dl_sel).unwrap();
        let dl_elem = document.select(&dl_sel).next().unwrap();

        Ok(dl_elem)
    }
}