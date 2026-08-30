//! Helper `join_nbi_to_corridor`.
#[allow(unused_imports)]
use crate::*;

/// Join NBI bridge condition data to a corridor.
pub(crate) fn join_nbi_to_corridor(
    route_id: &str,
    attrs: &mut route_network::CorridorAttributes,
    nbi: &std::collections::HashMap<String, NbiBridgeRecord>,
) {
    if let Some(rec) = nbi.get(route_id) {
        attrs.pct_bridges_poor = Some(rec.pct_bridges_poor);
        attrs.mean_year_built = Some(rec.mean_year_built);
        attrs.bridge_count = rec.bridge_count as usize;
    }
}
