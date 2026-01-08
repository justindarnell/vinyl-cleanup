#[derive(Debug, Clone)]
pub struct BaselineConfig {
    pub target_peak: f32,
    pub impulse_threshold_multiplier: f32,
    pub impulse_abs_min: f32,
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
pub struct ValidationResult {
    pub peak: f32,
    pub clipped_samples: usize,
    pub has_nan: bool,
}

#[derive(Debug, Clone)]
pub struct BaselineOutput {
    pub normalized: Vec<f32>,
    pub detected_impulses: Vec<usize>,
    pub repaired: Vec<f32>,
    pub validation: ValidationResult,
}

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
        if abs >= config.impulse_abs_min
            && abs >= threshold
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

        for (offset, index) in (left_index + 1..=right_index - 1).enumerate() {
            let t = (offset + 1) as f32 / span;
            repaired[index] = left_value + (right_value - left_value) * t;
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
