use serde::Serialize;

use crate::np_client::{NPClient, res_template::ResponseTemplate, helper_structs::{CounterpartyRole, CounterpartyType}, NPRequest};

use super::Counterparty;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateCounterpartyBuilder<'cli, Name, Phone, CounterpartyRole, CounterpartyType> {
    #[serde(skip)]
    client: &'cli NPClient,

    #[serde(flatten)]
    name: Name,

    phone: Phone,

    counterparty_property: CounterpartyRole,

    counterparty_type: CounterpartyType,

    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Name {
    first_name: String,
    middle_name: String,
    last_name: String,
}
#[derive(Debug, Serialize)]
pub struct Phone(String);
#[derive(Debug, Serialize)]
pub struct NoName {}
#[derive(Debug, Serialize)]
pub struct NoPhone {}

#[derive(Debug, Serialize)]
pub struct NoRole {}
#[derive(Debug, Serialize)]
pub struct NoType {}

impl<'cli> CreateCounterpartyBuilder<'cli, NoName, NoPhone, NoRole, NoType> {
    pub fn new(client: &'cli NPClient) -> Self {
        Self {
            client,
            name: NoName{},
            phone: NoPhone{},
            email: None,
            counterparty_property: NoRole {},
            counterparty_type: NoType {}
        }
    }
}

impl<'cli, P, R, T> CreateCounterpartyBuilder<'cli, NoName, P, R, T> 
    where T: Serialize,
        P: Serialize,
        R: Serialize,
{
    pub fn name(self, first_name: String, middle_name: String, last_name: String) -> CreateCounterpartyBuilder<'cli, Name, P, R, T> {
        CreateCounterpartyBuilder {
            client: self.client,
            name: Name {
                first_name,
                middle_name,
                last_name,
            },
            phone: self.phone,
            email: self.email,
            counterparty_property: self.counterparty_property,
            counterparty_type: self.counterparty_type,
        }
    } 
}

impl<'cli, N, R, T> CreateCounterpartyBuilder<'cli, N, NoPhone, R, T> 
    where T: Serialize,
        N: Serialize,
        R: Serialize,
{
    pub fn phone(self, phone: String) -> CreateCounterpartyBuilder<'cli, N, Phone, R, T> {
        CreateCounterpartyBuilder {
            client: self.client,
            name: self.name,
            phone: Phone(phone),
            email: self.email,
            counterparty_property: self.counterparty_property,
            counterparty_type: self.counterparty_type,
        }
    } 
}

impl<'cli, N, P, T> CreateCounterpartyBuilder<'cli, N, P, NoRole, T> 
    where T: Serialize,
        N: Serialize,
        P: Serialize,
{
    pub fn role(self, role: CounterpartyRole) -> CreateCounterpartyBuilder<'cli, N, P, CounterpartyRole, T> {
        CreateCounterpartyBuilder {
            client: self.client,
            name: self.name,
            phone: self.phone,
            email: self.email,
            counterparty_property: role,
            counterparty_type: self.counterparty_type,
        }
    } 
}

impl<'cli, N, P, R> CreateCounterpartyBuilder<'cli, N, P, R, NoType> 
    where R: Serialize,
        N: Serialize,
        P: Serialize,
{
    pub fn cp_type(self, ty: CounterpartyType) -> CreateCounterpartyBuilder<'cli, N, P, R, CounterpartyType> {
        CreateCounterpartyBuilder {
            client: self.client,
            name: self.name,
            phone: self.phone,
            email: self.email,
            counterparty_property: self.counterparty_property,
            counterparty_type: ty,
        }
    } 
}

impl<'cli, N, P, R, T> CreateCounterpartyBuilder<'cli, N, P, R, T> {
    pub fn email(self, email: String) -> CreateCounterpartyBuilder<'cli, N, P, R, T> {
        CreateCounterpartyBuilder {
            client: self.client,
            name: self.name,
            phone: self.phone,
            email: Some(email),
            counterparty_property: self.counterparty_property,
            counterparty_type: self.counterparty_type,
        }
    } 
}

impl<'cli> CreateCounterpartyBuilder<'cli, Name, Phone, CounterpartyRole, CounterpartyType> {
    pub async fn send(self) -> Result<ResponseTemplate<Counterparty>, reqwest::Error> {
        let request = NPRequest {
            api_key: &self.client.api_key,
            model_name: "Counterparty",
            called_method: "save",
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

    // pub async fn get_counterparties(
    //     &self,
    //     find_by_string: Option<&str>,
    //     page: Option<u16>,
    //     counterparty_property: &str,
    // ) -> Result<ResponseTemplate<Counterparty>, reqwest::Error> {
    //     let res = self
    //         .send_request(Model::Counterparty(CounterpartyMethod::GetCounterparties {
    //             page,
    //             find_by_string,
    //             counterparty_property: CounterpartyRole::try_from(counterparty_property).unwrap(),
    //         }))
    //         .await?;

    //     let res_data = res.json().await;
    //     res_data
    // }

