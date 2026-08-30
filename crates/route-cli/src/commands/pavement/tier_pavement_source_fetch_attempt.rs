//! `TierPavementSourceFetchAttempt` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    source_access: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-pavement-source-fetch-attempt");
    let source_access_rows = load_tier_pavement_source_access(&source_access)
        .with_context(|| format!("loading {}", source_access.display()))?;
    let rows = tier_pavement_source_fetch_attempt_rows(&source_access_rows)?;
    write_tier_pavement_source_fetch_attempt(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_pavement_source_fetch_attempt_summary(&output, &rows);

    if gate {
        let failures = tier_pavement_source_fetch_attempt_gate_failures(&rows, &source_access_rows);
        if !failures.is_empty() {
            println!();
            println!("Tier pavement source fetch attempt gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier pavement source fetch attempt gate failed");
        }
        println!();
        println!("Tier pavement source fetch attempt gate: PASS");
    }

    Ok(())
}
