//! Helper `t1_sla_candidate_pair_score`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_sla_candidate_pair_score(row: &T1SlaCandidateUniverseRow) -> f64 {
    row.market_score
        + row.conversion_score
        + row.coverage_score
        + row.reuse_score
        + row.resilience_score
        + row.evidence_score
        - row.budget_penalty
}

