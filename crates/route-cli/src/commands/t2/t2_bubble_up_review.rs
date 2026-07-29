//! `T2BubbleUpReview` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    intake: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-bubble-up-review");
    let intake_rows = load_t3_t4_pressure_intake(&intake)
        .with_context(|| format!("loading {}", intake.display()))?;
    let rows = t2_bubble_up_review_rows(&intake_rows);
    write_t2_bubble_up_review(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_bubble_up_review_summary(&output, &rows);

    if gate {
        let failures = t2_bubble_up_review_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T2 bubble-up review gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 bubble-up review gate failed");
        }
        println!();
        println!("T2 bubble-up review gate: PASS");
    }
        
    Ok(())
}

