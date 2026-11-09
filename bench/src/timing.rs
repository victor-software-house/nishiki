//! Timing helpers the per-line and per-delta programs share.

use std::hint::black_box;
use std::time::{Duration, Instant};

/// Empty timings measured for the timer's overhead.
const OVERHEAD_SAMPLES: usize = 10_000;

/// The median of many empty timings: what reading the clock costs.
#[must_use]
pub fn timer_overhead() -> Duration {
    let mut samples: Vec<Duration> = (0..OVERHEAD_SAMPLES)
        .map(|_| {
            let start = Instant::now();
            black_box(start.elapsed())
        })
        .collect();
    samples.sort_unstable();
    samples[samples.len() / 2]
}

/// The value at nearest rank `per_mille` / 1000 of sorted `values`.
#[must_use]
pub fn rank(sorted: &[Duration], per_mille: usize) -> Duration {
    let index = (sorted.len() * per_mille).div_ceil(1000).saturating_sub(1);
    sorted.get(index).copied().unwrap_or_default()
}

/// A time in µs, or `<bound` when it is below resolution.
#[must_use]
pub fn micros(time: Duration, resolution: Duration) -> String {
    if time < resolution {
        format!("<{:.3}", resolution.as_secs_f64() * 1e6)
    } else {
        format!("{:.3}", time.as_secs_f64() * 1e6)
    }
}
