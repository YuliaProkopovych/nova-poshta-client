use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::np_client::{NPClient, NPRequest, res_template::ResponseTemplate};

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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Settlements {
    total_count: u16,
    addresses: Vec<Settlement>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SearchSettlementsBuilder<'cli> {
    #[serde(skip)]
    client: &'cli NPClient,

    city_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u16>,
}


impl<'cli> SearchSettlementsBuilder<'cli> {
    pub fn new(client: &'cli NPClient, search_param: String) -> Self {
        Self {
            client,
            city_name: search_param,
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

    pub async fn send(self) -> Result<ResponseTemplate<Settlements>, reqwest::Error> {
        let request = NPRequest {
            api_key: &self.client.api_key,
            model_name: "Address",
            called_method: "searchSettlements",
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