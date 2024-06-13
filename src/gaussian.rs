use crate::manager::get_image;
use image::{ImageFormat, Pixel, Rgb, RgbImage};
use std::cmp::max;
use std::f64::consts::{E, PI};

fn generate_gaussian_kernel(radius: i32) -> Vec<Vec<f32>> {
    let sigma = max(radius / 2, 1) as f32;
    let kernel_size = (2 * radius) + 1;

    let mut sum = 0.0;
    let mut kernel: Vec<Vec<f32>> = Vec::new();

    for _ in 0..kernel_size {
        let zero_vec = vec![0.0; kernel_size as usize];
        kernel.push(zero_vec.to_vec());
    }

    for x in -radius..radius {
        for y in -radius..radius {
            let exponent_num = -(x * x + y * y) as f32;
            let exponent_denom = 2.0 * sigma * sigma as f32;

            let e_expression = E.powf((exponent_num / exponent_denom).into()) as f32;
            let e_value = e_expression / (2.0 * PI as f32 * sigma * sigma);

            kernel[(x + radius) as usize][(y + radius) as usize] = e_value;
            sum += e_value;
        }
    }

    for x in 0..kernel_size {
        for y in 0..kernel_size {
            kernel[x as usize][y as usize] /= sum;
        }
    }

    kernel
}

pub fn gaussian_blur_png(image_name: &String, radius: i32) -> Result<(), &str> {
    let get_path: String = format!("images/{}", image_name);
    let set_path: String = format!("new_images/blur_{}", image_name);
    let kernel = generate_gaussian_kernel(radius);

    let img = get_image(&get_path);

    if img.is_none() {
        return Err("Image not found");
    }

    let template = img.unwrap().clone();
    let (rows, cols) = template.dimensions();
    let mut new_image = RgbImage::new(rows, cols);

    for x in radius..cols as i32 - radius {
        for y in radius..rows as i32 - radius {
            let mut red: f32 = 0.0;
            let mut green: f32 = 0.0;
            let mut blue: f32 = 0.0;

            for kernel_x in -radius..radius {
                for kernel_y in -radius..radius {
                    let kernel_val =
                        kernel[(kernel_x + radius) as usize][(kernel_y + radius) as usize];

                    red += template
                        .get_pixel((x - kernel_x) as u32, (y - kernel_y) as u32)
                        .to_rgb()
                        .0[0] as f32
                        * kernel_val;

                    green += template
                        .get_pixel((x - kernel_x) as u32, (y - kernel_y) as u32)
                        .to_rgb()
                        .0[1] as f32
                        * kernel_val;

                    blue += template
                        .get_pixel((x - kernel_x) as u32, (y - kernel_y) as u32)
                        .to_rgb()
                        .0[2] as f32
                        * kernel_val;
                }
            }

            new_image.put_pixel(
                x as u32,
                y as u32,
                Rgb([red as u8, green as u8, blue as u8]),
            );
        }
    }

    let _ = new_image.save_with_format(set_path, ImageFormat::Png);

    Ok(())
}
