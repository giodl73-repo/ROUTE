//! `TierContactWitnesses` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(ctx: &ctx::Ctx<'_>, repairs: PathBuf, output: PathBuf, gate: bool) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route tier-contact-witnesses");
    let repair_rows = load_tier_region_repairs(&repairs)
        .with_context(|| format!("loading {}", repairs.display()))?;
    let witness_rows = tier_contact_witness_rows(&repair_rows, &route_map::beck_t2_diagnostics());
    write_tier_contact_witnesses(&output, &witness_rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_tier_contact_witness_summary(&output, &witness_rows);

    if gate {
        let failures = tier_contact_witness_gate_failures(&witness_rows);
        if !failures.is_empty() {
            println!();
            println!("tier contact witness gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("tier contact witness gate failed");
        }
        println!();
        println!("tier contact witness gate: PASS");
    }

    Ok(())
}
