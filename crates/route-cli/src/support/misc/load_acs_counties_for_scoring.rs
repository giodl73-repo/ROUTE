//! Helper `load_acs_counties_for_scoring`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_acs_counties_for_scoring(
    manifest: &route_data::Manifest,
) -> Option<Vec<route_data::CountyCentroid>> {
    // Locate gazetteer
    let gaz_path: Option<std::path::PathBuf> = std::fs::read_dir(&manifest.cache_dir)
        .ok()
        .and_then(|entries| {
            entries
                .filter_map(|e| e.ok())
                .find(|e| {
                    e.file_name()
                        .to_string_lossy()
                        .ends_with("counties_national.txt")
                })
                .map(|e| e.path())
        });

    let gaz_path = gaz_path?;
    let mut counties = route_data::read_county_gazetteer(&gaz_path).ok()?;

    // Join ACS population if cached
    let pop_path = manifest.cache_dir.join("acs_county_pop_2022.csv");
    if pop_path.exists() {
        let _ = route_data::join_population(&mut counties, &pop_path);
    }

    // Join ACS median household income if cached (for C3 scoring)
    let inc_path = manifest.cache_dir.join("acs_county_income_2022.csv");
    if inc_path.exists() {
        let _ = route_data::join_income(&mut counties, &inc_path);
    }

    // Join RUCC rural codes if cached (for C2 rural_share scoring)
    let rucc_path = manifest.cache_dir.join("rucc_2023.csv");
    if rucc_path.exists() {
        let _ = route_data::join_rucc(&mut counties, &rucc_path);
    }

    Some(counties)
}
