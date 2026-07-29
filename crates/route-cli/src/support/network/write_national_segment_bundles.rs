//! Helper `write_national_segment_bundles`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn write_national_segment_bundles(path: &Path, rows: &[NationalSegmentBundleRow]) -> Result<()> {
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

