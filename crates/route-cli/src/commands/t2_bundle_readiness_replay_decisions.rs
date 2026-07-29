//! `T2BundleReadinessReplayDecisions` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    evidence: PathBuf,
    repair_delta: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route t2-bundle-readiness-replay-decisions");
            let evidence_rows = load_t2_bundle_readiness_repair_evidence(&evidence)
                .with_context(|| format!("loading {}", evidence.display()))?;
            let delta_rows = load_t2_bundle_overlay_repair_delta(&repair_delta)
                .with_context(|| format!("loading {}", repair_delta.display()))?;
            let rows = t2_bundle_readiness_replay_decision_rows(&evidence_rows, &delta_rows);
            write_t2_bundle_readiness_replay_decisions(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t2_bundle_readiness_replay_decision_summary(&output, &rows);

            if gate {
                let failures = t2_bundle_readiness_replay_decision_gate_failures(
                    &rows,
                    &evidence_rows,
                    &delta_rows,
                );
                if !failures.is_empty() {
                    println!();
                    println!("T2 bundle readiness replay decision gate: FAIL");
                    for failure in failures {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("t2 bundle readiness replay decision gate failed");
                }
                println!("T2 bundle readiness replay decision gate: PASS");
            }
        
    Ok(())
}

