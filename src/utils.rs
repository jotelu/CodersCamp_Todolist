use crate::Error;
use futures_lite::io::AsyncReadExt;
use isahc::http::Request;
use isahc::HttpClient;
use serde::de::DeserializeOwned;

pub async fn get_json<T: DeserializeOwned>(client: &HttpClient, uri: &str) -> Result<T, Error> {
    let request = Request::get(uri)
        .header("content-type", "application/javascript")
        .body(())
        .unwrap();

    let mut bytes = Vec::new();

    client
        .send_async(request)