//! Helper `t1_diamond_validation_missing_sites`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_diamond_validation_missing_sites(rows: &[T1DiamondValidationRow]) -> Vec<String> {
    let present: std::collections::HashSet<_> =
        rows.iter().map(|row| row.site_id.as_str()).collect();
    EXPECTED_T1_DIAMOND_SITES
        .iter()
        .filter(|site_id| !present.contains(**site_id))
        .map(|site_id| (*site_id).to_string())
        .collect()
}

