//! `T2TerminalContactValidation` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    held_actions: PathBuf,
    exceptions: PathBuf,
    witnesses: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-terminal-contact-validation");
    let held_rows = load_t2_held_contact_actions(&held_actions)
        .with_context(|| format!("loading {}", held_actions.display()))?;
    let exception_rows = load_endpoint_exceptions(&exceptions)
        .with_context(|| format!("loading {}", exceptions.display()))?;
    let witness_rows = load_tier_contact_witnesses(&witnesses)
        .with_context(|| format!("loading {}", witnesses.display()))?;
    let rows =
        t2_terminal_contact_validation_rows(&held_rows, &exception_rows, &witness_rows);
    write_t2_terminal_contact_validation(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_terminal_contact_validation_summary(&output, &rows);

    if gate {
        let failures = t2_terminal_contact_validation_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T2 terminal contact validation gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 terminal contact validation gate failed");
        }
        println!();
        println!("T2 terminal contact validation gate: PASS");
    }
        
    Ok(())
}

