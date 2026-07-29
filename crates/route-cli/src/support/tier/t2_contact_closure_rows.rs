//! Helper `t2_contact_closure_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_contact_closure_rows(
    closure_rows: &[T2BlockerClosureRow],
    witness_rows: &[TierContactWitnessInputRow],
) -> Vec<T2ContactClosureRow> {
    let witness_by_route = witness_rows
        .iter()
        .map(|row| (canonical_route_key(&row.route), row))
        .collect::<std::collections::HashMap<_, _>>();

    let mut rows = closure_rows
        .iter()
        .filter(|row| {
            matches!(
                row.blocker_class.as_str(),
                "parent-contact-repair" | "relief-contact-repair" | "terminal-contact-repair"
            )
        })
        .map(|row| {
            let witness = witness_by_route.get(&canonical_route_key(&row.route));
            let observed_t1_node_count = witness
                .map(|witness| witness.observed_t1_node_count)
                .unwrap_or_default();
            let observed_dual_contacts = witness
                .map(|witness| witness.observed_dual_contacts)
                .unwrap_or_default();
            let observed_parent_trunks = witness
                .map(|witness| witness.observed_parent_trunks.clone())
                .unwrap_or_default();
            let has_contact = match row.blocker_class.as_str() {
                "parent-contact-repair" => observed_dual_contacts > 0,
                "relief-contact-repair" => observed_t1_node_count > 0 || observed_dual_contacts > 0,
                "terminal-contact-repair" => {
                    observed_t1_node_count > 0 || observed_dual_contacts > 0
                }
                _ => false,
            };
            let (contact_action, disposition, required_evidence, next_artifact, optimizer_effect) =
                if has_contact {
                    (
                        "accept-observed-contact",
                        "candidate-review",
                        "observed T1/T2 contact",
                        "data/tier-candidate-columns.csv",
                        "eligible for T2 candidate-column review",
                    )
                } else {
                    (
                        "demote-unless-contact-added",
                        "lower-tier-pressure",
                        "source-backed T1/T2 contact",
                        "data/lower-tier-pressure-witnesses.csv",
                        "kept out of T2 until contact evidence exists",
                    )
                };

            T2ContactClosureRow {
                route: row.route.clone(),
                blocker_class: row.blocker_class.clone(),
                observed_t1_node_count,
                observed_dual_contacts,
                observed_parent_trunks,
                contact_action: contact_action.to_string(),
                disposition: disposition.to_string(),
                required_evidence: required_evidence.to_string(),
                next_artifact: next_artifact.to_string(),
                optimizer_effect: optimizer_effect.to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    if rows.is_empty() {
        rows.push(T2ContactClosureRow {
            route: "__all_t2_contact_closures__".to_string(),
            blocker_class: "contact-closure-clear".to_string(),
            observed_t1_node_count: 0,
            observed_dual_contacts: 0,
            observed_parent_trunks: String::new(),
            contact_action: "contact-closure-clear".to_string(),
            disposition: "clear".to_string(),
            required_evidence: "no contact-closure blockers remain".to_string(),
            next_artifact: "data/tier-candidate-columns.csv".to_string(),
            optimizer_effect: "contact closure lane is clear".to_string(),
            validation_status: "pass".to_string(),
        });
    }
    rows
}

