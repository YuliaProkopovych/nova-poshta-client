use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::np_client::{NPClient, res_template::ResponseTemplate, NPRequest};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct ContactPerson {
    r#ref: Uuid,
    description: String,
    first_name: String,
    middle_name: String,
    last_name: String,
    phones: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct GetContactPersonsBuilder<'cli> {
    #[serde(skip)]
    client: &'cli NPClient,

    counterparty_ref: Uuid,

    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<u16>,
}


impl<'cli> GetContactPersonsBuilder<'cli> {
    pub fn new(client: &'cli NPClient, counterparty_ref: Uuid) -> Self {
        Self {
            client,
            counterparty_ref,
            page: None,
        }
    }

    pub fn page(mut self, page: u16) -> Self {
        self.page = Some(page);
        self
    }

    pub async fn send(self) -> Result<ResponseTemplate<ContactPerson>, reqwest::Error> {
        let request = NPRequest {
            api_key: &self.client.api_key,
            model_name: "Counterparty",
            called_method: "getCounterpartyContactPersons",
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

