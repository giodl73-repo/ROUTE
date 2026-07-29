//! Helper `temp_path_for_atomic_write`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn temp_path_for_atomic_write(path: &Path) -> PathBuf {
    let mut file_name = path
        .file_name()
        .map(|value| value.to_os_string())
        .unwrap_or_else(|| "route-cache".into());
    file_name.push(format!(".{}.tmp", std::process::id()));
    path.with_file_name(file_name)
}

