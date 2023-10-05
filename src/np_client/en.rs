use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ENumber(String);

impl From<ENumber> for String {
    fn from(en: ENumber) -> Self {
        en.0
    }
}

impl TryFrom<String> for ENumber {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 14 {
            return Err("Invalid value".to_owned());
        }
        let email_regex = Regex::new(r"[0-9+]").unwrap();

        if email_regex.is_match(&value) {
            Ok(Self(value))
        } else {
            Err("Invalid value".to_owned())
        }
    }
}

impl TryFrom<&str> for ENumber {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let email_regex = Regex::new(r"[0-9+]").unwrap();

        if email_regex.is_match(value) {
            Ok(Self(value.to_owned()))
        } else {
            Err("Cannot convert from string".to_owned())
        }
    }
}

impl AsRef<str> for ENumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
