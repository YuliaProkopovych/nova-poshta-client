use crate::np_client::{NPClient, helper_structs::{CounterpartyType, CounterpartyRole}};

use serde_json::json;
use wiremock::{
    matchers::{body_partial_json, method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn create_counterparty_request_ok() {
    let mock_server = MockServer::start().await;
    let mut np_client = NPClient::default().unwrap();
    np_client.base_url(&mock_server.uri());

    let expected_body = json!({
        "apiKey": "",
        "modelName": "Counterparty",
        "calledMethod": "save",
        "methodProperties": {
            "FirstName": "Богдан",
            "MiddleName": "Ігор",
            "LastName": "Антонич",
            "Phone": "380997979780",
            "Email": "test@test.com",
            "CounterpartyType": "PrivatePerson",
            "CounterpartyProperty": "Recipient"
        }
    });

    Mock::given(path("/"))
        .and(method("POST"))
        .and(body_partial_json(&expected_body))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            include_str!("resources/create_counterparty_response.json"),
            "application/json",
        ))
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = np_client
        .counterparty()
        .create_counterparty()
        .name("Богдан".to_owned(), "Ігор".to_owned(), "Антонич".to_owned())
        .phone("380997979780".to_owned())
        .email("test@test.com".to_owned())
        .cp_type(CounterpartyType::PrivatePerson)
        .role(CounterpartyRole::Recipient)
        .send()
        .await;

    assert!(res.is_ok());
    assert!(res.unwrap().success);
}
