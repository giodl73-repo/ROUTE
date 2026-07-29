//! Helper `t2_held_contact_action_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_held_contact_action_contract(
    row: &T2ContactResolutionRow,
) -> (&'static str, &'static str, &'static str, &'static str) {
    match row.resolution_action.as_str() {
        "hold-for-terminal-contact-validation" => (
            "terminal-contact-validation",
            "prove terminal endpoint plus at least one T1/T2 contact chain",
            "data/t2-terminal-contact-validation.csv",
            "retain as T2 only if terminal contact validates; otherwise demote",
        ),
        "hold-for-terminal-exception" => (
            "terminal-exception-review",
            "split route family or validate terminal-worthy endpoint exception",
            "data/t2-terminal-contact-validation.csv",
            "retain only validated terminal segment; demote unsplit local loop behavior",
        ),
        "hold-for-relief-evidence-or-demotion" => (
            "relief-evidence-review",
            "source-backed bottleneck or resilience relief evidence plus T1/T2 contact",
            "data/t2-relief-evidence-docket.csv",
            "retain as relief service only with evidence; otherwise demote",
        ),
        "hold-for-parent-contact-or-demotion" => (
            "parent-contact-validation",
            "prove relief loop dual-route contact to parent trunk",
            "data/t2-parent-contact-validation.csv",
            "retain with parent contact; otherwise demote",
        ),
        _ => (
            "graph-contact-repair",
            "repair route geometry or split route family before tier decision",
            "data/tier-contact-witnesses.csv",
            "blocked from T2 regionalizer until contact evidence exists",
        ),
    }
}

