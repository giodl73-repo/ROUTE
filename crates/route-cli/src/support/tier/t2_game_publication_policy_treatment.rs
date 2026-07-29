//! Helper `t2_game_publication_policy_treatment`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_game_publication_policy_treatment(required_evidence: &str) -> &'static str {
    match required_evidence {
        "port-surge-demand-and-flood-closure-evidence" => {
            "require port surge demand evidence and flood closure source before scenario publication"
        }
        "managed-lane-merge-and-spillback-validation" => {
            "require managed-lane merge and spillback validation before scenario publication"
        }
        _ => "require standards proof and scenario promotion record before scenario publication",
    }
}

