//! Helper `dimension_score_values`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn dimension_score_values(scores: &route_score::DimensionScores) -> [f64; 16] {
    [
        scores.a1.score,
        scores.a2.score,
        scores.a3.score,
        scores.a4.score,
        scores.a5.score,
        scores.b1.score,
        scores.b2.score,
        scores.b3.score,
        scores.b4.score,
        scores.c1.score,
        scores.c2.score,
        scores.c3.score,
        scores.c4.score,
        scores.d1.score,
        scores.d2.score,
        scores.d3.score,
    ]
}

