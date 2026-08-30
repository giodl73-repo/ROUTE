//! Helper `ensure_shapefile`.
#[allow(unused_imports)]
use crate::*;

/// Ensure the TIGER shapefile is extracted; return path to .shp file.
pub(crate) fn ensure_shapefile(manifest: &route_data::Manifest) -> Result<std::path::PathBuf> {
    let extract_dir = manifest.cache_dir.join("tiger-primary-roads");
    let shp_path = extract_dir.join("tl_2023_us_primaryroads.shp");
    if shp_path.exists() {
        return Ok(shp_path);
    }

    let zip_path = manifest.cache_path("tiger-primary-roads");
    if !zip_path.exists() {
        anyhow::bail!("TIGER primary roads not cached — run `route fetch` first.");
    }
    println!("  extracting shapefile…");
    route_data::fetch::extract_shp(&zip_path, &extract_dir)
}
