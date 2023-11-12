use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use uuid::Uuid;

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
            _ => Err("Invalid value".to_owned()),
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Coordinates {
    #[serde(alias = "lat")]
    #[serde_as(as = "DisplayFromStr")]
    latitude: f32,
    #[serde(alias = "lon")]
    #[serde_as(as = "DisplayFromStr")]
    longitude: f32,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ServiceType {
    DoorsDoors,
    DoorsWarehouse,
    WarehouseWarehouse,
    WarehouseDoors,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PaymentMethod {
    Cash,
    Card,
    NonCash,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CargoType {
    Parcel,
    Cargo,
    Documents,
    TiresWheels,
    Pallet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CityID (Uuid);

impl FromStr for CityID {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(CityID(Uuid::parse_str(value)?))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CounterpartyID (Uuid);

impl FromStr for CounterpartyID {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(CounterpartyID(Uuid::parse_str(value)?))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactPersonID (Uuid);

impl FromStr for ContactPersonID {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(ContactPersonID(Uuid::parse_str(value)?))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddressID (Uuid);

impl FromStr for AddressID {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(AddressID(Uuid::parse_str(value)?))
    }
}