use crate::np_client::NPClient;

use serde_json::json;
use wiremock::{
    matchers::{method, path, body_partial_json},
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
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(
                include_str!("resources/create_counterparty_response.json"),
                "application/json"
            )
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = np_client.create_counterparty(
        "Богдан",
        "Ігор",
        "Антонич",
        "380997979780",
        "test@test.com"
    ).await;

    assert!(res.is_ok());
    assert!(res.unwrap().success);
}