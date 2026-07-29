//! Helper `load_t1_sla_candidate_universe`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t1_sla_candidate_universe(path: &Path) -> Result<Vec<T1SlaCandidateUniverseRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

