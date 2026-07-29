//! Helper `load_ports`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_ports() -> Vec<PortLocation> {
    let path = std::path::Path::new("data/ports.csv");
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
            let lat: f64 = rec[1].parse().ok()?;
            let lon: f64 = rec[2].parse().ok()?;
            let rank: u32 = rec[3].parse().ok()?;
            let is_border = rec[4].contains("border");
            Some(PortLocation {
                lat,
                lon,
                _rank: rank,
                is_border,
            })
        })
        .collect()
}

