
#[derive(cucumber::World, Debug, Default)]
pub struct APIWorld {
    http_client: Client,
    request_url: String,
    request_path: String,
    request_headers: HeaderMap,
    response_body: Option<String>,
}