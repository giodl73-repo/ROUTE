//! Helper `load_t1_snapshot_plan`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t1_snapshot_plan(path: &Path) -> Result<Vec<T1SnapshotPlanRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_snapshot_plan(file)
}

