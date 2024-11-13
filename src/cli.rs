use std::ffi::OsString;

use clap::{command, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "veganet")]
#[command(
    about = "Dynamically fetch data for the One Piece TCG from official sites.",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Language to use for the data
    #[arg(short, long = "lang", value_name = "LANGUAGE", default_value_t = LanguageCode::English, value_enum)]
    pub language: LanguageCode,

    // /// Write data to a file instead of stdout
    // #[arg(short, long = "output", value_name = "file")]
    // pub output_file: PathBuf,
    //
    /// Outputs information in JSON
    // #[arg(short = 'j', long = "json")]
    // pub output_json: bool,

    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Get the list of all existing packs
    #[command(alias = "pack", alias = "pak")]
    Packs,
    /// Get all cards within the given pack
    #[command(alias = "card", alias = "car")]
    Cards {
        /// ID of the pack
        pack_id: OsString,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub enum LanguageCode {
    #[value(name = "english", alias = "en")]
    English,
    #[value(name = "japanese", alias = "jp")]
    Japanese,
}
