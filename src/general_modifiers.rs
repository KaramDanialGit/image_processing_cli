use crate::manager::get_rgb8_image;
use image::{
    imageops::{flip_horizontal, flip_vertical},
    DynamicImage::ImageRgb8,
    ImageBuffer, Rgb,
};
use std::path::Path;

pub fn flip_h(image_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let get_path: String = format!("images/{}", image_name);

    let img = image::open(&Path::new(&get_path))
        .expect("File not found!")
        .to_rgb8();

    let new_image = flip_horizontal(&img);
    let save_path: String = format!("new_images/h_flipped_{}", image_name);
    ImageRgb8(new_image).save(save_path).unwrap();

    Ok(())
}

pub fn flip_v(image_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let get_path: String = format!("images/{}", image_name);

    let img = image::open(&Path::new(&get_path))
        .expect("File not found!")
        .to_rgb8();

    let new_image = flip_vertical(&img);
    let save_path: String = format!("new_images/v_flipped_{}", image_name);
    ImageRgb8(new_image).save(save_path).unwrap();

    Ok(())
}
