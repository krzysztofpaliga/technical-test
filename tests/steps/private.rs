use std::{
    time::{SystemTime},
};

use cucumber::{when, World};
use hmac;
use hmac::{Mac, NewMac};
use reqwest::header::{HeaderValue};
use sha2::{Digest, Sha256, Sha512};

mod common;
use common::APIWorld;

#[when("I retrieve open orders")]
async fn retrieve_open_orders(w: &mut APIWorld) {
    w.request_path.push_str("/OpenOrders");
    // Normally, keys would be read from .env file
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

#[allow(dead_code)]
#[tokio::main]
async fn main() {
    APIWorld::run("./tests/features/private.feature").await;
}




