use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use serde_with::{serde_as, DisplayFromStr, NoneAsEmptyString};

use super::date_format::common_date_format;
use super::deserializer::deserialize_bool_from_str_num;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Dimensions {
    width: u16,
    height: u16,
    length: u16,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Schedule {
    monday: String,
    tuesday: String,
    wednesday: String,
    thursday: String,
    friday: String,
    saturday: String,
    sunday: String,
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

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Warehouse {
    #[serde_as(as = "DisplayFromStr")]
    site_key: u16,
    description: String,
    description_ru: String,
    short_address: String,
    short_address_ru: String,
    phone: String,
    type_of_warehouse: Uuid,
    r#ref: Uuid,
    #[serde_as(as = "DisplayFromStr")]
    number: u16,
    city_ref: Uuid,
    city_description: String,
    city_description_ru: String,
    settlement_ref: Uuid,
    settlement_description: String,
    settlement_area_description: String,
    settlement_regions_description: String,
    settlement_type_description: String,
    settlement_type_description_ru: String,
    #[serde(flatten)]
    coordinates: Coordinates,
    #[serde(deserialize_with = "deserialize_bool_from_str_num")]
    post_finance: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_num")]
    bicycle_parking: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_num")]
    payment_access: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_num")]
    POS_terminal: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_num")]
    international_shipping: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_num")]
    self_service_workplaces_count: bool,
    #[serde_as(as = "DisplayFromStr")]
    total_max_weight_allowed: u16,
    #[serde_as(as = "DisplayFromStr")]
    place_max_weight_allowed: u16,
    sending_limitations_on_dimensions: Dimensions,
    receiving_limitations_on_dimensions: Dimensions,
    reception: Schedule,
    delivery: Schedule,
    schedule: Schedule,
    district_code: String,
    warehouse_status: String,
    #[serde(with = "common_date_format")]
    warehouse_status_date: Option<NaiveDateTime>,
    //warehouse_illusha: String,
    category_of_warehouse: String,
    #[serde_as(as = "NoneAsEmptyString")]
    direct: Option<String>,
    region_city: String,
    #[serde(deserialize_with = "deserialize_bool_from_str_num")]
    warehouse_for_agent: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_num")]
    generator_enabled: bool,
    #[serde_as(as = "DisplayFromStr")]
    max_declared_cost: f32,
    #[serde(deserialize_with = "deserialize_bool_from_str_num")]
    work_in_mobile_awis: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_num")]
    deny_to_select: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_num")]
    can_get_money_transfer: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_num")]
    has_mirror: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_num")]
    has_fitting_room: bool,
    #[serde(deserialize_with = "deserialize_bool_from_str_num")]
    only_receiving_parcel: bool,
    #[serde_as(as = "NoneAsEmptyString")]
    post_machine_type: Option<String>,
    postal_code_UA: String,
    warehouse_index: String,
    #[serde_as(as = "NoneAsEmptyString")]
    beacon_code: Option<String>,
}
