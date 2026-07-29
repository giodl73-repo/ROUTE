//! `T1BeckAlignment` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    stop_selector: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route t1-beck-alignment");
            let stop_rows = load_t1_stop_selector(&stop_selector)
                .with_context(|| format!("loading {}", stop_selector.display()))?;
            let rows = t1_beck_alignment_rows(&stop_rows);
            write_t1_beck_alignment(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t1_beck_alignment_summary(&output, &rows);

            if gate {
                let failures = t1_beck_alignment_gate_failures(&rows);
                if !failures.is_empty() {
                    println!();
                    println!("T1 Beck alignment gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("T1 Beck alignment gate failed");
                }
                println!();
                println!("T1 Beck alignment gate: PASS");
            }
        
    Ok(())
}

