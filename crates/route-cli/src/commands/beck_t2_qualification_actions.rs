//! `BeckT2QualificationActions` command handler extracted from main.
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

    println!("route beck-t2-qualification-actions");
    let rows = route_map::beck_t2_qualification_actions();
    let csv = route_map::build_beck_t2_qualification_actions_csv();
    if let Some(parent) = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    std::fs::write(&output, csv)
        .with_context(|| format!("writing {}", output.display()))?;
    println!("  qualification actions: {}", rows.len());
    println!("  wrote actions: {}", output.display());
    println!("  {:<20} {:<42} Gate", "Action", "Map treatment");
    for row in &rows {
        println!(
            "  {:<20} {:<42} {}",
            row.service_action,
            truncate_for_table(row.map_treatment, 42),
            row.gate_policy
        );
    }
    if gate {
        let action_rows = rows
            .iter()
            .map(|row| row.service_action)
            .collect::<std::collections::BTreeSet<_>>();
        let covered_pairs = rows
            .iter()
            .flat_map(|row| {
                row.covered_bases
                    .iter()
                    .map(move |basis| (row.service_action, *basis))
            })
            .collect::<std::collections::BTreeSet<_>>();
        let missing = route_map::beck_t2_diagnostics()
            .iter()
            .filter(|row| {
                !action_rows.contains(row.service_action)
                    || !covered_pairs
                        .contains(&(row.service_action, row.qualification_basis))
            })
            .map(|row| {
                format!(
                    "{} {} {}",
                    row.corridor, row.service_action, row.qualification_basis
                )
            })
            .collect::<Vec<_>>();
        if missing.is_empty() {
            println!("Beck T2 qualification actions gate: PASS");
        } else {
            println!("Beck T2 qualification actions gate: FAIL");
            for item in missing.iter().take(10) {
                println!("  - {item}");
            }
            anyhow::bail!("Beck T2 qualification actions gate failed");
        }
    }
        
    Ok(())
}

