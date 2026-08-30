//! Helper `write_t4_terminal_contact_proof_source_registry`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn write_t4_terminal_contact_proof_source_registry(
    path: &Path,
    rows: &[T4TerminalContactProofSourceRegistryRow],
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
