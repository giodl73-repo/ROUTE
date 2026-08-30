//! Helper `route_family_split_optimizer_effect`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn route_family_split_optimizer_effect(
    effect: &str,
    qualification_effects: &str,
) -> String {
    if qualification_effects.trim().is_empty() {
        return effect.to_string();
    }
    format!("{effect}; qualification_effects={qualification_effects}")
}
