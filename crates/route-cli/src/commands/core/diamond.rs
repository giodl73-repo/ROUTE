//! `Diamond` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(ctx: &ctx::Ctx<'_>, at: String) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let manifest = route_data::Manifest::load(&manifest_path)
        .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
    let graph = load_graph(&manifest)?;

    if at.to_uppercase() == "ALL" {
        // Analyze all T1/T1 intersections
        let intersections = route_network::find_t1_intersections(&graph);
        println!(
            "route diamond all  ({} T1/T1 intersections found)",
            intersections.len()
        );
        println!(
            "\n  {:25} {:>4}  {:>6}  {:>10}  {:>6}",
            "Intersection", "k", "SPF?", "Cost ($B)", "Connectors"
        );
        println!("  {}", "─".repeat(60));
        let mut results: Vec<_> = intersections
            .into_iter()
            .map(|ix| route_network::analyze_diamond(&graph, ix))
            .collect();
        results.sort_by_key(|r| r.k_current);
        for r in &results {
            let spf = if r.is_spf { "YES ⚠" } else { "no " };
            println!(
                "  {:25} {:>4}  {:>6}  {:>9.2}B  {:>6}",
                r.intersection.name, r.k_current, spf, r.est_cost_b, r.connectors_needed
            );
        }
        let spf_count = results.iter().filter(|r| r.is_spf).count();
        println!(
            "\n  Single points of failure: {}/{}",
            spf_count,
            results.len()
        );
        println!(
            "  Total diamond investment needed: ${:.1}B",
            results.iter().map(|r| r.est_cost_b).sum::<f64>()
        );
    } else {
        // Analyze one specific intersection
        println!("route diamond {at}");
        let intersection = route_network::find_intersection(&graph, &at).ok_or_else(|| {
            anyhow::anyhow!(
                "No T1/T1 intersection found matching '{}'. Try 'route diamond all' to list all.",
                at
            )
        })?;
        println!(
            "  Found: {} ({:.2}°N {:.2}°W)",
            intersection.name, intersection.lat, -intersection.lon
        );
        let result = route_network::analyze_diamond(&graph, intersection);
        println!("\n┌─────────────────────────────────────────────────────┐");
        println!("│  {} Diamond Analysis", result.intersection.name);
        println!("├─────────────────────────────────────────────────────┤");
        println!(
            "│  k-connectivity (current):  {:>3}                    │",
            result.k_current
        );
        println!(
            "│  Single point of failure:   {}                  │",
            if result.is_spf { "YES ⚠" } else { "no" }
        );
        println!(
            "│  Zone nodes (route A):      {:>3}                    │",
            result.zone_nodes_a.len()
        );
        println!(
            "│  Zone nodes (route B):      {:>3}                    │",
            result.zone_nodes_b.len()
        );
        println!(
            "│  Connectors needed (→k≥3): {:>3}                    │",
            result.connectors_needed
        );
        println!(
            "│  Estimated cost:           ${:.2}B                 │",
            result.est_cost_b
        );
        println!("└─────────────────────────────────────────────────────┘");

        if result.is_spf {
            println!("\n  ⚠ This is a single point of failure.");
            println!(
                "  A closure here disrupts both {} and {} simultaneously.",
                result.intersection.route_a, result.intersection.route_b
            );
            println!(
                "  Adding {} connector road(s) within 50 miles would bring k to ≥3.",
                result.connectors_needed
            );
        } else {
            println!(
                "\n  This intersection has adequate path redundancy (k={}).",
                result.k_current
            );
        }
    }

    Ok(())
}
