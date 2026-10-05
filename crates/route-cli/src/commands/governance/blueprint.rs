//! `Blueprint` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    ledger: PathBuf,
    blockers: bool,
    details: bool,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let rows = load_blueprint_packages(&ledger)
        .with_context(|| format!("loading Blueprint package ledger {}", ledger.display()))?;
    print_blueprint_packages(&rows, blockers, details);

    if gate {
        let failures = blueprint_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("Blueprint intake gate: FAIL");
            println!(
                "  {} package rows violate Forum intake rules.",
                failures.len()
            );
            for failure in failures.iter().take(12) {
                println!("  - {failure}");
            }
            anyhow::bail!("blueprint intake gate failed");
        }
        println!();
        println!("Blueprint intake gate: PASS");
    }

    Ok(())
}
