use color_eyre::{Help, Report};
use image::ImageError;
use image::{imageops::FilterType, GenericImageView};
use material_color_utilities_rs::{quantize::quantizer_celebi::QuantizerCelebi, score::score};

use super::color::Color;

pub fn source_color_from_image(image: &String) -> Result<Vec<[u8; 4]>, Report> {
    info!("Opening image {}", image);
    let img = match image::open(image) {
        Ok(img) => img,
        Err(ImageError::Unsupported(e)) => {
            return Err(Report::new(e)
                .note("This might have failed due to the image format not being supported.")
                .suggestion(
                    "Try using another image that is valid (for example `wallpaper.png`).",
                ))
        }
        Err(ImageError::IoError(e)) => {
            return Err(Report::new(e)
                .note("This might have failed due to the image not being found.")
                .suggestion(
                    "Try using an image that exists or make sure the path provided is valid.",
                ))
        }
        Err(e) => return Err(Report::new(e)),
    };

    let (width, height) = img.dimensions();

    let newwidth = width / 64;
    let newheight = height / 64;

    info!("Resizing image to {}x{} from {}x{}", newheight, newwidth, width, height);
    let resized_img = img.resize(newheight, newwidth, FilterType::Lanczos3);

    let mut pixels = vec![];

    for pixel in resized_img.pixels() {
        // TODO: Make this use Color::new()
        let color: Color = Color {
            red: pixel.2[0],
            green: pixel.2[1],
            blue: pixel.2[2],
            alpha: pixel.2[3],
        };

        let argb: [u8; 4] = [color.alpha, color.red, color.green, color.blue];

        pixels.push(argb);
    }

    let theme = QuantizerCelebi::quantize(&mut QuantizerCelebi, &pixels, 10);

    let score = score(&theme);

    Ok(score)
}
