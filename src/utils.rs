use crate::Error;
use futures_lite::io::AsyncReadExt;
use isahc::http::Request;
use isahc::HttpClient;
use serde::de::DeserializeOwned;

pub async fn get_json<T: DeserializeOwned>(client: &HttpClient, uri: &str) -> Result<T, 