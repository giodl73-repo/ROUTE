//! Helper `t1_source_health_blockers`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_source_health_blockers(rows: &[T1SourceHealthRow]) -> Vec<&T1SourceHealthRow> {
    rows.iter()
        .filter(|row| t1_source_health_is_blocked(row))
        .collect()
}
