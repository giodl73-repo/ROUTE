//! Helper `load_t1_design_policy_actions`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t1_design_policy_actions(path: &Path) -> Result<Vec<T1DesignPolicyActionRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_design_policy_actions(file)
}
