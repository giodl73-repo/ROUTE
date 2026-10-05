//! Helper `merge_hpms_state_records`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn merge_hpms_state_records(
    mut existing: Vec<route_data::HpmsRecord>,
    fetched: Vec<route_data::HpmsRecord>,
    states: &std::collections::BTreeSet<String>,
) -> Vec<route_data::HpmsRecord> {
    existing.retain(|row| !states.contains(&row.state.to_ascii_uppercase()));
    existing.extend(fetched);
    existing.sort_by(|a, b| {
        a.state
            .cmp(&b.state)
            .then_with(|| a.route_id.cmp(&b.route_id))
            .then_with(|| a.aadt.cmp(&b.aadt))
    });
    existing
}
