//! Helper `route_date`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn route_date() -> String {
    std::env::var("ROUTE_DATE").unwrap_or_else(|_| "2026-05-06".to_string())
}

