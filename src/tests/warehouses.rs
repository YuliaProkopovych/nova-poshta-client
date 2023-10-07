use crate::np_client::NPClient;

use serde_json::json;
use wiremock::{
    matchers::{body_partial_json, method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn warehouses_request_ok() {
    let mock_server = MockServer::start().await;
    let mut np_client = NPClient::default().unwrap();
    np_client.base_url(&mock_server.uri());

    let expected_body = json!({
        "modelName": "Address",
        "calledMethod": "getWarehouses",
        "methodProperties": {
            "Page": 1,
            "Limit": 10,
            "CityName": "львів",
        }
    });

    Mock::given(path("/"))
        .and(method("POST"))
        .and(body_partial_json(&expected_body))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            include_str!("resources/warehouses_response.json"),
            "application/json",
        ))
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = np_client
        .address()
        .get_warehouses()
        .city_name("львів".to_owned())
        .limit(10)
        .page(1)
        .send()
        .await;

    assert!(res.is_ok());
    assert!(res.unwrap().success);
}
