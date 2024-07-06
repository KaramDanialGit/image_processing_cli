use crate::manager::get_rgb8_image;
use image::{
    imageops::{
        colorops::invert, crop, flip_horizontal, flip_vertical, rotate180, rotate270, rotate90,
    },
    DynamicImage::ImageRgb8,
    ImageBuffer, Rgb,
};
use std::path::Path;

pub fn flip_h(image_name: &String) -> Result<(), &str> {
    let get_path: String = format!("images/{}", image_name);

    let img = image::open(&Path::new(&get_path))
        .expect("File not found!")
        .to_rgb8();

    let new_image = flip_horizontal(&img);
    let save_path: String = format!("new_images/h_flipped_{}", image_name);
    ImageRgb8(new_image).save(save_path).unwrap();

    Ok(())
}

pub fn flip_v(image_name: &String) -> Result<(), &str> {
    let get_path: String = format!("images/{}", image_name);

    let img = image::open(&Path::new(&get_path))
        .expect("File not found!")
        .to_rgb8();

    let new_image = flip_vertical(&img);
    let save_path: String = format!("new_images/v_flipped_{}", image_name);
    ImageRgb8(new_image).save(save_path).unwrap();

    Ok(())
}

pub fn rotate_image(image_name: &String, angle: i32) -> Result<(), &str> {
    let get_path: String = format!("images/{}", image_name);
    let mut valid_angle = true;

    let mut img = image::open(&Path::new(&get_path))
        .expect("File not found!")
        .to_rgb8();

    let (width, height) = img.dimensions();

    let mut new_image = ImageBuffer::new(width, height);

    match angle {
        90 => new_image = rotate90(&img),
        180 => new_image = rotate180(&img),
        270 => new_image = rotate270(&img),
        _ => {
            valid_angle = false;
            ()
        }
    }

    if !valid_angle {
        return Err("Please enter valid angle");
    }

    let save_path = format!("new_images/rotate_image_{}", image_name);
    ImageRgb8(new_image).save(save_path).unwrap();

    Ok(())
}

pub fn crop_image(image_name: &String, new_width: u32, new_height: u32) -> Result<(), &str> {
    let get_path: String = format!("images/{}", image_name);

    let mut img = image::open(&Path::new(&get_path))
        .expect("File not found!")
        .to_rgb8();

    let (width, height) = img.dimensions();

    let new_image = crop(&mut img, new_width, new_height, width as u32, height as u32);

    let save_path: String = format!("new_images/cropped_{}", image_name);
    ImageRgb8(new_image.to_image()).save(save_path).unwrap();

    Ok(())
}

pub fn invert_image(image_name: &String) -> Result<(), &str> {
    let get_path: String = format!("images/{}", image_name);

    let mut img = image::open(&Path::new(&get_path))
        .expect("File not found!")
        .to_rgb8();

    invert(&mut img);

    let save_path: String = format!("new_images/inverted_{}", image_name);
    ImageRgb8(img).save(save_path).unwrap();

    Ok(())
}
