//! Helper `t2_route_family_split_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_route_family_split_gate_failures(rows: &[T2RouteFamilySplitRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.len() == 1 && rows[0].route == "__all_t2_route_family_splits__" {
        let row = &rows[0];
        if row.family_action != "route-family-split-clear" || row.validation_status != "pass" {
            failures
                .push("route-family split clearance row has incomplete clear status".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.family_action.trim().is_empty()
            || row.disposition.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!("{} has incomplete route-family split", row.route));
        }
        if !row.qualification_effects.trim().is_empty()
            && !row.optimizer_effect.contains("qualification")
        {
            failures.push(format!(
                "{} route-family split drops qualification effects",
                row.route
            ));
        }
    }
    failures
}
