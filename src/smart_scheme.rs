use std::path::Path;

use color_eyre::Report;
use image::{imageops::FilterType, DynamicImage, GenericImageView, ImageReader};
use material_colors::hct::Hct;

use crate::color::format::argb_from_rgb;
use crate::scheme::{SchemeTypes, SchemesEnum};
use colorsys::Rgb;

pub struct SmartOpts {
    pub mode: SchemesEnum,
    pub variant: SchemeTypes,
}

fn calc_colorfulness(image: &DynamicImage) -> f64 {
    let rgb_image = image.to_rgb8();
    let pixels = rgb_image.pixels();

    let mut rg_sum = 0.0;
    let mut yb_sum = 0.0;
    let mut rg_sq_sum = 0.0;
    let mut yb_sq_sum = 0.0;
    let mut count = 0u64;

    for pixel in pixels {
        let r = pixel[0] as f64;
        let g = pixel[1] as f64;
        let b = pixel[2] as f64;

        let rg = (r - g).abs();
        let yb = (0.5 * (r + g) - b).abs();

        rg_sum += rg;
        yb_sum += yb;
        rg_sq_sum += rg * rg;
        yb_sq_sum += yb * yb;
        count += 1;
    }

    if count == 0 {
        return 0.0;
    }

    let mean_rg = rg_sum / count as f64;
    let mean_yb = yb_sum / count as f64;
    let variance_rg = (rg_sq_sum / count as f64) - (mean_rg * mean_rg);
    let variance_yb = (yb_sq_sum / count as f64) - (mean_yb * mean_yb);
    let std_rg = variance_rg.sqrt().max(0.0);
    let std_yb = variance_yb.sqrt().max(0.0);

    (std_rg.powi(2) + std_yb.powi(2)).sqrt() + 0.3 * (mean_rg.powi(2) + mean_yb.powi(2)).sqrt()
}

fn detect_variant(colorfulness: f64) -> SchemeTypes {
    match colorfulness {
        ..6.0 => SchemeTypes::SchemeMonochrome,
        6.0..20.0 => SchemeTypes::SchemeNeutral,
        20.0..70.0 => SchemeTypes::SchemeTonalSpot,
        _ => SchemeTypes::SchemeVibrant,
    }
}

fn detect_mode(image: &DynamicImage) -> SchemesEnum {
    let resized = image.resize_exact(1, 1, FilterType::Lanczos3);
    let pixel = resized.get_pixel(0, 0);

    let rgb = Rgb::from((pixel[0] as f64, pixel[1] as f64, pixel[2] as f64));
    let argb = argb_from_rgb(&rgb);
    let hct: Hct = argb.into();

    if hct.get_tone() > 60.0 {
        SchemesEnum::Light
    } else {
        SchemesEnum::Dark
    }
}

pub fn get_smart_opts(image_path: &Path) -> Result<SmartOpts, Report> {
    let img = ImageReader::open(image_path)?.decode()?;
    let thumb = img.thumbnail(128, 128);

    let mode = detect_mode(&thumb);
    let colorfulness = calc_colorfulness(&thumb);
    let variant = detect_variant(colorfulness);

    Ok(SmartOpts { mode, variant })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colorfulness_grayscale() {
        let img = DynamicImage::ImageRgb8(image::RgbImage::from_pixel(
            128,
            128,
            image::Rgb([128, 128, 128]),
        ));
        let score = calc_colorfulness(&img);
        assert!(
            score < 1.0,
            "Grayscale image should have near-zero colorfulness, got {score}"
        );
    }

    #[test]
    fn test_colorfulness_colorful() {
        let mut img_buf = image::RgbImage::new(128, 128);
        for y in 0..128 {
            for x in 0..128 {
                let r = ((x * 2) % 256) as u8;
                let g = ((y * 2) % 256) as u8;
                let b = (((x + y) * 2) % 256) as u8;
                img_buf.put_pixel(x, y, image::Rgb([r, g, b]));
            }
        }
        let img = DynamicImage::ImageRgb8(img_buf);
        let score = calc_colorfulness(&img);
        assert!(
            score > 20.0,
            "Colorful gradient should score high, got {score}"
        );
    }

    #[test]
    fn test_detect_variant_boundaries() {
        assert!(matches!(detect_variant(0.0), SchemeTypes::SchemeMonochrome));
        assert!(matches!(detect_variant(6.0), SchemeTypes::SchemeNeutral));
        assert!(matches!(detect_variant(20.0), SchemeTypes::SchemeTonalSpot));
        assert!(matches!(detect_variant(70.0), SchemeTypes::SchemeVibrant));
        assert!(matches!(detect_variant(100.0), SchemeTypes::SchemeVibrant));
    }

    #[test]
    fn test_detect_mode_dark() {
        let img = DynamicImage::ImageRgb8(image::RgbImage::from_pixel(
            128,
            128,
            image::Rgb([20, 20, 30]),
        ));
        assert!(matches!(detect_mode(&img), SchemesEnum::Dark));
    }

    #[test]
    fn test_detect_mode_light() {
        let img = DynamicImage::ImageRgb8(image::RgbImage::from_pixel(
            128,
            128,
            image::Rgb([240, 240, 250]),
        ));
        assert!(matches!(detect_mode(&img), SchemesEnum::Light));
    }
}
