//! Runtime discovery for repository-relative development assets.
use std::path::{Path, PathBuf};

pub(crate) fn repository_root() -> Option<PathBuf> {
    if let Some(configured) = std::env::var_os("ROUTE_REPOSITORY_ROOT") {
        if !configured.is_empty() {
            return Some(PathBuf::from(configured));
        }
    }

    let current_dir = std::env::current_dir().ok();
    let executable_dir = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf));

    current_dir
        .iter()
        .chain(executable_dir.iter())
        .flat_map(|start| start.ancestors())
        .find(|candidate| {
            candidate.join("Cargo.toml").is_file()
                && candidate.join("crates/route-cli/Cargo.toml").is_file()
        })
        .map(Path::to_path_buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discovers_workspace_without_a_compiled_checkout_path() {
        let root = repository_root().expect("ROUTE workspace root");
        assert!(root.join("crates/route-cli/Cargo.toml").is_file());
    }
}
