//! Helper `dimension_confidence_values`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn dimension_confidence_values(scores: &route_score::DimensionScores) -> [f32; 16] {
    [
        scores.a1.confidence,
        scores.a2.confidence,
        scores.a3.confidence,
        scores.a4.confidence,
        scores.a5.confidence,
        scores.b1.confidence,
        scores.b2.confidence,
        scores.b3.confidence,
        scores.b4.confidence,
        scores.c1.confidence,
        scores.c2.confidence,
        scores.c3.confidence,
        scores.c4.confidence,
        scores.d1.confidence,
        scores.d2.confidence,
        scores.d3.confidence,
    ]
}
