use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::addresses::GetAddressesBuilder;
use self::contact_persons::{ContactPerson, GetContactPersonsBuilder};
use self::counterparties::GetCounterpartiesBuilder;
use self::create::{CreateCounterpartyBuilder, NoName, NoPhone, NoType, NoRole};

use super::NPClient;
use super::helper_structs::CounterpartyType;
use super::res_template::ResponseTemplate;

mod addresses;
mod contact_persons;
mod create;
mod counterparties;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Counterparty {
    r#ref: Uuid,
    description: String,
    first_name: String,
    middle_name: String,
    last_name: String,
    counterparty: Option<String>,
    ownership_form: Option<String>,
    ownership_form_ref: Option<String>,
    ownership_form_description: Option<String>,
    #[serde(rename = "EDRPOU")]
    edrpou: Option<String>,
    counterparty_type: CounterpartyType,
    contact_person: Option<ResponseTemplate<ContactPerson>>,
    city: Option<String>,
}

pub struct CounterpartyHandler<'c> {
    client: &'c NPClient,
}

impl<'cli> CounterpartyHandler<'cli> {
    pub(crate) fn new(client: &'cli NPClient) -> Self {
        Self { 
            client
        }
    }

    pub fn get_addresses(&self, cp_ref: Uuid) -> GetAddressesBuilder<'cli> {
        GetAddressesBuilder::new(self.client, cp_ref)
    }

    pub fn get_counterparties(&self) -> GetCounterpartiesBuilder<'cli> {
        GetCounterpartiesBuilder::new(self.client)
    }

    pub fn get_contact_persons(&self, cp_ref: Uuid) -> GetContactPersonsBuilder<'cli> {
        GetContactPersonsBuilder::new(self.client, cp_ref)
    }

    pub fn create_counterparty(&self) -> CreateCounterpartyBuilder<'cli, NoName, NoPhone, NoRole, NoType> {
        CreateCounterpartyBuilder::new(self.client)
    }

}