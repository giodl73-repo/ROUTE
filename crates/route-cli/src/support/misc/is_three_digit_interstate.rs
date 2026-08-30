//! Helper `is_three_digit_interstate`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn is_three_digit_interstate(route: &str) -> bool {
    canonical_route_key(route)
        .strip_prefix('I')
        .and_then(|number| number.parse::<u16>().ok())
        .map(|number| number >= 100)
        .unwrap_or_default()
}
