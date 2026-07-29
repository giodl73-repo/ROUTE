//! `T2GameOpsBundleEvidenceReview` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    decisions: PathBuf,
    targets: PathBuf,
    service_docket: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-game-ops-bundle-evidence-review");
    let decision_rows = load_t2_game_ops_binding_decisions(&decisions)
        .with_context(|| format!("loading {}", decisions.display()))?;
    let target_rows = load_t2_bundle_overlay_repair_targets(&targets)
        .with_context(|| format!("loading {}", targets.display()))?;
    let service_docket_rows = load_t2_service_class_repair_docket(&service_docket)
        .with_context(|| format!("loading {}", service_docket.display()))?;
    let rows = t2_game_ops_bundle_evidence_review_rows(
        &decision_rows,
        &target_rows,
        &service_docket_rows,
    );
    write_t2_game_ops_bundle_evidence_review(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_game_ops_bundle_evidence_review_summary(&output, &rows);

    if gate {
        let failures =
            t2_game_ops_bundle_evidence_review_gate_failures(&rows, &decision_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 game/ops bundle evidence review gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 game/ops bundle evidence review gate failed");
        }
        println!();
        println!("T2 game/ops bundle evidence review gate: PASS");
    }
        
    Ok(())
}

