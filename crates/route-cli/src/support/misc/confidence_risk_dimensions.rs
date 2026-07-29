//! Helper `confidence_risk_dimensions`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn confidence_risk_dimensions(scores: &[f64; 16], confidences: &[f32; 16]) -> String {
    let contribution = dimension_confidence_risks(scores, confidences);
    let mut risks: Vec<(&str, f64, f32, f64)> = DIMENSION_CODES
        .iter()
        .zip(scores.iter())
        .zip(confidences.iter())
        .zip(contribution.iter())
        .filter_map(|(((code, score), confidence), risk)| {
            let score = *score;
            let confidence = confidence.clamp(0.0, 1.0);
            if *risk >= 1.0 {
                Some((*code, score, confidence, *risk))
            } else {
                None
            }
        })
        .collect();

    risks.sort_by(|a, b| {
        b.3.total_cmp(&a.3)
            .then_with(|| b.1.total_cmp(&a.1))
            .then_with(|| a.0.cmp(b.0))
    });

    risks
        .into_iter()
        .take(3)
        .map(|(code, score, confidence, _)| format!("{code}:{score:.1}@{confidence:.2}"))
        .collect::<Vec<_>>()
        .join(";")
}

