//! `TierSegmentCandidates` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    t1_selector: PathBuf,
    t2_service_selection: PathBuf,
    t2_bundle_repair_queue: PathBuf,
    t2_route_family_splits: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-segment-candidates");
    let manifest = route_data::Manifest::load(&manifest_path)
        .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
    let graph = load_graph(&manifest)?;
    let t1_rows = load_t1_line_selector(&t1_selector)
        .with_context(|| format!("loading {}", t1_selector.display()))?;
    let t2_rows = load_t2_service_selection(&t2_service_selection)
        .with_context(|| format!("loading {}", t2_service_selection.display()))?;
    let repair_rows = load_t2_bundle_repair_queue(&t2_bundle_repair_queue)
        .with_context(|| format!("loading {}", t2_bundle_repair_queue.display()))?;
    let route_family_rows = load_t2_route_family_splits(&t2_route_family_splits)
        .with_context(|| format!("loading {}", t2_route_family_splits.display()))?;
    let rows = support::tier::tier_segment_candidate_rows::tier_segment_candidate_rows(
        &graph,
        &t1_rows,
        &t2_rows,
        &repair_rows,
        &route_family_rows,
    );
    write_tier_segment_candidates(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_segment_candidate_summary(&output, &rows);

    if gate {
        let failures =
            tier_segment_candidate_gate_failures(&rows, &t1_rows, &t2_rows, &repair_rows);
        if !failures.is_empty() {
            println!();
            println!("Tier segment candidate gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier segment candidate gate failed");
        }
        println!();
        println!("Tier segment candidate gate: PASS");
    }

    Ok(())
}
