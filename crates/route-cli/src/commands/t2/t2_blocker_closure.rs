//! `T2BlockerClosure` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    graph_repairs: PathBuf,
    parent_validation: PathBuf,
    relief_evidence: PathBuf,
    terminal_validation: PathBuf,
    bundles: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-blocker-closure");
    let graph_rows = load_t2_graph_contact_repairs(&graph_repairs)
        .with_context(|| format!("loading {}", graph_repairs.display()))?;
    let parent_rows = load_t2_parent_contact_validation(&parent_validation)
        .with_context(|| format!("loading {}", parent_validation.display()))?;
    let relief_rows = load_t2_relief_evidence_docket(&relief_evidence)
        .with_context(|| format!("loading {}", relief_evidence.display()))?;
    let terminal_rows = load_t2_terminal_contact_validation(&terminal_validation)
        .with_context(|| format!("loading {}", terminal_validation.display()))?;
    let bundle_rows = load_national_segment_bundles(&bundles)
        .with_context(|| format!("loading {}", bundles.display()))?;
    let rows = support::tier::t2_blocker_closure_rows::t2_blocker_closure_rows(
        &graph_rows,
        &parent_rows,
        &relief_rows,
        &terminal_rows,
        &bundle_rows,
    );
    write_t2_blocker_closure(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_blocker_closure_summary(&output, &rows);

    if gate {
        let failures = t2_blocker_closure_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T2 blocker closure gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 blocker closure gate failed");
        }
        println!();
        println!("T2 blocker closure gate: PASS");
    }
        
    Ok(())
}

