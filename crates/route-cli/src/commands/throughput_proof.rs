//! `ThroughputProof` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    matrix: PathBuf,
    blockers: bool,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let rows = load_throughput_proof_matrix(&matrix)
        .with_context(|| format!("loading throughput proof matrix {}", matrix.display()))?;
    print_throughput_proof_matrix(&rows, blockers, details);

    if gate {
        let failures = throughput_proof_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("Throughput proof gate: FAIL");
            println!(
                "  {} proof rows still lack bounded congestion/resilience contracts.",
                failures.len()
            );
            for row in failures.iter().take(10) {
                println!(
                    "  - {} [{} {}]: {}",
                    row.proof_id, row.binding_type, row.current_status, row.blocking_gap
                );
            }
            anyhow::bail!("throughput proof gate failed");
        }
        println!();
        println!("Throughput proof gate: PASS");
    }
        
    Ok(())
}

