//! Helper `current_utc_year`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn current_utc_year() -> u16 {
    let millis = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis().min(i64::MAX as u128) as i64)
        .unwrap_or(0);
    epoch_millis_year(millis).unwrap_or(1970)
}
