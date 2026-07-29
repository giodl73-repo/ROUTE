//! Helper `optimizer_constraint_ledger_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn optimizer_constraint_ledger_rows(
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
    beck_t1_rows: &[route_map::BeckT1DiagnosticRow],
    beck_t2_rows: &[route_map::BeckT2DiagnosticRow],
    source_policy_rows: &[SourceFetchPolicyRow],
    source_snapshot_publication_exclusion_rows: &[SourceSnapshotPublicationExclusionRow],
    scenario_hook_rows: &[T2ScenarioHookRow],
    bundle_overlay_rows: &[T2BundleOverlayRow],
) -> Vec<OptimizerConstraintLedgerRow> {
    support::optimizer::optimizer_ledger::optimizer_constraint_ledger_rows_with_terminal_proof(
        pavement_rows,
        t2_asset_condition_map_publication_exclusion_rows,
        topology_rows,
        schematic_relief_rows,
        t2_transfer_relief_rows,
        t2_label_relief_rows,
        t2_long_relief_rows,
        t2_game_relief_rows,
        t2_game_ops_bundle_relief_rows,
        t3_feeder_relief_rows,
        parallel_rows,
        access_gap_rows,
        t4_terminal_access_map_exclusion_rows,
        &[],
        &[],
        beck_t1_rows,
        beck_t2_rows,
        source_policy_rows,
        source_snapshot_publication_exclusion_rows,
        scenario_hook_rows,
        bundle_overlay_rows,
    )
}

