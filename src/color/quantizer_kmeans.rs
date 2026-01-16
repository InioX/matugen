pub fn kmeans(samples: &[[f32; 3]], k: usize, iterations: usize) -> Vec<[f32; 3]> {
    let mut centroids = samples.iter().take(k).cloned().collect::<Vec<_>>();

    for _ in 0..iterations {
        let mut buckets = vec![Vec::new(); k];

        for s in samples {
            let (mut best, mut best_d) = (0, f32::MAX);

            for (i, c) in centroids.iter().enumerate() {
                let d = (s[0] - c[0]).powi(2) + (s[1] - c[1]).powi(2) + (s[2] - c[2]).powi(2);

                if d < best_d {
                    best = i;
                    best_d = d;
                }
            }

            buckets[best].push(*s);
        }

        for (i, bucket) in buckets.iter().enumerate() {
            if bucket.is_empty() {
                continue;
            }

            let mut sum = [0.0; 3];
            for p in bucket {
                sum[0] += p[0];
                sum[1] += p[1];
                sum[2] += p[2];
            }

            centroids[i] = [
                sum[0] / bucket.len() as f32,
                sum[1] / bucket.len() as f32,
                sum[2] / bucket.len() as f32,
            ];
        }
    }

    centroids
}
