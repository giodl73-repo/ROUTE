//! `T2ServiceClassRepairDocket` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    targets: PathBuf,
    service_diagnostics: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-service-class-repair-docket");
    let target_rows = load_t2_bundle_overlay_repair_targets(&targets)
        .with_context(|| format!("loading {}", targets.display()))?;
    let diagnostic_rows = load_t2_service_diagnostic_queue(&service_diagnostics)
        .with_context(|| format!("loading {}", service_diagnostics.display()))?;
    let rows = t2_service_class_repair_docket_rows(&target_rows, &diagnostic_rows);
    write_t2_service_class_repair_docket(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_service_class_repair_docket_summary(&output, &rows);

    if gate {
        let failures = t2_service_class_repair_docket_gate_failures(&rows, &target_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 service-class repair docket gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 service-class repair docket gate failed");
        }
        println!();
        println!("T2 service-class repair docket gate: PASS");
    }

    Ok(())
}
