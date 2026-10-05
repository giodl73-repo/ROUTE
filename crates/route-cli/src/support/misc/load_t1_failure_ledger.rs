//! Helper `load_t1_failure_ledger`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t1_failure_ledger(path: &Path) -> Result<Vec<T1FailureRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_failure_ledger(file)
}
