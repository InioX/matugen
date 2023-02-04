use crate::util::color::ARGB;
use ahash::AHashMap;

/// Creates a dictionary with keys of colors, and values of count of the color
#[derive(Debug, Default, Clone)]
pub struct QuantizerMap;

impl QuantizerMap {
    pub fn quantize(pixels: &[ARGB]) -> AHashMap<ARGB, u32> {
        let mut pixel_by_count = AHashMap::new();

        for pixel in pixels {
            pixel_by_count
                .entry(*pixel)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        pixel_by_count
    }
}
