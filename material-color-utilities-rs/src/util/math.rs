/**
 * The linear interpolation function.
 *
 * returns: start if amount = 0 and stop if amount = 1
 */
pub fn lerp(start: f64, stop: f64, amount: f64) -> f64 {
    (1.0 - amount) * start + amount * stop
}

/// Sign of direction change needed to travel from one angle to another.
/// <p>For angles that are 180 degrees apart from each other, both directions have the same travel
/// distance, so either direction is shortest. The value 1.0 is returned in this case.
///
/// # Arguments
///
/// * `from`: The angle travel starts from, in degrees.
/// * `to`: The angle travel ends at, in degrees.
///
/// returns: -1 if decreasing from leads to the shortest travel distance, 1 if increasing from leads
/// to the shortest travel distance.
pub fn rotation_direction(from: f64, to: f64) -> f64 {
    let increasing_difference = sanitize_degrees_double(to - from);
    if increasing_difference <= 180.0 {
        1.0
    } else {
        -1.0
    }
}

/** Distance of two points on a circle, represented using degrees. */
pub fn difference_degrees(a: f64, b: f64) -> f64 {
    180.0 - ((a - b).abs() - 180.0).abs()
}

/**
 * Sanitizes a degree measure as an integer.
 *
 * returns: a degree measure between 0 (inclusive) and 360 (exclusive).
 */
pub fn sanitize_degrees_int(mut degrees: i32) -> u32 {
    degrees %= 360;
    if degrees < 0 {
        degrees += 360;
    }

    degrees as u32
}

/**
 * Sanitizes a degree measure as a floating-point number.
 *
 * returns: a degree measure between 0.0 (inclusive) and 360.0 (exclusive).
 */
pub fn sanitize_degrees_double(mut degrees: f64) -> f64 {
    degrees %= 360.0;
    if degrees < 0.0 {
        degrees += 360.0;
    }
    degrees
}

pub fn matrix_multiply(row: [f64; 3], matrix: [[f64; 3]; 3]) -> [f64; 3] {
    let a = row[0] * matrix[0][0] + row[1] * matrix[0][1] + row[2] * matrix[0][2];
    let b = row[0] * matrix[1][0] + row[1] * matrix[1][1] + row[2] * matrix[1][2];
    let c = row[0] * matrix[2][0] + row[1] * matrix[2][1] + row[2] * matrix[2][2];
    [a, b, c]
}
