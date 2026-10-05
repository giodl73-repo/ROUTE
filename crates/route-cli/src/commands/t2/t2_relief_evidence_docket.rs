//! `T2ReliefEvidenceDocket` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    held_actions: PathBuf,
    bottlenecks: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-relief-evidence-docket");
    let held_rows = load_t2_held_contact_actions(&held_actions)
        .with_context(|| format!("loading {}", held_actions.display()))?;
    let bottleneck_rows = load_atri_bottlenecks(&bottlenecks)
        .with_context(|| format!("loading {}", bottlenecks.display()))?;
    let rows = t2_relief_evidence_rows(&held_rows, &bottleneck_rows);
    write_t2_relief_evidence_docket(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_relief_evidence_summary(&output, &rows);

    if gate {
        let failures = t2_relief_evidence_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T2 relief evidence gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 relief evidence gate failed");
        }
        println!();
        println!("T2 relief evidence gate: PASS");
    }

    Ok(())
}
