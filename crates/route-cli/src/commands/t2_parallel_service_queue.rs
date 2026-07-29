//! `T2ParallelServiceQueue` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    service_selection: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-parallel-service-queue");
    let service_rows = load_t2_service_selection(&service_selection)
        .with_context(|| format!("loading {}", service_selection.display()))?;
    let rows = t2_parallel_service_queue_rows(&service_rows);
    write_t2_parallel_service_queue(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_parallel_service_queue_summary(&output, &rows);

    if gate {
        let failures = t2_parallel_service_queue_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T2 parallel service queue gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 parallel service queue gate failed");
        }
        println!();
        println!("T2 parallel service queue gate: PASS");
    }
        
    Ok(())
}

