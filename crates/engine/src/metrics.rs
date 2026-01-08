#[derive(Debug, Clone, Copy)]
pub struct ClickMetrics {
    pub recall: f32,
    pub precision: f32,
}

/// Computes click detection precision and recall metrics.
///
/// This function evaluates the quality of impulse detection by comparing detected
/// impulse positions against expected (ground truth) positions. A detection is
/// considered a true positive if it falls within `tolerance` samples of an expected
/// impulse.
///
/// # Parameters
/// - `detected`: Indices of samples classified as impulses by the detector.
/// - `expected`: Ground truth indices of actual impulses in the signal.
/// - `tolerance`: Maximum allowed distance (in samples) between a detected impulse
///   and an expected impulse for them to be considered a match. For example,
///   `tolerance = 1` means a detection at index 100 will match an expected impulse
///   at index 99, 100, or 101.
///
/// # Returns
/// A [`ClickMetrics`] struct containing:
/// - `recall`: Fraction of expected impulses that were successfully detected
///   (true positives / total expected). Range: [0.0, 1.0].
/// - `precision`: Fraction of detections that correspond to actual impulses
///   (true positives / total detected). Range: [0.0, 1.0].
///
/// # Edge Cases
/// - If both `detected` and `expected` are empty, returns recall=1.0 and precision=1.0
///   (perfect performance when there's nothing to detect).
/// - If only `expected` is empty but `detected` is not, returns recall=1.0 (no true
///   impulses were missed) and precision=0.0 (all detections are false positives).
/// - If only `detected` is empty, returns recall=0.0 and precision=0.0.
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
        1.0
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

/// Measures how well transient energy is preserved after signal repair.
///
/// This metric quantifies the similarity between the original and repaired signals
/// within specified transient regions (e.g., drum hits, plucks, or other sharp
/// musical events). It computes the normalized mean squared error in these regions
/// and returns a score where 1.0 means perfect preservation and 0.0 means complete
/// loss of transient energy.
///
/// # Parameters
/// - `original`: The original input signal before repair.
/// - `repaired`: The signal after impulse repair/removal. **Must have the same length as `original`**.
/// - `transient_regions`: List of `(start, end)` index pairs defining regions
///   containing important transients that should be preserved. Indices are clamped
///   to the valid range `[0, original.len())`.
///
/// # Returns
/// A score in the range [0.0, 1.0]:
/// - `1.0` indicates the repaired signal perfectly matches the original in transient regions.
/// - `0.0` indicates complete loss or distortion of transient energy.
/// - Values close to 1.0 indicate good preservation; values significantly below 1.0
///   suggest the repair process damaged important musical content.
///
/// # Edge Cases
/// - If `transient_regions` is empty, returns 1.0 (no transients to preserve).
/// - If the original signal has zero energy in the transient regions, returns 1.0.
///
/// # Panics
/// Panics if `repaired.len()` is less than `original.len()`, as this would cause out-of-bounds
/// access when iterating through transient regions.
///
/// # Implementation Note
/// The score is computed as `1.0 - (squared_error / original_energy)`, where
/// `squared_error` is the sum of squared differences between original and repaired
/// samples in the transient regions, and `original_energy` is the sum of squared
/// original samples in those regions.
pub fn transient_preservation(
    original: &[f32],
    repaired: &[f32],
    transient_regions: &[(usize, usize)],
) -> f32 {
    if transient_regions.is_empty() {
        return 1.0;
    }

    // Validate that repaired has at least as many samples as original to prevent
    // out-of-bounds access in the loop below.
    assert!(
        repaired.len() >= original.len(),
        "repaired signal length ({}) must be at least as long as original signal length ({})",
        repaired.len(),
        original.len()
    );

    let mut squared_error = 0.0_f32;
    let mut original_energy = 0.0_f32;

    for (start, end) in transient_regions {
        let end = (*end).min(original.len());
        let start = (*start).min(end);
        for index in start..end {
            let original_sample = original[index];
            let repaired_sample = repaired[index];
            let diff = original_sample - repaired_sample;
            squared_error += diff * diff;
            original_energy += original_sample * original_sample;
        }
    }

    if original_energy == 0.0 {
        1.0
    } else {
        1.0 - (squared_error / original_energy).min(1.0)
    }
}
