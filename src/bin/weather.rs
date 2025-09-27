use serde::Deserialize;
use std::{error::Error, time::Duration};
use ureq::Agent;

const API_KEY_VAR: &str = "OPENWEATHERMAP_KEY";

#[derive(Deserialize, Debug)]
pub struct WeatherMain {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: f32,
    pub humidity: f32,
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f32,
    pub deg: u32,
}

#[derive(Deserialize, Debug)]
pub struct Clouds {
    pub all: u32,
}

#[derive(Deserialize, Debug)]
pub struct WeatherApiResult {
    pub main: WeatherMain,
    pub visibility: u32,
    pub wind: Wind,
    pub clouds: Clouds,
}

fn main() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var(API_KEY_VAR).expect("OPENWEATHERMAP_KEY variable is not set.");

    let api_url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q=blacksburg,va,us&units=imperial&APPID={api_key}"
    );

    // From ureq rustdoc example:
    let config = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(5)))
        .build();
    let agent: Agent = config.into();

    let result: WeatherApiResult = agent.get(api_url).call()?.body_mut().read_json()?;

    dbg!(result);

    // TODO: Call weather API and parse results.

    Ok(())
}
