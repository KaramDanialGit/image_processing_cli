use image::Rgb;
use image::{ImageBuffer, Luma, Pixel};
use std::path::Path;

pub fn get_image(image_name: &String) -> Option<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let img = image::open(&Path::new(image_name))
        .expect("File not found!")
        .to_rgb8();

    match img.is_empty() {
        false => Some(img),
        true => None,
    }
}

pub fn get_image_type(img_type: &String) -> String {
    let mut period_index: usize = 0;

    for ch in img_type.chars() {
        if ch == '.' {
            return img_type[period_index..].to_string();
        }
        period_index += 1;
    }

    return String::from("");
}

pub fn get_red_channel(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u8> {
    let mut red_channel: Vec<u8> = Vec::new();

    for pixel in image.pixels() {
        red_channel.push(pixel.to_rgb().0[0]);
    }

    return red_channel;
}

pub fn get_green_channel(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u8> {
    let mut green_channel: Vec<u8> = Vec::new();

    for pixel in image.pixels() {
        green_channel.push(pixel.to_rgb().0[1]);
    }

    return green_channel;
}

pub fn get_bw_channel(image: ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<u8> {
    let mut bw_channel: Vec<u8> = Vec::new();

    for pixel in image.pixels() {
        bw_channel.push(pixel.0[0]);
    }

    return bw_channel;
}

pub fn get_blue_channel(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u8> {
    let mut blue_channel: Vec<u8> = Vec::new();

    for pixel in image.pixels() {
        blue_channel.push(pixel.to_rgb().0[2]);
    }

    return blue_channel;
}
