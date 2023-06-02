// Fully ported

use crate::htc::cam16;
use crate::util::color::{y_from_lstar, WHITE_POINT_D65};
use crate::util::math::lerp;
use lazy_static::lazy_static;
use std::f64::consts::PI;

/// In traditional color spaces, a color can be identified solely by the observer's measurement of
/// the color. Color appearance models such as CAM16 also use information about the environment where
/// the color was observed, known as the viewing conditions.
/// <p>For example, white under the traditional assumption of a midday sun white point is accurately
/// measured as a slightly chromatic blue by CAM16. (roughly, hue 203, chroma 3, lightness 100)
/// <p>This class caches intermediate values of the CAM16 conversion process that depend only on
/// viewing conditions, enabling speed ups.
#[derive(Clone, Debug)]
pub struct ViewingConditions {
    aw: f64,
    nbb: f64,
    ncb: f64,
    c: f64,
    nc: f64,
    n: f64,
    rgb_d: [f64; 3],
    fl: f64,
    fl_root: f64,
    z: f64,
}

impl ViewingConditions {
    pub fn aw(&self) -> f64 {
        self.aw
    }
    pub fn nbb(&self) -> f64 {
        self.nbb
    }
    pub fn ncb(&self) -> f64 {
        self.ncb
    }
    pub fn c(&self) -> f64 {
        self.c
    }
    pub fn nc(&self) -> f64 {
        self.nc
    }
    pub fn n(&self) -> f64 {
        self.n
    }
    pub fn rgb_d(&self) -> [f64; 3] {
        self.rgb_d
    }
    pub fn fl(&self) -> f64 {
        self.fl
    }
    pub fn fl_root(&self) -> f64 {
        self.fl_root
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    /// Create ViewingConditions from a simple, physically relevant, set of parameters.
    ///
    /// # Arguments
    ///
    /// * `white_point`: White point, measured in the XYZ color space. default = D65, or sunny day afternoon
    /// * `adapting_luminance`: The luminance of the adapting field. Informally, how bright it is in
    /// the room where the color is viewed. Can be calculated from lux by multiplying lux by
    /// 0.0586. default = 11.72, or 200 lux.
    /// * `background_lstar`: The lightness of the area surrounding the color. measured by L* in
    /// L*a*b*. default = 50.0
    /// * `surround`: A general description of the lighting surrounding the color. 0 is pitch dark,
    /// like watching a movie in a theater. 1.0 is a dimly light room, like watching TV at home at
    /// night. 2.0 means there is no difference between the lighting on the color and around it.
    /// default = 2.0
    /// * `discounting_illuminant`: Whether the eye accounts for the tint of the ambient lighting,
    /// such as knowing an apple is still red in green light. default = false, the eye does not
    /// perform this process on self-luminous objects like displays.
    ///
    /// returns: ViewingConditions
    pub fn new(
        white_point: [f64; 3],
        adapting_luminance: f64,
        background_lstar: f64,
        surround: f64,
        discounting_illuminant: bool,
    ) -> ViewingConditions {
        let matrix = cam16::XYZ_TO_CAM16RGB;
        let xyz = white_point;
        let r_w = (xyz[0] * matrix[0][0]) + (xyz[1] * matrix[0][1]) + (xyz[2] * matrix[0][2]);
        let g_w = (xyz[0] * matrix[1][0]) + (xyz[1] * matrix[1][1]) + (xyz[2] * matrix[1][2]);
        let b_w = (xyz[0] * matrix[2][0]) + (xyz[1] * matrix[2][1]) + (xyz[2] * matrix[2][2]);
        let f = 0.8 + (surround / 10.0);

        let c = if f >= 0.9 {
            lerp(0.59, 0.69, (f - 0.9) * 10.0)
        } else {
            lerp(0.525, 0.59, (f - 0.8) * 10.0)
        };

        let d = if discounting_illuminant {
            1.0
        } else {
            f * (1.0 - ((1.0 / 3.6) * ((-adapting_luminance - 42.0) / 92.0).exp()))
        };

        let nc = f;
        let rgb_d = [
            d * (100.0 / r_w) + 1.0 - d,
            d * (100.0 / g_w) + 1.0 - d,
            d * (100.0 / b_w) + 1.0 - d,
        ];
        let k = 1.0 / (5.0 * adapting_luminance + 1.0);
        let k4 = k * k * k * k;
        let k4f = 1.0 - k4;
        let fl = (k4 * adapting_luminance) + (0.1 * k4f * k4f * (5.0 * adapting_luminance).cbrt());

        let n = y_from_lstar(background_lstar) / white_point[1];
        let z = 1.48 + n.sqrt();
        let nbb = 0.725 / n.powf(0.2);
        let ncb = nbb;

        let rgb_afactors = [
            (fl * rgb_d[0] * r_w / 100.0).powf(0.42),
            (fl * rgb_d[1] * g_w / 100.0).powf(0.42),
            (fl * rgb_d[2] * b_w / 100.0).powf(0.42),
        ];
        let rgb_a = [
            (400.0 * rgb_afactors[0]) / (rgb_afactors[0] + 27.13),
            (400.0 * rgb_afactors[1]) / (rgb_afactors[1] + 27.13),
            (400.0 * rgb_afactors[2]) / (rgb_afactors[2] + 27.13),
        ];
        let aw = ((2.0 * rgb_a[0]) + rgb_a[1] + (0.05 * rgb_a[2])) * nbb;

        ViewingConditions {
            aw,
            nbb,
            ncb,
            c,
            nc,
            n,
            rgb_d,
            fl,
            fl_root: fl.powf(0.25),
            z,
        }
    }
}

lazy_static! {
    static ref DEFAULT: ViewingConditions = ViewingConditions::new(
        WHITE_POINT_D65,
        200.0 / PI * y_from_lstar(50.0) / 100.0,
        50.0,
        2.0,
        false,
    );
}

impl Default for ViewingConditions {
    fn default() -> Self {
        DEFAULT.clone()
    }
}
