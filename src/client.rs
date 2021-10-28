use std::time::Duration;

use reqwest::header;

pub async fn build_client() -> reqwest::Client {
    let mut auth_token = header::HeaderValue::from_static(env!("CURRENT_API_KEY"));
    auth_token.set_sensitive(true);

    let mut headers = header::HeaderMap::new();
    headers.insert("X-SUBDOMAIN", header::HeaderValue::from_static(env!("CURRENT_SUBDOMAIN")));
    headers.insert("X-AUTH-TOKEN", auth_token);

    let timeout = Duration::new(30, 0);

    let client: reqwest::Client = reqwest::Client::builder()
        .default_headers(headers)
        .https_only(true)
        .timeout(timeout)
        .build()
        .expect("Failed to build the http Client.");

    client
}