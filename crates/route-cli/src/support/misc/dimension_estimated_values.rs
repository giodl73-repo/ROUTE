//! Helper `dimension_estimated_values`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn dimension_estimated_values(scores: &route_score::DimensionScores) -> [bool; 16] {
    [
        scores.a1.estimated,
        scores.a2.estimated,
        scores.a3.estimated,
        scores.a4.estimated,
        scores.a5.estimated,
        scores.b1.estimated,
        scores.b2.estimated,
        scores.b3.estimated,
        scores.b4.estimated,
        scores.c1.estimated,
        scores.c2.estimated,
        scores.c3.estimated,
        scores.c4.estimated,
        scores.d1.estimated,
        scores.d2.estimated,
        scores.d3.estimated,
    ]
}

