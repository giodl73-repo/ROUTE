//! `EndpointExceptions` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    ledger: PathBuf,
    tier: Option<String>,
    route: Option<String>,
    blockers: bool,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route endpoint-exceptions");
            let rows = load_endpoint_exceptions(&ledger)
                .with_context(|| format!("loading endpoint exceptions {}", ledger.display()))?;
            let filtered = filter_endpoint_exceptions(&rows, tier.as_deref(), route.as_deref());
            print_endpoint_exceptions(&filtered, blockers, details);

            if gate {
                let failures = endpoint_exception_gate_failures(&filtered, blockers);
                if !failures.is_empty() {
                    println!();
                    println!("endpoint exception gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("endpoint exception gate failed");
                }
                println!();
                println!("endpoint exception gate: PASS");
            }
        
    Ok(())
}

