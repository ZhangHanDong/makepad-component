/// Scale type for axis transformation
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum ScaleType {
    #[default]
    Linear,
    Log,      // Logarithmic (base 10)
    SymLog,   // Symmetric log (handles negative values)
    Time,     // Time axis (values are Unix timestamps in seconds)
}

impl ScaleType {
    /// Transform a value according to the scale type
    pub fn transform(&self, value: f64) -> f64 {
        match self {
            ScaleType::Linear | ScaleType::Time => value,
            ScaleType::Log => {
                if value > 0.0 {
                    value.log10()
                } else {
                    f64::NEG_INFINITY
                }
            }
            ScaleType::SymLog => {
                // Symmetric log: sign(x) * log10(1 + |x|)
                let sign = if value >= 0.0 { 1.0 } else { -1.0 };
                sign * (1.0 + value.abs()).log10()
            }
        }
    }

    /// Inverse transform a value
    pub fn inverse(&self, value: f64) -> f64 {
        match self {
            ScaleType::Linear | ScaleType::Time => value,
            ScaleType::Log => 10.0_f64.powf(value),
            ScaleType::SymLog => {
                let sign = if value >= 0.0 { 1.0 } else { -1.0 };
                sign * (10.0_f64.powf(value.abs()) - 1.0)
            }
        }
    }

    /// Generate nice tick values for this scale type
    pub fn generate_ticks(&self, min: f64, max: f64, count: usize) -> Vec<f64> {
        match self {
            ScaleType::Linear => {
                let step = (max - min) / count as f64;
                (0..=count).map(|i| min + i as f64 * step).collect()
            }
            ScaleType::Time => {
                // Time intervals in seconds
                let intervals = [
                    1.0,           // 1 second
                    5.0,           // 5 seconds
                    10.0,          // 10 seconds
                    30.0,          // 30 seconds
                    60.0,          // 1 minute
                    300.0,         // 5 minutes
                    600.0,         // 10 minutes
                    1800.0,        // 30 minutes
                    3600.0,        // 1 hour
                    7200.0,        // 2 hours
                    21600.0,       // 6 hours
                    43200.0,       // 12 hours
                    86400.0,       // 1 day
                    172800.0,      // 2 days
                    604800.0,      // 1 week
                    2592000.0,     // 30 days
                    7776000.0,     // 90 days
                    31536000.0,    // 1 year
                ];

                let range = max - min;
                let target_interval = range / count as f64;

                // Find best interval
                let interval = intervals
                    .iter()
                    .copied()
                    .find(|&i| i >= target_interval)
                    .unwrap_or(intervals[intervals.len() - 1]);

                // Generate ticks aligned to interval
                let first_tick = (min / interval).ceil() * interval;
                let mut ticks = Vec::new();
                let mut tick = first_tick;
                while tick <= max {
                    ticks.push(tick);
                    tick += interval;
                }
                ticks
            }
            ScaleType::Log => {
                if min <= 0.0 || max <= 0.0 {
                    return vec![];
                }
                let log_min = min.log10().floor() as i32;
                let log_max = max.log10().ceil() as i32;
                (log_min..=log_max)
                    .map(|exp| 10.0_f64.powi(exp))
                    .filter(|&v| v >= min && v <= max)
                    .collect()
            }
            ScaleType::SymLog => {
                // For symlog, generate ticks including negative, zero, and positive
                let mut ticks = Vec::new();

                // Add negative ticks
                if min < 0.0 {
                    let neg_max = min.abs();
                    let log_max = neg_max.log10().ceil() as i32;
                    for exp in (0..=log_max).rev() {
                        let val = -10.0_f64.powi(exp);
                        if val >= min {
                            ticks.push(val);
                        }
                    }
                }

                // Add zero if in range
                if min <= 0.0 && max >= 0.0 {
                    ticks.push(0.0);
                }

                // Add positive ticks
                if max > 0.0 {
                    let log_max = max.log10().ceil() as i32;
                    for exp in 0..=log_max {
                        let val = 10.0_f64.powi(exp);
                        if val <= max && val >= min {
                            ticks.push(val);
                        }
                    }
                }

                ticks
            }
        }
    }

    /// Format a tick label for this scale type
    pub fn format_tick(&self, value: f64) -> String {
        match self {
            ScaleType::Linear => format!("{:.1}", value),
            ScaleType::Time => {
                // Format Unix timestamp as human-readable date
                let secs = value as i64;
                let days_since_epoch = secs / 86400;

                // Simple year/month/day calculation
                let mut year: i64 = 1970;
                let mut days_left = days_since_epoch;

                // Advance years
                while days_left >= 365 {
                    let days_in_year: i64 = if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 366 } else { 365 };
                    if days_left < days_in_year {
                        break;
                    }
                    days_left -= days_in_year;
                    year += 1;
                }

                // Days in each month
                let is_leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
                let days_in_months: [i64; 12] = [31, if is_leap { 29 } else { 28 }, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

                let mut month: i64 = 1;
                for &days in &days_in_months {
                    if days_left < days {
                        break;
                    }
                    days_left -= days;
                    month += 1;
                }

                let day = days_left + 1;

                // Show as M/D format
                format!("{}/{}", month, day)
            }
            ScaleType::Log => {
                if value > 0.0 {
                    let exp = value.log10().round() as i32;
                    if (10.0_f64.powi(exp) - value).abs() < 1e-10 {
                        format!("10^{}", exp)
                    } else {
                        format!("{:.1}", value)
                    }
                } else {
                    format!("{:.1}", value)
                }
            }
            ScaleType::SymLog => {
                if value == 0.0 {
                    "0".to_string()
                } else if value.abs() >= 1.0 {
                    let exp = value.abs().log10().round() as i32;
                    if (10.0_f64.powi(exp) - value.abs()).abs() < 1e-10 {
                        if value < 0.0 {
                            format!("-10^{}", exp)
                        } else {
                            format!("10^{}", exp)
                        }
                    } else {
                        format!("{:.1}", value)
                    }
                } else {
                    format!("{:.2}", value)
                }
            }
        }
    }
}

/// Legend position options
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum LegendPosition {
    #[default]
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    None, // Hidden
}
