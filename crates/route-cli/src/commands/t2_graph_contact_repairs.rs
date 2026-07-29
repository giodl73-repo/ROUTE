//! `T2GraphContactRepairs` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    held_actions: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route t2-graph-contact-repairs");
    let held_rows = load_t2_held_contact_actions(&held_actions)
        .with_context(|| format!("loading {}", held_actions.display()))?;
    let rows = t2_graph_contact_repair_rows(&held_rows);
    write_t2_graph_contact_repairs(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_t2_graph_contact_repair_summary(&output, &rows);

    if gate {
        let failures = t2_graph_contact_repair_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("T2 graph contact repair gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("T2 graph contact repair gate failed");
        }
        println!();
        println!("T2 graph contact repair gate: PASS");
    }
        
    Ok(())
}

