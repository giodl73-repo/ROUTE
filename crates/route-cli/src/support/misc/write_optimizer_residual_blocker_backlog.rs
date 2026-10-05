//! Helper `write_optimizer_residual_blocker_backlog`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn write_optimizer_residual_blocker_backlog(
    path: &Path,
    rows: &[OptimizerResidualBlockerBacklogRow],
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
