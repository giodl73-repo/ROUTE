//! `BeckT2ServiceStandards` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(ctx: &ctx::Ctx<'_>, output: PathBuf, gate: bool) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route beck-t2-service-standards");
    let rows = route_map::beck_t2_service_standards();
    let csv = route_map::build_beck_t2_service_standards_csv();
    if let Some(parent) = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    std::fs::write(&output, csv).with_context(|| format!("writing {}", output.display()))?;
    println!("  service classes: {}", rows.len());
    println!("  wrote standards: {}", output.display());
    println!("  {:<16} {:<34} {:<18} Gate", "Class", "Visual", "Review");
    for row in &rows {
        println!(
            "  {:<16} {:<34} {:<18} {}",
            row.service_class,
            truncate_for_table(row.visual_convention, 34),
            truncate_for_table(row.review_policy, 18),
            row.gate_policy
        );
    }
    if gate {
        let standard_classes = rows
            .iter()
            .map(|row| row.service_class)
            .collect::<std::collections::BTreeSet<_>>();
        let missing = route_map::beck_t2_diagnostics()
            .iter()
            .filter(|row| !standard_classes.contains(row.service_class))
            .map(|row| format!("{} {}", row.corridor, row.service_class))
            .collect::<Vec<_>>();
        if missing.is_empty() {
            println!("Beck T2 service standards gate: PASS");
        } else {
            println!("Beck T2 service standards gate: FAIL");
            for item in missing.iter().take(10) {
                println!("  - {item}");
            }
            anyhow::bail!("Beck T2 service standards gate failed");
        }
    }

    Ok(())
}
