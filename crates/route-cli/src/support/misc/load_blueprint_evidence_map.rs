//! Helper `load_blueprint_evidence_map`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_blueprint_evidence_map(path: &Path) -> Result<Vec<BlueprintEvidenceRow>> {
    let file = std::fs::File::open(path)?;
    parse_blueprint_evidence_map(file)
}

