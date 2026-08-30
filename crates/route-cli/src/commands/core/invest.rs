//! `Invest` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    budget: f64,
    include_upgrades: bool,
    top: usize,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!(
        "route invest --budget ${budget}B{}",
        if include_upgrades {
            " --include-upgrades"
        } else {
            ""
        }
    );
    let manifest = route_data::Manifest::load(&manifest_path)
        .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
    let graph = load_graph(&manifest)?;

    // Build candidate list from graph
    let route_ids: Vec<String> = if include_upgrades {
        graph.route_ids()
    } else {
        graph.interstate_ids()
    };

    let candidates: Vec<route_network::InvestmentCandidate> = route_ids
        .iter()
        .filter_map(|id| {
            route_network::aggregate_corridor(&graph, id).map(|c| {
                route_network::InvestmentCandidate::from_corridor(
                    id,
                    &c.designation,
                    c.total_miles,
                    c.attributes.is_upgrade_candidate,
                )
            })
        })
        .collect();

    println!("  {} corridors in candidate pool", candidates.len());

    let plan = route_network::allocate_investment(&candidates, budget);

    println!("\n┌─────────────────────────────────────────────────────────────────────────┐");
    println!("│  Investment Plan — ${:.0}B budget", budget);
    println!("├─────────────────────────────────────────────────────────────────────────┤");
    println!(
        "│  Allocated: ${:.1}B of ${:.0}B budget",
        plan.allocated_b, plan.budget_b
    );
    println!(
        "│  Total throughput gain: {:.0} vehicles/day",
        plan.total_throughput_gain_vpd
    );
    println!("│  Corridors funded: {}", plan.items.len());
    println!("├──────┬───────────────┬───────────┬──────────────┬───────────────────────┤");
    println!("│ Rank │ Corridor      │    Miles  │   Cost ($B)  │ Gain (vpd)  | Type    │");
    println!("├──────┼───────────────┼───────────┼──────────────┼───────────────────────┤");

    for (i, item) in plan.items.iter().take(top).enumerate() {
        let type_label = match item.upgrade_type {
            route_network::UpgradeType::InterstateWidening => "widen  ",
            route_network::UpgradeType::UsHighwayToInterstate => "US→Int ",
            route_network::UpgradeType::StateHighwayToInterstate => "SR→Int ",
            route_network::UpgradeType::Greenfield => "new    ",
        };
        let alloc_pct = if item.allocation < 0.999 {
            format!("{:.0}%", item.allocation * 100.0)
        } else {
            "100%".to_string()
        };
        println!(
            "│ {:>4} │ {:<13} │ {:>6.0} mi │ {:>8.1} {} │ {:>10.0}  │ {} │",
            i + 1,
            item.designation,
            item.miles,
            item.cost_b,
            alloc_pct,
            item.throughput_gain_vpd,
            type_label
        );
    }
    println!("└──────┴───────────────┴───────────┴──────────────┴───────────────────────┘");
    println!(
        "\n  Costs: widen=$10M/mi, US→Int=$30M/mi, SR→Int=$40M/mi, new=$75M/mi (rough FHWA ranges)"
    );
    println!("  † Upgrade costs and throughput gains are order-of-magnitude estimates.");
    println!("  † Run `route score-all` to improve gain estimates with real AADT data.");

    Ok(())
}
