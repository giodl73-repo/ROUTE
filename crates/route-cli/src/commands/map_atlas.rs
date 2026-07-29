//! `MapAtlas` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    ledger: PathBuf,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let rows = load_map_atlas(&ledger)
        .with_context(|| format!("loading map atlas {}", ledger.display()))?;
    print_map_atlas(&rows, details);
    if gate {
        let failures = map_atlas_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("Map atlas gate: FAIL");
            println!("  {} map artifacts failed contract.", failures.len());
            for failure in failures.iter().take(12) {
                println!("  - {failure}");
            }
            anyhow::bail!("map atlas gate failed");
        }
        println!();
        println!("Map atlas gate: PASS");
    }
        
    Ok(())
}

