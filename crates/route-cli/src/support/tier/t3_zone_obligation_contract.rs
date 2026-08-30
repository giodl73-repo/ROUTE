//! Helper `t3_zone_obligation_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_obligation_contract(
    obligation_class: &str,
) -> (&'static str, u8, &'static str, &'static str) {
    match obligation_class {
        "regional-upgrade-review" => (
            "prove T2 contact and regional service value before upgrade",
            24,
            "data/t2-bubble-up-review.csv",
            "keeps lower-tier upgrade pressure attached to zone maps before any T2 reopening",
        ),
        "terminal-local-access" => (
            "select T4 terminal/local access chain inside the zone",
            1,
            "data/t4-terminal-access-columns.csv",
            "turns local pressure into terminal access columns instead of national promotion",
        ),
        _ => (
            "select T3 feeder/contact chain inside the zone",
            6,
            "data/t3-zone-route-columns.csv",
            "turns lower-tier pressure into regional feeder obligations for zone maps",
        ),
    }
}
