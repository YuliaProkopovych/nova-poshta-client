use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Settlement {
    #[serde(rename = "Present")]
    full_name: String,
    #[serde(rename = "Warehouses")]
    warehouses_number: u16,
    main_description: String,
    area: String,
    region: String,
    settlement_type_code: String,
    r#ref: Uuid,
    delivery_city: Uuid,
    address_delivery_allowed: bool,
    streets_availability: bool,
    parent_region_types: String,
    parent_region_code: String,
    region_types: String,
    region_types_code: String,
}