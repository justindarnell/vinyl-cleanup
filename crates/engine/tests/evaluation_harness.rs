use vinyl_engine::{
    click_precision_recall, run_baseline_pipeline, transient_preservation, BaselineConfig,
};

struct TestClip {
    name: &'static str,
    samples: Vec<f32>,
    impulses: Vec<usize>,
    transients: Vec<(usize, usize)>,
}

fn generate_corpus() -> Vec<TestClip> {
    let mut clips = Vec::new();

    let mut sine = Vec::with_capacity(2048);
    for i in 0..2048 {
        let phase = (i as f32 / 2048.0) * std::f32::consts::TAU * 8.0;
        sine.push(0.3 * phase.sin());
    }

    let mut sine_with_clicks = sine.clone();
    let impulses = vec![256, 1024, 1536];
    for &index in &impulses {
        sine_with_clicks[index] += 1.2;
    }
    clips.push(TestClip {
        name: "sine_with_clicks",
        samples: sine_with_clicks,
        impulses,
        transients: Vec::new(),
    });

    let mut burst = vec![0.0_f32; 2048];
    for i in 900..980 {
        burst[i] = (1.0 - ((i - 900) as f32 / 80.0)) * 0.7;
    }
    let mut burst_with_clicks = burst.clone();
    let impulses = vec![200, 1300];
    for &index in &impulses {
        burst_with_clicks[index] -= 1.0;
    }
    clips.push(TestClip {
        name: "burst_with_clicks",
        samples: burst_with_clicks,
        impulses,
        transients: vec![(900, 980)],
    });

    // Edge case: Empty signal
    clips.push(TestClip {
        name: "empty_signal",
        samples: Vec::new(),
        impulses: Vec::new(),
        transients: Vec::new(),
    });

    // Edge case: Single sample signal (len < 3, no detection possible)
    clips.push(TestClip {
        name: "single_sample",
        samples: vec![0.5],
        impulses: Vec::new(),
        transients: Vec::new(),
    });

    // Edge case: Two sample signal (len < 3, no detection possible)
    clips.push(TestClip {
        name: "two_samples",
        samples: vec![0.3, 0.8],
        impulses: Vec::new(),
        transients: Vec::new(),
    });

    // Edge case: All-zero signal
    clips.push(TestClip {
        name: "all_zero",
        samples: vec![0.0; 512],
        impulses: Vec::new(),
        transients: Vec::new(),
    });

    // Edge case: Near-zero amplitude signal
    clips.push(TestClip {
        name: "near_zero",
        samples: vec![0.001; 512],
        impulses: Vec::new(),
        transients: Vec::new(),
    });

    // Edge case: Consecutive impulses (Note: neighbor-based detection cannot detect
    // all samples in a consecutive run, only peaks. This test validates graceful handling)
    let mut consecutive_impulses_signal = vec![0.15; 512];
    // Create impulse groups where the middle one is highest (detectable as local peak)
    consecutive_impulses_signal[99] = 0.3;
    consecutive_impulses_signal[100] = 3.0; // This peak should be detected
    consecutive_impulses_signal[101] = 0.3;
    consecutive_impulses_signal[199] = 0.3;
    consecutive_impulses_signal[200] = 3.0; // This peak should be detected
    consecutive_impulses_signal[201] = 0.3;
    clips.push(TestClip {
        name: "consecutive_impulses",
        samples: consecutive_impulses_signal,
        impulses: vec![100, 200], // Only the peaks of each group
        transients: Vec::new(),
    });

    // Edge case: Impulses near edges (but not at index 0 or len-1, which cannot be detected)
    let mut edge_impulses_signal = vec![0.15; 512];
    let impulses = vec![1, 3, 508, 510]; // Near start and end, but detectable and non-consecutive
    for &index in &impulses {
        edge_impulses_signal[index] = 3.0; // Larger spike to ensure detection
    }
    clips.push(TestClip {
        name: "impulses_near_edges",
        samples: edge_impulses_signal,
        impulses,
        transients: Vec::new(),
    });

    clips
}

#[test]
fn baseline_pipeline_meets_quality_thresholds() {
    let config = BaselineConfig::default();
    let corpus = generate_corpus();

    // Allow detected impulses to deviate by ±1 sample from the ground truth.
    // This tight tolerance is appropriate for the synthetic clips defined in `generate_corpus`.
    let click_tolerance_samples: usize = 1;

    for clip in corpus {
        let output = run_baseline_pipeline(&clip.samples, &config);
        let metrics =
            click_precision_recall(&output.detected_impulses, &clip.impulses, click_tolerance_samples);
        let transient_score =
            transient_preservation(&clip.samples, &output.repaired, &clip.transients);

        assert!(
            metrics.recall >= 0.8,
            "{} recall below threshold: {:.2}",
            clip.name,
            metrics.recall
        );
        assert!(
            metrics.precision >= 0.8,
            "{} precision below threshold: {:.2}",
            clip.name,
            metrics.precision
        );
        assert!(
            transient_score >= 0.9,
            "{} transient preservation below threshold: {:.2}",
            clip.name,
            transient_score
        );
    }
}
