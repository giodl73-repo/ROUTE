//! `T1FailureEvents` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    events: PathBuf,
    ledger: PathBuf,
    write_ledger: Option<PathBuf>,
    gate_observations: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            let event_rows = load_t1_failure_events(&events)
                .with_context(|| format!("loading T1 failure events {}", events.display()))?;
            print_t1_failure_event_summary(&event_rows);
            if gate_observations {
                let failures = t1_failure_event_observation_gate_failures(&event_rows);
                if !failures.is_empty() {
                    println!();
                    println!("T1/T1 event observation gate: FAIL");
                    println!(
                        "  {} event rows are empty or missing normalized evidence fields.",
                        failures.len()
                    );
                    for failure in failures.iter().take(10) {
                        println!("  - {}", failure);
                    }
                    if failures.len() > 10 {
                        println!("  ... {} more", failures.len() - 10);
                    }
                    anyhow::bail!("T1/T1 event observation gate failed");
                }
                println!();
                println!("T1/T1 event observation gate: PASS");
            }
            if let Some(output) = write_ledger {
                let ledger_rows = load_t1_failure_ledger(&ledger)
                    .with_context(|| format!("loading T1 failure ledger {}", ledger.display()))?;
                let updated = apply_t1_failure_events_to_ledger(&ledger_rows, &event_rows, &events);
                write_t1_failure_ledger(&output, &updated)
                    .with_context(|| format!("writing T1 failure ledger {}", output.display()))?;
                println!();
                println!("  updated ledger -> {}", output.display());
            }
        
    Ok(())
}

