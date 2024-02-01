use color_eyre::Report;
use image::DynamicImage;
use material_colors::utils::image::source_color_from_image as other_source_color_from_image;

use image::Rgba;
use material_colors::Argb;
use image::ImageBuffer;

pub fn fetch_image(url: &str) -> Result<DynamicImage, Report> {
    let bytes = reqwest::blocking::get(url)?.bytes()?;
    Ok(image::load_from_memory(&bytes)?)
}

pub fn source_color_from_image(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<[u8; 4], Report> {
    let pixels: Vec<Argb> = img.pixels().fold(vec![], |mut pixels, pixel| {
        // creating ARGB from RGBA
        pixels.push([pixel[3], pixel[0], pixel[1], pixel[2]]);

        pixels
   });

    Ok(other_source_color_from_image(&pixels))
}