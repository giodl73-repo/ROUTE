//! Helper `tier_contact_witness_status`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_contact_witness_status(
    repair_action: &str,
) -> (&'static str, &'static str, &'static str) {
    match repair_action {
        "keep-for-regionalizer" => ("regionalizer-ready", "accepted", "pass"),
        "keep-with-parent-region-review" => ("parent-region-review", "review", "review"),
        "add-dual-contact-witness" => ("dual-contact-needed", "source-needed", "review"),
        "add-parent-contact-or-demote" => ("parent-contact-needed", "source-needed", "review"),
        "terminal-exception-or-demote" => ("terminal-exception-needed", "source-needed", "review"),
        "demote-to-t3-t4" => ("tier-demotion-needed", "policy-action", "review"),
        "fix-graph-contact-or-demote" => ("graph-contact-needed", "source-needed", "review"),
        _ => ("unknown-repair-action", "source-needed", "review"),
    }
}
