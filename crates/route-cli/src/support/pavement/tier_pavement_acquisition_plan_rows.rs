//! Helper `tier_pavement_acquisition_plan_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_pavement_acquisition_plan_rows(
    gap_rows: &[TierPavementSourceGapRow],
) -> Vec<TierPavementAcquisitionPlanRow> {
    let mut builders = std::collections::BTreeMap::<String, TierPavementAcquisitionBuilder>::new();

    for row in gap_rows {
        let states = row
            .affected_states
            .split(';')
            .map(str::trim)
            .filter(|state| !state.is_empty())
            .collect::<Vec<_>>();
        let member_share = if states.is_empty() {
            row.blocker_count
        } else {
            row.blocker_count.div_ceil(states.len())
        };
        for state in states {
            let builder = builders.entry(state.to_string()).or_insert_with(|| {
                TierPavementAcquisitionBuilder {
                    state: state.to_string(),
                    ..Default::default()
                }
            });
            builder.tiers.insert(row.tier.clone());
            builder.routes.insert(row.route.clone());
            builder.bundles.insert(row.segment_bundle_id.clone());
            builder.blocked_member_count += member_share;
        }
    }

    builders
        .into_values()
        .map(|builder| {
            let affected_routes = join_string_set(&builder.routes);
            let affected_bundles = join_string_set(&builder.bundles);
            let tier = join_string_set(&builder.tiers);
            let (source_priority, acquisition_action) =
                pavement_acquisition_action(builder.routes.len(), builder.blocked_member_count);
            TierPavementAcquisitionPlanRow {
                state: builder.state,
                tier,
                source_family: "HPMS IRI plus state pavement condition feed".to_string(),
                route_count: builder.routes.len(),
                affected_routes,
                bundle_count: builder.bundles.len(),
                affected_bundles,
                blocked_member_count: builder.blocked_member_count,
                source_priority: source_priority.to_string(),
                acquisition_action: acquisition_action.to_string(),
                required_fields:
                    "route id; segment geometry or linear reference; IRI or condition score; observation year; source owner"
                        .to_string(),
                next_artifact: "data/tier-pavement-docket.csv".to_string(),
                optimizer_effect:
                    "populate member pavement evidence so held T2 bundles can pass or become repair rows"
                        .to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect()
}

