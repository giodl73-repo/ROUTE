//! Extracted helper `t2_route_family_split_rows` from main.
use super::*;

pub(crate) fn t2_route_family_split_rows(
    closure_rows: &[T2BlockerClosureRow],
    service_diagnostic_rows: &[T2ServiceDiagnosticQueueRow],
    bundle_rows: &[NationalSegmentBundleRow],
    exception_rows: &[EndpointExceptionRow],
) -> Vec<T2RouteFamilySplitRow> {
    let mut emitted_routes = std::collections::BTreeSet::<String>::new();
    let mut rows = closure_rows
        .iter()
        .filter(|row| row.blocker_class == "route-family-split")
        .map(|row| {
            emitted_routes.insert(canonical_route_key(&row.route));
            let exception = endpoint_exceptions_for_route(exception_rows, &row.route, "T2")
                .into_iter()
                .next();
            let endpoint_role = exception
                .map(|exception| exception.endpoint_role.clone())
                .unwrap_or_default();
            let exception_type = exception
                .map(|exception| exception.exception_type.clone())
                .unwrap_or_default();
            let source_artifact = exception
                .map(|exception| exception.artifact.clone())
                .unwrap_or_else(|| row.next_artifact.clone());
            let (family_action, disposition, required_evidence, next_artifact, optimizer_effect) =
                t2_route_family_split_decision(exception);

            T2RouteFamilySplitRow {
                route: row.route.clone(),
                endpoint_name: exception
                    .map(|exception| exception.endpoint_name.clone())
                    .unwrap_or_default(),
                endpoint_role,
                exception_type,
                source_artifact,
                family_action: family_action.to_string(),
                disposition: disposition.to_string(),
                required_evidence: required_evidence.to_string(),
                next_artifact: next_artifact.to_string(),
                qualification_effects: row.qualification_effects.clone(),
                optimizer_effect: route_family_split_optimizer_effect(
                    optimizer_effect,
                    &row.qualification_effects,
                ),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.extend(
        service_diagnostic_rows
            .iter()
            .filter(|row| row.diagnostic_status == "route-family-diagnostic-split-needed")
            .filter(|row| {
                let route_key = canonical_route_key(&row.route);
                if emitted_routes.contains(&route_key) {
                    return false;
                }
                emitted_routes.insert(route_key);
                true
            })
            .map(|row| T2RouteFamilySplitRow {
                route: row.route.clone(),
                endpoint_name: String::new(),
                endpoint_role: "service_diagnostic_route_family".to_string(),
                exception_type: row.diagnostic_status.clone(),
                source_artifact: "data/t2-service-diagnostic-queue.csv".to_string(),
                family_action: "split-numbered-service-family".to_string(),
                disposition: "blocked".to_string(),
                required_evidence:
                    "represented segment family plus Beck service diagnostic for each selected segment"
                        .to_string(),
                next_artifact: row.next_artifact.clone(),
                qualification_effects: row.qualification_effects.clone(),
                optimizer_effect: route_family_split_optimizer_effect(
                    "blocked from national T2 service rendering until multi-state route label is split",
                    &row.qualification_effects,
                ),
                validation_status: "review".to_string(),
            }),
    );
    let mut service_bundle_count_by_route =
        std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();
    for row in service_diagnostic_rows {
        service_bundle_count_by_route
            .entry(canonical_route_key(&row.route))
            .or_default()
            .insert(row.segment_bundle_id.clone());
    }
    rows.extend(
        service_bundle_count_by_route
            .into_iter()
            .filter(|(route, bundles)| bundles.len() > 1 && is_three_digit_interstate(route))
            .filter(|(route, _)| {
                if emitted_routes.contains(route) {
                    return false;
                }
                emitted_routes.insert(route.clone());
                true
            })
            .map(|(route, bundles)| T2RouteFamilySplitRow {
                route,
                endpoint_name: String::new(),
                endpoint_role: "service_diagnostic_route_family".to_string(),
                exception_type: "route-family-segment-bundles-present".to_string(),
                source_artifact: "data/t2-service-diagnostic-queue.csv".to_string(),
                family_action: "split-numbered-service-family".to_string(),
                disposition: "segment-family-split-complete".to_string(),
                required_evidence: format!(
                    "{} represented segment bundles emitted",
                    bundles.len()
                ),
                next_artifact: "data/beck-t2-diagnostics.csv".to_string(),
                qualification_effects: "qualification_gate_policy=stop-first".to_string(),
                optimizer_effect:
                    "keeps state-scoped T2 segment families stable while Beck diagnostics are authored"
                        .to_string(),
                validation_status: "pass".to_string(),
            }),
    );
    let mut scoped_bundle_count_by_route =
        std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();
    for bundle in bundle_rows {
        if !bundle.bundle_aliases.contains("route-family-scope:") {
            continue;
        }
        for route in semicolon_values(&bundle.route_labels) {
            scoped_bundle_count_by_route
                .entry(canonical_route_key(&route))
                .or_default()
                .insert(bundle.segment_bundle_id.clone());
        }
    }
    rows.extend(
        scoped_bundle_count_by_route
            .into_iter()
            .filter(|(route, bundles)| bundles.len() > 1 && is_three_digit_interstate(route))
            .filter(|(route, _)| {
                if emitted_routes.contains(route) {
                    return false;
                }
                emitted_routes.insert(route.clone());
                true
            })
            .map(|(route, bundles)| T2RouteFamilySplitRow {
                route,
                endpoint_name: String::new(),
                endpoint_role: "service_diagnostic_route_family".to_string(),
                exception_type: "route-family-segment-bundles-present".to_string(),
                source_artifact: "data/national-segment-bundles.csv".to_string(),
                family_action: "split-numbered-service-family".to_string(),
                disposition: "segment-family-split-complete".to_string(),
                required_evidence: format!(
                    "{} represented segment bundles emitted",
                    bundles.len()
                ),
                next_artifact: "data/beck-t2-diagnostics.csv".to_string(),
                qualification_effects: "qualification_gate_policy=stop-first".to_string(),
                optimizer_effect:
                    "keeps state-scoped T2 segment families stable while Beck diagnostics are authored"
                        .to_string(),
                validation_status: "pass".to_string(),
            }),
    );
    if rows.is_empty() {
        rows.push(T2RouteFamilySplitRow {
            route: "__all_t2_route_family_splits__".to_string(),
            endpoint_name: String::new(),
            endpoint_role: String::new(),
            exception_type: String::new(),
            source_artifact: "data/t2-blocker-closure.csv".to_string(),
            family_action: "route-family-split-clear".to_string(),
            disposition: "clear".to_string(),
            required_evidence: "no route-family split blockers remain".to_string(),
            next_artifact: "data/tier-candidate-columns.csv".to_string(),
            qualification_effects: "qualification_gate_policy=stop-first".to_string(),
            optimizer_effect: "route-family split lane is clear".to_string(),
            validation_status: "pass".to_string(),
        });
    }
    rows
}

