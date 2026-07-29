//! `T2HeldContactActions` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    resolutions: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-held-contact-actions");
    let resolution_rows = load_t2_contact_resolutions(&resolutions)
        .with_context(|| format!("loading {}", resolutions.display()))?;
    let rows = t2_held_contact_action_rows(&resolution_rows);
    write_t2_held_contact_actions(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_held_contact_action_summary(&output, &rows);

    if gate {
        let failures = t2_held_contact_action_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T2 held contact action gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 held contact action gate failed");
        }
        println!();
        println!("T2 held contact action gate: PASS");
    }
        
    Ok(())
}

