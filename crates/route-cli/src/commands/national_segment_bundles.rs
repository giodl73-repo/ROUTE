//! `NationalSegmentBundles` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    registry: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route national-segment-bundles");
            let registry_rows = load_national_segment_registry(&registry)
                .with_context(|| format!("loading {}", registry.display()))?;
            let rows = national_segment_bundle_rows(&registry_rows);
            write_national_segment_bundles(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_national_segment_bundle_summary(&output, &rows);

            if gate {
                let failures = national_segment_bundle_gate_failures(&rows, &registry_rows);
                if !failures.is_empty() {
                    println!();
                    println!("National segment bundle gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("national segment bundle gate failed");
                }
                println!();
                println!("National segment bundle gate: PASS");
            }
        
    Ok(())
}

