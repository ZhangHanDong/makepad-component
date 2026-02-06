//! FFT Voice Waveform Demo
//!
//! Generates A2UI JSON showing FFT analysis of a simulated voice waveform.
//! Displays time-domain signal and frequency spectrum side by side.
//!
//! Usage:
//!   cargo run -p a2ui-demo --bin fft-demo

use serde_json::{json, Value};
use std::f64::consts::PI;

fn main() {
    let messages = generate_fft_ui();
    let json_str = serde_json::to_string_pretty(&messages).unwrap();

    // Write to ui_live.json for immediate preview
    std::fs::write("ui_live.json", &json_str).unwrap();
    println!("Written ui_live.json ({} bytes)", json_str.len());
    println!("\nTo view: run a2ui-demo (it auto-loads ui_live.json on startup)");
}

/// Simple DFT (for demo purposes - real apps would use FFT library)
fn dft(signal: &[f64]) -> Vec<f64> {
    let n = signal.len();
    let mut magnitude = Vec::with_capacity(n / 2);

    for k in 0..n/2 {
        let mut re = 0.0;
        let mut im = 0.0;
        for (t, &x) in signal.iter().enumerate() {
            let angle = 2.0 * PI * k as f64 * t as f64 / n as f64;
            re += x * angle.cos();
            im -= x * angle.sin();
        }
        magnitude.push((re * re + im * im).sqrt() / (n as f64).sqrt());
    }
    magnitude
}

/// Generate a synthetic voice-like waveform
/// Simulates vowel sound with fundamental + harmonics + noise
fn generate_voice_waveform(samples: usize, sample_rate: f64) -> Vec<f64> {
    let mut signal = vec![0.0; samples];

    // Fundamental frequency ~150 Hz (typical male voice pitch)
    let f0 = 150.0;

    // Add harmonics (voice has rich harmonic content)
    let harmonics = [
        (1.0, 1.0),    // Fundamental
        (2.0, 0.7),    // 2nd harmonic
        (3.0, 0.5),    // 3rd harmonic
        (4.0, 0.3),    // 4th harmonic
        (5.0, 0.2),    // 5th harmonic
        (6.0, 0.15),   // 6th harmonic
        (7.0, 0.1),    // 7th harmonic
        (8.0, 0.08),   // 8th harmonic
    ];

    for i in 0..samples {
        let t = i as f64 / sample_rate;

        // Add each harmonic
        for &(harmonic, amplitude) in &harmonics {
            signal[i] += amplitude * (2.0 * PI * f0 * harmonic * t).sin();
        }

        // Add formant-like resonances (vowel character)
        // F1 ~ 500 Hz, F2 ~ 1500 Hz for "ah" vowel
        signal[i] += 0.3 * (2.0 * PI * 500.0 * t).sin() * (-t * 50.0).exp().max(0.0);
        signal[i] += 0.2 * (2.0 * PI * 1500.0 * t).sin() * (-t * 80.0).exp().max(0.0);

        // Add slight amplitude modulation (natural voice fluctuation)
        signal[i] *= 1.0 + 0.1 * (2.0 * PI * 5.0 * t).sin();

        // Add small noise component
        signal[i] += 0.05 * (((i * 12345 + 67890) % 1000) as f64 / 500.0 - 1.0);
    }

    // Normalize
    let max_val = signal.iter().map(|x| x.abs()).fold(0.0_f64, f64::max);
    if max_val > 0.0 {
        for s in &mut signal {
            *s /= max_val;
        }
    }

    signal
}

fn generate_fft_ui() -> Vec<Value> {
    let mut messages = Vec::new();

    // Parameters
    let sample_rate = 8000.0; // 8 kHz
    let duration = 0.1; // 100ms window
    let samples = (sample_rate * duration) as usize; // 800 samples

    // Generate voice waveform
    let waveform = generate_voice_waveform(samples, sample_rate);

    // Compute FFT
    let spectrum = dft(&waveform);

    // Frequency axis (Hz)
    let freq_resolution = sample_rate / samples as f64;
    let frequencies: Vec<f64> = (0..spectrum.len())
        .map(|k| k as f64 * freq_resolution)
        .collect();

    // Time axis (ms)
    let times: Vec<f64> = (0..samples)
        .map(|i| i as f64 / sample_rate * 1000.0)
        .collect();

    // 1. Begin rendering
    messages.push(json!({
        "beginRendering": {
            "surfaceId": "main",
            "root": "root",
            "styles": {
                "primaryColor": "#00BCD4",
                "font": "Roboto"
            }
        }
    }));

    // 2. Build component tree
    let mut components = Vec::new();

    // Root layout - vertical scroll
    components.push(json!({
        "id": "root",
        "component": {
            "Column": {
                "children": { "explicitList": ["header", "main-content"] }
            }
        }
    }));

    // Header
    components.push(json!({
        "id": "header",
        "component": {
            "Text": {
                "text": { "literalString": "🎤 Voice Waveform FFT Analysis" },
                "usageHint": "h1"
            }
        }
    }));

    // Main content - two rows
    components.push(json!({
        "id": "main-content",
        "component": {
            "Column": {
                "children": { "explicitList": ["waveform-section", "spectrum-section", "info-section"] }
            }
        }
    }));

    // ========== WAVEFORM SECTION ==========
    components.push(json!({
        "id": "waveform-section",
        "component": {
            "Column": {
                "children": { "explicitList": ["wave-title", "wave-desc", "wave-chart"] }
            }
        }
    }));

    components.push(json!({
        "id": "wave-title",
        "component": {
            "Text": {
                "text": { "literalString": "Time Domain - Voice Waveform" },
                "usageHint": "h2"
            }
        }
    }));

    components.push(json!({
        "id": "wave-desc",
        "component": {
            "Text": {
                "text": { "literalString": "Synthetic vowel sound: f₀ = 150 Hz + harmonics (2f₀, 3f₀, ... 8f₀) + formants (F1≈500Hz, F2≈1500Hz)" },
                "usageHint": "body"
            }
        }
    }));

    // Waveform chart (Line plot)
    components.push(json!({
        "id": "wave-chart",
        "component": {
            "Chart": {
                "chartType": "line",
                "title": { "literalString": "Amplitude vs Time (100ms window)" },
                "width": 900.0,
                "height": 300.0,
                "series": [{
                    "name": "Voice",
                    "values": waveform,
                    "x_values": times
                }],
                "colors": ["#00BCD4"],
                "x_label": "Time (ms)",
                "y_label": "Amplitude"
            }
        }
    }));

    // ========== SPECTRUM SECTION ==========
    components.push(json!({
        "id": "spectrum-section",
        "component": {
            "Column": {
                "children": { "explicitList": ["spec-title", "spec-desc", "spec-chart"] }
            }
        }
    }));

    components.push(json!({
        "id": "spec-title",
        "component": {
            "Text": {
                "text": { "literalString": "Frequency Domain - FFT Magnitude Spectrum" },
                "usageHint": "h2"
            }
        }
    }));

    components.push(json!({
        "id": "spec-desc",
        "component": {
            "Text": {
                "text": { "literalString": "DFT: X[k] = Σₙ x[n]·e^(-j2πkn/N)  |  Peaks at f₀=150Hz and harmonics 300, 450, 600, 750, 900, 1050, 1200 Hz" },
                "usageHint": "body"
            }
        }
    }));

    // Spectrum chart (Bar plot for clear frequency bins)
    // Use stem plot style for spectrum
    components.push(json!({
        "id": "spec-chart",
        "component": {
            "Chart": {
                "chartType": "stem",
                "title": { "literalString": "FFT Magnitude |X[k]| vs Frequency (0-4000 Hz)" },
                "width": 900.0,
                "height": 300.0,
                "series": [{
                    "name": "Magnitude",
                    "values": spectrum,
                    "x_values": frequencies
                }],
                "colors": ["#E91E63"]
            }
        }
    }));

    // ========== INFO SECTION ==========
    components.push(json!({
        "id": "info-section",
        "component": {
            "Column": {
                "children": { "explicitList": ["info-title", "info-params", "info-peaks"] }
            }
        }
    }));

    components.push(json!({
        "id": "info-title",
        "component": {
            "Text": {
                "text": { "literalString": "Signal Parameters" },
                "usageHint": "h3"
            }
        }
    }));

    components.push(json!({
        "id": "info-params",
        "component": {
            "Text": {
                "text": { "literalString": format!(
                    "Sample Rate: {} Hz  |  Window: {} ms  |  Samples: {}  |  FFT bins: {}  |  Freq Resolution: {:.1} Hz",
                    sample_rate as i32, (duration * 1000.0) as i32, samples, spectrum.len(), freq_resolution
                )},
                "usageHint": "body"
            }
        }
    }));

    // Find peaks in spectrum
    let mut peaks: Vec<(f64, f64)> = Vec::new();
    for i in 1..spectrum.len()-1 {
        if spectrum[i] > spectrum[i-1] && spectrum[i] > spectrum[i+1] && spectrum[i] > 0.1 {
            peaks.push((frequencies[i], spectrum[i]));
        }
    }
    peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    peaks.truncate(8);
    peaks.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let peaks_str = peaks.iter()
        .map(|(f, m)| format!("{:.0}Hz ({:.2})", f, m))
        .collect::<Vec<_>>()
        .join(", ");

    components.push(json!({
        "id": "info-peaks",
        "component": {
            "Text": {
                "text": { "literalString": format!("Detected Peaks: {}", peaks_str) },
                "usageHint": "body"
            }
        }
    }));

    // Package the surface update
    messages.push(json!({
        "surfaceUpdate": {
            "surfaceId": "main",
            "components": components
        }
    }));

    messages
}
