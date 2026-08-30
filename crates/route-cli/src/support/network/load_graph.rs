//! Helper `load_graph`.
#[allow(unused_imports)]
use crate::*;

/// Load the HighwayGraph from cached TIGER + optional HPMS.
pub(crate) fn load_graph(manifest: &route_data::Manifest) -> Result<route_network::HighwayGraph> {
    let shp_path = ensure_shapefile(manifest)?;
    // Always load all road classes — US highways needed for upgrade-candidate scoring
    let segments = route_data::nhs::read_nhs_shapefile(&shp_path, true)
        .map_err(|e| anyhow::anyhow!("shapefile error: {e}"))?;

    // Auto-load HPMS if cached
    let hpms_path = manifest.cache_dir.join("hpms_2018.csv");
    let hpms = if hpms_path.exists() {
        route_data::hpms::read_hpms_csv(&hpms_path).unwrap_or_default()
    } else {
        Vec::new()
    };

    let fpm = load_cached_fpm(manifest);
    let (graph, _) = route_network::build_graph_with_fpm(segments, &hpms, &fpm);
    Ok(graph)
}
