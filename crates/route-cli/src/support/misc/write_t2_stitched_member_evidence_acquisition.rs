//! Helper `write_t2_stitched_member_evidence_acquisition`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn write_t2_stitched_member_evidence_acquisition(
    path: &Path,
    rows: &[T2StitchedMemberEvidenceAcquisitionRow],
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

