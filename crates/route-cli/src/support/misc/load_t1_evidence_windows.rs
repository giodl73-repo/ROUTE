//! Helper `load_t1_evidence_windows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t1_evidence_windows(path: &Path) -> Result<Vec<T1EvidenceWindowRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_evidence_windows(file)
}
