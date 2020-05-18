extern crate image;
extern crate chrono;

mod api_query;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let options = m::LedMatrixOptions::new();
    let options = m::LedMatrixOptions { cols: 64, ..options };
    let matrix = m::LedMatrix::new(options);
    //matrix.set_brightness(20);
    let canvas = matrix.offscreen_canvas();


    let resp = reqwest::blocking::get("https://api.frankfurter.app/latest?from=EUR")?
        .json::<api_query::CurrencyRes>()?;

    //println!("{:?}", resp);

    let yen = match resp.rates.get("JPY") {
        Some(n) => *n,
        None => -1.0
    };

    //println!("EUR 1 = JPY {:}", yen);

    let f_5x7 = m::LedFont::new(Path::new("/home/pi/rpi-rgb-led-matrix/fonts/5x7.bdf"));
    let f_5x8 = m::LedFont::new(Path::new("/home/pi/rpi-rgb-led-matrix/fonts/5x8.bdf"));
    let f_4x6 = m::LedFont::new(Path::new("/home/pi/rpi-rgb-led-matrix/fonts/4x6.bdf"));
    let f_large = m::LedFont::new(Path::new("/home/pi/rpi-rgb-led-matrix/fonts/6x10.bdf"));
    let f_awe = m::LedFont::new(Path::new("/home/pi/otf2bdf-3.1/fontawesome_32.bdf"));

    canvas.draw_text(&f_large, 0, 7, 255, 255, 255, format!("¥{:.0}", yen), 0);
    
    let weather = reqwest::blocking::get("https://www.metaweather.com/api/location/1118370/")?
        .json::<api_query::WeatherRes>()?;


    let s = weather.weather[0].weather_state_abbr.to_string();

    let image = image::open(format!("images/small/{}.ico", s)).unwrap();
    canvas.show_image(&image, 47, 1);

    let temp_min = weather.weather[0].min_temp;
    let temp_max = weather.weather[0].max_temp;
    let temp_str = format!("{:>2.0}/{:>2.0}", temp_min, temp_max);
    println!("{}", temp_str);
    canvas.draw_text(&f_large, 63 - 16 - 2 * 6, 9, 255, 0, 0, format!("{:>2.0}", temp_max), 0);
    canvas.draw_text(&f_large, 63 - 16 - 2 * 6, 18, 0, 0, 255, format!("{:>2.0}", temp_min), 0);

    let sunrise_str = weather.sun_rise.format("↑%_H:%M");
    let sunset = weather.sun_set.format("↓%_H:%M");
    println!("sunrise: {}", sunrise_str);

    canvas.draw_text(&f_4x6, 64 - 6 * 4 - 1, 32-8, 255, 255, 255, sunrise_str.to_string(), 0);
    canvas.draw_text(&f_4x6, 64 - 6 * 4 - 1, 32-1, 255, 255, 255, sunset.to_string(), 0);

    let canvas = matrix.swap(canvas);
    thread::sleep(std::time::Duration::from_millis(150000));
    Ok(())
}


