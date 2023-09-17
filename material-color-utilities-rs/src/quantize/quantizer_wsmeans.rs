use ahash::AHashMap;

use crate::util::color::ARGB;

use super::{
    lab_point_provider::LabPointProvider,
    point_provider::{Point, PointProvider},
};

const MAX_ITERATIONS: u8 = 10;
const MIN_MOVEMENT_DISTANCE: f64 = 3.0;

/// An image quantizer that improves on the speed of a standard K-Means algorithm by implementing
/// several optimizations, including deduping identical pixels and a triangle inequality rule that
/// reduces the number of comparisons needed to identify which cluster a point should be moved to.
///
/// Wsmeans stands for Weighted Square Means.
///
/// This algorithm was designed by M. Emre Celebi, and was found in their 2011 paper, Improving
/// the Performance of K-Means for Color Quantization. https://arxiv.org/abs/1101.0395
pub struct QuantizerWsmeans;

impl QuantizerWsmeans {
    /// # Arguments
    ///
    /// * `input_pixels` - Colors in ARGB format.
    /// * `starting_clusters` - Defines the initial state of the quantizer. Passing
    /// an empty array is fine, the implementation will create its own initial
    /// state that leads to reproducible results for the same inputs.
    /// Passing an array that is the result of Wu quantization leads to higher
    /// quality results.
    /// * `max_colors` The number of colors to divide the image into. A lower
    /// number of colors may be returned.
    ///
    /// # Returns
    ///
    /// Colors in ARGB format.
    pub fn quantize(
        &mut self,
        input_pixels: &[ARGB],
        starting_clusters: &[ARGB],
        max_colors: usize,
    ) -> AHashMap<ARGB, u32> {
        let mut pixel_to_count: AHashMap<ARGB, u32> = AHashMap::new();
        let mut points: Vec<Point> = Vec::with_capacity(input_pixels.len());
        let mut pixels: Vec<ARGB> = Vec::with_capacity(input_pixels.len());
        let point_provider = LabPointProvider::new();
        let mut point_count = 0;
        for input_pixel in input_pixels {
            if let Some(pixel_count) = pixel_to_count.get(input_pixel) {
                pixel_to_count.insert(*input_pixel, pixel_count + 1);
            } else {
                point_count += 1;
                points.push(point_provider.from_int(*input_pixel));
                pixels.push(*input_pixel);
                pixel_to_count.insert(*input_pixel, 1);
            }
        }

        let mut counts = vec![0u32; point_count];
        for i in 0..point_count {
            let pixel = pixels[i];
            if let Some(count) = pixel_to_count.get(&pixel) {
                counts[i] = *count;
            }
        }

        let mut cluster_count = max_colors.min(point_count);
        if !starting_clusters.is_empty() {
            cluster_count = cluster_count.min(starting_clusters.len())
        }

        let mut clusters = Vec::from_iter(
            starting_clusters
                .iter()
                .map(|cluster| point_provider.from_int(*cluster)),
        );

        let additional_clusters_needed = cluster_count - clusters.len();
        if starting_clusters.is_empty() && additional_clusters_needed > 0 {
            for _ in 0..additional_clusters_needed {
                let l = rand::random::<f64>() * 100.0;
                let a = rand::random::<f64>() * (100.0 - (-100.0) + 1.0) + -100.0;
                let b = rand::random::<f64>() * (100.0 - (-100.0) + 1.0) + -100.0;

                clusters.push([l, a, b])
            }
        }

        let mut cluster_indices = Vec::from_iter((0..point_count).map(|_| {
            let index = rand::random::<f32>() * cluster_count as f32;
            index.floor() as usize
        }));

        let mut index_matrix = Vec::from_iter((0..cluster_count).map(|_| vec![0; cluster_count]));

        let mut distance_to_index_matrix = Vec::from_iter(
            (0..cluster_count)
                .map(|_| Vec::from_iter((0..cluster_count).map(|_| Distance::default()))),
        );

        let mut pixel_count_sums = vec![0; cluster_count];
        for iteration in 0..MAX_ITERATIONS {
            for i in 0..cluster_count {
                for j in 0..cluster_count {
                    let distance = point_provider.distance(clusters[i], clusters[j]);
                    distance_to_index_matrix[j][i].distance = distance;
                    distance_to_index_matrix[j][i].index = i;
                    distance_to_index_matrix[i][j].distance = distance;
                    distance_to_index_matrix[i][j].index = j;
                }
                // requires alternative sorting since f64 doesn't implement Ord
                // https://doc.rust-lang.org/std/primitive.slice.html#method.sort_by
                distance_to_index_matrix[i].sort_by(|a, b| a.partial_cmp(b).unwrap());
                for j in 0..cluster_count {
                    index_matrix[i][j] = distance_to_index_matrix[i][j].index;
                }
            }

            let mut points_moved = 0usize;
            for i in 0..point_count {
                let point = points[i];
                let previous_cluster_index = cluster_indices[i];
                let previous_cluster = clusters[previous_cluster_index];
                let previous_distance = point_provider.distance(point, previous_cluster);
                let mut minimum_distance = previous_distance;
                let mut new_cluster_index_option: Option<usize> = None;
                for j in 0..cluster_count {
                    let distance_index = distance_to_index_matrix[previous_cluster_index][j];
                    if distance_index.distance >= 4.0 * previous_distance {
                        continue;
                    }
                    let distance = point_provider.distance(point, clusters[j]);
                    if distance < minimum_distance {
                        minimum_distance = distance;
                        new_cluster_index_option = Some(j);
                    }
                }
                if let Some(new_cluster_index) = new_cluster_index_option {
                    let distance_change =
                        (minimum_distance.sqrt() - previous_distance.sqrt()).abs();
                    if distance_change > MIN_MOVEMENT_DISTANCE {
                        points_moved += 1;
                        cluster_indices[i] = new_cluster_index;
                    }
                }
            }

            if points_moved == 0 && iteration != 0 {
                break;
            }

            let component_sums = {
                let mut component_sums: [Vec<f64>; 3] = [
                    vec![0.0; cluster_count],
                    vec![0.0; cluster_count],
                    vec![0.0; cluster_count],
                ];
                for i in 0..point_count {
                    let cluster_index = cluster_indices[i];
                    let point = points[i];
                    let count = counts[i];
                    pixel_count_sums[cluster_index] += count;
                    component_sums[0][cluster_index] += point[0] * count as f64;
                    component_sums[1][cluster_index] += point[1] * count as f64;
                    component_sums[2][cluster_index] += point[2] * count as f64;
                }
                component_sums
            };

            for i in 0..cluster_count {
                let count = pixel_count_sums[i];
                clusters[i] = if count == 0 {
                    Default::default()
                } else {
                    [
                        component_sums[0][i] / count as f64,
                        component_sums[1][i] / count as f64,
                        component_sums[2][i] / count as f64,
                    ]
                };
            }
        }

        let mut argb_to_population = AHashMap::new();

        for i in 0..cluster_count {
            let count = pixel_count_sums[i];
            if count == 0 {
                continue;
            }
            let possible_new_cluster = point_provider.to_int(clusters[i]);
            if argb_to_population.contains_key(&possible_new_cluster) {
                continue;
            }

            argb_to_population.insert(possible_new_cluster, count);
        }

        argb_to_population
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
struct Distance {
    index: usize,
    distance: f64,
}
