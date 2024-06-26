// Reference: https://github.com/mpizenberg/fft2d/blob/main/src/slice.rs#L23
use crate::manager::get_luma8_image;
use fft2d::slice::{fft_2d, fftshift};
use image::DynamicImage::ImageLuma8;
use image::{GrayImage, ImageBuffer, ImageFormat};
use rustfft::{num_complex::Complex, FftDirection, FftPlanner};

pub fn fft_image(image_name: &str) -> Result<(), &str> {
    let get_path: String = format!("images/{}", image_name);
    let gray_image = get_luma8_image(&get_path);

    if gray_image.is_none() {
        return Err("Image not found".into());
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

    let fft = plot_fft(height, width, plot_buff.clone());
    let set_path: String = format!("new_images/fft_mag_{}", image_name);
    ImageLuma8(fft).save(set_path).unwrap();

    let phase = plot_phase(height, width, plot_buff);
    let set_path: String = format!("new_images/fft_phase_{}", image_name);
    ImageLuma8(phase).save(set_path).unwrap();

    Ok(())
}

pub fn plot_fft(width: u32, height: u32, fft_buf: Vec<Complex<f64>>) -> GrayImage {
    let buf_size = fft_buf.len();
    let mut mag_fft = vec![0f64; buf_size];

    for i in 0..buf_size {
        mag_fft[i] = fft_buf[i].norm().ln();
    }

    let max_norm = mag_fft.iter().cloned().fold(0.0 / 0.0, f64::max);

    let norm_fft: Vec<u8> = mag_fft
        .into_iter()
        .map(|x| ((x / max_norm) * 255.0) as u8)
        .collect();

    GrayImage::from_raw(width, height, norm_fft).unwrap()
}

pub fn plot_phase(width: u32, height: u32, fft_buf: Vec<Complex<f64>>) -> GrayImage {
    let buf_size = fft_buf.len();
    let mut tmp_fft = vec![0f64; buf_size];
    let mut phase_fft = vec![0u8; buf_size];

    for i in 0..buf_size {
        tmp_fft[i] = (fft_buf[i].im / fft_buf[i].re)
            .atan()
            .rem_euclid(2.0 * std::f64::consts::PI)
            .to_degrees();
    }

    let max_phase = tmp_fft.iter().cloned().fold(0.0 / 0.0, f64::max);
    let min_phase = tmp_fft.iter().cloned().fold(-0.0 / 0.0, f64::min);

    for i in 0..buf_size {
        phase_fft[i] = ((tmp_fft[i] - min_phase) / (max_phase - min_phase) * 255.0) as u8;
    }

    GrayImage::from_raw(width, height, phase_fft).unwrap()
}
