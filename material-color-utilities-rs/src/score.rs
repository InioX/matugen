use crate::htc::cam16::Cam16;
use crate::util::color::lstar_from_argb;
use crate::util::math::{difference_degrees, sanitize_degrees_int};
use ahash::AHashMap;

const CUTOFF_CHROMA: f64 = 15.0;
const CUTOFF_EXCITED_PROPORTION: f64 = 0.01;
const CUTOFF_TONE: f64 = 10.;
const TARGET_CHROMA: f64 = 48.;
const WEIGHT_PROPORTION: f64 = 0.7;
const WEIGHT_CHROMA_ABOVE: f64 = 0.3;
const WEIGHT_CHROMA_BELOW: f64 = 0.1;

pub fn score(colors_to_population: &AHashMap<[u8; 4], u32>) -> Vec<[u8; 4]> {
    // Determine the total count of all colors.
    let mut population_sum = 0.0;
    for population in colors_to_population.values() {
        population_sum += *population as f64;
    }

    // Turn the count of each color into a proportion by dividing by the total
    // count. Also, fill a cache of CAM16 colors representing each color, and
    // record the proportion of colors for each CAM16 hue.
    let mut colors_to_cam = AHashMap::with_capacity(colors_to_population.len());
    let mut hue_proportions: Vec<f64> = vec![0.0; 361];
    for (color, population) in colors_to_population {
        let proportion = (*population as f64) / population_sum;

        let cam = Cam16::from_argb(*color);

        let hue = cam.hue().round() as usize;
        hue_proportions[hue] += proportion;

        colors_to_cam.insert(*color, cam);
    }

    // Determine the proportion of the colors around each color, by summing the
    // proportions around each color's hue.
    let mut colors_to_excited_proportion = AHashMap::with_capacity(colors_to_cam.len());
    for (color, cam) in &colors_to_cam {
        let hue = cam.hue().round() as i32;
        let mut excited_proportion = 0.;
        for j in (hue - 15)..(hue + 15) {
            let neighbor_hue = sanitize_degrees_int(j);
            excited_proportion += hue_proportions[neighbor_hue as usize];
        }
        colors_to_excited_proportion.insert(*color, excited_proportion);
    }

    // Score the colors by their proportion, as well as how chromatic they are.
    let mut colors_to_score = AHashMap::with_capacity(colors_to_cam.len());
    for (color, cam) in &colors_to_cam {
        let proportion = colors_to_excited_proportion.get(color).unwrap();
        let proportion_score = proportion * 100.0 * WEIGHT_PROPORTION;

        let chroma_weight = if cam.chroma() < TARGET_CHROMA {
            WEIGHT_CHROMA_BELOW
        } else {
            WEIGHT_CHROMA_ABOVE
        };
        let chroma_score = (cam.chroma() - TARGET_CHROMA) * chroma_weight;

        let score = proportion_score + chroma_score;
        colors_to_score.insert(color, score);
    }

    // Remove colors that are unsuitable, ex. very dark or unchromatic colors.
    // Also, remove colors that are very similar in hue.
    let filtered_colors_to_score: AHashMap<[u8; 4], f64> =
        filter(&colors_to_excited_proportion, &colors_to_cam)
            .into_iter()
            .map(|v| {
                let score = *colors_to_score.get(&v).unwrap();
                (v, score)
            })
            .collect();

    // Ensure the list of colors returned is sorted such that the first in the
    // list is the most suitable, and the last is the least suitable.
    let mut entry_list: Vec<([u8; 4], f64)> = filtered_colors_to_score.into_iter().collect();
    entry_list.sort_by(|(_, v0), (_, v1)| v0.total_cmp(v1).reverse());

    let mut colors_by_score_descending: Vec<[u8; 4]> = Vec::new();
    for (color, _) in entry_list {
        let cam = colors_to_cam.get(&color).unwrap();
        let mut duplicate_hue = false;

        for already_chosen_color in &colors_by_score_descending {
            let already_chosen_cam = colors_to_cam.get(already_chosen_color).unwrap();
            if difference_degrees(cam.hue(), already_chosen_cam.hue()) < 15.0 {
                duplicate_hue = true;
                break;
            }
        }

        if duplicate_hue {
            continue;
        }

        colors_by_score_descending.push(color);
    }

    // Ensure that at least one color is returned.
    if colors_by_score_descending.is_empty() {
        colors_by_score_descending.push([
            // Google Blue
            0xff, 0x42, 0x85, 0xF4,
        ]);
    }

    colors_by_score_descending
}

fn filter(
    colors_to_excited_proportion: &AHashMap<[u8; 4], f64>,
    colors_to_cam: &AHashMap<[u8; 4], Cam16>,
) -> Vec<[u8; 4]> {
    let mut filtered = Vec::new();
    for (color, cam) in colors_to_cam {
        let proportion = *colors_to_excited_proportion.get(color).unwrap();

        if cam.chroma() >= CUTOFF_CHROMA
            && lstar_from_argb(*color) >= CUTOFF_TONE
            && proportion >= CUTOFF_EXCITED_PROPORTION
        {
            filtered.push(*color);
        }
    }
    filtered
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priority_test() {
        let ranked = score(&AHashMap::from([
            ([0xff, 0xff, 0x00, 0x00], 1),
            ([0xff, 0x00, 0xff, 0x00], 1),
            ([0xff, 0x00, 0x00, 0xff], 1),
        ]));

        assert_eq!(ranked[0], [0xff, 0xff, 0x00, 0x00]);
        assert_eq!(ranked[1], [0xff, 0x00, 0xff, 0x00]);
        assert_eq!(ranked[2], [0xff, 0x00, 0x00, 0xff]);
    }
}
