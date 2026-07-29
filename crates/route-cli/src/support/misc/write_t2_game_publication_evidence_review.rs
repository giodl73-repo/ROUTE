//! Helper `write_t2_game_publication_evidence_review`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn write_t2_game_publication_evidence_review(
    path: &Path,
    rows: &[T2GamePublicationEvidenceReviewRow],
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

