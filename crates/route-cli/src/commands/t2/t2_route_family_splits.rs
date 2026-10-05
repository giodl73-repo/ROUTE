//! `T2RouteFamilySplits` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    closure: PathBuf,
    service_diagnostics: PathBuf,
    bundles: PathBuf,
    exceptions: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-route-family-splits");
    let closure_rows = load_t2_blocker_closure(&closure)
        .with_context(|| format!("loading {}", closure.display()))?;
    let service_diagnostic_rows = load_t2_service_diagnostic_queue(&service_diagnostics)
        .with_context(|| format!("loading {}", service_diagnostics.display()))?;
    let bundle_rows = load_national_segment_bundles(&bundles)
        .with_context(|| format!("loading {}", bundles.display()))?;
    let exception_rows = load_endpoint_exceptions(&exceptions)
        .with_context(|| format!("loading {}", exceptions.display()))?;
    let rows = support::tier::t2_route_family_split_rows::t2_route_family_split_rows(
        &closure_rows,
        &service_diagnostic_rows,
        &bundle_rows,
        &exception_rows,
    );
    write_t2_route_family_splits(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_route_family_split_summary(&output, &rows);

    if gate {
        let failures = t2_route_family_split_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T2 route family split gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 route family split gate failed");
        }
        println!();
        println!("T2 route family split gate: PASS");
    }

    Ok(())
}
