//! Helper `split_claim_tokens`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn split_claim_tokens(claims: &str) -> Vec<&str> {
    claims
        .split(['|', ';', ','])
        .map(str::trim)
        .filter(|claim| !claim.is_empty())
        .collect()
}

