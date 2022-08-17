use crate::util::math::matrix_multiply;

pub const SRGB_TO_XYZ: [[f64; 3]; 3] = [
    [0.41233895, 0.35762064, 0.18051042],
    [0.2126, 0.7152, 0.0722],
    [0.01932141, 0.11916382, 0.95034478],
];

pub const XYZ_TO_SRGB: [[f64; 3]; 3] = [
    [
        3.2413774792388685,
        -1.5376652402851851,
        -0.49885366846268053,
    ],
    [-0.9691452513005321, 1.8758853451067872, 0.04156585616912061],
    [
        0.05562093689691305,
        -0.20395524564742123,
        1.0571799111220335,
    ],
];

pub const WHITE_POINT_D65: [f64; 3] = [95.047, 100.0, 108.883];

/** Converts a color from RGB components to ARGB format. */
pub fn argb_from_rgb(rgb: [u8; 3]) -> [u8; 4] {
    [255, rgb[0], rgb[1], rgb[2]]
}

/** Converts a color from linear RGB components to ARGB format. */
pub fn argb_from_linrgb(linrgb: [f64; 3]) -> [u8; 4] {
    let r = delinearized(linrgb[0]);
    let g = delinearized(linrgb[1]);
    let b = delinearized(linrgb[2]);
    argb_from_rgb([r, g, b])
}

/** Converts a color from ARGB to XYZ. */
pub fn argb_from_xyz(xyz: [f64; 3]) -> [u8; 4] {
    let rgb = matrix_multiply(xyz, XYZ_TO_SRGB);
    let r = delinearized(rgb[0]);
    let g = delinearized(rgb[1]);
    let b = delinearized(rgb[2]);
    return argb_from_rgb([r, g, b]);
}

/** Converts a color from XYZ to ARGB. */
pub fn xyz_from_argb(argb: [u8; 4]) -> [f64; 3] {
    let r = linearized(argb[1]);
    let g = linearized(argb[2]);
    let b = linearized(argb[3]);
    matrix_multiply([r, g, b], SRGB_TO_XYZ)
}

/** Converts a color represented in Lab color space into an ARGB integer. */
pub fn argb_from_lab(l: f64, a: f64, b: f64) -> [u8; 4] {
    let fy = (l + 16.0) / 116.0;
    let fx = a / 500.0 + fy;
    let fz = fy - b / 200.0;
    let x = lab_invf(fx) * WHITE_POINT_D65[0];
    let y = lab_invf(fy) * WHITE_POINT_D65[1];
    let z = lab_invf(fz) * WHITE_POINT_D65[2];
    argb_from_xyz([x, y, z])
}

/// Converts a color from ARGB representation to L*a*b* representation.
///
/// # Arguments
///
/// * `argb`: the ARGB representation of a color
///
/// returns: a Lab object representing the color
pub fn lab_from_argb(argb: [u8; 4]) -> [f64; 3] {
    let [x, y, z] = xyz_from_argb(argb);
    let fx = lab_f(x / WHITE_POINT_D65[0]);
    let fy = lab_f(y / WHITE_POINT_D65[1]);
    let fz = lab_f(z / WHITE_POINT_D65[2]);
    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let b = 200.0 * (fy - fz);
    [l, a, b]
}

/// Converts an L* value to an ARGB representation.
///
/// # Arguments
///
/// * `lstar`: L* in L*a*b*
///
/// returns: ARGB representation of grayscale color with lightness matching L*
pub fn argb_from_lstar(lstar: f64) -> [u8; 4] {
    let y = y_from_lstar(lstar);
    let w = delinearized(y);
    argb_from_rgb([w, w, w])
}

/// Computes the L* value of a color in ARGB representation.
///
/// # Arguments
///
/// * `argb`: ARGB representation of a color
///
/// returns: L*, from L*a*b*, coordinate of the color
pub fn lstar_from_argb(argb: [u8; 4]) -> f64 {
    let y = xyz_from_argb(argb)[1];
    116.0 * lab_f(y / 100.0) - 16.0
}

/// Converts an L* value to a Y value.
/// <p>L* in L*a*b* and Y in XYZ measure the same quantity, luminance.
/// <p>L* measures perceptual luminance, a linear scale. Y in XYZ measures relative luminance, a
/// logarithmic scale.
///
/// # Arguments
///
/// * `lstar`: L* in L*a*b*
///
/// returns: Y in XYZ
pub fn y_from_lstar(lstar: f64) -> f64 {
    100.0 * lab_invf((lstar + 16.0) / 116.0)
}

/// Linearizes an RGB component.
///
/// # Arguments
///
/// * `rgb_comp`: 0 <= rgb_component <= 255, represents R/G/B channel
///
/// returns: 0.0 <= output <= 100.0, color channel converted to linear RGB space
pub fn linearized(rgb_comp: u8) -> f64 {
    let normalized = rgb_comp as f64 / 255.0;
    if normalized <= 0.040449936 {
        normalized / 12.92 * 100.0
    } else {
        ((normalized + 0.055) / 1.055).powf(2.4) * 100.0
    }
}

/// Delinearizes an RGB component.
///
/// # Arguments
///
/// * `rgb_comp`: 0.0 <= rgb_component <= 100.0, represents linear R/G/B channel
///
/// returns: 0 <= output <= 255, color channel converted to regular RGB space
pub fn delinearized(rgb_comp: f64) -> u8 {
    let normalized = rgb_comp / 100.0;
    let delinearized = if normalized <= 0.0031308 {
        normalized * 12.92
    } else {
        1.055 * normalized.powf(1.0 / 2.4) - 0.055
    };
    (delinearized * 255.0).round().clamp(0.0, 255.0) as u8
}

fn lab_f(t: f64) -> f64 {
    let e = 216.0 / 24389.0;
    let kappa = 24389.0 / 27.0;
    if t > e {
        t.powf(1.0 / 3.0)
    } else {
        (kappa * t + 16.0) / 116.0
    }
}

fn lab_invf(ft: f64) -> f64 {
    let e = 216.0 / 24389.0;
    let kappa = 24389.0 / 27.0;
    let ft3 = ft * ft * ft;
    if ft3 > e {
        ft3
    } else {
        (116.0 * ft - 16.0) / kappa
    }
}
