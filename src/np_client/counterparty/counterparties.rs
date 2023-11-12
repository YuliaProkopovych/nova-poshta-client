use serde::Serialize;

use crate::np_client::{
    NPClient, 
    res_template::ResponseTemplate, 
    helper_structs::CounterpartyRole, 
    NPRequest
};

use super::Counterparty;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetCounterpartiesBuilder<'cli> {
    #[serde(skip)]
    client: &'cli NPClient,

    #[serde(skip_serializing_if = "Option::is_none")]
    find_by_string: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    counterparty_property: Option<CounterpartyRole>,
}


impl<'cli> GetCounterpartiesBuilder<'cli> {
    pub fn new(client: &'cli NPClient) -> Self {
        Self {
            client,
            find_by_string: None,
            page: None,
            counterparty_property: None,
        }
    }

    pub fn counterparty_property(mut self, prop: CounterpartyRole) -> Self {
        self.counterparty_property = Some(prop);
        self
    }

    pub fn find_by_string(mut self, val: String) -> Self {
        self.find_by_string = Some(val);
        self
    }

    pub fn page(mut self, page: u16) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn send(self) -> Result<ResponseTemplate<Counterparty>, reqwest::Error> {
        let request = NPRequest {
            api_key: &self.client.api_key,
            model_name: "Counterparty",
            called_method: "getCounterparties",
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
