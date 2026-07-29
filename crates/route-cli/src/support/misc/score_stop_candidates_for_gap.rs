//! Helper `score_stop_candidates_for_gap` (support::misc).
#[allow(unused_imports)]
use crate::*;

pub(crate) fn score_stop_candidates_for_gap(
    gap: &RecurringStopGap,
    stop_rows: &[StopCandidateRow],
    city_rows: &[CitySeedRow],
    catalog: &std::collections::HashMap<String, route_map::BeckStopCatalogRow>,
) -> Vec<StopSlaCandidateScore> {
    let Some((from_id, to_id)) = gap.segment_id.split_once("->") else {
        return Vec::new();
    };
    let (Some(from), Some(to)) = (catalog.get(from_id), catalog.get(to_id)) else {
        return Vec::new();
    };
    let route_set = gap
        .route_path
        .split(';')
        .map(normalise_designation)
        .filter(|route| !route.is_empty())
        .collect::<std::collections::BTreeSet<_>>();

    let mut scores = stop_rows
        .iter()
        .filter_map(|row| {
            let lat = parse_coord(&row.lat)?;
            let lon = parse_coord(&row.lon)?;
            let candidate_routes = stop_candidate_routes(row);
            if !route_set.is_empty()
                && !candidate_routes
                    .iter()
                    .any(|route| route_set.contains(route))
            {
                return None;
            }
            let along = projection_fraction(from.lat, from.lon, to.lat, to.lon, lat, lon);
            if !(-0.12..=1.12).contains(&along) {
                return None;
            }
            let distance_from_segment =
                distance_to_geo_segment_miles(from.lat, from.lon, to.lat, to.lon, lat, lon);
            if distance_from_segment > 90.0 {
                return None;
            }
            let first_gap = geo_distance_miles(from.lat, from.lon, lat, lon);
            let second_gap = geo_distance_miles(lat, lon, to.lat, to.lon);
            if first_gap < 45.0 || second_gap < 45.0 {
                return None;
            }
            let largest_resulting_gap = first_gap.max(second_gap);
            if largest_resulting_gap + 1.0 >= gap.miles {
                return None;
            }
            let spacing_gain = gap.miles - largest_resulting_gap;
            let intersection_route_count = candidate_routes.len();
            let intersection_bonus = intersection_route_count.saturating_sub(1) as f64 * 12.0;
            let class_bonus = match row.requested_class.trim().to_ascii_uppercase().as_str() {
                "S1" => 30.0,
                "S2" => 24.0,
                "S3" => 18.0,
                "S4" => 10.0,
                _ => 4.0,
            };
            let route_match_bonus = candidate_routes
                .iter()
                .filter(|route| route_set.contains(*route))
                .count() as f64
                * 10.0;
            let score = spacing_gain + class_bonus + intersection_bonus + route_match_bonus
                - distance_from_segment * 0.6;
            Some(StopSlaCandidateScore {
                stop_id: row.stop_id.clone(),
                name: row.name.clone(),
                lat,
                lon,
                requested_class: row.requested_class.clone(),
                route_refs: row.route_refs.clone(),
                evidence_status: row.evidence_status.clone(),
                source_type: "stop-ledger".to_string(),
                basis: format!(
                    "ledger candidate; route matches {}; {} route contact(s)",
                    gap.route_path, intersection_route_count
                ),
                spacing_gain_miles: spacing_gain,
                largest_resulting_gap_miles: largest_resulting_gap,
                distance_from_segment_miles: distance_from_segment,
                intersection_route_count,
                score,
            })
        })
        .collect::<Vec<_>>();
    scores.extend(city_rows.iter().filter_map(|city| {
        let along = projection_fraction(from.lat, from.lon, to.lat, to.lon, city.lat, city.lon);
        if !(-0.08..=1.08).contains(&along) {
            return None;
        }
        let distance_from_segment =
            distance_to_geo_segment_miles(from.lat, from.lon, to.lat, to.lon, city.lat, city.lon);
        if distance_from_segment > 75.0 {
            return None;
        }
        let first_gap = geo_distance_miles(from.lat, from.lon, city.lat, city.lon);
        let second_gap = geo_distance_miles(city.lat, city.lon, to.lat, to.lon);
        if first_gap < 45.0 || second_gap < 45.0 {
            return None;
        }
        let largest_resulting_gap = first_gap.max(second_gap);
        if largest_resulting_gap + 1.0 >= gap.miles {
            return None;
        }
        let spacing_gain = gap.miles - largest_resulting_gap;
        let midpoint_balance_bonus = (1.0 - (along - 0.5).abs() * 2.0).max(0.0) * 18.0;
        let score = spacing_gain + midpoint_balance_bonus - distance_from_segment * 0.8;
        Some(StopSlaCandidateScore {
            stop_id: format!("DRAFT-{}", city.abbr),
            name: city.name.clone(),
            lat: city.lat,
            lon: city.lon,
            requested_class: "S4?".to_string(),
            route_refs: gap.route_path.clone(),
            evidence_status: "draft-city-seed".to_string(),
            source_type: "city-seed".to_string(),
            basis: "city seed near segment; requires stop-ledger validation".to_string(),
            spacing_gain_miles: spacing_gain,
            largest_resulting_gap_miles: largest_resulting_gap,
            distance_from_segment_miles: distance_from_segment,
            intersection_route_count: route_set.len().max(1),
            score,
        })
    }));
    if scores.is_empty() {
        scores.push(algorithmic_midpoint_candidate(gap, from, to, &route_set));
    }
    scores.sort_by(|a, b| {
        b.score
            .total_cmp(&a.score)
            .then_with(|| a.name.cmp(&b.name))
    });
    scores
}

