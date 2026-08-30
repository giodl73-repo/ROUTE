//! Helper `terminal_district_seed`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn terminal_district_seed(terminal_obligation: &str) -> String {
    terminal_obligation
        .split_once(": ")
        .map(|(_, seed)| seed.to_string())
        .unwrap_or_else(|| terminal_obligation.to_string())
}
