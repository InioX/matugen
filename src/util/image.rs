use color_eyre::Report;
use image::{imageops::FilterType, DynamicImage, GenericImageView};
use material_color_utilities_rs::{quantize::quantizer_celebi::QuantizerCelebi, score::score};

use super::color::Color;

pub fn fetch_image(url: &str) -> Result<DynamicImage, Report> {
    let bytes = reqwest::blocking::get(url)?.bytes()?;
    Ok(image::load_from_memory(&bytes)?)
}

pub fn source_color_from_image(img: DynamicImage) -> Result<Vec<[u8; 4]>, Report> {
    let (width, height) = img.dimensions();

    let (new_width, new_height) = (width / 64, height / 64);

    info!(
        "Resizing image from <b><cyan>{}x{}</> to <b><cyan>{}x{}</>",
        width, height, new_height, new_width
    );
    let resized_img: DynamicImage = img.resize(new_height, new_width, FilterType::Lanczos3);

    let pixels: Vec<[u8; 4]> = generate_pixels(&resized_img);

    let theme = QuantizerCelebi::quantize(&mut QuantizerCelebi, &pixels, 128);

    let score = score(&theme);

    Ok(score)
}

fn generate_pixels(image: &DynamicImage) -> Vec<[u8; 4]> {
    let mut pixels = vec![];
    for pixel in image.pixels() {
        let color: Color = Color {
            red: pixel.2[0],
            green: pixel.2[1],
            blue: pixel.2[2],
            alpha: pixel.2[3],
        };

        if color.alpha < 255 {
            continue;
        }

        let argb: [u8; 4] = [color.alpha, color.red, color.green, color.blue];

        pixels.push(argb);
    }
    pixels
}
