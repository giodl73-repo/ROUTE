//! `T4TerminalContactDistrictProofImport` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    source_registry: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t4-terminal-contact-district-proof-import");
    let registry_rows = load_t4_terminal_contact_proof_source_registry(&source_registry)
        .with_context(|| format!("loading {}", source_registry.display()))?;
    let rows = t4_terminal_contact_district_proof_import_rows(&registry_rows);
    write_t4_terminal_contact_district_proof_import(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t4_terminal_contact_district_proof_import_summary(&output, &rows);

    if gate {
        let failures =
            t4_terminal_contact_district_proof_import_gate_failures(&rows, &registry_rows);
        if !failures.is_empty() {
            println!();
            println!("T4 terminal contact district proof import gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T4 terminal contact district proof import gate failed");
        }
        println!();
        println!("T4 terminal contact district proof import gate: PASS");
    }
        
    Ok(())
}

