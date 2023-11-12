use std::str::FromStr;

use crate::np_client::{NPClient, helper_structs::{CounterpartyRole, PaymentMethod, CargoType, ServiceType, CityID, CounterpartyID, ContactPersonID, AddressID}};

use chrono::NaiveDate;
use phonenumber::country::Id::UA;
use serde_json::json;
use wiremock::{
    matchers::{body_partial_json, method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn create_internet_document_ok() {
    let mock_server = MockServer::start().await;
    let mut np_client = NPClient::default().unwrap();
    np_client.base_url(&mock_server.uri());

    let expected_body = json!({
        "apiKey": "",
        "modelName": "InternetDocument",
        "calledMethod": "save",
        "methodProperties": {
            "PayerType": "Sender",
            "PaymentMethod": "Cash",
            "DateTime": "11.10.2023",
            "CargoType": "Parcel",
            "Weight": "0.5",
            "ServiceType": "WarehouseDoors",
            "SeatsAmount": "2",
            "Description": "посилка",
            "Cost": "120",
            "CitySender": "db5c88d4-391c-11dd-90d9-001a92567626",
            "Sender": "57f2c3c2-596f-11ee-a60f-48df37b921db",
            "SenderAddress": "1ec09d2d-e1c2-11e3-8c4a-0050568002cf",
            "ContactSender": "57f35831-596f-11ee-a60f-48df37b921db",
            "SendersPhone": "+380660000001",
            "CityRecipient": "db5c88d4-391c-11dd-90d9-001a92567626",
            "Recipient": "580c30f7-596f-11ee-a60f-48df37b921db",
            "RecipientAddress": "39633d8b-645f-11ee-a60f-48df37b921db",
            "ContactRecipient": "c1ffd9b4-643e-11ee-a60f-48df37b921db",
            "RecipientsPhone": "+380660000000",
        }
    });

    Mock::given(path("/"))
        .and(method("POST"))
        .and(body_partial_json(&expected_body))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            include_str!("resources/create_internet_document_response.json"),
            "application/json",
        ))
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = np_client
        .i_document()
        .create_document()
        .general_info(
            CounterpartyRole::Sender,
            PaymentMethod::Cash,
            NaiveDate::from_ymd_opt(2023, 10, 11).unwrap(),
            CargoType::Parcel,
            0.5,
            ServiceType::WarehouseDoors,
            2,
            "посилка".to_owned(),
            120,
        )
        .sender_info(
            CityID::from_str("db5c88d4-391c-11dd-90d9-001a92567626").unwrap(),
            CounterpartyID::from_str("57f2c3c2-596f-11ee-a60f-48df37b921db").unwrap(),
            AddressID::from_str("1ec09d2d-e1c2-11e3-8c4a-0050568002cf").unwrap(),
            ContactPersonID::from_str("57f35831-596f-11ee-a60f-48df37b921db").unwrap(),
            phonenumber::parse(Some(UA), "380660000001").unwrap()
            
        )
        .recipient_info(
            CityID::from_str("db5c88d4-391c-11dd-90d9-001a92567626").unwrap(),
            CounterpartyID::from_str("580c30f7-596f-11ee-a60f-48df37b921db").unwrap(),
            AddressID::from_str("39633d8b-645f-11ee-a60f-48df37b921db").unwrap(),
            ContactPersonID::from_str("c1ffd9b4-643e-11ee-a60f-48df37b921db").unwrap(),
            phonenumber::parse(Some(UA), "380660000000").unwrap()
        )
        .send()
        .await;

    assert!(res.is_ok());
    assert!(res.unwrap().success);
}
