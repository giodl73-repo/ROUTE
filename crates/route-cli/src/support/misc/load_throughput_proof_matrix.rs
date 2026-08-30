//! Helper `load_throughput_proof_matrix`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_throughput_proof_matrix(path: &Path) -> Result<Vec<ThroughputProofRow>> {
    let file = std::fs::File::open(path)?;
    parse_throughput_proof_matrix(file)
}
