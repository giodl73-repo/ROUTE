//! `OptimizerConstraintLedger` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    pavement_debt_budget: PathBuf,
    t2_asset_condition_map_publication_exclusion: PathBuf,
    t1_topology_repairs: PathBuf,
    t1_schematic_geometry_blocker_relief: PathBuf,
    t2_beck_transfer_complexity_blocker_relief: PathBuf,
    t2_beck_label_density_blocker_relief: PathBuf,
    t2_beck_long_connector_blocker_relief: PathBuf,
    t2_game_publication_evidence_blocker_relief: PathBuf,
    t2_game_ops_bundle_evidence_blocker_relief: PathBuf,
    t3_lower_tier_feeder_gap_blocker_relief: PathBuf,
    t2_parallel_service_queue: PathBuf,
    t3_t4_access_gaps: PathBuf,
    t4_terminal_access_map_exclusion: PathBuf,
    t4_terminal_contact_district_proof_import: PathBuf,
    t4_terminal_contact_rejected_proof_sources: PathBuf,
    source_fetch_policy: PathBuf,
    source_snapshot_publication_exclusion: PathBuf,
    t2_scenario_hooks: PathBuf,
    t2_bundle_overlays: PathBuf,
    output: PathBuf,
    details: bool,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route optimizer-constraint-ledger");
    let pavement_rows = load_tier_pavement_debt_budget(&pavement_debt_budget)
        .with_context(|| format!("loading {}", pavement_debt_budget.display()))?;
    let t2_asset_condition_map_publication_exclusion_rows =
        load_t2_asset_condition_map_publication_exclusion(
            &t2_asset_condition_map_publication_exclusion,
        )
        .with_context(|| {
            format!(
                "loading {}",
                t2_asset_condition_map_publication_exclusion.display()
            )
        })?;
    let topology_rows = load_t1_topology_repairs(&t1_topology_repairs)
        .with_context(|| format!("loading {}", t1_topology_repairs.display()))?;
    let schematic_relief_rows =
        load_t1_schematic_geometry_blocker_relief(&t1_schematic_geometry_blocker_relief)
            .with_context(|| {
                format!("loading {}", t1_schematic_geometry_blocker_relief.display())
            })?;
    let t2_transfer_relief_rows = load_t2_beck_transfer_complexity_blocker_relief(
        &t2_beck_transfer_complexity_blocker_relief,
    )
    .with_context(|| {
        format!(
            "loading {}",
            t2_beck_transfer_complexity_blocker_relief.display()
        )
    })?;
    let t2_label_relief_rows =
        load_t2_beck_label_density_blocker_relief(&t2_beck_label_density_blocker_relief)
            .with_context(|| {
                format!("loading {}", t2_beck_label_density_blocker_relief.display())
            })?;
    let t2_long_relief_rows =
        load_t2_beck_long_connector_blocker_relief(&t2_beck_long_connector_blocker_relief)
            .with_context(|| {
                format!(
                    "loading {}",
                    t2_beck_long_connector_blocker_relief.display()
                )
            })?;
    let t2_game_relief_rows = load_t2_game_publication_evidence_blocker_relief(
        &t2_game_publication_evidence_blocker_relief,
    )
    .with_context(|| {
        format!(
            "loading {}",
            t2_game_publication_evidence_blocker_relief.display()
        )
    })?;
    let t2_game_ops_bundle_relief_rows = load_t2_game_ops_bundle_evidence_blocker_relief(
        &t2_game_ops_bundle_evidence_blocker_relief,
    )
    .with_context(|| {
        format!(
            "loading {}",
            t2_game_ops_bundle_evidence_blocker_relief.display()
        )
    })?;
    let t3_feeder_relief_rows = load_t3_lower_tier_feeder_gap_blocker_relief(
        &t3_lower_tier_feeder_gap_blocker_relief,
    )
    .with_context(|| {
        format!(
            "loading {}",
            t3_lower_tier_feeder_gap_blocker_relief.display()
        )
    })?;
    let parallel_rows = load_t2_parallel_service_queue(&t2_parallel_service_queue)
        .with_context(|| format!("loading {}", t2_parallel_service_queue.display()))?;
    let access_gap_rows = load_t3_t4_access_gaps(&t3_t4_access_gaps)
        .with_context(|| format!("loading {}", t3_t4_access_gaps.display()))?;
    let t4_terminal_access_map_exclusion_rows =
        load_t4_terminal_access_map_exclusion(&t4_terminal_access_map_exclusion)
            .with_context(|| {
                format!("loading {}", t4_terminal_access_map_exclusion.display())
            })?;
    let t4_terminal_contact_district_proof_import_rows =
        load_t4_terminal_contact_district_proof_import(
            &t4_terminal_contact_district_proof_import,
        )
        .with_context(|| {
            format!(
                "loading {}",
                t4_terminal_contact_district_proof_import.display()
            )
        })?;
    let t4_terminal_contact_rejected_proof_source_rows =
        load_t4_terminal_contact_rejected_proof_sources(
            &t4_terminal_contact_rejected_proof_sources,
        )
        .with_context(|| {
            format!(
                "loading {}",
                t4_terminal_contact_rejected_proof_sources.display()
            )
        })?;
    let mut source_policy_rows = load_source_fetch_policy(&source_fetch_policy)
        .with_context(|| format!("loading {}", source_fetch_policy.display()))?;
    if source_policy_rows.is_empty() {
        source_policy_rows = source_fetch_policy_rows();
    }
    let source_snapshot_publication_exclusion_rows =
        load_source_snapshot_publication_exclusion(&source_snapshot_publication_exclusion)
            .with_context(|| {
                format!(
                    "loading {}",
                    source_snapshot_publication_exclusion.display()
                )
            })?;
    let scenario_hook_rows = load_t2_scenario_hooks(&t2_scenario_hooks)
        .with_context(|| format!("loading {}", t2_scenario_hooks.display()))?;
    let bundle_overlay_rows = load_t2_bundle_overlays(&t2_bundle_overlays)
        .with_context(|| format!("loading {}", t2_bundle_overlays.display()))?;
    let rows = support::optimizer::optimizer_ledger::optimizer_constraint_ledger_rows_with_terminal_proof(
        &pavement_rows,
        &t2_asset_condition_map_publication_exclusion_rows,
        &topology_rows,
        &schematic_relief_rows,
        &t2_transfer_relief_rows,
        &t2_label_relief_rows,
        &t2_long_relief_rows,
        &t2_game_relief_rows,
        &t2_game_ops_bundle_relief_rows,
        &t3_feeder_relief_rows,
        &parallel_rows,
        &access_gap_rows,
        &t4_terminal_access_map_exclusion_rows,
        &t4_terminal_contact_district_proof_import_rows,
        &t4_terminal_contact_rejected_proof_source_rows,
        &route_map::beck_t1_diagnostics(),
        &route_map::beck_t2_diagnostics(),
        &source_policy_rows,
        &source_snapshot_publication_exclusion_rows,
        &scenario_hook_rows,
        &bundle_overlay_rows,
    );
    write_optimizer_constraint_ledger(&output, &rows)
        .with_context(|| format!("writing {}", output.display()))?;
    print_optimizer_constraint_ledger_summary(&output, &rows, details);

    if gate {
        let failures = optimizer_constraint_ledger_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("optimizer constraint ledger gate: FAIL");
            for failure in failures.iter().take(20) {
                println!("  - {failure}");
            }
            anyhow::bail!("optimizer constraint ledger gate failed");
        }
        println!();
        println!("optimizer constraint ledger gate: PASS");
    }
        
    Ok(())
}

