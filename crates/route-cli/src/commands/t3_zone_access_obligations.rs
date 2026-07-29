//! `T3ZoneAccessObligations` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    intake: PathBuf,
    map_atlas: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t3-zone-access-obligations");
    let intake_rows = load_t3_t4_pressure_intake(&intake)
        .with_context(|| format!("loading {}", intake.display()))?;
    let atlas_rows = load_map_atlas(&map_atlas)
        .with_context(|| format!("loading {}", map_atlas.display()))?;
    let rows = t3_zone_access_obligation_rows(&intake_rows, &atlas_rows);
    write_t3_zone_access_obligations(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t3_zone_access_obligation_summary(&output, &rows);

    if gate {
        let failures = t3_zone_access_obligation_gate_failures(&rows, &atlas_rows);
        if !failures.is_empty() {
            println!();
            println!("T3 zone access obligation gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T3 zone access obligation gate failed");
        }
        println!();
        println!("T3 zone access obligation gate: PASS");
    }
        
    Ok(())
}

