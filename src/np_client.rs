use reqwest::{Client, Response};
use serde_with::{serde_as, skip_serializing_none};
use url::Url;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

mod date_format;
mod counterparty;
mod helper_structs;
mod en;
mod tracking;
mod cities;
mod deserializer;
mod warehouses;
mod settlements;
mod res_template;
use helper_structs::{CounterpartyType, CounterpartyRole};
use warehouses::Warehouse;
use en::ENumber;
use cities::City;
use tracking::{Document, TrackingDoc};
use settlements::{Settlements, Streets};
use counterparty::Counterparty;

#[serde_as]
#[derive(Debug, Serialize)]
#[skip_serializing_none]
#[serde(
    tag = "calledMethod",
    content = "methodProperties",
    rename_all_fields = "PascalCase",
    rename_all = "camelCase"
)]
pub enum Method<'a> {
    Save { 
        counterparty_property: CounterpartyRole,
        counterparty_type: CounterpartyType,
        first_name: &'a str,
        middle_name: &'a str,
        last_name: &'a str,
        email: &'a str,
        phone: &'a str,
    },
    GetCounterparties {
        page: Option<u16>,
        find_by_string: Option<&'a str>,
        counterparty_property: CounterpartyRole,
    },
    GetStatusDocuments { documents: Vec<Document> },
    GetWarehouses {
        city_name: Option<String>,
        city_ref: Option<Uuid>,
        page: Option<u16>,
        limit: Option<u16>,
        type_of_warehouse_ref: Option<Uuid>,
        warehouse_id: Option<u16>,
        #[serde(skip_serializing_if = "Option::is_none")]
        bicycle_parking: Option<u16>,
    },
    GetCities {
        r#ref: Option<Uuid>,
        page: Option<u16>,
        find_by_string: Option<&'a str>,
        limit: Option<u16>,
    },
    SearchSettlements {
        city_name: &'a str,
        limit: u16,
        page: u16,
    },
    SearchSettlementStreets {
        street_name: &'a str,
        settlement_ref: Uuid,
        limit: Option<u16>,
    },
}

const URL: &str = "https://api.novaposhta.ua/v2.0/json/";

#[derive(Clone, Debug)]
pub struct NPClient {
    http_client: Client,
    base_url: Url,
    api_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseTemplate<NPData> {
    pub success: bool,
    pub data: Vec<NPData>,
    pub errors: Vec<String>,
    pub warnings: Vec<serde_json::Value>,
    pub info: serde_json::Value,
    pub message_codes: Vec<String>,
    pub error_codes: Vec<String>,
    pub warning_codes: Vec<String>,
    pub info_codes: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub enum Models {
    TrackingDocument,
    Address,
    Counterparty,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NPRequest<'a> {
    api_key: &'a str,
    model_name: Models,
    #[serde(flatten)]
    called_method: Method<'a>,
}

impl NPClient {
    pub fn default() -> Result<Self, reqwest::Error> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()?;

        let base_url = Url::parse(URL).unwrap();

        Ok(Self {
            http_client: client,
            base_url,
            api_key: String::new(),
        })
    }

    pub fn with_api_key(api_key: String) -> Result<Self, reqwest::Error> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(5))
            .build()?;

        let base_url = Url::parse(URL).unwrap();

        Ok(Self {
            http_client: client,
            base_url,
            api_key,
        })
    }

    pub fn base_url(&mut self, url: &str) {
        self.base_url = Url::parse(url).unwrap();
    }

    async fn send_request<'a>(
        &self,
        model_name: Models,
        called_method: Method<'a>,
    ) -> Result<Response, reqwest::Error> {
        let request = NPRequest {
            api_key: &self.api_key,
            model_name,
            called_method,
        };

        self.http_client
            .post(self.base_url.to_owned())
            .json(&request)
            .send()
            .await
    }

    pub async fn search_settlements(
        &self, 
        city_name: &str, 
        page: u16, 
        limit: u16,
    ) -> Result<ResponseTemplate<Settlements>, reqwest::Error> {
        let res = self
            .send_request(
                Models::Address,
                Method::SearchSettlements {
                    page,
                    limit,
                    city_name,
                },
            )
            .await?;
        let res_data = res.json().await;
        res_data
    }

    pub async fn search_streets(
        &self, 
        street_name: &str, 
        settlement_ref: Uuid, 
        limit: Option<u16>,
    ) -> Result<ResponseTemplate<Streets>, reqwest::Error> {
        let res = self
        .send_request(
            Models::Address,
            Method::SearchSettlementStreets { 
                street_name, 
                settlement_ref, 
                limit,
            } ,
        )
        .await?;

        let res_data = res.json().await;
        res_data
    }

    pub async fn get_tracking(&self, en: String, phone_number: String) -> Result<ResponseTemplate<TrackingDoc>, reqwest::Error> {
        let res = self
            .send_request(
                Models::TrackingDocument,
                Method::GetStatusDocuments {
                    documents: vec![Document::new(
                        ENumber::try_from(en).unwrap(),
                        phone_number.to_owned(),
                    )],
                },
            )
            .await?;

        let res_data = res.json().await;
        res_data
    }

    pub async fn get_cities(
        &self, 
        page: Option<u16>, 
        limit: Option<u16>, 
        city_ref: Option<Uuid>, 
        find_by_string: Option<&str>,
    ) -> Result<ResponseTemplate<City>, reqwest::Error> {
        let res = self
            .send_request(
                Models::Address,
                Method::GetCities {
                    page,
                    limit,
                    r#ref: city_ref,
                    find_by_string,
                },
            )
            .await?;
        let res_data = res.json().await;
        res_data
    }

    pub async fn get_warehouses(
        &self,                 
        city_name: Option<String>,
        city_ref: Option<Uuid>,
        page: Option<u16>,
        limit: Option<u16>,
        bicycle_parking: Option<u16>,
        type_of_warehouse_ref: Option<Uuid>,
        warehouse_id: Option<u16>,
    ) -> Result<ResponseTemplate<Warehouse>, reqwest::Error> {
        let res = self
            .send_request(
                Models::Address,
                Method::GetWarehouses { 
                    city_name,
                    city_ref,
                    page,
                    limit,
                    bicycle_parking,
                    type_of_warehouse_ref,
                    warehouse_id,
                },
            )
            .await?;

        let res_data = res.json().await;
        res_data
    }

    pub async fn create_counterparty(
        &self, 
        first_name: &str,
        middle_name: &str,
        last_name: &str,
        phone: &str,
        email: &str,
    ) -> Result<ResponseTemplate<Counterparty>, reqwest::Error> {

        let res = self
        .send_request(
            Models::Counterparty,
            Method::Save { 
                counterparty_property: CounterpartyRole::Recipient,
                counterparty_type: CounterpartyType::PrivatePerson,
                first_name,
                middle_name,
                last_name,
                email,
                phone,
            },
        )
        .await?;

        let res_data = res.json().await;
        res_data
    }

    pub async fn get_counterparties(
        &self, 
        find_by_string: Option<&str>,
        page: Option<u16>,
        counterparty_property: &str,
    ) ->  Result<ResponseTemplate<Counterparty>, reqwest::Error> {

        let res = self
        .send_request(
            Models::Counterparty,
            Method::GetCounterparties { 
                page, 
                find_by_string, 
                counterparty_property: CounterpartyRole::try_from(counterparty_property).unwrap(), 
            }
        )
        .await?;

        let res_data = res.json().await;
        res_data
    }
}