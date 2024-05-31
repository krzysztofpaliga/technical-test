use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime};
use cucumber::{given, then, when, World};
use reqwest::{Client};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;

mod private;

#[derive(cucumber::World, Debug, Default)]
pub struct APIWorld {
    http_client: Client,
    request_url: String,
    request_path: String,
    request_headers: HeaderMap,
    response_body: Option<String>,
}

#[given("I am about to make a request to Kraken Public API")]
async fn prepare_public_request(w: &mut APIWorld) {
    w.http_client = Client::new();

    w.request_url = "https://api.kraken.com".to_string();
    w.request_path = "/0/public".to_string();

    w.request_headers = HeaderMap::new();
    w.request_headers.insert("Accept", HeaderValue::from_static("application/json"));
}

#[when("I retrieve the server time")]
async fn retrieve_server_time(w: &mut APIWorld) {
    w.request_path.push_str("/Time");

    let result = w.http_client.get(format!("{}{}", &w.request_url, &w.request_path)).headers(w.request_headers.clone()).send().await;
    match result {
        Ok(response) => {
            assert_eq!(response.status(), 200);
            w.response_body = Some(response.text().await.unwrap());
        }
        Err(error) => {
            panic!("Request failed with error: {:?}", error);
        }
    }
}

#[when(expr = "I retrieve the {word}-{word} trading pair")]
async fn retrieve_traiding_pair(w: &mut APIWorld, base:String, quote:String) {
    w.request_path.push_str(&format!("/AssetPairs?pair={}{}", base, quote));
    let result = w.http_client.get(format!("{}{}", &w.request_url, &w.request_path)).headers(w.request_headers.clone()).send().await;
    match result {
        Ok(response) => {
            assert_eq!(response.status(), 200);
            w.response_body = Some(response.text().await.unwrap());
        }
        Err(error) => {
            panic!("Request failed with error: {:?}", error);
        }
    }
}

#[then("I report the result")]
async fn report_result(w: &mut APIWorld) {
    let response_body = w.response_body.as_ref().unwrap();
    let mut file = File::create("kraken_api_response.json").expect("Unable to create file");
    file.write_all(response_body.as_bytes()).expect("Unable to write data");
}

#[then("Unixtime is equal to current time")]
fn assert_current_time(w: &mut APIWorld) {
    let response_body = w.response_body.as_ref().unwrap();
    let response_json: Value = serde_json::from_str(response_body).unwrap();
    let server_unixtime = response_json["result"]["unixtime"].as_i64().unwrap();

    let current_unixtime = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

    // Allow for a small time difference due to network delay
    assert!((server_unixtime - current_unixtime).abs() <= 5, "Server time is not within 5 seconds of current time");
}

#[then("Unixtime is equal to RFC1123 time")]
fn assert_server_time(w: &mut APIWorld) {
    let response_body = w.response_body.as_ref().unwrap();
    let response_json: Value = serde_json::from_str(response_body).unwrap();
    let server_unixtime = response_json["result"]["unixtime"].as_i64().unwrap();
    let server_rfc1123_time = response_json["result"]["rfc1123"].as_str().unwrap();

    let parsed_server_time = DateTime::parse_from_rfc2822(server_rfc1123_time).unwrap();
    let parsed_server_unixtime = parsed_server_time.timestamp();

    assert_eq!(server_unixtime, parsed_server_unixtime, "Unixtime does not match RFC1123 time");
}


#[then(expr = "Result has to expected format for {word}-{word}")]
fn assert_same_fields(w: &mut APIWorld, base: String, quote:String) {
    let response_body = w.response_body.as_ref().unwrap();
    let response_json: Value = serde_json::from_str(response_body).unwrap();
    let server_data = &response_json["result"][format!("X{}Z{}",base, quote)];

    let file = File::open(format!("./assets/{}{}.json", base, quote)).expect("Unable to open file");
    let file_json: Value = serde_json::from_reader(file).expect("Unable to parse JSON from file");
    let file_data = &file_json["result"][format!("X{}Z{}",base, quote)];

    assert_json_fields(&server_data, &file_data);
}

fn assert_json_fields(server_data: &Value, file_data: &Value) {
    let server_keys: HashSet<_> = server_data.as_object().unwrap().keys().collect();
    let file_keys: HashSet<_> = file_data.as_object().unwrap().keys().collect();

    assert_eq!(server_keys, file_keys, "The fields in the JSON response do not match the fields in the file");
}
#[allow(dead_code)]
#[tokio::main]
async fn main() {
    APIWorld::run("./tests/features/public.feature").await;
}

