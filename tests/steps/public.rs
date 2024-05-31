use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime};
use cucumber::{then, when, World};
use serde_json::Value;

mod common;
use common::APIWorld;

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

#[then("Unixtime is equal to current time")]
async fn assert_current_time(w: &mut APIWorld) {
    let response_body = w.response_body.as_ref().unwrap();
    let response_json: Value = serde_json::from_str(response_body).unwrap();
    let server_unixtime = response_json["result"]["unixtime"].as_i64().unwrap();

    let current_unixtime = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

    // Allow for a small time difference due to network delay
    assert!((server_unixtime - current_unixtime).abs() <= 5, "Server time is not within 5 seconds of current time");
}

#[then("Unixtime is equal to RFC1123 time")]
async fn assert_server_time(w: &mut APIWorld) {
    let response_body = w.response_body.as_ref().unwrap();
    let response_json: Value = serde_json::from_str(response_body).unwrap();
    let server_unixtime = response_json["result"]["unixtime"].as_i64().unwrap();
    let server_rfc1123_time = response_json["result"]["rfc1123"].as_str().unwrap();

    let parsed_server_time = DateTime::parse_from_rfc2822(server_rfc1123_time).unwrap();
    let parsed_server_unixtime = parsed_server_time.timestamp();

    assert_eq!(server_unixtime, parsed_server_unixtime, "Unixtime does not match RFC1123 time");
}



#[allow(dead_code)]
#[tokio::main]
async fn main() {
    APIWorld::cucumber().run_and_exit("./tests/features/public.feature").await;
}

