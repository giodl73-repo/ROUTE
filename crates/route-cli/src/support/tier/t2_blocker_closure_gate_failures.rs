//! Helper `t2_blocker_closure_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_blocker_closure_gate_failures(rows: &[T2BlockerClosureRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T2 blocker closure rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.bundle_status.trim().is_empty()
            || row.bundle_action.trim().is_empty()
            || row.source_surface.trim().is_empty()
            || row.blocker_class.trim().is_empty()
            || row.blocker_action.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
            || row.closure_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete blocker closure", row.route));
        }
        if row.bundle_status == "bundle-missing"
            && !matches!(
                row.blocker_class.as_str(),
                "route-family-split"
                    | "relief-contact-repair"
                    | "parent-contact-repair"
                    | "endpoint-exception-upgrade"
            )
        {
            failures.push(format!(
                "{} blocker closure lacks bundle binding for {}",
                row.route, row.blocker_class
            ));
        }
        if !row.qualification_effects.trim().is_empty() && row.segment_bundle_id.trim().is_empty() {
            failures.push(format!(
                "{} carries qualification effects without a segment bundle",
                row.route
            ));
        }
    }
    failures
}

