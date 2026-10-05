//! `BundleArchitecture` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(ctx: &ctx::Ctx<'_>, output: PathBuf, gate: bool) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route bundle-architecture");
    let rows = bundle_architecture_rows();
    write_bundle_architecture(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_bundle_architecture_summary(&output, &rows);

    if gate {
        let failures = bundle_architecture_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("bundle architecture gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("bundle architecture gate failed");
        }
        println!();
        println!("bundle architecture gate: PASS");
    }

    Ok(())
}
