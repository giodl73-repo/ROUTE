//! Helper `t2_graph_contact_repair_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_graph_contact_repair_contract(
    row: &T2HeldContactActionRow,
) -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
) {
    if row
        .exception_type
        .trim()
        .eq_ignore_ascii_case("missing_graph_geometry")
    {
        (
            "route-family-split",
            "split-numbered-route-family-before-tier-decision",
            "identify represented segment and its T1/T2 contacts",
            "data/tier-node-exceptions.csv",
            "blocked until route family is disambiguated",
        )
    } else {
        (
            "graph-contact-repair",
            "repair-route-geometry-or-demote",
            "prove at least one T1/T2 graph contact or demotion basis",
            "data/tier-contact-witnesses.csv",
            "blocked until graph contact evidence exists",
        )
    }
}

