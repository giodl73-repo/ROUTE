//! `T2ContactResolutions` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    witnesses: PathBuf,
    exceptions: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-contact-resolutions");
    let witness_rows = load_tier_contact_witnesses(&witnesses)
        .with_context(|| format!("loading {}", witnesses.display()))?;
    let exception_rows = load_endpoint_exceptions(&exceptions)
        .with_context(|| format!("loading {}", exceptions.display()))?;
    let rows = t2_contact_resolution_rows(&witness_rows, &exception_rows);
    write_t2_contact_resolutions(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_contact_resolution_summary(&output, &rows);

    if gate {
        let failures = t2_contact_resolution_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T2 contact resolution gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 contact resolution gate failed");
        }
        println!();
        println!("T2 contact resolution gate: PASS");
    }
        
    Ok(())
}

