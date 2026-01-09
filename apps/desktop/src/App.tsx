import "./App.css";

const metrics = [
  { label: "Clicks detected", value: "1,024", trend: "+12%" },
  { label: "Noise reduction", value: "-18 dB", trend: "Balanced" },
  { label: "Harmonic integrity", value: "98%", trend: "Preserved" }
];

const activity = [
  "Drop audio file to start cleaning.",
  "Auto profile: Warm vinyl (78 RPM preset).",
  "Preview cleanup ready."
];

const presets = ["Warm vinyl", "Modern reissue", "Live bootleg"];

function App() {
  return (
    <div className="app">
      <header className="app__header">
        <div>
          <p className="app__eyebrow">Vinyl Transfer Cleaner</p>
          <h1>Restore the soul of every groove.</h1>
          <p className="app__subtext">
            One-click cleanup tuned for pops, crackle, and tape hiss — with the
            music left untouched.
          </p>
        </div>
        <div className="app__status">
          <div>
            <p className="app__status-label">Engine status</p>
            <p className="app__status-value">Idle • Ready</p>
          </div>
          <button className="app__secondary" type="button">
            Connect hardware
          </button>
        </div>
      </header>

      <main className="app__grid">
        <section className="card card--drop">
          <div className="card__header">
            <h2>Source audio</h2>
            <span className="pill">PCM/WAV/AIFF</span>
          </div>
          <div className="dropzone">
            <p>Drag & drop a recording or browse files.</p>
            <button className="app__primary" type="button">
              Choose file
            </button>
          </div>
          <div className="dropzone__footer">
            <div>
              <p className="label">Auto detect</p>
              <p className="value">Needle drop • 96 kHz</p>
            </div>
            <div>
              <p className="label">Selected preset</p>
              <p className="value">Warm vinyl</p>
            </div>
          </div>
        </section>

        <section className="card">
          <div className="card__header">
            <h2>Cleanup controls</h2>
            <span className="pill pill--accent">Real-time</span>
          </div>
          <div className="control-stack">
            <div>
              <p className="label">Preset</p>
              <div className="chip-row">
                {presets.map((preset) => (
                  <button className="chip" key={preset} type="button">
                    {preset}
                  </button>
                ))}
              </div>
            </div>
            <div>
              <div className="slider-row">
                <div>
                  <p className="label">Cleanup intensity</p>
                  <p className="value">Balanced</p>
                </div>
                <input aria-label="Cleanup intensity" defaultValue={60} min={0} max={100} type="range" />
              </div>
            </div>
            <div>
              <div className="toggle">
                <input defaultChecked id="preview" type="checkbox" />
                <label htmlFor="preview">Instant A/B preview</label>
              </div>
              <div className="toggle">
                <input defaultChecked id="transient" type="checkbox" />
                <label htmlFor="transient">Preserve transients</label>
              </div>
            </div>
          </div>
          <button className="app__primary app__primary--wide" type="button">
            Clean + Export
          </button>
        </section>

        <section className="card">
          <div className="card__header">
            <h2>Live feedback</h2>
            <span className="pill">Auto</span>
          </div>
          <div className="metric-grid">
            {metrics.map((metric) => (
              <div className="metric" key={metric.label}>
                <p className="metric__label">{metric.label}</p>
                <p className="metric__value">{metric.value}</p>
                <p className="metric__trend">{metric.trend}</p>
              </div>
            ))}
          </div>
          <div className="waveform">
            <div className="waveform__bars" aria-hidden="true"></div>
            <p className="waveform__label">Preview waveform (before/after)</p>
          </div>
        </section>

        <section className="card card--activity">
          <div className="card__header">
            <h2>Session activity</h2>
            <span className="pill pill--muted">Queue</span>
          </div>
          <ul className="activity-list">
            {activity.map((item) => (
              <li key={item}>{item}</li>
            ))}
          </ul>
          <details>
            <summary>Advanced options</summary>
            <div className="advanced">
              <div>
                <p className="label">Click profile</p>
                <p className="value">Adaptive split-band</p>
              </div>
              <div>
                <p className="label">Crackle guard</p>
                <p className="value">Vintage lacquer</p>
              </div>
              <div>
                <p className="label">Noise floor</p>
                <p className="value">Match room tone</p>
              </div>
            </div>
          </details>
        </section>
      </main>
    </div>
  );
}

export default App;
