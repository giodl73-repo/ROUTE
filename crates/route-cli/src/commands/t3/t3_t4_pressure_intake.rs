//! `T3T4PressureIntake` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    pressure: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t3-t4-pressure-intake");
    let pressure_rows = load_lower_tier_pressure_witnesses(&pressure)
        .with_context(|| format!("loading {}", pressure.display()))?;
    let rows = t3_t4_pressure_intake_rows(&pressure_rows);
    write_t3_t4_pressure_intake(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t3_t4_pressure_intake_summary(&output, &rows);

    if gate {
        let failures = t3_t4_pressure_intake_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T3/T4 pressure intake gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T3/T4 pressure intake gate failed");
        }
        println!();
        println!("T3/T4 pressure intake gate: PASS");
    }
        
    Ok(())
}

