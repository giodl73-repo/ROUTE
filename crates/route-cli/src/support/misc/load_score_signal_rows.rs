//! Helper `load_score_signal_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_score_signal_rows() -> Result<std::collections::HashMap<String, ScoreSignalRow>>
{
    let mut rdr = csv::Reader::from_path("data/scores-all.csv").context("reading scores-all")?;
    let headers = rdr.headers()?.clone();
    let route_idx = headers.iter().position(|h| h == "route").unwrap_or(0);
    let a1_idx = headers.iter().position(|h| h == "A1").unwrap_or(9);
    let a3_idx = headers.iter().position(|h| h == "A3").unwrap_or(11);
    let b2_idx = headers.iter().position(|h| h == "B2").unwrap_or(15);
    let mut rows = std::collections::HashMap::new();
    for record in rdr.records() {
        let row = record?;
        rows.insert(
            normalise_designation(csv_get(&row, route_idx)),
            ScoreSignalRow {
                a1: csv_get(&row, a1_idx).parse().unwrap_or(0.0),
                a3: csv_get(&row, a3_idx).parse().unwrap_or(0.0),
                b2: csv_get(&row, b2_idx).parse().unwrap_or(0.0),
            },
        );
    }
    Ok(rows)
}
