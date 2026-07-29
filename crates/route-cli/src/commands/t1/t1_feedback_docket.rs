//! `T1FeedbackDocket` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    service_selection: PathBuf,
    bubble_up: PathBuf,
    intake: PathBuf,
    sla_pairs: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t1-feedback-docket");
    let service_rows = load_t2_service_selection(&service_selection)
        .with_context(|| format!("loading {}", service_selection.display()))?;
    let bubble_rows = load_t2_bubble_up_review(&bubble_up)
        .with_context(|| format!("loading {}", bubble_up.display()))?;
    let intake_rows = load_t3_t4_pressure_intake(&intake)
        .with_context(|| format!("loading {}", intake.display()))?;
    let sla_rows = load_t1_sla_pairs(&sla_pairs)
        .with_context(|| format!("loading {}", sla_pairs.display()))?;
    let rows =
        t1_feedback_docket_rows(&service_rows, &bubble_rows, &intake_rows, &sla_rows);
    write_t1_feedback_docket(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t1_feedback_docket_summary(&output, &rows);

    if gate {
        let failures = t1_feedback_docket_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T1 feedback docket gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T1 feedback docket gate failed");
        }
        println!();
        println!("T1 feedback docket gate: PASS");
    }
        
    Ok(())
}

