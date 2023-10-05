use serde::{Deserialize, Serialize};
use serde_with::{
    serde_as,
    NoneAsEmptyString,
    DisplayFromStr,
    BoolFromInt,
};
use uuid::Uuid;

use super::deserializer::deserialize_bool_from_str_num;

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct City {
    description: String,
    description_ru: String,
    r#ref: Uuid,
    #[serde_as(as = "NoneAsEmptyString")]
    delivery1: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    delivery2: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    delivery3: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    delivery4: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    delivery5: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    delivery6: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    delivery7: Option<String>,
    area: Uuid,
    settlement_type: Uuid,
    #[serde(deserialize_with = "deserialize_bool_from_str_num")]
    is_branch: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_num")]
    prevent_entry_new_streets_user: bool,
    #[serde_as(as = "DisplayFromStr")]
    city_ID: u16,
    settlement_type_description: String,
    settlement_type_description_ru: String,
    #[serde_as(as = "BoolFromInt")]
    special_cash_check: bool,
    area_description: String,
    area_description_ru: String,
}