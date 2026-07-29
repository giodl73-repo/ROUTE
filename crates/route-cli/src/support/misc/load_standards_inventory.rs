//! Helper `load_standards_inventory`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_standards_inventory(path: &Path) -> Result<Vec<StandardsInventoryRow>> {
    let file = std::fs::File::open(path)?;
    parse_standards_inventory(file)
}

