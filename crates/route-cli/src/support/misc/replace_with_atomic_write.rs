//! Helper `replace_with_atomic_write`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn replace_with_atomic_write(tmp: &Path, path: &Path) -> Result<()> {
    if path.exists() {
        std::fs::remove_file(path)
            .with_context(|| format!("removing previous {}", path.display()))?;
    }
    std::fs::rename(tmp, path)
        .with_context(|| format!("replacing {} with {}", path.display(), tmp.display()))?;
    Ok(())
}

