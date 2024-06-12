use image::DynamicImage::ImageLuma8;
use image::{buffer::ConvertBuffer, GrayImage, ImageBuffer, ImageFormat, Luma, Pixel};
use image::{Rgb, RgbImage};
use plotters::prelude::*;
use std::cmp::max;
use std::env;
use std::f64::consts::{E, PI};
use std::path::Path;
use std::result::Result;

#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn get_image(image_name: &String) -> Option<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let img = image::open(&Path::new(image_name))
        .expect("File not found!")
        .to_rgb8();

    match img.is_empty() {
        false => Some(img),
        true => None,
    }
}

fn get_red_channel(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u8> {
    let mut red_channel: Vec<u8> = Vec::new();

    for pixel in image.pixels() {
        red_channel.push(pixel.to_rgb().0[0]);
    }

    return red_channel;
}

fn get_green_channel(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u8> {
    let mut green_channel: Vec<u8> = Vec::new();

    for pixel in image.pixels() {
        green_channel.push(pixel.to_rgb().0[1]);
    }

    return green_channel;
}

fn get_bw_channel(image: ImageBuffer<Luma<u8>, Vec<u8>>) -> Vec<u8> {
    let mut bw_channel: Vec<u8> = Vec::new();

    for pixel in image.pixels() {
        bw_channel.push(pixel.0[0]);
    }

    return bw_channel;
}

fn get_blue_channel(image: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> Vec<u8> {
    let mut blue_channel: Vec<u8> = Vec::new();

    for pixel in image.pixels() {
        blue_channel.push(pixel.to_rgb().0[2]);
    }

    return blue_channel;
}

fn generate_red_histogram(image_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let get_path: String = format!("images/{}", image_name);
    let hist_out_name: &str = "plotters-doc-data/red_histogram.png";
    let img = get_image(&get_path).unwrap();

    let red_channel = get_red_channel(&img);
    let mut red_u32_channel: Vec<u32> = Vec::new();

    for val in red_channel.iter() {
        red_u32_channel.push(*val as u32);
    }

    let root = BitMapBackend::new(hist_out_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Red Image Histogram", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..300u32).into_segmented(), 0u32..6000u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(RED.mix(0.5).filled())
            .data(red_u32_channel.iter().map(|x: &u32| (*x, 1))),
    )?;

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    Ok(())
}

fn generate_green_histogram(image_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let get_path: String = format!("images/{}", image_name);
    let hist_out_name: &str = "plotters-doc-data/green_histogram.png";
    let img = get_image(&get_path).unwrap();

    let green_channel = get_green_channel(&img);
    let mut green_u32_channel: Vec<u32> = Vec::new();

    for val in green_channel.iter() {
        green_u32_channel.push(*val as u32);
    }

    let root = BitMapBackend::new(hist_out_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Green Image Histogram", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..300u32).into_segmented(), 0u32..6000u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(GREEN.mix(0.5).filled())
            .data(green_u32_channel.iter().map(|x: &u32| (*x, 1))),
    )?;

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    Ok(())
}

fn generate_blue_histogram(image_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let get_path: String = format!("images/{}", image_name);
    let hist_out_name: &str = "plotters-doc-data/blue_histogram.png";
    let img = get_image(&get_path).unwrap();

    let blue_channel = get_blue_channel(&img);
    let mut blue_u32_channel: Vec<u32> = Vec::new();

    for val in blue_channel.iter() {
        blue_u32_channel.push(*val as u32);
    }

    let root = BitMapBackend::new(hist_out_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Blue Image Histogram", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..300u32).into_segmented(), 0u32..6000u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLUE.mix(0.5).filled())
            .data(blue_u32_channel.iter().map(|x: &u32| (*x, 1))),
    )?;

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    Ok(())
}

fn generate_bw_histogram(image_name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let get_path: String = format!("images/{}", image_name);
    let hist_out_name: &str = "plotters-doc-data/bw_histogram.png";
    let img = get_image(&get_path).unwrap();
    let gray_image: GrayImage = img.convert();

    let bw_channel = get_bw_channel(gray_image);
    let mut bw_u32_channel: Vec<u32> = Vec::new();

    for val in bw_channel.iter() {
        bw_u32_channel.push(*val as u32);
    }

    let root = BitMapBackend::new(hist_out_name, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .margin(5)
        .caption("Intensity Image Histogram", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..300u32).into_segmented(), 0u32..6000u32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(WHITE.mix(0.3))
        .y_desc("Count")
        .x_desc("Bucket")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    chart.draw_series(
        Histogram::vertical(&chart)
            .style(BLACK.mix(0.5).filled())
            .data(bw_u32_channel.iter().map(|x: &u32| (*x, 1))),
    )?;

    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");

    Ok(())
}

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

fn gaussian_blur_png(image_name: &String, radius: i32) -> Result<(), &str> {
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

fn convert_to_gray_png(image_name: &String) -> Result<(), &str> {
    let get_path: String = format!("images/{}", image_name);
    let set_path: String = format!("new_images/gray_{}", image_name);
    let img = get_image(&get_path);

    if img.is_none() {
        return Err("Image not found");
    }

    let gray_image: GrayImage = img.unwrap().convert();

    ImageLuma8(gray_image).save(set_path).unwrap();

    Ok(())
}

fn get_image_type(img_type: &String) -> String {
    let mut period_index: usize = 0;

    for ch in img_type.chars() {
        if ch == '.' {
            return img_type[period_index..].to_string();
        }
        period_index += 1;
    }

    return String::from("");
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

    let image_type: String = get_image_type(&args[1]);
    let image_name: String = args[1].clone();
    let op: String = args[2].clone();

    match image_type.as_str() {
        ".jpeg" => println!("jpeg image"),
        ".png" => println!("png image"),
        _ => println!("Please, enter a compatible image file"),
    };

    // let _ = match op.as_str() {
    //     "gray" => convert_to_gray_png(&image_name),
    //     "gaussian_blur" => {
    //         let parameter: String = args[3].clone();
    //         gaussian_blur_png(&image_name, parameter.parse::<i32>().unwrap())
    //     }
    //     _ => Err("Please, enter a valid function"),
    // };

    Ok(())
}
