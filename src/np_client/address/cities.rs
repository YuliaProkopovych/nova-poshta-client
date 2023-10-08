use serde::{Deserialize, Serialize};
use serde_with::{serde_as, BoolFromInt, DisplayFromStr, NoneAsEmptyString};
use uuid::Uuid;

use crate::np_client::{deserializer::deserialize_bool_from_str_num, NPClient, NPRequest, res_template::ResponseTemplate};

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
    #[serde(rename = "CityID")]
    #[serde_as(as = "DisplayFromStr")]
    city_id: u16,
    settlement_type_description: String,
    settlement_type_description_ru: String,
    #[serde_as(as = "BoolFromInt")]
    special_cash_check: bool,
    area_description: String,
    area_description_ru: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetCitiesBuilder<'cli> {
    #[serde(skip)]
    client: &'cli NPClient,

    #[serde(skip_serializing_if = "Option::is_none")]
    city_ref: Option<Uuid>,

    #[serde(skip_serializing_if = "Option::is_none")]
    find_by_string: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u16>,
}


impl<'cli> GetCitiesBuilder<'cli> {
    pub fn new(client: &'cli NPClient) -> Self {
        Self {
            client,
            city_ref:None,
            find_by_string: None,
            page: None,
            limit: None,
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

    pub fn find_by_string(mut self, search_val: String) -> Self {
        self.find_by_string = Some(search_val);
        self
    }

    pub fn city_ref(mut self, c_ref: Uuid) -> Self {
        self.city_ref = Some(c_ref);
        self
    }

    pub async fn send(self) -> Result<ResponseTemplate<City>, reqwest::Error> {
        let request = NPRequest {
            api_key: &self.client.api_key,
            model_name: "Address",
            called_method: "getCities",
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