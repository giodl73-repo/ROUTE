//! `T2GameOpsBundleEvidencePolicy` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    review: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-game-ops-bundle-evidence-policy");
    let review_rows = load_t2_game_ops_bundle_evidence_review(&review)
        .with_context(|| format!("loading {}", review.display()))?;
    let rows = t2_game_ops_bundle_evidence_policy_rows(&review_rows);
    write_t2_game_ops_bundle_evidence_policy(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_game_ops_bundle_evidence_policy_summary(&output, &rows);

    if gate {
        let failures =
            t2_game_ops_bundle_evidence_policy_gate_failures(&rows, &review_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 game/ops bundle evidence policy gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 game/ops bundle evidence policy gate failed");
        }
        println!();
        println!("T2 game/ops bundle evidence policy gate: PASS");
    }
        
    Ok(())
}

