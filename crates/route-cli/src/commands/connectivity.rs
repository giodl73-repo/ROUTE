//! `Connectivity` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    all_pairs: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route connectivity (T1 network completeness test)");
    let manifest = route_data::Manifest::load(&manifest_path)
        .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
    let graph = load_graph(&manifest)?;

    println!(
        "  analyzing T1 network ({} T1 corridors, {} total routes)…",
        graph
            .interstate_ids()
            .iter()
            .filter(|id| route_network::T1_BACKBONE_ROUTES.contains(&id.as_str()))
            .count(),
        graph.route_ids().len()
    );

    let report = route_network::analyze_t1_connectivity(&graph);

    println!("\n┌─────────────────────────────────────────────────────────────┐");
    println!("│  T1 Network Connectivity Report");
    println!("├─────────────────────────────────────────────────────────────┤");
    println!(
        "│  T1 endpoints analyzed:    {:>4}                            │",
        report.endpoints.len()
    );
    println!(
        "│  Endpoint pairs tested:    {:>4}                            │",
        report.pair_results.len()
    );
    println!(
        "│  Gaps (require T2 bridge): {:>4}                            │",
        report.gaps.len()
    );
    println!(
        "│  Network fully connected:  {}                         │",
        if report.is_fully_connected {
            "YES ✓"
        } else {
            "NO  ✗ — GAPS FOUND"
        }
    );
    println!("└─────────────────────────────────────────────────────────────┘");

    if !report.gaps.is_empty() {
        println!("\n  STRUCTURAL GAPS — endpoint pairs requiring T2 to connect:");
        println!("  {:12} → {:12}  T1 miles  All miles  Detour", "From", "To");
        println!("  {}", "─".repeat(60));
        for gap in &report.gaps {
            let t1 = gap
                .t1_only_miles
                .map(|m| format!("{m:.0}"))
                .unwrap_or("NONE".into());
            let all = gap
                .all_corridors_miles
                .map(|m| format!("{m:.0}"))
                .unwrap_or("—".into());
            let det = gap
                .detour_factor
                .map(|d| format!("{d:.1}×"))
                .unwrap_or("∞".into());
            let flag = if gap.requires_t2 {
                " ← T2 required!"
            } else {
                ""
            };
            println!(
                "  {:12} → {:12}  {:>8}  {:>8}  {:>6}{}",
                gap.from_route, gap.to_route, t1, all, det, flag
            );
        }
        println!(
            "\n  Isolated terminals: {}",
            report.isolated_terminals.join(", ")
        );
        println!("\n  → These gaps are I2.0 missing link targets:");
        println!(
            "    A new T1 corridor filling each gap would close the structural disconnect."
        );
        println!("    Example: I-40/I-70 western endpoint → I-5 requires I-15 (T2).");
        println!(
            "    A Pacific extension of I-40/I-70 (via US-50 alignment) would close it."
        );
    }

    if all_pairs {
        println!("\n  All T1 endpoint pairs:");
        println!("  {:12} → {:12}  T1-only   All-crdr  Detour", "From", "To");
        println!("  {}", "─".repeat(65));
        let mut pairs = report.pair_results.clone();
        pairs.sort_by(|a, b| {
            b.detour_factor
                .unwrap_or(0.0)
                .total_cmp(&a.detour_factor.unwrap_or(0.0))
        });
        for r in pairs.iter().take(20) {
            let t1 = r
                .t1_only_miles
                .map(|m| format!("{m:.0}mi"))
                .unwrap_or("UNREACHABLE".into());
            let all = r
                .all_corridors_miles
                .map(|m| format!("{m:.0}mi"))
                .unwrap_or("—".into());
            let det = r
                .detour_factor
                .map(|d| format!("{d:.2}×"))
                .unwrap_or("∞".into());
            println!(
                "  {:12} → {:12}  {:>12}  {:>9}  {:>6}",
                r.from_route, r.to_route, t1, all, det
            );
        }
    }
        
    Ok(())
}

