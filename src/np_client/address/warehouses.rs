use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, NoneAsEmptyString};
use uuid::Uuid;

use crate::np_client::helper_structs::Coordinates;
use crate::np_client::{NPClient, NPRequest};
use crate::np_client::res_template::ResponseTemplate;
use crate::np_client::date_format::common_date_format;
use crate::np_client::deserializer::deserialize_bool_from_str_num;

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

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetWarehousesBuilder<'cli> {
    #[serde(skip)]
    client: &'cli NPClient,

    #[serde(skip_serializing_if = "Option::is_none")]
    city_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    city_ref: Option<Uuid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    bicycle_parking: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    warehouse_id: Option<Uuid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    type_of_warehouse_ref: Option<Uuid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u16>,
}


impl<'cli> GetWarehousesBuilder<'cli> {
    pub fn new(client: &'cli NPClient) -> Self {
        Self {
            client,
            city_name: None,
            city_ref:None,
            warehouse_id: None,
            page: None,
            limit: None,
            bicycle_parking: None,
            type_of_warehouse_ref: None,
        }
    }

    pub fn page(mut self, page: u16) -> Self {
        self.page = Some(page);
        self
    }

    pub fn limit(mut self, limit: u16) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn warehouse_id(mut self, warehouse_id: Uuid) -> Self {
        self.warehouse_id = Some(warehouse_id);
        self
    }

    pub fn city_name(mut self, name: String) -> Self {
        self.city_name = Some(name);
        self
    }

    pub fn city_ref(mut self, c_ref: Uuid) -> Self {
        self.city_ref = Some(c_ref);
        self
    }

    pub fn type_of_warehouse_ref(mut self, w_ref: Uuid) -> Self {
        self.type_of_warehouse_ref = Some(w_ref);
        self
    }

    pub fn bicycle_parking(mut self, bicycle_parking: u16) -> Self {
        self.bicycle_parking = Some(bicycle_parking);
        self
    }

    pub async fn send(self) -> Result<ResponseTemplate<Warehouse>, reqwest::Error> {
        let request = NPRequest {
            api_key: &self.client.api_key,
            model_name: "Address",
            called_method: "getWarehouses",
            method_properties: &self
        };
        let url = (&self.client.base_url).to_owned();
        self.client.http_client
            .post(url)
            .json(&request)
            .send()
            .await?
            .json()
            .await
    }
}
