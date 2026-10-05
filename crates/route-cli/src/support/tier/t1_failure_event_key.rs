//! Helper `t1_failure_event_key`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t1_failure_event_key(row: &T1FailureEventRow) -> (String, String) {
    (row.site_id.clone(), row.event_id.clone())
}
