//! `T1SlaCandidatePairs` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    candidates: PathBuf,
    selected_pairs: PathBuf,
    output: PathBuf,
    selected_budget: usize,
    gate: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t1-sla-candidate-pairs");
    let candidate_rows = load_t1_sla_candidate_universe(&candidates)
        .with_context(|| format!("loading {}", candidates.display()))?;
    let selected_rows = load_t1_sla_pairs(&selected_pairs)
        .with_context(|| format!("loading {}", selected_pairs.display()))?;
    let rows = t1_sla_candidate_pair_rows(&candidate_rows, &selected_rows, selected_budget);
    write_t1_sla_candidate_pairs(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t1_sla_candidate_pair_summary(&output, &rows, selected_budget);

    if gate {
        let failures = t1_sla_candidate_pair_gate_failures(&rows, &selected_rows, selected_budget);
        if failures.is_empty() {
            println!("T1 SLA candidate pair gate: PASS");
        } else {
            println!("T1 SLA candidate pair gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T1 SLA candidate pair gate failed");
        }
    }

    Ok(())
}
