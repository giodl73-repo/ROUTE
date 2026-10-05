//! Helper `load_excluded_i80_sources`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_excluded_i80_sources(
    path: &std::path::Path,
) -> Result<std::collections::BTreeSet<String>> {
    let mut reader = csv::Reader::from_path(path)
        .with_context(|| format!("reading I-80 source contract {}", path.display()))?;
    let mut excluded = std::collections::BTreeSet::new();
    for row in reader.deserialize::<I80SourcePolicyRow>() {
        let row = row.with_context(|| format!("parsing {}", path.display()))?;
        if row.acquisition_status.ends_with("excluded") {
            excluded.insert(row.source_id);
        }
    }
    Ok(excluded)
}
