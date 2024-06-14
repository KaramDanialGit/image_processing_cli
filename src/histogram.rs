use crate::manager::{
    get_blue_channel, get_bw_channel, get_green_channel, get_image, get_red_channel,
};

use image::{buffer::ConvertBuffer, GrayImage};
use plotters::prelude::*;

pub fn generate_red_histogram(image_name: &String) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn generate_green_histogram(image_name: &String) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn generate_blue_histogram(image_name: &String) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn generate_bw_histogram(image_name: &String) -> Result<(), Box<dyn std::error::Error>> {
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
