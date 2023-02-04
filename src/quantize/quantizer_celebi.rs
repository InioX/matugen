use ahash::AHashMap;

use crate::util::color::ARGB;

use super::{quantizer_wsmeans::QuantizerWsmeans, quantizer_wu::QuantizerWu};

/// An image quantizer that improves on the quality of a standard K-Means algorithm by setting the
/// K-Means initial state to the output of a Wu quantizer, instead of random centroids. Improves on
/// speed by several optimizations, as implemented in Wsmeans, or Weighted Square Means, K-Means with
/// those optimizations.
///
/// This algorithm was designed by M. Emre Celebi, and was found in their 2011 paper, Improving
/// the Performance of K-Means for Color Quantization.
///
/// https://arxiv.org/abs/1101.0395
pub struct QuantizerCelebi;

impl QuantizerCelebi {
    /// Reduce the number of colors needed to represented the input, minimizing the difference
    /// between the original image and the recolored image.
    ///
    /// # Arguments
    ///
    /// * `pixels` - Colors in ARGB format.
    /// * `max_colors` - The number of colors to divide the image into. A lower number of colors
    /// may be returned.
    ///
    /// # Returns
    ///
    /// Map with keys of colors in ARGB format, and values of number of pixels in the original
    /// image that correspond to the color in the quantized image.
    pub fn quantize(&mut self, pixels: &[ARGB], max_colors: usize) -> AHashMap<ARGB, u32> {
        let mut quantizer_wu = QuantizerWu::new();
        let colors = quantizer_wu.quantize(pixels, max_colors);

        QuantizerWsmeans.quantize(pixels, &colors, max_colors)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RED: ARGB = [0xff, 0xff, 0x00, 0x00];
    const GREEN: ARGB = [0xff, 0x00, 0xff, 0x00];
    const BLUE: ARGB = [0xff, 0x00, 0x00, 0xff];

    #[test]
    fn test_1r() {
        let answer = QuantizerCelebi.quantize(&[RED], 128);
        assert_eq!(answer.len(), 1);
        assert_eq!(answer.get(&RED), Some(&1));
    }

    #[test]
    fn test_1g() {
        let answer = QuantizerCelebi.quantize(&[GREEN], 128);
        assert_eq!(answer.len(), 1);
        assert_eq!(answer.get(&GREEN), Some(&1));
    }

    #[test]
    fn test_1b() {
        let answer = QuantizerCelebi.quantize(&[BLUE], 128);
        assert_eq!(answer.len(), 1);
        assert_eq!(answer.get(&BLUE), Some(&1));
    }

    #[test]
    fn test_5b() {
        let answer = QuantizerCelebi.quantize(&[BLUE, BLUE, BLUE, BLUE, BLUE], 128);
        assert_eq!(answer.len(), 1);
        assert_eq!(answer.get(&BLUE), Some(&5));
    }

    #[test]
    fn test_2r_3g() {
        let answer = QuantizerCelebi.quantize(&[RED, RED, GREEN, GREEN, GREEN], 128);
        assert_eq!(answer.len(), 2);
        assert_eq!(answer.get(&RED), Some(&2));
        assert_eq!(answer.get(&GREEN), Some(&3));
    }

    #[test]
    fn test_1r_1g_1b() {
        let answer = QuantizerCelebi.quantize(&[RED, GREEN, BLUE], 128);
        assert_eq!(answer.len(), 3);
        assert_eq!(answer.get(&RED), Some(&1));
        assert_eq!(answer.get(&GREEN), Some(&1));
        assert_eq!(answer.get(&BLUE), Some(&1));
    }
}
