//! Helper `blueprint_evidence_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn blueprint_evidence_gate_failures(
    rows: &[BlueprintEvidenceRow],
    packages: &[BlueprintPackageRow],
    standards: &[StandardsProofRow],
) -> Vec<String> {
    if rows.is_empty() {
        return vec!["blueprint evidence map has no rows".to_string()];
    }

    let package_ids = packages
        .iter()
        .map(|row| row.package_id.as_str())
        .collect::<std::collections::HashSet<_>>();
    let standard_evidence = standards
        .iter()
        .map(|row| (row.standard_id.as_str(), row.evidence_level.as_str()))
        .collect::<std::collections::HashMap<_, _>>();

    let mut failures = Vec::new();
    for row in rows {
        if let Some(failure) = blueprint_evidence_row_failure(row, &package_ids, &standard_evidence)
        {
            failures.push(failure);
        }
    }

    for package in packages {
        for standard_id in package
            .standards
            .split(';')
            .map(|part| part.trim())
            .filter(|part| !part.is_empty())
        {
            if !rows.iter().any(|row| {
                row.package_id.trim() == package.package_id.trim()
                    && row.standard_id.trim() == standard_id
            }) {
                failures.push(format!(
                    "{} missing evidence-map row for {}",
                    package.package_id, standard_id
                ));
            }
        }
    }

    failures
}

