//! `StopCandidates` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    ledger: PathBuf,
    stop_class: Option<String>,
    route: Option<String>,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route stop-candidates");
            let file = std::fs::File::open(&ledger)
                .with_context(|| format!("opening stop candidate ledger {}", ledger.display()))?;
            let rows = parse_stop_candidates(file)
                .with_context(|| format!("parsing stop candidate ledger {}", ledger.display()))?;
            let filtered = filter_stop_candidates(&rows, stop_class.as_deref(), route.as_deref());
            print_stop_candidates(&filtered, details);

            if gate {
                let failures = stop_candidate_gate_failures(&filtered);
                if !failures.is_empty() {
                    println!();
                    println!("stop candidate gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("stop candidate gate failed");
                }
                println!();
                println!("stop candidate gate: PASS");
            }
        
    Ok(())
}

