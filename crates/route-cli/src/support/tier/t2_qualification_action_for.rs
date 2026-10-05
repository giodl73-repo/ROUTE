//! Helper `t2_qualification_action_for`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_qualification_action_for(
    service_action: &str,
    qualification_basis: &str,
) -> Option<route_map::BeckT2QualificationActionRow> {
    route_map::beck_t2_qualification_actions()
        .into_iter()
        .find(|action| {
            action.service_action == service_action
                && action
                    .covered_bases
                    .iter()
                    .any(|basis| *basis == qualification_basis)
        })
}
