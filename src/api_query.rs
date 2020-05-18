use chrono::{Local, DateTime};
use chrono::naive::NaiveDate;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyRes {
    pub amount: f64,
    pub base: String,
    pub date: String,
    pub rates: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherEntry {   
    pub id: u64,
    pub weather_state_name: String,
    pub weather_state_abbr: String,
    pub wind_direction_compass: String,
    pub created: DateTime<Local>,
    pub applicable_date: NaiveDate,
    pub min_temp: f64,
    pub max_temp: f64,
    pub the_temp: f64,
    pub wind_speed: f64,
    pub wind_direction: f64,
    pub air_pressure: f64,
    pub humidity: u32,
    pub visibility: f64,
    pub predictability: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherParent {
    pub title: String,
    pub location_type: String,
    pub woeid: u64,
    pub latt_long: String,
}

#[derive(Serialize, Deserialize, Debug)]
//#[derive(Deserialize)]
pub struct WeatherRes {
    #[serde(rename = "consolidated_weather")]
    //#[serde(borrow)]
    pub weather: Vec<WeatherEntry>,
    #[serde(rename = "time")]
    pub timestamp: DateTime<Local>,
    pub sun_rise: DateTime<Local>,
    pub sun_set: DateTime<Local>,
    pub timezone_name: String,
    pub parent: WeatherParent,
    #[serde(skip)]
    pub sources: (),
    pub title: String,
    pub location_type: String,
    pub woeid: u64,
    pub latt_long: String,
    pub timezone: String,
}

