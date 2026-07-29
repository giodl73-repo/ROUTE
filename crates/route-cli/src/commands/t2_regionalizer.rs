//! `T2Regionalizer` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    candidates: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-regionalizer");
    let candidate_rows = load_tier_candidate_columns(&candidates)
        .with_context(|| format!("loading {}", candidates.display()))?;
    let rows = t2_regionalizer_rows(&candidate_rows);
    write_t2_regionalizer(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_regionalizer_summary(&output, &rows);

    if gate {
        let failures = t2_regionalizer_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T2 regionalizer gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 regionalizer gate failed");
        }
        println!();
        println!("T2 regionalizer gate: PASS");
    }
        
    Ok(())
}

