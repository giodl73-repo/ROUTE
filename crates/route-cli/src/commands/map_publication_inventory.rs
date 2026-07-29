//! `MapPublicationInventory` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    inventory: PathBuf,
    map_atlas: PathBuf,
    readiness: PathBuf,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route map-publication-inventory");
            let inventory_rows = load_map_publication_inventory(&inventory)
                .with_context(|| format!("loading {}", inventory.display()))?;
            let atlas_rows = load_map_atlas(&map_atlas)
                .with_context(|| format!("loading map atlas {}", map_atlas.display()))?;
            let readiness_rows = load_map_publication_readiness(&readiness)
                .with_context(|| format!("loading {}", readiness.display()))?;
            print_map_publication_inventory_summary(&inventory, &inventory_rows, details);
            if gate {
                let failures = map_publication_inventory_gate_failures(
                    &inventory_rows,
                    &atlas_rows,
                    &readiness_rows,
                );
                if !failures.is_empty() {
                    println!();
                    println!("map publication inventory gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("map publication inventory gate failed");
                }
                println!();
                println!("map publication inventory gate: PASS");
            }
        
    Ok(())
}

