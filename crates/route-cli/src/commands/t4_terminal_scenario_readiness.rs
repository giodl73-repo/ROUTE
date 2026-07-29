//! `T4TerminalScenarioReadiness` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    contact_evidence: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route t4-terminal-scenario-readiness");
            let contact_rows = load_t4_terminal_contact_evidence(&contact_evidence)
                .with_context(|| format!("loading {}", contact_evidence.display()))?;
            let rows = t4_terminal_scenario_readiness_rows(&contact_rows);
            write_t4_terminal_scenario_readiness(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t4_terminal_scenario_readiness_summary(&output, &rows);

            if gate {
                let failures = t4_terminal_scenario_readiness_gate_failures(&rows, &contact_rows);
                if !failures.is_empty() {
                    println!();
                    println!("T4 terminal scenario readiness gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("T4 terminal scenario readiness gate failed");
                }
                println!();
                println!("T4 terminal scenario readiness gate: PASS");
            }
        
    Ok(())
}

