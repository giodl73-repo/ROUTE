//! Helper `percentile_nearest`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn percentile_nearest(sorted_values: &[f64], p: f64) -> Option<f64> {
    if sorted_values.is_empty() {
        return None;
    }
    let p = p.clamp(0.0, 1.0);
    let idx = ((sorted_values.len() - 1) as f64 * p).round() as usize;
    sorted_values.get(idx).copied()
}

