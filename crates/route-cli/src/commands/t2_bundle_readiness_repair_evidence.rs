//! `T2BundleReadinessRepairEvidence` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    repair_docket: PathBuf,
    registry: PathBuf,
    segment_candidates: PathBuf,
    service_selection: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-bundle-readiness-repair-evidence");
    let repair_rows = load_t2_bundle_readiness_repair_docket(&repair_docket)
        .with_context(|| format!("loading {}", repair_docket.display()))?;
    let registry_rows = load_national_segment_registry(&registry)
        .with_context(|| format!("loading {}", registry.display()))?;
    let candidate_rows = load_tier_segment_candidates(&segment_candidates)
        .with_context(|| format!("loading {}", segment_candidates.display()))?;
    let service_rows = load_t2_service_selection(&service_selection)
        .with_context(|| format!("loading {}", service_selection.display()))?;
    let rows = t2_bundle_readiness_repair_evidence_rows(
        &repair_rows,
        &registry_rows,
        &candidate_rows,
        &service_rows,
    );
    write_t2_bundle_readiness_repair_evidence(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_bundle_readiness_repair_evidence_summary(&output, &rows);

    if gate {
        let failures =
            t2_bundle_readiness_repair_evidence_gate_failures(&rows, &repair_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 bundle readiness repair evidence gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 bundle readiness repair evidence gate failed");
        }
        println!();
        println!("T2 bundle readiness repair evidence gate: PASS");
    }
        
    Ok(())
}

