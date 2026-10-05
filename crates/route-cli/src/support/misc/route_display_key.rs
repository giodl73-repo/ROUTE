//! Helper `route_display_key`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn route_display_key(route: &str) -> String {
    route.trim().replace('-', "")
}
