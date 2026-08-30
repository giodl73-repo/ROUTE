//! `T2GamePublicationEvidenceReview` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    claim_review: PathBuf,
    scenario_hooks: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-game-publication-evidence-review");
    let claim_rows = load_optimizer_claim_review(&claim_review)
        .with_context(|| format!("loading {}", claim_review.display()))?;
    let hook_rows = load_t2_scenario_hooks(&scenario_hooks)
        .with_context(|| format!("loading {}", scenario_hooks.display()))?;
    let rows = t2_game_publication_evidence_review_rows(&claim_rows, &hook_rows);
    write_t2_game_publication_evidence_review(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_game_publication_evidence_review_summary(&output, &rows);

    if gate {
        let failures =
            t2_game_publication_evidence_review_gate_failures(&rows, &claim_rows, &hook_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 game publication evidence review gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 game publication evidence review gate failed");
        }
        println!();
        println!("T2 game publication evidence review gate: PASS");
    }

    Ok(())
}
