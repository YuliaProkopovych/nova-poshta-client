use crate::np_client::{NPClient, ResponseTemplate as NPResponseTemplate};

use serde_json::json;
use wiremock::{
    matchers::{method, path, body_partial_json},
    Mock, MockServer, ResponseTemplate,
};

#[tokio::test]
async fn tracking_request_ok() {
    let mock_server = MockServer::start().await;
    let mut np_client = NPClient::default().unwrap();
    np_client.base_url(&mock_server.uri());

    let expected_body = json!({
        "modelName": "TrackingDocument",
        "calledMethod": "getStatusDocuments",
        "methodProperties": {
            "Documents": [
                {
                    "DocumentNumber": "20450777813966",
                    "Phone": "380123456787"
                }
            ]
        }
    });

    Mock::given(path("/"))
        .and(method("POST"))
        .and(body_partial_json(&expected_body))
        .respond_with(
            ResponseTemplate::new(200).set_body_raw(
                include_str!("resources/tracking_response.json"),
                "application/json"
            )
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = np_client.get_tracking("20450777813966".to_owned(), "380123456787".to_owned()).await;

    assert!(res.is_ok());
    assert!(res.unwrap().success);
}

#[tokio::test]
async fn tracking_request_without_phone() {
    let mock_server = MockServer::start().await;
    let mut np_client = NPClient::default().unwrap();
    np_client.base_url(&mock_server.uri());

    let expected_body = json!({
        "modelName": "TrackingDocument",
        "calledMethod": "getStatusDocuments",
        "methodProperties": {
            "Documents": [
                {
                    "DocumentNumber": "20450777813966",
                    "Phone": ""
                }
            ]
        }
    });

    Mock::given(path("/"))
        .and(method("POST"))
        .and(body_partial_json(&expected_body))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_raw(
                    include_str!("resources/tracking_response_no_phone.json"),
                    "application/json"
            )
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = np_client.get_tracking("20450777813966".to_owned(), "".to_owned()).await;
    
    assert!(res.is_ok());
    let res = res.unwrap();
    assert!(res.success);
    assert!(!res.warnings.is_empty());
}
#[tokio::test]
async fn tracking_request_invalid_en() {
    let mock_server = MockServer::start().await;
    let mut np_client = NPClient::default().unwrap();
    np_client.base_url(&mock_server.uri());

    let expected_body = json!({
        "modelName": "TrackingDocument",
        "calledMethod": "getStatusDocuments",
        "methodProperties": {
            "Documents": [
                {
                    "DocumentNumber": "20450777813966",
                    "Phone": ""
                }
            ]
        }
    });

    let r: NPResponseTemplate<()> = serde_json::from_str(
        r#"
        {
            "success": false,
            "data": [],
            "errors": [
                "Document number is not correct"
            ],
            "warnings": [
                "Invalid DocumentNumber: 00450774207578"
            ],
            "info": [],
            "messageCodes": [],
            "errorCodes": [],
            "warningCodes": [],
            "infoCodes": []
        }
        "#
    ).unwrap();

    Mock::given(path("/"))
        .and(method("POST"))
        .and(body_partial_json(&expected_body))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(&r)
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    let res = np_client.get_tracking("20450777813966".to_owned(), "".to_owned()).await;

    assert!(res.is_ok());
    let res = res.unwrap();
    assert!(!res.success);
    assert!(!res.warnings.is_empty());
    assert!(!res.errors.is_empty());
}