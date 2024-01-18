use color_eyre::Report;
use image::{imageops::FilterType, DynamicImage, GenericImageView};
use material_colors::quantize::quantizer::Quantizer;
use material_colors::quantize::quantizer_celebi::QuantizerCelebi;

use material_colors::score::Score;

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

    let result = QuantizerCelebi.quantize(&pixels, 128, None);

    let score = Score::score(&result.color_to_count, None, None, None);

    Ok(score)
}

fn generate_pixels(image: &DynamicImage) -> Vec<[u8; 4]> {
    let mut pixels = vec![];
    for pixel in image.pixels() {
        let red = pixel.2[0];
        let green = pixel.2[1];
        let blue = pixel.2[2];
        let alpha = pixel.2[3];

        if alpha < 255 {
            continue;
        }

        let argb: [u8; 4] = [alpha, red, green, blue];

        pixels.push(argb);
    }
    pixels
}
