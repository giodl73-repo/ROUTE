//! Helper `atomic_write_text`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn atomic_write_text(path: &Path, text: impl AsRef<str>) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let tmp = temp_path_for_atomic_write(path);
    std::fs::write(&tmp, text.as_ref()).with_context(|| format!("writing {}", tmp.display()))?;
    replace_with_atomic_write(&tmp, path)
}

