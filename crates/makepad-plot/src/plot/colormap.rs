use makepad_widgets::*;

/// Colormap for heatmap and other visualizations
#[derive(Clone, Debug, PartialEq)]
pub enum Colormap {
    // Perceptually uniform sequential
    Viridis,
    Plasma,
    Inferno,
    Magma,
    Cividis,  // Colorblind-friendly
    // Diverging
    Coolwarm,
    RdBu,     // Red-Blue diverging
    Spectral,
    // Sequential
    Blues,
    Greens,
    Oranges,
    Reds,
    Greys,
    // Classic
    Jet,      // Rainbow (legacy)
    Hot,      // Black-Red-Yellow-White
    // Special
    Turbo,    // Improved rainbow
    Custom(Vec<(f64, Vec4)>),  // User-defined color stops
}

impl Default for Colormap {
    fn default() -> Self {
        Colormap::Viridis
    }
}

impl Colormap {
    /// Sample a color from the colormap at position t (0.0 to 1.0)
    pub fn sample(&self, t: f64) -> Vec4 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Colormap::Viridis => {
                // Perceptually uniform purple-green-yellow
                let r = 0.267 + t * 0.329 - t * t * 0.5 + t * t * t * 0.9;
                let g = 0.004 + t * 0.873;
                let b = 0.329 + t * 0.5 - t * t * 0.6;
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Plasma => {
                // Purple-red-yellow
                let r = 0.05 + t * 0.9 + t * t * 0.05;
                let g = t * t * 0.9;
                let b = 0.53 + (1.0 - t) * 0.47 - t * t * 0.6;
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Inferno => {
                // Black-purple-red-yellow
                let r = t * t * 1.2;
                let g = t * t * t * 1.5;
                let b = (1.0 - t) * t * 2.0 + 0.1 * t;
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Magma => {
                // Black-purple-pink-white
                let r = t * t * 0.8 + t * 0.2;
                let g = t * t * t * 1.2;
                let b = t * 0.5 + (1.0 - t) * t * 1.0;
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Cividis => {
                // Colorblind-friendly blue-yellow
                let r = -0.01 + t * 1.0 + t * t * 0.01;
                let g = 0.14 + t * 0.72;
                let b = 0.35 + t * 0.1 - t * t * 0.35;
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Coolwarm => {
                // Blue-white-red diverging
                let r = if t < 0.5 { 0.2 + t * 1.6 } else { 1.0 };
                let g = if t < 0.5 { 0.2 + t * 1.0 } else { 1.0 - (t - 0.5) * 1.6 };
                let b = if t < 0.5 { 1.0 } else { 1.0 - (t - 0.5) * 1.6 };
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::RdBu => {
                // Red-white-blue diverging (red=high, blue=low)
                let r = if t < 0.5 { 0.1 + t * 1.8 } else { 1.0 - (t - 0.5) * 1.4 };
                let g = if t < 0.5 { t * 1.8 } else { 0.9 - (t - 0.5) * 1.6 };
                let b = if t < 0.5 { 1.0 - t * 0.2 } else { 0.9 - (t - 0.5) * 1.0 };
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Spectral => {
                // Red-orange-yellow-green-blue (diverging rainbow)
                let (r, g, b) = if t < 0.25 {
                    let s = t / 0.25;
                    (0.62 + s * 0.38, 0.0 + s * 0.5, 0.26 * (1.0 - s))
                } else if t < 0.5 {
                    let s = (t - 0.25) / 0.25;
                    (1.0, 0.5 + s * 0.5, 0.0)
                } else if t < 0.75 {
                    let s = (t - 0.5) / 0.25;
                    (1.0 - s * 0.5, 1.0 - s * 0.2, s * 0.4)
                } else {
                    let s = (t - 0.75) / 0.25;
                    (0.5 - s * 0.3, 0.8 - s * 0.4, 0.4 + s * 0.6)
                };
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Blues => {
                let r = 1.0 - t * 0.8;
                let g = 1.0 - t * 0.5;
                let b = 1.0;
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Greens => {
                let r = 1.0 - t * 0.75;
                let g = 1.0 - t * 0.15;
                let b = 1.0 - t * 0.7;
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Oranges => {
                let r = 1.0;
                let g = 1.0 - t * 0.6;
                let b = 1.0 - t * 0.85;
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Reds => {
                let r = 1.0;
                let g = 1.0 - t * 0.85;
                let b = 1.0 - t * 0.85;
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Greys => {
                let v = 1.0 - t * 0.9;
                vec4(v as f32, v as f32, v as f32, 1.0)
            }
            Colormap::Jet => {
                // Classic rainbow: blue-cyan-green-yellow-red
                let (r, g, b) = if t < 0.125 {
                    (0.0, 0.0, 0.5 + t * 4.0)
                } else if t < 0.375 {
                    let s = (t - 0.125) / 0.25;
                    (0.0, s, 1.0)
                } else if t < 0.625 {
                    let s = (t - 0.375) / 0.25;
                    (s, 1.0, 1.0 - s)
                } else if t < 0.875 {
                    let s = (t - 0.625) / 0.25;
                    (1.0, 1.0 - s, 0.0)
                } else {
                    let s = (t - 0.875) / 0.125;
                    (1.0 - s * 0.5, 0.0, 0.0)
                };
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Hot => {
                // Black-red-yellow-white
                let (r, g, b) = if t < 0.33 {
                    (t * 3.0, 0.0, 0.0)
                } else if t < 0.67 {
                    let s = (t - 0.33) / 0.34;
                    (1.0, s, 0.0)
                } else {
                    let s = (t - 0.67) / 0.33;
                    (1.0, 1.0, s)
                };
                vec4(r as f32, g as f32, b as f32, 1.0)
            }
            Colormap::Turbo => {
                // Improved rainbow with better perceptual uniformity
                let r = 0.13572 + t * (4.6153 + t * (-42.66 + t * (132.13 + t * (-152.95 + t * 56.31))));
                let g = 0.09140 + t * (2.1745 + t * (4.8321 + t * (-36.60 + t * (43.05 + t * (-13.22)))));
                let b = 0.10667 + t * (12.755 + t * (-60.58 + t * (109.33 + t * (-87.15 + t * 25.25))));
                vec4(r.clamp(0.0, 1.0) as f32, g.clamp(0.0, 1.0) as f32, b.clamp(0.0, 1.0) as f32, 1.0)
            }
            Colormap::Custom(stops) => {
                if stops.is_empty() {
                    return vec4(0.5, 0.5, 0.5, 1.0);
                }
                if stops.len() == 1 {
                    return stops[0].1;
                }
                // Find surrounding stops and interpolate
                for i in 0..stops.len() - 1 {
                    if t <= stops[i + 1].0 {
                        let t0 = stops[i].0;
                        let t1 = stops[i + 1].0;
                        let c0 = stops[i].1;
                        let c1 = stops[i + 1].1;
                        let s = if t1 > t0 { (t - t0) / (t1 - t0) } else { 0.0 };
                        return vec4(
                            c0.x + (c1.x - c0.x) * s as f32,
                            c0.y + (c1.y - c0.y) * s as f32,
                            c0.z + (c1.z - c0.z) * s as f32,
                            c0.w + (c1.w - c0.w) * s as f32,
                        );
                    }
                }
                stops.last().unwrap().1
            }
        }
    }

    /// Get a list of all named colormaps
    pub fn all_named() -> Vec<Colormap> {
        vec![
            Colormap::Viridis,
            Colormap::Plasma,
            Colormap::Inferno,
            Colormap::Magma,
            Colormap::Cividis,
            Colormap::Coolwarm,
            Colormap::RdBu,
            Colormap::Spectral,
            Colormap::Blues,
            Colormap::Greens,
            Colormap::Oranges,
            Colormap::Reds,
            Colormap::Greys,
            Colormap::Jet,
            Colormap::Hot,
            Colormap::Turbo,
        ]
    }

    /// Get the name of this colormap
    pub fn name(&self) -> &'static str {
        match self {
            Colormap::Viridis => "Viridis",
            Colormap::Plasma => "Plasma",
            Colormap::Inferno => "Inferno",
            Colormap::Magma => "Magma",
            Colormap::Cividis => "Cividis",
            Colormap::Coolwarm => "Coolwarm",
            Colormap::RdBu => "RdBu",
            Colormap::Spectral => "Spectral",
            Colormap::Blues => "Blues",
            Colormap::Greens => "Greens",
            Colormap::Oranges => "Oranges",
            Colormap::Reds => "Reds",
            Colormap::Greys => "Greys",
            Colormap::Jet => "Jet",
            Colormap::Hot => "Hot",
            Colormap::Turbo => "Turbo",
            Colormap::Custom(_) => "Custom",
        }
    }

    /// Create a custom colormap from color stops
    /// Stops should be (position, color) pairs where position is 0.0 to 1.0
    pub fn custom(stops: Vec<(f64, Vec4)>) -> Self {
        Colormap::Custom(stops)
    }
}

// =============================================================================
// Normalization for Colormaps
// =============================================================================

/// Linear normalization: maps [vmin, vmax] to [0, 1]
#[derive(Clone, Debug)]
pub struct Normalize {
    pub vmin: f64,
    pub vmax: f64,
    pub clip: bool,
}

impl Normalize {
    pub fn new(vmin: f64, vmax: f64) -> Self {
        Self { vmin, vmax, clip: true }
    }

    pub fn with_clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Normalize a value to [0, 1]
    pub fn normalize(&self, value: f64) -> f64 {
        if self.vmax == self.vmin {
            return 0.5;
        }
        let t = (value - self.vmin) / (self.vmax - self.vmin);
        if self.clip { t.clamp(0.0, 1.0) } else { t }
    }

    /// Inverse: convert [0, 1] back to original scale
    pub fn inverse(&self, t: f64) -> f64 {
        self.vmin + t * (self.vmax - self.vmin)
    }
}

impl Default for Normalize {
    fn default() -> Self {
        Self { vmin: 0.0, vmax: 1.0, clip: true }
    }
}

/// Logarithmic normalization: maps [vmin, vmax] to [0, 1] on log scale
/// Values must be positive
#[derive(Clone, Debug)]
pub struct LogNorm {
    pub vmin: f64,
    pub vmax: f64,
    pub clip: bool,
    log_vmin: f64,
    log_vmax: f64,
}

impl LogNorm {
    pub fn new(vmin: f64, vmax: f64) -> Self {
        let vmin = vmin.max(1e-10);  // Ensure positive
        let vmax = vmax.max(vmin + 1e-10);
        Self {
            vmin,
            vmax,
            clip: true,
            log_vmin: vmin.log10(),
            log_vmax: vmax.log10(),
        }
    }

    pub fn with_clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    /// Normalize a value to [0, 1] on log scale
    pub fn normalize(&self, value: f64) -> f64 {
        if value <= 0.0 {
            return 0.0;
        }
        let log_val = value.log10();
        let t = (log_val - self.log_vmin) / (self.log_vmax - self.log_vmin);
        if self.clip { t.clamp(0.0, 1.0) } else { t }
    }

    /// Inverse: convert [0, 1] back to original scale
    pub fn inverse(&self, t: f64) -> f64 {
        10.0_f64.powf(self.log_vmin + t * (self.log_vmax - self.log_vmin))
    }
}

impl Default for LogNorm {
    fn default() -> Self {
        Self::new(1.0, 10.0)
    }
}

/// Symmetric log normalization: handles negative values
#[derive(Clone, Debug)]
pub struct SymLogNorm {
    pub vmin: f64,
    pub vmax: f64,
    pub linthresh: f64,  // Linear threshold
    pub clip: bool,
}

impl SymLogNorm {
    pub fn new(vmin: f64, vmax: f64, linthresh: f64) -> Self {
        Self {
            vmin,
            vmax,
            linthresh: linthresh.abs().max(1e-10),
            clip: true,
        }
    }

    pub fn with_clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }

    fn transform(&self, value: f64) -> f64 {
        if value.abs() <= self.linthresh {
            value / self.linthresh
        } else {
            let sign = if value >= 0.0 { 1.0 } else { -1.0 };
            sign * (1.0 + (value.abs() / self.linthresh).log10())
        }
    }

    /// Normalize a value to [0, 1]
    pub fn normalize(&self, value: f64) -> f64 {
        let t_val = self.transform(value);
        let t_min = self.transform(self.vmin);
        let t_max = self.transform(self.vmax);

        if t_max == t_min {
            return 0.5;
        }

        let t = (t_val - t_min) / (t_max - t_min);
        if self.clip { t.clamp(0.0, 1.0) } else { t }
    }
}

impl Default for SymLogNorm {
    fn default() -> Self {
        Self::new(-10.0, 10.0, 1.0)
    }
}

