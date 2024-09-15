use image::{DynamicImage, GenericImageView, Pixel, Rgba};

use super::base64::encode;
use super::lcs_diff;

#[derive(Debug)]
pub struct CompareImage {
    dimensions: (u32, u32),
    pixels: Vec<Rgba<u8>>,
}

impl CompareImage {
    pub fn new(dimensions: (u32, u32), pixels: Vec<Rgba<u8>>) -> Self {
        CompareImage { dimensions, pixels }
    }

    pub fn create_encoded_rows(&self) -> Vec<String> {
        let mut rows = Vec::new();
        let mut row = Vec::new();
        for pixel in &self.pixels {
            row.push(pixel.0[0]);
            row.push(pixel.0[1]);
            row.push(pixel.0[2]);
            row.push(pixel.0[3]);
            if row.len() == self.dimensions.0 as usize * 4 {
                rows.push(encode(&row));
                row.clear();
            }
        }
        rows
    }
}

pub fn diff(imga: CompareImage, imgb: CompareImage) -> Vec<lcs_diff::DiffResult<String>> {
    let imga = imga.create_encoded_rows();
    let imgb = imgb.create_encoded_rows();
    lcs_diff::diff(&imga, &imgb)
}

// Return a difference ratio between 0 and 1 for the two images
pub fn calculate_diff_ratio(image1: DynamicImage, image2: DynamicImage) -> f64 {
    use std::u8;

    let image1_raw = get_raw_pixels(&image1);
    let image2_raw = get_raw_pixels(&image2);

    // All color types wrap an 8-bit value for each channel
    let total_possible = (u8::MAX as usize * image1_raw.len()) as f64;

    image1_raw
        .into_iter()
        .zip(image2_raw)
        .map(|(a, b)| abs_diff(a, b) as u64)
        .sum::<u64>() as f64
        / total_possible
}

fn get_raw_pixels(image: &DynamicImage) -> Vec<u8> {
    let mut pixels = Vec::new();
    for pixel in image.pixels() {
        let rgba = pixel.2.to_rgba();
        pixels.push(rgba[0]);
        pixels.push(rgba[1]);
        pixels.push(rgba[2]);
    }
    pixels
}

/// abs(x - y) for u8
fn abs_diff(x: u8, y: u8) -> u8 {
    if x > y {
        return x - y;
    }
    return y - x;
}
