//! Helper `epoch_millis_date`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn epoch_millis_date(millis: i64) -> Option<String> {
    epoch_millis_ymd(millis).map(|(year, month, day)| format!("{year:04}-{month:02}-{day:02}"))
}

