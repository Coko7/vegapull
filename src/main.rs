use core::panic;
use std::ffi::OsString;

use anyhow::Result;
use chrono::Utc;
use clap::Parser;
use cli::Cli;
use localizer::Localizer;
use log::info;

use op_scraper::OpTcgScraper;

mod card;
mod card_scraper;
mod card_set;
mod cli;
mod localizer;
mod op_data;
mod op_scraper;

fn main() -> Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    process_args(args)?;

    // let localizer = Localizer::load_from_file("en")?;
    //
    // match scrap_all_cards(&localizer) {
    //     Ok(()) => (),
    //     Err(error) => {
    //         error!("failed to scrap cards data: {}", error);
    //     }
    // }

    // match download_all_images(&localizer) {
    //     Ok(()) => (),
    //     Err(error) => {
    //         error!("failed to download card images: {}", error);
    //     }
    // }

    Ok(())
}

fn process_args(args: Cli) -> Result<()> {
    // let language = args.language;
    let localizer = Localizer::load_from_file("en")?;
    let scraper = OpTcgScraper::new(&localizer);

    match args.command {
        cli::Commands::Packs => list_packs(&scraper),
        cli::Commands::Cards { pack_id } => list_cards(&scraper, &pack_id.to_string_lossy()),
        cli::Commands::Image { card_id } => get_image(&scraper, &card_id.to_string_lossy()),
    }
}

fn list_packs(scraper: &OpTcgScraper) -> Result<()> {
    info!("fetching all pack ids...");
    let start_time = Utc::now();

    let packs = scraper.fetch_all_packs()?;
    info!("successfully fetched {} packs!", packs.len());

    for pack in packs {
        println!("{}", pack);
    }

    let end_time = Utc::now();

    info!("Time, start: {}, end: {}", start_time, end_time);
    Ok(())
}

fn list_cards(scraper: &OpTcgScraper, pack_id: &str) -> Result<()> {
    info!("fetching all cards...");
    let start_time = Utc::now();

    let cards = scraper.fetch_all_cards(&pack_id)?;
    info!(
        "successfully fetched {} cards for pack: `{}`!",
        cards.len(),
        pack_id
    );

    for card in cards {
        println!("{}", card);
    }

    let end_time = Utc::now();

    info!("Time, start: {}, end: {}", start_time, end_time);
    Ok(())
}

fn get_image(scraper: &OpTcgScraper, card_id: &str) -> Result<()> {
    panic!("Not Yet Implemented");
}
