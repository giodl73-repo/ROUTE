//! `T2NationalBundleReadinessAudit` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    replay_decisions: PathBuf,
    bundles: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-national-bundle-readiness-audit");
    let replay_rows = load_t2_bundle_readiness_replay_decisions(&replay_decisions)
        .with_context(|| format!("loading {}", replay_decisions.display()))?;
    let bundle_rows = load_national_segment_bundles(&bundles)
        .with_context(|| format!("loading {}", bundles.display()))?;
    let rows = t2_national_bundle_readiness_audit_rows(&replay_rows, &bundle_rows);
    write_t2_national_bundle_readiness_audit(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_national_bundle_readiness_audit_summary(&output, &rows);

    if gate {
        let failures = t2_national_bundle_readiness_audit_gate_failures(&rows, &replay_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 national bundle readiness audit gate: FAIL");
            for failure in failures {
                println!("  - {failure}");
            }
            anyhow::bail!("t2 national bundle readiness audit gate failed");
        }
        println!("T2 national bundle readiness audit gate: PASS");
    }

    Ok(())
}
