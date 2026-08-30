//! `T2ContactClosure` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    closure: PathBuf,
    witnesses: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-contact-closure");
    let closure_rows = load_t2_blocker_closure(&closure)
        .with_context(|| format!("loading {}", closure.display()))?;
    let witness_rows = load_tier_contact_witnesses(&witnesses)
        .with_context(|| format!("loading {}", witnesses.display()))?;
    let rows = t2_contact_closure_rows(&closure_rows, &witness_rows);
    write_t2_contact_closure(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_contact_closure_summary(&output, &rows);

    if gate {
        let failures = t2_contact_closure_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T2 contact closure gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 contact closure gate failed");
        }
        println!();
        println!("T2 contact closure gate: PASS");
    }

    Ok(())
}
