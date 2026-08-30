//! `T2StitchedMemberRegistryHandoff` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    audit: PathBuf,
    registry: PathBuf,
    segment_candidates: PathBuf,
    output: PathBuf,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-stitched-member-registry-handoff");
    let audit_rows = load_t2_national_bundle_readiness_audit(&audit)
        .with_context(|| format!("loading {}", audit.display()))?;
    let registry_rows = load_national_segment_registry(&registry)
        .with_context(|| format!("loading {}", registry.display()))?;
    let candidate_rows = load_tier_segment_candidates(&segment_candidates)
        .with_context(|| format!("loading {}", segment_candidates.display()))?;
    let rows =
        t2_stitched_member_registry_handoff_rows(&audit_rows, &registry_rows, &candidate_rows);
    write_t2_stitched_member_registry_handoff(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_stitched_member_registry_handoff_summary(&output, &rows);

    if gate {
        let failures = t2_stitched_member_registry_handoff_gate_failures(&rows, &audit_rows);
        if !failures.is_empty() {
            println!();
            println!("T2 stitched member registry handoff gate: FAIL");
            for failure in failures {
                println!("  - {failure}");
            }
            anyhow::bail!("t2 stitched member registry handoff gate failed");
        }
        println!("T2 stitched member registry handoff gate: PASS");
    }

    Ok(())
}
