//! Helper `write_intermodal_gap_section`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn write_intermodal_gap_section(md: &mut String) -> Result<()> {
    md.push_str("Source: `data/scores-all.csv`; candidates here have high B3 port/border access but low D2 multimodal integration.\n\n");
    let mut rdr = csv::Reader::from_path("data/scores-all.csv").context("reading scores-all")?;
    let headers = rdr.headers()?.clone();
    let b3_idx = headers.iter().position(|h| h == "B3").unwrap_or(16);
    let d2_idx = headers.iter().position(|h| h == "D2").unwrap_or(23);
    let b3_conf_idx = headers.iter().position(|h| h == "B3_conf").unwrap_or(32);
    let d2_conf_idx = headers.iter().position(|h| h == "D2_conf").unwrap_or(39);
    let mut rows = Vec::new();
    for record in rdr.records() {
        let row = record?;
        let b3 = csv_get(&row, b3_idx).parse::<f64>().unwrap_or(0.0);
        let d2 = csv_get(&row, d2_idx).parse::<f64>().unwrap_or(0.0);
        if b3 >= 8.0 && d2 <= 5.0 {
            rows.push((b3 - d2, row));
        }
    }
    rows.sort_by(|a, b| b.0.total_cmp(&a.0));
    md.push_str("| Route | Score | Tier | B3 port/border | B3 conf | D2 multimodal | D2 conf | Claim label |\n|---|---:|---|---:|---:|---:|---:|---|\n");
    for (_, row) in rows.into_iter().take(20) {
        let b3_conf = csv_get(&row, b3_conf_idx).parse::<f32>().unwrap_or(0.0);
        let d2_conf = csv_get(&row, d2_conf_idx).parse::<f32>().unwrap_or(0.0);
        let claim_conf = b3_conf.min(d2_conf);
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            csv_get(&row, 0),
            csv_get(&row, 1),
            csv_get(&row, 2),
            csv_get(&row, b3_idx),
            csv_get(&row, b3_conf_idx),
            csv_get(&row, d2_idx),
            csv_get(&row, d2_conf_idx),
            route_score::confidence_label(claim_conf)
        ));
    }
    md.push_str("\nInterpretation: these are port/border-adjacent corridors whose multimodal support is weak under the current source model; validate terminal and connector data before elevating them to projects.\n");
    Ok(())
}
