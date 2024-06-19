mod fft;
mod gaussian;
mod general_modifiers;
mod gray;
mod histogram;
mod manager;

use crate::fft::fft_image;
use crate::general_modifiers::flip_horizontal;
use crate::manager::get_luma8_image;
use image::{buffer::ConvertBuffer, GrayImage};
use rustfft::{num_complex::Complex, FftDirection};
use show_image::create_window;
use std::result::Result;
use std::{env, fs};

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

    let _ = fft_image(&image_name);
    let _ = flip_horizontal(&image_name);

    Ok(())
}
