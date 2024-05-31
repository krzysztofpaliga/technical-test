use std::{
    time::{SystemTime},
};
use std::fs::File;
use std::io::Write;

use cucumber::{given, then, when, World};
use hmac;
use hmac::{Mac, NewMac};
use reqwest::{Client};
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::{Value};
use serde_json;
use sha2::{Digest, Sha256, Sha512};



#[given("I am about to make a request to Kraken Private API")]
async fn prepare_private_request(w: &mut APIWorld) {
    w.request_url = "https://api.kraken.com".to_string();
    w.request_path = "/0/private".to_string();

    w.http_client = Client::new();

    w.request_headers = HeaderMap::new();
    w.request_headers.insert("Content-Type", HeaderValue::from_static("application/json"));
    w.request_headers.insert("Accept", HeaderValue::from_static("application/json"));
}


#[when("I retrieve open orders")]
async fn retrieve_open_orders(w: &mut APIWorld) {
    w.request_path.push_str("/OpenOrders");
    // Normally, keys woudl be read from .env file
    // For simplicity hardcoded
    let api_key = "jkQX5vKHdHOZY3EMmenbZOl/EBTyWO/h/hDVEZ2siScgSgw3ay8uSPBG";
    let private_key = "EUpz3x/N65jf3AxaSQ7rga5DXMbLYu4ClQvMlmiGe5p/4etXY23TqzuwCR6iQKtShFu1XdfUCuK718+GXHQmQQ==";

    let nonce = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis();

    // Normally, password would be read from .env file
    // For simplicity hardcoded
    let post_data = format!("{{\"nonce\":{},\"userref\":106313433,\"otp\":\"littleBrownFox7!\"}}", nonce);

    let sha2_result = {
        let mut hasher = Sha256::default();
        hasher.update(nonce.to_string());
        hasher.update(&post_data);
        hasher.finalize()
    };

    let hmac_sha_key = base64::decode(private_key).unwrap();

    type HmacSha = hmac::Hmac<Sha512>;
    let mut mac = HmacSha::new_varkey(&hmac_sha_key).expect("Hmac should work with any key length");
    mac.update(w.request_path.as_bytes());
    mac.update(&sha2_result);
    let mac = mac.finalize().into_bytes();

    let sig = base64::encode(&mac);

    w.request_headers.insert("API-Key", HeaderValue::from_str(api_key).unwrap());
    w.request_headers.insert("API-Sign", HeaderValue::from_str(&sig).unwrap());

    let result = w.http_client
        .post(format!("{}{}", &w.request_url, &w.request_path))
        .headers(w.request_headers.clone())
        .body(post_data)
        .send()
        .await;

    match result {
        Ok(response) => {
            w.response_body = Some(response.text().await.unwrap());
        }
        Err(error) => {
            panic!("Request failed with error: {:?}", error);
        }
    }
}

#[then("I assert no error in response")]
fn assert_response_error_free(w: &mut APIWorld) {
    let json_value: Value = serde_json::from_str(w.response_body.as_ref().unwrap()).expect("Failed to parse JSON");

    assert!(json_value["error"].is_array());
    assert!(json_value["error"].as_array().unwrap().is_empty(), "Error field is not empty");
}

#[then("I report the result")]
async fn report_result(w: &mut APIWorld) {
    let response_body = w.response_body.as_ref().unwrap();
    let mut file = File::create("kraken_private_api_response.json").expect("Unable to create file");
    file.write_all(response_body.as_bytes()).expect("Unable to write data");

}

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    APIWorld::run("./tests/features/private.feature").await;
}




