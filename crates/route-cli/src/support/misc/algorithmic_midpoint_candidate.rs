//! Helper `algorithmic_midpoint_candidate`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn algorithmic_midpoint_candidate(
    gap: &RecurringStopGap,
    from: &route_map::BeckStopCatalogRow,
    to: &route_map::BeckStopCatalogRow,
    route_set: &std::collections::BTreeSet<String>,
) -> StopSlaCandidateScore {
    let midpoint_gap = gap.miles / 2.0;
    let midpoint_lat = (from.lat + to.lat) / 2.0;
    let midpoint_lon = midpoint_lon(from.lon, to.lon);
    let route_refs = if route_set.is_empty() {
        gap.route_path.clone()
    } else {
        route_set.iter().cloned().collect::<Vec<_>>().join(";")
    };
    StopSlaCandidateScore {
        stop_id: format!("DRAFT-MID-{}-{}", from.id, to.id),
        name: format!("{} / {} midpoint", from.label, to.label),
        lat: midpoint_lat,
        lon: midpoint_lon,
        requested_class: "S4?".to_string(),
        route_refs,
        evidence_status: "draft-algorithmic-midpoint".to_string(),
        source_type: "algorithmic-midpoint".to_string(),
        basis: "computed midpoint for spacing only; choose nearest real interchange/service city"
            .to_string(),
        spacing_gain_miles: gap.miles - midpoint_gap,
        largest_resulting_gap_miles: midpoint_gap,
        distance_from_segment_miles: 0.0,
        intersection_route_count: route_set.len().max(1),
        score: gap.miles - midpoint_gap,
    }
}

