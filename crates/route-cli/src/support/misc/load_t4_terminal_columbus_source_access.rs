//! Helper `load_t4_terminal_columbus_source_access`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t4_terminal_columbus_source_access(
    path: &Path,
) -> Result<Vec<T4TerminalColumbusSourceAccessRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

