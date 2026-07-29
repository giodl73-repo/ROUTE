//! `BlueprintEvidence` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    ledger: PathBuf,
    evidence_map: PathBuf,
    standards_ledger: PathBuf,
    blockers: bool,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let packages = load_blueprint_packages(&ledger).with_context(|| {
        format!("loading Blueprint package ledger {}", ledger.display())
    })?;
    let standards = load_standards_proof_ledger(&standards_ledger).with_context(|| {
        format!(
            "loading standards proof ledger {}",
            standards_ledger.display()
        )
    })?;
    let rows = load_blueprint_evidence_map(&evidence_map).with_context(|| {
        format!("loading Blueprint evidence map {}", evidence_map.display())
    })?;
    print_blueprint_evidence_map(&rows, blockers, details);

    if gate {
        let failures = blueprint_evidence_gate_failures(&rows, &packages, &standards);
        if !failures.is_empty() {
            println!();
            println!("Blueprint evidence gate: FAIL");
            println!(
                "  {} evidence rows violate downgrade rules.",
                failures.len()
            );
            for failure in failures.iter().take(12) {
                println!("  - {failure}");
            }
            anyhow::bail!("blueprint evidence gate failed");
        }
        println!();
        println!("Blueprint evidence gate: PASS");
    }
        
    Ok(())
}

