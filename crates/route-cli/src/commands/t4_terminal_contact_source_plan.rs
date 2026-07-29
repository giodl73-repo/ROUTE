//! `T4TerminalContactSourcePlan` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    contact_evidence: PathBuf,
    output: PathBuf,
    catalog_output: PathBuf,
    proof_docket_output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route t4-terminal-contact-source-plan");
            let contact_rows = load_t4_terminal_contact_evidence(&contact_evidence)
                .with_context(|| format!("loading {}", contact_evidence.display()))?;
            let rows = t4_terminal_contact_source_plan_rows(&contact_rows);
            write_t4_terminal_contact_source_plan(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t4_terminal_contact_source_plan_summary(&output, &rows);
            let catalog_rows = t4_terminal_contact_source_catalog_rows(&rows);
            write_t4_terminal_contact_source_catalog(&catalog_output, &catalog_rows)
                .with_context(|| format!("writing {}", catalog_output.display()))?;
            print_t4_terminal_contact_source_catalog_summary(&catalog_output, &catalog_rows);
            let proof_rows = t4_terminal_contact_proof_docket_rows(&rows, &catalog_rows);
            write_t4_terminal_contact_proof_docket(&proof_docket_output, &proof_rows)
                .with_context(|| format!("writing {}", proof_docket_output.display()))?;
            print_t4_terminal_contact_proof_docket_summary(&proof_docket_output, &proof_rows);

            if gate {
                let mut failures =
                    t4_terminal_contact_source_plan_gate_failures(&rows, &contact_rows);
                failures.extend(t4_terminal_contact_source_catalog_gate_failures(
                    &catalog_rows,
                    &rows,
                ));
                failures.extend(t4_terminal_contact_proof_docket_gate_failures(
                    &proof_rows,
                    &rows,
                    &catalog_rows,
                ));
                if !failures.is_empty() {
                    println!();
                    println!("T4 terminal contact source plan gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("T4 terminal contact source plan gate failed");
                }
                println!();
                println!("T4 terminal contact source plan gate: PASS");
            }
        
    Ok(())
}

