//! Helper `insert_optimizer_qualification_effects`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn insert_optimizer_qualification_effects(
    target: &mut std::collections::BTreeSet<String>,
    optimizer_effect: &str,
) {
    for part in optimizer_effect.split(';').map(str::trim) {
        if let Some(effects) = part.strip_prefix("qualification_effects=") {
            insert_pipe_values(target, effects);
        } else if part.starts_with("qualification_gate_policy=")
            || part.starts_with("qualification_game_use=")
        {
            target.insert(part.to_string());
        }
    }
}
