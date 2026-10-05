//! Helper `load_t1_diamond_validation`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t1_diamond_validation(path: &Path) -> Result<Vec<T1DiamondValidationRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_diamond_validation(file)
}
