# Audio Engine

Rust DSP + ML audio cleanup engine. All processing is local-only.

## Baseline pipeline (Section 6.1)

The baseline DSP pipeline is implemented in `vinyl_engine::run_baseline_pipeline` and follows:

1. **Normalize** input to a target peak.
2. **Impulse detection** using adaptive threshold with local contrast gating combining absolute level, sample-to-sample delta, and neighbor comparisons.
3. **Repair** by interpolating across detected impulses.
4. **Validate** output for clipping and NaNs.

See `crates/engine/src/pipeline.rs` for the step-by-step implementation.

## Baseline quality thresholds

The evaluation harness (`crates/engine/tests/evaluation_harness.rs`) treats the baseline as
“good enough” when, for each clip in the test corpus:

- **Click recall ≥ 0.80**
- **Click precision ≥ 0.80**
- **Transient preservation ≥ 0.90**

These thresholds define the minimum acceptable quality for the DSP-only baseline before
introducing any ML-enhanced repair.
