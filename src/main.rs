use anyhow::Result;
use chrono::Utc;
use clap::Parser;
use cli::Cli;
use localizer::Localizer;
use log::info;

use op_scraper::OpTcgScraper;

mod card;
mod card_scraper;
mod cli;
mod localizer;
mod op_data;
mod op_scraper;
mod pack;

fn main() -> Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    process_args(args)?;
    Ok(())
}

fn process_args(args: Cli) -> Result<()> {
    let localizer = Localizer::load(args.language)?;
    let scraper = OpTcgScraper::new(&localizer);

    match args.command {
        cli::Commands::Packs => list_packs(&scraper),
        cli::Commands::Cards { pack_id } => list_cards(&scraper, &pack_id.to_string_lossy()),
    }
}

fn list_packs(scraper: &OpTcgScraper) -> Result<()> {
    info!("fetching all pack ids...");
    let start_time = Utc::now();

    let packs = scraper.fetch_all_packs()?;
    info!("successfully fetched {} packs!", packs.len());

    let json = serde_json::to_string(&packs)?;
    println!("{}", json);

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

    let json = serde_json::to_string(&cards)?;
    println!("{}", json);

    let end_time = Utc::now();

    info!("Time, start: {}, end: {}", start_time, end_time);
    Ok(())
}
