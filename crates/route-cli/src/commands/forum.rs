//! `Forum` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    docket: PathBuf,
    blockers: bool,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            let rows = load_forum_docket(&docket)
                .with_context(|| format!("loading Forum docket {}", docket.display()))?;
            print_forum_docket(&rows, blockers, details);

            if gate {
                let failures = forum_docket_gate_failures(&rows);
                if !failures.is_empty() {
                    println!();
                    println!("Forum docket gate: FAIL");
                    println!("  {} review rows lack complete contracts.", failures.len());
                    for failure in failures.iter().take(10) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("forum docket gate failed");
                }
                println!();
                println!("Forum docket gate: PASS");
            }
        
    Ok(())
}

