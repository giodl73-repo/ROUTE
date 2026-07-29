//! Helper `join_d3_iri_proxy`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn join_d3_iri_proxy(attrs: &mut route_network::CorridorAttributes) {
    // Only apply when NBI data is absent
    if attrs.pct_bridges_poor.is_some() {
        return;
    }
    let Some(iri) = attrs.mean_iri else {
        return;
    };

    let estimated_year = if iri < 50.0 {
        2005.0_f32
    } else if iri < 80.0 {
        1990.0
    } else if iri < 120.0 {
        1975.0
    } else {
        1965.0
    };

    if attrs.mean_year_built.is_none() {
        attrs.mean_year_built = Some(estimated_year);
    }
    let iri_proxy = (iri / 170.0).min(0.30);
    attrs.pct_bridges_poor = Some(iri_proxy);
}

