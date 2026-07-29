//! `T1TopologyRepairs` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    design_review: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t1-topology-repairs");
    let review_rows = load_t1_design_review(&design_review)
        .with_context(|| format!("loading {}", design_review.display()))?;
    let rows = t1_topology_repair_rows(&review_rows);
    write_t1_topology_repairs(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t1_topology_repair_summary(&output, &rows);

    if gate {
        let failures = t1_topology_repair_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T1 topology repair gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T1 topology repair gate failed");
        }
        println!();
        println!("T1 topology repair gate: PASS");
    }
        
    Ok(())
}

