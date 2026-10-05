//! Helper `t2_graph_contact_validation_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_graph_contact_validation_rows(
    closure_rows: &[T2BlockerClosureRow],
    witness_rows: &[TierContactWitnessInputRow],
) -> Vec<T2GraphContactValidationRow> {
    let witness_by_route = witness_rows
        .iter()
        .map(|row| (canonical_route_key(&row.route), row))
        .collect::<std::collections::HashMap<_, _>>();

    let mut rows = closure_rows
        .iter()
        .filter(|row| row.blocker_class == "graph-contact-repair")
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
            let has_contact = observed_t1_node_count > 0 || observed_dual_contacts > 0;
            let (contact_action, disposition, required_evidence, next_artifact, optimizer_effect) =
                if has_contact {
                    (
                        "accept-observed-graph-contact",
                        "candidate-review",
                        "observed T1/T2 graph contact",
                        "data/tier-candidate-columns.csv",
                        "eligible for T2 candidate-column review",
                    )
                } else {
                    (
                        "demote-unless-graph-contact-added",
                        "lower-tier-pressure",
                        "source-backed T1/T2 graph contact",
                        "data/lower-tier-pressure-witnesses.csv",
                        "kept out of T2 until graph contact evidence exists",
                    )
                };

            T2GraphContactValidationRow {
                route: row.route.clone(),
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
        rows.push(T2GraphContactValidationRow {
            route: "__all_t2_graph_contacts__".to_string(),
            observed_t1_node_count: 0,
            observed_dual_contacts: 0,
            observed_parent_trunks: String::new(),
            contact_action: "graph-contact-clear".to_string(),
            disposition: "clear".to_string(),
            required_evidence: "no graph-contact validation blockers remain".to_string(),
            next_artifact: "data/tier-candidate-columns.csv".to_string(),
            optimizer_effect: "graph-contact validation lane is clear".to_string(),
            validation_status: "pass".to_string(),
        });
    }
    rows
}
