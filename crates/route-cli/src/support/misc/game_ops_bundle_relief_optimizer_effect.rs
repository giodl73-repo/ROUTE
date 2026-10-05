//! Helper `game_ops_bundle_relief_optimizer_effect`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn game_ops_bundle_relief_optimizer_effect(
    row: &T2GameOpsBundleEvidenceBlockerReliefRow,
) -> String {
    let base = "accepted game/ops bundle evidence policy removes bundle-binding blockers";
    let mut parts = Vec::new();
    if !row.qualification_effects.trim().is_empty() {
        parts.push(format!(
            "qualification_effects={}",
            row.qualification_effects
        ));
    }
    if !row.qualification_gate_policy.trim().is_empty() {
        parts.push(format!(
            "qualification_gate_policy={}",
            row.qualification_gate_policy
        ));
    }
    if !row.qualification_game_use.trim().is_empty() {
        parts.push(format!(
            "qualification_game_use={}",
            row.qualification_game_use
        ));
    }
    if parts.is_empty() {
        return base.to_string();
    }
    format!("{base}; {}", parts.join("; "))
}
