//! Helper `pct_under`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pct_under(d: &route_sim::TransitDistribution, threshold_h: f64) -> f64 {
    // We only have percentile snapshots; approximate from distribution shape
    if threshold_h >= d.p99_hours {
        return 99.0;
    }
    if threshold_h >= d.p95_hours {
        return 95.0;
    }
    if threshold_h >= d.p90_hours {
        return 90.0;
    }
    if threshold_h >= d.p75_hours {
        return 75.0;
    }
    if threshold_h >= d.p50_hours {
        return 50.0;
    }
    0.0
}

