// Fully ported

use crate::htc::viewing_conditions::ViewingConditions;
use crate::util::color::{argb_from_xyz, xyz_from_argb};
use crate::util::math::matrix_multiply;
use std::f64::consts::PI;

pub const XYZ_TO_CAM16RGB: [[f64; 3]; 3] = [
    [0.401288, 0.650173, -0.051461],
    [-0.250268, 1.204414, 0.045854],
    [-0.002079, 0.048952, 0.953127],
];

pub const CAM16RGB_TO_XYZ: [[f64; 3]; 3] = [
    [1.8620678, -1.0112547, 0.14918678],
    [0.38752654, 0.62144744, -0.00897398],
    [-0.01584150, -0.03412294, 1.0499644],
];

pub struct Cam16 {
    // CAM16 color dimensions, see getters for documentation.
    hue: f64,
    chroma: f64,
    j: f64,
    q: f64,
    m: f64,
    s: f64,
    // Coordinates in UCS space. Used to determine color distance, like delta E equations in L*a*b*.
    jstar: f64,
    astar: f64,
    bstar: f64,
}

impl Cam16 {
    pub fn distance(&self, other: Cam16) -> f64 {
        let d_j = self.jstar() - other.jstar();
        let d_a = self.astar() - other.astar();
        let d_b = self.bstar() - other.bstar();
        let d_eprime = (d_j * d_j + d_a * d_a + d_b * d_b).sqrt();

        1.41 * d_eprime.powf(0.63)
    }

    /// Hue in CAM16
    pub fn hue(&self) -> f64 {
        self.hue
    }

    /// Chroma in CAM16
    pub fn chroma(&self) -> f64 {
        self.chroma
    }

    /// Lightness in CAM16
    pub fn j(&self) -> f64 {
        self.j
    }

    /// Brightness in CAM16.
    ///
    /// <p>Prefer lightness, brightness is an absolute quantity. For example, a sheet of white paper is
    /// much brighter viewed in sunlight than in indoor light, but it is the lightest object under any
    /// lighting.
    ///
    pub fn q(&self) -> f64 {
        self.q
    }

    /// Colorfulness in CAM16.
    ///
    /// <p>Prefer chroma, colorfulness is an absolute quantity. For example, a yellow toy car is much
    /// more colorful outside than inside, but it has the same chroma in both environments.
    ///
    pub fn m(&self) -> f64 {
        self.m
    }

    ///
    /// Saturation in CAM16.
    ///
    /// <p>Colorfulness in proportion to brightness. Prefer chroma, saturation measures colorfulness
    /// relative to the color's own brightness, where chroma is colorfulness relative to white.
    ///
    pub fn s(&self) -> f64 {
        self.s
    }

    /// Lightness coordinate in CAM16-UCS
    pub fn jstar(&self) -> f64 {
        self.jstar
    }

    /// a* coordinate in CAM16-UCS
    pub fn astar(&self) -> f64 {
        self.astar
    }

    /// b* coordinate in CAM16-UCS
    pub fn bstar(&self) -> f64 {
        self.bstar
    }

    /// All of the CAM16 dimensions can be calculated from 3 of the dimensions, in the following
    /// combinations: - {j or q} and {c, m, or s} and hue - jstar, astar, bstar Prefer using a static
    /// method that constructs from 3 of those dimensions. This constructor is intended for those
    /// methods to use to return all possible dimensions.
    ///
    /// # Arguments
    ///
    /// * `hue`: for example, red, orange, yellow, green, etc.
    /// * `chroma`: informally, colorfulness / color intensity. like saturation in HSL, except perceptually accurate.
    /// * `j`: lightness
    /// * `q`: brightness; ratio of lightness to white point's lightness
    /// * `m`: colorfulness
    /// * `s`: saturation; ratio of chroma to white point's chroma
    /// * `jstar`: CAM16-UCS J coordinate
    /// * `astar`: CAM16-UCS a coordinate
    /// * `bstar`: CAM16-UCS b coordinate
    #[allow(dead_code)]
    fn new(
        hue: f64,
        chroma: f64,
        j: f64,
        q: f64,
        m: f64,
        s: f64,
        jstar: f64,
        astar: f64,
        bstar: f64,
    ) -> Self {
        Self {
            hue,
            chroma,
            j,
            q,
            m,
            s,
            jstar,
            astar,
            bstar,
        }
    }

    /// Create a CAM16 color from a color, assuming the color was viewed in default viewing conditions.
    /// # Arguments
    /// * `argb`: ARGB representation of a color.
    pub fn from_argb(argb: [u8; 4]) -> Cam16 {
        Self::from_int_in_viewing_condition(argb, ViewingConditions::default())
    }

    /// Create a CAM16 color from a color in defined viewing conditions.
    ///
    /// # Arguments
    ///
    /// * `argb`: ARGB representation of a color.
    /// * `viewing_conditions`: Information about the environment where the color was observed.
    ///
    /// returns: Cam16
    // The RGB => XYZ conversion matrix elements are derived scientific constants. While the values
    // may differ at runtime due to floating point imprecision, keeping the values the same, and
    // accurate, across implementations takes precedence.
    pub fn from_int_in_viewing_condition(
        argb: [u8; 4],
        viewing_conditions: ViewingConditions,
    ) -> Cam16 {
        // Transform ARGB int to XYZ
        let xyz = xyz_from_argb(argb);

        // Transform XYZ to 'cone'/'rgb' responses
        let t = matrix_multiply(xyz, XYZ_TO_CAM16RGB);

        // Discount illuminant
        let d = [
            viewing_conditions.rgb_d()[0] * t[0],
            viewing_conditions.rgb_d()[1] * t[1],
            viewing_conditions.rgb_d()[2] * t[2],
        ];

        // Chromatic adaptation
        let af = [
            (viewing_conditions.fl() * d[0].abs() / 100.0).powf(0.42),
            (viewing_conditions.fl() * d[1].abs() / 100.0).powf(0.42),
            (viewing_conditions.fl() * d[2].abs() / 100.0).powf(0.42),
        ];

        let a = [
            d[0].signum() * 400.0 * af[0] / (af[0] + 27.13),
            d[1].signum() * 400.0 * af[1] / (af[1] + 27.13),
            d[2].signum() * 400.0 * af[2] / (af[2] + 27.13),
        ];

        // redness-greenness
        let red_greenness = (11.0 * a[0] + -12.0 * a[1] + a[2]) / 11.0;
        // yellowness-blueness
        let yellowness_blueness = (a[0] + a[1] - 2.0 * a[2]) / 9.0;

        // auxiliary components
        let u = (20.0 * a[0] + 20.0 * a[1] + 21.0 * a[2]) / 20.0;
        let p2 = (40.0 * a[0] + 20.0 * a[1] + a[2]) / 20.0;

        // hue
        let atan2 = yellowness_blueness.atan2(red_greenness);
        let atan_degrees = atan2.to_degrees();
        let hue = if atan_degrees < 0.0 {
            atan_degrees + 360.0
        } else if atan_degrees >= 360.0 {
            atan_degrees - 360.0
        } else {
            atan_degrees
        };
        let hue_radians = hue.to_radians();

        // achromatic response to color
        let ac = p2 * viewing_conditions.nbb();

        // CAM16 lightness and brightness
        let lightness = 100.0
            * (ac / viewing_conditions.aw()).powf(viewing_conditions.c() * viewing_conditions.z());
        let brightness = 4.0 / viewing_conditions.c()
            * (lightness / 100.0).sqrt()
            * (viewing_conditions.aw() + 4.0)
            * viewing_conditions.fl_root();

        // CAM16 chroma, colorfulness, and saturation.
        let hue_prime = if hue < 20.14 { hue + 360.0 } else { hue };
        let e_hue = 0.25 * ((hue_prime.to_radians() + 2.0).cos() + 3.8);
        let p1 = 50000.0 / 13.0 * e_hue * viewing_conditions.nc() * viewing_conditions.ncb();
        let t = p1 * red_greenness.hypot(yellowness_blueness) / (u + 0.305);
        let alpha = (1.64 - 0.29f64.powf(viewing_conditions.n())).powf(0.73) * t.powf(0.9);
        // CAM16 chroma, colorfulness, saturation
        let chroma = alpha * (lightness / 100.0).sqrt();
        let colorfulness = chroma * viewing_conditions.fl_root();
        let saturation =
            50.0 * ((alpha * viewing_conditions.c()) / (viewing_conditions.aw() + 4.0)).sqrt();

        // CAM16-UCS components
        let jstar = (1.0 + 100.0 * 0.007) * lightness / (1.0 + 0.007 * lightness);
        // TODO possible wrong math
        let mstar = 1.0 / 0.0228 * (0.0228 * colorfulness).ln_1p();
        let astar = mstar * hue_radians.cos();
        let bstar = mstar * hue_radians.sin();

        Cam16 {
            hue,
            chroma,
            j: lightness,
            q: brightness,
            m: colorfulness,
            s: saturation,
            jstar,
            astar,
            bstar,
        }
    }

    /// # Arguments
    ///
    /// * `j`: CAM16 lightness
    /// * `c`: CAM16 chroma
    /// * `h`: CAM16 hue
    pub(crate) fn from_jch(j: f64, c: f64, h: f64) -> Cam16 {
        Self::from_jch_in_viewing_conditions(j, c, h, ViewingConditions::default())
    }

    /// # Arguments
    ///
    /// * `j`: CAM16 lightness
    /// * `c`: CAM16 chroma
    /// * `h`: CAM16 hue
    /// * `viewing_conditions`: Information about the environment where the color was observed.
    fn from_jch_in_viewing_conditions(
        j: f64,
        c: f64,
        h: f64,
        viewing_conditions: ViewingConditions,
    ) -> Cam16 {
        let q = 4.0 / viewing_conditions.c()
            * (j / 100.0).sqrt()
            * (viewing_conditions.aw() + 4.0)
            * viewing_conditions.fl_root();
        let m = c * viewing_conditions.fl_root();
        let alpha = c / (j / 100.0).sqrt();
        let s = 50.0 * ((alpha * viewing_conditions.c()) / (viewing_conditions.aw() + 4.0)).sqrt();

        let hue_radians = h.to_radians();
        let jstar = (1.0 + 100.0 * 0.007) * j / (1.0 + 0.007 * j);
        // TODO log1p maybe wrong
        let mstar = 1.0 / 0.0228 * (0.0228 * m).ln_1p();
        let astar = mstar * hue_radians.cos();
        let bstar = mstar * hue_radians.sin();
        Cam16 {
            hue: h,
            chroma: c,
            j,
            q,
            m,
            s,
            jstar,
            astar,
            bstar,
        }
    }

    /// Create a CAM16 color from CAM16-UCS coordinates.
    ///
    /// # Arguments
    ///
    /// * `jstar`: CAM16-UCS lightness.
    /// * `astar`: CAM16-UCS a dimension. Like a* in L*a*b*, it is a Cartesian coordinate on the Y axis.
    /// * `bstar`: CAM16-UCS b dimension. Like a* in L*a*b*, it is a Cartesian coordinate on the X axis.
    pub fn from_ucs(jstar: f64, astar: f64, bstar: f64) -> Cam16 {
        Self::from_ucs_in_viewing_conditions(jstar, astar, bstar, ViewingConditions::default())
    }

    /// Create a CAM16 color from CAM16-UCS coordinates in defined viewing conditions.
    ///
    /// # Arguments
    ///
    /// * `jstar`: CAM16-UCS lightness.
    /// * `astar`: CAM16-UCS a dimension. Like a* in L*a*b*, it is a Cartesian coordinate on the Y axis.
    /// * `bstar`: CAM16-UCS b dimension. Like a* in L*a*b*, it is a Cartesian coordinate on the X axis.
    /// * `viewing_conditions`: Information about the environment where the color was observed.
    pub fn from_ucs_in_viewing_conditions(
        jstar: f64,
        astar: f64,
        bstar: f64,
        viewing_conditions: ViewingConditions,
    ) -> Cam16 {
        let m = astar.hypot(bstar);
        let m2 = (m * 0.0228).exp_m1() / 0.0228;
        let c = m2 / viewing_conditions.fl_root();
        let mut h = bstar.atan2(astar) * (180.0 / PI);
        if h < 0.0 {
            h += 360.0;
        }
        let j = jstar / (1. - (jstar - 100.) * 0.007);
        Self::from_jch_in_viewing_conditions(j, c, h, viewing_conditions)
    }

    pub fn to_int(&self) -> [u8; 4] {
        self.viewed(ViewingConditions::default())
    }

    pub fn viewed(&self, viewing_conditions: ViewingConditions) -> [u8; 4] {
        let alpha = if self.chroma() == 0.0 || self.j() == 0.0 {
            0.0
        } else {
            self.chroma() / (self.j() / 100.0).sqrt()
        };

        let t = (alpha / (1.64 - 0.29f64.powf(viewing_conditions.n())).powf(0.73)).powf(1.0 / 0.9);
        let h_rad = self.hue().to_radians();

        let e_hue = 0.25 * ((h_rad + 2.0).cos() + 3.8);
        let ac = viewing_conditions.aw()
            * (self.j() / 100.0).powf(1.0 / viewing_conditions.c() / viewing_conditions.z());
        let p1 = e_hue * (50000.0 / 13.0) * viewing_conditions.nc() * viewing_conditions.ncb();
        let p2 = ac / viewing_conditions.nbb();

        let h_sin = h_rad.sin();
        let h_cos = h_rad.cos();

        let gamma = 23.0 * (p2 + 0.305) * t / (23.0 * p1 + 11.0 * t * h_cos + 108.0 * t * h_sin);
        let a = gamma * h_cos;
        let b = gamma * h_sin;
        let r_a = (460.0 * p2 + 451.0 * a + 288.0 * b) / 1403.0;
        let g_a = (460.0 * p2 - 891.0 * a - 261.0 * b) / 1403.0;
        let b_a = (460.0 * p2 - 220.0 * a - 6300.0 * b) / 1403.0;

        let r_cbase = ((27.13 * r_a.abs()) / (400.0 - r_a.abs())).max(0.0);
        let r_c = r_a.signum() * (100.0 / viewing_conditions.fl()) * r_cbase.powf(1.0 / 0.42);
        let g_cbase = ((27.13 * g_a.abs()) / (400.0 - g_a.abs())).max(0.0);
        let g_c = g_a.signum() * (100.0 / viewing_conditions.fl()) * g_cbase.powf(1.0 / 0.42);
        let b_cbase = ((27.13 * b_a.abs()) / (400.0 - b_a.abs())).max(0.0);
        let b_c = b_a.signum() * (100.0 / viewing_conditions.fl()) * b_cbase.powf(1.0 / 0.42);
        let r_f = r_c / viewing_conditions.rgb_d()[0];
        let g_f = g_c / viewing_conditions.rgb_d()[1];
        let b_f = b_c / viewing_conditions.rgb_d()[2];

        let xyz = matrix_multiply([r_f, g_f, b_f], CAM16RGB_TO_XYZ);
        argb_from_xyz(xyz)
    }
}
