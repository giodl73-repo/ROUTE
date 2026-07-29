//! Helper `write_t2_held_contact_actions`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn write_t2_held_contact_actions(path: &Path, rows: &[T2HeldContactActionRow]) -> Result<()> {
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

