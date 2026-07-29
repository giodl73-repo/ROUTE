//! `OptimizerMapHooks` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route optimizer-map-hooks");
            let rows = optimizer_map_hook_rows();
            write_optimizer_map_hooks(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_optimizer_map_hook_summary(&output, &rows);

            if gate {
                let failures = optimizer_map_hook_gate_failures(&rows);
                if !failures.is_empty() {
                    println!();
                    println!("optimizer map hook gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("optimizer map hook gate failed");
                }
                println!();
                println!("optimizer map hook gate: PASS");
            }
        
    Ok(())
}

