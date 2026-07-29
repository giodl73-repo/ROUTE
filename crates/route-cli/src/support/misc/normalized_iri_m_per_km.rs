//! Helper `normalized_iri_m_per_km`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn normalized_iri_m_per_km(raw_iri: Option<f32>) -> Option<f32> {
    raw_iri.map(|value| {
        if value > 20.0 {
            value * 0.015_782_8
        } else {
            value
        }
    })
}

