extern crate image;

use image::{GenericImage, ImageBuffer, RgbImage, DynamicImage};

use rust_rpi_led_matrix as m;
use std::{thread, time};

fn main() {
    println!("Hello, world!");

    let options = m::LedMatrixOptions::new();
    let options = m::LedMatrixOptions { cols: 64, ..options };
    let matrix = m::LedMatrix::new(options);
    let canvas = matrix.offscreen_canvas();

    let image = image::open("./sun-yellow.png").unwrap();

    //let pixel = image.get_pixel(0, 0);
    //let pixel = image.get_pixel(0, 0);

    //println!("image: {:?}", image);

    let rgbi = match image {
        //DynamicImage::ImageRgb8(i) => { println!("rgb8"); i },
        DynamicImage::ImageRgba8(i) => { println!("rgba8"); i },
        _ => { println!("other"); image::ImageBuffer::new(32, 32) },
    };

    for y in 0..32 {
        for x in 0..32 {

            let px = rgbi.get_pixel(x,y);//.get(0);

            let image::Rgba([r, g, b, a]) = px; 

            //print!("{:?},", px);
            //print!("{}", if px[3] > 100 { "+ " } else { "  " });
            let brightness = (*a as u32 * 100 >> 8) as u8;
            if brightness > 5 {
                matrix.set_brightness(brightness);
                canvas.set_pixel(x as i32, y as i32, *r, *g, *b);
            } 
            println!("{} -> {}", px[3], (px[3] as u32 * 100 / 256) as u8);
        }
        println!();
    }

    /*{
        DynamicImage::ImageRgb8(i) => { println!("rgb8")},
        DynamicImage::ImageRgba8(i) => { println!("rgba8")},
        DynamicImage::ImageRgb16(i) => { println!("rgb16")},
        DynamicImage::ImageRgba16(i) => { println!("rgba16")},
        _ => { println!("other") },
    }*/
    //canvas.draw_line(0, 0, 63, 31, 255, 255, 255);
    //matrix.set_brightness(10);
    //thread::sleep(time::Duration::from_millis(1000));
    //canvas.draw_line(0, 31, 63, 0, 255, 255, 255);
    let canvas = matrix.swap(canvas);
    thread::sleep(time::Duration::from_millis(10000));

    //canvas.draw_circle(0, 0, 15, 255, 255, 255);
    //let canvas = matrix.swap(canvas);
    //thread::sleep(time::Duration::from_millis(5000));
}
