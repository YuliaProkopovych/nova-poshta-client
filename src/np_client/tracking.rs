use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr, NoneAsEmptyString};
use uuid::Uuid;

use super::res_template::ResponseTemplate;
use super::{NPClient, NPRequest};
use super::date_format::{common_date_format, np_date_format};
use super::deserializer::{deserialize_f32_option, deserialize_u16_option};
use super::en::ENumber;
use super::helper_structs::{CounterpartyRole, CounterpartyType, PaymentMethod, ServiceType};

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Document {
    document_number: ENumber,
    phone: String,
}

impl Document {
    pub fn new(en: ENumber, phone: String) -> Self {
        Self {
            document_number: en,
            phone,
        }
    }
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Redelivery {
    redelivery: u8,
    #[serde(deserialize_with = "deserialize_f32_option")]
    redelivery_sum: Option<f32>,
    redelivery_num: String,
    #[serde_as(as = "NoneAsEmptyString")]
    redelivery_payer: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct SenderInfo {
    city_sender: String,
    counterparty_sender_type: CounterpartyType,
    phone_sender: Option<String>,
    warehouse_sender_internet_address_ref: Uuid,
    warehouse_sender_address: String,
    warehouse_sender: String,
    ref_city_sender: Uuid,
    ref_settlement_sender: Uuid,
    sender_address: String,
    sender_full_name_e_w: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct RecipientInfo {
    recipient_full_name: String,
    #[serde(with = "common_date_format")]
    recipient_date_time: Option<NaiveDateTime>,
    phone_recipient: String,
    #[serde_as(as = "NoneAsEmptyString")]
    recipient_full_name_e_w: Option<String>,
    city_recipient: String,
    warehouse_recipient: String,
    warehouse_recipient_internet_address_ref: Uuid,
    recipient_address: String,
    #[serde_as(as = "NoneAsEmptyString")]
    counterparty_recipient_description: Option<String>,
    ref_settlement_recipient: Uuid,
    ref_city_recipient: Uuid,
    recipient_warehouse_type_ref: Uuid,
    warehouse_recipient_ref: Uuid,
    #[serde_as(as = "NoneAsEmptyString")]
    loyalty_card_recipient: Option<String>,
    warehouse_recipient_number: Option<u16>,
    category_of_warehouse: String,
    warehouse_recipient_address: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct RedeliveryInfo {
    redelivery: u8,
    #[serde(deserialize_with = "deserialize_f32_option")]
    redelivery_sum: Option<f32>,
    redelivery_num: String,
    #[serde_as(as = "NoneAsEmptyString")]
    redelivery_payer: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct PaymentInfo {
    payer_type: CounterpartyRole,
    #[serde_as(as = "NoneAsEmptyString")]
    payment_status: Option<String>,
    #[serde(deserialize_with = "deserialize_f32_option")]
    afterpayment_on_goods_cost: Option<f32>,
    #[serde(with = "common_date_format")]
    payment_status_date: Option<NaiveDateTime>,
    #[serde(deserialize_with = "deserialize_f32_option")]
    amount_to_pay: Option<f32>,
    #[serde(deserialize_with = "deserialize_f32_option")]
    amount_paid: Option<f32>,
    secure_payment: bool,
    payment_method: PaymentMethod,
    #[serde(deserialize_with = "deserialize_f32_option")]
    announced_price: Option<f32>,
    possibility_change_cash_2_card: bool,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrackingDoc {
    possibility_create_return: bool,
    possibility_create_refusal: bool,
    possibility_change_e_w: bool,
    possibility_create_redirecting: bool,
    number: ENumber,
    #[serde(flatten)]
    redelivery_info: RedeliveryInfo,
    #[serde(flatten)]
    recipient_info: RecipientInfo,
    #[serde(flatten)]
    payment_info: PaymentInfo,
    #[serde_as(as = "NoneAsEmptyString")]
    owner_document_type: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    last_created_on_the_basis_document_type: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    last_created_on_the_basis_payer_type: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    last_created_on_the_basis_date_time: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    last_transaction_status_g_m: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    last_transaction_date_time_g_m: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    last_amount_transfer_g_m: Option<String>,
    #[serde(with = "np_date_format")]
    date_created: NaiveDateTime,
    document_weight: serde_json::Number,
    #[serde_as(as = "DisplayFromStr")]
    factual_weight: f32,
    #[serde_as(as = "DisplayFromStr")]
    volume_weight: f32,
    check_weight: f32,
    #[serde_as(as = "NoneAsEmptyString")]
    check_weight_method: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    document_cost: f32,
    #[serde_as(as = "NoneAsEmptyString")]
    calculated_weight: Option<String>,
    sum_before_check_weight: Option<f32>,
    #[serde(with = "common_date_format")]
    scheduled_delivery_date: Option<NaiveDateTime>,
    cargo_description_string: String,
    //Parcel
    cargo_type: String,
    counterparty_type: CounterpartyType,
    service_type: ServiceType,
    #[serde_as(as = "NoneAsEmptyString")]
    undelivery_reasons_subtype_description: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    last_created_on_the_basis_number: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    marketplace_partner_token: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    client_barcode: Option<String>,
    #[serde(with = "common_date_format")]
    date_scan: Option<NaiveDateTime>,
    status: Option<String>,
    ///CODE
    status_code: Option<String>,
    ref_e_w: Uuid,
    backward_delivery_sub_types_actions: Option<String>,
    backward_delivery_sub_types_services: Option<String>,
    #[serde(with = "common_date_format")]
    date_payed_keeping: Option<NaiveDateTime>,
    #[serde_as(as = "NoneAsEmptyString")]
    international_delivery_type: Option<String>,
    #[serde_as(as = "DisplayFromStr")]
    seats_amount: u16,
    #[serde_as(as = "NoneAsEmptyString")]
    card_masked_number: Option<String>,
    express_waybill_payment_status: String,
    #[serde(deserialize_with = "deserialize_f32_option")]
    express_waybill_amount_to_pay: Option<f32>,
    #[serde(with = "common_date_format")]
    tracking_update_date: Option<NaiveDateTime>,
    date_return_cargo: Option<String>,
    #[serde(with = "common_date_format")]
    date_moving: Option<NaiveDateTime>,
    #[serde(with = "common_date_format")]
    date_first_day_storage: Option<NaiveDateTime>,
    #[serde_as(as = "NoneAsEmptyString")]
    additional_information_e_w: Option<String>,
    #[serde(with = "common_date_format")]
    actual_delivery_date: Option<NaiveDateTime>,
    postomat_v3_cell_reservation_number: bool,
    #[serde_as(as = "NoneAsEmptyString")]
    owner_document_number: Option<String>,
    last_amount_received_commission_g_m: Option<f32>,
    #[serde_as(as = "NoneAsEmptyString")]
    delivery_timeframe: Option<String>,
    #[serde_as(as = "NoneAsEmptyString")]
    created_on_the_basis: Option<String>,
    #[serde(with = "common_date_format")]
    undelivery_reasons_date: Option<NaiveDateTime>,
    avia_delivery: u16,
    #[serde_as(as = "NoneAsEmptyString")]
    barcode_red_box: Option<String>,
    cargo_return_refusal: bool,
    #[serde(deserialize_with = "deserialize_u16_option")]
    days_storage_cargo: Option<u16>,
    packaging: Option<Vec<String>>,
    partial_return_goods: Option<Vec<String>>,
    #[serde(deserialize_with = "deserialize_u16_option")]
    storage_amount: Option<u16>,
    #[serde(deserialize_with = "deserialize_f32_option")]
    storage_price: Option<f32>,
    #[serde_as(as = "NoneAsEmptyString")]
    free_shipping: Option<String>,
}

pub struct TrackingHandler<'c> {
    client: &'c NPClient,
}

impl<'cli> TrackingHandler<'cli> {
    pub(crate) fn new(client: &'cli NPClient) -> Self {
        Self { client }
    }

    pub fn track_parcel(&self, en: ENumber, phone: String) -> TrackParcelBuilder<'cli> {
        TrackParcelBuilder::new(self.client, en, phone)
    }

}

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrackParcelBuilder<'cli> {
    #[serde(skip)]
    client: &'cli NPClient,

    documents: Vec<Document>,
}

impl<'cli> TrackParcelBuilder<'cli> {
    pub fn new(client: &'cli NPClient, en: ENumber, phone_number: String) -> Self {
        Self {
            client,
            documents: vec![Document::new(
                en,
                phone_number,
            )],
        }
    }

    pub fn add_document(mut self, en: ENumber, phone_number: String) -> Self {
        self.documents.push(
            Document::new(en, phone_number)
        );
        self
    }

    pub async fn send(self) -> Result<ResponseTemplate<TrackingDoc>, reqwest::Error> {
        let request = NPRequest {
            api_key: &self.client.api_key,
            model_name: "TrackingDocument",
            called_method: "getStatusDocuments",
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
