// Reference: https://github.com/mpizenberg/fft2d/blob/main/src/slice.rs#L23
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

pub fn plot_fft(fft_buf: Vec<Complex<f64>>) {
    let abs_fft = fft_buf.iter().map(|x| *x = x.abs());
}
