extern crate image;
extern crate chrono;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

//use image::{GenericImage, ImageBuffer, RgbImage, DynamicImage};

use rust_rpi_led_matrix as m;
use std::thread;
use chrono::{DateTime, Utc};
use chrono::naive::NaiveDate;
use std::path::Path;

// TODO update the structs to reflect actual API data types
// TODO move everything into separate modules

#[derive(Serialize, Deserialize, Debug)]
struct APIRes {
    amount: f64,
    base: String,
    date: String,
    rates: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherEntry {   
    id: u64,
    weather_state_name: String,
    weather_state_abbr: String,
    wind_direction_compass: String,
    created: DateTime<Utc>,
    applicable_date: NaiveDate,
    min_temp: f64,
    max_temp: f64,
    the_temp: f64,
    wind_speed: f64,
    wind_direction: f64,
    air_pressure: f64,
    humidity: u32,
    visibility: f64,
    predictability: u32,
}

#[derive(Serialize, Deserialize, Debug)]
struct WeatherParent {
    title: String,
    location_type: String,
    woeid: u64,
    latt_long: String,
}

#[derive(Serialize, Deserialize, Debug)]
//#[derive(Deserialize)]
struct WeatherRes {
    #[serde(rename = "consolidated_weather")]
    //#[serde(borrow)]
    weather: Vec<WeatherEntry>,
    #[serde(rename = "time")]
    timestamp: DateTime<Utc>,
    sun_rise: DateTime<Utc>,
    sun_set: DateTime<Utc>,
    timezone_name: String,
    parent: WeatherParent,
    #[serde(skip)]
    sources: (),
    title: String,
    location_type: String,
    woeid: u64,
    latt_long: String,
    timezone: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let options = m::LedMatrixOptions::new();
    let options = m::LedMatrixOptions { cols: 64, ..options };
    let matrix = m::LedMatrix::new(options);
    //matrix.set_brightness(20);
    let canvas = matrix.offscreen_canvas();

    //let image = image::open("../../sun-red.png").unwrap();
    //let image2 = image::open("sun-yellow.png").unwrap();

    //let pixel = image.get_pixel(0, 0);
    //let pixel = image.get_pixel(0, 0);

    //println!("image: {:?}", image);
    //
    //canvas.show_image(&image, 32, 0);
    //canvas.show_image(&image2, 0, 0);

    //let canvas = matrix.swap(canvas);
    //thread::sleep(time::Duration::from_millis(10000));

    //canvas.draw_circle(0, 0, 15, 255, 255, 255);
    //let canvas = matrix.swap(canvas);
    //thread::sleep(time::Duration::from_millis(5000));

    let resp = reqwest::blocking::get("https://api.frankfurter.app/latest?from=EUR")?
        .json::<APIRes>()?;

    println!("{:?}", resp);

    let yen = match resp.rates.get("JPY") {
        Some(n) => *n,
        None => -1.0
    };

    println!("EUR 1 = JPY {:}", yen);

    let f_5x7 = m::LedFont::new(Path::new("/home/pi/rpi-rgb-led-matrix/fonts/5x7.bdf"));
    let f_4x6 = m::LedFont::new(Path::new("/home/pi/rpi-rgb-led-matrix/fonts/4x6.bdf"));
    let f_large = m::LedFont::new(Path::new("/home/pi/rpi-rgb-led-matrix/fonts/6x10.bdf"));
    let f_awe = m::LedFont::new(Path::new("/home/pi/otf2bdf-3.1/fontawesome_32.bdf"));

    //canvas.draw_text(&f_5x7, 0, 7, 255, 255, 255, "€1=".to_string(), 0);
    canvas.draw_text(&f_large, 0, 19, 255, 255, 255, format!("¥{}", yen), 0);
    //canvas.draw_text(&f_awe, 0, 18, 255, 255, 0, "".to_string(), 0);
    canvas.draw_text(&f_awe, 10*4, 19, 255, 255, 0, "".to_string(), 0);
    //canvas.vertical_draw_text(f, 16, 16, 255, 255, 255, "Test".to_string(), 0);
    
    
    let weather = reqwest::blocking::get("https://www.metaweather.com/api/location/1118370/")?
        .json::<WeatherRes>()?;

    println!("{:?}", weather);

    let s = weather.weather[0].weather_state_abbr.to_string();
    println!("{}", s);

    canvas.draw_text(&f_large, 0, 32, 255, 255, 255, s, 0);

    let canvas = matrix.swap(canvas);
    thread::sleep(std::time::Duration::from_millis(100000));
    Ok(())
}


