//! Helper `t3_obligation_class_for_intake`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_obligation_class_for_intake(intake_class: &str) -> &'static str {
    match intake_class {
        "bubble-up-t2-review" => "regional-upgrade-review",
        "t4-local-intake" => "terminal-local-access",
        _ => "regional-feeder-access",
    }
}
