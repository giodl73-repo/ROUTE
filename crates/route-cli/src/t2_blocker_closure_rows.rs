//! Extracted helper `t2_blocker_closure_rows` from main.
use super::*;

pub(crate) fn t2_blocker_closure_rows(
    graph_rows: &[T2GraphContactRepairRow],
    parent_rows: &[T2ParentContactValidationRow],
    relief_rows: &[T2ReliefEvidenceRow],
    terminal_rows: &[T2TerminalContactValidationRow],
    bundle_rows: &[NationalSegmentBundleRow],
) -> Vec<T2BlockerClosureRow> {
    let mut rows = Vec::new();
    let registry = route_network::BundleRegistry::new(
        bundle_rows
            .iter()
            .map(segment_bundle_from_national_row)
            .collect(),
    );
    let bundle_effects_by_route = bundle_qualification_effects_by_route(bundle_rows);

    for row in graph_rows
        .iter()
        .filter(|row| !row.route.starts_with("__all_"))
    {
        let blocker_class = if row.repair_class == "route-family-split" {
            "route-family-split"
        } else {
            "graph-contact-repair"
        };
        let (segment_bundle_id, bundle_status, bundle_action) =
            t2_blocker_bundle_fields(&registry, &row.route);
        let qualification_effects = bundle_effects_by_route
            .get(&canonical_route_key(&row.route))
            .cloned()
            .unwrap_or_default();
        rows.push(T2BlockerClosureRow {
            route: row.route.clone(),
            segment_bundle_id,
            bundle_status,
            bundle_action,
            source_surface: "t2-graph-contact-repairs".to_string(),
            blocker_class: blocker_class.to_string(),
            blocker_action: row.repair_action.clone(),
            required_evidence: row.required_evidence.clone(),
            next_artifact: row.next_artifact.clone(),
            optimizer_effect: row.optimizer_effect.clone(),
            qualification_effects,
            closure_status: "open".to_string(),
            validation_status: "review".to_string(),
        });
    }

    for row in parent_rows
        .iter()
        .filter(|row| !row.route.starts_with("__all_"))
    {
        let (segment_bundle_id, bundle_status, bundle_action) =
            t2_blocker_bundle_fields(&registry, &row.route);
        let qualification_effects = bundle_effects_by_route
            .get(&canonical_route_key(&row.route))
            .cloned()
            .unwrap_or_default();
        rows.push(T2BlockerClosureRow {
            route: row.route.clone(),
            segment_bundle_id,
            bundle_status,
            bundle_action,
            source_surface: "t2-parent-contact-validation".to_string(),
            blocker_class: "parent-contact-repair".to_string(),
            blocker_action: row.validation_action.clone(),
            required_evidence: row.required_evidence.clone(),
            next_artifact: row.next_artifact.clone(),
            optimizer_effect: row.optimizer_effect.clone(),
            qualification_effects,
            closure_status: "open".to_string(),
            validation_status: "review".to_string(),
        });
    }

    for row in relief_rows
        .iter()
        .filter(|row| !row.route.starts_with("__all_"))
    {
        let (blocker_class, closure_status) =
            if row.relief_action == "source-observed-relief-review" {
                ("relief-contact-repair", "evidence-observed")
            } else {
                ("relief-evidence-gap", "open")
            };
        let (segment_bundle_id, bundle_status, bundle_action) =
            t2_blocker_bundle_fields(&registry, &row.route);
        let qualification_effects = bundle_effects_by_route
            .get(&canonical_route_key(&row.route))
            .cloned()
            .unwrap_or_default();
        rows.push(T2BlockerClosureRow {
            route: row.route.clone(),
            segment_bundle_id,
            bundle_status,
            bundle_action,
            source_surface: "t2-relief-evidence-docket".to_string(),
            blocker_class: blocker_class.to_string(),
            blocker_action: row.relief_action.clone(),
            required_evidence: row.evidence_basis.clone(),
            next_artifact: row.next_artifact.clone(),
            optimizer_effect: row.optimizer_effect.clone(),
            qualification_effects,
            closure_status: closure_status.to_string(),
            validation_status: "review".to_string(),
        });
    }

    for row in terminal_rows
        .iter()
        .filter(|row| !row.route.starts_with("__all_"))
    {
        let blocker_class = match row.terminal_action.as_str() {
            "prove-terminal-contact-or-demote" => "terminal-contact-repair",
            "prove-terminal-exception-or-demote" => "endpoint-exception-upgrade",
            "accept-terminal-contact" => "terminal-contact-accepted",
            _ => "terminal-review",
        };
        let (segment_bundle_id, bundle_status, bundle_action) =
            t2_blocker_bundle_fields(&registry, &row.route);
        let qualification_effects = bundle_effects_by_route
            .get(&canonical_route_key(&row.route))
            .cloned()
            .unwrap_or_default();
        rows.push(T2BlockerClosureRow {
            route: row.route.clone(),
            segment_bundle_id,
            bundle_status,
            bundle_action,
            source_surface: "t2-terminal-contact-validation".to_string(),
            blocker_class: blocker_class.to_string(),
            blocker_action: row.terminal_action.clone(),
            required_evidence: row.required_evidence.clone(),
            next_artifact: row.next_artifact.clone(),
            optimizer_effect: row.optimizer_effect.clone(),
            qualification_effects,
            closure_status: if row.terminal_action == "accept-terminal-contact" {
                "ready".to_string()
            } else {
                "open".to_string()
            },
            validation_status: "review".to_string(),
        });
    }

    rows.sort_by(|a, b| {
        a.blocker_class
            .cmp(&b.blocker_class)
            .then_with(|| a.route.cmp(&b.route))
    });
    rows
}

