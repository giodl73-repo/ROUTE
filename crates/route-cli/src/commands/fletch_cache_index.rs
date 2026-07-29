//! `FletchCacheIndex` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    registry: PathBuf,
    cache_manifest: Option<PathBuf>,
    output: PathBuf,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route fletch-cache-index");
            let route_manifest = route_data::Manifest::load(&manifest_path)
                .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
            let cache_root = route_manifest.cache_dir.join(".fletch");
            let cache_manifest_path = cache_manifest
                .unwrap_or_else(|| route_data::fletch_cache_manifest_path(&cache_root));
            let registry_report = route_data::load_fletch_source_registry(&registry)
                .with_context(|| format!("loading {}", registry.display()))?;
            let cache_manifest =
                route_data::read_fletch_cache_manifest(&cache_manifest_path, &cache_root)
                    .with_context(|| format!("loading {}", cache_manifest_path.display()))?;
            let report = route_data::fletch_cache_index_report(&registry_report, &cache_manifest);
            route_data::write_fletch_cache_index(&output, &report)
                .with_context(|| format!("writing {}", output.display()))?;
            print_fletch_cache_index_summary(&output, &report, details);

            if gate {
                let failures = route_data::fletch_cache_index_gate_failures(&report);
                if !failures.is_empty() {
                    println!();
                    println!("FLETCH cache-index gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("FLETCH cache-index gate failed");
                }
                println!();
                println!("FLETCH cache-index gate: PASS");
            }
        
    Ok(())
}

