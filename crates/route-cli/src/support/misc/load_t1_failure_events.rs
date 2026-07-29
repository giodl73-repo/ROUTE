//! Helper `load_t1_failure_events`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t1_failure_events(path: &Path) -> Result<Vec<T1FailureEventRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_failure_events(file)
}

