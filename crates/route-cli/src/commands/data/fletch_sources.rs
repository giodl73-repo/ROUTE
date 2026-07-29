//! `FletchSources` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    registry: PathBuf,
    source_policy: PathBuf,
    output: PathBuf,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route fletch-sources");
    let registry_report = route_data::load_fletch_source_registry(&registry)
        .with_context(|| format!("loading {}", registry.display()))?;
    let source_policy_rows = route_data::load_route_source_fetch_policy(&source_policy)
        .with_context(|| format!("loading {}", source_policy.display()))?;
    let report =
        route_data::fletch_source_handoff_report(&registry_report, &source_policy_rows);
    route_data::write_fletch_source_handoff(&output, &report)
        .with_context(|| format!("writing {}", output.display()))?;
    print_fletch_source_handoff_summary(&output, &report, details);

    if gate {
        let failures = fletch_source_handoff_gate_failures(&report);
        if !failures.is_empty() {
            println!();
            println!("FLETCH source handoff gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("FLETCH source handoff gate failed");
        }
        println!();
        println!("FLETCH source handoff gate: PASS");
    }
        
    Ok(())
}

