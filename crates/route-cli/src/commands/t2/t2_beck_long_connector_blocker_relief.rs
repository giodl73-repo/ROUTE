//! `T2BeckLongConnectorBlockerRelief` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    acceptance: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-beck-long-connector-blocker-relief");
    let acceptance_rows = load_t2_beck_long_connector_policy_acceptance(&acceptance)
        .with_context(|| format!("loading {}", acceptance.display()))?;
    let rows = t2_beck_long_connector_blocker_relief_rows(&acceptance_rows);
    write_t2_beck_long_connector_blocker_relief(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_beck_long_connector_blocker_relief_summary(&output, &rows);

    if gate {
        let failures = t2_beck_long_connector_blocker_relief_gate_failures(&rows, &acceptance_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 Beck long connector blocker relief gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 Beck long connector blocker relief gate failed");
        }
        println!();
        println!("T2 Beck long connector blocker relief gate: PASS");
    }

    Ok(())
}
