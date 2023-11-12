use crate::np_client::NPClient;

use serde_json::json;
use wiremock::{
    matchers::{body_partial_json, method, path},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn get_cities_request_ok() {
    let mock_server = MockServer::start().await;
    let mut np_client = NPClient::default().unwrap();
    np_client.base_url(&mock_server.uri());

    let expected_body = json!({
        "modelName": "Address",
        "calledMethod": "getCities",
        "methodProperties": {
            "Page": 1,
            "Limit": 10,
            "FindByString": "львів",
        }
    });

    Mock::given(path("/"))
        .and(method("POST"))
        .and(body_partial_json(&expected_body))
        .respond_with(ResponseTemplate::new(200).set_body_raw(
            include_str!("resources/cities_response.json"),
            "application/json",
        ))
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = np_client
        .address()
        .get_cities()
        .page(1)
        .limit(10)
        .find_by_string("львів".to_owned())
        .send()
        .await;

    assert!(res.is_ok());
    assert!(res.unwrap().success);
}

#[tokio::test]
async fn get_cities_request_invalid_string() {
    let mock_server = MockServer::start().await;
    let mut np_client = NPClient::default().unwrap();
    np_client.base_url(&mock_server.uri());

    let expected_body = json!({
        "modelName": "Address",
        "calledMethod": "getCities",
        "methodProperties": {
            "Page": 1,
            "Limit": 10,
            "FindByString": "invalid value",
        }
    });

    let res = r#"
        {
            "success": false,
            "data": [],
            "errors": [
                "FindByString is not specified"
            ],
            "warnings": [],
            "info": [],
            "messageCodes": [],
            "errorCodes": [
                "20000500612"
            ],
            "warningCodes": [],
            "infoCodes": []
        }
    "#;

    Mock::given(path("/"))
        .and(method("POST"))
        .and(body_partial_json(&expected_body))
        .respond_with(ResponseTemplate::new(200).set_body_raw(res, "application/json"))
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = np_client
        .address()
        .get_cities()
        .page(1)
        .limit(10)
        .find_by_string("invalid value".to_owned())
        .send()
        .await;

    assert!(res.is_ok());
    let res = res.unwrap();
    assert!(!res.success);
    assert_eq!(res.data.len(), 0);
    assert_eq!(res.error_codes.len(), 1);
}
