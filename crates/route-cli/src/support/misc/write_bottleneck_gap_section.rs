//! Helper `write_bottleneck_gap_section`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn write_bottleneck_gap_section(md: &mut String) -> Result<()> {
    md.push_str("Source: `data/atri-bottlenecks.csv` hand-curated from ATRI truck bottleneck reporting.\n\n");
    let score_rows = load_score_signal_rows()?;
    let mut rdr =
        csv::Reader::from_path("data/atri-bottlenecks.csv").context("reading ATRI bottlenecks")?;
    md.push_str("| Rank | Location | Route | State | Annual cost $M | A1 | A3 | B2 | Signal |\n|---:|---|---|---|---:|---:|---:|---:|---|\n");
    for record in rdr.records().take(20) {
        let row = record?;
        let route = normalise_designation(csv_get(&row, 2));
        let signal = score_rows.get(&route);
        let (a1, a3, b2, label) = signal
            .map(|s| {
                (
                    format!("{:.1}", s.a1),
                    format!("{:.1}", s.a3),
                    format!("{:.1}", s.b2),
                    bottleneck_signal_label(s),
                )
            })
            .unwrap_or_else(|| ("".to_string(), "".to_string(), "".to_string(), "data_gap"));
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            csv_get(&row, 0),
            csv_get(&row, 1),
            csv_get(&row, 2),
            csv_get(&row, 3),
            csv_get(&row, 4),
            a1,
            a3,
            b2,
            label
        ));
    }
    md.push_str("\nInterpretation: ATRI bottlenecks are observed freight congestion seeds. `corridor_stress` means A1/A3 confirms broad congestion or reliability stress; `topology_chokepoint` means the route is central but the congestion is likely local/interchange-specific; `capacity_needs_flow` needs `route flow` or segment-level validation before being labeled structural capacity.\n");
    Ok(())
}

