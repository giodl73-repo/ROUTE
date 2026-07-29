//! `T3T4AccessGaps` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    route_columns: PathBuf,
    terminal_columns: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t3-t4-access-gaps");
    let route_rows = load_t3_zone_route_columns(&route_columns)
        .with_context(|| format!("loading {}", route_columns.display()))?;
    let terminal_rows = load_t4_terminal_access_columns(&terminal_columns)
        .with_context(|| format!("loading {}", terminal_columns.display()))?;
    let rows = t3_t4_access_gap_rows(&route_rows, &terminal_rows);
    write_t3_t4_access_gaps(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t3_t4_access_gap_summary(&output, &rows);

    if gate {
        let failures = t3_t4_access_gap_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T3/T4 access gap gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T3/T4 access gap gate failed");
        }
        println!();
        println!("T3/T4 access gap gate: PASS");
    }
        
    Ok(())
}

