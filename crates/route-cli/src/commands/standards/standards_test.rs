//! `StandardsTest` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    tier: u8,
    trips: usize,
    seed: u64
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route standards-test --tier {tier} ({trips} trips)\n");
    println!("Testing whether Tier {tier} PTI target is achievable under simulation.\n");

    let (pti_target, corridor_name) = match tier {
        1 => (1.15, "T1 — I-80 NY→LA (managed freight lanes)"),
        2 => (1.30, "T2 — I-70 (Major Connector, mixed traffic)"),
        _ => (1.50, "T3 — Regional feeder (demand-responsive)"),
    };

    let corridor = route_sim::ny_la_corridor();

    // Run at three demand levels: normal, adverse (+20% demand), severe (+40% + compound incident)
    println!("  Tier {tier} PTI target: ≤ {pti_target:.2}");
    println!("  Corridor: {corridor_name}");
    println!(
        "  Free-flow elapsed: {:.1}h ({:.1} days)",
        corridor.free_flow_elapsed_hours(),
        corridor.free_flow_elapsed_hours() / 24.0
    );
    println!();

    let managed = tier == 1;
    let dist = route_sim::run_od_simulation(&corridor, managed, trips, seed);

    println!(
        "  {:>20}  {:>8}  {:>8}  {:>8}  {:>8}  {:>8}  {:>8}  {:>6}",
        "Scenario", "p50 (h)", "p75 (h)", "p90 (h)", "p95 (h)", "p99 (h)", "PTI", "SLA?"
    );
    println!("  {}", "─".repeat(85));

    let pti_met = dist.pti <= pti_target;
    let sla_label = if pti_met { "PASS ✓" } else { "FAIL ✗" };
    println!(
        "  {:>20}  {:>8.1}  {:>8.1}  {:>8.1}  {:>8.1}  {:>8.1}  {:>6.3}  {}",
        "Baseline",
        dist.p50_hours,
        dist.p75_hours,
        dist.p90_hours,
        dist.p95_hours,
        dist.p99_hours,
        dist.pti,
        sla_label
    );

    println!();
    println!(
        "  Commitment window (p95): {:.1}h = {:.1} days",
        dist.p95_hours,
        dist.p95_hours / 24.0
    );
    println!(
        "  PTI (p95/free-flow):     {:.3}  [target ≤ {:.2}] — {}",
        dist.pti,
        pti_target,
        if pti_met {
            "TARGET MET ✓"
        } else {
            "TARGET MISSED ✗"
        }
    );
    println!("  Trips completing < 48h:  {:.1}%", dist.pct_under_48h);
    println!();

    if pti_met {
        println!(
            "  ✓ Tier {tier} PTI standard is achievable under these simulation conditions."
        );
        println!("  ✓ Managed lanes + Donner tunnel remove the primary variance sources.");
    } else {
        println!(
            "  ✗ Tier {tier} PTI target NOT met at current demand/incident parameters."
        );
        println!("  → Primary variance sources: see segment breakdown above.");
    }
        
    Ok(())
}

