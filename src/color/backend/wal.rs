use crate::color::base16::PaletteBackend;
use colorsys::Rgb;
use image::{imageops::FilterType, RgbImage};
use palette::IntoColor;
use palette::Oklab;

pub struct WalBackend {
    pub clusters: usize,
    pub resize_width: u32,
    pub sample_step: u32,
    pub iterations: usize,
}

impl Default for WalBackend {
    fn default() -> Self {
        Self {
            clusters: 16,
            resize_width: 300,
            sample_step: 2,
            iterations: 8,
        }
    }
}

impl PaletteBackend for WalBackend {
    fn extract(&self, image: &RgbImage) -> Vec<Rgb> {
        let resized = resize_image(image, self.resize_width);
        let samples = sample_pixels(&resized, self.sample_step);

        if samples.is_empty() {
            return Vec::new();
        }

        let centroids = kmeans(&samples, self.clusters, self.iterations);

        let mut colors: Vec<Rgb> = centroids
            .into_iter()
            .map(|c| Rgb::new(c[0].into(), c[1].into(), c[2].into(), None))
            .collect();

        colors.sort_by(|a, b| {
            let sa = score_colorsys(a);
            let sb = score_colorsys(b);
            sb.partial_cmp(&sa).unwrap()
        });

        colors
    }
}

fn resize_image(image: &RgbImage, target_width: u32) -> RgbImage {
    let (w, h) = image.dimensions();

    if w <= target_width {
        return image.clone();
    }

    let scale = target_width as f32 / w as f32;
    let target_height = (h as f32 * scale).round() as u32;

    image::imageops::resize(image, target_width, target_height, FilterType::Triangle)
}

fn sample_pixels(image: &RgbImage, step: u32) -> Vec<[f32; 3]> {
    let mut out = Vec::new();

    for y in (0..image.height()).step_by(step as usize) {
        for x in (0..image.width()).step_by(step as usize) {
            let p = image.get_pixel(x, y);
            out.push([p[0] as f32, p[1] as f32, p[2] as f32]);
        }
    }

    out
}

fn kmeans(samples: &[[f32; 3]], k: usize, iterations: usize) -> Vec<[f32; 3]> {
    let mut centroids: Vec<[f32; 3]> = samples.iter().take(k).cloned().collect();

    for _ in 0..iterations {
        let mut buckets: Vec<Vec<[f32; 3]>> = vec![Vec::new(); k];

        for sample in samples {
            let mut best = 0;
            let mut best_dist = f32::MAX;

            for (i, centroid) in centroids.iter().enumerate() {
                let d = (sample[0] - centroid[0]).powi(2)
                    + (sample[1] - centroid[1]).powi(2)
                    + (sample[2] - centroid[2]).powi(2);

                if d < best_dist {
                    best = i;
                    best_dist = d;
                }
            }

            buckets[best].push(*sample);
        }

        for (i, bucket) in buckets.iter().enumerate() {
            if bucket.is_empty() {
                continue;
            }

            let mut sum = [0.0; 3];
            for p in bucket {
                sum[0] += p[0];
                sum[1] += p[1];
                sum[2] += p[2];
            }

            centroids[i] = [
                sum[0] / bucket.len() as f32,
                sum[1] / bucket.len() as f32,
                sum[2] / bucket.len() as f32,
            ];
        }
    }

    centroids
}

fn score_colorsys(c: &Rgb) -> f32 {
    let linear = [
        c.red() as f32 / 255.0,
        c.green() as f32 / 255.0,
        c.blue() as f32 / 255.0,
    ];

    let srgb = palette::Srgb::new(linear[0], linear[1], linear[2]);
    let lab: Oklab = srgb.into_linear().into_color();

    let lightness = lab.l; // 0–1
    let chroma = (lab.a * lab.a + lab.b * lab.b).sqrt();

    chroma * 2.0 + (0.5 - lightness).abs()
}
