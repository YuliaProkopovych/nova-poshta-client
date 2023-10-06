use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum CounterpartyType {
    PrivatePerson,
    Organization,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CounterpartyRole {
    Sender,
    Recipient,
    ThirdPerson,
}

impl TryFrom<&str> for CounterpartyRole {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, String> {
        match value {
            "Sender" => Ok(Self::Sender),
            "Recepient" => Ok(Self::Recipient),
            "ThirdPerson" => Ok(Self::ThirdPerson),
            _ => Err("Invalid value".to_owned())
        }
    }
}