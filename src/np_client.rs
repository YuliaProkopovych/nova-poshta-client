use reqwest::Client;
use serde::Serialize;
use url::Url;

pub mod counterparty;
mod date_format;
mod deserializer;
mod address;
pub mod en;
pub mod helper_structs;
pub mod res_template;
mod settlements;
mod tracking;

const URL: &str = "https://api.novaposhta.ua/v2.0/json/";

#[derive(Clone, Debug)]
pub struct NPClient {
    http_client: Client,
    base_url: Url,
    api_key: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NPRequest<'a, T: Serialize> {
    api_key: &'a str,
    model_name: &'a str,
    called_method: &'a str,
    method_properties: T,
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

    pub fn address(&self) -> address::AddressHandler {
        address::AddressHandler::new(self)
    }

    pub fn tracking(&self) -> tracking::TrackingHandler {
        tracking::TrackingHandler::new(self)
    }

    pub fn counterparty(&self) -> counterparty::CounterpartyHandler {
        counterparty::CounterpartyHandler::new(self)
    }

}
