//! Helper `tier_for_score`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_for_score(score: f64) -> &'static str {
    route_network::RouteTier::from_score(score).as_str()
}

