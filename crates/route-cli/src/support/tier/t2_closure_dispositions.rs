//! Helper `t2_closure_dispositions`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_closure_dispositions(
    route_family_rows: &[T2RouteFamilySplitRow],
    graph_rows: &[T2GraphContactValidationRow],
    contact_rows: &[T2ContactClosureRow],
    endpoint_rows: &[T2EndpointClosureRow],
    blocker_rows: &[T2BlockerClosureRow],
) -> std::collections::HashMap<String, T2ClosureDisposition> {
    let mut dispositions = std::collections::HashMap::new();
    let bundle_by_route = blocker_rows
        .iter()
        .map(|row| {
            (
                canonical_route_key(&row.route),
                (
                    row.segment_bundle_id.clone(),
                    row.bundle_status.clone(),
                    row.bundle_action.clone(),
                    row.qualification_effects.clone(),
                ),
            )
        })
        .collect::<std::collections::HashMap<_, _>>();

    for row in route_family_rows {
        if row.route.starts_with("__all_") {
            continue;
        }
        let (segment_bundle_id, bundle_status, bundle_action, blocker_qualification_effects) =
            t2_closure_bundle_posture(&bundle_by_route, &row.route);
        let qualification_effects =
            merge_qualification_effects(&row.qualification_effects, &blocker_qualification_effects);
        dispositions.insert(
            canonical_route_key(&row.route),
            T2ClosureDisposition {
                route: row.route.clone(),
                disposition: row.disposition.clone(),
                action: row.family_action.clone(),
                basis: row.required_evidence.clone(),
                segment_bundle_id,
                bundle_status,
                bundle_action,
                qualification_effects,
                source_artifact: "data/t2-route-family-splits.csv".to_string(),
                next_artifact: row.next_artifact.clone(),
            },
        );
    }
    for row in graph_rows {
        if row.route.starts_with("__all_") {
            continue;
        }
        let (segment_bundle_id, bundle_status, bundle_action, qualification_effects) =
            t2_closure_bundle_posture(&bundle_by_route, &row.route);
        dispositions.insert(
            canonical_route_key(&row.route),
            T2ClosureDisposition {
                route: row.route.clone(),
                disposition: row.disposition.clone(),
                action: row.contact_action.clone(),
                basis: row.required_evidence.clone(),
                segment_bundle_id,
                bundle_status,
                bundle_action,
                qualification_effects,
                source_artifact: "data/t2-graph-contact-validation.csv".to_string(),
                next_artifact: row.next_artifact.clone(),
            },
        );
    }
    for row in contact_rows {
        if row.route.starts_with("__all_") {
            continue;
        }
        let (segment_bundle_id, bundle_status, bundle_action, qualification_effects) =
            t2_closure_bundle_posture(&bundle_by_route, &row.route);
        dispositions.insert(
            canonical_route_key(&row.route),
            T2ClosureDisposition {
                route: row.route.clone(),
                disposition: row.disposition.clone(),
                action: row.contact_action.clone(),
                basis: row.required_evidence.clone(),
                segment_bundle_id,
                bundle_status,
                bundle_action,
                qualification_effects,
                source_artifact: "data/t2-contact-closure.csv".to_string(),
                next_artifact: row.next_artifact.clone(),
            },
        );
    }
    for row in endpoint_rows {
        if row.route.starts_with("__all_") {
            continue;
        }
        let (segment_bundle_id, bundle_status, bundle_action, qualification_effects) =
            t2_closure_bundle_posture(&bundle_by_route, &row.route);
        dispositions.insert(
            canonical_route_key(&row.route),
            T2ClosureDisposition {
                route: row.route.clone(),
                disposition: row.disposition.clone(),
                action: row.endpoint_action.clone(),
                basis: row.required_evidence.clone(),
                segment_bundle_id,
                bundle_status,
                bundle_action,
                qualification_effects,
                source_artifact: "data/t2-endpoint-closure.csv".to_string(),
                next_artifact: row.next_artifact.clone(),
            },
        );
    }

    dispositions
}

