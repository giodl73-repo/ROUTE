//! Helper `t3_stop_state_scope`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_stop_state_scope(stops: &[&StopCandidateRow]) -> String {
    stops
        .iter()
        .flat_map(|stop| stop.state.split(['/', ';', ',']))
        .map(|state| state.trim().to_ascii_uppercase())
        .filter(|state| !state.is_empty())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
        .join(";")
}

