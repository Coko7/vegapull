use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

use crate::localizer::Localizer;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum CardAttribute {
    Slash,
    Strike,
    Ranged,
    Special,
    Wisdom,
}

impl CardAttribute {
    pub fn parse(localizer: &Localizer, value: &str) -> Result<CardAttribute> {
        match localizer.match_attribute(value.trim()) {
            Some(key) => Ok(Self::from_str(&key)?),
            None => bail!("Failed to match attribute `{}`", value),
        }
    }

    pub fn from_str(value: &str) -> Result<CardAttribute> {
        match value.to_lowercase().as_str() {
            "slash" => Ok(Self::Slash),
            "strike" => Ok(Self::Strike),
            "ranged" => Ok(Self::Ranged),
            "special" => Ok(Self::Special),
            "wisdom" => Ok(Self::Wisdom),
            _ => bail!("Unsupported attribute `{}`", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_slash_returns_ok() {
        assert_eq!(
            CardAttribute::from_str("SLash").unwrap(),
            CardAttribute::Slash
        );
    }

    #[test]
    fn from_str_strike_returns_ok() {
        assert_eq!(
            CardAttribute::from_str("strIKE").unwrap(),
            CardAttribute::Strike
        );
    }

    #[test]
    fn from_str_ranged_returns_ok() {
        assert_eq!(
            CardAttribute::from_str("rANged").unwrap(),
            CardAttribute::Ranged
        );
    }

    #[test]
    fn from_str_special_returns_ok() {
        assert_eq!(
            CardAttribute::from_str("spEciAl").unwrap(),
            CardAttribute::Special
        );
    }

    #[test]
    fn from_str_wisdom_returns_ok() {
        assert_eq!(
            CardAttribute::from_str("wiSDom").unwrap(),
            CardAttribute::Wisdom
        );
    }

    #[test]
    fn from_str_invalid_returns_err() {
        assert!(CardAttribute::from_str("not a valid value").is_err());
    }
}
