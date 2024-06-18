mod fft;
mod gaussian;
mod gray;
mod histogram;
mod manager;

use crate::fft::{directional_fft_2d, plot_fft};
use crate::manager::get_luma8_image;
use fft2d::slice::{fft_2d, fftshift};
use image::DynamicImage::ImageLuma8;
use image::{buffer::ConvertBuffer, GrayImage};
use rustfft::{num_complex::Complex, FftDirection};
use show_image::create_window;
use std::result::Result;
use std::{env, fs};

fn view_fft_norm(width: u32, height: u32, img_buffer: &[Complex<f64>]) -> GrayImage {
    let fft_log_norm: Vec<f64> = img_buffer.iter().map(|c| c.norm().ln()).collect();
    let max_norm = fft_log_norm.iter().cloned().fold(0.0 / 0.0, f64::max);
    let fft_norm_u8: Vec<u8> = fft_log_norm
        .into_iter()
        .map(|x| ((x / max_norm) * 255.0) as u8)
        .collect();
    GrayImage::from_raw(width, height, fft_norm_u8).unwrap()
}

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
    let gray_image = manager::get_luma8_image(&get_path);

    if gray_image.is_none() {
        return Err("Image not found");
    }

    let buf = gray_image.unwrap();
    let (width, height) = buf.dimensions();

    let mut to_fft_buffer: Vec<Complex<f64>> = buf
        .as_raw()
        .iter()
        .map(|&x| Complex::new(x as f64 / 255.0, 0.0))
        .collect();

    fft_2d(
        width as usize,
        height as usize,
        to_fft_buffer.as_mut_slice(),
    );

    let plot_buff = fftshift(
        width as usize,
        height as usize,
        to_fft_buffer.as_mut_slice(),
    );

    let fft = view_fft_norm(height, width, &plot_buff);

    let set_path: String = format!("new_images/fft_test_{}", image_name);
    ImageLuma8(fft).save(set_path).unwrap();

    Ok(())
}
