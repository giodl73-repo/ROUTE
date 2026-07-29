//! `LowerTierPressureWitnesses` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    tier_table: PathBuf,
    candidates: PathBuf,
    resolutions: PathBuf,
    route_family_splits: PathBuf,
    graph_contact_validation: PathBuf,
    contact_closure: PathBuf,
    endpoint_closure: PathBuf,
    output: PathBuf,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route lower-tier-pressure-witnesses");
    let tier_rows = load_tier_table_rows(&tier_table)
        .with_context(|| format!("loading {}", tier_table.display()))?;
    let candidate_rows = load_tier_candidate_columns(&candidates)
        .with_context(|| format!("loading {}", candidates.display()))?;
    let resolution_rows = load_t2_contact_resolutions(&resolutions)
        .with_context(|| format!("loading {}", resolutions.display()))?;
    let route_family_rows = load_t2_route_family_splits(&route_family_splits)
        .with_context(|| format!("loading {}", route_family_splits.display()))?;
    let graph_rows = load_t2_graph_contact_validation(&graph_contact_validation)
        .with_context(|| format!("loading {}", graph_contact_validation.display()))?;
    let contact_rows = load_t2_contact_closure(&contact_closure)
        .with_context(|| format!("loading {}", contact_closure.display()))?;
    let endpoint_rows = load_t2_endpoint_closure(&endpoint_closure)
        .with_context(|| format!("loading {}", endpoint_closure.display()))?;
    let dispositions = t2_closure_dispositions(
        &route_family_rows,
        &graph_rows,
        &contact_rows,
        &endpoint_rows,
        &[],
    );
    let rows = lower_tier_pressure_witness_rows(
        &tier_rows,
        &candidate_rows,
        &resolution_rows,
        &dispositions,
    );
    write_lower_tier_pressure_witnesses(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_lower_tier_pressure_witness_summary(&output, &rows);

    if gate {
        let failures = lower_tier_pressure_witness_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("lower-tier pressure witness gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("lower-tier pressure witness gate failed");
        }
        println!();
        println!("lower-tier pressure witness gate: PASS");
    }
        
    Ok(())
}

