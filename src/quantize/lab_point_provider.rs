use crate::util::color::{self, ARGB};

use super::point_provider::{Point, PointProvider};

pub struct LabPointProvider;

impl LabPointProvider {
    pub fn new() -> Self {
        Self
    }
}

/// Provides conversions needed for K-Means quantization. Converting input to
/// points, and converting the final state of the K-Means algorithm to colors.
impl PointProvider for LabPointProvider {
    /// Convert a 3-element array to a color represented in ARGB.
    fn to_int(&self, [l, a, b]: Point) -> ARGB {
        color::argb_from_lab(l, a, b)
    }

    /// Convert a color represented in ARGB to a 3-element array of L*a*b*
    /// coordinates of the color.
    fn from_int(&self, argb: ARGB) -> Point {
        color::lab_from_argb(argb)
    }

    /// Standard CIE 1976 delta E formula also takes the square root, unneeded
    /// here. This method is used by quantization algorithms to compare distance,
    /// and the relative ordering is the same, with or without a square root.
    ///
    /// This relatively minor optimization is helpful because this method is
    /// called at least once for each pixel in an image.
    fn distance(&self, from: Point, to: Point) -> f64 {
        let l_diff = from[0] - to[0];
        let a_diff = from[1] - to[1];
        let b_diff = from[2] - to[2];
        l_diff * l_diff + a_diff * a_diff + b_diff * b_diff
    }
}
