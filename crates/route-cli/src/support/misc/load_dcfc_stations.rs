//! Helper `load_dcfc_stations`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_dcfc_stations() -> Vec<(f64, f64)> {
    // (lat, lon)
    let path = std::path::Path::new("data/cache/dcfc_stations.csv");
    if !path.exists() {
        return Vec::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return Vec::new();
    };
    rdr.records()
        .filter_map(|r| r.ok())
        .filter_map(|rec| {
            if rec.len() < 7 {
                return None;
            }
            let lat: f64 = rec[4].parse().ok()?;
            let lon: f64 = rec[5].parse().ok()?;
            if lat.abs() < 1.0 || lon.abs() < 1.0 {
                return None;
            }
            Some((lat, lon))
        })
        .collect()
}
