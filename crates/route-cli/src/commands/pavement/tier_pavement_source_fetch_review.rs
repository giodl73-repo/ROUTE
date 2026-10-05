//! `TierPavementSourceFetchReview` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    fetch_attempt: PathBuf,
    acquisition_docket: PathBuf,
    source_gaps: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-source-fetch-review");
    let fetch_attempt_rows = load_tier_pavement_source_fetch_attempt(&fetch_attempt)
        .with_context(|| format!("loading {}", fetch_attempt.display()))?;
    let docket_rows = load_tier_pavement_acquisition_docket(&acquisition_docket)
        .with_context(|| format!("loading {}", acquisition_docket.display()))?;
    let source_gap_rows = load_tier_pavement_source_gaps(&source_gaps)
        .with_context(|| format!("loading {}", source_gaps.display()))?;
    let rows =
        tier_pavement_source_fetch_review_rows(&fetch_attempt_rows, &docket_rows, &source_gap_rows);
    write_tier_pavement_source_fetch_review(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_source_fetch_review_summary(&output, &rows);

    if gate {
        let failures = tier_pavement_source_fetch_review_gate_failures(
            &rows,
            &fetch_attempt_rows,
            &docket_rows,
        );
        if !failures.is_empty() {
            println!();
            println!("Tier pavement source fetch review gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement source fetch review gate failed");
        }
        println!();
        println!("Tier pavement source fetch review gate: PASS");
    }

    Ok(())
}
