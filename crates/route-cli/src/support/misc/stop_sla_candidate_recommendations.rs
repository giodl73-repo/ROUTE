//! Helper `stop_sla_candidate_recommendations`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn stop_sla_candidate_recommendations(
    rows: &[StopSlaRow],
    stop_rows: &[StopCandidateRow],
    city_rows: &[CitySeedRow],
    target_gap: f64,
    top: usize,
) -> Vec<StopSlaCandidateRecommendation> {
    let catalog = route_map::beck_stop_catalog()
        .into_iter()
        .map(|stop| (stop.id.to_string(), stop))
        .collect::<std::collections::HashMap<_, _>>();
    recurring_stop_gaps(rows)
        .into_iter()
        .filter(|gap| gap.miles > target_gap)
        .take(top)
        .map(|gap| {
            let candidates = score_stop_candidates_for_gap(&gap, stop_rows, city_rows, &catalog);
            StopSlaCandidateRecommendation { gap, candidates }
        })
        .collect()
}

