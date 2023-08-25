use reqwest::Error;
use serde_json::Value;
use std::{env, fmt::Display};

use crate::location::get_location;

pub struct Weather {
    pub max_temp: f64,
    pub min_temp: f64,
    pub current_temp: f64,
    pub condition: String,
    pub location: String,
}

impl Display for Weather {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Location: {}", self.location)?;
        writeln!(f, "Current: {} °C", self.current_temp)?;
        writeln!(f, "Maximum: {} °C", self.max_temp)?;
        writeln!(f, "Minimum: {} °C", self.min_temp)?;
        writeln!(f, "Condition: {}", self.condition)
    }
}

impl Weather {
    pub async fn get() -> Weather {
        let weather_data = get_weather_data().await.unwrap();
        Weather {
            current_temp: weather_data["current"]["temp_c"].as_f64().unwrap(),
            max_temp: weather_data["forecast"]["forecastday"][0]["day"]["maxtemp_c"]
                .as_f64()
                .unwrap(),
            min_temp: weather_data["forecast"]["forecastday"][0]["day"]["mintemp_c"]
                .as_f64()
                .unwrap(),
            condition: weather_data["current"]["condition"]["text"]
                .as_str()
                .unwrap()
                .to_string(),
            location: weather_data["location"]["name"].as_str().unwrap().to_string()
        }
    }
}

async fn get_weather_data() -> Result<Value, Error> {
    let args: Vec<String> = env::args().collect();

    let city = if args.len() > 1 {
        args[1].to_owned()
    } else {
        get_location().await.unwrap()
    };

    let api_key = env::var("WEATHERAPI_KEY").unwrap();
    let url = format!("https://api.weatherapi.com/v1/forecast.json?key={api_key}&q={city}&days=1&aqi=no&alerts=no");

    let resp = reqwest::get(url).await?;

    Ok(serde_json::from_str(&resp.text().await.unwrap()).unwrap())
}
