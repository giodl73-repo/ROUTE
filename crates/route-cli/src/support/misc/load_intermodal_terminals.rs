//! Helper `load_intermodal_terminals`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_intermodal_terminals() -> Vec<(f64, f64)> {
    let path = std::path::Path::new("data/intermodal_terminals.csv");
    if !path.exists() {
        return Vec::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return Vec::new();
    };
    rdr.records()
        .filter_map(|r| r.ok())
        .filter_map(|rec| {
            if rec.len() < 5 {
                return None;
            }
            let lat: f64 = rec[3].parse().ok()?;
            let lon: f64 = rec[4].parse().ok()?;
            Some((lat, lon))
        })
        .collect()
}

