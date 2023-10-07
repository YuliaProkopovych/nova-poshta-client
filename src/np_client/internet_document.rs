use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};
use uuid::Uuid;
use phonenumber::{self, PhoneNumber};

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IDocument {
    r#ref: Uuid,
    cost_on_site: u16,
    #[serde(with = "id_date_format")]
    estimated_delivery_date: NaiveDate,
    int_doc_number: ENumber,
    type_document: String,
}

use super::date_format::id_date_format;
use super::{NPClient, NPRequest};
use super::en::ENumber;
use super::helper_structs::{CounterpartyRole, PaymentMethod, ServiceType, CargoType};
use super::res_template::ResponseTemplate;

pub struct IDocumentHandler<'c> {
    client: &'c NPClient,
}

impl<'cli> IDocumentHandler<'cli> {
    pub(crate) fn new(client: &'cli NPClient) -> Self {
        Self { 
            client
        }
    }

    pub fn create_document(&self) -> CreateIDocumentBuilder<'cli, NoGeneralInfo, NoSenderInfo, NoRecipientInfo> {
        CreateIDocumentBuilder::new(self.client)
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct RecepientInfo {
    city_recipient: Uuid, 
    recipient: Uuid, 
    recipient_address: Uuid, 
    #[serde_as(as = "DisplayFromStr")]
    recipients_phone: PhoneNumber,
    contact_recipient: Uuid,
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SenderInfo {
    city_sender: Uuid, 
    sender: Uuid,
    sender_address: Uuid, 
    contact_sender: Uuid,
    #[serde_as(as = "DisplayFromStr")]
    senders_phone: PhoneNumber,
}

pub enum Address {
    CityWarehouse {
        city_sender: Uuid,
        sender_address: Uuid,
    },
    Address {
        sender_adderss: Uuid,
    }
}

#[serde_as]
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct IDGeneral {
    payer_type: CounterpartyRole,
    payment_method: PaymentMethod, 
    #[serde(with = "id_date_format")]
    date_time: NaiveDate, 
    cargo_type: CargoType,
    #[serde_as(as = "DisplayFromStr")]
    weight: f32, 
    service_type: ServiceType,
    #[serde_as(as = "DisplayFromStr")]
    seats_amount: u16,
    description: String,
    #[serde_as(as = "DisplayFromStr")]
    cost: u16,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CreateIDocumentBuilder<'cli, IDGeneral, SenderInfo, RecepientInfo> {
    #[serde(skip)]
    client: &'cli NPClient,

    #[serde(flatten)]
    general_info: IDGeneral,

    #[serde(flatten)]
    recipient_info: RecepientInfo,

    #[serde(flatten)]
    sender_info: SenderInfo,
}

#[derive(Debug, Serialize)]
pub struct NoRecipientInfo {}

#[derive(Debug, Serialize)]
pub struct NoGeneralInfo {}

#[derive(Debug, Serialize)]
pub struct NoSenderInfo {}

impl<'cli> CreateIDocumentBuilder<'cli, NoGeneralInfo, NoSenderInfo, NoRecipientInfo> {
    pub fn new(client: &'cli NPClient) -> Self {
        Self {
            client,
            general_info: NoGeneralInfo{},
            sender_info: NoSenderInfo {},
            recipient_info: NoRecipientInfo{},
        }
    }
}

impl<'cli, S, R> CreateIDocumentBuilder<'cli, NoGeneralInfo, S, R> 
    where S: Serialize,
        R: Serialize,
{
    pub fn general_info(
        self, 
        payer_type: CounterpartyRole,
        payment_method: PaymentMethod, 
        date_time: NaiveDate, 
        cargo_type: CargoType,
        weight: f32, 
        service_type: ServiceType,
        seats_amount: u16,
        description: String,
        cost: u16,
    ) -> CreateIDocumentBuilder<'cli, IDGeneral, S, R> {
        CreateIDocumentBuilder {
            client: self.client,
            general_info: IDGeneral {
                payer_type,
                payment_method,
                date_time,
                cargo_type,
                weight,
                seats_amount,
                service_type,
                description,
                cost,
            },
            sender_info: self.sender_info,
            recipient_info: self.recipient_info,
        }
    } 
}

impl<'cli, G, R> CreateIDocumentBuilder<'cli, G, NoSenderInfo, R> 
    where G: Serialize,
        R: Serialize,
{
    pub fn sender_info(
        self, 
        city_sender: Uuid, 
        sender: Uuid,
        sender_address: Uuid, 
        contact_sender: Uuid,
        senders_phone: PhoneNumber,
    ) -> CreateIDocumentBuilder<'cli, G, SenderInfo, R> {
        CreateIDocumentBuilder {
            client: self.client,
            general_info: self.general_info,
            sender_info: SenderInfo {
                city_sender,
                sender,
                sender_address,
                senders_phone,
                contact_sender,
            },
            recipient_info: self.recipient_info,
        }
    } 
}

impl<'cli, G, S> CreateIDocumentBuilder<'cli, G, S, NoRecipientInfo> 
    where G: Serialize,
        S: Serialize,
{
    pub fn recipient_info(
        self, 
        city_recipient: Uuid, 
        recipient: Uuid, 
        recipient_address: Uuid, 
        contact_recipient: Uuid,
        recipients_phone: PhoneNumber,
    ) -> CreateIDocumentBuilder<'cli, G, S, RecepientInfo> {
        CreateIDocumentBuilder {
            client: self.client,
            general_info: self.general_info,
            sender_info: self.sender_info,
            recipient_info: RecepientInfo { 
                city_recipient, 
                recipient, 
                recipient_address, 
                recipients_phone, 
                contact_recipient, 
            },
        }
    } 
}

impl<'cli> CreateIDocumentBuilder<'cli, IDGeneral, SenderInfo, RecepientInfo> {
    pub async fn send(self) -> Result<ResponseTemplate<IDocument>, reqwest::Error> {
        let request = NPRequest {
            api_key: &self.client.api_key,
            model_name: "InternetDocument",
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


