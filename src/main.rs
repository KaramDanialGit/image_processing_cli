mod fft;
mod gaussian;
mod gray;
mod histogram;
mod manager;

use crate::fft::directional_fft_2d;
use crate::manager::get_image;
use image::{buffer::ConvertBuffer, GrayImage, ImageBuffer, Rgb};
use rustfft::{num_complex::Complex, FftDirection};
use std::env;
use std::result::Result;

fn print_cmd_debug() {
    println!("--------------------------------------------------------");
    println!("Use the following command format: <image_name> <command>");
    println!("");
    println!("<image_type> to see supported image functions (e.g. png)");
    println!("--------------------------------------------------------");
}

fn main() -> Result<(), &'static str> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_cmd_debug();
        return Err("Please, enter a valid command");
    }

    let image_type: String = manager::get_image_type(&args[1]);
    let image_name: &String = &args[1];
    let op: &String = &args[2];
    let parameter: Option<String> = Some(args[3].clone());

    match image_type.as_str() {
        ".jpeg" => println!("jpeg image"),
        ".png" => println!("png image"),
        _ => println!("Please, enter a compatible image file"),
    };

    // let _ = match op.as_str() {
    //     "gray" => gray::convert_to_gray_png(image_name),
    //     "gaussian_blur" => gaussian::gaussian_blur_png(
    //         image_name,
    //         parameter
    //             .expect("No parameter passed after funciton call")
    //             .parse::<i32>()
    //             .unwrap(),
    //     ),
    //     _ => Err("Please, enter a valid function"),
    // };
    let get_path: String = format!("images/{}", image_name);
    let image_buffer = manager::get_image(&get_path);

    if image_buffer.is_none() {
        return Err("Image not found");
    }

    let buf = image_buffer.unwrap();
    let (width, height) = buf.dimensions();
    let gray_image: GrayImage = buf.convert();
    let bw_data = manager::get_bw_channel(gray_image);

    let mut to_fft_buffer: Vec<Complex<f64>> = Vec::new();

    for element in bw_data.iter() {
        to_fft_buffer.push(Complex {
            re: *element as f64,
            im: 0.0,
        });
    }

    let _ = fft::directional_fft_2d(
        FftDirection::Forward,
        &mut to_fft_buffer,
        width as usize,
        height as usize,
    );

    Ok(())
}
