//! `BeckT2Diagnostics` command handler extracted from main.
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

    println!("route beck-t2-diagnostics");
    let rows = route_map::beck_t2_diagnostics();
    let csv = route_map::build_beck_t2_diagnostics_csv();
    if let Some(parent) = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    std::fs::write(&output, csv)
        .with_context(|| format!("writing {}", output.display()))?;
    println!("  T2 lines: {}", rows.len());
    println!("  wrote diagnostics: {}", output.display());
    println!(
        "  {:<8} {:<18} {:<13} {:<16} {:<10} {:>5} {:>5} {:>5} {:>5} {:>5} {:<18} {:>7} Flag",
        "Line",
        "Service",
        "Color",
        "Class",
        "Split",
        "Touch",
        "Near",
        "Dup",
        "Xfer",
        "Stops",
        "Action",
        "Label"
    );
    for row in rows.iter().take(12) {
        println!(
            "  {:<8} {:<18} {:<13} {:<16} {:<10} {:>5} {:>5} {:>5} {:>5} {:>5} {:<18} {:>7.2} {}",
            row.corridor,
            truncate_for_table(row.service_label, 18),
            row.color_mode,
            row.service_class,
            truncate_for_table(row.split_anchor, 10),
            row.unstopped_t1_contact_count,
            row.close_parallel_count,
            row.duplicate_service_count,
            row.transfer_stop_count,
            row.stop_count,
            truncate_for_table(row.service_action, 18),
            row.label_density_per_100px,
            row.review_flag
        );
    }
    if gate {
        let flagged = rows
            .iter()
            .filter(|row| beck_t2_diagnostics_gate_failure(row.review_flag))
            .collect::<Vec<_>>();
        if flagged.is_empty() {
            println!("Beck T2 diagnostics gate: PASS");
        } else {
            println!("Beck T2 diagnostics gate: FAIL");
            for row in flagged.iter().take(10) {
                println!("  - {} {}", row.corridor, row.review_flag);
            }
            anyhow::bail!("Beck T2 diagnostics gate failed");
        }
    }
        
    Ok(())
}

