//! Helper `load_t1_design_review`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t1_design_review(path: &Path) -> Result<Vec<T1DesignReviewCsvRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_design_review(file)
}
