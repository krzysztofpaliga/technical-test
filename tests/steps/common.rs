use std::collections::HashSet;
use cucumber::{given, then};
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use std::io::Write;
use std::fs::File;

#[derive(cucumber::World, Debug, Default)]
pub struct APIWorld {
    pub http_client: Client,
    pub request_url: String,
    pub request_path: String,
    pub request_headers: HeaderMap,
    pub response_body: Option<String>,
}

#[given(expr = "I am about to make a request to Kraken {word} API")]
pub async fn prepare_api_request(w: &mut APIWorld, api: String) {
    w.http_client = Client::new();

    w.request_url = "https://api.kraken.com".to_string();
    w.request_path = format!("/0/{}", api);

    w.request_headers = HeaderMap::new();
    w.request_headers.insert("Accept", HeaderValue::from_static("application/json"));
    w.request_headers.insert("Content-Type", HeaderValue::from_static("application/json"));
}

#[then("I assert no error in response")]
pub async fn assert_response_error_free(w: &mut APIWorld) {
    let json_value: Value = serde_json::from_str(w.response_body.as_ref().unwrap()).expect("Failed to parse JSON");

    assert!(json_value["error"].is_array());
    assert!(json_value["error"].as_array().unwrap().is_empty(), "Error field is not empty");
}

#[then(expr = "I report the result under {word}")]
pub async fn report_result(w: &mut APIWorld, file_name: String) {
    let response_body = w.response_body.as_ref().unwrap();
    let mut file = File::create(file_name).expect("Unable to create file");
    file.write_all(response_body.as_bytes()).expect("Unable to write data");
}

#[then(expr = "Result has to expected format for {word}-{word}")]
pub async fn assert_same_fields(w: &mut APIWorld, base: String, quote: String) {
    let response_body = w.response_body.as_ref().unwrap();
    let response_json: Value = serde_json::from_str(response_body).unwrap();
    let server_data = &response_json["result"][format!("X{}Z{}", base, quote)];

    let file = File::open(format!("./assets/{}{}.json", base, quote)).expect("Unable to open file");
    let file_json: Value = serde_json::from_reader(file).expect("Unable to parse JSON from file");
    let file_data = &file_json["result"][format!("X{}Z{}", base, quote)];

    assert_json_fields(&server_data, &file_data);
}

pub fn assert_json_fields(server_data: &Value, file_data: &Value) {
    let server_keys: HashSet<_> = server_data.as_object().unwrap().keys().collect();
    let file_keys: HashSet<_> = file_data.as_object().unwrap().keys().collect();

    assert_eq!(server_keys, file_keys, "The fields in the JSON response do not match the fields in the file");
}

