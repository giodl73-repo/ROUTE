//! Helper `load_significant_moments`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_significant_moments(path: &Path) -> Result<Vec<SignificantMomentRow>> {
    let file = std::fs::File::open(path)?;
    parse_significant_moments(file)
}

