//! `T2EndpointClosure` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    closure: PathBuf,
    exceptions: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-endpoint-closure");
    let closure_rows = load_t2_blocker_closure(&closure)
        .with_context(|| format!("loading {}", closure.display()))?;
    let exception_rows = load_endpoint_exceptions(&exceptions)
        .with_context(|| format!("loading {}", exceptions.display()))?;
    let rows = t2_endpoint_closure_rows(&closure_rows, &exception_rows);
    write_t2_endpoint_closure(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_endpoint_closure_summary(&output, &rows);

    if gate {
        let failures = t2_endpoint_closure_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T2 endpoint closure gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 endpoint closure gate failed");
        }
        println!();
        println!("T2 endpoint closure gate: PASS");
    }

    Ok(())
}
