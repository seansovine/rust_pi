use serde::Deserialize;
use sqlx::{SqlitePool, query, query_scalar, sqlite::SqliteConnectOptions};
use std::{error::Error, str::FromStr, time::Duration};
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
    pub dt: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get openweathermap api key from environment.
    let api_key = std::env::var(API_KEY_VAR).expect("OPENWEATHERMAP_KEY variable is not set.");

    // Connect to database, setting up if needed.
    let pool = setup_db_conn().await?;

    let api_url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q=blacksburg,va,us&units=imperial&APPID={api_key}"
    );

    let config = Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(5)))
        .build();
    let agent: Agent = config.into();

    let result: WeatherApiResult = agent.get(api_url).call()?.body_mut().read_json()?;
    dbg!(&result);
    log_to_db(&pool, &result).await?;

    pool.close().await;

    Ok(())
}

async fn log_to_db(pool: &SqlitePool, reading: &WeatherApiResult) -> Result<(), sqlx::Error> {
    // TODO: Do not insert reading with duplicate timestamp.
    query(
        r#"INSERT INTO weather_readings (timestamp, temp)
           VALUES ($1, $2)
    "#,
    )
    .bind(reading.dt)
    .bind(reading.main.temp)
    .execute(pool)
    .await?;
    Ok(())
}

async fn setup_db_conn() -> Result<SqlitePool, sqlx::Error> {
    let database_url = "sqlite://database.sqlite3";
    let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;

    let count: i64 =
        query_scalar("SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='$1'")
            .bind("weather_readings")
            .fetch_one(&pool)
            .await?;
    if count == 0 {
        query(
            "CREATE TABLE IF NOT EXISTS weather_readings (
                 id INTEGER PRIMARY KEY AUTOINCREMENT,
          timestamp INTEGER NOT NULL,
               temp INTEGER NOT NULL
        )",
        )
        .execute(&pool)
        .await?;
    }

    Ok(pool)
}
