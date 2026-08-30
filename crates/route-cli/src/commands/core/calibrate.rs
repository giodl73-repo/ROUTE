//! `Calibrate` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(ctx: &ctx::Ctx<'_>) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route calibrate — rubric calibration pass (v1.4)");
    let manifest = route_data::Manifest::load(&manifest_path)
        .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
    let mut graph = load_graph(&manifest)?;
    let ids = atlas_candidate_ids(&graph);
    println!("  scoring {} corridors for calibration…", ids.len());

    // Compute betweenness centrality so B2 is populated (same as score-all)
    println!("  computing betweenness centrality…");
    let bc_raw = route_network::centrality::compute_edge_betweenness(&graph);
    println!("  centrality: {} edges scored", bc_raw.len());
    // Normalize using P95 (not max) to prevent outlier edges from compressing distribution
    // A single hyper-central junction edge can be 100× larger than trunk route edges
    let mut vals_sorted: Vec<f64> = bc_raw.values().copied().filter(|v| v.is_finite()).collect();
    vals_sorted.sort_by(f64::total_cmp);
    let p95_idx =
        ((vals_sorted.len() as f64 * 0.95) as usize).min(vals_sorted.len().saturating_sub(1));
    let bc_norm = vals_sorted.get(p95_idx).cloned().unwrap_or(1.0).max(1.0);
    let bc = bc_raw
        .into_iter()
        .map(|(k, v)| (k, (v / bc_norm).min(1.0)))
        .collect();
    graph.edge_betweenness = Some(bc);

    // Load ACS population once for C1/C2/C3 wiring
    let acs_counties = load_acs_counties_for_scoring(&manifest);
    if acs_counties.is_some() {
        println!("  ACS population data loaded — C1/C2/C3 will use real values");
    }
    // Load ports for B3 scoring
    let ports = load_ports();
    if !ports.is_empty() {
        println!(
            "  {} port/border locations loaded — B3 will use real values",
            ports.len()
        );
    }
    // Load DCFC stations for D2 scoring (partial — DEMO_KEY rate limit)
    let dcfc = load_dcfc_stations();
    if !dcfc.is_empty() {
        println!(
            "  {} DCFC stations loaded — D2 EV component will use real values",
            dcfc.len()
        );
    }
    // Load intermodal terminals for D2 hub count
    let intermodal = load_intermodal_terminals();
    if !intermodal.is_empty() {
        println!(
            "  {} intermodal terminals loaded — D2 hub count will use real values",
            intermodal.len()
        );
    }
    // Load NBI bridge condition data for D3
    let nbi = load_nbi_bridges();
    if !nbi.is_empty() {
        println!(
            "  {} NBI bridge records loaded — D3 will use real condition data",
            nbi.len()
        );
    }
    // Load FEMA SFHA tile counts for D1 scoring
    let fema_tiles = load_fema_tiles();
    if !fema_tiles.is_empty() {
        println!(
            "  {} FEMA SFHA tiles loaded — D1 will use real flood-zone data",
            fema_tiles.len()
        );
    }
    // Load FARS fatal crash rates for A5 scoring
    let fars_safety = load_fars_safety();
    if !fars_safety.is_empty() {
        println!(
            "  {} FARS route records loaded — A5 will use real safety data",
            fars_safety.len()
        );
    }
    // Load railroad parallel data for B1 discount
    let railroad_parallels = load_railroad_parallels();
    if !railroad_parallels.is_empty() {
        println!(
            "  {} railroad parallels loaded — B1 rail discount applied",
            railroad_parallels.len()
        );
    }
    // Load multi-hazard zones for D1 extension
    let hazard_zones = load_hazard_zones();
    if !hazard_zones.is_empty() {
        println!(
            "  {} hazard zone records loaded — D1 multi-hazard composite active",
            hazard_zones.len()
        );
    }

    // Collect per-dimension scores for all corridors
    const N_DIMS: usize = 16;
    let dim_names = [
        "A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "C1", "C2", "C3", "C4", "D1", "D2",
        "D3",
    ];
    let dim_labels = [
        "Throughput Gap",
        "Freight Intensity",
        "Speed Reliability",
        "International Trade",
        "Safety Record",
        "Redundancy",
        "Network Centrality",
        "Port/Border Access",
        "Military/Strategic",
        "Population Reach",
        "Rural Connectivity",
        "Economic Opportunity",
        "Agricultural Export",
        "Climate Resilience",
        "Multimodal Integration",
        "Infrastructure Vintage",
    ];

    let mut matrix: Vec<[f64; N_DIMS]> = Vec::new();
    let mut estimated_matrix: Vec<[bool; N_DIMS]> = Vec::new();
    let mut confidence_matrix: Vec<[f32; N_DIMS]> = Vec::new();
    let mut route_ids_used: Vec<String> = Vec::new();
    let mut total_scores: Vec<f64> = Vec::new();
    let mut flagged_congestion: Vec<(String, f64, f64)> = Vec::new(); // (route, A1, B2)
    let mut confidence_risks: Vec<ConfidenceRisk> = Vec::new();
    let mut dimension_risk_totals = [0.0_f64; N_DIMS];
    let mut dimension_review_risk_totals = [0.0_f64; N_DIMS];
    let mut dimension_risk_counts = [0_usize; N_DIMS];
    let mut dimension_review_counts = [0_usize; N_DIMS];

    for id in &ids {
        if let Some(mut corridor) = route_network::aggregate_corridor(&graph, id) {
            // Join ACS population for C1/C2/C3
            if acs_counties.is_some() {
                join_acs_population_to_corridor(
                    &manifest,
                    &graph,
                    id,
                    &mut corridor.attributes,
                    false,
                );
            }
            // Join port access for B3
            if !ports.is_empty() {
                join_port_access_to_corridor(&graph, id, &mut corridor.attributes, &ports);
            }
            // Join DCFC for D2 EV component
            if !dcfc.is_empty() {
                join_dcfc_to_corridor(
                    &graph,
                    id,
                    corridor.total_miles,
                    &mut corridor.attributes,
                    &dcfc,
                );
            }
            // Join intermodal terminals for D2 hub count
            if !intermodal.is_empty() {
                join_intermodal_to_corridor(&graph, id, &mut corridor.attributes, &intermodal);
            }
            // Join FEMA SFHA tile counts for D1 flood-zone scoring
            if !fema_tiles.is_empty() {
                join_fema_d1_to_corridor(&graph, id, &mut corridor.attributes, &fema_tiles);
            }
            // Apply D3 IRI proxy when NBI bridge data is unavailable
            // D3: join NBI real data first; fall back to IRI proxy if unavailable
            if !nbi.is_empty() {
                join_nbi_to_corridor(id, &mut corridor.attributes, &nbi);
            }
            join_d3_iri_proxy(&mut corridor.attributes); // no-op if NBI already set
                                                         // A5: join FARS fatal crash rate
            if let Some(&rate) = fars_safety.get(id) {
                corridor.attributes.fatal_crash_rate = Some(rate);
            }
            // B1: join railroad parallel flag
            if let Some(railroad) = railroad_parallels.get(id) {
                corridor.attributes.rail_parallel_flag = true;
                corridor.attributes.rail_parallel_name = Some(railroad.clone());
            }
            // D1: join multi-hazard zone data
            if let Some(zone) = hazard_zones.get(id) {
                corridor.attributes.wildfire_risk = Some(zone.wildfire);
                corridor.attributes.tornado_risk = Some(zone.tornado);
                corridor.attributes.seismic_risk = Some(zone.seismic);
            }
            join_a2_freight_proxy(&mut corridor.attributes, corridor.total_miles);
            let s = route_score::score_corridor(&corridor.attributes, &scoring_cfg);
            let row = dimension_score_values(&s);
            let estimated_row = dimension_estimated_values(&s);
            let confidence_row = dimension_confidence_values(&s);
            let total = rounded_score(s.total());
            let tier = tier_for_score(total);
            let review = total >= T2_THRESHOLD && s.score_weighted_confidence() < 0.75;
            // Flag congestion-stress candidates: high A1 + low B2 + total near T1 threshold
            if s.a1.score > 7.0 && s.b2.score < 3.0 && total > 20.0 {
                flagged_congestion.push((id.clone(), s.a1.score, s.b2.score));
            }
            for (d, risk) in dimension_confidence_risks(&row, &confidence_row)
                .iter()
                .enumerate()
            {
                if *risk >= 1.0 {
                    dimension_risk_totals[d] += risk;
                    dimension_risk_counts[d] += 1;
                    if review {
                        dimension_review_risk_totals[d] += risk;
                        dimension_review_counts[d] += 1;
                    }
                }
            }
            let risk_dimensions = confidence_risk_dimensions(&row, &confidence_row);
            confidence_risks.push(ConfidenceRisk {
                route: id.clone(),
                score: total,
                tier,
                mean_confidence: s.mean_confidence(),
                score_confidence: s.score_weighted_confidence(),
                risk_dimensions,
            });
            matrix.push(row);
            estimated_matrix.push(estimated_row);
            confidence_matrix.push(confidence_row);
            route_ids_used.push(id.clone());
            total_scores.push(total);
        }
    }

    let n = matrix.len() as f64;
    println!("  {} corridors scored\n", matrix.len());

    // Per-dimension statistics
    println!("┌───────────────────────────────────────────────────────────────────────────────────────────────────┐");
    println!("│  Dimension Statistics (0.0–10.0 scale, n={})                                                    │", matrix.len());
    println!("├──────┬────────────────────────────┬──────┬──────┬──────┬──────┬──────┬──────┬──────┬──────┬────────────  ┤");
    println!("│  Dim │  Name                      │  Min │  Max │  Avg │  Std │  P90 │ Conf │ Est% │ NZ%  │  Status      │");
    println!("├──────┼────────────────────────────┼──────┼──────┼──────┼──────┼──────┼──────┼──────┼──────┼────────────  ┤");

    let mut dim_stats: Vec<(f64, f64, f64, f64, f64, f64, f64)> = Vec::new(); // min,max,mean,std,p90,est_rate,nonzero_rate

    for d in 0..N_DIMS {
        let vals: Vec<f64> = matrix.iter().map(|r| r[d]).collect();
        let estimated_count = estimated_matrix.iter().filter(|r| r[d]).count();
        let avg_conf = confidence_matrix.iter().map(|r| r[d] as f64).sum::<f64>() / n;
        let est_rate = estimated_count as f64 / n;
        let nonzero_rate = vals.iter().filter(|&&v| v > 0.0).count() as f64 / n;
        let min = vals.iter().cloned().fold(f64::MAX, f64::min);
        let max = vals.iter().cloned().fold(f64::MIN, f64::max);
        let mean = vals.iter().sum::<f64>() / n;
        let variance = vals.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
        let std = variance.sqrt();
        let mut sorted = vals.clone();
        sorted.sort_by(f64::total_cmp);
        let p90 = sorted[((n * 0.90) as usize).min(sorted.len() - 1)];

        // Status flags
        let status = if est_rate >= 0.80 {
            "PROXY GAP ⚠"
        } else if nonzero_rate < 0.10 && max >= 8.0 {
            "SPARSE ROLE"
        } else if std < 1.5 {
            "LOW VAR ⚠"
        } else if max - min < 3.0 {
            "NARROW  ⚠"
        } else {
            "OK      ✓"
        };

        println!("│  {:>2}  │  {:<26} │ {:>4.1} │ {:>4.1} │ {:>4.1} │ {:>4.1} │ {:>4.1} │ {:>4.2} │ {:>4.0} │ {:>4.0} │  {:<10}  │",
            dim_names[d], dim_labels[d], min, max, mean, std, p90, avg_conf, est_rate * 100.0, nonzero_rate * 100.0, status);
        dim_stats.push((min, max, mean, std, p90, est_rate, nonzero_rate));
    }
    println!("└──────┴────────────────────────────┴──────┴──────┴──────┴──────┴──────┴──────┴──────┴──────┴────────────  ┘");

    // Pairwise correlation (Pearson) — flag pairs > 0.60
    println!("\n  Computing pairwise Pearson correlations…");
    let means: Vec<f64> = (0..N_DIMS)
        .map(|d| matrix.iter().map(|r| r[d]).sum::<f64>() / n)
        .collect();
    let stds: Vec<f64> = dim_stats.iter().map(|s| s.3).collect();

    let mut high_corr: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..N_DIMS {
        for j in (i + 1)..N_DIMS {
            if stds[i] < 0.01 || stds[j] < 0.01 {
                continue;
            }
            let cov: f64 = matrix
                .iter()
                .map(|r| (r[i] - means[i]) * (r[j] - means[j]))
                .sum::<f64>()
                / n;
            let r = cov / (stds[i] * stds[j]);
            if r.abs() > 0.55 {
                high_corr.push((i, j, r));
            }
        }
    }
    high_corr.sort_by(|a, b| b.2.abs().total_cmp(&a.2.abs()));

    if !high_corr.is_empty() {
        println!("\n  High-correlation pairs (|r| > 0.55):");
        println!("  {:>2} × {:>2}   r       Status", "D1", "D2");
        println!("  {}", "─".repeat(50));
        for (i, j, r) in &high_corr {
            let warn = if r.abs() > 0.70 {
                " ⚠ REDUNDANT?"
            } else {
                ""
            };
            println!(
                "  {} × {}  {:>+5.2}  {}{}",
                dim_names[*i], dim_names[*j], r, "", warn
            );
        }
    } else {
        println!("  No high-correlation pairs found (all |r| ≤ 0.55) ✓");
    }

    // Congestion-stress paradox candidates
    if !flagged_congestion.is_empty() {
        println!("\n  Congestion-stress candidates (high A1, low B2, near T1):");
        println!("  {:>8}  {:>6}  {:>6}", "Route", "A1", "B2");
        println!("  {}", "─".repeat(30));
        for (route, a1, b2) in &flagged_congestion {
            println!(
                "  {:>8}  {:>6.1}  {:>6.1}  ⚠ urban connector inflation",
                route, a1, b2
            );
        }
        println!("  → These corridors may need centrality-adjusted tier classification.");
        println!("    See A.1 paper: betweenness centrality correction (α=0.65).");
    }

    // Tier distribution
    let t1 = total_scores.iter().filter(|&&s| s >= T1_THRESHOLD).count();
    let t2 = total_scores
        .iter()
        .filter(|&&s| s >= T2_THRESHOLD && s < T1_THRESHOLD)
        .count();
    let t3 = total_scores
        .iter()
        .filter(|&&s| s >= T3_THRESHOLD && s < T2_THRESHOLD)
        .count();
    let t4 = total_scores.iter().filter(|&&s| s < T3_THRESHOLD).count();
    println!(
        "\n  Tier distribution (v1.4 thresholds: T1≥{T1_THRESHOLD:.1}, T2≥{T2_THRESHOLD:.1}, T3≥{T3_THRESHOLD:.1}):"
    );
    println!(
        "    T1: {} corridors  T2: {} corridors  T3: {} corridors  T4: {} corridors",
        t1, t2, t3, t4
    );
    if t1 > 30 {
        println!(
            "    ⚠ T1 count {} exceeds promotion atlas target range (~12-30).",
            t1
        );
        println!(
            "    → Review congestion-stress candidates and promotion thresholds before freezing a release."
        );
    }

    confidence_risks.sort_by(|a, b| {
        a.score_confidence
            .total_cmp(&b.score_confidence)
            .then_with(|| b.score.total_cmp(&a.score))
            .then_with(|| a.route.cmp(&b.route))
    });
    println!("\n  Lowest score-weighted confidence corridors:");
    println!(
        "  {:>8}  {:>6}  {:>4}  {:>7}  {:>10}  {}",
        "Route", "Score", "Tier", "Conf", "ScoreConf", "RiskDims"
    );
    println!("  {}", "─".repeat(78));
    for risk in confidence_risks.iter().take(12) {
        let flag = if risk.score >= T2_THRESHOLD && risk.score_confidence < 0.75 {
            "  ⚠ review"
        } else {
            ""
        };
        println!(
            "  {:>8}  {:>6.1}  {:>4}  {:>7.2}  {:>10.2}  {:<24}{}",
            risk.route,
            risk.score,
            risk.tier,
            risk.mean_confidence,
            risk.score_confidence,
            risk.risk_dimensions,
            flag
        );
    }
    println!(
        "  → Sort by score_confidence and risk_dimensions to find rankings most dependent on weak dimensions."
    );

    let mut dimension_risks: Vec<(usize, f64, f64, usize, usize)> = (0..N_DIMS)
        .map(|d| {
            (
                d,
                dimension_risk_totals[d],
                dimension_review_risk_totals[d],
                dimension_risk_counts[d],
                dimension_review_counts[d],
            )
        })
        .collect();
    dimension_risks.sort_by(|a, b| {
        b.2.total_cmp(&a.2)
            .then_with(|| b.1.total_cmp(&a.1))
            .then_with(|| b.4.cmp(&a.4))
            .then_with(|| DIMENSION_CODES[a.0].cmp(DIMENSION_CODES[b.0]))
    });

    println!("\n  Confidence risk by dimension:");
    println!(
        "  {:>2}  {:<28}  {:>9}  {:>10}  {:>9}  {:>9}",
        "Dim", "Name", "Risk", "ReviewRisk", "Corridors", "Reviews"
    );
    println!("  {}", "─".repeat(78));
    for (d, total_risk, review_risk, corridor_count, review_count) in dimension_risks.iter().take(8)
    {
        println!(
            "  {:>2}  {:<28}  {:>9.1}  {:>10.1}  {:>9}  {:>9}",
            dim_names[*d], dim_labels[*d], total_risk, review_risk, corridor_count, review_count
        );
    }

    std::fs::create_dir_all("data")?;
    let risk_path = PathBuf::from("data/confidence-risks.csv");
    let mut risk_wtr = csv::Writer::from_path(&risk_path)?;
    risk_wtr.write_record([
        "route",
        "score",
        "tier",
        "confidence",
        "score_confidence",
        "confidence_label",
        "score_confidence_label",
        "review",
        "risk_dimensions",
    ])?;
    for risk in &confidence_risks {
        let review = risk.score >= T2_THRESHOLD && risk.score_confidence < 0.75;
        risk_wtr.write_record([
            risk.route.clone(),
            format!("{:.1}", risk.score),
            risk.tier.to_string(),
            format!("{:.2}", risk.mean_confidence),
            format!("{:.2}", risk.score_confidence),
            route_score::confidence_label(risk.mean_confidence).to_string(),
            route_score::confidence_label(risk.score_confidence).to_string(),
            review.to_string(),
            risk.risk_dimensions.clone(),
        ])?;
    }
    risk_wtr.flush()?;
    println!("  wrote confidence risk ledger → {}", risk_path.display());

    let summary_path = PathBuf::from("data/confidence-risk-summary.csv");
    let mut summary_wtr = csv::Writer::from_path(&summary_path)?;
    summary_wtr.write_record([
        "dimension",
        "name",
        "total_risk",
        "review_risk",
        "corridors",
        "review_corridors",
    ])?;
    for (d, total_risk, review_risk, corridor_count, review_count) in &dimension_risks {
        summary_wtr.write_record([
            dim_names[*d].to_string(),
            dim_labels[*d].to_string(),
            format!("{total_risk:.1}"),
            format!("{review_risk:.1}"),
            corridor_count.to_string(),
            review_count.to_string(),
        ])?;
    }
    summary_wtr.flush()?;
    println!(
        "  wrote confidence risk summary → {}",
        summary_path.display()
    );

    // Retirement candidates
    println!("\n  Retirement candidates (std < 1.5, estimated < 80%):");
    let mut any_retire = false;
    for d in 0..N_DIMS {
        let (_, max, _, std, _, est_rate, nonzero_rate) = dim_stats[d];
        let sparse_role = nonzero_rate < 0.10 && max >= 8.0;
        if std < 1.5 && est_rate < 0.80 && !sparse_role {
            println!(
                "    {} ({}) — std={:.2} — consider retiring or merging",
                dim_names[d], dim_labels[d], std
            );
            any_retire = true;
        }
    }
    if !any_retire {
        println!("    None — all dimensions show adequate variance ✓");
    }

    println!("\n  Sparse role dimensions (nonzero < 10%, max ≥ 8):");
    let mut any_sparse = false;
    for d in 0..N_DIMS {
        let (_, max, _, std, _, est_rate, nonzero_rate) = dim_stats[d];
        if est_rate < 0.80 && nonzero_rate < 0.10 && max >= 8.0 {
            println!(
                "    {} ({}) — nonzero={:.0}% std={:.2} — keep if this is an intentional role flag; expand source list if not",
                dim_names[d],
                dim_labels[d],
                nonzero_rate * 100.0,
                std
            );
            any_sparse = true;
        }
    }
    if !any_sparse {
        println!("    None — no sparse high-ceiling role dimensions ✓");
    }

    println!("\n  Data/proxy gaps (estimated ≥ 80%):");
    let mut any_proxy_gap = false;
    for d in 0..N_DIMS {
        let (_, _, _, std, _, est_rate, _) = dim_stats[d];
        if est_rate >= 0.80 {
            println!(
                "    {} ({}) — estimated={:.0}% std={:.2} — improve source coverage before retirement decisions",
                dim_names[d],
                dim_labels[d],
                est_rate * 100.0,
                std
            );
            any_proxy_gap = true;
        }
    }
    if !any_proxy_gap {
        println!("    None — all dimensions have adequate source coverage ✓");
    }

    println!("\ncalibrate complete. Review output above before bumping rubric version.");
    Ok(())
}
