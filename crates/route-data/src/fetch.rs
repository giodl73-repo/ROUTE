use crate::manifest::Manifest;
use anyhow::{Context, Result};
use std::io::Write;
use std::path::Path;

/// Download all manifest sources to the cache directory.
/// Skips files already present unless `--force` is set.
pub fn fetch_all(manifest: &Manifest, force: bool) -> Result<()> {
    std::fs::create_dir_all(&manifest.cache_dir)
        .context("creating cache directory")?;

    for (name, source) in &manifest.sources {
        let dest = manifest.cache_dir.join(&source.filename);
        if dest.exists() && !force {
            println!("  [skip] {name} — already cached at {}", dest.display());
            continue;
        }
        println!("  [fetch] {name} from {}", source.url);
        download(&source.url, &dest)
            .with_context(|| format!("downloading {name}"))?;
        println!("  [ok]    {name} → {}", dest.display());
    }
    Ok(())
}

/// Extract the .shp file from a downloaded .zip archive.
/// Returns the path to the extracted .shp file.
pub fn extract_shp(zip_path: &Path, dest_dir: &Path) -> Result<std::path::PathBuf> {
    let file = std::fs::File::open(zip_path)
        .with_context(|| format!("opening {}", zip_path.display()))?;
    let mut archive = zip::ZipArchive::new(file)
        .context("reading zip archive")?;

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
    let mut file = std::fs::File::create(dest)
        .with_context(|| format!("creating {}", dest.display()))?;
    file.write_all(&bytes).context("writing file")?;
    Ok(())
}
