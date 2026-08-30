//! `TierPavementUnmatchedJoinReview` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    fetch_review: PathBuf,
    source_gaps: PathBuf,
    pavement_docket: PathBuf,
    cache_dir: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-unmatched-join-review");
    let fetch_review_rows = load_tier_pavement_source_fetch_review(&fetch_review)
        .with_context(|| format!("loading {}", fetch_review.display()))?;
    let source_gap_rows = load_tier_pavement_source_gaps(&source_gaps)
        .with_context(|| format!("loading {}", source_gaps.display()))?;
    let docket_rows = load_tier_pavement_docket(&pavement_docket)
        .with_context(|| format!("loading {}", pavement_docket.display()))?;
    let rows = tier_pavement_unmatched_join_review_rows(
        &fetch_review_rows,
        &source_gap_rows,
        &docket_rows,
        &cache_dir,
    )?;
    write_tier_pavement_unmatched_join_review(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_unmatched_join_review_summary(&output, &rows);

    if gate {
        let failures = tier_pavement_unmatched_join_review_gate_failures(&rows, &fetch_review_rows);
        if !failures.is_empty() {
            println!();
            println!("Tier pavement unmatched join review gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement unmatched join review gate failed");
        }
        println!();
        println!("Tier pavement unmatched join review gate: PASS");
    }

    Ok(())
}
