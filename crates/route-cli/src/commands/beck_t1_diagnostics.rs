//! `BeckT1Diagnostics` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route beck-t1-diagnostics");
            let rows = route_map::beck_t1_diagnostics();
            let csv = route_map::build_beck_t1_diagnostics_csv();
            if let Some(parent) = output
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
            {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("creating {}", parent.display()))?;
            }
            std::fs::write(&output, csv)
                .with_context(|| format!("writing {}", output.display()))?;
            println!("  T1 lines: {}", rows.len());
            println!("  wrote diagnostics: {}", output.display());
            println!(
                "  {:<8} {:<12} {:<12} {:>5} {:>5} {:>5} {:>5} {:>5} {:<16} Flag",
                "Line", "Start", "End", "Stops", "Drawn", "Xfer", "Share", "Segs", "Action"
            );
            for row in &rows {
                println!(
                    "  {:<8} {:<12} {:<12} {:>5} {:>5} {:>5} {:>5} {:>5} {:<16} {}",
                    row.corridor,
                    truncate_for_table(row.endpoint_start, 12),
                    truncate_for_table(row.endpoint_end, 12),
                    row.stop_count,
                    row.drawn_stop_count,
                    row.transfer_stop_count,
                    row.shared_stop_count,
                    row.shared_segment_count,
                    truncate_for_table(row.service_action, 16),
                    row.review_flag
                );
            }
            if gate {
                let flagged = rows
                    .iter()
                    .filter(|row| row.review_flag != "ok" && row.review_flag != "overlap-review")
                    .collect::<Vec<_>>();
                if flagged.is_empty() {
                    println!("Beck T1 diagnostics gate: PASS");
                } else {
                    println!("Beck T1 diagnostics gate: FAIL");
                    for row in flagged.iter().take(10) {
                        println!("  - {} {}", row.corridor, row.review_flag);
                    }
                    anyhow::bail!("Beck T1 diagnostics gate failed");
                }
            }
        
    Ok(())
}

