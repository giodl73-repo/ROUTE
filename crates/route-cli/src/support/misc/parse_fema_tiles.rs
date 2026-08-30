//! Helper `parse_fema_tiles`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_fema_tiles(reader: impl std::io::Read) -> Vec<FemaTile> {
    let mut rdr = csv::Reader::from_reader(reader);
    rdr.records()
        .filter_map(|r| r.ok())
        .filter_map(|rec| {
            if rec.len() < 7 {
                return None;
            }
            let xmin: f64 = rec[1].trim().parse().ok()?;
            let ymin: f64 = rec[2].trim().parse().ok()?;
            let xmax: f64 = rec[3].trim().parse().ok()?;
            let ymax: f64 = rec[4].trim().parse().ok()?;
            let sfha_count: u32 = rec[5].trim().parse().ok()?;
            let status = rec[6].trim().to_string();
            if status != "ok" {
                return None;
            }
            Some(FemaTile {
                name: rec[0].trim().to_string(),
                xmin,
                ymin,
                xmax,
                ymax,
                sfha_count,
                status,
            })
        })
        .collect()
}
