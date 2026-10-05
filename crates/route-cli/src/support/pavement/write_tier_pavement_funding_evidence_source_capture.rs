//! Helper `write_tier_pavement_funding_evidence_source_capture`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn write_tier_pavement_funding_evidence_source_capture(
    path: &Path,
    rows: &[TierPavementFundingEvidenceSourceCaptureRow],
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
