# 👒 vegapull

![build](https://github.com/coko7/vegapull/actions/workflows/rust.yml/badge.svg)

A CLI utility to retrieve data for the One Piece Trading Cards Game (TCG).

The CLI directly goes against the [onepiece-cardgame.com](https://onepiece-cardgame.com) website and attempts to scrap information such as packs, cards and images.

## Where is the data?

For practical reasons, data is kept in a separate repository: https://github.com/Coko7/vegapull-records

## How to use?

1. Install / build from source:
```console
coko7@example:~$ git clone https://github.com/Coko7/vegapull.git
coko7@example:~$ cd vegapull 
coko7@example:~$ cargo build --release
```
2. Use
```console
coko7@example:~$ vegapull -h
Dynamically fetch data for the One Piece TCG from official sites.

Usage: vegapull [OPTIONS] <COMMAND>

Commands:
  packs        Get the list of all existing packs
  cards        Get all cards within the given pack
  images       Download all card images for a given pack
  inter        Launch into interactive mode
  test-config  Test what configuration files are found
  help         Print this message or the help of the given subcommand(s)

Options:
  -l, --language <LANGUAGE>
          Language to use for the data [default: english] [possible values: chinese-hongkong, chinese-simplified, chinese-taiwan, english, english-asia, japanese, thai]
  -c, --config-dir <CONFIG_DIRECTORY_PATH>
          Specify path to the config directory (where locales are stored)
  -v, --verbose...
          Increase logging verbosity
  -q, --quiet...
          Decrease logging verbosity
  -h, --help
          Print help
```

## Helper Scripts
You can use the example scripts that directly use the `vegapull` CLI to download data for all existing packs:

### Bash
```console
coko7@example:~$ bash scripts/pull-all.sh
// the `gum` one is more complete but requires some additional tooling to install in your shell:
coko7@example:~$ bash scripts/pull-all-gum.sh
```

### Go
> Requires [Go](https://go.dev/) to be installed.

```console
➜ go run scripts/pull.go
```

## 🃏 Supported card fields

```rust
#[derive(Debug, Deserialize, Serialize)]
pub struct Card {
    pub id: String,
    pub name: String,
    pub rarity: CardRarity,
    pub category: CardCategory,
    // pub number: i32,
    // #[serde(skip_serializing)]
    // pub set_id: String,
    // pub copyright: String,

    // Images
    pub img_url: String,
    // pub illustration: CardIllustration,
    // pub illustrator_name: String,

    // Gameplay
    pub colors: Vec<CardColor>,
    pub cost: Option<i32>, // Only Character, Event and Stage (called life for Leader)
    pub attributes: Vec<CardAttribute>, // Only Leader and Character
    pub power: Option<i32>, // Only Leader and Character
    pub counter: Option<i32>, // Only Character

    pub types: Vec<String>,
    pub effect: String,
    pub trigger: Option<String>,
    // pub notes: String,
}
```
Fields have been named following the terms used in the official [rule book](https://en.onepiece-cardgame.com/pdf/rule_comprehensive.pdf)

## 🐛 Issues

When using `jp` locale to fetch data, the scraper will likely fail when handling `counter` or `colors` values for some cards.

## 🗺️ Road Map

- [x] Fetch card sets data
- [x] Better error handling
- [x] Fetch cards data for each card set (wip)
- [x] Get card data for all card sets
- [x] Organize and save cards data as JSON to files
- [x] Add logs
- [x] Support more card fields
- [x] Download card images as well
- [x] Make it locale-agnostic to be able to download data from Japanese and other versions
    - [ ] Handle problems with the `jp` version (inconsistent cards data on official site)
- [ ] Better configuration 
- [x] User friendly CLI
- [ ] Add tests
