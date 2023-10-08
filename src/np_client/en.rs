use std::str::FromStr;

use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum ENumberPasingError {
    #[error("Invalid value length")]
    InvalidLength,
    #[error("Value contains unexpected symbols")]
    InvalidSymbols,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ENumber(String);

impl From<ENumber> for String {
    fn from(en: ENumber) -> Self {
        en.0
    }
}

impl FromStr for ENumber {
    type Err = ENumberPasingError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 14 {
            return Err(ENumberPasingError::InvalidLength);
        }
        let en_regex = Regex::new(r"[0-9+]").unwrap();

        if en_regex.is_match(&value) {
            Ok(Self(value.to_owned()))
        } else {
            Err(ENumberPasingError::InvalidSymbols)
        }
    }
}

impl AsRef<str> for ENumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
