use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::np_client::{NPClient, res_template::ResponseTemplate, helper_structs::CounterpartyRole, NPRequest};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CounterpartyAddress {
    r#ref: Uuid,
    description: String,
    city_ref: Uuid,
    city_description: String,
    street_ref: Uuid,
    street_description: String,
    building_ref: Uuid,
    building_description: String,
    note: String,
    address_name: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetAddressesBuilder<'cli> {
    #[serde(skip)]
    client: &'cli NPClient,

    counterparty_ref: Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    counterparty_property: Option<CounterpartyRole>,
}


impl<'cli> GetAddressesBuilder<'cli> {
    pub fn new(client: &'cli NPClient, counterparty_ref: Uuid) -> Self {
        Self {
            client,
            counterparty_ref,
            counterparty_property: None,
        }
    }

    pub fn counterparty_property(mut self, prop: CounterpartyRole) -> Self {
        self.counterparty_property = Some(prop);
        self
    }

    pub async fn send(self) -> Result<ResponseTemplate<CounterpartyAddress>, reqwest::Error> {
        let request = NPRequest {
            api_key: &self.client.api_key,
            model_name: "Counterparty",
            called_method: "getCounterpartyAddresses",
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
