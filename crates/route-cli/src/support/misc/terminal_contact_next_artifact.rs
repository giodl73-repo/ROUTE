//! Helper `terminal_contact_next_artifact`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn terminal_contact_next_artifact(zone_id: &str) -> String {
    match zone_id {
        "t3-great-lakes" => {
            "waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-02.md".to_string()
        }
        _ => "waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-03.md".to_string(),
    }
}

