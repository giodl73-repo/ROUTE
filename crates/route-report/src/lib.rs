use anyhow::{Context, Result};
use route_network::Corridor;
use route_score::DimensionScores;
use std::path::Path;

/// Write a corpus entry markdown file for a corridor.
/// Follows corpus/SCHEMA.md exactly.
/// Idempotent — overwrites the file with current scores.
pub fn write_corpus_entry(
    corridor: &Corridor,
    scores: &DimensionScores,
    output_path: &Path,
) -> Result<()> {
    let content = format_corpus_entry(corridor, scores);
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).context("creating corpus directory")?;
    }
    std::fs::write(output_path, content)
        .with_context(|| format!("writing {}", output_path.display()))
}

fn format_corpus_entry(corridor: &Corridor, scores: &DimensionScores) -> String {
    let today = chrono_today();
    let estimated_flag = if scores.any_estimated() { "†" } else { "" };
    let attrs = &corridor.attributes;

    let mut md = String::new();

    // Frontmatter
    md.push_str("---\n");
    md.push_str(&format!(
        "name: \"{} — {} to {}\"\n",
        corridor.designation, corridor.termini[0], corridor.termini[1]
    ));
    md.push_str(&format!("slug: {}\n", slug(&corridor.designation)));
    md.push_str("type: existing-corridor\n");
    md.push_str("status: draft\n");
    md.push_str(&format!("rubric_version: {}\n", scores.rubric_version));
    md.push_str("author: route-score\n");
    md.push_str(&format!("created: {today}\n"));
    md.push_str(&format!("updated: {today}\n"));
    md.push_str("sources:\n");
    md.push_str("  - \"FHWA HPMS 2023\"\n");
    md.push_str("  - \"FHWA NBI 2023\"\n");
    md.push_str("  - \"FAF5 v5.6 BTS/FHWA 2022\"\n");
    md.push_str("  - \"Census ACS 2022\"\n");
    md.push_str("  - \"BEA CAINC4 2022\"\n");
    md.push_str("corridor:\n");
    md.push_str(&format!(
        "  termini: [\"{}\", \"{}\"]\n",
        corridor.termini[0], corridor.termini[1]
    ));
    md.push_str(&format!(
        "  states: [{}]\n",
        corridor
            .states
            .iter()
            .map(|s| format!("\"{s}\""))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    md.push_str(&format!("  approx_miles: {:.0}\n", corridor.total_miles));
    md.push_str(&format!("  designation: \"{}\"\n", corridor.designation));
    md.push_str("  classification: trunk\n");
    md.push_str("---\n\n");

    // Title
    md.push_str(&format!("# {}{}\n\n", corridor.designation, estimated_flag));

    // Overview — placeholder for human annotation
    md.push_str("## Overview\n\n");
    md.push_str("*[Human annotation — describe corridor function and primary economic role.]*\n\n");

    // Key facts
    md.push_str("## Key Facts\n\n");
    md.push_str("| Fact | Value | Source |\n|---|---|---|\n");
    md.push_str(&format!(
        "| Total miles | {:.0} | NHS shapefile |\n",
        corridor.total_miles
    ));
    md.push_str(&format!(
        "| Mean AADT | {} | HPMS 2023 |\n",
        attrs
            .mean_aadt
            .map(|v| format!("{v:.0}"))
            .unwrap_or("N/A".into())
    ));
    md.push_str(&format!(
        "| Mean truck % | {} | HPMS 2023 |\n",
        attrs
            .mean_pct_truck
            .map(|v| format!("{:.0}%", v * 100.0))
            .unwrap_or("N/A".into())
    ));
    md.push_str(&format!(
        "| Bridges | {} | NBI 2023 |\n",
        attrs.bridge_count
    ));
    md.push_str(&format!(
        "| Bridges poor | {} | NBI 2023 |\n",
        attrs
            .pct_bridges_poor
            .map(|v| format!("{:.0}%", v * 100.0))
            .unwrap_or("N/A".into())
    ));
    md.push_str(&format!(
        "| States | {} | NHS shapefile |\n",
        corridor.states.join(", ")
    ));
    md.push('\n');

    // Dimension scores table
    md.push_str("## Dimension Scores\n\n");
    md.push_str(&format!("Rubric version: `{}`\n\n", scores.rubric_version));
    md.push_str("| Band | Dim | Name | Score | Quality | Confidence | Justification |\n|---|---|---|---|---|---|---|\n");

    let all = [
        ("A", &scores.a1),
        ("A", &scores.a2),
        ("A", &scores.a3),
        ("A", &scores.a4),
        ("A", &scores.a5),
        ("B", &scores.b1),
        ("B", &scores.b2),
        ("B", &scores.b3),
        ("B", &scores.b4),
        ("C", &scores.c1),
        ("C", &scores.c2),
        ("C", &scores.c3),
        ("C", &scores.c4),
        ("D", &scores.d1),
        ("D", &scores.d2),
        ("D", &scores.d3),
    ];

    for (band, sd) in all.iter() {
        let est = if sd.estimated { "†" } else { "" };
        md.push_str(&format!(
            "| {} | {} | {} | {:.1}{} | {} | {:.2} | {} |\n",
            band,
            sd.dim.code(),
            sd.dim.name(),
            sd.score,
            est,
            sd.quality_label(),
            sd.confidence,
            sd.justification
        ));
    }

    md.push_str(&format!("\n**Band totals**: A: {:.1}/50 · B: {:.1}/40 · C: {:.1}/40 · D: {:.1}/30 · **Total: {:.1}/160**\n\n",
        scores.band_a(), scores.band_b(), scores.band_c(), scores.band_d(), scores.total()));

    if scores.any_estimated() {
        md.push_str("† Estimated value — see score justification for details.\n\n");
    }

    md.push_str("## Notable Segments\n\n*[Human annotation.]*\n\n");
    md.push_str("## Interstate 2.0 Fit\n\n*[Human annotation.]*\n\n");
    md.push_str("## Open Questions\n\n*[Human annotation.]*\n\n");
    md.push_str("## Sources\n\nSee `data/sources.md` for full citations.\n");

    md
}

fn slug(designation: &str) -> String {
    designation
        .to_lowercase()
        .replace('-', "-")
        .replace(' ', "-")
}

fn chrono_today() -> String {
    // Simple date without chrono dependency — use env or fixed fallback
    std::env::var("ROUTE_DATE").unwrap_or_else(|_| "2026-05-06".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use route_network::CorridorAttributes;
    use route_score::score_corridor;

    fn corridor() -> Corridor {
        Corridor {
            designation: "I80".into(),
            termini: ["Teaneck, NJ".into(), "San Francisco, CA".into()],
            states: vec!["NJ".into(), "PA".into(), "OH".into()],
            total_miles: 2909.0,
            edge_count: 0,
            edges: vec![],
            attributes: CorridorAttributes {
                p90_aadt: Some(90_000.0),
                mean_aadt: Some(60_000.0),
                annual_freight_value_b: Some(500.0),
                mean_pct_truck: Some(0.22),
                p90_pti: Some(1.8),
                intl_trade_score: 4.0,
                fatal_crash_rate: Some(1.0),
                detour_penalty_miles: Some(250.0),
                nearest_parallel_miles: Some(70.0),
                betweenness_centrality: Some(0.8),
                nearest_top25_port_miles: Some(40.0),
                pop_within_50mi: Some(12_000_000),
                pct_rural_in_buffer: Some(0.35),
                max_rural_interchange_gap_miles: Some(45.0),
                gdp_per_capita_relative: Some(0.75),
                agricultural_export_score: 6.0,
                max_consecutive_sfha_miles: Some(18.0),
                fema_sfha_miles: Some(75.0),
                intermodal_hub_count: 3,
                dcfc_per_100mi: Some(22.0),
                bridge_count: 100,
                pct_bridges_poor: Some(0.08),
                mean_year_built: Some(1970.0),
                military_strategic_score: 5.0,
                ..Default::default()
            },
        }
    }

    #[test]
    fn corpus_entry_contains_all_sixteen_dimensions_and_160_total() {
        let corridor = corridor();
        let scores = score_corridor(
            &corridor.attributes,
            &route_score::ScoringConfig::default_config(),
        );
        let md = format_corpus_entry(&corridor, &scores);

        for code in [
            "A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "C1", "C2", "C3", "C4", "D1",
            "D2", "D3",
        ] {
            assert!(md.contains(&format!("| {code} |")), "missing {code}");
        }
        assert!(md.contains("A: "));
        assert!(md.contains("/50"));
        assert!(md.contains("/40"));
        assert!(md.contains("/30"));
        assert!(md.contains("Total: "));
        assert!(md.contains("/160"));
        assert!(md.contains("† Estimated value"));
    }

    #[test]
    fn corpus_entry_uses_route_date_for_reproducibility() {
        unsafe {
            std::env::set_var("ROUTE_DATE", "2030-01-02");
        }

        let corridor = corridor();
        let scores = score_corridor(
            &corridor.attributes,
            &route_score::ScoringConfig::default_config(),
        );
        let md = format_corpus_entry(&corridor, &scores);

        assert!(md.contains("created: 2030-01-02"));
        assert!(md.contains("updated: 2030-01-02"));

        unsafe {
            std::env::remove_var("ROUTE_DATE");
        }
    }

    #[test]
    fn write_corpus_entry_creates_parent_directories() {
        let dir = std::env::temp_dir().join(format!("route-report-test-{}", std::process::id()));
        let path = dir.join("nested").join("i80.md");
        let corridor = corridor();
        let scores = score_corridor(
            &corridor.attributes,
            &route_score::ScoringConfig::default_config(),
        );

        write_corpus_entry(&corridor, &scores, &path).expect("write corpus entry");

        let written = std::fs::read_to_string(&path).expect("read written corpus entry");
        assert!(written.contains("# I80"));
        assert!(written.contains("rubric_version:"));

        let _ = std::fs::remove_dir_all(dir);
    }
}
