//! Helper `epoch_millis_year`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn epoch_millis_year(millis: i64) -> Option<u16> {
    epoch_millis_ymd(millis).and_then(|(year, _, _)| u16::try_from(year).ok())
}

