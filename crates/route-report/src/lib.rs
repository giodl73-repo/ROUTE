use anyhow::{Context, Result};
use route_network::{Corridor, SegmentBundle};
use route_score::{BundleScores, DimensionScores};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CorpusAnnotations {
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub overview: String,
    #[serde(default)]
    pub notable_segments: Vec<String>,
    #[serde(default)]
    pub interstate_2_0_fit: Vec<String>,
    #[serde(default)]
    pub claim_holds: Vec<String>,
    #[serde(default)]
    pub held_dimensions: Vec<String>,
    #[serde(default)]
    pub open_questions: Vec<String>,
    #[serde(default)]
    pub sources: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CorpusProvenance {
    pub command: String,
    pub manifest_version: String,
    pub manifest_path: String,
    pub scoring_config_path: String,
}

impl Default for CorpusProvenance {
    fn default() -> Self {
        Self {
            command: "unknown".to_string(),
            manifest_version: "unknown".to_string(),
            manifest_path: "unknown".to_string(),
            scoring_config_path: "unknown".to_string(),
        }
    }
}

/// Write a corpus entry markdown file for a corridor.
/// Follows corpus/SCHEMA.md exactly.
/// Idempotent — overwrites the file with current scores.
pub fn write_corpus_entry(
    corridor: &Corridor,
    scores: &DimensionScores,
    output_path: &Path,
) -> Result<()> {
    write_corpus_entry_with_provenance(corridor, scores, output_path, &CorpusProvenance::default())
}

pub fn write_corpus_entry_with_provenance(
    corridor: &Corridor,
    scores: &DimensionScores,
    output_path: &Path,
    provenance: &CorpusProvenance,
) -> Result<()> {
    let annotations = load_annotations(output_path)?;
    let content =
        format_corpus_entry_with_annotations(corridor, scores, provenance, annotations.as_ref());
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).context("creating corpus directory")?;
    }
    std::fs::write(output_path, content)
        .with_context(|| format!("writing {}", output_path.display()))
}

pub fn write_bundle_corpus_entry(
    bundle: &SegmentBundle,
    corridor: &Corridor,
    scores: &BundleScores,
    output_path: &Path,
) -> Result<()> {
    write_bundle_corpus_entry_with_provenance(
        bundle,
        corridor,
        scores,
        output_path,
        &CorpusProvenance::default(),
    )
}

pub fn write_bundle_corpus_entry_with_provenance(
    bundle: &SegmentBundle,
    corridor: &Corridor,
    scores: &BundleScores,
    output_path: &Path,
    provenance: &CorpusProvenance,
) -> Result<()> {
    let annotations = load_annotations(output_path)?;
    let content =
        format_bundle_corpus_entry(bundle, corridor, scores, provenance, annotations.as_ref());
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent).context("creating corpus directory")?;
    }
    std::fs::write(output_path, content)
        .with_context(|| format!("writing {}", output_path.display()))
}

fn format_bundle_corpus_entry(
    bundle: &SegmentBundle,
    corridor: &Corridor,
    scores: &BundleScores,
    provenance: &CorpusProvenance,
    annotations: Option<&CorpusAnnotations>,
) -> String {
    let mut content =
        format_corpus_entry_with_annotations(corridor, &scores.scores, provenance, annotations);
    let insertion = format!(
        "bundle:\n  segment_bundle_id: \"{}\"\n  bundle_role: \"{}\"\n  member_count: {}\n  member_segment_ids: [{}]\n",
        bundle.segment_bundle_id,
        bundle.bundle_role,
        bundle.member_segment_ids.len(),
        bundle
            .member_segment_ids
            .iter()
            .map(|id| format!("\"{id}\""))
            .collect::<Vec<_>>()
            .join(", ")
    );
    if let Some(index) = content.find("corridor:\n") {
        content.insert_str(index, &insertion);
    }
    content
}

fn format_corpus_entry_with_annotations(
    corridor: &Corridor,
    scores: &DimensionScores,
    provenance: &CorpusProvenance,
    annotations: Option<&CorpusAnnotations>,
) -> String {
    let today = chrono_today();
    let estimated_flag = if scores.any_estimated() { "†" } else { "" };
    let attrs = &corridor.attributes;
    let document_status = annotations
        .and_then(|value| value.status.as_deref())
        .unwrap_or("draft");

    let mut md = String::new();

    // Frontmatter
    md.push_str("---\n");
    md.push_str(&format!(
        "name: \"{} — {} to {}\"\n",
        corridor.designation, corridor.termini[0], corridor.termini[1]
    ));
    md.push_str(&format!("slug: {}\n", slug(&corridor.designation)));
    md.push_str("type: existing-corridor\n");
    md.push_str(&format!("status: {document_status}\n"));
    md.push_str(&format!("rubric_version: {}\n", scores.rubric_version));
    md.push_str(&format!("generated_by: \"{}\"\n", provenance.command));
    md.push_str(&format!(
        "data_manifest_version: \"{}\"\n",
        provenance.manifest_version
    ));
    md.push_str(&format!(
        "data_manifest_path: \"{}\"\n",
        provenance.manifest_path
    ));
    md.push_str(&format!(
        "scoring_config_path: \"{}\"\n",
        provenance.scoring_config_path
    ));
    md.push_str(&format!("estimated: {}\n", scores.any_estimated()));
    md.push_str(&format!("confidence: {:.2}\n", scores.mean_confidence()));
    md.push_str(&format!(
        "score_confidence: {:.2}\n",
        scores.score_weighted_confidence()
    ));
    md.push_str(&format!(
        "confidence_label: \"{}\"\n",
        route_score::confidence_label(scores.mean_confidence())
    ));
    md.push_str(&format!(
        "score_confidence_label: \"{}\"\n",
        route_score::confidence_label(scores.score_weighted_confidence())
    ));
    md.push_str("author: route-score\n");
    md.push_str(&format!("created: {today}\n"));
    md.push_str(&format!("updated: {today}\n"));
    md.push_str("sources:\n");
    md.push_str("  - \"FHWA HPMS 2023\"\n");
    md.push_str("  - \"FHWA NBI 2023\"\n");
    md.push_str("  - \"FAF5 v5.6 BTS/FHWA 2022\"\n");
    md.push_str("  - \"Census ACS 2022\"\n");
    md.push_str("  - \"BEA CAINC4 2022\"\n");
    if let Some(annotations) = annotations {
        for source in &annotations.sources {
            md.push_str(&format!("  - \"{}\"\n", yaml_escape(source)));
        }
    }
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

    md.push_str("## Generation\n\n");
    md.push_str("| Field | Value |\n|---|---|\n");
    md.push_str(&format!("| Command | `{}` |\n", provenance.command));
    md.push_str(&format!(
        "| Data manifest | `{}` (`{}`) |\n",
        provenance.manifest_version, provenance.manifest_path
    ));
    md.push_str(&format!(
        "| Scoring config | `{}` |\n",
        provenance.scoring_config_path
    ));
    md.push_str(&format!(
        "| Confidence | {:.2} ({}) |\n",
        scores.mean_confidence(),
        route_score::confidence_label(scores.mean_confidence())
    ));
    md.push_str(&format!(
        "| Score confidence | {:.2} ({}) |\n\n",
        scores.score_weighted_confidence(),
        route_score::confidence_label(scores.score_weighted_confidence())
    ));

    md.push_str("## Overview\n\n");
    match annotations.map(|value| value.overview.trim()) {
        Some(overview) if !overview.is_empty() => {
            md.push_str(overview);
            md.push_str("\n\n");
        }
        _ => md.push_str(
            "*[Human annotation — describe corridor function and primary economic role.]*\n\n",
        ),
    }

    // Key facts
    md.push_str("## Key Facts\n\n");
    md.push_str("| Fact | Value | Source |\n|---|---|---|\n");
    md.push_str(&format!(
        "| Total miles | {:.0} | NHS shapefile |\n",
        corridor.total_miles
    ));
    md.push_str(&format!(
        "| Mean AADT across matched segments | {} | HPMS 2023; unweighted segment mean |\n",
        attrs
            .mean_aadt
            .map(|v| format!("{v:.0}"))
            .unwrap_or("N/A".into())
    ));
    md.push_str(&format!(
        "| Mean truck % across matched segments | {} | HPMS 2023; unweighted segment mean |\n",
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
        let held = annotations.is_some_and(|value| {
            value
                .held_dimensions
                .iter()
                .any(|code| code.eq_ignore_ascii_case(sd.dim.code()))
        });
        let est = if sd.estimated || held { "†" } else { "" };
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
    md.push_str(&format!(
        "**Confidence**: mean {:.2} ({}) · score-weighted {:.2} ({})\n\n",
        scores.mean_confidence(),
        route_score::confidence_label(scores.mean_confidence()),
        scores.score_weighted_confidence(),
        route_score::confidence_label(scores.score_weighted_confidence())
    ));

    if scores.any_estimated() {
        md.push_str("† Estimated value — see score justification for details.\n\n");
    }

    render_annotation_list(
        &mut md,
        "Notable Segments",
        annotations.map(|value| value.notable_segments.as_slice()),
    );
    render_annotation_list(
        &mut md,
        "Interstate 2.0 Fit",
        annotations.map(|value| value.interstate_2_0_fit.as_slice()),
    );
    render_annotation_list(
        &mut md,
        "Flagship Claim Holds",
        annotations.map(|value| value.claim_holds.as_slice()),
    );
    render_annotation_list(
        &mut md,
        "Open Questions",
        annotations.map(|value| value.open_questions.as_slice()),
    );

    md.push_str("## Sources\n\n");
    match annotations.map(|value| value.sources.as_slice()) {
        Some(sources) if !sources.is_empty() => {
            for source in sources {
                md.push_str(&format!("- {source}\n"));
            }
        }
        _ => md.push_str("See `data/sources.md` for full citations.\n"),
    }

    md
}

fn load_annotations(output_path: &Path) -> Result<Option<CorpusAnnotations>> {
    let Some(corpus_dir) = output_path.parent().and_then(Path::parent) else {
        return Ok(None);
    };
    let Some(file_stem) = output_path.file_stem().and_then(|value| value.to_str()) else {
        return Ok(None);
    };
    let annotation_path = corpus_dir
        .join("annotations")
        .join(format!("{file_stem}.toml"));
    if !annotation_path.exists() {
        return Ok(None);
    }

    let content = std::fs::read_to_string(&annotation_path)
        .with_context(|| format!("reading {}", annotation_path.display()))?;
    let annotations: CorpusAnnotations = toml::from_str(&content)
        .with_context(|| format!("parsing {}", annotation_path.display()))?;
    if let Some(status) = annotations.status.as_deref() {
        if !matches!(
            status,
            "draft" | "reviewed" | "validated" | "deprecated" | "superseded"
        ) {
            anyhow::bail!(
                "invalid corpus annotation status '{}' in {}",
                status,
                annotation_path.display()
            );
        }
    }
    Ok(Some(annotations))
}

fn render_annotation_list(md: &mut String, heading: &str, items: Option<&[String]>) {
    md.push_str(&format!("## {heading}\n\n"));
    match items {
        Some(items) if !items.is_empty() => {
            for item in items {
                md.push_str(&format!("- {item}\n"));
            }
            md.push('\n');
        }
        _ => md.push_str("*[Human annotation.]*\n\n"),
    }
}

fn yaml_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn slug(designation: &str) -> String {
    designation.to_lowercase().replace(' ', "-")
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
        let md = format_corpus_entry_with_annotations(
            &corridor,
            &scores,
            &CorpusProvenance::default(),
            None,
        );

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
        assert!(md.contains("score-weighted"));
        assert!(md.contains("(Medium)") || md.contains("(Low)") || md.contains("(High)"));
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
        let md = format_corpus_entry_with_annotations(
            &corridor,
            &scores,
            &CorpusProvenance::default(),
            None,
        );

        assert!(md.contains("created: 2030-01-02"));
        assert!(md.contains("updated: 2030-01-02"));

        unsafe {
            std::env::remove_var("ROUTE_DATE");
        }
    }

    #[test]
    fn bundle_corpus_entry_includes_bundle_frontmatter() {
        let corridor = corridor();
        let bundle = SegmentBundle {
            segment_bundle_id: "US.HWYBUNDLE.I80".to_string(),
            bundle_role: "single-segment".to_string(),
            member_segment_ids: vec!["US.HWYSEG.I80".to_string()],
            stitch_group_ids: vec!["US.HWYSTITCH.I80".to_string()],
            current_tiers: vec!["T1".to_string()],
            current_zone_ids: vec!["national".to_string()],
            route_labels: vec!["I80".to_string()],
            state_scope: vec!["IA".to_string()],
            evidence_state_scope: vec!["IA".to_string()],
            geometry_state_scope: Vec::new(),
            bundle_aliases: vec!["route:I80".to_string()],
            source_artifacts: vec!["fixture".to_string()],
            registry_actions: vec!["eligible-for-geometry-layout".to_string()],
            validation_statuses: vec!["pass".to_string()],
            bundle_status: route_network::BundleStatus::BundleReady,
        };
        let scores = route_score::score_bundle(
            &bundle,
            &corridor.attributes,
            &route_score::ScoringConfig::default_config(),
        );

        let md = format_bundle_corpus_entry(
            &bundle,
            &corridor,
            &scores,
            &CorpusProvenance::default(),
            None,
        );

        assert!(md.contains("bundle:\n"));
        assert!(md.contains("segment_bundle_id: \"US.HWYBUNDLE.I80\""));
        assert!(md.contains("member_segment_ids: [\"US.HWYSEG.I80\"]"));
        assert!(md.contains("corridor:\n"));
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

    #[test]
    fn corpus_entry_records_generation_provenance_in_frontmatter_and_body() {
        let corridor = corridor();
        let scores = score_corridor(
            &corridor.attributes,
            &route_score::ScoringConfig::default_config(),
        );
        let provenance = CorpusProvenance {
            command: "route report I80".to_string(),
            manifest_version: "1".to_string(),
            manifest_path: "data/manifest.json".to_string(),
            scoring_config_path: "config/scoring.toml".to_string(),
        };

        let md = format_corpus_entry_with_annotations(&corridor, &scores, &provenance, None);

        assert!(md.contains("generated_by: \"route report I80\""));
        assert!(md.contains("data_manifest_version: \"1\""));
        assert!(md.contains("scoring_config_path: \"config/scoring.toml\""));
        assert!(md.contains("estimated: "));
        assert!(md.contains("confidence: "));
        assert!(md.contains("score_confidence_label: "));
        assert!(md.contains("## Generation"));
        assert!(md.contains("| Command | `route report I80` |"));
    }

    #[test]
    fn write_corpus_entry_loads_checked_annotations() {
        let dir =
            std::env::temp_dir().join(format!("route-report-annotations-{}", std::process::id()));
        let output_path = dir.join("corpus").join("existing").join("i80.md");
        let annotation_dir = dir.join("corpus").join("annotations");
        std::fs::create_dir_all(&annotation_dir).expect("create annotation directory");
        std::fs::write(
            annotation_dir.join("i80.toml"),
            r#"
overview = "Reviewed corridor overview."
status = "reviewed"
notable_segments = ["Donner remains evidence-limited."]
interstate_2_0_fit = ["Compare treatments after gap diagnosis."]
claim_holds = ["No guaranteed SLA claim."]
held_dimensions = ["A4"]
open_questions = ["What is the matched HPMS coverage?"]
sources = ["FHWA, National Highway System, https://example.com/nhs"]
"#,
        )
        .expect("write annotations");

        let corridor = corridor();
        let scores = score_corridor(
            &corridor.attributes,
            &route_score::ScoringConfig::default_config(),
        );
        write_corpus_entry(&corridor, &scores, &output_path).expect("write annotated corpus entry");

        let written = std::fs::read_to_string(&output_path).expect("read corpus entry");
        assert!(written.contains("Reviewed corridor overview."));
        assert!(written.contains("status: reviewed"));
        assert!(written.contains("Donner remains evidence-limited."));
        assert!(written.contains("## Flagship Claim Holds"));
        assert!(written.contains("No guaranteed SLA claim."));
        assert!(written.contains("| A4 | International Trade Corridor | 4.0† |"));
        assert!(written.contains("FHWA, National Highway System"));
        assert!(!written.contains("*[Human annotation"));

        let _ = std::fs::remove_dir_all(dir);
    }
}
