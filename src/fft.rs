// Reference: https://github.com/mpizenberg/fft2d/blob/main/src/slice.rs#L23
use image::{ImageBuffer, ImageFormat};
use plotters::prelude::*;
use rustfft::{num_complex::Complex, FftDirection, FftPlanner};

pub fn directional_fft_2d(
    direction: FftDirection,
    image_buffer: &mut Vec<Complex<f64>>,
    width: usize,
    height: usize,
) {
    let mut planner = FftPlanner::new();
    let fft_width = planner.plan_fft(width, direction);
    let mut scratch_vec = vec![Complex::default(); fft_width.get_inplace_scratch_len()];

    for row_buffer in image_buffer.chunks_exact_mut(width) {
        fft_width.process_with_scratch(row_buffer, &mut scratch_vec);
    }

    let mut transposed_img = transpose(image_buffer, width, height);
    let fft_height = planner.plan_fft(height, direction);
    scratch_vec.resize(fft_height.get_outofplace_scratch_len(), Complex::default());

    for (row, col) in transposed_img
        .chunks_exact_mut(height)
        .zip(image_buffer.chunks_exact_mut(height))
    {
        fft_height.process_outofplace_with_scratch(row, col, &mut scratch_vec);
    }
}

pub fn transpose<T: Copy + Default>(
    image_buffer: &mut Vec<T>,
    width: usize,
    height: usize,
) -> Vec<T> {
    let mut index = 0;
    let mut index_row = 0;
    let mut transposed_img = vec![T::default(); image_buffer.len()];
    for i in 0..height {
        index_row = i;
        for _ in 0..width {
            transposed_img[index_row] = image_buffer[index];
            index_row += height;
            index += 1;
        }
    }
    transposed_img
}

pub fn plot_fft(
    width: u32,
    height: u32,
    fft_buf: Vec<Complex<f64>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let buf_size = fft_buf.len();
    let mut abs_fft = vec![0u8; buf_size];

    for i in 0..buf_size {
        abs_fft[i] = (fft_buf[i].norm_sqr() / buf_size as f64) as u8;
    }

    let _ = image::save_buffer_with_format(
        "new_images/fft_image.png",
        abs_fft.as_mut_slice(),
        width,
        height,
        image::ColorType::L8,
        ImageFormat::Png,
    );

    Ok(())
}
