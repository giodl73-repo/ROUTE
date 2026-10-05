//! Extracted from main.rs for maintainability.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn optimizer_constraint_ledger_rows_with_terminal_proof(
    pavement_rows: &[TierPavementDebtBudgetRow],
    t2_asset_condition_map_publication_exclusion_rows: &[T2AssetConditionMapPublicationExclusionRow],
    topology_rows: &[T1TopologyRepairRow],
    schematic_relief_rows: &[T1SchematicGeometryBlockerReliefRow],
    t2_transfer_relief_rows: &[T2BeckTransferComplexityBlockerReliefRow],
    t2_label_relief_rows: &[T2BeckLabelDensityBlockerReliefRow],
    t2_long_relief_rows: &[T2BeckLongConnectorBlockerReliefRow],
    t2_game_relief_rows: &[T2GamePublicationEvidenceBlockerReliefRow],
    t2_game_ops_bundle_relief_rows: &[T2GameOpsBundleEvidenceBlockerReliefRow],
    t3_feeder_relief_rows: &[T3LowerTierFeederGapBlockerReliefRow],
    parallel_rows: &[T2ParallelServiceQueueRow],
    access_gap_rows: &[T3T4AccessGapRow],
    t4_terminal_access_map_exclusion_rows: &[T4TerminalAccessMapExclusionRow],
    t4_terminal_contact_district_proof_import_rows: &[T4TerminalContactDistrictProofImportRow],
    t4_terminal_contact_rejected_proof_source_rows: &[T4TerminalContactRejectedProofSourceRow],
    beck_t1_rows: &[route_map::BeckT1DiagnosticRow],
    beck_t2_rows: &[route_map::BeckT2DiagnosticRow],
    source_policy_rows: &[SourceFetchPolicyRow],
    source_snapshot_publication_exclusion_rows: &[SourceSnapshotPublicationExclusionRow],
    scenario_hook_rows: &[T2ScenarioHookRow],
    bundle_overlay_rows: &[T2BundleOverlayRow],
) -> Vec<OptimizerConstraintLedgerRow> {
    let mut rows = Vec::new();
    let relieved_t1_schematic_routes = t1_schematic_relief_route_set(schematic_relief_rows);
    let relieved_t2_transfer_routes = t2_transfer_relief_route_set(t2_transfer_relief_rows);
    let relieved_t2_label_routes = t2_label_density_relief_route_set(t2_label_relief_rows);
    let relieved_t2_long_routes = t2_long_connector_relief_route_set(t2_long_relief_rows);
    let relieved_t2_game_scenarios = t2_game_publication_relief_scenario_set(t2_game_relief_rows);
    let relieved_t2_game_ops_bundles =
        t2_game_ops_bundle_relief_bundle_set(t2_game_ops_bundle_relief_rows);
    let relieved_t3_feeder_routes = t3_feeder_relief_route_set(t3_feeder_relief_rows);
    let t2_asset_condition_map_publication_exclusion =
        accepted_t2_asset_condition_map_publication_exclusion(
            t2_asset_condition_map_publication_exclusion_rows,
        );
    let t4_terminal_access_map_exclusion =
        accepted_t4_terminal_access_map_exclusion(t4_terminal_access_map_exclusion_rows);
    let accepted_t4_terminal_proof_routes =
        accepted_t4_terminal_proof_route_set(t4_terminal_contact_district_proof_import_rows);
    let rejected_t4_terminal_proof_routes =
        rejected_t4_terminal_proof_route_set(t4_terminal_contact_rejected_proof_source_rows);
    let source_snapshot_publication_exclusion =
        accepted_source_snapshot_publication_exclusion(source_snapshot_publication_exclusion_rows);

    for row in source_policy_rows {
        let snapshot_hold = row.mutation_mode == "live-snapshot-preserve";
        let snapshot_publication_excluded = snapshot_hold
            && source_snapshot_publication_exclusion
                .as_ref()
                .is_some_and(|decision| decision.affected_fetch_family == row.fetch_family);
        rows.push(OptimizerConstraintLedgerRow {
            constraint_id: format!("CON-SOURCE-{}", stable_id_fragment(&row.fetch_family)),
            optimizer_run_id: "tier-optimizer-current".to_string(),
            tier: "all".to_string(),
            region_id: "source-acquisition".to_string(),
            constraint_order: 0,
            constraint_class: if snapshot_hold {
                "source_acquisition_snapshot_guard".to_string()
            } else {
                "source_acquisition_contract".to_string()
            },
            behavior_type: if snapshot_hold {
                "claim-blocker".to_string()
            } else {
                "review".to_string()
            },
            constraint_scope: "source".to_string(),
            subject_id: row.fetch_family.clone(),
            segment_bundle_id: String::new(),
            national_segment_id: String::new(),
            stitch_group_id: String::new(),
            route: String::new(),
            stop_id: String::new(),
            pair_id: String::new(),
            map_id: String::new(),
            source_artifact: "data/source-fetch-policy.csv".to_string(),
            source_row_id: row.fetch_family.clone(),
            standard_artifact: row.policy_doc.clone(),
            evidence_status: if snapshot_hold {
                "source-needed".to_string()
            } else {
                "accepted".to_string()
            },
            constraint_status: if snapshot_hold {
                "review".to_string()
            } else {
                row.validation_status.clone()
            },
            observed_value: row.mutation_mode.clone(),
            threshold_value: if snapshot_hold {
                "archive-or-repeat-window".to_string()
            } else {
                row.mutation_mode.clone()
            },
            measurement_unit: "source_fetch_policy".to_string(),
            blocks_claims: if let Some(decision) = source_snapshot_publication_exclusion
                .as_ref()
                .filter(|_| snapshot_publication_excluded)
            {
                decision.preserved_claims_after.clone()
            } else if snapshot_hold {
                "evidence|publication".to_string()
            } else {
                String::new()
            },
            budget_cost_m: 0.0,
            cost_category: String::new(),
            cost_basis: String::new(),
            cost_confidence: String::new(),
            budget_units: format!("commands={}", row.commands.split(';').count()),
            penalty_score: if snapshot_hold { 1.0 } else { 0.0 },
            repair_action: if snapshot_hold {
                "accumulate-repeat-window-or-archive-history".to_string()
            } else {
                "preserve-cache-contract".to_string()
            },
            payment_action: String::new(),
            owner_jurisdiction: "route-program".to_string(),
            funding_program: String::new(),
            delivery_risk: if snapshot_hold {
                "unknown".to_string()
            } else {
                "low".to_string()
            },
            exception_id: if let Some(decision) = source_snapshot_publication_exclusion
                .as_ref()
                .filter(|_| snapshot_publication_excluded)
            {
                decision.decision_id.clone()
            } else if snapshot_hold {
                row.fetch_family.clone()
            } else {
                String::new()
            },
            exception_artifact: if snapshot_publication_excluded {
                "data/source-snapshot-publication-exclusion.csv".to_string()
            } else if snapshot_hold {
                "data/source-fetch-policy.csv".to_string()
            } else {
                String::new()
            },
            next_artifact: if snapshot_hold {
                "data/t1-evidence-windows.csv".to_string()
            } else {
                row.policy_doc.clone()
            },
            optimizer_effect: if let Some(decision) = source_snapshot_publication_exclusion
                .as_ref()
                .filter(|_| snapshot_publication_excluded)
            {
                format!(
                    "{}; publication excluded by {} while {} remains blocked",
                    row.preservation_contract,
                    decision.decision_id,
                    decision.preserved_claims_after
                )
            } else {
                row.preservation_contract.clone()
            },
            validation_status: if snapshot_hold {
                "review".to_string()
            } else {
                row.validation_status.clone()
            },
        });
    }

    for row in pavement_rows {
        let repair_debt = row.debt_class.contains("repair");
        let publication_excluded =
            row.tier == "T2" && t2_asset_condition_map_publication_exclusion.is_some();
        rows.push(OptimizerConstraintLedgerRow {
            constraint_id: format!(
                "CON-PAVEMENT-{}",
                stable_id_fragment(&row.segment_bundle_id)
            ),
            optimizer_run_id: "tier-optimizer-current".to_string(),
            tier: row.tier.clone(),
            region_id: row.region_id.clone(),
            constraint_order: 8,
            constraint_class: "asset_condition_debt".to_string(),
            behavior_type: "budget-debt".to_string(),
            constraint_scope: "bundle".to_string(),
            subject_id: row.segment_bundle_id.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            national_segment_id: String::new(),
            stitch_group_id: row.stitch_group_id.clone(),
            route: row.route.clone(),
            stop_id: String::new(),
            pair_id: String::new(),
            map_id: String::new(),
            source_artifact: "data/tier-pavement-debt-budget.csv".to_string(),
            source_row_id: row.segment_bundle_id.clone(),
            standard_artifact: "docs/tier-pavement-standards.md".to_string(),
            evidence_status: if repair_debt {
                "accepted".to_string()
            } else {
                "source-needed".to_string()
            },
            constraint_status: "debt".to_string(),
            observed_value: row.blocked_member_count.to_string(),
            threshold_value: "0".to_string(),
            measurement_unit: "blocked_members".to_string(),
            blocks_claims: if let Some(decision) = t2_asset_condition_map_publication_exclusion
                .as_ref()
                .filter(|_| publication_excluded)
            {
                decision.preserved_claims_after.clone()
            } else {
                "sla|transit|upgrade|publication".to_string()
            },
            budget_cost_m: row.total_debt_cost_m,
            cost_category: if repair_debt {
                "capital_repair".to_string()
            } else {
                "source_acquisition".to_string()
            },
            cost_basis: row.budget_basis.clone(),
            cost_confidence: "planning_proxy".to_string(),
            budget_units: format!(
                "evidence_members={};repair_members={}",
                row.evidence_debt_units, row.repair_debt_units
            ),
            penalty_score: row.total_debt_cost_m,
            repair_action: if repair_debt {
                "pay_debt".to_string()
            } else {
                "source_needed".to_string()
            },
            payment_action: if repair_debt {
                "fund_pavement_repair".to_string()
            } else {
                "fund_source_acquisition".to_string()
            },
            owner_jurisdiction: row.affected_states.clone(),
            funding_program: "state_dot_hpms_or_nhpp".to_string(),
            delivery_risk: if repair_debt { "medium" } else { "unknown" }.to_string(),
            exception_id: if let Some(decision) = t2_asset_condition_map_publication_exclusion
                .as_ref()
                .filter(|_| publication_excluded)
            {
                decision.decision_id.clone()
            } else {
                String::new()
            },
            exception_artifact: if publication_excluded {
                "data/t2-asset-condition-map-publication-exclusion.csv".to_string()
            } else {
                String::new()
            },
            next_artifact: row.next_artifact.clone(),
            optimizer_effect: if let Some(decision) = t2_asset_condition_map_publication_exclusion
                .as_ref()
                .filter(|_| publication_excluded)
            {
                format!(
                    "{}; publication excluded by {} while {} remains blocked",
                    row.optimizer_penalty, decision.decision_id, decision.preserved_claims_after
                )
            } else {
                row.optimizer_penalty.clone()
            },
            validation_status: row.validation_status.clone(),
        });
    }

    for row in topology_rows
        .iter()
        .filter(|row| !relieved_t1_schematic_routes.contains(&route_display_key(&row.route)))
    {
        let (constraint_order, constraint_class, behavior_type, constraint_status, blocks_claims) =
            t1_topology_constraint_mapping(row);
        rows.push(OptimizerConstraintLedgerRow {
            constraint_id: format!("CON-T1TOPO-{}", stable_id_fragment(&row.route)),
            optimizer_run_id: "tier-optimizer-current".to_string(),
            tier: "T1".to_string(),
            region_id: "national".to_string(),
            constraint_order,
            constraint_class: constraint_class.to_string(),
            behavior_type: behavior_type.to_string(),
            constraint_scope: "route".to_string(),
            subject_id: row.route.clone(),
            segment_bundle_id: String::new(),
            national_segment_id: String::new(),
            stitch_group_id: String::new(),
            route: row.route.clone(),
            stop_id: String::new(),
            pair_id: String::new(),
            map_id: "beck-schematic".to_string(),
            source_artifact: "data/t1-topology-repairs.csv".to_string(),
            source_row_id: row.route.clone(),
            standard_artifact: "docs/tier-optimizer-design.md".to_string(),
            evidence_status: if row.validation_status == "pass" {
                "accepted".to_string()
            } else {
                "exception".to_string()
            },
            constraint_status: constraint_status.to_string(),
            observed_value: row.design_status.clone(),
            threshold_value: "accepted".to_string(),
            measurement_unit: "design_status".to_string(),
            blocks_claims: blocks_claims.to_string(),
            budget_cost_m: 0.0,
            cost_category: String::new(),
            cost_basis: String::new(),
            cost_confidence: String::new(),
            budget_units: String::new(),
            penalty_score: if row.validation_status == "review" {
                1.0
            } else {
                0.0
            },
            repair_action: row.next_action.clone(),
            payment_action: String::new(),
            owner_jurisdiction: "route-program".to_string(),
            funding_program: String::new(),
            delivery_risk: "unknown".to_string(),
            exception_id: if row.validation_status == "review" {
                row.next_action.clone()
            } else {
                String::new()
            },
            exception_artifact: if row.validation_status == "review" {
                row.next_artifact.clone()
            } else {
                String::new()
            },
            next_artifact: row.next_artifact.clone(),
            optimizer_effect: row.repair_basis.clone(),
            validation_status: row.validation_status.clone(),
        });
    }

    for row in parallel_rows {
        rows.push(OptimizerConstraintLedgerRow {
            constraint_id: format!("CON-T2PAR-{}", stable_id_fragment(&row.route)),
            optimizer_run_id: "tier-optimizer-current".to_string(),
            tier: "T2".to_string(),
            region_id: row.region_id.clone(),
            constraint_order: 11,
            constraint_class: "duplication_and_parallel_service".to_string(),
            behavior_type: if row.validation_status == "pass" {
                "review".to_string()
            } else {
                "penalty-soft".to_string()
            },
            constraint_scope: "route".to_string(),
            subject_id: row.route.clone(),
            segment_bundle_id: String::new(),
            national_segment_id: String::new(),
            stitch_group_id: String::new(),
            route: row.route.clone(),
            stop_id: String::new(),
            pair_id: String::new(),
            map_id: "beck-schematic-t2-only".to_string(),
            source_artifact: "data/t2-parallel-service-queue.csv".to_string(),
            source_row_id: row.route.clone(),
            standard_artifact: "docs/t2-regional-treatment.md".to_string(),
            evidence_status: "accepted".to_string(),
            constraint_status: if row.validation_status == "pass" {
                "pass".to_string()
            } else {
                "review".to_string()
            },
            observed_value: row.close_parallel_count.to_string(),
            threshold_value: "0".to_string(),
            measurement_unit: "close_parallel_services".to_string(),
            blocks_claims: if row.validation_status == "pass" {
                String::new()
            } else {
                "promotion|map|publication".to_string()
            },
            budget_cost_m: 0.0,
            cost_category: String::new(),
            cost_basis: String::new(),
            cost_confidence: String::new(),
            budget_units: String::new(),
            penalty_score: row.close_parallel_count as f64,
            repair_action: row.parallel_action.clone(),
            payment_action: String::new(),
            owner_jurisdiction: "route-program".to_string(),
            funding_program: String::new(),
            delivery_risk: "unknown".to_string(),
            exception_id: String::new(),
            exception_artifact: String::new(),
            next_artifact: row.next_artifact.clone(),
            optimizer_effect: row.optimizer_effect.clone(),
            validation_status: row.validation_status.clone(),
        });
    }

    for row in access_gap_rows.iter().filter(|row| {
        let route_key = route_display_key(&row.route);
        !(row.gap_class == "below-threshold-feeder"
            && row.promise_horizon_hours == 6
            && relieved_t3_feeder_routes.contains(&route_key))
            && !(row.gap_class == "terminal-evidence-needed"
                && (accepted_t4_terminal_proof_routes.contains(&route_key)
                    || rejected_t4_terminal_proof_routes.contains(&route_key)))
    }) {
        let tier = if row.source_surface == "t4-terminal-access-columns"
            || row.promise_horizon_hours == 1
        {
            "T4"
        } else {
            "T3"
        };
        let (constraint_class, repair_action) = match row.gap_class.as_str() {
            "below-threshold-feeder" => ("lower_tier_feeder_gap", row.repair_action.as_str()),
            "terminal-evidence-needed" => {
                ("terminal_access_evidence_gap", row.repair_action.as_str())
            }
            "zone-assignment-needed" => ("zone_assignment_gap", row.repair_action.as_str()),
            _ => ("lower_tier_access_gap", row.repair_action.as_str()),
        };
        let map_publication_excluded =
            t4_terminal_access_map_exclusion
                .as_ref()
                .is_some_and(|decision| {
                    tier == "T4"
                        && constraint_class == "terminal_access_evidence_gap"
                        && row.gap_class == decision.affected_gap_class
                });
        rows.push(OptimizerConstraintLedgerRow {
            constraint_id: format!("CON-T3T4-{}", stable_id_fragment(&row.gap_id)),
            optimizer_run_id: "tier-optimizer-current".to_string(),
            tier: tier.to_string(),
            region_id: row.zone_id.clone(),
            constraint_order: 12,
            constraint_class: constraint_class.to_string(),
            behavior_type: "claim-blocker".to_string(),
            constraint_scope: "route".to_string(),
            subject_id: row.route.clone(),
            segment_bundle_id: String::new(),
            national_segment_id: String::new(),
            stitch_group_id: String::new(),
            route: row.route.clone(),
            stop_id: String::new(),
            pair_id: String::new(),
            map_id: row.zone_id.clone(),
            source_artifact: "data/t3-t4-access-gaps.csv".to_string(),
            source_row_id: row.gap_id.clone(),
            standard_artifact: "docs/t3-t4-access-optimization.md".to_string(),
            evidence_status: if map_publication_excluded {
                "exception".to_string()
            } else {
                "review".to_string()
            },
            constraint_status: "review".to_string(),
            observed_value: row.current_score.to_string(),
            threshold_value: if row.promise_horizon_hours == 1 {
                "terminal evidence".to_string()
            } else {
                format!("{T3_THRESHOLD:.1}")
            },
            measurement_unit: if row.promise_horizon_hours == 1 {
                "terminal_access_evidence".to_string()
            } else {
                "route_score".to_string()
            },
            blocks_claims: if let Some(decision) = t4_terminal_access_map_exclusion
                .as_ref()
                .filter(|_| map_publication_excluded)
            {
                decision.preserved_claims_after.clone()
            } else {
                "upgrade|map|publication".to_string()
            },
            budget_cost_m: 0.0,
            cost_category: String::new(),
            cost_basis: String::new(),
            cost_confidence: String::new(),
            budget_units: format!("promise_horizon_hours={}", row.promise_horizon_hours),
            penalty_score: 1.0,
            repair_action: repair_action.to_string(),
            payment_action: String::new(),
            owner_jurisdiction: "route-program".to_string(),
            funding_program: String::new(),
            delivery_risk: "unknown".to_string(),
            exception_id: if let Some(decision) = t4_terminal_access_map_exclusion
                .as_ref()
                .filter(|_| map_publication_excluded)
            {
                decision.decision_id.clone()
            } else {
                row.gap_class.clone()
            },
            exception_artifact: if map_publication_excluded {
                "data/t4-terminal-access-map-exclusion.csv".to_string()
            } else {
                "data/t3-t4-access-gaps.csv".to_string()
            },
            next_artifact: row.next_artifact.clone(),
            optimizer_effect: if let Some(decision) = t4_terminal_access_map_exclusion
                .as_ref()
                .filter(|_| map_publication_excluded)
            {
                format!(
                    "{}; map/publication excluded by {} while {} remains blocked",
                    row.gap_reason, decision.decision_id, decision.preserved_claims_after
                )
            } else {
                row.gap_reason.clone()
            },
            validation_status: row.validation_status.clone(),
        });
    }

    for row in t3_feeder_relief_rows.iter().filter(|row| {
        row.relief_decision == "relief-ready-for-constraint-ledger-replay"
            && row.blocker_count_after == 0
            && row.claim_blocker_delta < 0
    }) {
        rows.push(OptimizerConstraintLedgerRow {
            constraint_id: format!("CON-T3FEEDERRELIEF-{}", stable_id_fragment(&row.route)),
            optimizer_run_id: "tier-optimizer-current".to_string(),
            tier: "T3".to_string(),
            region_id: row.zone_id.clone(),
            constraint_order: 12,
            constraint_class: "lower_tier_feeder_gap_relief".to_string(),
            behavior_type: "review".to_string(),
            constraint_scope: "route".to_string(),
            subject_id: row.route.clone(),
            segment_bundle_id: String::new(),
            national_segment_id: String::new(),
            stitch_group_id: String::new(),
            route: row.route.clone(),
            stop_id: String::new(),
            pair_id: String::new(),
            map_id: row.zone_id.clone(),
            source_artifact: "data/t3-lower-tier-feeder-gap-blocker-relief.csv".to_string(),
            source_row_id: row.relief_id.clone(),
            standard_artifact: "docs/t3-t4-access-optimization.md".to_string(),
            evidence_status: "accepted".to_string(),
            constraint_status: "pass".to_string(),
            observed_value: row.relief_decision.clone(),
            threshold_value: "relief-ready-for-constraint-ledger-replay".to_string(),
            measurement_unit: "relief_decision".to_string(),
            blocks_claims: String::new(),
            budget_cost_m: 0.0,
            cost_category: String::new(),
            cost_basis: String::new(),
            cost_confidence: String::new(),
            budget_units: format!("claim_blocker_delta={}", row.claim_blocker_delta),
            penalty_score: 0.0,
            repair_action: "constraint-ledger-replay-applied".to_string(),
            payment_action: String::new(),
            owner_jurisdiction: "route-program".to_string(),
            funding_program: String::new(),
            delivery_risk: "low".to_string(),
            exception_id: row.acceptance_id.clone(),
            exception_artifact: "data/t3-lower-tier-feeder-gap-policy-acceptance.csv".to_string(),
            next_artifact: "data/optimizer-constraint-budget.csv".to_string(),
            optimizer_effect:
                "accepted lower-tier feeder policy removes T3 map publication upgrade blockers"
                    .to_string(),
            validation_status: "pass".to_string(),
        });
    }

    for row in beck_t1_rows.iter().filter(|row| {
        row.review_flag != "ok"
            && !relieved_t1_schematic_routes.contains(&route_display_key(row.corridor))
    }) {
        rows.push(OptimizerConstraintLedgerRow {
            constraint_id: format!("CON-BECKT1-{}", stable_id_fragment(row.corridor)),
            optimizer_run_id: "tier-optimizer-current".to_string(),
            tier: "T1".to_string(),
            region_id: "national".to_string(),
            constraint_order: 13,
            constraint_class: "beck_schematic_geometry".to_string(),
            behavior_type: "claim-blocker".to_string(),
            constraint_scope: "route".to_string(),
            subject_id: row.corridor.to_string(),
            segment_bundle_id: String::new(),
            national_segment_id: String::new(),
            stitch_group_id: String::new(),
            route: normalise_designation(row.corridor),
            stop_id: String::new(),
            pair_id: String::new(),
            map_id: "beck-schematic".to_string(),
            source_artifact: "data/beck-t1-diagnostics.csv".to_string(),
            source_row_id: row.corridor.to_string(),
            standard_artifact: "docs/beck-renderer-contract.md".to_string(),
            evidence_status: "accepted".to_string(),
            constraint_status: "review".to_string(),
            observed_value: row.review_flag.to_string(),
            threshold_value: "ok".to_string(),
            measurement_unit: "beck_review_flag".to_string(),
            blocks_claims: "map|publication".to_string(),
            budget_cost_m: 0.0,
            cost_category: String::new(),
            cost_basis: String::new(),
            cost_confidence: String::new(),
            budget_units: format!(
                "shared_segments={};shared_stops={}",
                row.shared_segment_count, row.shared_stop_count
            ),
            penalty_score: 1.0 + row.shared_segment_count as f64,
            repair_action: row.service_action.to_string(),
            payment_action: String::new(),
            owner_jurisdiction: "route-program".to_string(),
            funding_program: String::new(),
            delivery_risk: "unknown".to_string(),
            exception_id: row.review_flag.to_string(),
            exception_artifact: "data/beck-t1-diagnostics.csv".to_string(),
            next_artifact: "data/t1-design-policy-actions.csv".to_string(),
            optimizer_effect: row.qualification_basis.to_string(),
            validation_status: "review".to_string(),
        });
    }

    for row in schematic_relief_rows.iter().filter(|row| {
        row.relief_decision == "relief-ready-for-constraint-ledger-replay"
            && row.blocker_count_after == 0
            && row.claim_blocker_delta < 0
    }) {
        rows.push(OptimizerConstraintLedgerRow {
            constraint_id: format!("CON-T1SCHEMRELIEF-{}", stable_id_fragment(&row.route_pair)),
            optimizer_run_id: "tier-optimizer-current".to_string(),
            tier: "T1".to_string(),
            region_id: "national".to_string(),
            constraint_order: 13,
            constraint_class: "schematic_geometry_relief".to_string(),
            behavior_type: "review".to_string(),
            constraint_scope: "route".to_string(),
            subject_id: row.route_pair.clone(),
            segment_bundle_id: String::new(),
            national_segment_id: String::new(),
            stitch_group_id: String::new(),
            route: row.affected_routes.clone(),
            stop_id: String::new(),
            pair_id: row.route_pair.clone(),
            map_id: "beck-schematic".to_string(),
            source_artifact: "data/t1-schematic-geometry-blocker-relief.csv".to_string(),
            source_row_id: row.relief_id.clone(),
            standard_artifact: "docs/beck-renderer-contract.md".to_string(),
            evidence_status: "accepted".to_string(),
            constraint_status: "pass".to_string(),
            observed_value: row.relief_decision.clone(),
            threshold_value: "relief-ready-for-constraint-ledger-replay".to_string(),
            measurement_unit: "relief_decision".to_string(),
            blocks_claims: String::new(),
            budget_cost_m: 0.0,
            cost_category: String::new(),
            cost_basis: String::new(),
            cost_confidence: String::new(),
            budget_units: format!("claim_blocker_delta={}", row.claim_blocker_delta),
            penalty_score: 0.0,
            repair_action: "constraint-ledger-replay-applied".to_string(),
            payment_action: String::new(),
            owner_jurisdiction: "route-program".to_string(),
            funding_program: String::new(),
            delivery_risk: "low".to_string(),
            exception_id: row.acceptance_id.clone(),
            exception_artifact: "data/t1-shared-segment-policy-acceptance.csv".to_string(),
            next_artifact: "data/optimizer-constraint-budget.csv".to_string(),
            optimizer_effect:
                "accepted shared-segment policy removes T1 schematic map publication blockers"
                    .to_string(),
            validation_status: "pass".to_string(),
        });
    }

    for row in beck_t2_rows.iter().filter(|row| {
        row.review_flag != "ok"
            && !(row.review_flag == "transfer-complexity-review"
                && relieved_t2_transfer_routes.contains(&route_display_key(row.corridor)))
            && !(matches!(
                row.review_flag,
                "dense-label-review" | "dense-transfer-review"
            ) && relieved_t2_label_routes.contains(&route_display_key(row.corridor)))
            && !(row.review_flag == "long-connector-review"
                && relieved_t2_long_routes.contains(&route_display_key(row.corridor)))
    }) {
        let (constraint_class, repair_action, next_artifact) =
            beck_t2_constraint_mapping(row.review_flag);
        rows.push(OptimizerConstraintLedgerRow {
            constraint_id: format!("CON-BECKT2-{}", stable_id_fragment(row.corridor)),
            optimizer_run_id: "tier-optimizer-current".to_string(),
            tier: "T2".to_string(),
            region_id: String::new(),
            constraint_order: 13,
            constraint_class: constraint_class.to_string(),
            behavior_type: "claim-blocker".to_string(),
            constraint_scope: "route".to_string(),
            subject_id: row.corridor.to_string(),
            segment_bundle_id: String::new(),
            national_segment_id: String::new(),
            stitch_group_id: String::new(),
            route: normalise_designation(row.corridor),
            stop_id: String::new(),
            pair_id: String::new(),
            map_id: "beck-schematic-t2-only".to_string(),
            source_artifact: "data/beck-t2-diagnostics.csv".to_string(),
            source_row_id: row.corridor.to_string(),
            standard_artifact: "docs/beck-renderer-contract.md".to_string(),
            evidence_status: "accepted".to_string(),
            constraint_status: "review".to_string(),
            observed_value: row.review_flag.to_string(),
            threshold_value: "ok".to_string(),
            measurement_unit: "beck_review_flag".to_string(),
            blocks_claims: "map|promotion|publication".to_string(),
            budget_cost_m: 0.0,
            cost_category: String::new(),
            cost_basis: String::new(),
            cost_confidence: String::new(),
            budget_units: format!(
                "stops={};transfers={};label_density={:.2}",
                row.stop_count, row.transfer_stop_count, row.label_density_per_100px
            ),
            penalty_score: beck_t2_constraint_penalty(row),
            repair_action: repair_action.to_string(),
            payment_action: String::new(),
            owner_jurisdiction: "route-program".to_string(),
            funding_program: String::new(),
            delivery_risk: "unknown".to_string(),
            exception_id: row.review_flag.to_string(),
            exception_artifact: "data/beck-t2-diagnostics.csv".to_string(),
            next_artifact: next_artifact.to_string(),
            optimizer_effect: row.qualification_basis.to_string(),
            validation_status: "review".to_string(),
        });
    }

    for row in t2_transfer_relief_rows.iter().filter(|row| {
        row.relief_decision == "relief-ready-for-constraint-ledger-replay"
            && row.blocker_count_after == 0
            && row.claim_blocker_delta < 0
    }) {
        rows.push(OptimizerConstraintLedgerRow {
            constraint_id: format!("CON-T2TRANSFERRELIEF-{}", stable_id_fragment(&row.route)),
            optimizer_run_id: "tier-optimizer-current".to_string(),
            tier: "T2".to_string(),
            region_id: String::new(),
            constraint_order: 13,
            constraint_class: "beck_transfer_complexity_relief".to_string(),
            behavior_type: "review".to_string(),
            constraint_scope: "route".to_string(),
            subject_id: row.route.clone(),
            segment_bundle_id: String::new(),
            national_segment_id: String::new(),
            stitch_group_id: String::new(),
            route: row.route.clone(),
            stop_id: String::new(),
            pair_id: String::new(),
            map_id: "beck-schematic-t2-only".to_string(),
            source_artifact: "data/t2-beck-transfer-complexity-blocker-relief.csv".to_string(),
            source_row_id: row.relief_id.clone(),
            standard_artifact: "docs/beck-renderer-contract.md".to_string(),
            evidence_status: "accepted".to_string(),
            constraint_status: "pass".to_string(),
            observed_value: row.relief_decision.clone(),
            threshold_value: "relief-ready-for-constraint-ledger-replay".to_string(),
            measurement_unit: "relief_decision".to_string(),
            blocks_claims: String::new(),
            budget_cost_m: 0.0,
            cost_category: String::new(),
            cost_basis: String::new(),
            cost_confidence: String::new(),
            budget_units: format!("claim_blocker_delta={}", row.claim_blocker_delta),
            penalty_score: 0.0,
            repair_action: "constraint-ledger-replay-applied".to_string(),
            payment_action: String::new(),
            owner_jurisdiction: "route-program".to_string(),
            funding_program: String::new(),
            delivery_risk: "low".to_string(),
            exception_id: row.acceptance_id.clone(),
            exception_artifact: "data/t2-beck-transfer-complexity-policy-acceptance.csv"
                .to_string(),
            next_artifact: "data/optimizer-constraint-budget.csv".to_string(),
            optimizer_effect:
                "accepted transfer-complexity policy removes T2 Beck map promotion publication blockers"
                    .to_string(),
            validation_status: "pass".to_string(),
        });
    }

    for row in t2_label_relief_rows.iter().filter(|row| {
        row.relief_decision == "relief-ready-for-constraint-ledger-replay"
            && row.blocker_count_after == 0
            && row.claim_blocker_delta < 0
    }) {
        rows.push(OptimizerConstraintLedgerRow {
            constraint_id: format!("CON-T2LABELRELIEF-{}", stable_id_fragment(&row.route)),
            optimizer_run_id: "tier-optimizer-current".to_string(),
            tier: "T2".to_string(),
            region_id: String::new(),
            constraint_order: 13,
            constraint_class: "beck_label_density_relief".to_string(),
            behavior_type: "review".to_string(),
            constraint_scope: "route".to_string(),
            subject_id: row.route.clone(),
            segment_bundle_id: String::new(),
            national_segment_id: String::new(),
            stitch_group_id: String::new(),
            route: row.route.clone(),
            stop_id: String::new(),
            pair_id: String::new(),
            map_id: "beck-schematic-t2-only".to_string(),
            source_artifact: "data/t2-beck-label-density-blocker-relief.csv".to_string(),
            source_row_id: row.relief_id.clone(),
            standard_artifact: "docs/beck-renderer-contract.md".to_string(),
            evidence_status: "accepted".to_string(),
            constraint_status: "pass".to_string(),
            observed_value: row.relief_decision.clone(),
            threshold_value: "relief-ready-for-constraint-ledger-replay".to_string(),
            measurement_unit: "relief_decision".to_string(),
            blocks_claims: String::new(),
            budget_cost_m: 0.0,
            cost_category: String::new(),
            cost_basis: String::new(),
            cost_confidence: String::new(),
            budget_units: format!("claim_blocker_delta={}", row.claim_blocker_delta),
            penalty_score: 0.0,
            repair_action: "constraint-ledger-replay-applied".to_string(),
            payment_action: String::new(),
            owner_jurisdiction: "route-program".to_string(),
            funding_program: String::new(),
            delivery_risk: "low".to_string(),
            exception_id: row.acceptance_id.clone(),
            exception_artifact: "data/t2-beck-label-density-policy-acceptance.csv".to_string(),
            next_artifact: "data/optimizer-constraint-budget.csv".to_string(),
            optimizer_effect:
                "accepted label-density policy removes T2 Beck map promotion publication blockers"
                    .to_string(),
            validation_status: "pass".to_string(),
        });
    }

    for row in t2_long_relief_rows.iter().filter(|row| {
        row.relief_decision == "relief-ready-for-constraint-ledger-replay"
            && row.blocker_count_after == 0
            && row.claim_blocker_delta < 0
    }) {
        rows.push(OptimizerConstraintLedgerRow {
            constraint_id: format!("CON-T2LONGRELIEF-{}", stable_id_fragment(&row.route)),
            optimizer_run_id: "tier-optimizer-current".to_string(),
            tier: "T2".to_string(),
            region_id: String::new(),
            constraint_order: 13,
            constraint_class: "beck_long_connector_relief".to_string(),
            behavior_type: "review".to_string(),
            constraint_scope: "route".to_string(),
            subject_id: row.route.clone(),
            segment_bundle_id: String::new(),
            national_segment_id: String::new(),
            stitch_group_id: String::new(),
            route: row.route.clone(),
            stop_id: String::new(),
            pair_id: String::new(),
            map_id: "beck-schematic-t2-only".to_string(),
            source_artifact: "data/t2-beck-long-connector-blocker-relief.csv".to_string(),
            source_row_id: row.relief_id.clone(),
            standard_artifact: "docs/beck-renderer-contract.md".to_string(),
            evidence_status: "accepted".to_string(),
            constraint_status: "pass".to_string(),
            observed_value: row.relief_decision.clone(),
            threshold_value: "relief-ready-for-constraint-ledger-replay".to_string(),
            measurement_unit: "relief_decision".to_string(),
            blocks_claims: String::new(),
            budget_cost_m: 0.0,
            cost_category: String::new(),
            cost_basis: String::new(),
            cost_confidence: String::new(),
            budget_units: format!("claim_blocker_delta={}", row.claim_blocker_delta),
            penalty_score: 0.0,
            repair_action: "constraint-ledger-replay-applied".to_string(),
            payment_action: String::new(),
            owner_jurisdiction: "route-program".to_string(),
            funding_program: String::new(),
            delivery_risk: "low".to_string(),
            exception_id: row.acceptance_id.clone(),
            exception_artifact: "data/t2-beck-long-connector-policy-acceptance.csv".to_string(),
            next_artifact: "data/optimizer-constraint-budget.csv".to_string(),
            optimizer_effect:
                "accepted long-connector policy removes T2 Beck map promotion publication blockers"
                    .to_string(),
            validation_status: "pass".to_string(),
        });
    }

    for row in scenario_hook_rows
        .iter()
        .filter(|row| !relieved_t2_game_scenarios.contains(&row.scenario_id))
    {
        rows.push(OptimizerConstraintLedgerRow {
            constraint_id: format!("CON-GAMEHOOK-{}", stable_id_fragment(&row.scenario_id)),
            optimizer_run_id: "tier-optimizer-current".to_string(),
            tier: "T2".to_string(),
            region_id: "game-campaign".to_string(),
            constraint_order: 14,
            constraint_class: "game_ops_publication_readiness".to_string(),
            behavior_type: "claim-blocker".to_string(),
            constraint_scope: "game".to_string(),
            subject_id: row.scenario_id.clone(),
            segment_bundle_id: String::new(),
            national_segment_id: String::new(),
            stitch_group_id: String::new(),
            route: String::new(),
            stop_id: String::new(),
            pair_id: String::new(),
            map_id: row.t2_map_id.clone(),
            source_artifact: "data/game/t2-scenario-hooks.csv".to_string(),
            source_row_id: row.scenario_id.clone(),
            standard_artifact: "docs/game/interstate-tycoon-plan.md".to_string(),
            evidence_status: "source-needed".to_string(),
            constraint_status: "review".to_string(),
            observed_value: row.evidence_hold.clone(),
            threshold_value: "no-evidence-hold".to_string(),
            measurement_unit: "game_evidence_hold".to_string(),
            blocks_claims: "game|upgrade|publication".to_string(),
            budget_cost_m: 0.0,
            cost_category: String::new(),
            cost_basis: String::new(),
            cost_confidence: String::new(),
            budget_units: format!("service_class={}", row.service_class),
            penalty_score: 1.0,
            repair_action: "close-evidence-hold-before-publication".to_string(),
            payment_action: String::new(),
            owner_jurisdiction: "route-program".to_string(),
            funding_program: String::new(),
            delivery_risk: "unknown".to_string(),
            exception_id: row.scenario_id.clone(),
            exception_artifact: "data/game/t2-scenario-hooks.csv".to_string(),
            next_artifact: "data/game/t2-scenario-hooks.csv".to_string(),
            optimizer_effect: row.player_decision.clone(),
            validation_status: "review".to_string(),
        });
    }

    for row in t2_game_relief_rows.iter().filter(|row| {
        row.relief_decision == "relief-ready-for-constraint-ledger-replay"
            && row.blocker_count_after == 0
            && row.claim_blocker_delta < 0
    }) {
        rows.push(OptimizerConstraintLedgerRow {
            constraint_id: format!("CON-GAMERELIEF-{}", stable_id_fragment(&row.scenario_id)),
            optimizer_run_id: "tier-optimizer-current".to_string(),
            tier: "T2".to_string(),
            region_id: "game-campaign".to_string(),
            constraint_order: 14,
            constraint_class: "game_ops_publication_readiness_relief".to_string(),
            behavior_type: "review".to_string(),
            constraint_scope: "game".to_string(),
            subject_id: row.scenario_id.clone(),
            segment_bundle_id: String::new(),
            national_segment_id: String::new(),
            stitch_group_id: String::new(),
            route: String::new(),
            stop_id: String::new(),
            pair_id: String::new(),
            map_id: "beck-schematic-t2-only".to_string(),
            source_artifact: "data/t2-game-publication-evidence-blocker-relief.csv".to_string(),
            source_row_id: row.relief_id.clone(),
            standard_artifact: "docs/game/interstate-tycoon-plan.md".to_string(),
            evidence_status: "accepted".to_string(),
            constraint_status: "pass".to_string(),
            observed_value: row.relief_decision.clone(),
            threshold_value: "relief-ready-for-constraint-ledger-replay".to_string(),
            measurement_unit: "relief_decision".to_string(),
            blocks_claims: String::new(),
            budget_cost_m: 0.0,
            cost_category: String::new(),
            cost_basis: String::new(),
            cost_confidence: String::new(),
            budget_units: format!("claim_blocker_delta={}", row.claim_blocker_delta),
            penalty_score: 0.0,
            repair_action: "constraint-ledger-replay-applied".to_string(),
            payment_action: String::new(),
            owner_jurisdiction: "route-program".to_string(),
            funding_program: String::new(),
            delivery_risk: "low".to_string(),
            exception_id: row.acceptance_id.clone(),
            exception_artifact: "data/t2-game-publication-evidence-policy-acceptance.csv"
                .to_string(),
            next_artifact: "data/optimizer-constraint-budget.csv".to_string(),
            optimizer_effect:
                "accepted game publication evidence policy removes scenario publication blockers"
                    .to_string(),
            validation_status: "pass".to_string(),
        });
    }

    for row in t2_game_ops_bundle_relief_rows.iter().filter(|row| {
        row.relief_decision == "relief-ready-for-constraint-ledger-replay"
            && row.blocker_count_after == 0
            && row.claim_blocker_delta < 0
    }) {
        rows.push(OptimizerConstraintLedgerRow {
            constraint_id: format!(
                "CON-GAMEOPSRELIEF-{}",
                stable_id_fragment(&row.segment_bundle_id)
            ),
            optimizer_run_id: "tier-optimizer-current".to_string(),
            tier: "T2".to_string(),
            region_id: "game-campaign".to_string(),
            constraint_order: 14,
            constraint_class: "game_ops_bundle_binding_relief".to_string(),
            behavior_type: "review".to_string(),
            constraint_scope: "bundle".to_string(),
            subject_id: row.segment_bundle_id.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            national_segment_id: String::new(),
            stitch_group_id: String::new(),
            route: row.route.clone(),
            stop_id: String::new(),
            pair_id: String::new(),
            map_id: "beck-schematic-t2-only".to_string(),
            source_artifact: "data/t2-game-ops-bundle-evidence-blocker-relief.csv".to_string(),
            source_row_id: row.relief_id.clone(),
            standard_artifact: "docs/game/interstate-tycoon-plan.md".to_string(),
            evidence_status: "accepted".to_string(),
            constraint_status: "pass".to_string(),
            observed_value: row.relief_decision.clone(),
            threshold_value: "relief-ready-for-constraint-ledger-replay".to_string(),
            measurement_unit: "relief_decision".to_string(),
            blocks_claims: String::new(),
            budget_cost_m: 0.0,
            cost_category: String::new(),
            cost_basis: String::new(),
            cost_confidence: String::new(),
            budget_units: format!("claim_blocker_delta={}", row.claim_blocker_delta),
            penalty_score: 0.0,
            repair_action: "constraint-ledger-replay-applied".to_string(),
            payment_action: String::new(),
            owner_jurisdiction: "route-program".to_string(),
            funding_program: String::new(),
            delivery_risk: "low".to_string(),
            exception_id: row.acceptance_id.clone(),
            exception_artifact: "data/t2-game-ops-bundle-evidence-policy-acceptance.csv"
                .to_string(),
            next_artifact: "data/optimizer-constraint-budget.csv".to_string(),
            optimizer_effect: crate::game_ops_bundle_relief_optimizer_effect(row),
            validation_status: "pass".to_string(),
        });
    }

    for row in bundle_overlay_rows.iter().filter(|row| {
        !relieved_t2_game_ops_bundles.contains(&row.segment_bundle_id)
            && (row.validation_status != "pass" || row.binding_status != "bundle-bound")
    }) {
        let repair_action = match row.binding_status.as_str() {
            "service-class-overlay-pending" => "add-service-class-overlay",
            "service-class-held-known" => "author-service-class-before-game-use",
            "bundle-bound-review" => "resolve-bundle-validation",
            "bundle-binding-pending" => "bind-route-to-segment-bundle",
            _ => "review-game-overlay-binding",
        };
        rows.push(OptimizerConstraintLedgerRow {
            constraint_id: format!(
                "CON-GAMEOVERLAY-{}",
                stable_id_fragment(&format!("{}-{}", row.route, row.binding_status))
            ),
            optimizer_run_id: "tier-optimizer-current".to_string(),
            tier: "T2".to_string(),
            region_id: row.region_id.clone(),
            constraint_order: 14,
            constraint_class: "game_ops_bundle_binding".to_string(),
            behavior_type: "claim-blocker".to_string(),
            constraint_scope: if row.segment_bundle_id.trim().is_empty() {
                "route".to_string()
            } else {
                "bundle".to_string()
            },
            subject_id: if row.segment_bundle_id.trim().is_empty() {
                row.route.clone()
            } else {
                row.segment_bundle_id.clone()
            },
            segment_bundle_id: row.segment_bundle_id.clone(),
            national_segment_id: String::new(),
            stitch_group_id: String::new(),
            route: row.route.clone(),
            stop_id: String::new(),
            pair_id: String::new(),
            map_id: row.map_id.clone(),
            source_artifact: "data/game/t2-bundle-overlays.csv".to_string(),
            source_row_id: row.route.clone(),
            standard_artifact: "docs/game/interstate-tycoon-plan.md".to_string(),
            evidence_status: "accepted".to_string(),
            constraint_status: "review".to_string(),
            observed_value: row.binding_status.clone(),
            threshold_value: "bundle-bound".to_string(),
            measurement_unit: "game_overlay_binding_status".to_string(),
            blocks_claims: "game|incident|upgrade|publication".to_string(),
            budget_cost_m: 0.0,
            cost_category: String::new(),
            cost_basis: String::new(),
            cost_confidence: String::new(),
            budget_units: format!(
                "service_class={};bundle_status={}",
                row.service_class, row.bundle_status
            ),
            penalty_score: 1.0,
            repair_action: repair_action.to_string(),
            payment_action: String::new(),
            owner_jurisdiction: "route-program".to_string(),
            funding_program: String::new(),
            delivery_risk: "unknown".to_string(),
            exception_id: row.binding_status.clone(),
            exception_artifact: "data/game/t2-bundle-overlays.csv".to_string(),
            next_artifact: row.next_artifact.clone(),
            optimizer_effect: row.pavement_debt_basis.clone(),
            validation_status: "review".to_string(),
        });
    }

    rows.sort_by(|left, right| {
        left.constraint_order
            .cmp(&right.constraint_order)
            .then_with(|| left.constraint_id.cmp(&right.constraint_id))
    });
    rows
}
