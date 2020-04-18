extern crate image;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use image::{GenericImage, ImageBuffer, RgbImage, DynamicImage};

use rust_rpi_led_matrix as m;
use std::{thread, time};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct APIRes {
    amount: f64,
    base: String,
    date: String,
    rates: HashMap<String, f64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let options = m::LedMatrixOptions::new();
    let options = m::LedMatrixOptions { cols: 64, ..options };
    let matrix = m::LedMatrix::new(options);
    let canvas = matrix.offscreen_canvas();

    let image = image::open("../../sun-red.png").unwrap();
    let image2 = image::open("sun-yellow.png").unwrap();

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

    let f = m::LedFont::new(Path::new("/home/pi/rpi-rgb-led-matrix/fonts/5x7.bdf"));

    canvas.draw_text(f, 6, 6, 255, 255, 255, format!("¥{}", yen), 0);
    //canvas.vertical_draw_text(f, 16, 16, 255, 255, 255, "Test".to_string(), 0);
    
    let canvas = matrix.swap(canvas);
    thread::sleep(time::Duration::from_millis(10000));
    Ok(())
}


