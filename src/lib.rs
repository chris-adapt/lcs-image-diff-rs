extern crate base64;
extern crate image;
extern crate lcs_diff;

mod diff;
mod image_creator;

pub use base64::DecodeError;
use diff::*;
use image::*;
use image_creator::*;

/// Accepts two mutable references to `image::DynamicImage` and rate.
/// Returns diff `image::DynamicImage` and marks removed and added
/// parts on input images.
///
/// # Examples
///
/// ```no_run
/// extern crate image;
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<Error>> {
/// use lcs_image_diff::compare;
///
/// let mut before = image::open("before.png")?;
/// let mut after = image::open("after.png")?;
///
/// let diff = compare(&mut before, &mut after, 100.0 / 256.0)?;
///
/// before.save("marked_before.png")?;
/// after.save("marked_after.png")?;
/// diff.save("diff.png")?;
/// # Ok(())
/// # }
/// ```
pub fn compare(
    before: &mut DynamicImage,
    after: &mut DynamicImage,
    rate: f32,
) -> Result<DynamicImage, DecodeError> {
    let compare_before = CompareImage::new(
        before.dimensions(),
        before.pixels().map(|pix| pix.2).collect(),
    );
    let compare_after = CompareImage::new(
        after.dimensions(),
        after.pixels().map(|pix| pix.2).collect(),
    );
    let result = diff(compare_before, compare_after);

    let mut added: Vec<usize> = Vec::new();
    let mut removed: Vec<usize> = Vec::new();
    for d in result.iter() {
        match d {
            &lcs_diff::DiffResult::Added(ref a) => added.push(a.new_index.unwrap()),
            &lcs_diff::DiffResult::Removed(ref r) => removed.push(r.old_index.unwrap()),
            _ => (),
        }
    }

    mark_org_image(before, RED, rate, &removed);
    mark_org_image(after, GREEN, rate, &added);

    get_diff_image(before.dimensions().0, after.dimensions().0, &result, rate)
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
