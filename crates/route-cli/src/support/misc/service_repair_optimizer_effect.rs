//! Helper `service_repair_optimizer_effect`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn service_repair_optimizer_effect(
    effect: &str,
    row: &T2BundleOverlayRepairTargetRow,
) -> String {
    if row.qualification_effects.trim().is_empty() {
        return effect.to_string();
    }
    format!(
        "{effect}; qualification_effects={}",
        row.qualification_effects
    )
}
