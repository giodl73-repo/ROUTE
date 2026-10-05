//! `T2ServiceSelection` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    regionalizer: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-service-selection");
    let regionalizer_rows = load_t2_regionalizer(&regionalizer)
        .with_context(|| format!("loading {}", regionalizer.display()))?;
    let rows = t2_service_selection_rows(&regionalizer_rows, &route_map::beck_t2_diagnostics());
    write_t2_service_selection(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_service_selection_summary(&output, &rows);

    if gate {
        let failures = t2_service_selection_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T2 service selection gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 service selection gate failed");
        }
        println!();
        println!("T2 service selection gate: PASS");
    }

    Ok(())
}
