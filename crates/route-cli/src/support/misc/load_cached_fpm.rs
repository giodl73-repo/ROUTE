//! Helper `load_cached_fpm`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_cached_fpm(manifest: &route_data::Manifest) -> Vec<route_data::HpmsFpmRecord> {
    [
        "hpms_fpm.csv",
        "fpm_2023.csv",
        "freight_performance_measures.csv",
    ]
    .iter()
    .map(|name| manifest.cache_dir.join(name))
    .find(|path| path.exists())
    .and_then(|path| route_data::hpms::read_hpms_fpm_csv(&path).ok())
    .unwrap_or_default()
}

