//! `SourceFetchPolicy` command handler extracted from main.
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

    println!("route source-fetch-policy");
    let rows = source_fetch_policy_rows();
    write_source_fetch_policy(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_source_fetch_policy_summary(&output, &rows);

    if gate {
        let failures = source_fetch_policy_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("source fetch policy gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("source fetch policy gate failed");
        }
        println!();
        println!("source fetch policy gate: PASS");
    }
        
    Ok(())
}

