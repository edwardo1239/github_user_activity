use core::str;
use std::{collections::HashMap, error::Error};

use reqwest::Client;
use serde_json::{from_str, Value};

use crate::models::error::HttpError;


pub async fn enviar_solicitur(usuario: &str) -> Result<Vec<HashMap<String, Value>>, Box<dyn Error>> {
    let url = format!("https://api.github.com/users/{}/events", usuario);
    let client = Client::new();

    let response = client
        .get(&url)
        .header("User-Agent", "reqwest")
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let eventos: Vec<HashMap<String, Value>> = from_str(&body)?;
        Ok(eventos)
    } else {
        let status_code = response.status().as_u16();
        Err(Box::new(HttpError::new(
            status_code,
            &format!("HTTP request failed status: {}", status_code),
        )))
    }
}
