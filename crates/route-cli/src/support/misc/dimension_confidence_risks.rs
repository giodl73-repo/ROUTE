//! Helper `dimension_confidence_risks`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn dimension_confidence_risks(scores: &[f64; 16], confidences: &[f32; 16]) -> [f64; 16] {
    let mut risks = [0.0; 16];
    for d in 0..16 {
        risks[d] = scores[d] * (1.0 - confidences[d].clamp(0.0, 1.0) as f64);
    }
    risks
}
