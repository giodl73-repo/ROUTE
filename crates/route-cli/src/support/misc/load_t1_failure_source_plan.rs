//! Helper `load_t1_failure_source_plan`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t1_failure_source_plan(path: &Path) -> Result<Vec<T1FailureSourceRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_failure_source_plan(file)
}

