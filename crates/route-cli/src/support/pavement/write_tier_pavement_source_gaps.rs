//! Helper `write_tier_pavement_source_gaps`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn write_tier_pavement_source_gaps(
    path: &Path,
    rows: &[TierPavementSourceGapRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}
