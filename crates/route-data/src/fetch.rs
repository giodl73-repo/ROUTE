use crate::manifest::Manifest;
use anyhow::{Context, Result};
use std::io::Write;
use std::path::{Path, PathBuf};

/// Download all manifest sources to the cache directory.
/// Skips files already present unless `--force` is set.
pub fn fetch_all(manifest: &Manifest, force: bool) -> Result<()> {
    std::fs::create_dir_all(&manifest.cache_dir).context("creating cache directory")?;

    for (name, source) in &manifest.sources {
        // Skip sources with no URL or placeholder URLs
        if source.url.is_empty() {
            println!("  [skip] {name} — no URL (manual source)");
            continue;
        }

        let dest = manifest.cache_dir.join(&source.filename);
        if dest.exists() && !force {
            println!(
                "  [skip] {name} — already cached ({} bytes)",
                dest.metadata().map(|m| m.len()).unwrap_or(0)
            );
            continue;
        }
        println!("  [fetch] {name}");
        println!("          {}", source.url);
        download(&source.url, &dest).with_context(|| format!("downloading {name}"))?;
        println!(
            "  [ok]    {} → {} bytes",
            name,
            dest.metadata().map(|m| m.len()).unwrap_or(0)
        );
    }
    Ok(())
}

/// Extract the .shp file from a downloaded .zip archive.
/// Returns the path to the extracted .shp file.
pub fn extract_shp(zip_path: &Path, dest_dir: &Path) -> Result<std::path::PathBuf> {
    let file =
        std::fs::File::open(zip_path).with_context(|| format!("opening {}", zip_path.display()))?;
    let mut archive = zip::ZipArchive::new(file).context("reading zip archive")?;

    std::fs::create_dir_all(dest_dir).context("creating extraction directory")?;

    let mut shp_path = None;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i)?;
        let name = entry.name().to_string();
        let out_path = dest_dir.join(&name);
        let mut out = std::fs::File::create(&out_path)
            .with_context(|| format!("creating {}", out_path.display()))?;
        std::io::copy(&mut entry, &mut out)?;
        if name.ends_with(".shp") {
            shp_path = Some(out_path);
        }
    }

    shp_path.ok_or_else(|| anyhow::anyhow!("no .shp file found in zip"))
}

fn download(url: &str, dest: &Path) -> Result<()> {
    let response = reqwest::blocking::get(url)
        .with_context(|| format!("GET {url}"))?
        .error_for_status()
        .context("HTTP error response")?;

    let bytes = response.bytes().context("reading response body")?;
    atomic_write_bytes(dest, &bytes).with_context(|| format!("writing {}", dest.display()))?;
    Ok(())
}

pub fn atomic_write_bytes(dest: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = dest
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let tmp = temp_path_for(dest);
    let mut file =
        std::fs::File::create(&tmp).with_context(|| format!("creating {}", tmp.display()))?;
    file.write_all(&bytes).context("writing file")?;
    file.flush().context("flushing file")?;
    drop(file);
    replace_with_temp(&tmp, dest)
}

pub fn temp_path_for(dest: &Path) -> PathBuf {
    let mut file_name = dest
        .file_name()
        .map(|value| value.to_os_string())
        .unwrap_or_else(|| "download".into());
    file_name.push(format!(".{}.tmp", std::process::id()));
    dest.with_file_name(file_name)
}

pub fn replace_with_temp(tmp: &Path, dest: &Path) -> Result<()> {
    if dest.exists() {
        std::fs::remove_file(dest)
            .with_context(|| format!("removing previous {}", dest.display()))?;
    }
    std::fs::rename(tmp, dest)
        .with_context(|| format!("replacing {} with {}", dest.display(), tmp.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::atomic_write_bytes;

    #[test]
    fn atomic_write_replaces_existing_file_after_temp_write() {
        let dir =
            std::env::temp_dir().join(format!("route_fetch_atomic_write_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create temp test dir");
        let path = dir.join("cache.csv");
        std::fs::write(&path, "old cache").expect("seed cache");

        atomic_write_bytes(&path, b"new cache").expect("atomic write");

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "new cache");
        assert!(std::fs::read_dir(&dir).unwrap().all(|entry| !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .contains(".tmp")));

        let _ = std::fs::remove_dir_all(&dir);
    }
}
