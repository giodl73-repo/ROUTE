//! Helper `load_blueprint_cost_ranges`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_blueprint_cost_ranges(path: &Path) -> Result<Vec<BlueprintCostRow>> {
    let file = std::fs::File::open(path)?;
    parse_blueprint_cost_ranges(file)
}

