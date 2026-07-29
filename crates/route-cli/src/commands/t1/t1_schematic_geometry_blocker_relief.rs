//! `T1SchematicGeometryBlockerRelief` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    acceptance: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t1-schematic-geometry-blocker-relief");
    let acceptance_rows = load_t1_shared_segment_policy_acceptance(&acceptance)
        .with_context(|| format!("loading {}", acceptance.display()))?;
    let rows = t1_schematic_geometry_blocker_relief_rows(&acceptance_rows);
    write_t1_schematic_geometry_blocker_relief(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t1_schematic_geometry_blocker_relief_summary(&output, &rows);

    if gate {
        let failures =
            t1_schematic_geometry_blocker_relief_gate_failures(&rows, &acceptance_rows);
        if !failures.is_empty() {
            println!();
            println!("T1 schematic geometry blocker relief gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T1 schematic geometry blocker relief gate failed");
        }
        println!();
        println!("T1 schematic geometry blocker relief gate: PASS");
    }
        
    Ok(())
}

