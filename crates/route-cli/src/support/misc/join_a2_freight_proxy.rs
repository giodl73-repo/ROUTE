//! Helper `join_a2_freight_proxy`.
#[allow(unused_imports)]
use crate::*;

/// Estimate A2 freight value from representative HPMS daily truck crossings.
/// Uses p90 AADT when available, then mean AADT as the secondary A2 path.
pub(crate) fn join_a2_freight_proxy(attrs: &mut route_network::CorridorAttributes, _corridor_miles: f64) {
    if attrs.annual_freight_value_b.is_some() {
        return;
    }
    let Some(aadt) = attrs.p90_aadt.or(attrs.mean_aadt) else {
        return;
    };
    let truck_pct = attrs.mean_pct_truck.unwrap_or(0.084) as f64;
    let truck_aadt = aadt * truck_pct;
    let freight_b = truck_aadt * 365.0 * 16.0 * 1_000.0 / 1_000_000_000.0;
    attrs.annual_freight_value_b = Some(freight_b);
    attrs.freight_value_is_hpms_proxy = true;
}

