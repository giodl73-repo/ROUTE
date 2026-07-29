//! Helper `t2_parent_contact_validation_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_parent_contact_validation_rows(
    held_rows: &[T2HeldContactActionRow],
    witness_rows: &[TierContactWitnessInputRow],
) -> Vec<T2ParentContactValidationRow> {
    let witness_by_route = witness_rows
        .iter()
        .map(|row| (canonical_route_key(&row.route), row))
        .collect::<std::collections::HashMap<_, _>>();

    let mut rows = held_rows
        .iter()
        .filter(|row| row.held_action_type == "parent-contact-validation")
        .map(|row| {
            let witness = witness_by_route.get(&canonical_route_key(&row.route));
            let observed_dual_contacts = witness
                .map(|witness| witness.observed_dual_contacts)
                .unwrap_or_default();
            let parent_trunks = witness
                .map(|witness| witness.observed_parent_trunks.clone())
                .unwrap_or_default();
            let (validation_action, required_evidence, next_artifact, optimizer_effect) =
                if observed_dual_contacts > 0 {
                    (
                        "accept-parent-contact",
                        "dual-route parent contact observed",
                        "data/tier-candidate-columns.csv",
                        "eligible for parent-region review",
                    )
                } else {
                    (
                        "prove-parent-contact-or-demote",
                        "dual-route contact to named parent trunk",
                        "data/tier-contact-witnesses.csv",
                        "blocked from T2 regionalizer until parent contact exists",
                    )
                };
            T2ParentContactValidationRow {
                route: row.route.clone(),
                parent_trunks,
                observed_dual_contacts,
                validation_action: validation_action.to_string(),
                required_evidence: required_evidence.to_string(),
                next_artifact: next_artifact.to_string(),
                optimizer_effect: optimizer_effect.to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    if rows.is_empty() {
        rows.push(T2ParentContactValidationRow {
            route: "__all_t2_parent_contacts__".to_string(),
            parent_trunks: String::new(),
            observed_dual_contacts: 0,
            validation_action: "parent-contact-clear".to_string(),
            required_evidence: "no parent-contact validation blockers remain".to_string(),
            next_artifact: "data/tier-candidate-columns.csv".to_string(),
            optimizer_effect: "parent-contact validation lane is clear".to_string(),
            validation_status: "pass".to_string(),
        });
    }
    rows
}

