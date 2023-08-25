use std::env;

use reqwest::Error;
use serde_json::Value;

pub async fn get_location() -> Result<String, Error> {
    let api_key = env::var("IP_STACK_API_KEY").unwrap();
    let ip = get_my_ip().await.unwrap();
    let url = format!("http://api.ipstack.com/{ip}?access_key={api_key}");

    let resp = reqwest::get(url).await?;
    let parsed: Value = serde_json::from_str(&resp.text().await.unwrap()).unwrap();
    Ok(parsed["city"].to_string())
}

async fn get_my_ip() -> Result<String, Error> {
    let resp = reqwest::get("https://api.bigdatacloud.net/data/client-ip").await?;
    let parsed: Value = serde_json::from_str(&resp.text().await.unwrap()).unwrap();
    Ok(parsed["ipString"].as_str().unwrap().to_string())
}
