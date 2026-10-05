//! `StopSlaSummary` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    input: PathBuf,
    top: usize,
    gate_max_gap: Option<f64>,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let file = std::fs::File::open(&input)
        .with_context(|| format!("reading stop SLA surface {}", input.display()))?;
    let rows = parse_stop_sla_rows(file)?;
    print_stop_sla_summary(&rows, top);
    if let Some(max_gap) = gate_max_gap {
        let blockers = stop_sla_gap_failures(&rows, max_gap);
        if blockers.is_empty() {
            println!("stop SLA max-gap gate: PASS");
        } else {
            println!("stop SLA max-gap gate: FAIL");
            for row in blockers.iter().take(top.max(1)) {
                println!(
                    "  - {}→{} max gap {:.0} mi via {}",
                    row.origin_id, row.dest_id, row.max_stop_gap_miles, row.route_path
                );
            }
            anyhow::bail!("stop SLA max-gap gate failed");
        }
    }

    Ok(())
}
