# Agent Notes (vinyl-cleanup)

## Project overview
- **Product:** Vinyl Transfer Cleaner (desktop app)
- **Goal:** One-click vinyl click/pop/crackle cleanup with strong audio fidelity.
- **Primary platforms:** macOS + Windows (Linux optional).

## Intended stack
- **Desktop app:** Tauri
- **UI:** React + TypeScript
- **Audio/DSP engine:** Rust
- **Optional ML runtime:** ONNX Runtime (local-only inference)

## Repo layout
- `apps/desktop/` – Tauri UI application (React/TS)
- `crates/engine/` – Rust DSP + ML audio engine
- `docs/` – Project documentation
- `tools/` – Developer tooling (evaluation harness, scripts)

## Development notes
- All processing is **local by default**; no network calls in the audio engine.
- The ML model should be used **only if** it demonstrates superior quality vs DSP-only on agreed metrics.
- Keep UI controls minimal; expert options are hidden by default.

## Suggested commands (once implemented)
- UI dev server: `pnpm dev` (or `npm run dev`) from `apps/desktop/`
- Rust engine tests: `cargo test` from repo root or `crates/engine/`
- Tauri dev app: `pnpm tauri dev` (or `npm run tauri dev`)
