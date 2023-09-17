use std::ops::{Add, Sub};

use num::{traits::AsPrimitive, Num};

use crate::util::color::ARGB;

use super::quantizer_map::QuantizerMap;

/// A histogram of all the input colors is constructed. It has the shape of a
/// cube. The cube would be too large if it contained all 16 million colors:
/// historical best practice is to use 5 bits  of the 8 in each channel,
/// reducing the histogram to a volume of ~32,000.
const INDEX_BITS: u8 = 5;
const BITS_TO_REMOVE: u8 = 8 - INDEX_BITS;
const SIDE_LENGTH: usize = 33; // ((1 << INDEX_BITS) + 1)
const TOTAL_SIZE: usize = 35937; // INDEX_COUNT * INDEX_COUNT * INDEX_COUNT

pub enum Direction {
    Red,
    Green,
    Blue,
}

/// Represents requested and result counts of Wu algorithm.
#[derive(Debug, Default, Clone)]
pub struct QuantizerWuCounter {
    /// How many colors the caller asked to be returned from quantization
    pub requested_count: usize,
    /// Actual number of colors achieved from quantization. May be lower than the requested count
    pub result_count: usize,
}

impl QuantizerWuCounter {
    pub fn new(requested_count: usize, result_count: usize) -> Self {
        Self {
            requested_count,
            result_count,
        }
    }
}

/// Represents the result of calculating where to cut an existing box in such
/// a way to maximize variance between the two new boxes created by a cut.
#[derive(Debug, Default, Clone)]
pub struct Maximized {
    pub cut_location: Option<u8>,
    pub maximum: f64,
}

impl Maximized {
    pub fn new(cut_location: Option<u8>, maximum: f64) -> Self {
        Self {
            cut_location,
            maximum,
        }
    }
}

/// An image quantizer that divides the image's pixels into clusters by recursively cutting an RGB
/// cube, based on the weight of pixels in each area of the cube.
///
/// The algorithm was described by Xiaolin Wu in Graphic Gems II, published in 1991.
#[derive(Debug)]
pub struct QuantizerWu {
    weights: Vec<u32>,
    moments_r: Vec<u32>,
    moments_g: Vec<u32>,
    moments_b: Vec<u32>,
    moments: Vec<f64>,
    cubes: Vec<Box>,
}

fn get_index<T>(r: T, g: T, b: T) -> usize
where
    T: AsPrimitive<usize>,
{
    fn inner(r: usize, g: usize, b: usize) -> usize {
        (r << (INDEX_BITS * 2)) + (r << (INDEX_BITS + 1)) + r + (g << INDEX_BITS) + g + b
    }
    inner(r.as_(), g.as_(), b.as_())
}

impl Default for QuantizerWu {
    fn default() -> Self {
        Self {
            weights: vec![0; TOTAL_SIZE],
            moments_r: vec![0; TOTAL_SIZE],
            moments_g: vec![0; TOTAL_SIZE],
            moments_b: vec![0; TOTAL_SIZE],
            moments: vec![0.0; TOTAL_SIZE],
            cubes: Default::default(),
        }
    }
}

impl QuantizerWu {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn quantize(&mut self, pixels: &[ARGB], max_colors: usize) -> Vec<ARGB> {
        self.construct_histogram(pixels);
        self.compute_moments();

        let create_boxes_result = self.create_boxes(max_colors);
        self.create_result(create_boxes_result.result_count)
    }

    fn construct_histogram(&mut self, pixels: &[ARGB]) {
        self.weights = vec![0; TOTAL_SIZE];
        self.moments_r = vec![0; TOTAL_SIZE];
        self.moments_g = vec![0; TOTAL_SIZE];
        self.moments_b = vec![0; TOTAL_SIZE];
        self.moments = vec![0.0; TOTAL_SIZE];

        let count_by_color = QuantizerMap::quantize(pixels);

        for ([_, red, green, blue], count) in count_by_color {
            let index = get_index(
                (red >> BITS_TO_REMOVE) + 1,
                (green >> BITS_TO_REMOVE) + 1,
                (blue >> BITS_TO_REMOVE) + 1,
            );

            let (red, green, blue, count) = (red as u32, green as u32, blue as u32, count);

            self.weights[index] += count;
            self.moments_r[index] += count * red;
            self.moments_g[index] += count * green;
            self.moments_b[index] += count * blue;

            let amount: f64 = (count as f64) * ((red * red + green * green + blue * blue) as f64);
            self.moments[index] += amount;
        }
    }

    fn compute_moments(&mut self) {
        for r in 1..SIDE_LENGTH {
            let mut area = [0; SIDE_LENGTH];
            let mut area_r = [0; SIDE_LENGTH];
            let mut area_g = [0; SIDE_LENGTH];
            let mut area_b = [0; SIDE_LENGTH];
            let mut area2 = [0.0; SIDE_LENGTH];

            for g in 1..SIDE_LENGTH {
                let mut line = 0;
                let mut line_r = 0;
                let mut line_g = 0;
                let mut line_b = 0;
                let mut line2 = 0.0;

                for b in 1..SIDE_LENGTH {
                    let index = get_index(r, g, b);
                    line += self.weights[index];
                    line_r += self.moments_r[index];
                    line_g += self.moments_g[index];
                    line_b += self.moments_b[index];
                    line2 += self.moments[index];

                    area[b] += line;
                    area_r[b] += line_r;
                    area_g[b] += line_g;
                    area_b[b] += line_b;
                    area2[b] += line2;

                    let previous_index = get_index(r - 1, g, b);
                    self.weights[index] = self.weights[previous_index] + area[b];
                    self.moments_r[index] = self.moments_r[previous_index] + area_r[b];
                    self.moments_g[index] = self.moments_g[previous_index] + area_g[b];
                    self.moments_b[index] = self.moments_b[previous_index] + area_b[b];
                    self.moments[index] = self.moments[previous_index] + area2[b];
                }
            }
        }
    }

    fn create_boxes(&mut self, max_colors: usize) -> QuantizerWuCounter {
        self.cubes = vec![Box::default(); max_colors];
        let mut volume_variance = vec![0.0; max_colors];

        let len: u8 = SIDE_LENGTH.try_into().unwrap();
        let max_index = len - 1;
        self.cubes[0].pixels = (
            Pixel::new(0, 0, 0),
            Pixel::new(max_index, max_index, max_index),
        );

        let mut generated_color_count = max_colors;
        let mut next_index = 0usize;
        let mut index = 1usize;
        while index < max_colors {
            if self.cut(next_index, index) {
                let next_cube = &self.cubes[next_index];
                volume_variance[next_index] = if next_cube.vol > 1 {
                    self.variance(next_cube)
                } else {
                    0.0
                };

                let current_cube = &self.cubes[index];
                volume_variance[index] = if current_cube.vol > 1 {
                    self.variance(current_cube)
                } else {
                    0.0
                };
            } else {
                volume_variance[next_index] = 0.0;
                index -= 1;
            };

            next_index = 0;
            let mut temp = volume_variance[0];
            for j in 1..=index {
                if volume_variance[j] > temp {
                    temp = volume_variance[j];
                    next_index = j;
                }
            }
            if temp <= 0.0 {
                generated_color_count = index + 1;
                break;
            }

            index += 1;
        }

        QuantizerWuCounter::new(max_colors, generated_color_count)
    }

    fn create_result(&self, color_count: usize) -> Vec<ARGB> {
        Vec::from_iter((0..color_count).filter_map(|i| {
            let cube = &self.cubes[i];
            let weight = self.volume(cube, &self.weights);

            if weight == 0 {
                return None;
            }

            let r = self.volume(cube, &self.moments_r) / weight;
            let g = self.volume(cube, &self.moments_g) / weight;
            let b = self.volume(cube, &self.moments_b) / weight;

            Some([0xff, r as u8, g as u8, b as u8])
        }))
    }

    fn variance(&self, cube: &Box) -> f64 {
        let dr = self.volume(cube, &self.moments_r);
        let dg = self.volume(cube, &self.moments_g);
        let db = self.volume(cube, &self.moments_b);
        let xx = self.volume(cube, &self.moments);

        let (dr, dg, db) = (dr as f64, dg as f64, db as f64);
        let hypotenuse = dr * dr + dg * dg + db * db;
        let volume: f64 = self.volume(cube, &self.weights) as f64;

        (xx as f64) - (hypotenuse / volume)
    }

    fn cut(&mut self, next_index: usize, current_index: usize) -> bool {
        let (mut one, mut two) = (
            self.cubes[next_index].clone(),
            self.cubes[current_index].clone(),
        );

        let whole_r = self.volume(&one, &self.moments_r);
        let whole_g = self.volume(&one, &self.moments_g);
        let whole_b = self.volume(&one, &self.moments_b);
        let whole_w = self.volume(&one, &self.weights);

        let max_r_result = self.maximize(
            &one,
            Direction::Red,
            one.pixels.0.r + 1,
            one.pixels.1.r,
            whole_r,
            whole_g,
            whole_b,
            whole_w,
        );
        let max_g_result = self.maximize(
            &one,
            Direction::Green,
            one.pixels.0.g + 1,
            one.pixels.1.g,
            whole_r,
            whole_g,
            whole_b,
            whole_w,
        );
        let max_b_result = self.maximize(
            &one,
            Direction::Blue,
            one.pixels.0.b + 1,
            one.pixels.1.b,
            whole_r,
            whole_g,
            whole_b,
            whole_w,
        );

        let max_r = max_r_result.maximum;
        let max_g = max_g_result.maximum;
        let max_b = max_b_result.maximum;

        let direction = {
            if max_r >= max_g && max_r >= max_b {
                if max_r_result.cut_location.is_none() {
                    return false;
                }
                Direction::Red
            } else if max_g >= max_r && max_g >= max_b {
                Direction::Green
            } else {
                Direction::Blue
            }
        };

        two.pixels.1 = one.pixels.1;

        match direction {
            Direction::Red => {
                one.pixels.1.r = max_r_result.cut_location.unwrap_or_default();
                two.pixels.0.r = one.pixels.1.r;
                two.pixels.0.g = one.pixels.0.g;
                two.pixels.0.b = one.pixels.0.b;
            }
            Direction::Green => {
                one.pixels.1.g = max_g_result.cut_location.unwrap_or_default();
                two.pixels.0.r = one.pixels.0.r;
                two.pixels.0.g = one.pixels.1.g;
                two.pixels.0.b = one.pixels.0.b;
            }
            Direction::Blue => {
                one.pixels.1.b = max_b_result.cut_location.unwrap_or_default();
                two.pixels.0.r = one.pixels.0.r;
                two.pixels.0.g = one.pixels.0.g;
                two.pixels.0.b = one.pixels.1.b;
            }
        }

        one.vol = one.calculate_vol();
        two.vol = two.calculate_vol();

        self.cubes[next_index] = one;
        self.cubes[current_index] = two;

        true
    }

    fn maximize(
        &self,
        cube: &Box,
        direction: Direction,
        first: u8,
        last: u8,
        whole_r: i64,
        whole_g: i64,
        whole_b: i64,
        whole_w: i64,
    ) -> Maximized {
        let bottom_r = self.bottom(cube, &direction, &self.moments_r);
        let bottom_g = self.bottom(cube, &direction, &self.moments_g);
        let bottom_b = self.bottom(cube, &direction, &self.moments_b);
        let bottom_w = self.bottom(cube, &direction, &self.weights);

        let mut max = 0.0;
        let mut cut = Option::<u8>::None;

        let mut half_r;
        let mut half_g;
        let mut half_b;
        let mut half_w;

        for i in first..last {
            half_r = bottom_r + self.top(cube, &direction, i, &self.moments_r);
            half_g = bottom_g + self.top(cube, &direction, i, &self.moments_g);
            half_b = bottom_b + self.top(cube, &direction, i, &self.moments_b);
            half_w = bottom_w + self.top(cube, &direction, i, &self.weights);
            if half_w == 0 {
                continue;
            }

            let temp_numerator: f64 = (half_r as f64) * (half_r as f64)
                + (half_g as f64) * (half_g as f64)
                + (half_b as f64) * (half_b as f64);
            let temp_denominator: f64 = half_w as f64;
            let temp = temp_numerator / temp_denominator;

            half_r = whole_r - half_r;
            half_g = whole_g - half_g;
            half_b = whole_b - half_b;
            half_w = whole_w - half_w;
            if half_w == 0 {
                continue;
            }

            let temp_numerator: f64 = (half_r as f64) * (half_r as f64)
                + (half_g as f64) * (half_g as f64)
                + (half_b as f64) * (half_b as f64);
            let temp_denominator: f64 = half_w as f64;
            let temp = temp + (temp_numerator / temp_denominator);

            if temp > max {
                max = temp;
                cut = Some(i)
            }
        }

        Maximized::new(cut, max)
    }

    fn volume<T>(&self, cube: &Box, moment: &Vec<T>) -> i64
    where
        T: Copy + Num + AsPrimitive<i64>,
    {
        let (pixel0, pixel1) = &cube.pixels;
        moment[get_index(pixel1.r, pixel1.g, pixel1.b)].as_()
            - moment[get_index(pixel1.r, pixel1.g, pixel0.b)].as_()
            - moment[get_index(pixel1.r, pixel0.g, pixel1.b)].as_()
            + moment[get_index(pixel1.r, pixel0.g, pixel0.b)].as_()
            - moment[get_index(pixel0.r, pixel1.g, pixel1.b)].as_()
            + moment[get_index(pixel0.r, pixel1.g, pixel0.b)].as_()
            + moment[get_index(pixel0.r, pixel0.g, pixel1.b)].as_()
            - moment[get_index(pixel0.r, pixel0.g, pixel0.b)].as_()
    }

    fn bottom<T>(&self, cube: &Box, direction: &Direction, moment: &Vec<T>) -> i64
    where
        T: Copy + Add<Output = T> + Sub<Output = T> + AsPrimitive<i64>,
    {
        match direction {
            Direction::Red => {
                moment[get_index(cube.pixels.0.r, cube.pixels.1.g, cube.pixels.0.b)].as_()
                    + moment[get_index(cube.pixels.0.r, cube.pixels.0.g, cube.pixels.1.b)].as_()
                    - moment[get_index(cube.pixels.0.r, cube.pixels.0.g, cube.pixels.0.b)].as_()
                    - moment[get_index(cube.pixels.0.r, cube.pixels.1.g, cube.pixels.1.b)].as_()
            }
            Direction::Green => {
                moment[get_index(cube.pixels.1.r, cube.pixels.0.g, cube.pixels.0.b)].as_()
                    + moment[get_index(cube.pixels.0.r, cube.pixels.0.g, cube.pixels.1.b)].as_()
                    - moment[get_index(cube.pixels.0.r, cube.pixels.0.g, cube.pixels.0.b)].as_()
                    - moment[get_index(cube.pixels.1.r, cube.pixels.0.g, cube.pixels.1.b)].as_()
            }
            Direction::Blue => {
                moment[get_index(cube.pixels.1.r, cube.pixels.0.g, cube.pixels.0.b)].as_()
                    + moment[get_index(cube.pixels.0.r, cube.pixels.1.g, cube.pixels.0.b)].as_()
                    - moment[get_index(cube.pixels.0.r, cube.pixels.0.g, cube.pixels.0.b)].as_()
                    - moment[get_index(cube.pixels.1.r, cube.pixels.1.g, cube.pixels.0.b)].as_()
            }
        }
    }

    fn top<T>(&self, cube: &Box, direction: &Direction, position: u8, moment: &Vec<T>) -> i64
    where
        T: Copy + Add<Output = T> + Sub<Output = T> + AsPrimitive<i64>,
    {
        match direction {
            Direction::Red => {
                moment[get_index(position, cube.pixels.1.g, cube.pixels.1.b)].as_()
                    - moment[get_index(position, cube.pixels.1.g, cube.pixels.0.b)].as_()
                    - moment[get_index(position, cube.pixels.0.g, cube.pixels.1.b)].as_()
                    + moment[get_index(position, cube.pixels.0.g, cube.pixels.0.b)].as_()
            }
            Direction::Green => {
                moment[get_index(cube.pixels.1.r, position, cube.pixels.1.b)].as_()
                    - moment[get_index(cube.pixels.1.r, position, cube.pixels.0.b)].as_()
                    - moment[get_index(cube.pixels.0.r, position, cube.pixels.1.b)].as_()
                    + moment[get_index(cube.pixels.0.r, position, cube.pixels.0.b)].as_()
            }
            Direction::Blue => {
                moment[get_index(cube.pixels.1.r, cube.pixels.1.g, position)].as_()
                    - moment[get_index(cube.pixels.1.r, cube.pixels.0.g, position)].as_()
                    - moment[get_index(cube.pixels.0.r, cube.pixels.1.g, position)].as_()
                    + moment[get_index(cube.pixels.0.r, cube.pixels.0.g, position)].as_()
            }
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

/// Keeps track of the state of each box created as the Wu quantization
/// algorithm progresses through, dividing the image's pixels as plotted in RGB.
#[derive(Debug, Default, Clone)]
pub struct Box {
    pub pixels: (Pixel, Pixel),
    pub vol: u16,
}

impl Box {
    pub fn new(pixels: (Pixel, Pixel)) -> Self {
        Self { vol: 0, pixels }
    }

    pub fn calculate_vol(&self) -> u16 {
        (self.pixels.1.r - self.pixels.0.r) as u16
            * (self.pixels.1.g - self.pixels.0.g) as u16
            * (self.pixels.1.b - self.pixels.0.b) as u16
    }
}
