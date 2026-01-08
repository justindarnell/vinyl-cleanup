#[derive(Debug, Clone, Copy)]
pub struct ClickMetrics {
    pub recall: f32,
    pub precision: f32,
}

pub fn click_precision_recall(
    detected: &[usize],
    expected: &[usize],
    tolerance: usize,
) -> ClickMetrics {
    if expected.is_empty() && detected.is_empty() {
        return ClickMetrics {
            recall: 1.0,
            precision: 1.0,
        };
    }

    let mut true_positive = 0;
    let mut matched = vec![false; expected.len()];

    for detection in detected {
        if let Some((index, _)) = expected
            .iter()
            .enumerate()
            .find(|(idx, &expected_index)| {
                !matched[*idx] && detection.abs_diff(expected_index) <= tolerance
            })
        {
            matched[index] = true;
            true_positive += 1;
        }
    }

    let recall = if expected.is_empty() {
        0.0
    } else {
        true_positive as f32 / expected.len() as f32
    };
    let precision = if detected.is_empty() {
        0.0
    } else {
        true_positive as f32 / detected.len() as f32
    };

    ClickMetrics { recall, precision }
}

pub fn transient_preservation(
    original: &[f32],
    repaired: &[f32],
    transient_regions: &[(usize, usize)],
) -> f32 {
    if transient_regions.is_empty() {
        return 1.0;
    }

    let mut preserved_energy = 0.0_f32;
    let mut original_energy = 0.0_f32;

    for (start, end) in transient_regions {
        let end = (*end).min(original.len());
        let start = (*start).min(end);
        for index in start..end {
            let original_sample = original[index];
            let repaired_sample = repaired[index];
            let diff = original_sample - repaired_sample;
            preserved_energy += diff * diff;
            original_energy += original_sample * original_sample;
        }
    }

    if original_energy == 0.0 {
        1.0
    } else {
        1.0 - (preserved_energy / original_energy).min(1.0)
    }
}
