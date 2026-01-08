#[derive(Debug, Clone)]
/// Configuration parameters for the baseline normalization and impulse-detection pipeline.
///
/// These values control how the input signal is normalized and how impulsive artifacts
/// are detected and filtered. Adjust them to trade off between sensitivity to impulses
/// and robustness to normal signal variation.
pub struct BaselineConfig {
    /// Target absolute peak level after normalization.
    ///
    /// The input is scaled so that its maximum absolute sample value is close to this
    /// value (provided the original peak is non-zero). Typical values are in the range
    /// `[0.0, 1.0]`.
    pub target_peak: f32,
    /// Multiplier applied to the mean absolute signal level to form the impulse
    /// detection threshold.
    ///
    /// The mean absolute value of the input is multiplied by this factor, and the
    /// result is combined with `impulse_abs_min` to obtain the effective threshold
    /// for considering a sample as a potential impulse:  
    /// `threshold = max(mean_abs * impulse_threshold_multiplier, impulse_abs_min)`.
    pub impulse_threshold_multiplier: f32,
    /// Minimum absolute amplitude that a sample must have to be considered as an
    /// impulse candidate.
    ///
    /// This acts as a floor on the detection threshold so that impulses are not
    /// detected purely due to very low-level noise when the overall signal level is
    /// small.
    pub impulse_abs_min: f32,
    /// Minimum allowed difference between consecutive samples for an impulse to be
    /// detected.
    ///
    /// A sample is only flagged as an impulse if the absolute difference between it
    /// and the previous sample is at least this value. Increasing this value reduces
    /// sensitivity to small, rapid changes; decreasing it makes detection more
    /// sensitive.
    pub diff_threshold: f32,
}

impl Default for BaselineConfig {
    fn default() -> Self {
        Self {
            target_peak: 0.95,
            impulse_threshold_multiplier: 6.0,
            impulse_abs_min: 0.25,
            diff_threshold: 0.2,
        }
    }
}

#[derive(Debug, Clone)]
/// Summary of validation checks performed on a processed audio buffer.
///
/// This is typically produced by [`validate_output`] and attached to
/// [`BaselineOutput::validation`] so that callers can verify that the
/// resulting signal is numerically well‑behaved.
pub struct ValidationResult {
    /// Maximum absolute sample value observed in the validated buffer.
    ///
    /// This can be used to confirm that normalization behaved as expected
    /// and that the signal stays within the desired peak range.
    pub peak: f32,
    /// Number of samples whose absolute value exceeded the nominal
    /// full‑scale range, typically `[-1.0, 1.0]`.
    ///
    /// A non‑zero value indicates that the signal clipped or would clip when
    /// rendered, which may cause audible distortion.
    pub clipped_samples: usize,
    /// Indicates whether any `NaN` values were detected in the buffer.
    ///
    /// If this is `true`, it usually points to numerical instability or bugs
    /// in earlier processing stages, and downstream consumers should treat
    /// the output as invalid.
    pub has_nan: bool,
}

#[derive(Debug, Clone)]
/// Output of the baseline processing pipeline.
///
/// This contains the normalized signal, information about detected impulse
/// artifacts, a repaired version of the signal, and a [`ValidationResult`]
/// that callers can inspect for numerical issues (clipping, `NaN`s, etc.).
pub struct BaselineOutput {
    /// Input signal after peak normalization using [`BaselineConfig::target_peak`].
    pub normalized: Vec<f32>,
    /// Indices (in samples) where impulses/outliers were detected in the
    /// normalized signal.
    pub detected_impulses: Vec<usize>,
    /// Signal after repairing/removing the detected impulses.
    ///
    /// This is typically the buffer that downstream processing should use.
    pub repaired: Vec<f32>,
    /// Validation metrics computed from the repaired signal.
    ///
    /// Callers should check this before trusting the output, in particular
    /// [`ValidationResult::has_nan`] and [`ValidationResult::clipped_samples`].
    pub validation: ValidationResult,
}

/// Runs the baseline processing pipeline on a single-channel signal.
///
/// This pipeline performs four main steps:
/// 1. **Normalization** – Scales the input so that its peak amplitude matches
///    `config.target_peak`.
/// 2. **Impulse detection** – Identifies impulsive artifacts in the normalized
///    signal using the thresholds defined in `BaselineConfig`.
/// 3. **Impulse repair** – Produces a repaired version of the signal where
///    detected impulses have been mitigated.
/// 4. **Validation** – Computes basic quality metrics (such as peak level,
///    clipped samples, and NaN presence) on the repaired signal.
///
/// # Parameters
/// - `input`: Input samples as a slice of `f32`, typically a mono
///   time-domain audio signal.
/// - `config`: Baseline pipeline configuration controlling normalization
///   and impulse detection behavior.
///
/// # Returns
/// A [`BaselineOutput`] struct containing:
/// - `normalized`: The normalized version of `input`.
/// - `detected_impulses`: Indices of samples classified as impulses.
/// - `repaired`: The signal after impulse repair.
/// - `validation`: Summary metrics describing the repaired signal.
///
/// # Examples
/// ```
/// use vinyl_engine::pipeline::{BaselineConfig, run_baseline_pipeline};
///
/// // Example mono signal
/// let samples: Vec<f32> = vec![0.0, 0.5, -0.4, 1.2, -1.1, 0.3];
///
/// // Use default configuration for the baseline pipeline
/// let config = BaselineConfig::default();
///
/// // Run the pipeline
/// let output = run_baseline_pipeline(&samples, &config);
///
/// // Access normalized samples and repaired output
/// assert_eq!(output.normalized.len(), samples.len());
/// assert_eq!(output.repaired.len(), samples.len());
/// ```
pub fn run_baseline_pipeline(input: &[f32], config: &BaselineConfig) -> BaselineOutput {
    let normalized = normalize(input, config.target_peak);
    let detected_impulses = detect_impulses(&normalized, config);
    let repaired = repair_impulses(&normalized, &detected_impulses);
    let validation = validate_output(&repaired);

    BaselineOutput {
        normalized,
        detected_impulses,
        repaired,
        validation,
    }
}

fn normalize(input: &[f32], target_peak: f32) -> Vec<f32> {
    let peak = input
        .iter()
        .map(|sample| sample.abs())
        .fold(0.0_f32, f32::max);

    if peak <= 0.0 {
        return input.to_vec();
    }

    let scale = target_peak / peak;
    input.iter().map(|sample| sample * scale).collect()
}

fn detect_impulses(input: &[f32], config: &BaselineConfig) -> Vec<usize> {
    if input.is_empty() {
        return Vec::new();
    }

    let mean_abs = input.iter().map(|sample| sample.abs()).sum::<f32>() / input.len() as f32;
    let threshold = (mean_abs * config.impulse_threshold_multiplier).max(config.impulse_abs_min);
    let mut impulses = Vec::new();

    if input.len() < 3 {
        return impulses;
    }

    for index in 1..input.len() - 1 {
        let sample = input[index];
        let prev = input[index - 1];
        let next = input[index + 1];
        let diff = (sample - prev).abs();
        let local_mean = (prev.abs() + next.abs()) * 0.5;
        let abs = sample.abs();
        if abs >= threshold
            && diff >= config.diff_threshold
            && abs >= local_mean * 2.5
            && abs >= prev.abs()
            && abs >= next.abs()
        {
            impulses.push(index);
        }
    }

    impulses
}

fn repair_impulses(input: &[f32], impulses: &[usize]) -> Vec<f32> {
    if impulses.is_empty() {
        return input.to_vec();
    }

    let mut repaired = input.to_vec();
    let mut sorted = impulses.to_vec();
    sorted.sort_unstable();

    let mut start = 0;
    while start < sorted.len() {
        let mut end = start;
        while end + 1 < sorted.len() && sorted[end + 1] == sorted[end] + 1 {
            end += 1;
        }

        let left_index = sorted[start].saturating_sub(1);
        let right_index = (sorted[end] + 1).min(input.len() - 1);
        let left_value = input[left_index];
        let right_value = input[right_index];
        let span = (right_index - left_index) as f32;

        if span != 0.0 {
            for (offset, index) in (left_index + 1..=right_index - 1).enumerate() {
                let t = (offset + 1) as f32 / span;
                repaired[index] = left_value + (right_value - left_value) * t;
            }
        }

        start = end + 1;
    }

    repaired
}

fn validate_output(output: &[f32]) -> ValidationResult {
    let mut peak = 0.0_f32;
    let mut clipped_samples = 0;
    let mut has_nan = false;

    for sample in output {
        if sample.is_nan() {
            has_nan = true;
        }
        let abs = sample.abs();
        if abs > peak {
            peak = abs;
        }
        if abs > 1.0 {
            clipped_samples += 1;
        }
    }

    ValidationResult {
        peak,
        clipped_samples,
        has_nan,
    }
}
