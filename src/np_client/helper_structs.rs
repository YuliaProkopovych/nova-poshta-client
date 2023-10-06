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