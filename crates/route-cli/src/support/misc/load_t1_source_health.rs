//! Helper `load_t1_source_health`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t1_source_health(path: &Path) -> Result<Vec<T1SourceHealthRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_source_health(file)
}

