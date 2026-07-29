    use super::{
        atlas_candidate_ids, beck_t2_diagnostics_gate_failure, blueprint_cost_gate_failures,
        blueprint_cost_row_failure, blueprint_evidence_gate_failures,
        blueprint_evidence_row_failure, blueprint_gate_failures, blueprint_row_contract_failure,
        bridge_standard_missing_routes, bundle_architecture_gate_failures,
        bundle_architecture_rows, confidence_risk_dimensions, dimension_confidence_risks,
        dimension_confidence_values, dimension_estimated_values, dimension_score_values,
        endpoint_exception_gate_failures, endpoint_exception_is_terminal_worthy,
        filter_endpoint_exceptions, filter_stop_candidates, forum_docket_gate_failures,
        forum_docket_row_failure, gap_type_slug, join_fema_d1_to_corridor,
        known_source_fetch_commands, load_tier_routes, lower_tier_pressure_witness_gate_failures,
        lower_tier_pressure_witness_rows, map_atlas_gate_failures,
        map_publication_inventory_gate_failures, map_publication_readiness_gate_failures,
        map_publication_readiness_rows, merge_hpms_state_records,
        national_segment_bundle_gate_failures, national_segment_bundle_rows,
        national_segment_registry_gate_failures, national_segment_registry_rows,
        normalized_iri_m_per_km, optimizer_claim_review_gate_failures, optimizer_claim_review_rows,
        optimizer_constraint_budget_gate_failures, optimizer_constraint_budget_rows,
        optimizer_constraint_ledger_gate_failures, optimizer_constraint_ledger_rows,
        optimizer_manifest_gate_failures, optimizer_map_hook_gate_failures,
        optimizer_residual_blocker_backlog_gate_failures, optimizer_residual_blocker_backlog_rows,
        parse_blueprint_cost_ranges, parse_blueprint_evidence_map, parse_blueprint_packages,
        parse_endpoint_exceptions, parse_forum_docket, parse_hpms_functional_systems,
        parse_indot_trafficwise_events, parse_iowa511_events, parse_map_atlas,
        parse_mdot_midrive_events, parse_pressure_scenarios, parse_release_manifest,
        parse_significant_moments, parse_standards_inventory, parse_standards_proof_ledger,
        parse_stop_candidates, parse_t1_diamond_validation, parse_t1_evidence_windows,
        parse_t1_failure_events, parse_t1_failure_ledger, parse_t1_failure_source_plan,
        parse_t1_snapshot_plan, parse_t1_source_health, parse_tdot_smartway_events,
        parse_throughput_proof_matrix, pavement_debt_budget_index, pavement_standard_gate_failures,
        planned_standard_inventory_missing, pressure_scenario_gate_failures,
        pressure_scenario_has_bounded_contract, pressure_scenario_is_executable,
        pressure_scenario_missing_required_adversity, pressure_scenario_readiness_gate_failures,
        pressure_scenario_unknown_standard_refs, pressure_standard_coverage_failures,
        release_manifest_gate_failures, rounded_score, scenario_edge_candidates,
        significant_moment_gate_failures, significant_moment_row_failure,
        source_fetch_policy_gate_failures, source_fetch_policy_row,
        source_fetch_policy_row_covers_command, source_fetch_policy_rows,
        standards_blueprint_gate_failures, standards_evidence_level_is_allowed,
        standards_inventory_gate_failures, standards_inventory_row_has_contract,
        standards_pressure_gate_failures, stop_candidate_gate_failures, stop_coverage_for_routes,
        stop_coverage_gate_failures, stop_plan_for_route, stop_plan_gate_failures,
        summarize_t1_failure_events, t1_beck_alignment_gate_failures, t1_beck_alignment_rows,
        t1_failure_event_has_observation_contract, t1_failure_event_observation_gate_failures,
        t1_failure_evidence_gate_failures, t1_failure_row_has_evidence_contract,
        t1_feedback_docket_gate_failures, t1_feedback_docket_rows, t1_line_selector_gate_failures,
        t1_line_selector_rows, t1_schematic_geometry_blocker_relief_gate_failures,
        t1_schematic_geometry_blocker_relief_rows,
        t1_schematic_geometry_claim_review_gate_failures, t1_schematic_geometry_claim_review_rows,
        t1_shared_segment_map_policy_gate_failures, t1_shared_segment_map_policy_rows,
        t1_shared_segment_policy_acceptance_gate_failures,
        t1_shared_segment_policy_acceptance_rows, t1_sla_candidate_pair_gate_failures,
        t1_sla_candidate_pair_rows, t1_stop_selector_gate_failures, t1_stop_selector_rows,
        t1_topology_repair_gate_failures, t1_topology_repair_rows,
        t2_beck_label_density_blocker_relief_gate_failures,
        t2_beck_label_density_blocker_relief_rows,
        t2_beck_label_density_policy_acceptance_gate_failures,
        t2_beck_label_density_policy_acceptance_rows, t2_beck_label_density_policy_gate_failures,
        t2_beck_label_density_policy_rows, t2_beck_label_density_review_gate_failures,
        t2_beck_label_density_review_rows, t2_beck_long_connector_blocker_relief_gate_failures,
        t2_beck_long_connector_blocker_relief_rows,
        t2_beck_long_connector_policy_acceptance_gate_failures,
        t2_beck_long_connector_policy_acceptance_rows, t2_beck_long_connector_policy_gate_failures,
        t2_beck_long_connector_policy_rows, t2_beck_long_connector_review_gate_failures,
        t2_beck_long_connector_review_rows,
        t2_beck_transfer_complexity_blocker_relief_gate_failures,
        t2_beck_transfer_complexity_blocker_relief_rows,
        t2_beck_transfer_complexity_policy_acceptance_gate_failures,
        t2_beck_transfer_complexity_policy_acceptance_rows,
        t2_beck_transfer_complexity_policy_gate_failures, t2_beck_transfer_complexity_policy_rows,
        t2_beck_transfer_complexity_review_gate_failures, t2_beck_transfer_complexity_review_rows,
        t2_blocker_closure_gate_failures, t2_blocker_closure_rows,
        t2_bubble_up_review_gate_failures, t2_bubble_up_review_rows,
        t2_bundle_overlay_gate_failures, t2_bundle_overlay_repair_delta_gate_failures,
        t2_bundle_overlay_repair_delta_rows, t2_bundle_overlay_repair_target_gate_failures,
        t2_bundle_overlay_repair_target_rows, t2_bundle_overlay_rows,
        t2_bundle_readiness_disposition_gate_failures, t2_bundle_readiness_disposition_rows,
        t2_bundle_readiness_repair_docket_gate_failures, t2_bundle_readiness_repair_docket_rows,
        t2_bundle_readiness_repair_evidence_gate_failures,
        t2_bundle_readiness_repair_evidence_rows,
        t2_bundle_readiness_replay_decision_gate_failures,
        t2_bundle_readiness_replay_decision_rows, t2_bundle_repair_queue_gate_failures,
        t2_bundle_repair_queue_rows, t2_closure_dispositions, t2_contact_closure_gate_failures,
        t2_contact_closure_rows, t2_contact_resolution_gate_failures, t2_contact_resolution_rows,
        t2_endpoint_closure_gate_failures, t2_endpoint_closure_rows,
        t2_game_ops_binding_decision_gate_failures, t2_game_ops_binding_decision_rows,
        t2_game_ops_binding_intake_gate_failures, t2_game_ops_binding_intake_rows,
        t2_game_ops_bundle_evidence_blocker_relief_gate_failures,
        t2_game_ops_bundle_evidence_blocker_relief_rows,
        t2_game_ops_bundle_evidence_policy_acceptance_gate_failures,
        t2_game_ops_bundle_evidence_policy_acceptance_rows,
        t2_game_ops_bundle_evidence_policy_gate_failures, t2_game_ops_bundle_evidence_policy_rows,
        t2_game_ops_bundle_evidence_review_gate_failures, t2_game_ops_bundle_evidence_review_rows,
        t2_game_publication_evidence_blocker_relief_gate_failures,
        t2_game_publication_evidence_blocker_relief_rows,
        t2_game_publication_evidence_policy_acceptance_gate_failures,
        t2_game_publication_evidence_policy_acceptance_rows,
        t2_game_publication_evidence_policy_gate_failures,
        t2_game_publication_evidence_policy_rows,
        t2_game_publication_evidence_review_gate_failures,
        t2_game_publication_evidence_review_rows, t2_graph_contact_repair_gate_failures,
        t2_graph_contact_repair_rows, t2_graph_contact_validation_gate_failures,
        t2_graph_contact_validation_rows, t2_held_contact_action_gate_failures,
        t2_held_contact_action_rows, t2_local_zone_overlay_handoff_gate_failures,
        t2_local_zone_overlay_handoff_rows, t2_national_bundle_readiness_audit_gate_failures,
        t2_national_bundle_readiness_audit_rows, t2_overlay_optimizer_action_docket_gate_failures,
        t2_overlay_optimizer_action_docket_rows,
        t2_overlay_p1_structural_readiness_review_gate_failures,
        t2_overlay_p1_structural_readiness_review_rows,
        t2_overlay_p2_service_overlay_review_gate_failures,
        t2_overlay_p2_service_overlay_review_rows,
        t2_overlay_p3_local_zone_overlay_review_gate_failures,
        t2_overlay_p3_local_zone_overlay_review_rows, t2_parallel_service_queue_gate_failures,
        t2_parallel_service_queue_rows, t2_parent_contact_validation_gate_failures,
        t2_parent_contact_validation_rows, t2_regionalizer_gate_failures, t2_regionalizer_rows,
        t2_relief_evidence_gate_failures, t2_relief_evidence_rows,
        t2_route_family_split_gate_failures, t2_route_family_split_rows,
        t2_service_class_repair_docket_gate_failures, t2_service_class_repair_docket_rows,
        t2_service_diagnostic_queue_gate_failures, t2_service_diagnostic_queue_rows,
        t2_service_overlay_diagnostic_decision_gate_failures,
        t2_service_overlay_diagnostic_decision_rows, t2_service_selection_gate_failures,
        t2_service_selection_rows, t2_stitched_member_candidate_scope_review_gate_failures,
        t2_stitched_member_candidate_scope_review_rows,
        t2_stitched_member_decision_docket_gate_failures, t2_stitched_member_decision_docket_rows,
        t2_stitched_member_evidence_acquisition_gate_failures,
        t2_stitched_member_evidence_acquisition_rows,
        t2_stitched_member_evidence_contract_gate_failures,
        t2_stitched_member_evidence_contract_rows,
        t2_stitched_member_proof_artifact_attachment_gate_failures,
        t2_stitched_member_proof_artifact_attachment_rows,
        t2_stitched_member_proof_intake_gate_failures, t2_stitched_member_proof_intake_rows,
        t2_stitched_member_proof_review_docket_gate_failures,
        t2_stitched_member_proof_review_docket_rows,
        t2_stitched_member_proof_source_capture_gate_failures,
        t2_stitched_member_proof_source_capture_rows,
        t2_stitched_member_registry_handoff_gate_failures,
        t2_stitched_member_registry_handoff_rows,
        t2_stitched_member_selection_docket_gate_failures,
        t2_stitched_member_selection_docket_rows,
        t2_stitched_member_source_access_policy_gate_failures,
        t2_stitched_member_source_access_policy_rows, t2_stitched_member_split_plan_gate_failures,
        t2_stitched_member_split_plan_rows, t2_terminal_contact_validation_gate_failures,
        t2_terminal_contact_validation_rows, t3_lower_tier_feeder_gap_blocker_relief_gate_failures,
        t3_lower_tier_feeder_gap_blocker_relief_rows,
        t3_lower_tier_feeder_gap_policy_acceptance_gate_failures,
        t3_lower_tier_feeder_gap_policy_acceptance_rows,
        t3_lower_tier_feeder_gap_policy_gate_failures, t3_lower_tier_feeder_gap_policy_rows,
        t3_lower_tier_feeder_gap_review_gate_failures, t3_lower_tier_feeder_gap_review_rows,
        t3_national_segment_id, t3_segment_aliases, t3_segment_bundle_id, t3_stitch_group_id,
        t3_t4_access_gap_gate_failures, t3_t4_access_gap_rows, t3_t4_pressure_intake_gate_failures,
        t3_t4_pressure_intake_rows, t3_zone_access_obligation_gate_failures,
        t3_zone_access_obligation_rows, t3_zone_map_diagnostic_gate_failures,
        t3_zone_map_diagnostic_rows, t3_zone_render_board_gate_failures, t3_zone_render_board_rows,
        t3_zone_route_column_gate_failures, t3_zone_route_column_rows,
        t3_zone_stop_placement_gate_failures, t3_zone_stop_placement_rows,
        t4_terminal_access_column_gate_failures, t4_terminal_access_column_rows,
        t4_terminal_access_evidence_review_gate_failures, t4_terminal_access_evidence_review_rows,
        t4_terminal_access_proof_acquisition_gate_failures,
        t4_terminal_access_proof_acquisition_rows,
        t4_terminal_access_proof_artifact_acquisition_target_gate_failures,
        t4_terminal_access_proof_artifact_acquisition_target_rows,
        t4_terminal_access_proof_artifact_attachment_gate_failures,
        t4_terminal_access_proof_artifact_attachment_rows,
        t4_terminal_access_proof_artifact_gate_failures, t4_terminal_access_proof_artifact_rows,
        t4_terminal_access_proof_artifact_source_access_gate_failures,
        t4_terminal_access_proof_artifact_source_access_rows,
        t4_terminal_access_proof_attachment_review_gate_failures,
        t4_terminal_access_proof_attachment_review_rows,
        t4_terminal_access_proof_intake_gate_failures, t4_terminal_access_proof_intake_rows,
        t4_terminal_access_proof_review_gate_failures, t4_terminal_access_proof_review_rows,
        t4_terminal_access_proof_source_capture_gate_failures,
        t4_terminal_access_proof_source_capture_rows,
        t4_terminal_access_source_access_gate_failures, t4_terminal_access_source_access_rows,
        t4_terminal_columbus_proof_attempt_gate_failures, t4_terminal_columbus_proof_attempt_rows,
        t4_terminal_columbus_proof_intake_gate_failures, t4_terminal_columbus_proof_intake_rows,
        t4_terminal_columbus_source_access_gate_failures, t4_terminal_columbus_source_access_rows,
        t4_terminal_contact_district_proof_import_gate_failures,
        t4_terminal_contact_district_proof_import_rows, t4_terminal_contact_evidence_gate_failures,
        t4_terminal_contact_evidence_rows,
        t4_terminal_contact_proof_artifact_contract_gate_failures,
        t4_terminal_contact_proof_artifact_contract_rows,
        t4_terminal_contact_proof_docket_gate_failures, t4_terminal_contact_proof_docket_rows,
        t4_terminal_contact_proof_source_registry_gate_failures,
        t4_terminal_contact_proof_source_registry_rows,
        t4_terminal_contact_source_catalog_gate_failures, t4_terminal_contact_source_catalog_rows,
        t4_terminal_contact_source_plan_gate_failures, t4_terminal_contact_source_plan_rows,
        t4_terminal_scenario_readiness_gate_failures, t4_terminal_scenario_readiness_rows,
        throughput_proof_gate_failures, throughput_proof_has_bounded_contract,
        tier_candidate_column_gate_failures, tier_candidate_column_rows,
        tier_connectivity_gate_failures_with_exceptions, tier_contact_witness_gate_failures,
        tier_contact_witness_rows, tier_for_score, tier_optimizer_run_gate_failures,
        tier_pavement_acquisition_docket_gate_failures, tier_pavement_acquisition_docket_rows,
        tier_pavement_acquisition_plan_gate_failures, tier_pavement_acquisition_plan_rows,
        tier_pavement_debt_budget_gate_failures, tier_pavement_debt_budget_rows_with_exclusions,
        tier_pavement_docket_gate_failures, tier_pavement_docket_rows,
        tier_pavement_downgrade_exclusion_decision_gate_failures,
        tier_pavement_downgrade_exclusion_decision_rows,
        tier_pavement_funding_commitment_review_gate_failures,
        tier_pavement_funding_commitment_review_rows,
        tier_pavement_funding_evidence_accepted_artifact_acquisition_gate_failures,
        tier_pavement_funding_evidence_accepted_artifact_acquisition_rows,
        tier_pavement_funding_evidence_accepted_artifact_attachment_gate_failures,
        tier_pavement_funding_evidence_accepted_artifact_attachment_rows,
        tier_pavement_funding_evidence_accepted_attachment_review_gate_failures,
        tier_pavement_funding_evidence_accepted_attachment_review_rows,
        tier_pavement_funding_evidence_accepted_intake_gate_failures,
        tier_pavement_funding_evidence_accepted_intake_rows,
        tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition_gate_failures,
        tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition_rows,
        tier_pavement_funding_evidence_accepted_metadata_artifact_attachment_gate_failures,
        tier_pavement_funding_evidence_accepted_metadata_artifact_attachment_rows,
        tier_pavement_funding_evidence_accepted_metadata_attachment_review_gate_failures,
        tier_pavement_funding_evidence_accepted_metadata_attachment_review_rows,
        tier_pavement_funding_evidence_accepted_metadata_capture_gate_failures,
        tier_pavement_funding_evidence_accepted_metadata_capture_rows,
        tier_pavement_funding_evidence_accepted_metadata_intake_gate_failures,
        tier_pavement_funding_evidence_accepted_metadata_intake_rows,
        tier_pavement_funding_evidence_accepted_metadata_source_access_gate_failures,
        tier_pavement_funding_evidence_accepted_metadata_source_access_rows,
        tier_pavement_funding_evidence_accepted_metadata_source_capture_artifact_attachment_gate_failures,
        tier_pavement_funding_evidence_accepted_metadata_source_capture_artifact_attachment_rows,
        tier_pavement_funding_evidence_accepted_metadata_source_capture_gate_failures,
        tier_pavement_funding_evidence_accepted_metadata_source_capture_rows,
        tier_pavement_funding_evidence_accepted_source_access_gate_failures,
        tier_pavement_funding_evidence_accepted_source_access_rows,
        tier_pavement_funding_evidence_acquisition_gate_failures,
        tier_pavement_funding_evidence_acquisition_rows,
        tier_pavement_funding_evidence_artifact_attachment_gate_failures,
        tier_pavement_funding_evidence_artifact_attachment_rows,
        tier_pavement_funding_evidence_contract_gate_failures,
        tier_pavement_funding_evidence_contract_rows,
        tier_pavement_funding_evidence_intake_gate_failures,
        tier_pavement_funding_evidence_intake_rows,
        tier_pavement_funding_evidence_metadata_capture_gate_failures,
        tier_pavement_funding_evidence_metadata_capture_rows,
        tier_pavement_funding_evidence_review_docket_gate_failures,
        tier_pavement_funding_evidence_review_docket_rows,
        tier_pavement_funding_evidence_source_access_gate_failures,
        tier_pavement_funding_evidence_source_access_rows,
        tier_pavement_funding_evidence_source_capture_gate_failures,
        tier_pavement_funding_evidence_source_capture_rows,
        tier_pavement_hpms_scope_broadening_gate_failures,
        tier_pavement_hpms_scope_broadening_rows, tier_pavement_repair_debt_review_gate_failures,
        tier_pavement_repair_debt_review_rows, tier_pavement_repair_disposition_gate_failures,
        tier_pavement_repair_disposition_rows, tier_pavement_repair_funding_package_gate_failures,
        tier_pavement_repair_funding_package_rows, tier_pavement_route_state_scope,
        tier_pavement_source_access_gate_failures, tier_pavement_source_access_rows,
        tier_pavement_source_fetch_attempt_gate_failures, tier_pavement_source_fetch_attempt_rows,
        tier_pavement_source_fetch_review_gate_failures, tier_pavement_source_fetch_review_rows,
        tier_pavement_source_gap_gate_failures, tier_pavement_source_gap_rows,
        tier_pavement_unmatched_join_review_gate_failures,
        tier_pavement_unmatched_join_review_rows, tier_region_gate_failures,
        tier_segment_candidate_gate_failures, tier_segment_candidate_rows, write_tier_artifacts_to,
        AtriBottleneckRow, EndpointExceptionRow, FemaTile, GameT2ServiceOverlayRow, GapType,
        LowerTierPressureWitnessRow, MapAtlasRow, MapPublicationInventoryRow,
        MapPublicationReadinessRow, MapPublicationScopeDecisionRow, NationalSegmentBundleRow,
        NationalSegmentRegistryRow, NbiBridgeRecord, OptimizerClaimReviewRow,
        OptimizerConstraintBudgetIndex, OptimizerConstraintBudgetRow, OptimizerConstraintLedgerRow,
        OptimizerMapHookRow, OptimizerResidualBlockerBacklogRow, PavementDebtBudgetIndex,
        PavementStandardRow, ScoreAllRow, ScoreSignalRow, SourceFetchPolicyRow,
        SourceSnapshotPublicationExclusionRow, StopCandidateRow, T1DesignPolicyActionRow,
        T1DesignReviewCsvRow, T1LineSelectorInputRow, T1SchematicGeometryBlockerReliefRow,
        T1SchematicGeometryClaimReviewRow, T1SharedSegmentMapPolicyRow,
        T1SharedSegmentPolicyAcceptanceRow, T1SlaCandidateUniverseRow, T1SlaPairRow,
        T1StopSelectorInputRow, T1TopologyRepairRow, T2AssetConditionMapPublicationExclusionRow,
        T2BeckLabelDensityBlockerReliefRow, T2BeckLabelDensityPolicyAcceptanceRow,
        T2BeckLabelDensityPolicyRow, T2BeckLabelDensityReviewRow,
        T2BeckLongConnectorBlockerReliefRow, T2BeckLongConnectorPolicyAcceptanceRow,
        T2BeckLongConnectorPolicyRow, T2BeckLongConnectorReviewRow,
        T2BeckTransferComplexityBlockerReliefRow, T2BeckTransferComplexityPolicyAcceptanceRow,
        T2BeckTransferComplexityPolicyRow, T2BeckTransferComplexityReviewRow, T2BlockerClosureRow,
        T2BubbleUpReviewRow, T2BundleOverlayRepairDeltaRow, T2BundleOverlayRow,
        T2BundleReadinessDispositionRow, T2BundleReadinessRepairDocketRow,
        T2BundleReadinessRepairEvidenceRow, T2BundleReadinessReplayDecisionRow,
        T2BundleRepairQueueRow, T2ContactClosureRow, T2ContactResolutionRow, T2EndpointClosureRow,
        T2GameOpsBindingDecisionRow, T2GameOpsBindingIntakeRow,
        T2GameOpsBundleEvidenceBlockerReliefRow, T2GameOpsBundleEvidencePolicyAcceptanceRow,
        T2GameOpsBundleEvidencePolicyRow, T2GameOpsBundleEvidenceReviewRow,
        T2GamePublicationEvidenceBlockerReliefRow, T2GamePublicationEvidencePolicyAcceptanceRow,
        T2GamePublicationEvidencePolicyRow, T2GamePublicationEvidenceReviewRow,
        T2GraphContactRepairRow, T2GraphContactValidationRow, T2HeldContactActionRow,
        T2OverlayOptimizerActionDocketRow, T2ParallelServiceQueueRow, T2ParentContactValidationRow,
        T2RegionalizerRow, T2ReliefEvidenceRow, T2RouteFamilySplitRow, T2ScenarioHookRow,
        T2ServiceDiagnosticQueueRow, T2ServiceSelectionRow,
        T2StitchedMemberCandidateScopeReviewRow, T2StitchedMemberDecisionDocketRow,
        T2StitchedMemberEvidenceAcquisitionRow, T2StitchedMemberEvidenceContractRow,
        T2StitchedMemberProofArtifactAttachmentRow, T2StitchedMemberProofIntakeRow,
        T2StitchedMemberProofSourceCaptureRow, T2StitchedMemberRegistryHandoffRow,
        T2StitchedMemberSelectionDocketRow, T2StitchedMemberSourceAccessPolicyRow,
        T2StitchedMemberSplitPlanRow, T2TerminalContactValidationRow,
        T3LowerTierFeederGapBlockerReliefRow, T3LowerTierFeederGapPolicyAcceptanceRow,
        T3LowerTierFeederGapPolicyRow, T3LowerTierFeederGapReviewRow, T3T4AccessGapRow,
        T3T4PressureIntakeRow, T3ZoneAccessObligationRow, T3ZoneMapDiagnosticRow,
        T3ZoneRenderBoardRow, T3ZoneRouteColumnRow, T3ZoneStopPlacementRow,
        T4TerminalAccessColumnRow, T4TerminalAccessEvidenceReviewRow,
        T4TerminalAccessMapExclusionRow, T4TerminalAccessProofAcquisitionRow,
        T4TerminalAccessProofArtifactAcquisitionTargetRow,
        T4TerminalAccessProofArtifactAttachmentRow, T4TerminalAccessProofArtifactRow,
        T4TerminalAccessProofAttachmentReviewRow, T4TerminalAccessProofIntakeRow,
        T4TerminalAccessProofReviewRow, T4TerminalAccessProofSourceCaptureRow,
        T4TerminalAccessSourceAccessRow, T4TerminalColumbusProofAttemptRow,
        T4TerminalColumbusProofIntakeRow, T4TerminalColumbusSourceAccessRow,
        T4TerminalContactDistrictProofImportRow, T4TerminalContactEvidenceRow,
        T4TerminalContactProofArtifactContractRow, T4TerminalContactProofDocketRow,
        T4TerminalContactProofSourceRegistryRow, T4TerminalContactSourceCatalogRow,
        T4TerminalContactSourcePlanRow, T4TerminalScenarioReadinessRow, TierCandidateColumnRow,
        TierContactWitnessInputRow, TierOptimizerRunRow, TierPavementAcquisitionDocketRow,
        TierPavementAcquisitionPlanRow, TierPavementDebtBudgetRow, TierPavementDocketRow,
        TierPavementDowngradeExclusionDecisionRow, TierPavementFundingCommitmentReviewRow,
        TierPavementFundingEvidenceAcceptedArtifactAcquisitionRow,
        TierPavementFundingEvidenceAcceptedArtifactAttachmentRow,
        TierPavementFundingEvidenceAcceptedAttachmentReviewRow,
        TierPavementFundingEvidenceAcceptedIntakeRow,
        TierPavementFundingEvidenceAcceptedMetadataArtifactAcquisitionRow,
        TierPavementFundingEvidenceAcceptedMetadataArtifactAttachmentRow,
        TierPavementFundingEvidenceAcceptedMetadataAttachmentReviewRow,
        TierPavementFundingEvidenceAcceptedMetadataCaptureRow,
        TierPavementFundingEvidenceAcceptedMetadataIntakeRow,
        TierPavementFundingEvidenceAcceptedMetadataSourceAccessRow,
        TierPavementFundingEvidenceAcceptedMetadataSourceCaptureRow,
        TierPavementFundingEvidenceAcceptedSourceAccessRow,
        TierPavementFundingEvidenceAcquisitionRow,
        TierPavementFundingEvidenceArtifactAttachmentRow, TierPavementFundingEvidenceContractRow,
        TierPavementFundingEvidenceIntakeRow, TierPavementFundingEvidenceMetadataCaptureRow,
        TierPavementFundingEvidenceReviewDocketRow, TierPavementFundingEvidenceSourceAccessRow,
        TierPavementFundingEvidenceSourceCaptureRow, TierPavementHpmsScopeBroadeningRow,
        TierPavementRepairDebtReviewRow, TierPavementRepairDispositionRow,
        TierPavementRepairFundingAcceptanceRow, TierPavementRepairFundingPackageRow,
        TierPavementRouteStateExclusionRow, TierPavementSourceAccessRow,
        TierPavementSourceFetchAttemptRow, TierPavementSourceFetchReviewRow,
        TierPavementSourceGapRow, TierPavementUnmatchedJoinReviewRow, TierRegionRepairInputRow,
        TierRegionWorkloadRow, TierSegmentCandidateRow, TierTableScoreRow,
    };
    use geo_types::{coord, LineString};
    use route_network::{CorridorAttributes, HighwayEdge, HighwayGraph, HighwayNode};
    use route_score::{score_corridor, ScoringConfig};
    use std::collections::HashMap;

    fn write_optimizer_manifest_fixture(name: &str, records: usize) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "route-optimizer-manifest-{}-{}",
            name,
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create optimizer manifest fixture dir");
        let path = dir.join("artifact.csv");
        let mut csv = String::from("id,value\n");
        for index in 0..records {
            csv.push_str(&format!("{index},fixture\n"));
        }
        std::fs::write(&path, csv).expect("write optimizer manifest fixture");
        path
    }

    #[test]
    fn reviewed_report_refuses_incomplete_source_overwrite() {
        let root = std::env::temp_dir().join(format!(
            "route-reviewed-report-sources-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        let cache_dir = root.join("data").join("cache");
        let output_path = root.join("corpus").join("existing").join("i80.md");
        std::fs::create_dir_all(&cache_dir).expect("create cache directory");
        std::fs::create_dir_all(output_path.parent().expect("output parent"))
            .expect("create corpus directory");
        std::fs::write(&output_path, "---\nstatus: reviewed\n---\n")
            .expect("write reviewed corpus entry");

        let error = super::ensure_reviewed_report_sources(&output_path, &cache_dir, &root, false)
            .expect_err("incomplete reviewed report should fail");
        assert!(error.to_string().contains("refusing to overwrite"));
        assert!(error.to_string().contains("hpms_2018.csv"));

        super::ensure_reviewed_report_sources(&output_path, &cache_dir, &root, true)
            .expect("explicit partial report should be allowed");
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn draft_report_allows_incomplete_sources() {
        let root =
            std::env::temp_dir().join(format!("route-draft-report-sources-{}", std::process::id()));
        let cache_dir = root.join("data").join("cache");
        let output_path = root.join("corpus").join("existing").join("i80.md");
        std::fs::create_dir_all(&cache_dir).expect("create cache directory");
        std::fs::create_dir_all(output_path.parent().expect("output parent"))
            .expect("create corpus directory");
        std::fs::write(&output_path, "---\nstatus: draft\n---\n")
            .expect("write draft corpus entry");

        super::ensure_reviewed_report_sources(&output_path, &cache_dir, &root, false)
            .expect("draft report should allow partial sources");
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn census_key_validation_rejects_missing_and_empty_values() {
        assert!(super::validate_census_api_key(None).is_err());
        assert!(super::validate_census_api_key(Some("  ".to_string())).is_err());
        assert_eq!(
            super::validate_census_api_key(Some("secret".to_string())).unwrap(),
            "secret"
        );
    }

    #[test]
    fn i80_source_contract_exclusions_are_loaded() {
        let dir =
            std::env::temp_dir().join(format!("route-i80-source-policy-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create policy fixture");
        let path = dir.join("contract.csv");
        std::fs::write(
            &path,
            "source_id,acquisition_status\nSRC-I80-NBI,adapter-deferred-excluded\nSRC-I80-HPMS,automated-partial\n",
        )
        .expect("write policy fixture");

        let excluded = super::load_excluded_i80_sources(&path).expect("load exclusions");
        assert!(excluded.contains("SRC-I80-NBI"));
        assert!(!excluded.contains("SRC-I80-HPMS"));
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn tier_for_score_matches_megamap_thresholds() {
        assert_eq!(tier_for_score(70.0), "T1");
        assert_eq!(tier_for_score(69.9), "T2");
        assert_eq!(tier_for_score(50.0), "T2");
        assert_eq!(tier_for_score(49.9), "T3");
        assert_eq!(tier_for_score(30.0), "T3");
        assert_eq!(tier_for_score(29.9), "T4");
    }

    #[test]
    fn rounded_score_matches_score_all_csv_precision() {
        assert_eq!(rounded_score(59.95), 60.0);
        assert_eq!(rounded_score(59.94), 59.9);
    }

    #[test]
    fn gap_type_slugs_match_cli_values() {
        assert_eq!(gap_type_slug(&GapType::MissingLink), "missing-link");
        assert_eq!(gap_type_slug(&GapType::Bottleneck), "bottleneck");
        assert_eq!(gap_type_slug(&GapType::Resilience), "resilience");
        assert_eq!(gap_type_slug(&GapType::Intermodal), "intermodal");
    }

    #[test]
    fn tier_region_gate_fails_component_bridging() {
        let rows = vec![
            TierRegionWorkloadRow {
                tier: "T2".to_string(),
                graph_kind: "dual-route-graph".to_string(),
                split_objective: "route-mile-workload".to_string(),
                requested_regions: 2,
                region_id: 0,
                route: "I10".to_string(),
                node_class: "trunk_connector".to_string(),
                route_weight: 100,
                route_miles: 100.0,
                t1_node_count: 2,
                parent_trunk_count: 2,
                parent_trunks: "I5;I95".to_string(),
                contact_route_count: 1,
                component_id: 0,
                component_route_count: 1,
                component_status: "component-bridged:2".to_string(),
                repair_action: "keep-for-regionalizer".to_string(),
                repair_basis: "touches-multiple-t1-trunks".to_string(),
                validation_status: "review".to_string(),
            },
            TierRegionWorkloadRow {
                tier: "T2".to_string(),
                graph_kind: "dual-route-graph".to_string(),
                split_objective: "route-mile-workload".to_string(),
                requested_regions: 2,
                region_id: 1,
                route: "I20".to_string(),
                node_class: "trunk_connector".to_string(),
                route_weight: 100,
                route_miles: 100.0,
                t1_node_count: 2,
                parent_trunk_count: 2,
                parent_trunks: "I10;I95".to_string(),
                contact_route_count: 1,
                component_id: 1,
                component_route_count: 1,
                component_status: "component-bridged:2".to_string(),
                repair_action: "keep-for-regionalizer".to_string(),
                repair_basis: "touches-multiple-t1-trunks".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let failures = tier_region_gate_failures(&rows, 2);

        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("component-bridged:2"));
    }

    #[test]
    fn tier_contact_witness_gate_tracks_unresolved_repairs() {
        let rows = vec![
            TierRegionRepairInputRow {
                tier: "T2".to_string(),
                route: "I15".to_string(),
                node_class: "trunk_connector".to_string(),
                route_miles: 2882.0,
                t1_node_count: 6,
                parent_trunks: "I84;I90".to_string(),
                contact_route_count: 3,
                component_id: 1,
                component_route_count: 18,
                component_status: "component-bridged:2".to_string(),
                repair_action: "keep-for-regionalizer".to_string(),
                repair_basis: "touches-multiple-t1-trunks".to_string(),
                next_artifact: "data/tier-candidate-columns.csv".to_string(),
            },
            TierRegionRepairInputRow {
                tier: "T2".to_string(),
                route: "I110".to_string(),
                node_class: "missing_graph_data".to_string(),
                route_miles: 79.0,
                t1_node_count: 0,
                parent_trunks: String::new(),
                contact_route_count: 0,
                component_id: 0,
                component_route_count: 1,
                component_status: "component-bridged:2".to_string(),
                repair_action: "fix-graph-contact-or-demote".to_string(),
                repair_basis: "missing-t1-contact-evidence".to_string(),
                next_artifact: "data/tier-contact-witnesses.csv".to_string(),
            },
            TierRegionRepairInputRow {
                tier: "T2".to_string(),
                route: "I220".to_string(),
                node_class: "local_spur".to_string(),
                route_miles: 59.0,
                t1_node_count: 0,
                parent_trunks: String::new(),
                contact_route_count: 0,
                component_id: 5,
                component_route_count: 1,
                component_status: "component-bridged:2".to_string(),
                repair_action: "demote-to-t3-t4".to_string(),
                repair_basis: "local-spur".to_string(),
                next_artifact: "data/tier-table.csv".to_string(),
            },
        ];

        let witnesses = tier_contact_witness_rows(&rows, &[]);
        let failures = tier_contact_witness_gate_failures(&witnesses);

        assert_eq!(witnesses[0].validation_status, "pass");
        assert_eq!(witnesses[1].witness_type, "graph-contact-needed");
        assert_eq!(witnesses[2].witness_type, "tier-demotion-needed");
        assert_eq!(
            failures,
            vec![
                "I110 requires graph-contact-needed via data/tier-contact-witnesses.csv"
                    .to_string()
            ]
        );
    }

    #[test]
    fn tier_contact_witnesses_accept_clean_beck_t2_contacts() {
        let rows = vec![TierRegionRepairInputRow {
            tier: "T2".to_string(),
            route: "I22".to_string(),
            node_class: "missing_graph_data".to_string(),
            route_miles: 403.6,
            t1_node_count: 0,
            parent_trunks: String::new(),
            contact_route_count: 0,
            component_id: 1,
            component_route_count: 1,
            component_status: "component-bridged:2".to_string(),
            repair_action: "fix-graph-contact-or-demote".to_string(),
            repair_basis: "missing-t1-contact-evidence".to_string(),
            next_artifact: "data/tier-contact-witnesses.csv".to_string(),
        }];
        let diagnostics = vec![route_map::BeckT2DiagnosticRow {
            corridor: "I-22",
            trunk: "I-40",
            start_trunk: "I-40",
            end_trunk: "I-20",
            color_mode: "split-parent",
            service_class: "compact-service",
            split_anchor: "TUPELO",
            split_anchor_offset_pct: 13.0,
            unstopped_t1_contact_count: 0,
            unstopped_t1_contacts: String::new(),
            close_parallel_count: 0,
            close_parallel_corridors: String::new(),
            duplicate_service_count: 0,
            duplicate_service_corridors: String::new(),
            unique_duplicate_stop_count: 3,
            service_action: "keep",
            qualification_basis: "distinct-parent-service",
            service_label: "Memphis-Birmingham",
            stop_count: 3,
            drawn_stop_count: 3,
            transfer_stop_count: 2,
            schematic_length_px: 227.0,
            min_x: 1371.0,
            min_y: 860.0,
            max_x: 1529.0,
            max_y: 996.0,
            label_density_per_100px: 1.32,
            review_flag: "ok",
        }];

        let witnesses = tier_contact_witness_rows(&rows, &diagnostics);
        let failures = tier_contact_witness_gate_failures(&witnesses);

        assert!(failures.is_empty());
        assert_eq!(witnesses[0].witness_type, "regionalizer-ready");
        assert_eq!(witnesses[0].node_class, "trunk_connector");
        assert_eq!(witnesses[0].observed_t1_node_count, 2);
        assert_eq!(witnesses[0].observed_parent_trunks, "I20;I40");
        assert_eq!(witnesses[0].evidence_status, "beck-contact-observed");
    }

    #[test]
    fn t2_contact_resolutions_move_policy_rows_downstream() {
        let rows = vec![
            TierContactWitnessInputRow {
                tier: "T2".to_string(),
                route: "I220".to_string(),
                witness_type: "tier-demotion-needed".to_string(),
                node_class: "local_spur".to_string(),
                route_miles: 59.0,
                observed_t1_node_count: 0,
                observed_parent_trunks: String::new(),
                observed_dual_contacts: 0,
                component_id: 5,
                component_route_count: 1,
                component_status: "component-bridged:21".to_string(),
                repair_action: "demote-to-t3-t4".to_string(),
                repair_basis: "local-spur".to_string(),
                evidence_status: "policy-action".to_string(),
                required_artifact: "data/tier-table.csv".to_string(),
                validation_status: "review".to_string(),
            },
            TierContactWitnessInputRow {
                tier: "T2".to_string(),
                route: "I24".to_string(),
                witness_type: "parent-contact-needed".to_string(),
                node_class: "relief_loop".to_string(),
                route_miles: 635.0,
                observed_t1_node_count: 3,
                observed_parent_trunks: "I69".to_string(),
                observed_dual_contacts: 0,
                component_id: 7,
                component_route_count: 1,
                component_status: "component-bridged:21".to_string(),
                repair_action: "add-parent-contact-or-demote".to_string(),
                repair_basis: "relief-loop-has-no-dual-route-contact".to_string(),
                evidence_status: "source-needed".to_string(),
                required_artifact: "data/tier-contact-witnesses.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let resolutions = t2_contact_resolution_rows(&rows, &[]);
        let failures = t2_contact_resolution_gate_failures(&resolutions);

        assert_eq!(
            resolutions[0].resolution_action,
            "move-to-lower-tier-pressure"
        );
        assert!(failures.is_empty());
        assert_eq!(resolutions[1].validation_status, "review");
    }

    #[test]
    fn t2_contact_resolutions_use_endpoint_exceptions() {
        let rows = vec![
            TierContactWitnessInputRow {
                tier: "T2".to_string(),
                route: "I110".to_string(),
                witness_type: "graph-contact-needed".to_string(),
                node_class: "missing_graph_data".to_string(),
                route_miles: 79.0,
                observed_t1_node_count: 0,
                observed_parent_trunks: String::new(),
                observed_dual_contacts: 0,
                component_id: 0,
                component_route_count: 1,
                component_status: "component-bridged:21".to_string(),
                repair_action: "fix-graph-contact-or-demote".to_string(),
                repair_basis: "missing-t1-contact-evidence".to_string(),
                evidence_status: "source-needed".to_string(),
                required_artifact: "data/tier-contact-witnesses.csv".to_string(),
                validation_status: "review".to_string(),
            },
            TierContactWitnessInputRow {
                tier: "T2".to_string(),
                route: "I65".to_string(),
                witness_type: "graph-contact-needed".to_string(),
                node_class: "missing_graph_data".to_string(),
                route_miles: 1777.0,
                observed_t1_node_count: 0,
                observed_parent_trunks: String::new(),
                observed_dual_contacts: 0,
                component_id: 17,
                component_route_count: 1,
                component_status: "component-bridged:21".to_string(),
                repair_action: "fix-graph-contact-or-demote".to_string(),
                repair_basis: "missing-t1-contact-evidence".to_string(),
                evidence_status: "source-needed".to_string(),
                required_artifact: "data/tier-contact-witnesses.csv".to_string(),
                validation_status: "review".to_string(),
            },
            TierContactWitnessInputRow {
                tier: "T2".to_string(),
                route: "I285".to_string(),
                witness_type: "terminal-exception-needed".to_string(),
                node_class: "one_ended_feeder".to_string(),
                route_miles: 172.2,
                observed_t1_node_count: 1,
                observed_parent_trunks: "I40".to_string(),
                observed_dual_contacts: 9,
                component_id: 0,
                component_route_count: 39,
                component_status: "component-bridged:2".to_string(),
                repair_action: "terminal-exception-or-demote".to_string(),
                repair_basis: "one-ended-feeder".to_string(),
                evidence_status: "source-needed".to_string(),
                required_artifact: "data/tier-node-exceptions.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let exceptions = vec![
            EndpointExceptionRow {
                route: "I110".to_string(),
                requested_tier: "T2".to_string(),
                endpoint_name: "Port approach".to_string(),
                endpoint_role: "local_access_end".to_string(),
                exception_type: "demote".to_string(),
                evidence_level: "heuristic".to_string(),
                artifact: "data/tier-table.csv".to_string(),
                next_step: "demote".to_string(),
            },
            EndpointExceptionRow {
                route: "I65".to_string(),
                requested_tier: "T2".to_string(),
                endpoint_name: "Mobile".to_string(),
                endpoint_role: "t2_terminal_exception".to_string(),
                exception_type: "port_terminal".to_string(),
                evidence_level: "heuristic".to_string(),
                artifact: "data/corridor-designations.csv".to_string(),
                next_step: "validate contact".to_string(),
            },
            EndpointExceptionRow {
                route: "I285".to_string(),
                requested_tier: "T2".to_string(),
                endpoint_name: "Atlanta loop".to_string(),
                endpoint_role: "local_access_end".to_string(),
                exception_type: "metro_beltway_relief".to_string(),
                evidence_level: "heuristic".to_string(),
                artifact: "data/atri-bottlenecks.csv".to_string(),
                next_step: "validate relief".to_string(),
            },
        ];

        let resolutions = t2_contact_resolution_rows(&rows, &exceptions);

        assert_eq!(
            resolutions[0].resolution_action,
            "move-to-lower-tier-pressure"
        );
        assert_eq!(
            resolutions[1].resolution_action,
            "hold-for-terminal-contact-validation"
        );
        assert_eq!(
            resolutions[2].resolution_action,
            "hold-for-relief-evidence-or-demotion"
        );
        assert_eq!(resolutions[2].next_artifact, "data/atri-bottlenecks.csv");
    }

    #[test]
    fn t2_held_contact_actions_emit_next_artifacts() {
        let rows = vec![
            T2ContactResolutionRow {
                route: "I65".to_string(),
                witness_type: "graph-contact-needed".to_string(),
                node_class: "missing_graph_data".to_string(),
                repair_action: "fix-graph-contact-or-demote".to_string(),
                required_artifact: "data/tier-contact-witnesses.csv".to_string(),
                exception_type: "port_terminal".to_string(),
                exception_evidence_level: "heuristic".to_string(),
                resolution_action: "hold-for-terminal-contact-validation".to_string(),
                resolution_basis: "terminal-worthy-exception-needs-graph-contact".to_string(),
                next_artifact: "data/tier-contact-witnesses.csv".to_string(),
                validation_status: "review".to_string(),
            },
            T2ContactResolutionRow {
                route: "I285".to_string(),
                witness_type: "graph-contact-needed".to_string(),
                node_class: "missing_graph_data".to_string(),
                repair_action: "fix-graph-contact-or-demote".to_string(),
                required_artifact: "data/tier-contact-witnesses.csv".to_string(),
                exception_type: "metro_beltway_relief".to_string(),
                exception_evidence_level: "heuristic".to_string(),
                resolution_action: "hold-for-relief-evidence-or-demotion".to_string(),
                resolution_basis: "metro-beltway-relief-needs-source-backed-contact".to_string(),
                next_artifact: "data/atri-bottlenecks.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let actions = t2_held_contact_action_rows(&rows);
        let failures = t2_held_contact_action_gate_failures(&actions);

        assert_eq!(actions[0].held_action_type, "terminal-contact-validation");
        assert_eq!(actions[1].held_action_type, "relief-evidence-review");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_graph_contact_repairs_split_family_and_contact_repairs() {
        let rows = vec![
            T2HeldContactActionRow {
                route: "I195".to_string(),
                held_action_type: "graph-contact-repair".to_string(),
                source_resolution_action: "hold-for-graph-contact-repair".to_string(),
                exception_type: "missing_graph_geometry".to_string(),
                required_evidence: "repair route geometry".to_string(),
                next_artifact: "data/tier-contact-witnesses.csv".to_string(),
                optimizer_effect: "blocked".to_string(),
                validation_status: "review".to_string(),
            },
            T2HeldContactActionRow {
                route: "I44".to_string(),
                held_action_type: "graph-contact-repair".to_string(),
                source_resolution_action: "hold-for-graph-contact-repair".to_string(),
                exception_type: String::new(),
                required_evidence: "repair route geometry".to_string(),
                next_artifact: "data/tier-contact-witnesses.csv".to_string(),
                optimizer_effect: "blocked".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let repairs = t2_graph_contact_repair_rows(&rows);
        let failures = t2_graph_contact_repair_gate_failures(&repairs);

        assert_eq!(repairs[0].repair_class, "route-family-split");
        assert_eq!(repairs[1].repair_class, "graph-contact-repair");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_parent_contact_validation_requires_dual_contact_or_demote() {
        let held = vec![T2HeldContactActionRow {
            route: "I24".to_string(),
            held_action_type: "parent-contact-validation".to_string(),
            source_resolution_action: "hold-for-parent-contact-or-demotion".to_string(),
            exception_type: String::new(),
            required_evidence: "prove relief loop dual-route contact".to_string(),
            next_artifact: "data/t2-parent-contact-validation.csv".to_string(),
            optimizer_effect: "retain with parent contact".to_string(),
            validation_status: "review".to_string(),
        }];
        let witnesses = vec![TierContactWitnessInputRow {
            tier: "T2".to_string(),
            route: "I24".to_string(),
            witness_type: "parent-contact-needed".to_string(),
            node_class: "relief_loop".to_string(),
            route_miles: 635.0,
            observed_t1_node_count: 3,
            observed_parent_trunks: "I69".to_string(),
            observed_dual_contacts: 0,
            component_id: 7,
            component_route_count: 1,
            component_status: "component-bridged:21".to_string(),
            repair_action: "add-parent-contact-or-demote".to_string(),
            repair_basis: "relief-loop-has-no-dual-route-contact".to_string(),
            evidence_status: "source-needed".to_string(),
            required_artifact: "data/tier-contact-witnesses.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_parent_contact_validation_rows(&held, &witnesses);
        let failures = t2_parent_contact_validation_gate_failures(&rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].validation_action, "prove-parent-contact-or-demote");
        assert_eq!(rows[0].parent_trunks, "I69");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_parent_contact_validation_emits_clear_row_when_no_parent_blockers_remain() {
        let rows = t2_parent_contact_validation_rows(&[], &[]);
        let failures = t2_parent_contact_validation_gate_failures(&rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].route, "__all_t2_parent_contacts__");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_relief_evidence_docket_uses_atri_route_matches() {
        let held = vec![
            T2HeldContactActionRow {
                route: "I285".to_string(),
                held_action_type: "relief-evidence-review".to_string(),
                source_resolution_action: "hold-for-relief-evidence-or-demotion".to_string(),
                exception_type: "relief_loop".to_string(),
                required_evidence: "source-backed relief evidence".to_string(),
                next_artifact: "data/t2-relief-evidence-docket.csv".to_string(),
                optimizer_effect: "retain with evidence".to_string(),
                validation_status: "review".to_string(),
            },
            T2HeldContactActionRow {
                route: "I405".to_string(),
                held_action_type: "relief-evidence-review".to_string(),
                source_resolution_action: "hold-for-relief-evidence-or-demotion".to_string(),
                exception_type: "relief_loop".to_string(),
                required_evidence: "source-backed relief evidence".to_string(),
                next_artifact: "data/t2-relief-evidence-docket.csv".to_string(),
                optimizer_effect: "retain with evidence".to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let bottlenecks = vec![AtriBottleneckRow {
            rank: 1,
            location: "I-285/I-20 interchange Atlanta".to_string(),
            route: "I285".to_string(),
            state: "GA".to_string(),
            annual_cost_m: 916.0,
            lat: 33.748,
            lon: -84.462,
        }];

        let rows = t2_relief_evidence_rows(&held, &bottlenecks);
        let failures = t2_relief_evidence_gate_failures(&rows);

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].relief_action, "source-observed-relief-review");
        assert_eq!(rows[0].bottleneck_match_count, 1);
        assert_eq!(rows[1].relief_action, "source-gap-demote-or-find-evidence");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_relief_evidence_docket_emits_clear_row_when_no_relief_blockers_remain() {
        let rows = t2_relief_evidence_rows(&[], &[]);
        let failures = t2_relief_evidence_gate_failures(&rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].route, "__all_t2_relief_evidence__");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_terminal_contact_validation_separates_endpoint_and_contact_proof() {
        let held = vec![
            T2HeldContactActionRow {
                route: "I65".to_string(),
                held_action_type: "terminal-contact-validation".to_string(),
                source_resolution_action: "hold-for-terminal-contact-validation".to_string(),
                exception_type: "port_terminal".to_string(),
                required_evidence: "terminal endpoint plus graph contact".to_string(),
                next_artifact: "data/t2-terminal-contact-validation.csv".to_string(),
                optimizer_effect: "retain with terminal contact".to_string(),
                validation_status: "review".to_string(),
            },
            T2HeldContactActionRow {
                route: "I270".to_string(),
                held_action_type: "terminal-exception-review".to_string(),
                source_resolution_action: "hold-for-terminal-exception".to_string(),
                exception_type: "metro_beltway_relief".to_string(),
                required_evidence: "terminal-worthy endpoint exception".to_string(),
                next_artifact: "data/t2-terminal-contact-validation.csv".to_string(),
                optimizer_effect: "retain only validated terminal segment".to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let exceptions = vec![
            EndpointExceptionRow {
                route: "I65".to_string(),
                requested_tier: "T2".to_string(),
                endpoint_name: "Mobile".to_string(),
                endpoint_role: "t2_terminal_exception".to_string(),
                exception_type: "port_terminal".to_string(),
                evidence_level: "heuristic".to_string(),
                artifact: "data/ports.csv".to_string(),
                next_step: "validate terminal".to_string(),
            },
            EndpointExceptionRow {
                route: "I270".to_string(),
                requested_tier: "T2".to_string(),
                endpoint_name: "St. Louis beltway".to_string(),
                endpoint_role: "one_ended_feeder".to_string(),
                exception_type: "metro_beltway_relief".to_string(),
                evidence_level: "heuristic".to_string(),
                artifact: "data/atri-bottlenecks.csv".to_string(),
                next_step: "validate endpoint".to_string(),
            },
        ];
        let witnesses = vec![
            TierContactWitnessInputRow {
                tier: "T2".to_string(),
                route: "I65".to_string(),
                witness_type: "graph-contact-needed".to_string(),
                node_class: "missing_graph_data".to_string(),
                route_miles: 1776.0,
                observed_t1_node_count: 0,
                observed_parent_trunks: String::new(),
                observed_dual_contacts: 0,
                component_id: 17,
                component_route_count: 1,
                component_status: "component-bridged:21".to_string(),
                repair_action: "fix-graph-contact-or-demote".to_string(),
                repair_basis: "missing-t1-contact-evidence".to_string(),
                evidence_status: "source-needed".to_string(),
                required_artifact: "data/tier-contact-witnesses.csv".to_string(),
                validation_status: "review".to_string(),
            },
            TierContactWitnessInputRow {
                tier: "T2".to_string(),
                route: "I270".to_string(),
                witness_type: "terminal-exception-needed".to_string(),
                node_class: "one_ended_feeder".to_string(),
                route_miles: 342.0,
                observed_t1_node_count: 1,
                observed_parent_trunks: "I70".to_string(),
                observed_dual_contacts: 3,
                component_id: 1,
                component_route_count: 18,
                component_status: "component-bridged:21".to_string(),
                repair_action: "terminal-exception-or-demote".to_string(),
                repair_basis: "one-ended-feeder-needs-terminal-worthy-endpoint".to_string(),
                evidence_status: "source-needed".to_string(),
                required_artifact: "data/tier-node-exceptions.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t2_terminal_contact_validation_rows(&held, &exceptions, &witnesses);
        let failures = t2_terminal_contact_validation_gate_failures(&rows);

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].terminal_action, "prove-terminal-contact-or-demote");
        assert!(rows[0].terminal_worthy);
        assert_eq!(
            rows[1].terminal_action,
            "prove-terminal-exception-or-demote"
        );
        assert!(!rows[1].terminal_worthy);
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_blocker_closure_normalizes_all_held_surfaces() {
        let graph_rows = vec![T2GraphContactRepairRow {
            route: "I195".to_string(),
            repair_class: "route-family-split".to_string(),
            source_exception_type: "missing_graph_geometry".to_string(),
            repair_action: "split-numbered-route-family-before-tier-decision".to_string(),
            required_evidence: "identify represented segment".to_string(),
            next_artifact: "data/tier-node-exceptions.csv".to_string(),
            optimizer_effect: "blocked until route family is disambiguated".to_string(),
            validation_status: "review".to_string(),
        }];
        let parent_rows = vec![T2ParentContactValidationRow {
            route: "I24".to_string(),
            parent_trunks: "I69".to_string(),
            observed_dual_contacts: 0,
            validation_action: "prove-parent-contact-or-demote".to_string(),
            required_evidence: "dual-route contact to named parent trunk".to_string(),
            next_artifact: "data/tier-contact-witnesses.csv".to_string(),
            optimizer_effect: "blocked from T2 regionalizer until parent contact exists"
                .to_string(),
            validation_status: "review".to_string(),
        }];
        let relief_rows = vec![T2ReliefEvidenceRow {
            route: "I285".to_string(),
            source_exception_type: "metro_beltway_relief".to_string(),
            bottleneck_match_count: 2,
            top_bottleneck_rank: 1,
            top_bottleneck_location: "Atlanta".to_string(),
            annual_cost_m: 1705.0,
            relief_action: "source-observed-relief-review".to_string(),
            evidence_basis: "atri-bottleneck-route-match".to_string(),
            next_artifact: "data/tier-contact-witnesses.csv".to_string(),
            optimizer_effect: "retain relief review only after contact repair validates"
                .to_string(),
            validation_status: "review".to_string(),
        }];
        let terminal_rows = vec![T2TerminalContactValidationRow {
            route: "I270".to_string(),
            held_action_type: "terminal-exception-review".to_string(),
            endpoint_name: "St. Louis beltway".to_string(),
            endpoint_role: "one_ended_feeder".to_string(),
            exception_type: "metro_beltway_relief".to_string(),
            terminal_worthy: false,
            observed_t1_node_count: 1,
            observed_dual_contacts: 3,
            terminal_action: "prove-terminal-exception-or-demote".to_string(),
            required_evidence: "terminal-worthy endpoint exception under T2 endpoint standard"
                .to_string(),
            next_artifact: "data/tier-node-exceptions.csv".to_string(),
            optimizer_effect:
                "blocked from T2 unless endpoint exception is upgraded or route demotes".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows =
            crate::support::tier::t2_blocker_closure_rows::t2_blocker_closure_rows(&graph_rows, &parent_rows, &relief_rows, &terminal_rows, &[]);
        let failures = t2_blocker_closure_gate_failures(&rows);
        let classes = rows
            .iter()
            .map(|row| row.blocker_class.as_str())
            .collect::<std::collections::BTreeSet<_>>();

        assert!(classes.contains("route-family-split"));
        assert!(classes.contains("parent-contact-repair"));
        assert!(classes.contains("relief-contact-repair"));
        assert!(classes.contains("endpoint-exception-upgrade"));
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_blocker_closure_joins_bundle_registry() {
        let graph_rows = vec![T2GraphContactRepairRow {
            route: "I30".to_string(),
            repair_class: "graph-contact-repair".to_string(),
            source_exception_type: String::new(),
            repair_action: "repair-route-geometry-or-demote".to_string(),
            required_evidence: "prove contact".to_string(),
            next_artifact: "data/tier-contact-witnesses.csv".to_string(),
            optimizer_effect: "blocked until contact exists".to_string(),
            validation_status: "review".to_string(),
        }];
        let bundles = vec![NationalSegmentBundleRow {
            segment_bundle_id: "US.HWYBUNDLE.I30".to_string(),
            bundle_role: "single-segment".to_string(),
            member_segment_ids: "US.HWYSEG.I30".to_string(),
            member_count: 1,
            stitch_group_ids: "US.HWYSTITCH.I30".to_string(),
            current_tiers: "T2".to_string(),
            current_zone_ids: "component-1".to_string(),
            route_labels: "I30".to_string(),
            state_scope: "TX;AR".to_string(),
            evidence_state_scope: "TX;AR".to_string(),
            geometry_state_scope: String::new(),
            bundle_aliases: "route:I30".to_string(),
            source_artifacts: "fixture".to_string(),
            bundle_status: "bundle-ready".to_string(),
            bundle_action: "use bundle as service join surface".to_string(),
            qualification_effects: "qualification_gate_policy=stop-first".to_string(),
            next_artifact: "maps/t3-zone".to_string(),
            validation_status: "pass".to_string(),
        }];

        let rows = crate::support::tier::t2_blocker_closure_rows::t2_blocker_closure_rows(&graph_rows, &[], &[], &[], &bundles);
        let failures = t2_blocker_closure_gate_failures(&rows);

        assert_eq!(rows[0].segment_bundle_id, "US.HWYBUNDLE.I30");
        assert_eq!(rows[0].bundle_status, "bundle-ready");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_gate_policy=stop-first"
        );
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_route_family_splits_use_exception_disposition() {
        let closure_rows = vec![
            T2BlockerClosureRow {
                route: "I195".to_string(),
                segment_bundle_id: String::new(),
                bundle_status: "bundle-missing".to_string(),
                bundle_action: "resolve route family or add segment bundle".to_string(),
                source_surface: "t2-graph-contact-repairs".to_string(),
                blocker_class: "route-family-split".to_string(),
                blocker_action: "split-numbered-route-family-before-tier-decision".to_string(),
                required_evidence: "identify represented segment".to_string(),
                next_artifact: "data/tier-node-exceptions.csv".to_string(),
                optimizer_effect: "blocked until route family is disambiguated".to_string(),
                qualification_effects: "qualification_gate_policy=stop-first".to_string(),
                closure_status: "open".to_string(),
                validation_status: "review".to_string(),
            },
            T2BlockerClosureRow {
                route: "I205".to_string(),
                segment_bundle_id: String::new(),
                bundle_status: "bundle-missing".to_string(),
                bundle_action: "resolve route family or add segment bundle".to_string(),
                source_surface: "t2-graph-contact-repairs".to_string(),
                blocker_class: "route-family-split".to_string(),
                blocker_action: "split-numbered-route-family-before-tier-decision".to_string(),
                required_evidence: "identify represented segment".to_string(),
                next_artifact: "data/tier-node-exceptions.csv".to_string(),
                optimizer_effect: "blocked until route family is disambiguated".to_string(),
                qualification_effects: "qualification_gate_policy=stop-first".to_string(),
                closure_status: "open".to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let exceptions = vec![
            EndpointExceptionRow {
                route: "I195".to_string(),
                requested_tier: "T2".to_string(),
                endpoint_name: "Numbered I-195 route family".to_string(),
                endpoint_role: "graph_endpoint_gap".to_string(),
                exception_type: "missing_graph_geometry".to_string(),
                evidence_level: "missing_graph_data".to_string(),
                artifact: "data/tier-table.csv".to_string(),
                next_step: "disambiguate segment".to_string(),
            },
            EndpointExceptionRow {
                route: "I205".to_string(),
                requested_tier: "T2".to_string(),
                endpoint_name: "Portland local loop".to_string(),
                endpoint_role: "local_access_end".to_string(),
                exception_type: "missing_graph_geometry".to_string(),
                evidence_level: "missing_graph_data".to_string(),
                artifact: "data/tier-table.csv".to_string(),
                next_step: "confirm graph contact".to_string(),
            },
        ];

        let rows = crate::support::tier::t2_route_family_split_rows::t2_route_family_split_rows(&closure_rows, &[], &[], &exceptions);
        let failures = t2_route_family_split_gate_failures(&rows);

        assert_eq!(rows[0].family_action, "split-numbered-family");
        assert_eq!(rows[0].disposition, "blocked");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_gate_policy=stop-first"
        );
        assert!(rows[0]
            .optimizer_effect
            .contains("qualification_gate_policy=stop-first"));
        assert_eq!(rows[1].family_action, "split-local-family-or-demote");
        assert_eq!(rows[1].disposition, "lower-tier-pressure");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_route_family_splits_emit_clear_row_when_no_split_blockers_remain() {
        let rows = crate::support::tier::t2_route_family_split_rows::t2_route_family_split_rows(&[], &[], &[], &[]);
        let failures = t2_route_family_split_gate_failures(&rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].route, "__all_t2_route_family_splits__");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_route_family_splits_include_service_diagnostic_families() {
        let service_rows = vec![T2ServiceDiagnosticQueueRow {
            route: "I295".to_string(),
            region_id: "component-0".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            bundle_status: "bundle-ready".to_string(),
            selection_action: "source-needed".to_string(),
            selection_basis: "missing-beck-t2-diagnostic".to_string(),
            qualification_effects: "qualification_gate_policy=stop-first".to_string(),
            diagnostic_status: "route-family-diagnostic-split-needed".to_string(),
            service_diagnostic_action: "split-numbered-route-family-before-beck-diagnostic"
                .to_string(),
            required_artifact: "data/t2-service-selection.csv".to_string(),
            next_artifact: "data/national-segment-bundles.csv".to_string(),
            optimizer_effect: "holds multi-state route label".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = crate::support::tier::t2_route_family_split_rows::t2_route_family_split_rows(&[], &service_rows, &[], &[]);
        let failures = t2_route_family_split_gate_failures(&rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].route, "I295");
        assert_eq!(rows[0].family_action, "split-numbered-service-family");
        assert_eq!(rows[0].next_artifact, "data/national-segment-bundles.csv");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_graph_contact_validation_splits_observed_contact_from_demotions() {
        let closure_rows = vec![
            T2BlockerClosureRow {
                route: "I30".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.TEST.I30".to_string(),
                bundle_status: "bundle-ready".to_string(),
                bundle_action: "use bundle as service join surface".to_string(),
                source_surface: "t2-graph-contact-repairs".to_string(),
                blocker_class: "graph-contact-repair".to_string(),
                blocker_action: "repair-route-geometry-or-demote".to_string(),
                required_evidence: "prove contact".to_string(),
                next_artifact: "data/tier-contact-witnesses.csv".to_string(),
                optimizer_effect: "blocked until contact exists".to_string(),
                qualification_effects: "qualification_gate_policy=stop-first".to_string(),
                closure_status: "open".to_string(),
                validation_status: "review".to_string(),
            },
            T2BlockerClosureRow {
                route: "I49".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.TEST.I49".to_string(),
                bundle_status: "bundle-ready".to_string(),
                bundle_action: "use bundle as service join surface".to_string(),
                source_surface: "t2-graph-contact-repairs".to_string(),
                blocker_class: "graph-contact-repair".to_string(),
                blocker_action: "repair-route-geometry-or-demote".to_string(),
                required_evidence: "prove contact".to_string(),
                next_artifact: "data/tier-contact-witnesses.csv".to_string(),
                optimizer_effect: "blocked until contact exists".to_string(),
                qualification_effects: "qualification_game_use=default-play".to_string(),
                closure_status: "open".to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let witnesses = vec![
            TierContactWitnessInputRow {
                tier: "T2".to_string(),
                route: "I30".to_string(),
                witness_type: "graph-contact-needed".to_string(),
                node_class: "missing_graph_data".to_string(),
                route_miles: 755.0,
                observed_t1_node_count: 0,
                observed_parent_trunks: String::new(),
                observed_dual_contacts: 1,
                component_id: 1,
                component_route_count: 18,
                component_status: "component-bridged:21".to_string(),
                repair_action: "fix-graph-contact-or-demote".to_string(),
                repair_basis: "missing-t1-contact-evidence".to_string(),
                evidence_status: "source-needed".to_string(),
                required_artifact: "data/tier-contact-witnesses.csv".to_string(),
                validation_status: "review".to_string(),
            },
            TierContactWitnessInputRow {
                tier: "T2".to_string(),
                route: "I49".to_string(),
                witness_type: "graph-contact-needed".to_string(),
                node_class: "missing_graph_data".to_string(),
                route_miles: 1118.0,
                observed_t1_node_count: 0,
                observed_parent_trunks: String::new(),
                observed_dual_contacts: 0,
                component_id: 13,
                component_route_count: 1,
                component_status: "component-bridged:21".to_string(),
                repair_action: "fix-graph-contact-or-demote".to_string(),
                repair_basis: "missing-t1-contact-evidence".to_string(),
                evidence_status: "source-needed".to_string(),
                required_artifact: "data/tier-contact-witnesses.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t2_graph_contact_validation_rows(&closure_rows, &witnesses);
        let failures = t2_graph_contact_validation_gate_failures(&rows);

        assert_eq!(rows[0].contact_action, "accept-observed-graph-contact");
        assert_eq!(rows[0].disposition, "candidate-review");
        assert_eq!(rows[1].contact_action, "demote-unless-graph-contact-added");
        assert_eq!(rows[1].disposition, "lower-tier-pressure");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_contact_closure_rolls_up_parent_relief_and_terminal_contact() {
        let closure_rows = vec![
            T2BlockerClosureRow {
                route: "I285".to_string(),
                segment_bundle_id: String::new(),
                bundle_status: "bundle-missing".to_string(),
                bundle_action: "resolve route family or add segment bundle".to_string(),
                source_surface: "t2-relief-evidence-docket".to_string(),
                blocker_class: "relief-contact-repair".to_string(),
                blocker_action: "source-observed-relief-review".to_string(),
                required_evidence: "atri-bottleneck-route-match".to_string(),
                next_artifact: "data/tier-contact-witnesses.csv".to_string(),
                optimizer_effect: "retain relief review only after contact repair validates"
                    .to_string(),
                qualification_effects: "qualification_gate_policy=stop-first".to_string(),
                closure_status: "evidence-observed".to_string(),
                validation_status: "review".to_string(),
            },
            T2BlockerClosureRow {
                route: "I25".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.TEST.I25".to_string(),
                bundle_status: "bundle-ready".to_string(),
                bundle_action: "use bundle as service join surface".to_string(),
                source_surface: "t2-terminal-contact-validation".to_string(),
                blocker_class: "terminal-contact-repair".to_string(),
                blocker_action: "prove-terminal-contact-or-demote".to_string(),
                required_evidence: "terminal endpoint plus graph contact".to_string(),
                next_artifact: "data/tier-contact-witnesses.csv".to_string(),
                optimizer_effect: "blocked from T2 until graph contact validates".to_string(),
                qualification_effects: "qualification_game_use=default-play".to_string(),
                closure_status: "open".to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let witnesses = vec![
            TierContactWitnessInputRow {
                tier: "T2".to_string(),
                route: "I285".to_string(),
                witness_type: "graph-contact-needed".to_string(),
                node_class: "missing_graph_data".to_string(),
                route_miles: 172.0,
                observed_t1_node_count: 0,
                observed_parent_trunks: String::new(),
                observed_dual_contacts: 1,
                component_id: 1,
                component_route_count: 18,
                component_status: "component-bridged:21".to_string(),
                repair_action: "fix-graph-contact-or-demote".to_string(),
                repair_basis: "missing-t1-contact-evidence".to_string(),
                evidence_status: "source-needed".to_string(),
                required_artifact: "data/tier-contact-witnesses.csv".to_string(),
                validation_status: "review".to_string(),
            },
            TierContactWitnessInputRow {
                tier: "T2".to_string(),
                route: "I25".to_string(),
                witness_type: "graph-contact-needed".to_string(),
                node_class: "missing_graph_data".to_string(),
                route_miles: 2126.0,
                observed_t1_node_count: 0,
                observed_parent_trunks: String::new(),
                observed_dual_contacts: 0,
                component_id: 8,
                component_route_count: 1,
                component_status: "component-bridged:21".to_string(),
                repair_action: "fix-graph-contact-or-demote".to_string(),
                repair_basis: "missing-t1-contact-evidence".to_string(),
                evidence_status: "source-needed".to_string(),
                required_artifact: "data/tier-contact-witnesses.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t2_contact_closure_rows(&closure_rows, &witnesses);
        let failures = t2_contact_closure_gate_failures(&rows);

        assert_eq!(rows[0].contact_action, "accept-observed-contact");
        assert_eq!(rows[0].disposition, "candidate-review");
        assert_eq!(rows[1].contact_action, "demote-unless-contact-added");
        assert_eq!(rows[1].disposition, "lower-tier-pressure");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_contact_closure_emits_clear_row_when_no_contact_closure_blockers_remain() {
        let rows = t2_contact_closure_rows(&[], &[]);
        let failures = t2_contact_closure_gate_failures(&rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].route, "__all_t2_contact_closures__");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_endpoint_closure_demotes_non_terminal_worthy_exceptions() {
        let closure_rows = vec![T2BlockerClosureRow {
            route: "I270".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.TEST.I270".to_string(),
            bundle_status: "needs-stop-chain".to_string(),
            bundle_action: "author zone-bounded stops before bundle geometry".to_string(),
            source_surface: "t2-terminal-contact-validation".to_string(),
            blocker_class: "endpoint-exception-upgrade".to_string(),
            blocker_action: "prove-terminal-exception-or-demote".to_string(),
            required_evidence: "terminal-worthy endpoint exception".to_string(),
            next_artifact: "data/tier-node-exceptions.csv".to_string(),
            optimizer_effect: "blocked from T2 unless endpoint exception is upgraded".to_string(),
            qualification_effects: "qualification_gate_policy=stop-first".to_string(),
            closure_status: "open".to_string(),
            validation_status: "review".to_string(),
        }];
        let exceptions = vec![EndpointExceptionRow {
            route: "I270".to_string(),
            requested_tier: "T2".to_string(),
            endpoint_name: "St. Louis / Columbus beltway".to_string(),
            endpoint_role: "one_ended_feeder".to_string(),
            exception_type: "metro_beltway_relief".to_string(),
            evidence_level: "heuristic".to_string(),
            artifact: "data/atri-bottlenecks.csv".to_string(),
            next_step: "validate endpoint contacts".to_string(),
        }];

        let rows = t2_endpoint_closure_rows(&closure_rows, &exceptions);
        let failures = t2_endpoint_closure_gate_failures(&rows);

        assert_eq!(rows.len(), 1);
        assert!(!rows[0].terminal_worthy);
        assert_eq!(
            rows[0].endpoint_action,
            "upgrade-endpoint-exception-or-demote"
        );
        assert_eq!(rows[0].disposition, "lower-tier-pressure");
        assert!(failures.is_empty());
    }

    #[test]
    fn tier_candidate_columns_select_only_accepted_witnesses() {
        let rows = vec![
            TierContactWitnessInputRow {
                tier: "T2".to_string(),
                route: "I15".to_string(),
                witness_type: "regionalizer-ready".to_string(),
                node_class: "trunk_connector".to_string(),
                route_miles: 2882.0,
                observed_t1_node_count: 6,
                observed_parent_trunks: "I84;I90".to_string(),
                observed_dual_contacts: 3,
                component_id: 1,
                component_route_count: 18,
                component_status: "component-bridged:2".to_string(),
                repair_action: "keep-for-regionalizer".to_string(),
                repair_basis: "touches-multiple-t1-trunks".to_string(),
                evidence_status: "accepted".to_string(),
                required_artifact: "data/tier-candidate-columns.csv".to_string(),
                validation_status: "pass".to_string(),
            },
            TierContactWitnessInputRow {
                tier: "T2".to_string(),
                route: "I110".to_string(),
                witness_type: "graph-contact-needed".to_string(),
                node_class: "missing_graph_data".to_string(),
                route_miles: 79.0,
                observed_t1_node_count: 0,
                observed_parent_trunks: String::new(),
                observed_dual_contacts: 0,
                component_id: 0,
                component_route_count: 1,
                component_status: "component-bridged:2".to_string(),
                repair_action: "fix-graph-contact-or-demote".to_string(),
                repair_basis: "missing-t1-contact-evidence".to_string(),
                evidence_status: "source-needed".to_string(),
                required_artifact: "data/tier-contact-witnesses.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let columns = tier_candidate_column_rows(
            &rows,
            &std::collections::HashMap::new(),
            &PavementDebtBudgetIndex::default(),
            &OptimizerConstraintBudgetIndex::default(),
        );
        let failures = tier_candidate_column_gate_failures(&columns);

        assert_eq!(columns[0].column_decision, "selected");
        assert_eq!(columns[1].column_decision, "blocked");
        assert!(failures.is_empty());
    }

    #[test]
    fn tier_candidate_columns_consume_candidate_review_closures() {
        let rows = vec![TierContactWitnessInputRow {
            tier: "T2".to_string(),
            route: "I30".to_string(),
            witness_type: "graph-contact-needed".to_string(),
            node_class: "missing_graph_data".to_string(),
            route_miles: 755.0,
            observed_t1_node_count: 0,
            observed_parent_trunks: String::new(),
            observed_dual_contacts: 1,
            component_id: 1,
            component_route_count: 18,
            component_status: "component-bridged:21".to_string(),
            repair_action: "fix-graph-contact-or-demote".to_string(),
            repair_basis: "missing-t1-contact-evidence".to_string(),
            evidence_status: "source-needed".to_string(),
            required_artifact: "data/tier-contact-witnesses.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let graph_rows = vec![T2GraphContactValidationRow {
            route: "I30".to_string(),
            observed_t1_node_count: 0,
            observed_dual_contacts: 1,
            observed_parent_trunks: String::new(),
            contact_action: "accept-observed-graph-contact".to_string(),
            disposition: "candidate-review".to_string(),
            required_evidence: "observed T1/T2 graph contact".to_string(),
            next_artifact: "data/tier-candidate-columns.csv".to_string(),
            optimizer_effect: "eligible for T2 candidate-column review".to_string(),
            validation_status: "review".to_string(),
        }];
        let blocker_rows = vec![T2BlockerClosureRow {
            route: "I30".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I30".to_string(),
            bundle_status: "bundle-ready".to_string(),
            bundle_action: "use bundle as service join surface".to_string(),
            source_surface: "t2-graph-contact-repairs".to_string(),
            blocker_class: "graph-contact-repair".to_string(),
            blocker_action: "repair-route-geometry-or-demote".to_string(),
            required_evidence: "prove contact".to_string(),
            next_artifact: "data/tier-contact-witnesses.csv".to_string(),
            optimizer_effect: "blocked until contact exists".to_string(),
            qualification_effects: "qualification_gate_policy=stop-first".to_string(),
            closure_status: "open".to_string(),
            validation_status: "review".to_string(),
        }];
        let dispositions = t2_closure_dispositions(&[], &graph_rows, &[], &[], &blocker_rows);
        let pavement_debt_rows = vec![TierPavementDebtBudgetRow {
            tier: "T2".to_string(),
            route: "I30".to_string(),
            region_id: "component-1".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I30".to_string(),
            stitch_group_id: "US.HWYSTITCH.I30".to_string(),
            debt_class: "repair-debt".to_string(),
            blocked_member_count: 2,
            affected_states: "TX".to_string(),
            evidence_debt_units: 0,
            repair_debt_units: 2,
            estimated_evidence_cost_m: 0.0,
            estimated_repair_cost_m: 5.0,
            total_debt_cost_m: 5.0,
            budget_basis: "fixture pavement repair debt".to_string(),
            optimizer_penalty: "subtract 5.00 budget-cost units".to_string(),
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let pavement_debt_index = pavement_debt_budget_index(&pavement_debt_rows);

        let columns = tier_candidate_column_rows(
            &rows,
            &dispositions,
            &pavement_debt_index,
            &OptimizerConstraintBudgetIndex::default(),
        );

        assert_eq!(columns[0].column_decision, "review");
        assert_eq!(columns[0].bundle_status, "bundle-ready");
        assert_eq!(
            columns[0].qualification_effects,
            "qualification_gate_policy=stop-first"
        );
        assert_eq!(columns[0].pavement_debt_cost_m, 5.0);
        assert_eq!(columns[0].pavement_debt_class, "repair-debt");
        assert_eq!(columns[0].evidence_status, "closure-accepted-bundle-ready");
        assert_eq!(
            columns[0].required_artifact,
            "data/t2-graph-contact-validation.csv"
        );
    }

    #[test]
    fn tier_candidate_columns_block_candidate_review_without_bundle() {
        let rows = vec![TierContactWitnessInputRow {
            tier: "T2".to_string(),
            route: "I285".to_string(),
            witness_type: "graph-contact-needed".to_string(),
            node_class: "missing_graph_data".to_string(),
            route_miles: 172.0,
            observed_t1_node_count: 0,
            observed_parent_trunks: String::new(),
            observed_dual_contacts: 1,
            component_id: 1,
            component_route_count: 18,
            component_status: "component-bridged:21".to_string(),
            repair_action: "fix-graph-contact-or-demote".to_string(),
            repair_basis: "missing-t1-contact-evidence".to_string(),
            evidence_status: "source-needed".to_string(),
            required_artifact: "data/tier-contact-witnesses.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let contact_rows = vec![T2ContactClosureRow {
            route: "I285".to_string(),
            blocker_class: "relief-contact-repair".to_string(),
            observed_t1_node_count: 0,
            observed_dual_contacts: 1,
            observed_parent_trunks: String::new(),
            contact_action: "accept-observed-contact".to_string(),
            disposition: "candidate-review".to_string(),
            required_evidence: "observed T1/T2 contact".to_string(),
            next_artifact: "data/tier-candidate-columns.csv".to_string(),
            optimizer_effect: "eligible for T2 candidate-column review".to_string(),
            validation_status: "review".to_string(),
        }];
        let blocker_rows = vec![T2BlockerClosureRow {
            route: "I285".to_string(),
            segment_bundle_id: String::new(),
            bundle_status: "bundle-missing".to_string(),
            bundle_action: "resolve route family or add segment bundle".to_string(),
            source_surface: "t2-relief-evidence-docket".to_string(),
            blocker_class: "relief-contact-repair".to_string(),
            blocker_action: "source-observed-relief-review".to_string(),
            required_evidence: "atri-bottleneck-route-match".to_string(),
            next_artifact: "data/tier-contact-witnesses.csv".to_string(),
            optimizer_effect: "retain relief review only after contact repair validates"
                .to_string(),
            qualification_effects: "qualification_game_use=default-play".to_string(),
            closure_status: "evidence-observed".to_string(),
            validation_status: "review".to_string(),
        }];
        let dispositions = t2_closure_dispositions(&[], &[], &contact_rows, &[], &blocker_rows);

        let columns = tier_candidate_column_rows(
            &rows,
            &dispositions,
            &PavementDebtBudgetIndex::default(),
            &OptimizerConstraintBudgetIndex::default(),
        );

        assert_eq!(columns[0].column_decision, "blocked");
        assert_eq!(columns[0].bundle_status, "bundle-missing");
        assert_eq!(columns[0].evidence_status, "closure-bundle-pending");
        assert_eq!(columns[0].required_artifact, "data/t2-blocker-closure.csv");
    }

    #[test]
    fn t2_bundle_repair_queue_names_bundle_blockers() {
        let candidates = vec![TierCandidateColumnRow {
            tier: "T2".to_string(),
            route: "I285".to_string(),
            candidate_type: "route-service-column".to_string(),
            graph_kind: "dual-route-graph".to_string(),
            split_objective: "route-mile-workload".to_string(),
            node_class: "missing_graph_data".to_string(),
            route_miles: 172.0,
            observed_t1_node_count: 0,
            observed_dual_contacts: 1,
            parent_trunks: String::new(),
            component_id: 1,
            component_route_count: 18,
            component_status: "component-bridged:21".to_string(),
            witness_type: "graph-contact-needed".to_string(),
            repair_action: "fix-graph-contact-or-demote".to_string(),
            repair_basis: "observed T1/T2 contact".to_string(),
            segment_bundle_id: String::new(),
            bundle_status: "bundle-missing".to_string(),
            bundle_action: "resolve route family or add segment bundle".to_string(),
            pavement_debt_cost_m: 0.0,
            pavement_debt_class: "none".to_string(),
            pavement_debt_basis: "no pavement debt row joined".to_string(),
            pavement_debt_artifact: String::new(),
            hard_blocker_count: 0,
            claim_blocker_count: 0,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 0.0,
            top_constraint_classes: "none".to_string(),
            qualification_effects: "qualification_gate_policy=stop-first".to_string(),
            constraint_ledger_artifact: String::new(),
            column_decision: "blocked".to_string(),
            evidence_status: "closure-bundle-pending".to_string(),
            required_artifact: "data/t2-blocker-closure.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let blockers = vec![T2BlockerClosureRow {
            route: "I285".to_string(),
            segment_bundle_id: String::new(),
            bundle_status: "bundle-missing".to_string(),
            bundle_action: "resolve route family or add segment bundle".to_string(),
            source_surface: "t2-relief-evidence-docket".to_string(),
            blocker_class: "relief-contact-repair".to_string(),
            blocker_action: "source-observed-relief-review".to_string(),
            required_evidence: "atri-bottleneck-route-match".to_string(),
            next_artifact: "data/tier-contact-witnesses.csv".to_string(),
            optimizer_effect: "retain relief review only after contact repair validates"
                .to_string(),
            qualification_effects: "qualification_gate_policy=stop-first".to_string(),
            closure_status: "evidence-observed".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows: Vec<T2BundleRepairQueueRow> = t2_bundle_repair_queue_rows(&candidates, &blockers);
        let failures = t2_bundle_repair_queue_gate_failures(&rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].route, "I285");
        assert_eq!(rows[0].repair_class, "relief-contact-repair");
        assert_eq!(
            rows[0].repair_action,
            "add-or-split-segment-bundle-before-regionalizer"
        );
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].next_artifact, "data/national-segment-bundles.csv");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_bundle_repair_queue_emits_clearance_when_no_bundle_blockers_remain() {
        let candidates = vec![TierCandidateColumnRow {
            tier: "T2".to_string(),
            route: "I285".to_string(),
            candidate_type: "route-service-column".to_string(),
            graph_kind: "dual-route-graph".to_string(),
            split_objective: "route-mile-workload".to_string(),
            node_class: "missing_graph_data".to_string(),
            route_miles: 172.0,
            observed_t1_node_count: 0,
            observed_dual_contacts: 1,
            parent_trunks: String::new(),
            component_id: 1,
            component_route_count: 18,
            component_status: "component-bridged:21".to_string(),
            witness_type: "graph-contact-needed".to_string(),
            repair_action: "fix-graph-contact-or-demote".to_string(),
            repair_basis: "observed T1/T2 contact".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I285".to_string(),
            bundle_status: "bundle-ready".to_string(),
            bundle_action: "use bundle as service join surface".to_string(),
            pavement_debt_cost_m: 0.0,
            pavement_debt_class: "none".to_string(),
            pavement_debt_basis: "no pavement debt row joined".to_string(),
            pavement_debt_artifact: String::new(),
            hard_blocker_count: 0,
            claim_blocker_count: 0,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 0.0,
            top_constraint_classes: "none".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            constraint_ledger_artifact: String::new(),
            column_decision: "review".to_string(),
            evidence_status: "closure-accepted-bundle-ready".to_string(),
            required_artifact: "data/t2-contact-closure.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_bundle_repair_queue_rows(&candidates, &[]);
        let failures = t2_bundle_repair_queue_gate_failures(&rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].route, "__all_t2_bundle_repairs__");
        assert_eq!(rows[0].bundle_status, "bundle-repair-clear");
        assert_eq!(rows[0].next_artifact, "data/t2-service-selection.csv");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_service_diagnostic_queue_names_missing_beck_rows() {
        let service_rows = vec![T2ServiceSelectionRow {
            tier: "T2".to_string(),
            region_id: "component-1".to_string(),
            route: "I285".to_string(),
            parent_trunks: String::new(),
            column_decision: "review".to_string(),
            treatment_status: "review-treatment".to_string(),
            beck_corridor: String::new(),
            beck_service_class: String::new(),
            beck_color_mode: String::new(),
            beck_start_trunk: String::new(),
            beck_end_trunk: String::new(),
            duplicate_service_count: 0,
            duplicate_service_corridors: String::new(),
            close_parallel_count: 0,
            close_parallel_corridors: String::new(),
            unstopped_t1_contact_count: 0,
            unstopped_t1_contacts: String::new(),
            pavement_debt_cost_m: 0.0,
            pavement_debt_class: "none".to_string(),
            pavement_debt_basis: "no pavement debt row joined".to_string(),
            hard_blocker_count: 0,
            claim_blocker_count: 0,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 0.0,
            top_constraint_classes: "none".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            constraint_ledger_artifact: String::new(),
            beck_service_action: String::new(),
            qualification_basis: String::new(),
            qualification_map_treatment: String::new(),
            qualification_gate_policy: String::new(),
            qualification_game_use: String::new(),
            selection_action: "source-needed".to_string(),
            selection_basis: "missing-beck-t2-diagnostic".to_string(),
            validation_status: "review".to_string(),
        }];
        let bundles = vec![NationalSegmentBundleRow {
            segment_bundle_id: "US.HWYBUNDLE.I285".to_string(),
            bundle_role: "stitched-service".to_string(),
            member_segment_ids: "US.HWYSEG.I285".to_string(),
            member_count: 1,
            stitch_group_ids: "US.HWYSTITCH.I285".to_string(),
            current_tiers: "T2".to_string(),
            current_zone_ids: "component-1".to_string(),
            route_labels: "I285".to_string(),
            state_scope: String::new(),
            evidence_state_scope: String::new(),
            geometry_state_scope: String::new(),
            bundle_aliases: "route:I285;route-label:I285".to_string(),
            source_artifacts: "fixture".to_string(),
            bundle_status: "bundle-ready".to_string(),
            bundle_action: "use bundle as service join surface".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            next_artifact: "maps/t3-zone".to_string(),
            validation_status: "pass".to_string(),
        }];

        let rows: Vec<T2ServiceDiagnosticQueueRow> =
            t2_service_diagnostic_queue_rows(&service_rows, &bundles);
        let failures = t2_service_diagnostic_queue_gate_failures(&rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].route, "I285");
        assert_eq!(rows[0].segment_bundle_id, "US.HWYBUNDLE.I285");
        assert_eq!(rows[0].diagnostic_status, "local-relief-map-review");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert!(rows[0]
            .optimizer_effect
            .contains("qualification_gate_policy=stop-first"));
        assert_eq!(rows[0].next_artifact, "data/t3-t4-pressure-intake.csv");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_service_diagnostic_queue_splits_multistate_route_families() {
        let service_rows = vec![T2ServiceSelectionRow {
            tier: "T2".to_string(),
            region_id: "component-0".to_string(),
            route: "I295".to_string(),
            parent_trunks: "I64;I95".to_string(),
            column_decision: "selected".to_string(),
            treatment_status: "selected-treatment".to_string(),
            beck_corridor: String::new(),
            beck_service_class: String::new(),
            beck_color_mode: String::new(),
            beck_start_trunk: String::new(),
            beck_end_trunk: String::new(),
            duplicate_service_count: 0,
            duplicate_service_corridors: String::new(),
            close_parallel_count: 0,
            close_parallel_corridors: String::new(),
            unstopped_t1_contact_count: 0,
            unstopped_t1_contacts: String::new(),
            pavement_debt_cost_m: 0.0,
            pavement_debt_class: "none".to_string(),
            pavement_debt_basis: "no pavement debt row joined".to_string(),
            hard_blocker_count: 0,
            claim_blocker_count: 0,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 0.0,
            top_constraint_classes: "none".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            constraint_ledger_artifact: String::new(),
            beck_service_action: String::new(),
            qualification_basis: String::new(),
            qualification_map_treatment: String::new(),
            qualification_gate_policy: String::new(),
            qualification_game_use: String::new(),
            selection_action: "source-needed".to_string(),
            selection_basis: "missing-beck-t2-diagnostic".to_string(),
            validation_status: "review".to_string(),
        }];
        let bundles = vec![NationalSegmentBundleRow {
            segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            bundle_role: "stitched-service".to_string(),
            member_segment_ids: "US.HWYSEG.I295".to_string(),
            member_count: 1,
            stitch_group_ids: "US.HWYSTITCH.I295".to_string(),
            current_tiers: "T2".to_string(),
            current_zone_ids: "component-0".to_string(),
            route_labels: "I295".to_string(),
            state_scope: "FL;GA;ME".to_string(),
            evidence_state_scope: "FL;GA;ME".to_string(),
            geometry_state_scope: "FL;GA;ME".to_string(),
            bundle_aliases: "route:I295;route-label:I295".to_string(),
            source_artifacts: "fixture".to_string(),
            bundle_status: "bundle-ready".to_string(),
            bundle_action: "use bundle as service join surface".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            next_artifact: "maps/t3-zone".to_string(),
            validation_status: "pass".to_string(),
        }];

        let rows = t2_service_diagnostic_queue_rows(&service_rows, &bundles);
        let failures = t2_service_diagnostic_queue_gate_failures(&rows);

        assert_eq!(
            rows[0].diagnostic_status,
            "route-family-diagnostic-split-needed"
        );
        assert_eq!(rows[0].next_artifact, "data/national-segment-bundles.csv");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_service_diagnostic_queue_skips_routes_before_bundle_readiness() {
        let service_rows = vec![T2ServiceSelectionRow {
            tier: "T2".to_string(),
            region_id: "component-1".to_string(),
            route: "I270".to_string(),
            parent_trunks: "I70".to_string(),
            column_decision: "selected".to_string(),
            treatment_status: "selected-treatment".to_string(),
            beck_corridor: String::new(),
            beck_service_class: String::new(),
            beck_color_mode: String::new(),
            beck_start_trunk: String::new(),
            beck_end_trunk: String::new(),
            duplicate_service_count: 0,
            duplicate_service_corridors: String::new(),
            close_parallel_count: 0,
            close_parallel_corridors: String::new(),
            unstopped_t1_contact_count: 0,
            unstopped_t1_contacts: String::new(),
            pavement_debt_cost_m: 0.0,
            pavement_debt_class: "none".to_string(),
            pavement_debt_basis: "no pavement debt row joined".to_string(),
            hard_blocker_count: 0,
            claim_blocker_count: 0,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 0.0,
            top_constraint_classes: "none".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            constraint_ledger_artifact: String::new(),
            beck_service_action: String::new(),
            qualification_basis: String::new(),
            qualification_map_treatment: String::new(),
            qualification_gate_policy: String::new(),
            qualification_game_use: String::new(),
            selection_action: "source-needed".to_string(),
            selection_basis: "missing-beck-t2-diagnostic".to_string(),
            validation_status: "review".to_string(),
        }];
        let bundles = vec![NationalSegmentBundleRow {
            segment_bundle_id: "US.HWYBUNDLE.I270".to_string(),
            bundle_role: "stitched-service".to_string(),
            member_segment_ids: "US.HWYSEG.I270".to_string(),
            member_count: 1,
            stitch_group_ids: "US.HWYSTITCH.I270".to_string(),
            current_tiers: "T2".to_string(),
            current_zone_ids: "component-1".to_string(),
            route_labels: "I270".to_string(),
            state_scope: String::new(),
            evidence_state_scope: String::new(),
            geometry_state_scope: String::new(),
            bundle_aliases: "route:I270;route-label:I270".to_string(),
            source_artifacts: "fixture".to_string(),
            bundle_status: "needs-stop-chain".to_string(),
            bundle_action: "author zone-bounded stops before bundle geometry".to_string(),
            qualification_effects: "qualification_game_use=default-play".to_string(),
            next_artifact: "data/tier-stop-candidates.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_service_diagnostic_queue_rows(&service_rows, &bundles);
        let failures = t2_service_diagnostic_queue_gate_failures(&rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].route, "__all_t2_service_diagnostics__");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_parallel_service_queue_names_close_parallel_review_rows() {
        let service_rows = vec![T2ServiceSelectionRow {
            tier: "T2".to_string(),
            region_id: "component-1".to_string(),
            route: "I59".to_string(),
            parent_trunks: "I20".to_string(),
            column_decision: "review".to_string(),
            treatment_status: "review-treatment".to_string(),
            beck_corridor: "I-59".to_string(),
            beck_service_class: "connector".to_string(),
            beck_color_mode: "split-parent".to_string(),
            beck_start_trunk: "I-10".to_string(),
            beck_end_trunk: "I-75".to_string(),
            duplicate_service_count: 0,
            duplicate_service_corridors: String::new(),
            close_parallel_count: 1,
            close_parallel_corridors: "I-65".to_string(),
            unstopped_t1_contact_count: 0,
            unstopped_t1_contacts: String::new(),
            pavement_debt_cost_m: 0.0,
            pavement_debt_class: "none".to_string(),
            pavement_debt_basis: "no pavement debt row joined".to_string(),
            hard_blocker_count: 0,
            claim_blocker_count: 0,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 0.0,
            top_constraint_classes: "none".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            constraint_ledger_artifact: String::new(),
            beck_service_action: "keep".to_string(),
            qualification_basis: "distinct-parent-service".to_string(),
            qualification_map_treatment: "draw as normal T2 service for its class".to_string(),
            qualification_gate_policy: "accepted when structural diagnostics pass".to_string(),
            qualification_game_use:
                "default playable service for incidents, upgrades, and restitches".to_string(),
            selection_action: "split-parallel-service".to_string(),
            selection_basis: "close-parallel-beck-service".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_parallel_service_queue_rows(&service_rows);
        let failures = t2_parallel_service_queue_gate_failures(&rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].route, "I59");
        assert_eq!(rows[0].close_parallel_corridors, "I-65");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert!(rows[0]
            .optimizer_effect
            .contains("qualification_gate_policy=stop-first"));
        assert_eq!(
            rows[0].parallel_action,
            "review-spacing-or-split-service-before-promotion"
        );
        assert!(failures.is_empty());
    }

    #[test]
    fn lower_tier_pressure_consumes_closure_demotions() {
        let tier_rows = vec![TierTableScoreRow {
            tier: "T2".to_string(),
            route: "I270".to_string(),
            score: 52.0,
            confidence: 0.75,
            confidence_label: "Medium".to_string(),
        }];
        let endpoint_rows = vec![T2EndpointClosureRow {
            route: "I270".to_string(),
            endpoint_name: "St. Louis / Columbus beltway".to_string(),
            endpoint_role: "one_ended_feeder".to_string(),
            exception_type: "metro_beltway_relief".to_string(),
            evidence_level: "heuristic".to_string(),
            terminal_worthy: false,
            endpoint_action: "upgrade-endpoint-exception-or-demote".to_string(),
            disposition: "lower-tier-pressure".to_string(),
            required_evidence: "terminal-worthy endpoint role and exception type".to_string(),
            next_artifact: "data/lower-tier-pressure-witnesses.csv".to_string(),
            optimizer_effect: "kept out of T2 until endpoint exception is upgraded".to_string(),
            validation_status: "review".to_string(),
        }];
        let dispositions = t2_closure_dispositions(&[], &[], &[], &endpoint_rows, &[]);

        let rows = lower_tier_pressure_witness_rows(&tier_rows, &[], &[], &dispositions);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].route, "I270");
        assert_eq!(rows[0].pressure_type, "closure-demotion-pressure");
        assert_eq!(rows[0].source_artifact, "data/t2-endpoint-closure.csv");
    }

    #[test]
    fn t3_t4_pressure_intake_keeps_lower_tier_pass_thin() {
        let pressure = vec![
            LowerTierPressureWitnessRow {
                route: "I25".to_string(),
                current_tier: "T2".to_string(),
                current_score: 66.3,
                confidence: 0.76,
                confidence_label: "Medium".to_string(),
                pressure_type: "closure-demotion-pressure".to_string(),
                witness_action: "demote-unless-contact-added".to_string(),
                target_tier: "T3/T4".to_string(),
                selection_basis: "source-backed T1/T2 contact".to_string(),
                source_artifact: "data/t2-contact-closure.csv".to_string(),
                next_artifact: "data/lower-tier-pressure-witnesses.csv".to_string(),
                validation_status: "review".to_string(),
            },
            LowerTierPressureWitnessRow {
                route: "I-57".to_string(),
                current_tier: "T3".to_string(),
                current_score: 49.6,
                confidence: 0.6,
                confidence_label: "Low".to_string(),
                pressure_type: "regional-upgrade-pressure".to_string(),
                witness_action: "evaluate-for-t2-upgrade-candidate".to_string(),
                target_tier: "T2".to_string(),
                selection_basis: "score-within-five-points-of-t2-threshold".to_string(),
                source_artifact: "data/tier-table.csv".to_string(),
                next_artifact: "data/tier-contact-witnesses.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t3_t4_pressure_intake_rows(&pressure);
        let failures = t3_t4_pressure_intake_gate_failures(&rows);
        let classes = rows
            .iter()
            .map(|row| row.intake_class.as_str())
            .collect::<std::collections::BTreeSet<_>>();

        assert!(classes.contains("t3-regional-intake"));
        assert!(classes.contains("bubble-up-t2-review"));
        assert!(failures.is_empty());
    }

    fn test_t3_map_atlas_row(map_id: &str) -> MapAtlasRow {
        MapAtlasRow {
            map_id: map_id.to_string(),
            path: format!("maps/{map_id}.png"),
            map_type: "t3-zone".to_string(),
            render_command: format!("route map {map_id} --output maps/{map_id}.png"),
            expected_width: 1800,
            expected_height: 1000,
            min_bytes: 80_000,
            tier_role: "T3 zone schematic".to_string(),
            game_use: "Regional feeder planning".to_string(),
        }
    }

    #[test]
    fn t3_zone_access_obligations_group_pressure_by_zone_maps() {
        let intake = vec![
            T3T4PressureIntakeRow {
                route: "I-57".to_string(),
                source_pressure_type: "regional-upgrade-pressure".to_string(),
                current_tier: "T3".to_string(),
                current_score: 49.6,
                target_tier: "T2".to_string(),
                intake_class: "bubble-up-t2-review".to_string(),
                intake_action: "send-to-t2-contact-review".to_string(),
                selection_basis: "score-within-five-points-of-t2-threshold".to_string(),
                source_artifact: "data/tier-table.csv".to_string(),
                next_artifact: "data/tier-contact-witnesses.csv".to_string(),
                optimizer_effect:
                    "lower-tier score pressure can reopen T2 only through contact gates".to_string(),
                validation_status: "review".to_string(),
            },
            T3T4PressureIntakeRow {
                route: "I25".to_string(),
                source_pressure_type: "closure-demotion-pressure".to_string(),
                current_tier: "T2".to_string(),
                current_score: 66.3,
                target_tier: "T3".to_string(),
                intake_class: "t3-regional-intake".to_string(),
                intake_action: "accept-as-t3-regional-review".to_string(),
                selection_basis: "source-backed T1/T2 contact".to_string(),
                source_artifact: "data/t2-contact-closure.csv".to_string(),
                next_artifact: "data/t3-t4-pressure-intake.csv".to_string(),
                optimizer_effect: "consume T2 demotion as regional feeder review".to_string(),
                validation_status: "review".to_string(),
            },
            T3T4PressureIntakeRow {
                route: "I-2".to_string(),
                source_pressure_type: "regional-upgrade-pressure".to_string(),
                current_tier: "T3".to_string(),
                current_score: 45.7,
                target_tier: "T2".to_string(),
                intake_class: "bubble-up-t2-review".to_string(),
                intake_action: "send-to-t2-contact-review".to_string(),
                selection_basis: "score-within-five-points-of-t2-threshold".to_string(),
                source_artifact: "data/tier-table.csv".to_string(),
                next_artifact: "data/tier-contact-witnesses.csv".to_string(),
                optimizer_effect:
                    "lower-tier score pressure can reopen T2 only through contact gates".to_string(),
                validation_status: "review".to_string(),
            },
            T3T4PressureIntakeRow {
                route: "I-74".to_string(),
                source_pressure_type: "regional-upgrade-pressure".to_string(),
                current_tier: "T3".to_string(),
                current_score: 47.2,
                target_tier: "T2".to_string(),
                intake_class: "bubble-up-t2-review".to_string(),
                intake_action: "send-to-t2-contact-review".to_string(),
                selection_basis: "score-within-five-points-of-t2-threshold".to_string(),
                source_artifact: "data/tier-table.csv".to_string(),
                next_artifact: "data/tier-contact-witnesses.csv".to_string(),
                optimizer_effect:
                    "lower-tier score pressure can reopen T2 only through contact gates".to_string(),
                validation_status: "review".to_string(),
            },
            T3T4PressureIntakeRow {
                route: "I-630".to_string(),
                source_pressure_type: "regional-upgrade-pressure".to_string(),
                current_tier: "T3".to_string(),
                current_score: 46.7,
                target_tier: "T2".to_string(),
                intake_class: "bubble-up-t2-review".to_string(),
                intake_action: "send-to-t2-contact-review".to_string(),
                selection_basis: "score-within-five-points-of-t2-threshold".to_string(),
                source_artifact: "data/tier-table.csv".to_string(),
                next_artifact: "data/tier-contact-witnesses.csv".to_string(),
                optimizer_effect:
                    "lower-tier score pressure can reopen T2 only through contact gates".to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let atlas = vec![
            test_t3_map_atlas_row("t3-great-lakes"),
            test_t3_map_atlas_row("t3-southeast"),
            test_t3_map_atlas_row("t3-texas-border"),
            test_t3_map_atlas_row("t3-mountain-west"),
            test_t3_map_atlas_row("t3-mid-south"),
        ];

        let rows = t3_zone_access_obligation_rows(&intake, &atlas);
        let failures = t3_zone_access_obligation_gate_failures(&rows, &atlas);
        let zones = rows
            .iter()
            .map(|row| row.zone_id.as_str())
            .collect::<std::collections::BTreeSet<_>>();

        assert!(zones.contains("t3-great-lakes"));
        assert!(zones.contains("t3-southeast"));
        assert!(zones.contains("t3-texas-border"));
        assert!(zones.contains("t3-mountain-west"));
        assert!(zones.contains("t3-mid-south"));
        assert!(rows
            .iter()
            .any(|row| row.obligation_class == "regional-feeder-access"));
        assert!(rows
            .iter()
            .any(|row| row.obligation_class == "regional-upgrade-review"));
        assert!(failures.is_empty());
    }

    #[test]
    fn t3_zone_route_columns_select_threshold_feeders_and_hold_upgrades() {
        let obligations = vec![
            T3ZoneAccessObligationRow {
                zone_id: "t3-mountain-west".to_string(),
                zone_name: "Mountain West / Interior Coverage".to_string(),
                obligation_class: "regional-feeder-access".to_string(),
                access_target: "select T3 feeder/contact chain inside the zone".to_string(),
                promise_horizon_hours: 6,
                source_route_count: 2,
                candidate_routes: "I25;I-135".to_string(),
                source_intake_classes: "t3-regional-intake".to_string(),
                map_id: "t3-mountain-west".to_string(),
                next_artifact: "data/t3-zone-route-columns.csv".to_string(),
                optimizer_effect:
                    "turns lower-tier pressure into regional feeder obligations for zone maps"
                        .to_string(),
                validation_status: "review".to_string(),
            },
            T3ZoneAccessObligationRow {
                zone_id: "t3-mountain-west".to_string(),
                zone_name: "Mountain West / Interior Coverage".to_string(),
                obligation_class: "regional-upgrade-review".to_string(),
                access_target: "prove T2 contact and regional service value before upgrade"
                    .to_string(),
                promise_horizon_hours: 24,
                source_route_count: 1,
                candidate_routes: "I-8".to_string(),
                source_intake_classes: "bubble-up-t2-review".to_string(),
                map_id: "t3-mountain-west".to_string(),
                next_artifact: "data/t2-bubble-up-review.csv".to_string(),
                optimizer_effect:
                    "keeps lower-tier upgrade pressure attached to zone maps before any T2 reopening"
                        .to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let intake = vec![
            T3T4PressureIntakeRow {
                route: "I25".to_string(),
                source_pressure_type: "closure-demotion-pressure".to_string(),
                current_tier: "T2".to_string(),
                current_score: 66.3,
                target_tier: "T3".to_string(),
                intake_class: "t3-regional-intake".to_string(),
                intake_action: "accept-as-t3-regional-review".to_string(),
                selection_basis: "source-backed T1/T2 contact".to_string(),
                source_artifact: "data/t2-contact-closure.csv".to_string(),
                next_artifact: "data/t3-t4-pressure-intake.csv".to_string(),
                optimizer_effect: "consume T2 demotion as regional feeder review".to_string(),
                validation_status: "review".to_string(),
            },
            T3T4PressureIntakeRow {
                route: "I-135".to_string(),
                source_pressure_type: "local-upgrade-pressure".to_string(),
                current_tier: "T4".to_string(),
                current_score: 29.8,
                target_tier: "T3".to_string(),
                intake_class: "t3-regional-intake".to_string(),
                intake_action: "evaluate-for-t3-zone-treatment".to_string(),
                selection_basis: "score-within-five-points-of-t3-threshold".to_string(),
                source_artifact: "data/tier-table.csv".to_string(),
                next_artifact: "data/t3-t4-pressure-intake.csv".to_string(),
                optimizer_effect: "hold for T3 zone treatment; no national map promotion"
                    .to_string(),
                validation_status: "review".to_string(),
            },
            T3T4PressureIntakeRow {
                route: "I-8".to_string(),
                source_pressure_type: "regional-upgrade-pressure".to_string(),
                current_tier: "T3".to_string(),
                current_score: 45.3,
                target_tier: "T2".to_string(),
                intake_class: "bubble-up-t2-review".to_string(),
                intake_action: "send-to-t2-contact-review".to_string(),
                selection_basis: "score-within-five-points-of-t2-threshold".to_string(),
                source_artifact: "data/tier-table.csv".to_string(),
                next_artifact: "data/tier-contact-witnesses.csv".to_string(),
                optimizer_effect:
                    "lower-tier score pressure can reopen T2 only through contact gates".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t3_zone_route_column_rows(
            &obligations,
            &intake,
            &OptimizerConstraintBudgetIndex::default(),
        );
        let failures = t3_zone_route_column_gate_failures(&rows, &obligations);
        let i25 = rows.iter().find(|row| row.route == "I25").unwrap();
        let i135 = rows.iter().find(|row| row.route == "I-135").unwrap();
        let i8 = rows.iter().find(|row| row.route == "I-8").unwrap();

        assert_eq!(i25.column_decision, "selected");
        assert_eq!(i135.column_decision, "review");
        assert_eq!(i8.column_decision, "upward-review");
        assert!(failures.is_empty());
    }

    #[test]
    fn t4_terminal_access_columns_hold_local_pressure_below_t3() {
        let intake = vec![
            T3T4PressureIntakeRow {
                route: "US90Z".to_string(),
                source_pressure_type: "local-upgrade-pressure".to_string(),
                current_tier: "T4".to_string(),
                current_score: 29.9,
                target_tier: "T3".to_string(),
                intake_class: "t3-regional-intake".to_string(),
                intake_action: "evaluate-for-t3-zone-treatment".to_string(),
                selection_basis: "score-within-five-points-of-t3-threshold".to_string(),
                source_artifact: "data/tier-table.csv".to_string(),
                next_artifact: "data/t3-t4-pressure-intake.csv".to_string(),
                optimizer_effect: "hold for T3 zone treatment; no national map promotion"
                    .to_string(),
                validation_status: "review".to_string(),
            },
            T3T4PressureIntakeRow {
                route: "US999".to_string(),
                source_pressure_type: "local-upgrade-pressure".to_string(),
                current_tier: "T4".to_string(),
                current_score: 25.7,
                target_tier: "T3".to_string(),
                intake_class: "t3-regional-intake".to_string(),
                intake_action: "evaluate-for-t3-zone-treatment".to_string(),
                selection_basis: "score-within-five-points-of-t3-threshold".to_string(),
                source_artifact: "data/tier-table.csv".to_string(),
                next_artifact: "data/t3-t4-pressure-intake.csv".to_string(),
                optimizer_effect: "hold for T3 zone treatment; no national map promotion"
                    .to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows =
            t4_terminal_access_column_rows(&intake, &OptimizerConstraintBudgetIndex::default());
        let failures = t4_terminal_access_column_gate_failures(&rows);
        let us90z = rows.iter().find(|row| row.route == "US90Z").unwrap();
        let us999 = rows.iter().find(|row| row.route == "US999").unwrap();

        assert_eq!(us90z.column_decision, "terminal-review");
        assert_eq!(us90z.zone_id, "t3-southeast");
        assert_eq!(us999.column_decision, "zone-assignment-needed");
        assert!(failures.is_empty());
    }

    #[test]
    fn t3_t4_access_gaps_collect_held_route_and_terminal_rows() {
        let route_rows = vec![T3ZoneRouteColumnRow {
            zone_id: "t3-southeast".to_string(),
            zone_name: "Southeast / Appalachia".to_string(),
            obligation_class: "regional-feeder-access".to_string(),
            route: "US90Z".to_string(),
            current_tier: "T4".to_string(),
            current_score: 29.9,
            constraint_adjusted_score: 29.9,
            hard_blocker_count: 0,
            claim_blocker_count: 0,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 0.0,
            top_constraint_classes: "none".to_string(),
            constraint_ledger_artifact: String::new(),
            promise_horizon_hours: 6,
            column_decision: "review".to_string(),
            zone_role: "below-threshold-feeder-candidate".to_string(),
            contact_requirement: "score-or-terminal-evidence-required".to_string(),
            map_treatment: "show-as-held-zone-candidate".to_string(),
            selection_basis: "candidate is below T3 threshold for a 6h feeder obligation"
                .to_string(),
            source_obligation: "select T3 feeder/contact chain inside the zone".to_string(),
            next_artifact: "data/t3-t4-access-gaps.csv".to_string(),
            optimizer_effect:
                "holds weak feeder pressure for access-gap review instead of selecting it"
                    .to_string(),
            validation_status: "review".to_string(),
        }];
        let terminal_rows = vec![
            T4TerminalAccessColumnRow {
                route: "US90Z".to_string(),
                zone_id: "t3-southeast".to_string(),
                current_score: 29.9,
                constraint_adjusted_score: 29.9,
                hard_blocker_count: 0,
                claim_blocker_count: 0,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 0.0,
                top_constraint_classes: "none".to_string(),
                constraint_ledger_artifact: String::new(),
                access_class: "terminal-upgrade-candidate".to_string(),
                terminal_obligation:
                    "prove one-hour terminal, port, yard, warehouse, or local freight access"
                        .to_string(),
                promise_horizon_hours: 1,
                column_decision: "terminal-review".to_string(),
                evidence_required:
                    "named terminal/local district plus contact to selected T3/T2/T1 column"
                        .to_string(),
                map_treatment: "show-as-local-inset-candidate".to_string(),
                selection_basis:
                    "within five points of T3 threshold but still a T4/local access problem"
                        .to_string(),
                source_artifact: "data/tier-table.csv".to_string(),
                next_artifact: "data/t3-t4-access-gaps.csv".to_string(),
                optimizer_effect:
                    "holds local pressure as a terminal access candidate instead of selecting T3"
                        .to_string(),
                validation_status: "review".to_string(),
            },
            T4TerminalAccessColumnRow {
                route: "US7".to_string(),
                zone_id: "zone-assignment-needed".to_string(),
                current_score: 25.7,
                constraint_adjusted_score: 25.7,
                hard_blocker_count: 0,
                claim_blocker_count: 0,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 0.0,
                top_constraint_classes: "none".to_string(),
                constraint_ledger_artifact: String::new(),
                access_class: "unassigned-local-access".to_string(),
                terminal_obligation: "assign local route to a T3 zone or terminal district"
                    .to_string(),
                promise_horizon_hours: 1,
                column_decision: "zone-assignment-needed".to_string(),
                evidence_required: "zone boundary plus terminal/local freight role".to_string(),
                map_treatment: "hide-until-assigned".to_string(),
                selection_basis: "T4 pressure lacks a deterministic zone assignment".to_string(),
                source_artifact: "data/tier-table.csv".to_string(),
                next_artifact: "data/t3-t4-access-gaps.csv".to_string(),
                optimizer_effect: "blocks promotion and sends the route to access-gap triage"
                    .to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t3_t4_access_gap_rows(&route_rows, &terminal_rows);
        let failures = t3_t4_access_gap_gate_failures(&rows);
        let classes = rows
            .iter()
            .map(|row| row.gap_class.as_str())
            .collect::<std::collections::BTreeSet<_>>();

        assert!(classes.contains("below-threshold-feeder"));
        assert!(classes.contains("terminal-evidence-needed"));
        assert!(classes.contains("zone-assignment-needed"));
        assert!(rows.iter().all(|row| !row.upward_pressure_allowed));
        assert!(rows
            .iter()
            .any(|row| row.gap_class == "terminal-evidence-needed"
                && row.next_artifact == "data/t4-terminal-contact-evidence.csv"));
        assert!(failures.is_empty());
    }

    #[test]
    fn t4_terminal_contact_evidence_queue_keeps_seed_sources_separate() {
        let terminal_rows = vec![T4TerminalAccessColumnRow {
            route: "US90Z".to_string(),
            zone_id: "t3-southeast".to_string(),
            current_score: 29.9,
            constraint_adjusted_score: 29.9,
            hard_blocker_count: 0,
            claim_blocker_count: 0,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 0.0,
            top_constraint_classes: "none".to_string(),
            constraint_ledger_artifact: String::new(),
            access_class: "terminal-upgrade-candidate".to_string(),
            terminal_obligation:
                "prove one-hour access to a Southeast / Appalachia terminal district: Atlanta Hulsey"
                    .to_string(),
            promise_horizon_hours: 1,
            column_decision: "terminal-review".to_string(),
            evidence_required:
                "district seed plus route-to-terminal contact proof from separate source".to_string(),
            map_treatment: "show-as-local-inset-candidate".to_string(),
            selection_basis: "within five points of T3 threshold".to_string(),
            source_artifact: "data/tier-table.csv".to_string(),
            next_artifact: "data/t3-t4-access-gaps.csv".to_string(),
            optimizer_effect: "hold for contact proof".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t4_terminal_contact_evidence_rows(&terminal_rows);
        let failures = t4_terminal_contact_evidence_gate_failures(&rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].terminal_district_seed, "New Orleans Gentilly");
        assert_eq!(
            rows[0].terminal_district_seed_source,
            "data/intermodal_terminals.csv"
        );
        assert_eq!(rows[0].contact_proof_source, "");
        assert_eq!(rows[0].decision, "source-needed");
        assert_eq!(
            rows[0].next_artifact,
            "waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-03.md"
        );
    }

    #[test]
    fn t4_terminal_contact_evidence_classifies_great_lakes_sample_districts() {
        let terminal_rows = vec![
            T4TerminalAccessColumnRow {
                route: "I-294".to_string(),
                zone_id: "t3-great-lakes".to_string(),
                current_score: 26.4,
                constraint_adjusted_score: 25.4,
                hard_blocker_count: 0,
                claim_blocker_count: 1,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 1.0,
                top_constraint_classes: "terminal_access_evidence_gap".to_string(),
                constraint_ledger_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
                access_class: "terminal-upgrade-candidate".to_string(),
                terminal_obligation:
                    "prove one-hour access to a Great Lakes / Ohio Valley terminal district: Chicago Intermodal Complex, Columbus South"
                        .to_string(),
                promise_horizon_hours: 1,
                column_decision: "terminal-review".to_string(),
                evidence_required:
                    "district seed plus route-to-terminal contact proof from separate source"
                        .to_string(),
                map_treatment: "show-as-local-inset-candidate".to_string(),
                selection_basis: "within five points of T3 threshold".to_string(),
                source_artifact: "data/tier-table.csv".to_string(),
                next_artifact: "data/t3-t4-access-gaps.csv".to_string(),
                optimizer_effect: "hold for contact proof".to_string(),
                validation_status: "review".to_string(),
            },
            T4TerminalAccessColumnRow {
                route: "US22".to_string(),
                zone_id: "t3-great-lakes".to_string(),
                current_score: 29.7,
                constraint_adjusted_score: 27.7,
                hard_blocker_count: 0,
                claim_blocker_count: 2,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 2.0,
                top_constraint_classes: "lower_tier_feeder_gap;terminal_access_evidence_gap"
                    .to_string(),
                constraint_ledger_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
                access_class: "terminal-upgrade-candidate".to_string(),
                terminal_obligation:
                    "prove one-hour access to a Great Lakes / Ohio Valley terminal district: Chicago Intermodal Complex, Columbus South"
                        .to_string(),
                promise_horizon_hours: 1,
                column_decision: "terminal-review".to_string(),
                evidence_required:
                    "district seed plus route-to-terminal contact proof from separate source"
                        .to_string(),
                map_treatment: "show-as-local-inset-candidate".to_string(),
                selection_basis: "within five points of T3 threshold".to_string(),
                source_artifact: "data/tier-table.csv".to_string(),
                next_artifact: "data/t3-t4-access-gaps.csv".to_string(),
                optimizer_effect: "hold for contact proof".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t4_terminal_contact_evidence_rows(&terminal_rows);
        let failures = t4_terminal_contact_evidence_gate_failures(&rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert!(rows.iter().all(|row| row.decision == "source-needed"));
        assert!(rows.iter().any(|row| row.route == "I-294"
            && row.terminal_district_seed == "Chicago Intermodal Complex"));
        assert!(rows
            .iter()
            .any(|row| row.route == "US22" && row.terminal_district_seed == "Columbus South"));
        assert!(rows.iter().all(|row| row
            .contact_basis
            .contains("route-to-terminal contact source still needed")));
    }

    #[test]
    fn t4_terminal_contact_evidence_classifies_remaining_zone_districts() {
        let terminal_rows = [
            ("US90Z", "t3-southeast", "New Orleans Gentilly"),
            ("I-840", "t3-mid-south", "Memphis Intermodal"),
            ("I-705", "t3-mountain-west", "Seattle BNSF"),
            ("US96", "t3-texas-border", "Houston Englewood"),
        ]
        .into_iter()
        .map(|(route, zone_id, _)| T4TerminalAccessColumnRow {
            route: route.to_string(),
            zone_id: zone_id.to_string(),
            current_score: 28.0,
            constraint_adjusted_score: 27.0,
            hard_blocker_count: 0,
            claim_blocker_count: 1,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 1.0,
            top_constraint_classes: "terminal_access_evidence_gap".to_string(),
            constraint_ledger_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            access_class: "terminal-upgrade-candidate".to_string(),
            terminal_obligation: "prove one-hour access to a terminal district".to_string(),
            promise_horizon_hours: 1,
            column_decision: "terminal-review".to_string(),
            evidence_required: "district seed plus route-to-terminal contact proof".to_string(),
            map_treatment: "show-as-local-inset-candidate".to_string(),
            selection_basis: "within five points of T3 threshold".to_string(),
            source_artifact: "data/tier-table.csv".to_string(),
            next_artifact: "data/t3-t4-access-gaps.csv".to_string(),
            optimizer_effect: "hold for contact proof".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();

        let rows = t4_terminal_contact_evidence_rows(&terminal_rows);
        let failures = t4_terminal_contact_evidence_gate_failures(&rows);

        assert!(failures.is_empty(), "{failures:?}");
        for (route, _, district) in [
            ("US90Z", "t3-southeast", "New Orleans Gentilly"),
            ("I-840", "t3-mid-south", "Memphis Intermodal"),
            ("I-705", "t3-mountain-west", "Seattle BNSF"),
            ("US96", "t3-texas-border", "Houston Englewood"),
        ] {
            assert!(rows.iter().any(|row| row.route == route
                && row.terminal_district_seed == district
                && row.decision == "source-needed"));
        }
    }

    #[test]
    fn t4_terminal_contact_evidence_gate_separates_source_needed_from_scenario_ready() {
        let source_needed = T4TerminalContactEvidenceRow {
            queue_id: "T4CONTACT-T3GREATLAKES-US10".to_string(),
            route: "US10".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district_seed: "Minneapolis Twin Cities".to_string(),
            terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
            contact_basis: "source-needed-route-to-terminal-contact".to_string(),
            contact_proof_source: String::new(),
            evidence_status: "source-needed".to_string(),
            selected_higher_tier_attachment: "source-needed".to_string(),
            decision: "source-needed".to_string(),
            next_artifact: "waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-02.md"
                .to_string(),
            source_column_artifact: "data/t4-terminal-access-columns.csv".to_string(),
            source_column_decision: "terminal-review".to_string(),
            validation_status: "review".to_string(),
        };
        let scenario_ready = T4TerminalContactEvidenceRow {
            queue_id: "T4CONTACT-T3GREATLAKES-I180".to_string(),
            route: "I-180".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district_seed: "Chicago Intermodal Complex".to_string(),
            terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
            contact_basis: "documented truck connector to terminal gate".to_string(),
            contact_proof_source: "future-contact-source.csv#I-180".to_string(),
            evidence_status: "accepted".to_string(),
            selected_higher_tier_attachment: "T3:t3-great-lakes:I-80".to_string(),
            decision: "scenario-ready".to_string(),
            next_artifact: "data/t4-terminal-scenario-readiness.csv".to_string(),
            source_column_artifact: "data/t4-terminal-access-columns.csv".to_string(),
            source_column_decision: "terminal-review".to_string(),
            validation_status: "pass".to_string(),
        };
        let proximity_only = T4TerminalContactEvidenceRow {
            queue_id: "T4CONTACT-T3GREATLAKES-US22".to_string(),
            route: "US22".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district_seed: "Columbus South".to_string(),
            terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
            contact_basis: "proximity-only terminal district".to_string(),
            contact_proof_source: "future-contact-source.csv#US22".to_string(),
            evidence_status: "accepted".to_string(),
            selected_higher_tier_attachment: "T3:t3-great-lakes:I-70".to_string(),
            decision: "scenario-ready".to_string(),
            next_artifact: "data/t4-terminal-scenario-readiness.csv".to_string(),
            source_column_artifact: "data/t4-terminal-access-columns.csv".to_string(),
            source_column_decision: "terminal-review".to_string(),
            validation_status: "pass".to_string(),
        };

        assert!(
            t4_terminal_contact_evidence_gate_failures(&[source_needed, scenario_ready]).is_empty()
        );
        let proximity_failures = t4_terminal_contact_evidence_gate_failures(&[proximity_only]);
        assert!(
            proximity_failures
                .iter()
                .any(|failure| failure.contains("proximity-only contact cannot be scenario-ready")),
            "{proximity_failures:?}"
        );
    }

    #[test]
    fn t4_terminal_access_evidence_review_preserves_source_needed_blockers() {
        let contact_rows = vec![T4TerminalContactEvidenceRow {
            queue_id: "T4CONTACT-T3GREATLAKES-US10".to_string(),
            route: "US10".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district_seed: "Minneapolis Twin Cities".to_string(),
            terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
            contact_basis: "source-needed-route-to-terminal-contact".to_string(),
            contact_proof_source: String::new(),
            evidence_status: "source-needed".to_string(),
            selected_higher_tier_attachment: "source-needed".to_string(),
            decision: "source-needed".to_string(),
            next_artifact: "waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-02.md"
                .to_string(),
            source_column_artifact: "data/t4-terminal-access-columns.csv".to_string(),
            source_column_decision: "terminal-review".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t4_terminal_access_evidence_review_rows(&contact_rows);
        let failures = t4_terminal_access_evidence_review_gate_failures(&rows, &contact_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].review_decision, "held-source-needed");
        assert_eq!(rows[0].blocker_claims_before, "map;publication;upgrade");
        assert_eq!(rows[0].blocker_claims_after, "map;publication;upgrade");
        assert_eq!(rows[0].claim_blocker_delta, 0);
        assert_eq!(rows[0].validation_status, "review");
    }

    #[test]
    fn t4_terminal_access_proof_acquisition_creates_one_task_per_held_review() {
        let review_rows = vec![T4TerminalAccessEvidenceReviewRow {
            review_id: "T4ACCESSREVIEW-US10".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-US10".to_string(),
            route: "US10".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district_seed: "Minneapolis Twin Cities".to_string(),
            terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
            evidence_status_before: "source-needed".to_string(),
            review_decision: "held-source-needed".to_string(),
            review_reason: "terminal district seed assignment is not contact proof".to_string(),
            source_action: "route-to-terminal-access-proof-acquisition".to_string(),
            blocker_claims_before: "map;publication;upgrade".to_string(),
            blocker_claims_after: "map;publication;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_artifact: "waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-02.md"
                .to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t4_terminal_access_proof_acquisition_rows(&review_rows);
        let failures = t4_terminal_access_proof_acquisition_gate_failures(&rows, &review_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].acquisition_status, "source-needed");
        assert_eq!(rows[0].proof_artifact_status, "not-attached");
        assert_eq!(rows[0].blocker_claims_before, "map;publication;upgrade");
        assert_eq!(rows[0].blocker_claims_after, "map;publication;upgrade");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn t4_terminal_access_proof_artifacts_stay_unattached() {
        let acquisition_rows = vec![T4TerminalAccessProofAcquisitionRow {
            acquisition_id: "T4ACCESSACQ-US10".to_string(),
            review_id: "T4ACCESSREVIEW-US10".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-US10".to_string(),
            route: "US10".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district_seed: "Minneapolis Twin Cities".to_string(),
            required_proof:
                "non-seed route-to-terminal contact source with route, terminal, connector, and date"
                    .to_string(),
            prohibited_seed_source: "data/intermodal_terminals.csv".to_string(),
            acquisition_status: "source-needed".to_string(),
            blocker_claims_before: "map;publication;upgrade".to_string(),
            blocker_claims_after: "map;publication;upgrade".to_string(),
            claim_blocker_delta: 0,
            proof_artifact_status: "not-attached".to_string(),
            next_artifact: "data/t4-terminal-access-proof-artifacts.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t4_terminal_access_proof_artifact_rows(&acquisition_rows);
        let failures = t4_terminal_access_proof_artifact_gate_failures(&rows, &acquisition_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].source_artifact_reference, "source-needed");
        assert_eq!(rows[0].attachment_status, "source-needed");
        assert_eq!(rows[0].evidence_review_status, "not-reviewed");
        assert_eq!(rows[0].proof_acceptance_status, "not-accepted");
        assert_eq!(rows[0].blocker_claims_before, "map;publication;upgrade");
        assert_eq!(rows[0].blocker_claims_after, "map;publication;upgrade");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn t4_terminal_access_proof_review_returns_to_optimizer_hold() {
        let artifact_rows = vec![T4TerminalAccessProofArtifactRow {
            proof_artifact_id: "T4ACCESSARTIFACT-US10".to_string(),
            acquisition_id: "T4ACCESSACQ-US10".to_string(),
            review_id: "T4ACCESSREVIEW-US10".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-US10".to_string(),
            route: "US10".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district_seed: "Minneapolis Twin Cities".to_string(),
            required_proof:
                "non-seed route-to-terminal contact source with route, terminal, connector, and date"
                    .to_string(),
            source_artifact_reference: "source-needed".to_string(),
            attachment_status: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            blocker_claims_before: "map;publication;upgrade".to_string(),
            blocker_claims_after: "map;publication;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_artifact: "data/t4-terminal-access-proof-review.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t4_terminal_access_proof_review_rows(&artifact_rows);
        let failures = t4_terminal_access_proof_review_gate_failures(&rows, &artifact_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].review_decision, "held-no-source-artifact");
        assert_eq!(rows[0].proof_acceptance_status, "not-accepted");
        assert_eq!(
            rows[0].optimization_return_status,
            "return-to-optimizer-held-known"
        );
        assert_eq!(rows[0].next_artifact, "data/tier-optimizer-runs.csv");
        assert_eq!(rows[0].blocker_claims_before, "map;publication;upgrade");
        assert_eq!(rows[0].blocker_claims_after, "map;publication;upgrade");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn t4_terminal_access_source_access_blocks_live_fetch() {
        let review_rows = vec![T4TerminalAccessProofReviewRow {
            proof_review_id: "T4ACCESSREVIEWPROOF-US10".to_string(),
            proof_artifact_id: "T4ACCESSARTIFACT-US10".to_string(),
            acquisition_id: "T4ACCESSACQ-US10".to_string(),
            review_id: "T4ACCESSREVIEW-US10".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-US10".to_string(),
            route: "US10".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            source_artifact_reference: "source-needed".to_string(),
            review_decision: "held-no-source-artifact".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            optimization_return_status: "return-to-optimizer-held-known".to_string(),
            review_reason:
                "proof artifact remains source-needed; terminal-access proof cannot be accepted"
                    .to_string(),
            blocker_claims_before: "map;publication;upgrade".to_string(),
            blocker_claims_after: "map;publication;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_artifact: "data/tier-optimizer-runs.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t4_terminal_access_source_access_rows(&review_rows);
        let failures = t4_terminal_access_source_access_gate_failures(&rows, &review_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].access_mode, "manual-or-cached-source-needed");
        assert_eq!(
            rows[0].live_fetch_status,
            "unsupported-no-safe-terminal-access-fetcher"
        );
        assert_eq!(rows[0].evidence_artifact, "source-needed");
        assert_eq!(rows[0].proof_acceptance_status, "not-accepted");
        assert_eq!(rows[0].blocker_claims_before, "map;publication;upgrade");
        assert_eq!(rows[0].blocker_claims_after, "map;publication;upgrade");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn t4_terminal_access_proof_intake_requires_artifact_without_acceptance() {
        let access_rows = vec![T4TerminalAccessSourceAccessRow {
            source_access_id: "T4ACCESSSOURCE-US10".to_string(),
            proof_review_id: "T4ACCESSREVIEWPROOF-US10".to_string(),
            proof_artifact_id: "T4ACCESSARTIFACT-US10".to_string(),
            acquisition_id: "T4ACCESSACQ-US10".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-US10".to_string(),
            route: "US10".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            source_owner: "terminal operator, port authority, state DOT, or public terminal map"
                .to_string(),
            access_mode: "manual-or-cached-source-needed".to_string(),
            live_fetch_status: "unsupported-no-safe-terminal-access-fetcher".to_string(),
            required_source_metadata:
                "source title; source url or cached artifact; capture date; route; terminal; connector; route-to-terminal contact statement"
                    .to_string(),
            cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                .to_string(),
            source_access_blocker:
                "no safe live terminal-access proof fetch command exists; use manual/cached non-seed proof artifact or add policy-compliant fetcher"
                    .to_string(),
            evidence_artifact: "source-needed".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            blocker_claims_before: "map;publication;upgrade".to_string(),
            blocker_claims_after: "map;publication;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_artifact: "data/t4-terminal-access-proof-artifacts.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t4_terminal_access_proof_intake_rows(&access_rows);
        let failures = t4_terminal_access_proof_intake_gate_failures(&rows, &access_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].proof_artifact, "source-needed");
        assert_eq!(rows[0].proof_status, "source-needed");
        assert_eq!(rows[0].validation_status, "review");
        assert_eq!(rows[0].blocker_claims_before, "map;publication;upgrade");
        assert_eq!(rows[0].blocker_claims_after, "map;publication;upgrade");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn t4_terminal_access_proof_source_capture_stays_source_needed() {
        let intake_rows = vec![T4TerminalAccessProofIntakeRow {
            proof_intake_id: "T4ACCESSINTAKE-US10".to_string(),
            source_access_id: "T4ACCESSSOURCE-US10".to_string(),
            proof_review_id: "T4ACCESSREVIEWPROOF-US10".to_string(),
            proof_artifact_id: "T4ACCESSARTIFACT-US10".to_string(),
            acquisition_id: "T4ACCESSACQ-US10".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-US10".to_string(),
            route: "US10".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            required_artifact_fields:
                "source title; source url or cached artifact; capture date; route; terminal; connector"
                    .to_string(),
            required_contact_statement:
                "non-seed source statement that the route provides route-to-terminal contact"
                    .to_string(),
            proof_artifact: "source-needed".to_string(),
            proof_status: "source-needed".to_string(),
            proof_blocker:
                "manual or cached non-seed terminal-access proof artifact has not been captured or reviewed"
                    .to_string(),
            blocker_claims_before: "map;publication;upgrade".to_string(),
            blocker_claims_after: "map;publication;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_artifact: "data/t4-terminal-access-proof-artifacts.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t4_terminal_access_proof_source_capture_rows(&intake_rows);
        let failures = t4_terminal_access_proof_source_capture_gate_failures(&rows, &intake_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].source_artifact_reference, "source-needed");
        assert_eq!(rows[0].capture_status, "source-needed");
        assert_eq!(rows[0].evidence_acceptance_status, "not-reviewed");
        assert_eq!(rows[0].validation_status, "review");
        assert_eq!(rows[0].blocker_claims_before, "map;publication;upgrade");
        assert_eq!(rows[0].blocker_claims_after, "map;publication;upgrade");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn t4_terminal_access_proof_artifact_attachment_stays_source_needed() {
        let capture_rows = vec![T4TerminalAccessProofSourceCaptureRow {
            source_capture_id: "T4ACCESSCAPTURE-US10".to_string(),
            proof_intake_id: "T4ACCESSINTAKE-US10".to_string(),
            source_access_id: "T4ACCESSSOURCE-US10".to_string(),
            proof_artifact_id: "T4ACCESSARTIFACT-US10".to_string(),
            acquisition_id: "T4ACCESSACQ-US10".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-US10".to_string(),
            route: "US10".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            source_artifact_reference: "source-needed".to_string(),
            source_artifact_type: "manual-or-cached-terminal-access-proof".to_string(),
            capture_status: "source-needed".to_string(),
            evidence_acceptance_status: "not-reviewed".to_string(),
            capture_blocker:
                "manual or cached non-seed terminal-access proof artifact has not been captured"
                    .to_string(),
            blocker_claims_before: "map;publication;upgrade".to_string(),
            blocker_claims_after: "map;publication;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_artifact: "data/t4-terminal-access-proof-artifacts.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t4_terminal_access_proof_artifact_attachment_rows(&capture_rows);
        let failures =
            t4_terminal_access_proof_artifact_attachment_gate_failures(&rows, &capture_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].source_artifact_reference, "source-needed");
        assert_eq!(rows[0].attachment_status, "source-needed");
        assert_eq!(rows[0].evidence_review_status, "not-reviewed");
        assert_eq!(rows[0].proof_acceptance_status, "not-accepted");
        assert_eq!(rows[0].validation_status, "review");
        assert_eq!(rows[0].blocker_claims_before, "map;publication;upgrade");
        assert_eq!(rows[0].blocker_claims_after, "map;publication;upgrade");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn t4_terminal_access_proof_attachment_review_preserves_map_blockers() {
        let attachment_rows = vec![T4TerminalAccessProofArtifactAttachmentRow {
            artifact_attachment_id: "T4ACCESSATTACH-US10".to_string(),
            source_capture_id: "T4ACCESSCAPTURE-US10".to_string(),
            proof_intake_id: "T4ACCESSINTAKE-US10".to_string(),
            proof_artifact_id: "T4ACCESSARTIFACT-US10".to_string(),
            acquisition_id: "T4ACCESSACQ-US10".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-US10".to_string(),
            route: "US10".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            source_artifact_reference: "source-needed".to_string(),
            attachment_status: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            attachment_blocker:
                "manual or cached non-seed terminal-access proof artifact has not been attached"
                    .to_string(),
            blocker_claims_before: "map;publication;upgrade".to_string(),
            blocker_claims_after: "map;publication;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_artifact: "data/t4-terminal-access-proof-attachment-review.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t4_terminal_access_proof_attachment_review_rows(&attachment_rows);
        let failures =
            t4_terminal_access_proof_attachment_review_gate_failures(&rows, &attachment_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].review_decision, "held-no-source-artifact");
        assert_eq!(rows[0].evidence_review_status, "not-reviewed");
        assert_eq!(rows[0].proof_acceptance_status, "not-accepted");
        assert_eq!(
            rows[0].optimization_return_status,
            "return-to-optimizer-held-known"
        );
        assert_eq!(rows[0].blocker_claims_before, "map;publication;upgrade");
        assert_eq!(rows[0].blocker_claims_after, "map;publication;upgrade");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn t4_terminal_access_proof_artifact_acquisition_targets_preserve_map_blockers() {
        let review_rows = vec![T4TerminalAccessProofAttachmentReviewRow {
            attachment_review_id: "T4ACCESSATTACHREVIEW-US10".to_string(),
            artifact_attachment_id: "T4ACCESSATTACH-US10".to_string(),
            source_capture_id: "T4ACCESSCAPTURE-US10".to_string(),
            proof_intake_id: "T4ACCESSINTAKE-US10".to_string(),
            proof_artifact_id: "T4ACCESSARTIFACT-US10".to_string(),
            acquisition_id: "T4ACCESSACQ-US10".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-US10".to_string(),
            route: "US10".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            source_artifact_reference: "source-needed".to_string(),
            review_decision: "held-no-source-artifact".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            optimization_return_status: "return-to-optimizer-held-known".to_string(),
            review_reason:
                "proof artifact attachment remains source-needed; terminal-access proof cannot be accepted"
                    .to_string(),
            blocker_claims_before: "map;publication;upgrade".to_string(),
            blocker_claims_after: "map;publication;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_artifact: "data/optimizer-residual-blocker-backlog.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t4_terminal_access_proof_artifact_acquisition_target_rows(&review_rows);
        let failures =
            t4_terminal_access_proof_artifact_acquisition_target_gate_failures(&rows, &review_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].acquisition_status, "source-needed");
        assert_eq!(rows[0].cache_status, "not-cached");
        assert_eq!(rows[0].source_artifact_reference, "source-needed");
        assert_eq!(rows[0].proof_acceptance_status, "not-accepted");
        assert_eq!(
            rows[0].prohibited_seed_source,
            "data/intermodal_terminals.csv"
        );
        assert_eq!(rows[0].blocker_claims_before, "map;publication;upgrade");
        assert_eq!(rows[0].blocker_claims_after, "map;publication;upgrade");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn t4_terminal_access_proof_artifact_source_access_preserves_map_blockers() {
        let target_rows = vec![T4TerminalAccessProofArtifactAcquisitionTargetRow {
            acquisition_target_id: "T4ACCESSARTIFACTTARGET-US10".to_string(),
            attachment_review_id: "T4ACCESSATTACHREVIEW-US10".to_string(),
            artifact_attachment_id: "T4ACCESSATTACH-US10".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-US10".to_string(),
            route: "US10".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            candidate_source_owner:
                "terminal operator, port authority, state DOT, MPO, or public terminal map"
                    .to_string(),
            required_artifact_fields:
                "source title; source url or cached artifact; capture date; route; terminal; connector; route-to-terminal contact statement"
                    .to_string(),
            prohibited_seed_source: "data/intermodal_terminals.csv".to_string(),
            acquisition_status: "source-needed".to_string(),
            cache_status: "not-cached".to_string(),
            source_artifact_reference: "source-needed".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            blocker_claims_before: "map;publication;upgrade".to_string(),
            blocker_claims_after: "map;publication;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "acquire or cache non-seed route-to-terminal proof artifact".to_string(),
            next_artifact:
                "data/t4-terminal-access-proof-artifact-acquisition-targets.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t4_terminal_access_proof_artifact_source_access_rows(&target_rows);
        let failures =
            t4_terminal_access_proof_artifact_source_access_gate_failures(&rows, &target_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].access_mode, "manual-or-cached-source-needed");
        assert_eq!(rows[0].cache_status, "not-cached");
        assert_eq!(
            rows[0].live_fetch_status,
            "unsupported-no-safe-terminal-access-fetcher"
        );
        assert_eq!(rows[0].evidence_artifact, "source-needed");
        assert_eq!(rows[0].proof_acceptance_status, "not-accepted");
        assert_eq!(rows[0].blocker_claims_before, "map;publication;upgrade");
        assert_eq!(rows[0].blocker_claims_after, "map;publication;upgrade");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn t4_terminal_contact_source_plan_covers_all_source_needed_rows() {
        let contact_rows = vec![
            T4TerminalContactEvidenceRow {
                queue_id: "T4CONTACT-T3GREATLAKES-I294".to_string(),
                route: "I-294".to_string(),
                zone_id: "t3-great-lakes".to_string(),
                terminal_district_seed: "Chicago Intermodal Complex".to_string(),
                terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
                contact_basis:
                    "candidate-terminal-district-assigned; route-to-terminal contact source still needed"
                        .to_string(),
                contact_proof_source: String::new(),
                evidence_status: "source-needed".to_string(),
                selected_higher_tier_attachment: "source-needed".to_string(),
                decision: "source-needed".to_string(),
                next_artifact:
                    "waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-02.md".to_string(),
                source_column_artifact: "data/t4-terminal-access-columns.csv".to_string(),
                source_column_decision: "terminal-review".to_string(),
                validation_status: "review".to_string(),
            },
            T4TerminalContactEvidenceRow {
                queue_id: "T4CONTACT-T3GREATLAKES-US22".to_string(),
                route: "US22".to_string(),
                zone_id: "t3-great-lakes".to_string(),
                terminal_district_seed: "Columbus South".to_string(),
                terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
                contact_basis:
                    "candidate-terminal-district-assigned; route-to-terminal contact source still needed"
                        .to_string(),
                contact_proof_source: String::new(),
                evidence_status: "source-needed".to_string(),
                selected_higher_tier_attachment: "source-needed".to_string(),
                decision: "source-needed".to_string(),
                next_artifact:
                    "waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-02.md".to_string(),
                source_column_artifact: "data/t4-terminal-access-columns.csv".to_string(),
                source_column_decision: "terminal-review".to_string(),
                validation_status: "review".to_string(),
            },
            T4TerminalContactEvidenceRow {
                queue_id: "T4CONTACT-T3SOUTHEAST-US90Z".to_string(),
                route: "US90Z".to_string(),
                zone_id: "t3-southeast".to_string(),
                terminal_district_seed: "New Orleans Gentilly".to_string(),
                terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
                contact_basis:
                    "candidate-terminal-district-assigned; route-to-terminal contact source still needed"
                        .to_string(),
                contact_proof_source: String::new(),
                evidence_status: "source-needed".to_string(),
                selected_higher_tier_attachment: "source-needed".to_string(),
                decision: "source-needed".to_string(),
                next_artifact:
                    "waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-03.md".to_string(),
                source_column_artifact: "data/t4-terminal-access-columns.csv".to_string(),
                source_column_decision: "terminal-review".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t4_terminal_contact_source_plan_rows(&contact_rows);
        let failures = t4_terminal_contact_source_plan_gate_failures(&rows, &contact_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 3);
        assert!(rows
            .iter()
            .all(|row| row.contact_proof_source_artifact == "source-needed"));
        assert!(rows.iter().all(|row| row
            .required_proof_fields
            .contains("route-to-terminal contact statement")));
        assert!(rows.iter().any(
            |row| row.route == "I-294" && row.terminal_district == "Chicago Intermodal Complex"
        ));
        assert!(rows
            .iter()
            .any(|row| row.route == "US22" && row.terminal_district == "Columbus South"));
        assert!(rows
            .iter()
            .any(|row| row.route == "US90Z" && row.terminal_district == "New Orleans Gentilly"));
    }

    #[test]
    fn t4_terminal_contact_proof_artifact_contract_requires_non_seed_proof() {
        let rows = t4_terminal_contact_proof_artifact_contract_rows();
        let failures = t4_terminal_contact_proof_artifact_contract_gate_failures(&rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].accepted_proof_status, "source-backed");
        assert!(rows[0]
            .required_fields
            .contains("route-to-terminal contact statement"));
        assert!(rows[0]
            .prohibited_sources
            .contains("data/intermodal_terminals.csv"));
        assert!(rows[0].promotion_rule.contains("non-seed source artifact"));
    }

    #[test]
    fn t4_terminal_contact_proof_artifact_contract_rejects_seed_laundering() {
        let bad_row = T4TerminalContactProofArtifactContractRow {
            contract_id: "T4CONTACT-PROOF-CONTRACT-BAD".to_string(),
            source_family: "public-terminal-contact-proof".to_string(),
            accepted_proof_status: "source-backed".to_string(),
            required_fields:
                "route; terminal district; route-to-terminal contact statement; source title; source url or cached artifact; capture date; selected higher-tier attachment; validation decision"
                    .to_string(),
            allowed_artifact_modes: "manual-citation;cached-source-artifact".to_string(),
            prohibited_sources: "route proximity".to_string(),
            promotion_rule:
                "source-backed requires a route terminal district contact statement"
                    .to_string(),
            source_needed_decision:
                "missing proof artifact remains source-needed and review".to_string(),
            blocked_decision: "inaccessible source remains blocked".to_string(),
            rejected_decision: "bad artifact remains rejected".to_string(),
            next_artifact:
                "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-02.md"
                    .to_string(),
            validation_status: "pass".to_string(),
        };

        let failures = t4_terminal_contact_proof_artifact_contract_gate_failures(&[bad_row]);

        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("data/intermodal_terminals.csv")),
            "{failures:?}"
        );
        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("non-seed route contact proof")),
            "{failures:?}"
        );
    }

    #[test]
    fn t4_terminal_contact_proof_source_registry_preserves_source_needed_rows() {
        let proof_rows = vec![
            T4TerminalContactProofDocketRow {
                task_id: "T4PROOF-I294".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-I294".to_string(),
                route: "I-294".to_string(),
                zone_id: "t3-great-lakes".to_string(),
                terminal_district: "Chicago Intermodal Complex".to_string(),
                source_family: "public-terminal-contact-proof".to_string(),
                required_proof_field: "route-to-terminal contact statement".to_string(),
                selected_higher_tier_attachment_requirement:
                    "must name selected higher-tier attachment or remain source-needed".to_string(),
                contact_proof_source_artifact: "source-needed".to_string(),
                proof_status: "source-needed".to_string(),
                proof_blocker:
                    "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                        .to_string(),
                scenario_effect:
                    "no scenario-readiness until contact proof source and attachment are accepted"
                        .to_string(),
                next_artifact:
                    "waves/2026-05-13-great-lakes-terminal-contact-sources/plans/pulse-04.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
            T4TerminalContactProofDocketRow {
                task_id: "T4PROOF-US22".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-US22".to_string(),
                route: "US22".to_string(),
                zone_id: "t3-great-lakes".to_string(),
                terminal_district: "Columbus South".to_string(),
                source_family: "public-terminal-contact-proof".to_string(),
                required_proof_field: "route-to-terminal contact statement".to_string(),
                selected_higher_tier_attachment_requirement:
                    "must name selected higher-tier attachment or remain source-needed".to_string(),
                contact_proof_source_artifact: "source-needed".to_string(),
                proof_status: "source-needed".to_string(),
                proof_blocker:
                    "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                        .to_string(),
                scenario_effect:
                    "no scenario-readiness until contact proof source and attachment are accepted"
                        .to_string(),
                next_artifact:
                    "waves/2026-05-13-great-lakes-terminal-contact-sources/plans/pulse-04.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t4_terminal_contact_proof_source_registry_rows(&proof_rows, &[]);
        let failures = t4_terminal_contact_proof_source_registry_gate_failures(&rows, &proof_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.registry_status == "source-needed"
            && row.proof_source_artifact == "source-needed"
            && row.validation_status == "review"));
    }

    #[test]
    fn t4_terminal_contact_proof_source_registry_rejects_seed_proof() {
        let bad_row = T4TerminalContactProofSourceRegistryRow {
            registry_id: "T4CONTACTREGISTRY-BAD".to_string(),
            task_id: "T4PROOF-I294".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-I294".to_string(),
            route: "I-294".to_string(),
            terminal_district: "Chicago Intermodal Complex".to_string(),
            source_family: "public-terminal-contact-proof".to_string(),
            source_artifact_mode: "manual-citation".to_string(),
            source_title: "intermodal terminal seed".to_string(),
            source_url_or_cache_artifact: "data/intermodal_terminals.csv".to_string(),
            capture_date: "2026-05-13".to_string(),
            contact_statement_status: "source-backed".to_string(),
            selected_higher_tier_attachment_status: "attached".to_string(),
            registry_status: "source-backed".to_string(),
            proof_source_artifact: "data/intermodal_terminals.csv".to_string(),
            registry_blocker: "none".to_string(),
            contract_artifact: "data/t4-terminal-contact-proof-artifact-contract.csv".to_string(),
            next_artifact:
                "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-03.md"
                    .to_string(),
            validation_status: "pass".to_string(),
        };
        let proof_row = T4TerminalContactProofDocketRow {
            task_id: "T4PROOF-I294".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-I294".to_string(),
            route: "I-294".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district: "Chicago Intermodal Complex".to_string(),
            source_family: "public-terminal-contact-proof".to_string(),
            required_proof_field: "route-to-terminal contact statement".to_string(),
            selected_higher_tier_attachment_requirement:
                "must name selected higher-tier attachment or remain source-needed".to_string(),
            contact_proof_source_artifact: "source-needed".to_string(),
            proof_status: "source-needed".to_string(),
            proof_blocker:
                "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                    .to_string(),
            scenario_effect:
                "no scenario-readiness until contact proof source and attachment are accepted"
                    .to_string(),
            next_artifact:
                "waves/2026-05-13-great-lakes-terminal-contact-sources/plans/pulse-04.md"
                    .to_string(),
            validation_status: "review".to_string(),
        };

        let failures =
            t4_terminal_contact_proof_source_registry_gate_failures(&[bad_row], &[proof_row]);

        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("cites terminal seed data as proof")),
            "{failures:?}"
        );
    }

    #[test]
    fn t4_terminal_contact_district_proof_import_selects_largest_unresolved_district() {
        let registry_rows = vec![
            T4TerminalContactProofSourceRegistryRow {
                registry_id: "REG-CHI-I294".to_string(),
                task_id: "T4PROOF-I294".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-I294".to_string(),
                route: "I-294".to_string(),
                terminal_district: "Chicago Intermodal Complex".to_string(),
                source_family: "public-terminal-contact-proof".to_string(),
                source_artifact_mode: "source-needed".to_string(),
                source_title: "source-needed".to_string(),
                source_url_or_cache_artifact: "source-needed".to_string(),
                capture_date: "source-needed".to_string(),
                contact_statement_status: "source-needed".to_string(),
                selected_higher_tier_attachment_status: "source-needed".to_string(),
                registry_status: "source-needed".to_string(),
                proof_source_artifact: "source-needed".to_string(),
                registry_blocker: "manual citation or cached source artifact not registered"
                    .to_string(),
                contract_artifact: "data/t4-terminal-contact-proof-artifact-contract.csv"
                    .to_string(),
                next_artifact:
                    "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-03.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
            T4TerminalContactProofSourceRegistryRow {
                registry_id: "REG-COL-US22".to_string(),
                task_id: "T4PROOF-US22".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-US22".to_string(),
                route: "US22".to_string(),
                terminal_district: "Columbus South".to_string(),
                source_family: "public-terminal-contact-proof".to_string(),
                source_artifact_mode: "source-needed".to_string(),
                source_title: "source-needed".to_string(),
                source_url_or_cache_artifact: "source-needed".to_string(),
                capture_date: "source-needed".to_string(),
                contact_statement_status: "source-needed".to_string(),
                selected_higher_tier_attachment_status: "source-needed".to_string(),
                registry_status: "source-needed".to_string(),
                proof_source_artifact: "source-needed".to_string(),
                registry_blocker: "manual citation or cached source artifact not registered"
                    .to_string(),
                contract_artifact: "data/t4-terminal-contact-proof-artifact-contract.csv"
                    .to_string(),
                next_artifact:
                    "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-03.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
            T4TerminalContactProofSourceRegistryRow {
                registry_id: "REG-COL-US35".to_string(),
                task_id: "T4PROOF-US35".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-US35".to_string(),
                route: "US35".to_string(),
                terminal_district: "Columbus South".to_string(),
                source_family: "public-terminal-contact-proof".to_string(),
                source_artifact_mode: "source-needed".to_string(),
                source_title: "source-needed".to_string(),
                source_url_or_cache_artifact: "source-needed".to_string(),
                capture_date: "source-needed".to_string(),
                contact_statement_status: "source-needed".to_string(),
                selected_higher_tier_attachment_status: "source-needed".to_string(),
                registry_status: "source-needed".to_string(),
                proof_source_artifact: "source-needed".to_string(),
                registry_blocker: "manual citation or cached source artifact not registered"
                    .to_string(),
                contract_artifact: "data/t4-terminal-contact-proof-artifact-contract.csv"
                    .to_string(),
                next_artifact:
                    "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-03.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t4_terminal_contact_district_proof_import_rows(&registry_rows);
        let failures =
            t4_terminal_contact_district_proof_import_gate_failures(&rows, &registry_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert!(rows
            .iter()
            .all(|row| row.terminal_district == "Columbus South"
                && row.import_status == "source-needed"
                && row.proof_decision == "source-needed"));
    }

    #[test]
    fn t4_terminal_contact_district_proof_import_rejects_accepted_seed_artifact() {
        let bad_row = T4TerminalContactDistrictProofImportRow {
            import_id: "IMPORT-BAD".to_string(),
            registry_id: "REG-CHI-I294".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-I294".to_string(),
            route: "I-294".to_string(),
            terminal_district: "Chicago Intermodal Complex".to_string(),
            source_artifact_mode: "manual-citation".to_string(),
            proof_source_artifact: "data/intermodal_terminals.csv".to_string(),
            contact_statement_status: "source-backed".to_string(),
            selected_higher_tier_attachment_status: "attached".to_string(),
            import_status: "accepted".to_string(),
            proof_decision: "source-backed".to_string(),
            import_blocker: "none".to_string(),
            selection_rule: "largest unresolved terminal district in proof source registry"
                .to_string(),
            next_artifact:
                "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-04.md"
                    .to_string(),
            validation_status: "pass".to_string(),
        };
        let registry_row = T4TerminalContactProofSourceRegistryRow {
            registry_id: "REG-CHI-I294".to_string(),
            task_id: "T4PROOF-I294".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-I294".to_string(),
            route: "I-294".to_string(),
            terminal_district: "Chicago Intermodal Complex".to_string(),
            source_family: "public-terminal-contact-proof".to_string(),
            source_artifact_mode: "source-needed".to_string(),
            source_title: "source-needed".to_string(),
            source_url_or_cache_artifact: "source-needed".to_string(),
            capture_date: "source-needed".to_string(),
            contact_statement_status: "source-needed".to_string(),
            selected_higher_tier_attachment_status: "source-needed".to_string(),
            registry_status: "source-needed".to_string(),
            proof_source_artifact: "source-needed".to_string(),
            registry_blocker: "manual citation or cached source artifact not registered"
                .to_string(),
            contract_artifact: "data/t4-terminal-contact-proof-artifact-contract.csv".to_string(),
            next_artifact:
                "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-03.md"
                    .to_string(),
            validation_status: "review".to_string(),
        };

        let failures =
            t4_terminal_contact_district_proof_import_gate_failures(&[bad_row], &[registry_row]);

        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("non-seed source-backed proof")),
            "{failures:?}"
        );
    }

    #[test]
    fn t4_terminal_contact_source_plan_rejects_seed_source_as_contact_proof() {
        let bad_row = T4TerminalContactSourcePlanRow {
            plan_id: "T4SOURCEPLAN-BAD".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-I294".to_string(),
            route: "I-294".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district: "Chicago Intermodal Complex".to_string(),
            terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
            contact_proof_source_family: "public-terminal-contact-proof".to_string(),
            contact_proof_source_artifact: "data/intermodal_terminals.csv".to_string(),
            required_proof_fields:
                "route; terminal district; route-to-terminal contact statement; source title"
                    .to_string(),
            acquisition_status: "source-needed".to_string(),
            proof_blocker:
                "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                    .to_string(),
            next_artifact:
                "waves/2026-05-13-great-lakes-terminal-contact-sources/plans/pulse-02.md"
                    .to_string(),
            validation_status: "review".to_string(),
        };
        let contact_row = T4TerminalContactEvidenceRow {
            queue_id: "T4CONTACT-T3GREATLAKES-I294".to_string(),
            route: "I-294".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district_seed: "Chicago Intermodal Complex".to_string(),
            terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
            contact_basis:
                "candidate-terminal-district-assigned; route-to-terminal contact source still needed"
                    .to_string(),
            contact_proof_source: String::new(),
            evidence_status: "source-needed".to_string(),
            selected_higher_tier_attachment: "source-needed".to_string(),
            decision: "source-needed".to_string(),
            next_artifact:
                "waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-02.md".to_string(),
            source_column_artifact: "data/t4-terminal-access-columns.csv".to_string(),
            source_column_decision: "terminal-review".to_string(),
            validation_status: "review".to_string(),
        };

        let failures = t4_terminal_contact_source_plan_gate_failures(&[bad_row], &[contact_row]);

        assert!(
            failures
                .iter()
                .any(|failure| failure
                    .contains("uses terminal district seed source as contact proof")),
            "{failures:?}"
        );
    }

    #[test]
    fn t4_terminal_contact_source_catalog_summarizes_district_families() {
        let plan_rows = vec![
            T4TerminalContactSourcePlanRow {
                plan_id: "T4SOURCEPLAN-A".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-I294".to_string(),
                route: "I-294".to_string(),
                zone_id: "t3-great-lakes".to_string(),
                terminal_district: "Chicago Intermodal Complex".to_string(),
                terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
                contact_proof_source_family: "public-terminal-contact-proof".to_string(),
                contact_proof_source_artifact: "source-needed".to_string(),
                required_proof_fields:
                    "route; terminal district; route-to-terminal contact statement; source title"
                        .to_string(),
                acquisition_status: "source-needed".to_string(),
                proof_blocker:
                    "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                        .to_string(),
                next_artifact:
                    "waves/2026-05-13-great-lakes-terminal-contact-sources/plans/pulse-02.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
            T4TerminalContactSourcePlanRow {
                plan_id: "T4SOURCEPLAN-B".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-US41".to_string(),
                route: "US41".to_string(),
                zone_id: "t3-great-lakes".to_string(),
                terminal_district: "Chicago Intermodal Complex".to_string(),
                terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
                contact_proof_source_family: "public-terminal-contact-proof".to_string(),
                contact_proof_source_artifact: "source-needed".to_string(),
                required_proof_fields:
                    "route; terminal district; route-to-terminal contact statement; source title"
                        .to_string(),
                acquisition_status: "source-needed".to_string(),
                proof_blocker:
                    "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                        .to_string(),
                next_artifact:
                    "waves/2026-05-13-great-lakes-terminal-contact-sources/plans/pulse-02.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
            T4TerminalContactSourcePlanRow {
                plan_id: "T4SOURCEPLAN-C".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-US22".to_string(),
                route: "US22".to_string(),
                zone_id: "t3-great-lakes".to_string(),
                terminal_district: "Columbus South".to_string(),
                terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
                contact_proof_source_family: "public-terminal-contact-proof".to_string(),
                contact_proof_source_artifact: "source-needed".to_string(),
                required_proof_fields:
                    "route; terminal district; route-to-terminal contact statement; source title"
                        .to_string(),
                acquisition_status: "source-needed".to_string(),
                proof_blocker:
                    "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                        .to_string(),
                next_artifact:
                    "waves/2026-05-13-great-lakes-terminal-contact-sources/plans/pulse-02.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let catalog_rows = t4_terminal_contact_source_catalog_rows(&plan_rows);
        let failures = t4_terminal_contact_source_catalog_gate_failures(&catalog_rows, &plan_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(catalog_rows.len(), 2);
        assert!(catalog_rows
            .iter()
            .all(|row| row.source_family == "public-terminal-contact-proof"));
        assert!(catalog_rows
            .iter()
            .all(|row| row.proof_blocker.contains("no safe live fetcher")));
        assert!(catalog_rows
            .iter()
            .any(|row| row.terminal_district == "Chicago Intermodal Complex"
                && row.route_task_count == 2));
        assert!(catalog_rows
            .iter()
            .any(|row| row.terminal_district == "Columbus South" && row.route_task_count == 1));
    }

    #[test]
    fn t4_terminal_contact_source_catalog_rejects_unowned_districts() {
        let bad_row = T4TerminalContactSourceCatalogRow {
            catalog_id: "T4SOURCECATALOG-BAD".to_string(),
            terminal_district: "Unowned Terminal".to_string(),
            route_task_count: 1,
            source_family: "public-terminal-contact-proof".to_string(),
            source_access_mode: "manual-or-cached-source-needed".to_string(),
            required_proof_fields:
                "route; terminal district; route-to-terminal contact statement; source title"
                    .to_string(),
            acquisition_status: "source-needed".to_string(),
            proof_blocker:
                "no safe live fetcher or cached contact proof source is registered for this district"
                    .to_string(),
            cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                .to_string(),
            next_artifact:
                "waves/2026-05-13-great-lakes-terminal-contact-sources/plans/pulse-03.md"
                    .to_string(),
            validation_status: "review".to_string(),
        };

        let failures = t4_terminal_contact_source_catalog_gate_failures(&[bad_row], &[]);

        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("does not appear in the route source plan")),
            "{failures:?}"
        );
    }

    #[test]
    fn t4_terminal_contact_proof_docket_keeps_one_source_needed_task_per_route() {
        let plan_rows = vec![
            T4TerminalContactSourcePlanRow {
                plan_id: "T4SOURCEPLAN-A".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-I294".to_string(),
                route: "I-294".to_string(),
                zone_id: "t3-great-lakes".to_string(),
                terminal_district: "Chicago Intermodal Complex".to_string(),
                terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
                contact_proof_source_family: "public-terminal-contact-proof".to_string(),
                contact_proof_source_artifact: "source-needed".to_string(),
                required_proof_fields:
                    "route; terminal district; route-to-terminal contact statement; source title"
                        .to_string(),
                acquisition_status: "source-needed".to_string(),
                proof_blocker:
                    "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                        .to_string(),
                next_artifact:
                    "waves/2026-05-13-great-lakes-terminal-contact-sources/plans/pulse-02.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
            T4TerminalContactSourcePlanRow {
                plan_id: "T4SOURCEPLAN-B".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-US22".to_string(),
                route: "US22".to_string(),
                zone_id: "t3-great-lakes".to_string(),
                terminal_district: "Columbus South".to_string(),
                terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
                contact_proof_source_family: "public-terminal-contact-proof".to_string(),
                contact_proof_source_artifact: "source-needed".to_string(),
                required_proof_fields:
                    "route; terminal district; route-to-terminal contact statement; source title"
                        .to_string(),
                acquisition_status: "source-needed".to_string(),
                proof_blocker:
                    "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                        .to_string(),
                next_artifact:
                    "waves/2026-05-13-great-lakes-terminal-contact-sources/plans/pulse-02.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let catalog_rows = t4_terminal_contact_source_catalog_rows(&plan_rows);

        let proof_rows = t4_terminal_contact_proof_docket_rows(&plan_rows, &catalog_rows);
        let failures =
            t4_terminal_contact_proof_docket_gate_failures(&proof_rows, &plan_rows, &catalog_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(proof_rows.len(), plan_rows.len());
        assert!(proof_rows
            .iter()
            .all(|row| row.proof_status == "source-needed"
                && row.contact_proof_source_artifact == "source-needed"));
        assert!(proof_rows.iter().all(|row| row
            .selected_higher_tier_attachment_requirement
            .contains("selected higher-tier attachment")));
        assert!(proof_rows
            .iter()
            .all(|row| row.scenario_effect.contains("no scenario-readiness")));
    }

    #[test]
    fn t4_terminal_contact_proof_docket_rejects_source_backed_without_artifact() {
        let bad_row = T4TerminalContactProofDocketRow {
            task_id: "T4PROOF-BAD".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-I294".to_string(),
            route: "I-294".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district: "Chicago Intermodal Complex".to_string(),
            source_family: "public-terminal-contact-proof".to_string(),
            required_proof_field: "route-to-terminal contact statement".to_string(),
            selected_higher_tier_attachment_requirement:
                "must name selected higher-tier attachment or remain source-needed".to_string(),
            contact_proof_source_artifact: "source-needed".to_string(),
            proof_status: "source-backed".to_string(),
            proof_blocker:
                "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                    .to_string(),
            scenario_effect:
                "no scenario-readiness until contact proof source and attachment are accepted"
                    .to_string(),
            next_artifact:
                "waves/2026-05-13-great-lakes-terminal-contact-sources/plans/pulse-04.md"
                    .to_string(),
            validation_status: "review".to_string(),
        };
        let plan_row = T4TerminalContactSourcePlanRow {
            plan_id: "T4SOURCEPLAN-A".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-I294".to_string(),
            route: "I-294".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district: "Chicago Intermodal Complex".to_string(),
            terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
            contact_proof_source_family: "public-terminal-contact-proof".to_string(),
            contact_proof_source_artifact: "source-needed".to_string(),
            required_proof_fields:
                "route; terminal district; route-to-terminal contact statement; source title"
                    .to_string(),
            acquisition_status: "source-needed".to_string(),
            proof_blocker:
                "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                    .to_string(),
            next_artifact:
                "waves/2026-05-13-great-lakes-terminal-contact-sources/plans/pulse-02.md"
                    .to_string(),
            validation_status: "review".to_string(),
        };
        let catalog_row = T4TerminalContactSourceCatalogRow {
            catalog_id: "T4SOURCECATALOG-CHICAGOINTERMODALCOMPLEX".to_string(),
            terminal_district: "Chicago Intermodal Complex".to_string(),
            route_task_count: 1,
            source_family: "public-terminal-contact-proof".to_string(),
            source_access_mode: "manual-or-cached-source-needed".to_string(),
            required_proof_fields:
                "route; terminal district; route-to-terminal contact statement; source title"
                    .to_string(),
            acquisition_status: "source-needed".to_string(),
            proof_blocker:
                "no safe live fetcher or cached contact proof source is registered for this district"
                    .to_string(),
            cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                .to_string(),
            next_artifact:
                "waves/2026-05-13-great-lakes-terminal-contact-sources/plans/pulse-03.md"
                    .to_string(),
            validation_status: "review".to_string(),
        };

        let failures =
            t4_terminal_contact_proof_docket_gate_failures(&[bad_row], &[plan_row], &[catalog_row]);

        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("source-backed proof task lacks proof artifact")),
            "{failures:?}"
        );
    }

    #[test]
    fn t4_terminal_columbus_proof_intake_filters_exact_source_needed_slice() {
        let proof_rows = vec![
            T4TerminalContactProofDocketRow {
                task_id: "T4PROOF-I271".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-I271".to_string(),
                route: "I-271".to_string(),
                zone_id: "t3-great-lakes".to_string(),
                terminal_district: "Columbus South".to_string(),
                source_family: "public-terminal-contact-proof".to_string(),
                required_proof_field: "route-to-terminal contact statement".to_string(),
                selected_higher_tier_attachment_requirement:
                    "must name selected higher-tier attachment or remain source-needed".to_string(),
                contact_proof_source_artifact: "source-needed".to_string(),
                proof_status: "source-needed".to_string(),
                proof_blocker:
                    "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                        .to_string(),
                scenario_effect:
                    "no scenario-readiness until contact proof source and attachment are accepted"
                        .to_string(),
                next_artifact:
                    "waves/2026-05-13-great-lakes-terminal-contact-sources/plans/pulse-04.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
            T4TerminalContactProofDocketRow {
                task_id: "T4PROOF-US22".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-US22".to_string(),
                route: "US22".to_string(),
                zone_id: "t3-great-lakes".to_string(),
                terminal_district: "Columbus South".to_string(),
                source_family: "public-terminal-contact-proof".to_string(),
                required_proof_field: "route-to-terminal contact statement".to_string(),
                selected_higher_tier_attachment_requirement:
                    "must name selected higher-tier attachment or remain source-needed".to_string(),
                contact_proof_source_artifact: "source-needed".to_string(),
                proof_status: "source-needed".to_string(),
                proof_blocker:
                    "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                        .to_string(),
                scenario_effect:
                    "no scenario-readiness until contact proof source and attachment are accepted"
                        .to_string(),
                next_artifact:
                    "waves/2026-05-13-great-lakes-terminal-contact-sources/plans/pulse-04.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
            T4TerminalContactProofDocketRow {
                task_id: "T4PROOF-I294".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-I294".to_string(),
                route: "I-294".to_string(),
                zone_id: "t3-great-lakes".to_string(),
                terminal_district: "Chicago Intermodal Complex".to_string(),
                source_family: "public-terminal-contact-proof".to_string(),
                required_proof_field: "route-to-terminal contact statement".to_string(),
                selected_higher_tier_attachment_requirement:
                    "must name selected higher-tier attachment or remain source-needed".to_string(),
                contact_proof_source_artifact: "source-needed".to_string(),
                proof_status: "source-needed".to_string(),
                proof_blocker:
                    "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                        .to_string(),
                scenario_effect:
                    "no scenario-readiness until contact proof source and attachment are accepted"
                        .to_string(),
                next_artifact:
                    "waves/2026-05-13-great-lakes-terminal-contact-sources/plans/pulse-04.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t4_terminal_columbus_proof_intake_rows(&proof_rows);
        let failures = t4_terminal_columbus_proof_intake_gate_failures(&rows, &proof_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert!(rows
            .iter()
            .all(|row| row.terminal_district == "Columbus South"
                && row.proof_status == "source-needed"
                && row.validation_status == "review"));
        assert!(rows.iter().any(|row| row.route == "I-271"));
        assert!(rows.iter().any(|row| row.route == "US22"));
    }

    #[test]
    fn t4_terminal_columbus_proof_intake_rejects_non_columbus_rows() {
        let bad_row = T4TerminalColumbusProofIntakeRow {
            intake_id: "T4COLUMBUS-BAD".to_string(),
            task_id: "T4PROOF-I294".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-I294".to_string(),
            route: "I-294".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district: "Chicago Intermodal Complex".to_string(),
            source_family: "public-terminal-contact-proof".to_string(),
            required_proof_field: "route-to-terminal contact statement".to_string(),
            selected_higher_tier_attachment_requirement:
                "must name selected higher-tier attachment or remain source-needed".to_string(),
            contact_proof_source_artifact: "source-needed".to_string(),
            proof_status: "source-needed".to_string(),
            proof_blocker:
                "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                    .to_string(),
            next_artifact:
                "waves/2026-05-13-columbus-south-terminal-contact-proof/plans/pulse-02.md"
                    .to_string(),
            validation_status: "review".to_string(),
        };

        let failures = t4_terminal_columbus_proof_intake_gate_failures(&[bad_row], &[]);

        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("not a Columbus South proof task")),
            "{failures:?}"
        );
    }

    #[test]
    fn t4_terminal_columbus_source_access_records_policy_blockers() {
        let intake_rows = vec![
            T4TerminalColumbusProofIntakeRow {
                intake_id: "T4COLUMBUS-I271".to_string(),
                task_id: "T4PROOF-I271".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-I271".to_string(),
                route: "I-271".to_string(),
                zone_id: "t3-great-lakes".to_string(),
                terminal_district: "Columbus South".to_string(),
                source_family: "public-terminal-contact-proof".to_string(),
                required_proof_field: "route-to-terminal contact statement".to_string(),
                selected_higher_tier_attachment_requirement:
                    "must name selected higher-tier attachment or remain source-needed".to_string(),
                contact_proof_source_artifact: "source-needed".to_string(),
                proof_status: "source-needed".to_string(),
                proof_blocker:
                    "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                        .to_string(),
                next_artifact:
                    "waves/2026-05-13-columbus-south-terminal-contact-proof/plans/pulse-02.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
            T4TerminalColumbusProofIntakeRow {
                intake_id: "T4COLUMBUS-US22".to_string(),
                task_id: "T4PROOF-US22".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-US22".to_string(),
                route: "US22".to_string(),
                zone_id: "t3-great-lakes".to_string(),
                terminal_district: "Columbus South".to_string(),
                source_family: "public-terminal-contact-proof".to_string(),
                required_proof_field: "route-to-terminal contact statement".to_string(),
                selected_higher_tier_attachment_requirement:
                    "must name selected higher-tier attachment or remain source-needed".to_string(),
                contact_proof_source_artifact: "source-needed".to_string(),
                proof_status: "source-needed".to_string(),
                proof_blocker:
                    "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                        .to_string(),
                next_artifact:
                    "waves/2026-05-13-columbus-south-terminal-contact-proof/plans/pulse-02.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t4_terminal_columbus_source_access_rows(&intake_rows);
        let failures = t4_terminal_columbus_source_access_gate_failures(&rows, &intake_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert!(rows
            .iter()
            .all(|row| row.live_fetch_status == "unsupported-no-safe-terminal-fetcher"));
        assert!(rows.iter().all(|row| row
            .required_source_metadata
            .contains("route-to-terminal contact statement")));
        assert!(rows.iter().all(|row| row
            .source_access_blocker
            .contains("no safe live terminal-contact fetch command")));
    }

    #[test]
    fn t4_terminal_columbus_source_access_rejects_live_fetch_enablement() {
        let bad_row = T4TerminalColumbusSourceAccessRow {
            access_id: "T4COLUMBUSACCESS-BAD".to_string(),
            intake_id: "T4COLUMBUS-I271".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-I271".to_string(),
            route: "I-271".to_string(),
            terminal_district: "Columbus South".to_string(),
            source_family: "public-terminal-contact-proof".to_string(),
            access_mode: "live-fetch".to_string(),
            live_fetch_status: "enabled".to_string(),
            required_source_metadata:
                "source title; source url or cached artifact; capture date; route; terminal district; route-to-terminal contact statement"
                    .to_string(),
            contact_proof_source_artifact: "source-needed".to_string(),
            acquisition_status: "source-needed".to_string(),
            source_access_blocker:
                "no safe live terminal-contact fetch command exists; use manual/cached proof artifact or add policy-compliant fetcher"
                    .to_string(),
            cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                .to_string(),
            next_artifact:
                "waves/2026-05-13-columbus-south-terminal-contact-proof/plans/pulse-03.md"
                    .to_string(),
            validation_status: "review".to_string(),
        };
        let intake_row = T4TerminalColumbusProofIntakeRow {
            intake_id: "T4COLUMBUS-I271".to_string(),
            task_id: "T4PROOF-I271".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-I271".to_string(),
            route: "I-271".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district: "Columbus South".to_string(),
            source_family: "public-terminal-contact-proof".to_string(),
            required_proof_field: "route-to-terminal contact statement".to_string(),
            selected_higher_tier_attachment_requirement:
                "must name selected higher-tier attachment or remain source-needed".to_string(),
            contact_proof_source_artifact: "source-needed".to_string(),
            proof_status: "source-needed".to_string(),
            proof_blocker:
                "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                    .to_string(),
            next_artifact:
                "waves/2026-05-13-columbus-south-terminal-contact-proof/plans/pulse-02.md"
                    .to_string(),
            validation_status: "review".to_string(),
        };

        let failures = t4_terminal_columbus_source_access_gate_failures(&[bad_row], &[intake_row]);

        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("unsupported live fetch status")),
            "{failures:?}"
        );
    }

    #[test]
    fn t4_terminal_columbus_proof_attempts_preserve_blockers() {
        let source_access_rows = vec![
            T4TerminalColumbusSourceAccessRow {
                access_id: "T4COLUMBUSACCESS-I271".to_string(),
                intake_id: "T4COLUMBUS-I271".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-I271".to_string(),
                route: "I-271".to_string(),
                terminal_district: "Columbus South".to_string(),
                source_family: "public-terminal-contact-proof".to_string(),
                access_mode: "manual-or-cached-source-needed".to_string(),
                live_fetch_status: "unsupported-no-safe-terminal-fetcher".to_string(),
                required_source_metadata:
                    "source title; source url or cached artifact; capture date; route; terminal district; route-to-terminal contact statement"
                        .to_string(),
                contact_proof_source_artifact: "source-needed".to_string(),
                acquisition_status: "source-needed".to_string(),
                source_access_blocker:
                    "no safe live terminal-contact fetch command exists; use manual/cached proof artifact or add policy-compliant fetcher"
                        .to_string(),
                cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                    .to_string(),
                next_artifact:
                    "waves/2026-05-13-columbus-south-terminal-contact-proof/plans/pulse-03.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
            T4TerminalColumbusSourceAccessRow {
                access_id: "T4COLUMBUSACCESS-US22".to_string(),
                intake_id: "T4COLUMBUS-US22".to_string(),
                queue_id: "T4CONTACT-T3GREATLAKES-US22".to_string(),
                route: "US22".to_string(),
                terminal_district: "Columbus South".to_string(),
                source_family: "public-terminal-contact-proof".to_string(),
                access_mode: "manual-or-cached-source-needed".to_string(),
                live_fetch_status: "unsupported-no-safe-terminal-fetcher".to_string(),
                required_source_metadata:
                    "source title; source url or cached artifact; capture date; route; terminal district; route-to-terminal contact statement"
                        .to_string(),
                contact_proof_source_artifact: "source-needed".to_string(),
                acquisition_status: "source-needed".to_string(),
                source_access_blocker:
                    "no safe live terminal-contact fetch command exists; use manual/cached proof artifact or add policy-compliant fetcher"
                        .to_string(),
                cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                    .to_string(),
                next_artifact:
                    "waves/2026-05-13-columbus-south-terminal-contact-proof/plans/pulse-03.md"
                        .to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t4_terminal_columbus_proof_attempt_rows(&source_access_rows);
        let failures = t4_terminal_columbus_proof_attempt_gate_failures(&rows, &source_access_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.proof_attempt_status == "blocked"
            && row.proof_decision == "source-needed"
            && row.source_artifact == "source-needed"
            && row.contact_statement_status == "source-needed"
            && row.selected_higher_tier_attachment_status == "source-needed"));
    }

    #[test]
    fn t4_terminal_columbus_proof_attempts_reject_source_backed_without_artifact() {
        let bad_row = T4TerminalColumbusProofAttemptRow {
            attempt_id: "T4COLUMBUSATTEMPT-I271".to_string(),
            access_id: "T4COLUMBUSACCESS-I271".to_string(),
            intake_id: "T4COLUMBUS-I271".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-I271".to_string(),
            route: "I-271".to_string(),
            terminal_district: "Columbus South".to_string(),
            source_family: "public-terminal-contact-proof".to_string(),
            source_artifact: "source-needed".to_string(),
            capture_status: "captured".to_string(),
            contact_statement_status: "source-backed".to_string(),
            selected_higher_tier_attachment_status: "attached".to_string(),
            proof_attempt_status: "accepted".to_string(),
            proof_decision: "source-backed".to_string(),
            proof_blocker: "none".to_string(),
            next_artifact:
                "waves/2026-05-13-columbus-south-terminal-contact-proof/plans/pulse-04.md"
                    .to_string(),
            validation_status: "pass".to_string(),
        };
        let source_access_row = T4TerminalColumbusSourceAccessRow {
            access_id: "T4COLUMBUSACCESS-I271".to_string(),
            intake_id: "T4COLUMBUS-I271".to_string(),
            queue_id: "T4CONTACT-T3GREATLAKES-I271".to_string(),
            route: "I-271".to_string(),
            terminal_district: "Columbus South".to_string(),
            source_family: "public-terminal-contact-proof".to_string(),
            access_mode: "manual-or-cached-source-needed".to_string(),
            live_fetch_status: "unsupported-no-safe-terminal-fetcher".to_string(),
            required_source_metadata:
                "source title; source url or cached artifact; capture date; route; terminal district; route-to-terminal contact statement"
                    .to_string(),
            contact_proof_source_artifact: "source-needed".to_string(),
            acquisition_status: "source-needed".to_string(),
            source_access_blocker:
                "no safe live terminal-contact fetch command exists; use manual/cached proof artifact or add policy-compliant fetcher"
                    .to_string(),
            cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                .to_string(),
            next_artifact:
                "waves/2026-05-13-columbus-south-terminal-contact-proof/plans/pulse-03.md"
                    .to_string(),
            validation_status: "review".to_string(),
        };

        let failures =
            t4_terminal_columbus_proof_attempt_gate_failures(&[bad_row], &[source_access_row]);

        assert!(
            failures
                .iter()
                .any(|failure| failure.contains("lacks non-seed proof evidence")),
            "{failures:?}"
        );
    }

    #[test]
    fn t4_terminal_scenario_readiness_emits_held_clear_row_without_source_backed_contacts() {
        let contact_rows = vec![T4TerminalContactEvidenceRow {
            queue_id: "T4CONTACT-T3GREATLAKES-US10".to_string(),
            route: "US10".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district_seed: "Minneapolis Twin Cities".to_string(),
            terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
            contact_basis:
                "candidate-terminal-district-assigned; route-to-terminal contact source still needed"
                    .to_string(),
            contact_proof_source: String::new(),
            evidence_status: "source-needed".to_string(),
            selected_higher_tier_attachment: "source-needed".to_string(),
            decision: "source-needed".to_string(),
            next_artifact:
                "waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-02.md".to_string(),
            source_column_artifact: "data/t4-terminal-access-columns.csv".to_string(),
            source_column_decision: "terminal-review".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t4_terminal_scenario_readiness_rows(&contact_rows);
        let failures = t4_terminal_scenario_readiness_gate_failures(&rows, &contact_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].docket_id, "__all_t4_terminal_scenarios__");
        assert_eq!(rows[0].scenario_decision, "no-source-backed-contacts");
        assert_eq!(rows[0].release_status, "held-source-needed");
    }

    #[test]
    fn t4_terminal_scenario_readiness_requires_source_backed_proof() {
        let scenario_ready = T4TerminalContactEvidenceRow {
            queue_id: "T4CONTACT-T3GREATLAKES-I180".to_string(),
            route: "I-180".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district_seed: "Chicago Intermodal Complex".to_string(),
            terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
            contact_basis: "documented truck connector to terminal gate".to_string(),
            contact_proof_source: "future-contact-source.csv#I-180".to_string(),
            evidence_status: "accepted".to_string(),
            selected_higher_tier_attachment: "T3:t3-great-lakes:I-80".to_string(),
            decision: "scenario-ready".to_string(),
            next_artifact: "data/t4-terminal-scenario-readiness.csv".to_string(),
            source_column_artifact: "data/t4-terminal-access-columns.csv".to_string(),
            source_column_decision: "terminal-review".to_string(),
            validation_status: "pass".to_string(),
        };
        let rows = t4_terminal_scenario_readiness_rows(&[scenario_ready.clone()]);
        let failures = t4_terminal_scenario_readiness_gate_failures(&rows, &[scenario_ready]);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows[0].scenario_decision, "scenario-candidate");
        assert_eq!(rows[0].release_status, "held-source-review");

        let bad_row = T4TerminalScenarioReadinessRow {
            docket_id: "T4SCENARIO-BAD".to_string(),
            route: "I-180".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            terminal_district: "Chicago Intermodal Complex".to_string(),
            contact_basis: "documented truck connector to terminal gate".to_string(),
            contact_proof_source: String::new(),
            selected_higher_tier_attachment: "T3:t3-great-lakes:I-80".to_string(),
            freight_access_rationale: "source-backed contact".to_string(),
            scenario_decision: "scenario-candidate".to_string(),
            scenario_artifact: "data/t4-terminal-scenario-readiness.csv".to_string(),
            source_evidence_status: "accepted".to_string(),
            release_status: "held-source-review".to_string(),
            next_artifact: "waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-05.md"
                .to_string(),
            validation_status: "review".to_string(),
        };
        let bad_failures = t4_terminal_scenario_readiness_gate_failures(&[bad_row], &[]);
        assert!(
            bad_failures
                .iter()
                .any(|failure| failure.contains("lacks proof")),
            "{bad_failures:?}"
        );
    }

    #[test]
    fn t3_zone_map_diagnostics_summarize_selected_routes_and_gaps() {
        let route_rows = vec![
            T3ZoneRouteColumnRow {
                zone_id: "t3-southeast".to_string(),
                zone_name: "Southeast / Appalachia".to_string(),
                obligation_class: "regional-feeder-access".to_string(),
                route: "I65".to_string(),
                current_tier: "T2".to_string(),
                current_score: 64.9,
                constraint_adjusted_score: 64.9,
                hard_blocker_count: 0,
                claim_blocker_count: 0,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 0.0,
                top_constraint_classes: "none".to_string(),
                constraint_ledger_artifact: String::new(),
                promise_horizon_hours: 6,
                column_decision: "selected".to_string(),
                zone_role: "regional-feeder".to_string(),
                contact_requirement: "higher-tier-or-regional-contact-required".to_string(),
                map_treatment: "render-as-zone-column".to_string(),
                selection_basis: "score meets T3 threshold and satisfies a 6h feeder obligation"
                    .to_string(),
                source_obligation: "select T3 feeder/contact chain inside the zone".to_string(),
                next_artifact: "data/t3-zone-map-diagnostics.csv".to_string(),
                optimizer_effect: "feeds the T3 zone map and stop-column selector".to_string(),
                validation_status: "pass".to_string(),
            },
            T3ZoneRouteColumnRow {
                zone_id: "t3-southeast".to_string(),
                zone_name: "Southeast / Appalachia".to_string(),
                obligation_class: "regional-feeder-access".to_string(),
                route: "US90Z".to_string(),
                current_tier: "T4".to_string(),
                current_score: 29.9,
                constraint_adjusted_score: 29.9,
                hard_blocker_count: 0,
                claim_blocker_count: 0,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 0.0,
                top_constraint_classes: "none".to_string(),
                constraint_ledger_artifact: String::new(),
                promise_horizon_hours: 6,
                column_decision: "review".to_string(),
                zone_role: "below-threshold-feeder-candidate".to_string(),
                contact_requirement: "score-or-terminal-evidence-required".to_string(),
                map_treatment: "show-as-held-zone-candidate".to_string(),
                selection_basis: "candidate is below T3 threshold for a 6h feeder obligation"
                    .to_string(),
                source_obligation: "select T3 feeder/contact chain inside the zone".to_string(),
                next_artifact: "data/t3-t4-access-gaps.csv".to_string(),
                optimizer_effect:
                    "holds weak feeder pressure for access-gap review instead of selecting it"
                        .to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let gap_rows = vec![T3T4AccessGapRow {
            gap_id: "T3GAP-T3SOUTHEAST-US90Z".to_string(),
            source_surface: "t3-zone-route-columns".to_string(),
            route: "US90Z".to_string(),
            zone_id: "t3-southeast".to_string(),
            current_score: 29.9,
            constraint_adjusted_score: 29.9,
            hard_blocker_count: 0,
            claim_blocker_count: 0,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 0.0,
            top_constraint_classes: "none".to_string(),
            constraint_ledger_artifact: String::new(),
            promise_horizon_hours: 6,
            gap_class: "below-threshold-feeder".to_string(),
            gap_reason: "candidate is below T3 threshold for a 6h feeder obligation".to_string(),
            required_evidence: "score-or-terminal-evidence-required".to_string(),
            repair_action: "prove-terminal-evidence-or-keep-t4".to_string(),
            next_artifact: "data/t3-zone-map-diagnostics.csv".to_string(),
            upward_pressure_allowed: false,
            validation_status: "review".to_string(),
        }];
        let atlas = vec![test_t3_map_atlas_row("t3-southeast")];

        let rows = t3_zone_map_diagnostic_rows(&route_rows, &gap_rows, &atlas);
        let failures = t3_zone_map_diagnostic_gate_failures(&rows, &atlas);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].selected_route_count, 1);
        assert_eq!(rows[0].review_connector_count, 1);
        assert_eq!(rows[0].below_threshold_feeder_count, 1);
        assert_eq!(rows[0].map_readiness, "review-terminal-and-feeder-gaps");
        assert!(failures.is_empty());
    }

    #[test]
    fn t3_zone_render_board_covers_selected_routes_and_gap_callouts() {
        let diagnostics = vec![T3ZoneMapDiagnosticRow {
            zone_id: "t3-southeast".to_string(),
            zone_name: "Southeast / Appalachia".to_string(),
            map_id: "t3-southeast".to_string(),
            map_path: "maps/t3-zone-southeast.svg".to_string(),
            selected_route_count: 1,
            selected_routes: "I65".to_string(),
            review_connector_count: 1,
            review_connectors: "US90Z".to_string(),
            access_gap_count: 1,
            below_threshold_feeder_count: 1,
            terminal_evidence_gap_count: 0,
            zone_assignment_gap_count: 0,
            map_readiness: "review-terminal-and-feeder-gaps".to_string(),
            diagnostic_action: "render selected feeders with held access-gap callouts".to_string(),
            next_artifact: "maps/t3-zone".to_string(),
            validation_status: "review".to_string(),
        }];
        let route_rows = vec![
            T3ZoneRouteColumnRow {
                zone_id: "t3-southeast".to_string(),
                zone_name: "Southeast / Appalachia".to_string(),
                obligation_class: "regional-feeder-access".to_string(),
                route: "I65".to_string(),
                current_tier: "T2".to_string(),
                current_score: 64.9,
                constraint_adjusted_score: 64.9,
                hard_blocker_count: 0,
                claim_blocker_count: 0,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 0.0,
                top_constraint_classes: "none".to_string(),
                constraint_ledger_artifact: String::new(),
                promise_horizon_hours: 6,
                column_decision: "selected".to_string(),
                zone_role: "regional-feeder".to_string(),
                contact_requirement: "higher-tier-or-regional-contact-required".to_string(),
                map_treatment: "render-as-zone-column".to_string(),
                selection_basis: "score meets T3 threshold and satisfies a 6h feeder obligation"
                    .to_string(),
                source_obligation: "select T3 feeder/contact chain inside the zone".to_string(),
                next_artifact: "data/t3-zone-map-diagnostics.csv".to_string(),
                optimizer_effect: "feeds the T3 zone map and stop-column selector".to_string(),
                validation_status: "pass".to_string(),
            },
            T3ZoneRouteColumnRow {
                zone_id: "t3-southeast".to_string(),
                zone_name: "Southeast / Appalachia".to_string(),
                obligation_class: "regional-feeder-access".to_string(),
                route: "US90Z".to_string(),
                current_tier: "T4".to_string(),
                current_score: 29.9,
                constraint_adjusted_score: 29.9,
                hard_blocker_count: 0,
                claim_blocker_count: 0,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 0.0,
                top_constraint_classes: "none".to_string(),
                constraint_ledger_artifact: String::new(),
                promise_horizon_hours: 6,
                column_decision: "review".to_string(),
                zone_role: "below-threshold-feeder-candidate".to_string(),
                contact_requirement: "score-or-terminal-evidence-required".to_string(),
                map_treatment: "show-as-held-zone-candidate".to_string(),
                selection_basis: "candidate is below T3 threshold for a 6h feeder obligation"
                    .to_string(),
                source_obligation: "select T3 feeder/contact chain inside the zone".to_string(),
                next_artifact: "data/t3-t4-access-gaps.csv".to_string(),
                optimizer_effect:
                    "holds weak feeder pressure for access-gap review instead of selecting it"
                        .to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let gap_rows = vec![T3T4AccessGapRow {
            gap_id: "T3GAP-T3SOUTHEAST-US90Z".to_string(),
            source_surface: "t3-zone-route-columns".to_string(),
            route: "US90Z".to_string(),
            zone_id: "t3-southeast".to_string(),
            current_score: 29.9,
            constraint_adjusted_score: 29.9,
            hard_blocker_count: 0,
            claim_blocker_count: 0,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 0.0,
            top_constraint_classes: "none".to_string(),
            constraint_ledger_artifact: String::new(),
            promise_horizon_hours: 6,
            gap_class: "below-threshold-feeder".to_string(),
            gap_reason: "candidate is below T3 threshold for a 6h feeder obligation".to_string(),
            required_evidence: "score-or-terminal-evidence-required".to_string(),
            repair_action: "prove-terminal-evidence-or-keep-t4".to_string(),
            next_artifact: "data/t3-zone-map-diagnostics.csv".to_string(),
            upward_pressure_allowed: false,
            validation_status: "review".to_string(),
        }];
        let atlas = vec![test_t3_map_atlas_row("t3-southeast")];

        let rows = crate::support::tier::t3_zone_render_board_rows::t3_zone_render_board_rows(&diagnostics, &route_rows, &gap_rows, &atlas);
        let failures = t3_zone_render_board_gate_failures(&rows, &atlas);
        let layers = rows
            .iter()
            .map(|row| row.board_layer.as_str())
            .collect::<std::collections::BTreeSet<_>>();

        assert_eq!(rows.len(), 4);
        assert!(layers.contains("zone-summary"));
        assert!(layers.contains("selected-route"));
        assert!(layers.contains("review-connector"));
        assert!(layers.contains("held-gap"));
        assert!(rows.iter().any(|row| row.board_layer == "selected-route"
            && row.route == "I65"
            && row.map_treatment == "render-as-zone-column"));
        assert!(failures.is_empty());
    }

    #[test]
    fn t3_zone_stop_placement_marks_ready_and_authoring_gaps() {
        let i65_segment = t3_national_segment_id("t3-southeast", "I65");
        let i65_stitch = t3_stitch_group_id("t3-southeast", "I65");
        let i65_bundle = t3_segment_bundle_id("t3-southeast", "I65");
        let i65_aliases = t3_segment_aliases("t3-southeast", "I65", "selected-route");
        let i22_segment = t3_national_segment_id("t3-southeast", "I22");
        let i22_stitch = t3_stitch_group_id("t3-southeast", "I22");
        let i22_bundle = t3_segment_bundle_id("t3-southeast", "I22");
        let i22_aliases = t3_segment_aliases("t3-southeast", "I22", "selected-route");
        let board_rows = vec![
            T3ZoneRenderBoardRow {
                zone_id: "t3-southeast".to_string(),
                zone_name: "Southeast / Appalachia".to_string(),
                map_id: "t3-southeast".to_string(),
                map_path: "maps/t3-southeast.png".to_string(),
                board_layer: "selected-route".to_string(),
                route: "I65".to_string(),
                national_segment_id: i65_segment.clone(),
                stitch_group_id: i65_stitch,
                segment_bundle_id: i65_bundle.clone(),
                segment_aliases: i65_aliases,
                route_status: "selected".to_string(),
                map_treatment: "render-as-zone-column".to_string(),
                selected_route_count: 2,
                access_gap_count: 0,
                source_artifact: "data/t3-zone-route-columns.csv".to_string(),
                render_action: "render selected T3 route column with stop placement constraints"
                    .to_string(),
                next_artifact: "maps/t3-zone".to_string(),
                validation_status: "pass".to_string(),
            },
            T3ZoneRenderBoardRow {
                zone_id: "t3-southeast".to_string(),
                zone_name: "Southeast / Appalachia".to_string(),
                map_id: "t3-southeast".to_string(),
                map_path: "maps/t3-southeast.png".to_string(),
                board_layer: "selected-route".to_string(),
                route: "I22".to_string(),
                national_segment_id: i22_segment,
                stitch_group_id: i22_stitch,
                segment_bundle_id: i22_bundle,
                segment_aliases: i22_aliases,
                route_status: "selected".to_string(),
                map_treatment: "render-as-zone-column".to_string(),
                selected_route_count: 2,
                access_gap_count: 0,
                source_artifact: "data/t3-zone-route-columns.csv".to_string(),
                render_action: "render selected T3 route column with stop placement constraints"
                    .to_string(),
                next_artifact: "maps/t3-zone".to_string(),
                validation_status: "pass".to_string(),
            },
        ];
        let stop_rows = vec![
            StopCandidateRow {
                stop_id: "STOP-BHM".to_string(),
                name: "Birmingham".to_string(),
                state: "AL".to_string(),
                lat: "33.52".to_string(),
                lon: "-86.80".to_string(),
                requested_class: "S2".to_string(),
                route_refs: "I-65".to_string(),
                stop_role: "major_interchange_hub".to_string(),
                transfer_value: "high".to_string(),
                freight_volume: "high".to_string(),
                spacing_need: "met".to_string(),
                resilience_value: "medium".to_string(),
                energy_service: "planned".to_string(),
                land_ops_feasibility: "medium".to_string(),
                equity_community: "review_needed".to_string(),
                evidence_status: "heuristic".to_string(),
                source_artifact: "data/tier-stop-candidates.csv".to_string(),
                next_step: "Validate stop geometry".to_string(),
            },
            StopCandidateRow {
                stop_id: "STOP-NASH".to_string(),
                name: "Nashville".to_string(),
                state: "TN".to_string(),
                lat: "36.16".to_string(),
                lon: "-86.78".to_string(),
                requested_class: "S3".to_string(),
                route_refs: "I-65".to_string(),
                stop_role: "transfer_stop".to_string(),
                transfer_value: "medium".to_string(),
                freight_volume: "medium".to_string(),
                spacing_need: "met".to_string(),
                resilience_value: "medium".to_string(),
                energy_service: "planned".to_string(),
                land_ops_feasibility: "medium".to_string(),
                equity_community: "review_needed".to_string(),
                evidence_status: "heuristic".to_string(),
                source_artifact: "data/tier-stop-candidates.csv".to_string(),
                next_step: "Validate stop geometry".to_string(),
            },
        ];

        let rows = t3_zone_stop_placement_rows(&board_rows, &stop_rows);
        let failures = t3_zone_stop_placement_gate_failures(&rows, &board_rows);

        assert_eq!(rows.len(), 2);
        assert!(rows.iter().any(|row| row.route == "I65"
            && row.national_segment_id == i65_segment
            && row.segment_bundle_id == i65_bundle
            && row.state_scope == "AL;TN"
            && row.placement_status == "ready-for-stop-layout"
            && row.validation_status == "pass"));
        assert!(rows.iter().any(|row| row.route == "I22"
            && row.placement_status == "needs-stop-chain"
            && row.validation_status == "review"));
        assert!(failures.is_empty());
    }

    #[test]
    fn national_segment_registry_merges_layers_and_stop_scope() {
        let i65_segment = t3_national_segment_id("t3-southeast", "I65");
        let i65_stitch = t3_stitch_group_id("t3-southeast", "I65");
        let i65_bundle = t3_segment_bundle_id("t3-southeast", "I65");
        let board_rows = vec![
            T3ZoneRenderBoardRow {
                zone_id: "t3-southeast".to_string(),
                zone_name: "Southeast / Appalachia".to_string(),
                map_id: "t3-southeast".to_string(),
                map_path: "maps/t3-southeast.png".to_string(),
                board_layer: "selected-route".to_string(),
                route: "I65".to_string(),
                national_segment_id: i65_segment.clone(),
                stitch_group_id: i65_stitch.clone(),
                segment_bundle_id: i65_bundle.clone(),
                segment_aliases: t3_segment_aliases("t3-southeast", "I65", "selected-route"),
                route_status: "selected".to_string(),
                map_treatment: "render-as-zone-column".to_string(),
                selected_route_count: 1,
                access_gap_count: 0,
                source_artifact: "data/t3-zone-route-columns.csv".to_string(),
                render_action: "render selected T3 route column with stop placement constraints"
                    .to_string(),
                next_artifact: "maps/t3-zone".to_string(),
                validation_status: "pass".to_string(),
            },
            T3ZoneRenderBoardRow {
                zone_id: "t3-southeast".to_string(),
                zone_name: "Southeast / Appalachia".to_string(),
                map_id: "t3-southeast".to_string(),
                map_path: "maps/t3-southeast.png".to_string(),
                board_layer: "held-gap".to_string(),
                route: "I65".to_string(),
                national_segment_id: i65_segment.clone(),
                stitch_group_id: i65_stitch.clone(),
                segment_bundle_id: i65_bundle.clone(),
                segment_aliases: t3_segment_aliases("t3-southeast", "I65", "held-gap"),
                route_status: "terminal-evidence-needed".to_string(),
                map_treatment: "render-gap-callout".to_string(),
                selected_route_count: 1,
                access_gap_count: 1,
                source_artifact: "t4-terminal-access-columns".to_string(),
                render_action: "prove-terminal-access-or-keep-local".to_string(),
                next_artifact: "data/t3-zone-map-diagnostics.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let placement_rows = vec![T3ZoneStopPlacementRow {
            zone_id: "t3-southeast".to_string(),
            zone_name: "Southeast / Appalachia".to_string(),
            route: "I65".to_string(),
            national_segment_id: i65_segment.clone(),
            stitch_group_id: i65_stitch,
            segment_bundle_id: i65_bundle,
            segment_aliases: t3_segment_aliases("t3-southeast", "I65", "selected-route"),
            state_scope: "AL;TN".to_string(),
            stop_count: 2,
            transfer_grade_stop_count: 2,
            stop_chain: "STOP-BHM;STOP-NASH".to_string(),
            stop_classes: "S2;S3".to_string(),
            placement_status: "ready-for-stop-layout".to_string(),
            placement_action: "place route on zone schematic using ordered stop chain".to_string(),
            source_artifact: "data/t3-zone-render-board.csv; data/tier-stop-candidates.csv"
                .to_string(),
            next_artifact: "maps/t3-zone".to_string(),
            validation_status: "pass".to_string(),
        }];

        let rows = crate::support::network::national_segment_registry_rows::national_segment_registry_rows(&board_rows, &placement_rows, &[], &[]);
        let failures = national_segment_registry_gate_failures(&rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].national_segment_id, i65_segment);
        assert_eq!(rows[0].member_segment_ids, rows[0].national_segment_id);
        assert_eq!(rows[0].current_zone_id, "t3-southeast");
        assert_eq!(rows[0].current_tier, "T3");
        assert_eq!(rows[0].route_label, "I65");
        assert!(rows[0].board_layers.contains("selected-route"));
        assert!(rows[0].board_layers.contains("held-gap"));
        assert!(rows[0].board_layers.contains("stop-placement"));
        assert_eq!(rows[0].state_scope, "AL;TN");
        assert_eq!(rows[0].evidence_state_scope, "AL;TN");
        assert_eq!(rows[0].geometry_state_scope, "");
        assert_eq!(rows[0].registry_action, "eligible-for-geometry-layout");
        assert!(failures.is_empty());
    }

    #[test]
    fn national_segment_bundles_roll_up_registry_members() {
        let registry_rows = vec![NationalSegmentRegistryRow {
            member_segment_ids: "US.HWYSEG.I65-SOUTHEAST".to_string(),
            bundle_role: "single-segment".to_string(),
            national_segment_id: "US.HWYSEG.I65-SOUTHEAST".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I65-SOUTHEAST".to_string(),
            stitch_group_id: "US.HWYSTITCH.I65-SOUTHEAST".to_string(),
            current_zone_id: "t3-southeast".to_string(),
            current_tier: "T3".to_string(),
            route_label: "I65".to_string(),
            zone_id: "t3-southeast".to_string(),
            route: "I65".to_string(),
            state_scope: "AL;TN".to_string(),
            evidence_state_scope: "AL;TN".to_string(),
            geometry_state_scope: String::new(),
            segment_aliases: "current-tier:T3;current-zone:t3-southeast;route:I65".to_string(),
            bundle_aliases: "current-tier:T3;current-zone:t3-southeast;route:I65".to_string(),
            board_layers: "selected-route;stop-placement".to_string(),
            source_artifacts: "data/t3-zone-render-board.csv".to_string(),
            stop_placement_status: "ready-for-stop-layout".to_string(),
            registry_action: "eligible-for-geometry-layout".to_string(),
            qualification_effects: "qualification_gate_policy=stop-first".to_string(),
            validation_status: "pass".to_string(),
        }];

        let rows = national_segment_bundle_rows(&registry_rows);
        let failures = national_segment_bundle_gate_failures(&rows, &registry_rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].segment_bundle_id, "US.HWYBUNDLE.I65-SOUTHEAST");
        assert_eq!(rows[0].bundle_role, "single-segment");
        assert_eq!(rows[0].member_count, 1);
        assert_eq!(rows[0].member_segment_ids, "US.HWYSEG.I65-SOUTHEAST");
        assert_eq!(rows[0].bundle_status, "bundle-ready");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].validation_status, "pass");
        assert!(failures.is_empty());
    }

    #[test]
    fn national_registry_ingests_tier_segment_members_and_pavement_readiness() {
        let segment_rows = vec![
            TierSegmentCandidateRow {
                tier: "T2".to_string(),
                source_selector: "t2-service-selection".to_string(),
                region_id: "component-1".to_string(),
                route: "I15".to_string(),
                edge_id: 100,
                edge_sequence: 1,
                national_segment_id: "US.HWYSEG.0000000000000100".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.I15".to_string(),
                stitch_group_id: "US.HWYSTITCH.I15".to_string(),
                member_role: "stitched-member".to_string(),
                state: "CA".to_string(),
                length_miles: 10.0,
                aadt: "100000".to_string(),
                lane_count: "6".to_string(),
                route_aliases: "current-tier:T2;current-zone:component-1;route:I15".to_string(),
                selector_basis: "I80;I10".to_string(),
                candidate_action: "connector".to_string(),
                qualification_effects: "qualification_gate_policy=stop-first".to_string(),
                next_artifact: "data/national-segment-registry.csv".to_string(),
                validation_status: "review".to_string(),
            },
            TierSegmentCandidateRow {
                tier: "T2".to_string(),
                source_selector: "t2-service-selection".to_string(),
                region_id: "component-1".to_string(),
                route: "I15".to_string(),
                edge_id: 101,
                edge_sequence: 2,
                national_segment_id: "US.HWYSEG.0000000000000101".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.I15".to_string(),
                stitch_group_id: "US.HWYSTITCH.I15".to_string(),
                member_role: "stitched-member".to_string(),
                state: "NV".to_string(),
                length_miles: 12.0,
                aadt: "90000".to_string(),
                lane_count: "4".to_string(),
                route_aliases: "current-tier:T2;current-zone:component-1;route:I15".to_string(),
                selector_basis: "I80;I10".to_string(),
                candidate_action: "connector".to_string(),
                qualification_effects: "qualification_gate_policy=stop-first".to_string(),
                next_artifact: "data/national-segment-registry.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let pavement_rows = vec![
            TierPavementDocketRow {
                tier: "T2".to_string(),
                source_selector: "t2-service-selection".to_string(),
                region_id: "component-1".to_string(),
                route: "I15".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.I15".to_string(),
                stitch_group_id: "US.HWYSTITCH.I15".to_string(),
                national_segment_id: "US.HWYSEG.0000000000000100".to_string(),
                edge_id: 100,
                edge_sequence: 1,
                state: "CA".to_string(),
                length_miles: 10.0,
                iri_m_per_km: "1.20".to_string(),
                max_iri_m_per_km: "1.90".to_string(),
                pavement_status: "pavement-floor-pass".to_string(),
                repair_action: "no pavement debt payment required for this member".to_string(),
                freight_ride_requirement: "regional freight ride quality".to_string(),
                transit_ride_requirement: "regional coach ride quality".to_string(),
                source_contract: "HPMS IRI".to_string(),
                qualification_effects: "qualification_gate_policy=stop-first".to_string(),
                next_artifact: "data/national-segment-registry.csv".to_string(),
                validation_status: "pass".to_string(),
            },
            TierPavementDocketRow {
                tier: "T2".to_string(),
                source_selector: "t2-service-selection".to_string(),
                region_id: "component-1".to_string(),
                route: "I15".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.I15".to_string(),
                stitch_group_id: "US.HWYSTITCH.I15".to_string(),
                national_segment_id: "US.HWYSEG.0000000000000101".to_string(),
                edge_id: 101,
                edge_sequence: 2,
                state: "NV".to_string(),
                length_miles: 12.0,
                iri_m_per_km: "unknown".to_string(),
                max_iri_m_per_km: "1.90".to_string(),
                pavement_status: "pavement-source-needed".to_string(),
                repair_action: "record pavement evidence debt".to_string(),
                freight_ride_requirement: "regional freight ride quality".to_string(),
                transit_ride_requirement: "regional coach ride quality".to_string(),
                source_contract: "HPMS IRI".to_string(),
                qualification_effects: "qualification_game_use=default-play".to_string(),
                next_artifact: "data/standards-l1-inventory.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let registry_rows = crate::support::network::national_segment_registry_rows::national_segment_registry_rows(&[], &[], &segment_rows, &pavement_rows);
        let registry_failures = national_segment_registry_gate_failures(&registry_rows);
        let bundle_rows = national_segment_bundle_rows(&registry_rows);

        assert!(registry_failures.is_empty(), "{registry_failures:?}");
        assert_eq!(registry_rows.len(), 2);
        assert!(registry_rows
            .iter()
            .all(|row| row.registry_action == "eligible-for-service-bundle"));
        assert!(registry_rows
            .iter()
            .any(|row| row.qualification_effects == "qualification_gate_policy=stop-first"));
        assert_eq!(bundle_rows.len(), 1);
        assert_eq!(bundle_rows[0].member_count, 2);
        assert_eq!(bundle_rows[0].bundle_status, "bundle-ready");
        assert_eq!(bundle_rows[0].next_artifact, "maps/t3-zone");
    }

    #[test]
    fn t2_bubble_up_review_requires_contact_gate() {
        let intake = vec![
            T3T4PressureIntakeRow {
                route: "I-57".to_string(),
                source_pressure_type: "regional-upgrade-pressure".to_string(),
                current_tier: "T3".to_string(),
                current_score: 49.6,
                target_tier: "T2".to_string(),
                intake_class: "bubble-up-t2-review".to_string(),
                intake_action: "send-to-t2-contact-review".to_string(),
                selection_basis: "score-within-five-points-of-t2-threshold".to_string(),
                source_artifact: "data/tier-table.csv".to_string(),
                next_artifact: "data/tier-contact-witnesses.csv".to_string(),
                optimizer_effect:
                    "lower-tier score pressure can reopen T2 only through contact gates".to_string(),
                validation_status: "review".to_string(),
            },
            T3T4PressureIntakeRow {
                route: "I25".to_string(),
                source_pressure_type: "closure-demotion-pressure".to_string(),
                current_tier: "T2".to_string(),
                current_score: 66.3,
                target_tier: "T3".to_string(),
                intake_class: "t3-regional-intake".to_string(),
                intake_action: "accept-as-t3-regional-review".to_string(),
                selection_basis: "source-backed T1/T2 contact".to_string(),
                source_artifact: "data/t2-contact-closure.csv".to_string(),
                next_artifact: "data/t3-t4-pressure-intake.csv".to_string(),
                optimizer_effect: "consume T2 demotion as regional feeder review".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t2_bubble_up_review_rows(&intake);
        let failures = t2_bubble_up_review_gate_failures(&rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].route, "I-57");
        assert_eq!(
            rows[0].review_action,
            "require-t2-contact-witness-before-upgrade"
        );
        assert!(failures.is_empty());
    }

    #[test]
    fn t1_feedback_docket_rejects_score_only_promotion() {
        let service = vec![T2ServiceSelectionRow {
            tier: "T2".to_string(),
            region_id: "R1".to_string(),
            route: "I285".to_string(),
            parent_trunks: "I75;I85".to_string(),
            column_decision: "review".to_string(),
            treatment_status: "review-treatment".to_string(),
            beck_corridor: String::new(),
            beck_service_class: String::new(),
            beck_color_mode: String::new(),
            beck_start_trunk: String::new(),
            beck_end_trunk: String::new(),
            duplicate_service_count: 0,
            duplicate_service_corridors: String::new(),
            close_parallel_count: 0,
            close_parallel_corridors: String::new(),
            unstopped_t1_contact_count: 0,
            unstopped_t1_contacts: String::new(),
            pavement_debt_cost_m: 0.0,
            pavement_debt_class: "none".to_string(),
            pavement_debt_basis: "no pavement debt row joined".to_string(),
            hard_blocker_count: 0,
            claim_blocker_count: 0,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 0.0,
            top_constraint_classes: "none".to_string(),
            qualification_effects: "qualification_gate_policy=stop-first".to_string(),
            constraint_ledger_artifact: String::new(),
            beck_service_action: String::new(),
            qualification_basis: String::new(),
            qualification_map_treatment: String::new(),
            qualification_gate_policy: String::new(),
            qualification_game_use: String::new(),
            selection_action: "closure-review-needs-beck-diagnostic".to_string(),
            selection_basis: "closure-accepted-missing-beck-t2-diagnostic".to_string(),
            validation_status: "review".to_string(),
        }];
        let bubble = vec![T2BubbleUpReviewRow {
            route: "I-57".to_string(),
            source_intake_class: "bubble-up-t2-review".to_string(),
            current_score: 49.6,
            review_action: "require-t2-contact-witness-before-upgrade".to_string(),
            required_evidence: "T2 contact witness plus source-backed regional service value"
                .to_string(),
            next_artifact: "data/tier-contact-witnesses.csv".to_string(),
            optimizer_effect: "may reopen T2 candidate review only after contact validation"
                .to_string(),
            validation_status: "review".to_string(),
        }];
        let sla = vec![T1SlaPairRow {
            pair_id: "NY-LA-48".to_string(),
            origin_id: "NYC".to_string(),
            dest_id: "LAX".to_string(),
            target_hours: 48.0,
            priority: 10,
            market_class: "national".to_string(),
            required_routes: "I95;I80;I15".to_string(),
            required_stops: "NYC;CHI;SLC;LAX".to_string(),
            evidence_basis: "promise portfolio".to_string(),
        }];

        let rows = t1_feedback_docket_rows(&service, &bubble, &[], &sla);
        let failures = t1_feedback_docket_gate_failures(&rows);

        assert_eq!(rows.len(), 2);
        assert!(rows
            .iter()
            .any(|row| row.route == "I285" && row.t1_feedback_class == "beck-diagnostic-needed"));
        assert!(rows
            .iter()
            .any(|row| row.route == "I-57" && row.t1_feedback_class == "t2-contact-first"));
        assert!(rows
            .iter()
            .all(|row| row.t1_feedback_class != "t1-sla-candidate"));
        assert!(failures.is_empty());
    }

    #[test]
    fn t1_feedback_docket_allows_named_sla_candidate() {
        let bubble = vec![T2BubbleUpReviewRow {
            route: "I-57".to_string(),
            source_intake_class: "bubble-up-t2-review".to_string(),
            current_score: 49.6,
            review_action: "require-t2-contact-witness-before-upgrade".to_string(),
            required_evidence: "T2 contact witness plus source-backed regional service value"
                .to_string(),
            next_artifact: "data/tier-contact-witnesses.csv".to_string(),
            optimizer_effect: "may reopen T2 candidate review only after contact validation"
                .to_string(),
            validation_status: "review".to_string(),
        }];
        let sla = vec![T1SlaPairRow {
            pair_id: "CHI-MEM-36".to_string(),
            origin_id: "CHI".to_string(),
            dest_id: "MEM".to_string(),
            target_hours: 36.0,
            priority: 8,
            market_class: "national".to_string(),
            required_routes: "I-57;I55".to_string(),
            required_stops: "CHI;MEM".to_string(),
            evidence_basis: "promise portfolio".to_string(),
        }];

        let rows = t1_feedback_docket_rows(&[], &bubble, &[], &sla);
        let failures = t1_feedback_docket_gate_failures(&rows);

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].t1_feedback_class, "t1-sla-candidate");
        assert_eq!(rows[0].t1_sla_pair_count, 1);
        assert_eq!(rows[0].t1_sla_pairs, "CHI-MEM-36");
        assert!(failures.is_empty());
    }

    #[test]
    fn t1_sla_candidate_pairs_explain_cutline() {
        let candidates = vec![
            T1SlaCandidateUniverseRow {
                pair_id: "A-B-48".to_string(),
                origin_id: "A".to_string(),
                dest_id: "B".to_string(),
                target_hours: 48.0,
                market_class: "national".to_string(),
                required_routes: "I80".to_string(),
                required_stops: "A;B".to_string(),
                evidence_basis: "test".to_string(),
                market_score: 20.0,
                conversion_score: 20.0,
                coverage_score: 20.0,
                reuse_score: 10.0,
                resilience_score: 5.0,
                evidence_score: 5.0,
                budget_penalty: 0.0,
                drop_reason_hint: String::new(),
                covered_by_selected_pair: String::new(),
            },
            T1SlaCandidateUniverseRow {
                pair_id: "C-D-36".to_string(),
                origin_id: "C".to_string(),
                dest_id: "D".to_string(),
                target_hours: 36.0,
                market_class: "national".to_string(),
                required_routes: "I95".to_string(),
                required_stops: "C;D".to_string(),
                evidence_basis: "test".to_string(),
                market_score: 10.0,
                conversion_score: 10.0,
                coverage_score: 10.0,
                reuse_score: 10.0,
                resilience_score: 5.0,
                evidence_score: 5.0,
                budget_penalty: 0.0,
                drop_reason_hint: "already-covered-by-selected-promise".to_string(),
                covered_by_selected_pair: "A-B-48".to_string(),
            },
        ];
        let selected = vec![T1SlaPairRow {
            pair_id: "A-B-48".to_string(),
            origin_id: "A".to_string(),
            dest_id: "B".to_string(),
            target_hours: 48.0,
            priority: 10,
            market_class: "national".to_string(),
            required_routes: "I80".to_string(),
            required_stops: "A;B".to_string(),
            evidence_basis: "test".to_string(),
        }];

        let rows = t1_sla_candidate_pair_rows(&candidates, &selected, 1);
        let failures = t1_sla_candidate_pair_gate_failures(&rows, &selected, 1);

        assert_eq!(rows[0].pair_id, "A-B-48");
        assert!(rows[0].portfolio_selected);
        assert_eq!(rows[1].cutline_status, "dropped-at-cutline");
        assert_eq!(rows[1].covered_by_selected_pair, "A-B-48");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_regionalizer_includes_selected_and_review_columns() {
        let rows = vec![
            TierCandidateColumnRow {
                tier: "T2".to_string(),
                route: "US30".to_string(),
                candidate_type: "route-service-column".to_string(),
                graph_kind: "dual-route-graph".to_string(),
                split_objective: "route-mile-workload".to_string(),
                node_class: "trunk_connector".to_string(),
                route_miles: 1800.0,
                observed_t1_node_count: 3,
                observed_dual_contacts: 5,
                parent_trunks: "I80;I84".to_string(),
                component_id: 1,
                component_route_count: 18,
                component_status: "component-bridged:21".to_string(),
                witness_type: "regionalizer-ready".to_string(),
                repair_action: "keep-for-regionalizer".to_string(),
                repair_basis: "touches-multiple-t1-trunks".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.US30".to_string(),
                bundle_status: "bundle-ready".to_string(),
                bundle_action: "use bundle as service join surface".to_string(),
                pavement_debt_cost_m: 5.85,
                pavement_debt_class: "evidence-debt".to_string(),
                pavement_debt_basis: "fixture pavement debt".to_string(),
                pavement_debt_artifact: "data/tier-pavement-debt-budget.csv".to_string(),
                hard_blocker_count: 0,
                claim_blocker_count: 0,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 0.0,
                top_constraint_classes: "none".to_string(),
                qualification_effects:
                    "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                        .to_string(),
                constraint_ledger_artifact: String::new(),
                column_decision: "selected".to_string(),
                evidence_status: "accepted".to_string(),
                required_artifact: "data/tier-candidate-columns.csv".to_string(),
                validation_status: "pass".to_string(),
            },
            TierCandidateColumnRow {
                tier: "T2".to_string(),
                route: "I29".to_string(),
                candidate_type: "route-service-column".to_string(),
                graph_kind: "dual-route-graph".to_string(),
                split_objective: "route-mile-workload".to_string(),
                node_class: "relief_loop".to_string(),
                route_miles: 1499.0,
                observed_t1_node_count: 2,
                observed_dual_contacts: 1,
                parent_trunks: "I35".to_string(),
                component_id: 1,
                component_route_count: 18,
                component_status: "component-bridged:21".to_string(),
                witness_type: "parent-region-review".to_string(),
                repair_action: "keep-with-parent-region-review".to_string(),
                repair_basis: "relief-loop-shares-parent-service-context".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.I29".to_string(),
                bundle_status: "bundle-ready".to_string(),
                bundle_action: "use bundle as service join surface".to_string(),
                pavement_debt_cost_m: 0.0,
                pavement_debt_class: "none".to_string(),
                pavement_debt_basis: "no pavement debt row joined".to_string(),
                pavement_debt_artifact: String::new(),
                hard_blocker_count: 0,
                claim_blocker_count: 0,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 0.0,
                top_constraint_classes: "none".to_string(),
                qualification_effects:
                    "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                        .to_string(),
                constraint_ledger_artifact: String::new(),
                column_decision: "review".to_string(),
                evidence_status: "review".to_string(),
                required_artifact: "data/tier-candidate-columns.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let regionalizer = t2_regionalizer_rows(&rows);
        let failures = t2_regionalizer_gate_failures(&regionalizer);

        assert_eq!(regionalizer.len(), 2);
        assert_eq!(regionalizer[0].treatment_status, "selected-treatment");
        assert_eq!(
            regionalizer[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(regionalizer[0].pavement_debt_cost_m, 5.85);
        assert_eq!(regionalizer[1].treatment_status, "review-treatment");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_service_selection_joins_regionalizer_to_beck_diagnostics() {
        let regionalizer = vec![
            T2RegionalizerRow {
                tier: "T2".to_string(),
                region_id: "component-1".to_string(),
                component_id: 1,
                route: "I15".to_string(),
                parent_trunks: "I5;I70".to_string(),
                route_miles: 500.0,
                column_decision: "selected".to_string(),
                treatment_status: "selected-treatment".to_string(),
                evidence_status: "accepted".to_string(),
                pavement_debt_cost_m: 0.0,
                pavement_debt_class: "none".to_string(),
                pavement_debt_basis: "no pavement debt row joined".to_string(),
                hard_blocker_count: 0,
                claim_blocker_count: 0,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 0.0,
                top_constraint_classes: "none".to_string(),
                qualification_effects:
                    "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                        .to_string(),
                constraint_ledger_artifact: String::new(),
                regionalizer_action: "include-in-regional-treatment".to_string(),
                validation_status: "pass".to_string(),
            },
            T2RegionalizerRow {
                tier: "T2".to_string(),
                region_id: "component-1".to_string(),
                component_id: 1,
                route: "US30".to_string(),
                parent_trunks: "I80;I84".to_string(),
                route_miles: 600.0,
                column_decision: "review".to_string(),
                treatment_status: "review-treatment".to_string(),
                evidence_status: "review".to_string(),
                pavement_debt_cost_m: 5.85,
                pavement_debt_class: "evidence-debt".to_string(),
                pavement_debt_basis: "fixture pavement debt".to_string(),
                hard_blocker_count: 0,
                claim_blocker_count: 0,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 0.0,
                top_constraint_classes: "none".to_string(),
                qualification_effects:
                    "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                        .to_string(),
                constraint_ledger_artifact: String::new(),
                regionalizer_action: "hold-for-parent-region-review".to_string(),
                validation_status: "review".to_string(),
            },
            T2RegionalizerRow {
                tier: "T2".to_string(),
                region_id: "component-1".to_string(),
                component_id: 1,
                route: "I285".to_string(),
                parent_trunks: String::new(),
                route_miles: 172.0,
                column_decision: "review".to_string(),
                treatment_status: "review-treatment".to_string(),
                evidence_status: "closure-accepted".to_string(),
                pavement_debt_cost_m: 0.0,
                pavement_debt_class: "none".to_string(),
                pavement_debt_basis: "no pavement debt row joined".to_string(),
                hard_blocker_count: 0,
                claim_blocker_count: 0,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 0.0,
                top_constraint_classes: "none".to_string(),
                qualification_effects: String::new(),
                constraint_ledger_artifact: String::new(),
                regionalizer_action: "hold-for-parent-region-review".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t2_service_selection_rows(&regionalizer, &route_map::beck_t2_diagnostics());
        let failures = t2_service_selection_gate_failures(&rows);

        assert_eq!(rows.len(), 3);
        assert_eq!(rows[0].beck_corridor, "I-15");
        assert_eq!(rows[0].selection_action, "keep-service-column");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[1].selection_action, "parent-region-review");
        assert_eq!(rows[2].beck_corridor, "I-285");
        assert_eq!(rows[2].beck_service_class, "transfer-spine");
        assert_eq!(rows[2].selection_action, "parent-region-review");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_service_selection_allows_selected_rows_to_queue_missing_beck_diagnostics() {
        let regionalizer = vec![T2RegionalizerRow {
            tier: "T2".to_string(),
            region_id: "component-1".to_string(),
            component_id: 1,
            route: "I195".to_string(),
            parent_trunks: "I95".to_string(),
            route_miles: 184.0,
            column_decision: "selected".to_string(),
            treatment_status: "selected-treatment".to_string(),
            evidence_status: "accepted".to_string(),
            pavement_debt_cost_m: 0.0,
            pavement_debt_class: "none".to_string(),
            pavement_debt_basis: "no pavement debt row joined".to_string(),
            hard_blocker_count: 0,
            claim_blocker_count: 0,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 0.0,
            top_constraint_classes: "none".to_string(),
            qualification_effects: String::new(),
            constraint_ledger_artifact: String::new(),
            regionalizer_action: "include-in-regional-treatment".to_string(),
            validation_status: "pass".to_string(),
        }];

        let rows = t2_service_selection_rows(&regionalizer, &[]);
        let failures = t2_service_selection_gate_failures(&rows);

        assert_eq!(rows[0].selection_action, "source-needed");
        assert_eq!(rows[0].selection_basis, "missing-beck-t2-diagnostic");
        assert!(failures.is_empty());
    }

    #[test]
    fn t2_bundle_overlays_use_registry_and_mark_pending_bindings() {
        let service_rows = vec![
            T2ServiceSelectionRow {
                tier: "T2".to_string(),
                region_id: "component-1".to_string(),
                route: "I-15".to_string(),
                parent_trunks: "I5;I70".to_string(),
                column_decision: "selected".to_string(),
                treatment_status: "selected-treatment".to_string(),
                beck_corridor: "I-15".to_string(),
                beck_service_class: "connector".to_string(),
                beck_color_mode: "split-parent".to_string(),
                beck_start_trunk: "I-5".to_string(),
                beck_end_trunk: "I-70".to_string(),
                duplicate_service_count: 0,
                duplicate_service_corridors: String::new(),
                close_parallel_count: 0,
                close_parallel_corridors: String::new(),
                unstopped_t1_contact_count: 0,
                unstopped_t1_contacts: String::new(),
                pavement_debt_cost_m: 12.5,
                pavement_debt_class: "repair-debt".to_string(),
                pavement_debt_basis: "fixture repair payment pressure".to_string(),
                hard_blocker_count: 0,
                claim_blocker_count: 0,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 0.0,
                top_constraint_classes: "none".to_string(),
                qualification_effects: "qualification_gate_policy=stop-first".to_string(),
                constraint_ledger_artifact: String::new(),
                beck_service_action: "keep".to_string(),
                qualification_basis: "distinct-parent-service".to_string(),
                qualification_map_treatment: "draw as normal T2 service for its class".to_string(),
                qualification_gate_policy: "accepted when structural diagnostics pass".to_string(),
                qualification_game_use:
                    "default playable service for incidents, upgrades, and restitches".to_string(),
                selection_action: "keep-service-column".to_string(),
                selection_basis: "diagnostic-backed-distinct-service".to_string(),
                validation_status: "pass".to_string(),
            },
            T2ServiceSelectionRow {
                tier: "T2".to_string(),
                region_id: "component-1".to_string(),
                route: "I-29".to_string(),
                parent_trunks: "I80;I35".to_string(),
                column_decision: "review".to_string(),
                treatment_status: "review-treatment".to_string(),
                beck_corridor: "I-29".to_string(),
                beck_service_class: "connector".to_string(),
                beck_color_mode: "split-parent".to_string(),
                beck_start_trunk: "I-80".to_string(),
                beck_end_trunk: "I-35".to_string(),
                duplicate_service_count: 0,
                duplicate_service_corridors: String::new(),
                close_parallel_count: 0,
                close_parallel_corridors: String::new(),
                unstopped_t1_contact_count: 0,
                unstopped_t1_contacts: String::new(),
                pavement_debt_cost_m: 0.0,
                pavement_debt_class: "none".to_string(),
                pavement_debt_basis: "no pavement debt row joined".to_string(),
                hard_blocker_count: 0,
                claim_blocker_count: 0,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 0.0,
                top_constraint_classes: "none".to_string(),
                qualification_effects: String::new(),
                constraint_ledger_artifact: String::new(),
                beck_service_action: "keep".to_string(),
                qualification_basis: "distinct-parent-service".to_string(),
                qualification_map_treatment: "draw as normal T2 service for its class".to_string(),
                qualification_gate_policy: "accepted when structural diagnostics pass".to_string(),
                qualification_game_use:
                    "default playable service for incidents, upgrades, and restitches".to_string(),
                selection_action: "parent-region-review".to_string(),
                selection_basis: "regionalizer-review-treatment".to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let bundles = vec![NationalSegmentBundleRow {
            segment_bundle_id: "US.HWYBUNDLE.I15".to_string(),
            bundle_role: "single-segment".to_string(),
            member_segment_ids: "US.HWYSEG.I15".to_string(),
            member_count: 1,
            stitch_group_ids: "US.HWYSTITCH.I15".to_string(),
            current_tiers: "T2".to_string(),
            current_zone_ids: "component-1".to_string(),
            route_labels: "I15".to_string(),
            state_scope: "CA;NV;UT".to_string(),
            evidence_state_scope: "CA;NV;UT".to_string(),
            geometry_state_scope: String::new(),
            bundle_aliases: "route:I15".to_string(),
            source_artifacts: "fixture".to_string(),
            bundle_status: "bundle-ready".to_string(),
            bundle_action: "use bundle as service join surface".to_string(),
            qualification_effects: "qualification_game_use=default-play".to_string(),
            next_artifact: "maps/t3-zone".to_string(),
            validation_status: "pass".to_string(),
        }];
        let overlays = vec![GameT2ServiceOverlayRow {
            service_class: "connector".to_string(),
            map_id: "beck-schematic-t2-only".to_string(),
            scenario_hook: "T2 bridge between parent trunks".to_string(),
            incident_lever: "reroute pressure".to_string(),
            upgrade_lever: "upgrade to transfer-spine".to_string(),
            restitch_lever: "restitch to nearest transfer stop".to_string(),
            release_gate: "beck-t2-service-standards gate".to_string(),
        }];

        let rows = t2_bundle_overlay_rows(&service_rows, &bundles, &overlays);
        let failures = t2_bundle_overlay_gate_failures(&rows);

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].segment_bundle_id, "US.HWYBUNDLE.I15");
        assert_eq!(rows[0].binding_status, "bundle-bound");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].pavement_debt_cost_m, 12.5);
        assert_eq!(rows[0].pavement_debt_class, "repair-debt");
        assert_eq!(rows[1].binding_status, "bundle-binding-pending");
        assert!(failures.is_empty());
    }

    #[test]
    fn tier_segment_candidates_decompose_selected_services_into_edges() {
        let mut graph = HighwayGraph::new();
        let a = graph.graph.add_node(HighwayNode {
            id: 1,
            coord: coord! { x: 0.0, y: 0.0 },
            is_interchange: false,
        });
        let b = graph.graph.add_node(HighwayNode {
            id: 2,
            coord: coord! { x: 1.0, y: 0.0 },
            is_interchange: true,
        });
        let c = graph.graph.add_node(HighwayNode {
            id: 3,
            coord: coord! { x: 2.0, y: 0.0 },
            is_interchange: false,
        });
        let e1 = graph.graph.add_edge(
            a,
            b,
            HighwayEdge {
                id: 10,
                route_id: "I80".to_string(),
                state: "NE".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: 0.0, y: 0.0 },
                    coord! { x: 1.0, y: 0.0 },
                ]),
                length_miles: 50.0,
                lane_count: Some(4),
                aadt: Some(40_000),
                pct_truck: None,
                iri: None,
                tti: None,
                pti: None,
                speed_limit: None,
            },
        );
        let e2 = graph.graph.add_edge(
            b,
            c,
            HighwayEdge {
                id: 11,
                route_id: "I80".to_string(),
                state: "IA".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: 1.0, y: 0.0 },
                    coord! { x: 2.0, y: 0.0 },
                ]),
                length_miles: 60.0,
                lane_count: Some(4),
                aadt: Some(45_000),
                pct_truck: None,
                iri: None,
                tti: None,
                pti: None,
                speed_limit: None,
            },
        );
        graph.route_index.insert("I80".to_string(), vec![e2, e1]);

        let t1_rows = vec![T1LineSelectorInputRow {
            route: "I-80".to_string(),
            selected: true,
            selected_stops: "STOP-OMAHA;STOP-DESMOINES".to_string(),
        }];
        let t2_rows = Vec::new();
        let repair_rows = Vec::new();
        let rows = crate::support::tier::tier_segment_candidate_rows::tier_segment_candidate_rows(&graph, &t1_rows, &t2_rows, &repair_rows, &[]);
        let failures =
            tier_segment_candidate_gate_failures(&rows, &t1_rows, &t2_rows, &repair_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].edge_id, 10);
        assert_eq!(rows[1].edge_id, 11);
        assert_ne!(rows[0].national_segment_id, rows[1].national_segment_id);
        assert_eq!(rows[0].segment_bundle_id, rows[1].segment_bundle_id);
        assert_eq!(rows[0].member_role, "stitched-member");
    }

    #[test]
    fn tier_segment_candidates_preserve_repair_queue_qualification_effects() {
        let mut graph = HighwayGraph::new();
        let a = graph.graph.add_node(HighwayNode {
            id: 1,
            coord: coord! { x: 0.0, y: 0.0 },
            is_interchange: false,
        });
        let b = graph.graph.add_node(HighwayNode {
            id: 2,
            coord: coord! { x: 1.0, y: 0.0 },
            is_interchange: true,
        });
        let edge = graph.graph.add_edge(
            a,
            b,
            HighwayEdge {
                id: 20,
                route_id: "I285".to_string(),
                state: "GA".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: 0.0, y: 0.0 },
                    coord! { x: 1.0, y: 0.0 },
                ]),
                length_miles: 64.0,
                lane_count: Some(6),
                aadt: Some(120_000),
                pct_truck: None,
                iri: None,
                tti: None,
                pti: None,
                speed_limit: None,
            },
        );
        graph.route_index.insert("I285".to_string(), vec![edge]);
        let repair_rows = vec![T2BundleRepairQueueRow {
            route: "I285".to_string(),
            segment_bundle_id: String::new(),
            bundle_status: "bundle-missing".to_string(),
            bundle_action: "resolve route family or add segment bundle".to_string(),
            contact_evidence_status: "closure-bundle-pending".to_string(),
            candidate_decision: "blocked".to_string(),
            repair_class: "relief-contact-repair".to_string(),
            repair_action: "add-or-split-segment-bundle-before-regionalizer".to_string(),
            required_artifact: "data/t2-blocker-closure.csv".to_string(),
            next_artifact: "data/national-segment-bundles.csv".to_string(),
            optimizer_effect: "I285 remains out of T2 regionalizer until bundle-missing"
                .to_string(),
            qualification_effects: "qualification_gate_policy=stop-first".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = crate::support::tier::tier_segment_candidate_rows::tier_segment_candidate_rows(&graph, &[], &[], &repair_rows, &[]);
        let failures = tier_segment_candidate_gate_failures(&rows, &[], &[], &repair_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].source_selector, "t2-bundle-repair-queue");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_gate_policy=stop-first"
        );
    }

    #[test]
    fn tier_segment_candidates_split_numbered_t2_families_by_state() {
        let mut graph = HighwayGraph::new();
        let a = graph.graph.add_node(HighwayNode {
            id: 1,
            coord: coord! { x: 0.0, y: 0.0 },
            is_interchange: false,
        });
        let b = graph.graph.add_node(HighwayNode {
            id: 2,
            coord: coord! { x: 1.0, y: 0.0 },
            is_interchange: false,
        });
        let c = graph.graph.add_node(HighwayNode {
            id: 3,
            coord: coord! { x: 2.0, y: 0.0 },
            is_interchange: false,
        });
        let e1 = graph.graph.add_edge(
            a,
            b,
            HighwayEdge {
                id: 20,
                route_id: "I295".to_string(),
                state: "NJ".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: 0.0, y: 0.0 },
                    coord! { x: 1.0, y: 0.0 },
                ]),
                length_miles: 10.0,
                lane_count: Some(4),
                aadt: Some(40_000),
                pct_truck: None,
                iri: None,
                tti: None,
                pti: None,
                speed_limit: None,
            },
        );
        let e2 = graph.graph.add_edge(
            b,
            c,
            HighwayEdge {
                id: 21,
                route_id: "I295".to_string(),
                state: "ME".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: 1.0, y: 0.0 },
                    coord! { x: 2.0, y: 0.0 },
                ]),
                length_miles: 20.0,
                lane_count: Some(4),
                aadt: Some(45_000),
                pct_truck: None,
                iri: None,
                tti: None,
                pti: None,
                speed_limit: None,
            },
        );
        graph.route_index.insert("I295".to_string(), vec![e1, e2]);

        let t2_rows = vec![T2ServiceSelectionRow {
            tier: "T2".to_string(),
            region_id: "component-0".to_string(),
            route: "I295".to_string(),
            parent_trunks: "I95".to_string(),
            column_decision: "selected".to_string(),
            treatment_status: "selected-treatment".to_string(),
            beck_corridor: String::new(),
            beck_service_class: String::new(),
            beck_color_mode: String::new(),
            beck_start_trunk: String::new(),
            beck_end_trunk: String::new(),
            duplicate_service_count: 0,
            duplicate_service_corridors: String::new(),
            close_parallel_count: 0,
            close_parallel_corridors: String::new(),
            unstopped_t1_contact_count: 0,
            unstopped_t1_contacts: String::new(),
            pavement_debt_cost_m: 0.0,
            pavement_debt_class: "none".to_string(),
            pavement_debt_basis: "no pavement debt row joined".to_string(),
            hard_blocker_count: 0,
            claim_blocker_count: 0,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 0.0,
            top_constraint_classes: "none".to_string(),
            qualification_effects: String::new(),
            constraint_ledger_artifact: String::new(),
            beck_service_action: String::new(),
            qualification_basis: String::new(),
            qualification_map_treatment: String::new(),
            qualification_gate_policy: String::new(),
            qualification_game_use: String::new(),
            selection_action: "source-needed".to_string(),
            selection_basis: "missing-beck-t2-diagnostic".to_string(),
            validation_status: "review".to_string(),
        }];
        let route_family_rows = vec![T2RouteFamilySplitRow {
            route: "I295".to_string(),
            endpoint_name: String::new(),
            endpoint_role: "service_diagnostic_route_family".to_string(),
            exception_type: "route-family-diagnostic-split-needed".to_string(),
            source_artifact: "data/t2-service-diagnostic-queue.csv".to_string(),
            family_action: "split-numbered-service-family".to_string(),
            disposition: "blocked".to_string(),
            required_evidence: "represented segment family".to_string(),
            next_artifact: "data/national-segment-bundles.csv".to_string(),
            qualification_effects: String::new(),
            optimizer_effect: "split before rendering".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = crate::support::tier::tier_segment_candidate_rows::tier_segment_candidate_rows(&graph, &[], &t2_rows, &[], &route_family_rows);
        let bundle_ids = rows
            .iter()
            .map(|row| row.segment_bundle_id.as_str())
            .collect::<std::collections::BTreeSet<_>>();

        assert_eq!(rows.len(), 2);
        assert_eq!(bundle_ids.len(), 2);
        assert!(rows[0].route_aliases.contains("route-family-scope:NJ"));
        assert!(rows[1].route_aliases.contains("route-family-scope:ME"));
    }

    #[test]
    fn t2_game_ops_binding_intake_filters_constraint_budget() {
        let budget_rows = vec![
            OptimizerConstraintBudgetRow {
                budget_id: "CB-T2-BUNDLE-1".to_string(),
                optimizer_run_id: "tier-optimizer-current".to_string(),
                tier: "T2".to_string(),
                region_id: "component-0".to_string(),
                subject_scope: "bundle".to_string(),
                subject_id: "US.HWYBUNDLE.TEST".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.TEST".to_string(),
                route: "I225".to_string(),
                ledger_row_count: 1,
                hard_blocker_count: 0,
                claim_blocker_count: 1,
                review_count: 1,
                budget_debt_count: 0,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 1.0,
                top_constraint_classes: "game_ops_bundle_binding".to_string(),
                blocking_claims: "game;incident;publication;upgrade".to_string(),
                qualification_effects: String::new(),
                next_artifacts: "data/game/t2-service-overlays.csv".to_string(),
                constraint_ledger_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
                validation_status: "review".to_string(),
            },
            OptimizerConstraintBudgetRow {
                budget_id: "CB-T3-OTHER".to_string(),
                optimizer_run_id: "tier-optimizer-current".to_string(),
                tier: "T3".to_string(),
                region_id: "component-0".to_string(),
                subject_scope: "route".to_string(),
                subject_id: "US30".to_string(),
                segment_bundle_id: String::new(),
                route: "US30".to_string(),
                ledger_row_count: 1,
                hard_blocker_count: 0,
                claim_blocker_count: 1,
                review_count: 1,
                budget_debt_count: 0,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 1.0,
                top_constraint_classes: "terminal_access_evidence_gap".to_string(),
                blocking_claims: "upgrade".to_string(),
                qualification_effects: String::new(),
                next_artifacts: "data/t3-t4-access-gaps.csv".to_string(),
                constraint_ledger_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
                validation_status: "review".to_string(),
            },
            OptimizerConstraintBudgetRow {
                budget_id: "CB-T2-BUNDLE-2".to_string(),
                optimizer_run_id: "tier-optimizer-current".to_string(),
                tier: "T2".to_string(),
                region_id: "component-0".to_string(),
                subject_scope: "bundle".to_string(),
                subject_id: "US.HWYBUNDLE.MIXED".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.MIXED".to_string(),
                route: "I110".to_string(),
                ledger_row_count: 2,
                hard_blocker_count: 0,
                claim_blocker_count: 1,
                review_count: 2,
                budget_debt_count: 1,
                constraint_debt_cost_m: 5.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 6.0,
                top_constraint_classes: "asset_condition_debt|game_ops_bundle_binding".to_string(),
                blocking_claims: "game;incident;publication;upgrade".to_string(),
                qualification_effects: String::new(),
                next_artifacts: "data/game/t2-service-overlays.csv;data/tier-pavement-docket.csv"
                    .to_string(),
                constraint_ledger_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t2_game_ops_binding_intake_rows(&budget_rows);
        let failures = t2_game_ops_binding_intake_gate_failures(&rows, &budget_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].route, "I110");
        assert_eq!(rows[1].route, "I225");
        assert_eq!(rows[0].intake_status, "decision-needed");
    }

    #[test]
    fn t2_game_ops_binding_decisions_preserve_residual_blockers() {
        let intake_rows = vec![T2GameOpsBindingIntakeRow {
            intake_id: "T2GAMEOPSINTAKE-1".to_string(),
            budget_id: "CB-T2-BUNDLE-1".to_string(),
            subject_id: "US.HWYBUNDLE.TEST".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.TEST".to_string(),
            route: "I225".to_string(),
            claim_blocker_count: 1,
            blocked_claims: "game;incident;publication;upgrade".to_string(),
            top_constraint_classes: "game_ops_bundle_binding".to_string(),
            qualification_effects: "qualification_gate_policy=stop-first".to_string(),
            next_artifacts: "data/game/t2-service-overlays.csv".to_string(),
            constraint_ledger_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            intake_status: "decision-needed".to_string(),
            decision_artifact: "data/t2-game-ops-binding-decisions.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let overlay_rows = vec![T2BundleOverlayRow {
            tier: "T2".to_string(),
            region_id: "component-0".to_string(),
            route: "I225".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.TEST".to_string(),
            bundle_status: "review".to_string(),
            service_class: "connector".to_string(),
            map_id: "beck-schematic-t2-only".to_string(),
            scenario_hook: "T2 bridge between parent trunks".to_string(),
            incident_lever: "reroute pressure".to_string(),
            upgrade_lever: "upgrade".to_string(),
            restitch_lever: "restitch".to_string(),
            release_gate: "gate".to_string(),
            qualification_map_treatment: "draw as normal T2 service for its class".to_string(),
            qualification_gate_policy: "accepted when structural diagnostics pass".to_string(),
            qualification_game_use:
                "default playable service for incidents, upgrades, and restitches".to_string(),
            qualification_effects: "qualification_game_use=default-play".to_string(),
            pavement_debt_cost_m: 0.0,
            pavement_debt_class: String::new(),
            pavement_debt_basis: String::new(),
            source_artifacts: "data/t2-service-selection.csv".to_string(),
            binding_status: "bundle-bound-review".to_string(),
            next_artifact: "data/national-segment-bundles.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_game_ops_binding_decision_rows(&intake_rows, &overlay_rows);
        let failures = t2_game_ops_binding_decision_gate_failures(&rows, &intake_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].decision, "repair-needed");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].validation_status, "review");
        assert!(rows[0].blocks_claims.contains("publication"));
    }

    #[test]
    fn t2_bundle_overlay_repair_targets_classify_residual_decisions() {
        let decision_rows = vec![T2GameOpsBindingDecisionRow {
            decision_id: "T2GAMEOPSDECISION-1".to_string(),
            intake_id: "T2GAMEOPSINTAKE-1".to_string(),
            subject_id: "US.HWYBUNDLE.TEST".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.TEST".to_string(),
            route: "I37".to_string(),
            service_class: "compact-service".to_string(),
            bundle_status: "needs-stop-chain".to_string(),
            binding_status: "bundle-bound-review".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            qualification_gate_policy: "accepted when structural diagnostics pass".to_string(),
            qualification_game_use:
                "default playable service for incidents, upgrades, and restitches".to_string(),
            decision: "repair-needed".to_string(),
            decision_reason: "bundle id exists but validation remains under review".to_string(),
            blocks_claims: "game;incident;publication;upgrade".to_string(),
            next_artifact: "data/national-segment-bundles.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let overlay_rows = vec![T2BundleOverlayRow {
            tier: "T2".to_string(),
            region_id: "component-0".to_string(),
            route: "I37".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.TEST".to_string(),
            bundle_status: "needs-stop-chain".to_string(),
            service_class: "compact-service".to_string(),
            map_id: "beck-schematic-t2-only".to_string(),
            scenario_hook: "short local service".to_string(),
            incident_lever: "localized disruption".to_string(),
            upgrade_lever: "upgrade stop spacing".to_string(),
            restitch_lever: "restitch inside local cluster".to_string(),
            release_gate: "gate".to_string(),
            qualification_map_treatment: "draw as normal T2 service for its class".to_string(),
            qualification_gate_policy: "accepted when structural diagnostics pass".to_string(),
            qualification_game_use:
                "default playable service for incidents, upgrades, and restitches".to_string(),
            qualification_effects: String::new(),
            pavement_debt_cost_m: 0.0,
            pavement_debt_class: "none".to_string(),
            pavement_debt_basis: "no pavement debt row joined".to_string(),
            source_artifacts: "data/t2-service-selection.csv".to_string(),
            binding_status: "bundle-bound-review".to_string(),
            next_artifact: "data/national-segment-bundles.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_bundle_overlay_repair_target_rows(&decision_rows, &overlay_rows);
        let failures = t2_bundle_overlay_repair_target_gate_failures(&rows, &decision_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].repair_class, "stop-chain");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].target_status, "repair-needed");
        assert_eq!(rows[0].next_artifact, "data/national-segment-bundles.csv");
    }

    #[test]
    fn t2_service_class_repair_docket_routes_local_zone_holds() {
        let decision_rows = vec![T2GameOpsBindingDecisionRow {
            decision_id: "T2GAMEOPSDECISION-2".to_string(),
            intake_id: "T2GAMEOPSINTAKE-2".to_string(),
            subject_id: "US.HWYBUNDLE.LOCAL".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.LOCAL".to_string(),
            route: "I225".to_string(),
            service_class: "unclassified".to_string(),
            bundle_status: "bundle-ready".to_string(),
            binding_status: "service-class-held-known".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            qualification_gate_policy: "accepted when structural diagnostics pass".to_string(),
            qualification_game_use:
                "default playable service for incidents, upgrades, and restitches".to_string(),
            decision: "held".to_string(),
            decision_reason: "service class overlay is missing or held".to_string(),
            blocks_claims: "game;incident;publication;upgrade".to_string(),
            next_artifact: "data/game/t2-service-overlays.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let target_rows = t2_bundle_overlay_repair_target_rows(&decision_rows, &[]);
        let diagnostic_rows = vec![T2ServiceDiagnosticQueueRow {
            route: "I225".to_string(),
            region_id: "component-0".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.LOCAL".to_string(),
            bundle_status: "bundle-ready".to_string(),
            selection_action: "source-needed".to_string(),
            selection_basis: "missing-beck-t2-diagnostic".to_string(),
            qualification_effects: String::new(),
            diagnostic_status: "local-relief-map-review".to_string(),
            service_diagnostic_action: "local-relief-map-review".to_string(),
            required_artifact: "data/t2-service-selection.csv".to_string(),
            next_artifact: "data/t3-t4-pressure-intake.csv".to_string(),
            optimizer_effect: "keeps review-treatment relief service below map/game overlay"
                .to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_service_class_repair_docket_rows(&target_rows, &diagnostic_rows);
        let failures = t2_service_class_repair_docket_gate_failures(&rows, &target_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].service_repair_class, "local-zone");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert!(rows[0]
            .optimizer_effect
            .contains("qualification_game_use=default-play"));
        assert_eq!(rows[0].next_artifact, "data/t3-zone-render-board.csv");
    }

    #[test]
    fn t2_game_ops_bundle_evidence_review_preserves_bound_blockers() {
        let decision_rows = vec![T2GameOpsBindingDecisionRow {
            decision_id: "T2GAMEOPSDECISION-2".to_string(),
            intake_id: "T2GAMEOPSINTAKE-2".to_string(),
            subject_id: "US.HWYBUNDLE.LOCAL".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.LOCAL".to_string(),
            route: "I225".to_string(),
            service_class: "unclassified".to_string(),
            bundle_status: "bundle-ready".to_string(),
            binding_status: "service-class-held-known".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            qualification_gate_policy: "accepted when structural diagnostics pass".to_string(),
            qualification_game_use:
                "default playable service for incidents, upgrades, and restitches".to_string(),
            decision: "held".to_string(),
            decision_reason: "service class overlay is missing or held".to_string(),
            blocks_claims: "game;incident;publication;upgrade".to_string(),
            next_artifact: "data/game/t2-service-overlays.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let target_rows = t2_bundle_overlay_repair_target_rows(&decision_rows, &[]);
        let diagnostic_rows = vec![T2ServiceDiagnosticQueueRow {
            route: "I225".to_string(),
            region_id: "component-0".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.LOCAL".to_string(),
            bundle_status: "bundle-ready".to_string(),
            selection_action: "source-needed".to_string(),
            selection_basis: "missing-beck-t2-diagnostic".to_string(),
            qualification_effects: String::new(),
            diagnostic_status: "local-relief-map-review".to_string(),
            service_diagnostic_action: "local-relief-map-review".to_string(),
            required_artifact: "data/t2-service-selection.csv".to_string(),
            next_artifact: "data/t3-t4-pressure-intake.csv".to_string(),
            optimizer_effect: "keeps review-treatment relief service below map/game overlay"
                .to_string(),
            validation_status: "review".to_string(),
        }];
        let docket_rows = t2_service_class_repair_docket_rows(&target_rows, &diagnostic_rows);

        let rows =
            t2_game_ops_bundle_evidence_review_rows(&decision_rows, &target_rows, &docket_rows);
        let failures = t2_game_ops_bundle_evidence_review_gate_failures(&rows, &decision_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].evidence_artifact, "data/t3-t4-pressure-intake.csv");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].blocker_claims_before, rows[0].blocker_claims_after);
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn t2_game_ops_bundle_evidence_policy_preserves_review_blockers() {
        let review_rows = vec![T2GameOpsBundleEvidenceReviewRow {
            review_id: "T2GAMEOPSBUNDLEEVIDENCEREVIEW-I110".to_string(),
            decision_id: "T2GAMEOPSDECISION-I110".to_string(),
            target_id: "T2OVERLAYREPAIR-I110".to_string(),
            route: "I110".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I110".to_string(),
            decision: "held".to_string(),
            binding_status: "service-class-held-known".to_string(),
            bundle_status: "bundle-ready".to_string(),
            service_class: "unclassified".to_string(),
            repair_class: "service-class".to_string(),
            repair_action: "repair-service-class-before-game-overlay".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            qualification_gate_policy: "accepted when structural diagnostics pass".to_string(),
            qualification_game_use:
                "default playable service for incidents, upgrades, and restitches".to_string(),
            evidence_artifact: "data/t3-t4-pressure-intake.csv".to_string(),
            service_repair_class: "local-zone".to_string(),
            evidence_status: "downstream-evidence-bound-blocker-preserved".to_string(),
            blocker_claims_before: "game;incident;publication;sla;transit;upgrade".to_string(),
            blocker_claims_after: "game;incident;publication;sla;transit;upgrade".to_string(),
            blocker_count_before: 6,
            blocker_count_after: 6,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-game-ops-bundle-evidence-policy.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_game_ops_bundle_evidence_policy_rows(&review_rows);
        let failures = t2_game_ops_bundle_evidence_policy_gate_failures(&rows, &review_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(
            rows[0].required_evidence,
            "accepted-local-zone-overlay-handoff"
        );
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(
            rows[0].blocker_claims_before,
            "game;incident;publication;sla;transit;upgrade"
        );
        assert_eq!(rows[0].claim_blocker_delta, 0);
        assert_eq!(
            rows[0].next_artifact,
            "data/t2-game-ops-bundle-evidence-policy-acceptance.csv"
        );
    }

    #[test]
    fn t2_game_ops_bundle_evidence_policy_acceptance_preserves_policy_blockers() {
        let policy_rows = vec![T2GameOpsBundleEvidencePolicyRow {
            policy_id: "T2GAMEOPSBUNDLEPOLICY-I110".to_string(),
            review_id: "T2GAMEOPSBUNDLEEVIDENCEREVIEW-I110".to_string(),
            decision_id: "T2GAMEOPSDECISION-I110".to_string(),
            target_id: "T2OVERLAYREPAIR-I110".to_string(),
            route: "I110".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I110".to_string(),
            repair_class: "service-class".to_string(),
            service_repair_class: "local-zone".to_string(),
            evidence_artifact: "data/t3-t4-pressure-intake.csv".to_string(),
            qualification_effects: "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                .to_string(),
            qualification_gate_policy: "accepted when structural diagnostics pass".to_string(),
            qualification_game_use:
                "default playable service for incidents, upgrades, and restitches".to_string(),
            required_evidence: "accepted-local-zone-overlay-handoff".to_string(),
            evidence_policy_decision: "bundle-evidence-policy-authored-review".to_string(),
            policy_treatment:
                "hold game/ops claims until local-zone overlay handoff is accepted or explicitly carried"
                    .to_string(),
            blocker_claims_before: "game;incident;publication;sla;transit;upgrade".to_string(),
            blocker_claims_after: "game;incident;publication;sla;transit;upgrade".to_string(),
            blocker_count_before: 6,
            blocker_count_after: 6,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-game-ops-bundle-evidence-policy-acceptance.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_game_ops_bundle_evidence_policy_acceptance_rows(&policy_rows);
        let failures =
            t2_game_ops_bundle_evidence_policy_acceptance_gate_failures(&rows, &policy_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(
            rows[0].accepted_required_evidence,
            "accepted-local-zone-overlay-handoff"
        );
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(
            rows[0].blocker_claims_before,
            "game;incident;publication;sla;transit;upgrade"
        );
        assert_eq!(rows[0].claim_blocker_delta, 0);
        assert_eq!(
            rows[0].next_artifact,
            "data/t2-game-ops-bundle-evidence-blocker-relief.csv"
        );
    }

    #[test]
    fn t2_game_ops_bundle_evidence_blocker_relief_reduces_accepted_blockers() {
        let acceptance_rows = vec![T2GameOpsBundleEvidencePolicyAcceptanceRow {
            acceptance_id: "T2GAMEOPSBUNDLEACCEPT-I110".to_string(),
            policy_id: "T2GAMEOPSBUNDLEPOLICY-I110".to_string(),
            review_id: "T2GAMEOPSBUNDLEEVIDENCEREVIEW-I110".to_string(),
            decision_id: "T2GAMEOPSDECISION-I110".to_string(),
            target_id: "T2OVERLAYREPAIR-I110".to_string(),
            route: "I110".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I110".to_string(),
            accepted_required_evidence: "accepted-local-zone-overlay-handoff".to_string(),
            accepted_policy_treatment:
                "hold game/ops claims until local-zone overlay handoff is accepted or explicitly carried"
                    .to_string(),
            qualification_effects: "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                .to_string(),
            qualification_gate_policy: "accepted when structural diagnostics pass".to_string(),
            qualification_game_use:
                "default playable service for incidents, upgrades, and restitches".to_string(),
            acceptance_decision: "bundle-evidence-policy-accepted".to_string(),
            blocker_claims_before: "game;incident;publication;sla;transit;upgrade".to_string(),
            blocker_claims_after: "game;incident;publication;sla;transit;upgrade".to_string(),
            blocker_count_before: 6,
            blocker_count_after: 6,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-game-ops-bundle-evidence-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_game_ops_bundle_evidence_blocker_relief_rows(&acceptance_rows);
        let failures =
            t2_game_ops_bundle_evidence_blocker_relief_gate_failures(&rows, &acceptance_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(
            rows[0].blocker_claims_before,
            "game;incident;publication;sla;transit;upgrade"
        );
        assert_eq!(rows[0].blocker_claims_after, "");
        assert_eq!(rows[0].blocker_count_after, 0);
        assert_eq!(rows[0].claim_blocker_delta, -6);
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(
            rows[0].next_artifact,
            "data/optimizer-constraint-ledger.csv"
        );
    }

    #[test]
    fn t2_service_overlay_diagnostic_decisions_hold_unclassified_rows() {
        let decision_rows = vec![T2GameOpsBindingDecisionRow {
            decision_id: "T2GAMEOPSDECISION-I195".to_string(),
            intake_id: "T2GAMEOPSINTAKE-I195".to_string(),
            subject_id: "US.HWYBUNDLE.I195".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I195".to_string(),
            route: "I195".to_string(),
            service_class: "unclassified".to_string(),
            bundle_status: "bundle-ready".to_string(),
            binding_status: "service-class-held-known".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            qualification_gate_policy: "accepted when structural diagnostics pass".to_string(),
            qualification_game_use:
                "default playable service for incidents, upgrades, and restitches".to_string(),
            decision: "held".to_string(),
            decision_reason: "service class overlay is missing or held".to_string(),
            blocks_claims: "game;incident;publication;upgrade".to_string(),
            next_artifact: "data/game/t2-service-overlays.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let target_rows = t2_bundle_overlay_repair_target_rows(&decision_rows, &[]);
        let diagnostic_rows = vec![T2ServiceDiagnosticQueueRow {
            route: "I195".to_string(),
            region_id: "component-1".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I195".to_string(),
            bundle_status: "bundle-ready".to_string(),
            selection_action: "source-needed".to_string(),
            selection_basis: "missing-beck-t2-diagnostic".to_string(),
            qualification_effects: String::new(),
            diagnostic_status: "beck-diagnostic-missing".to_string(),
            service_diagnostic_action: "beck-diagnostic-missing".to_string(),
            required_artifact: "data/t2-service-selection.csv".to_string(),
            next_artifact: "data/beck-t2-diagnostics.csv".to_string(),
            optimizer_effect: "keeps unclassified service overlay out of claim surfaces"
                .to_string(),
            validation_status: "review".to_string(),
        }];
        let docket_rows = t2_service_class_repair_docket_rows(&target_rows, &diagnostic_rows);

        let rows = t2_service_overlay_diagnostic_decision_rows(
            &docket_rows,
            &target_rows,
            &diagnostic_rows,
        );
        let failures = t2_service_overlay_diagnostic_decision_gate_failures(&rows, &docket_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].overlay_decision, "held");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].required_artifact, "data/beck-t2-diagnostics.csv");
        assert_eq!(rows[0].blocks_claims, "game;incident;publication;upgrade");
    }

    #[test]
    fn t2_local_zone_overlay_handoff_keeps_zone_rows_held() {
        let decision_rows = vec![T2GameOpsBindingDecisionRow {
            decision_id: "T2GAMEOPSDECISION-I225".to_string(),
            intake_id: "T2GAMEOPSINTAKE-I225".to_string(),
            subject_id: "US.HWYBUNDLE.I225".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I225".to_string(),
            route: "I225".to_string(),
            service_class: "unclassified".to_string(),
            bundle_status: "bundle-ready".to_string(),
            binding_status: "service-class-held-known".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            qualification_gate_policy: "accepted when structural diagnostics pass".to_string(),
            qualification_game_use:
                "default playable service for incidents, upgrades, and restitches".to_string(),
            decision: "held".to_string(),
            decision_reason: "service class overlay is missing or held".to_string(),
            blocks_claims: "game;incident;publication;upgrade".to_string(),
            next_artifact: "data/game/t2-service-overlays.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let target_rows = t2_bundle_overlay_repair_target_rows(&decision_rows, &[]);
        let diagnostic_rows = vec![T2ServiceDiagnosticQueueRow {
            route: "I225".to_string(),
            region_id: "component-0".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I225".to_string(),
            bundle_status: "bundle-ready".to_string(),
            selection_action: "source-needed".to_string(),
            selection_basis: "missing-beck-t2-diagnostic".to_string(),
            qualification_effects: String::new(),
            diagnostic_status: "local-relief-map-review".to_string(),
            service_diagnostic_action: "local-relief-map-review".to_string(),
            required_artifact: "data/t2-service-selection.csv".to_string(),
            next_artifact: "data/t3-t4-pressure-intake.csv".to_string(),
            optimizer_effect: "keeps local relief below T2 game overlay".to_string(),
            validation_status: "review".to_string(),
        }];
        let docket_rows = t2_service_class_repair_docket_rows(&target_rows, &diagnostic_rows);
        let route_rows = vec![T3ZoneRouteColumnRow {
            zone_id: "t3-mountain-west".to_string(),
            zone_name: "Mountain West / Interior Coverage".to_string(),
            obligation_class: "regional-feeder-access".to_string(),
            route: "I225".to_string(),
            current_tier: "T2".to_string(),
            current_score: 55.8,
            constraint_adjusted_score: 54.8,
            hard_blocker_count: 0,
            claim_blocker_count: 1,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 1.0,
            top_constraint_classes: "game_ops_bundle_binding".to_string(),
            constraint_ledger_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            promise_horizon_hours: 6,
            column_decision: "selected".to_string(),
            zone_role: "regional-feeder".to_string(),
            contact_requirement: "higher-tier-or-regional-contact-required".to_string(),
            map_treatment: "render-as-zone-column".to_string(),
            selection_basis: "score meets T3 threshold and satisfies a 6h feeder obligation"
                .to_string(),
            source_obligation: "select T3 feeder/contact chain inside the zone".to_string(),
            next_artifact: "data/t3-zone-map-diagnostics.csv".to_string(),
            optimizer_effect: "feeds the T3 zone map and stop-column selector".to_string(),
            validation_status: "pass".to_string(),
        }];
        let board_rows = vec![T3ZoneRenderBoardRow {
            zone_id: "t3-mountain-west".to_string(),
            zone_name: "Mountain West / Interior Coverage".to_string(),
            map_id: "t3-mountain-west".to_string(),
            map_path: "maps/t3-mountain-west.png".to_string(),
            board_layer: "selected-route".to_string(),
            route: "I225".to_string(),
            national_segment_id: "US.HWYSEG.I225".to_string(),
            stitch_group_id: "US.HWYSTITCH.I225".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I225".to_string(),
            segment_aliases: "route:I225".to_string(),
            route_status: "selected".to_string(),
            map_treatment: "render-as-zone-column".to_string(),
            selected_route_count: 4,
            access_gap_count: 0,
            source_artifact: "data/t3-zone-route-columns.csv".to_string(),
            render_action: "render selected T3 route column".to_string(),
            next_artifact: "maps/t3-zone".to_string(),
            validation_status: "pass".to_string(),
        }];

        let rows = t2_local_zone_overlay_handoff_rows(&docket_rows, &route_rows, &board_rows);
        let failures = t2_local_zone_overlay_handoff_gate_failures(&rows, &docket_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].handoff_decision, "held-local-zone");
        assert_eq!(rows[0].zone_role, "regional-feeder");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].blocks_claims, "game;incident;publication;upgrade");
    }

    #[test]
    fn t2_bundle_readiness_disposition_keeps_i37_blocked() {
        let decision_rows = vec![T2GameOpsBindingDecisionRow {
            decision_id: "T2GAMEOPSDECISION-I37".to_string(),
            intake_id: "T2GAMEOPSINTAKE-I37".to_string(),
            subject_id: "US.HWYBUNDLE.I37".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I37".to_string(),
            route: "I37".to_string(),
            service_class: "compact-service".to_string(),
            bundle_status: "needs-stop-chain".to_string(),
            binding_status: "bundle-bound-review".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            qualification_gate_policy: "accepted when structural diagnostics pass".to_string(),
            qualification_game_use:
                "default playable service for incidents, upgrades, and restitches".to_string(),
            decision: "repair-needed".to_string(),
            decision_reason: "bundle id exists but validation remains under review".to_string(),
            blocks_claims: "game;incident;publication;upgrade".to_string(),
            next_artifact: "data/national-segment-bundles.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let target_rows = t2_bundle_overlay_repair_target_rows(&decision_rows, &[]);

        let rows = t2_bundle_readiness_disposition_rows(&target_rows);
        let failures = t2_bundle_readiness_disposition_gate_failures(&rows, &target_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].disposition, "repair-needed");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].validation_status, "review");
    }

    #[test]
    fn t2_bundle_readiness_repair_docket_dockets_only_repair_needed_rows() {
        let readiness_rows = vec![
            T2BundleReadinessDispositionRow {
                disposition_id: "T2BUNDLEREADINESS-I37".to_string(),
                target_id: "T2OVERLAYREPAIR-I37".to_string(),
                route: "I37".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.I37".to_string(),
                bundle_status: "needs-stop-chain".to_string(),
                service_class: "compact-service".to_string(),
                readiness_class: "stop-chain".to_string(),
                disposition: "repair-needed".to_string(),
                disposition_action: "author-stop-chain-before-bundle-pass".to_string(),
                qualification_effects:
                    "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                        .to_string(),
                required_artifact: "data/national-segment-registry.csv".to_string(),
                next_artifact: "data/national-segment-bundles.csv".to_string(),
                blocks_claims: "game;incident;publication;upgrade".to_string(),
                validation_status: "review".to_string(),
            },
            T2BundleReadinessDispositionRow {
                disposition_id: "T2BUNDLEREADINESS-I220".to_string(),
                target_id: "T2OVERLAYREPAIR-I220".to_string(),
                route: "I220".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
                bundle_status: "needs-stop-chain".to_string(),
                service_class: "unclassified".to_string(),
                readiness_class: "stop-chain".to_string(),
                disposition: "held".to_string(),
                disposition_action: "repair-service-class-before-stop-chain-pass".to_string(),
                qualification_effects: String::new(),
                required_artifact: "data/game/t2-service-overlays.csv".to_string(),
                next_artifact: "data/national-segment-bundles.csv".to_string(),
                blocks_claims: "game;incident;publication;upgrade".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t2_bundle_readiness_repair_docket_rows(&readiness_rows);
        let failures = t2_bundle_readiness_repair_docket_gate_failures(&rows, &readiness_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].route, "I37");
        assert_eq!(rows[0].repair_decision, "repair-needed");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].blocks_claims, "game;incident;publication;upgrade");
    }

    #[test]
    fn t2_bundle_readiness_repair_evidence_stays_held_after_candidate_probe() {
        let repair_rows = vec![T2BundleReadinessRepairDocketRow {
            repair_id: "T2BUNDLEREADINESSREPAIR-I295".to_string(),
            disposition_id: "T2BUNDLEREADINESS-I295".to_string(),
            target_id: "T2OVERLAYREPAIR-I295".to_string(),
            route: "I295".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            readiness_class: "stitched-member".to_string(),
            repair_decision: "repair-needed".to_string(),
            repair_action: "stitch-member-segments-before-bundle-pass".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            required_artifact: "data/tier-segment-candidates.csv".to_string(),
            next_artifact: "data/national-segment-bundles.csv".to_string(),
            blocks_claims: "game;incident;publication;upgrade".to_string(),
            validation_status: "review".to_string(),
        }];
        let candidate_rows = vec![TierSegmentCandidateRow {
            tier: "T2".to_string(),
            source_selector: "t2-service-selection".to_string(),
            region_id: "component-0".to_string(),
            route: "I295".to_string(),
            edge_id: 1,
            edge_sequence: 1,
            national_segment_id: "US.HWYSEG.I295".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            stitch_group_id: "US.HWYSTITCH.I295".to_string(),
            member_role: "stitched-member".to_string(),
            state: "NC".to_string(),
            length_miles: 1.0,
            aadt: "source-needed".to_string(),
            lane_count: "source-needed".to_string(),
            route_aliases: "route:I295".to_string(),
            selector_basis: "test".to_string(),
            candidate_action: "candidate".to_string(),
            qualification_effects: String::new(),
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows =
            t2_bundle_readiness_repair_evidence_rows(&repair_rows, &[], &candidate_rows, &[]);
        let failures = t2_bundle_readiness_repair_evidence_gate_failures(&rows, &repair_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].evidence_status, "candidate-evidence-found");
        assert_eq!(rows[0].evidence_decision, "held-for-readiness-replay");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].blocks_claims, "game;incident;publication;upgrade");
    }

    #[test]
    fn t2_bundle_readiness_replay_decisions_preserve_candidate_probe_blockers() {
        let evidence_rows = vec![T2BundleReadinessRepairEvidenceRow {
            evidence_id: "T2BUNDLEREADINESSEVIDENCE-I295".to_string(),
            repair_id: "T2BUNDLEREADINESSREPAIR-I295".to_string(),
            route: "I295".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            readiness_class: "stitched-member".to_string(),
            evidence_artifact: "data/tier-segment-candidates.csv".to_string(),
            evidence_status: "candidate-evidence-found".to_string(),
            evidence_row_count: 1,
            evidence_summary: "1 segment candidate rows match route I295".to_string(),
            evidence_decision: "held-for-readiness-replay".to_string(),
            qualification_effects: "qualification_game_use=default-play".to_string(),
            next_artifact: "data/t2-bundle-overlay-repair-delta.csv".to_string(),
            blocks_claims: "game;incident;publication;upgrade".to_string(),
            validation_status: "review".to_string(),
        }];
        let delta_rows = vec![T2BundleOverlayRepairDeltaRow {
            delta_id: "T2OVERLAYDELTA-I295".to_string(),
            decision_id: "T2GAMEOPSDECISION-I295".to_string(),
            route: "I295".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            previous_decision: "held".to_string(),
            target_status: "repair-needed".to_string(),
            service_action: "repair-service-overlay-before-game-ops-binding".to_string(),
            readiness_disposition: "repair-needed".to_string(),
            replay_decision: "held".to_string(),
            qualification_effects: "qualification_gate_policy=stop-first".to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-bundles.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_bundle_readiness_replay_decision_rows(&evidence_rows, &delta_rows);
        let failures =
            t2_bundle_readiness_replay_decision_gate_failures(&rows, &evidence_rows, &delta_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].replay_decision, "held-for-bundle-replay");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(
            rows[0].blocked_claims_after,
            "game;incident;publication;upgrade"
        );
        assert_eq!(rows[0].blocker_delta, 0);
    }

    #[test]
    fn t2_national_bundle_readiness_audit_stays_held_on_structural_status() {
        let replay_rows = vec![T2BundleReadinessReplayDecisionRow {
            replay_id: "T2BUNDLEREADINESSREPLAY-I295".to_string(),
            evidence_id: "T2BUNDLEREADINESSEVIDENCE-I295".to_string(),
            delta_id: "T2OVERLAYDELTA-I295".to_string(),
            route: "I295".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            readiness_class: "stitched-member".to_string(),
            evidence_status: "candidate-evidence-found".to_string(),
            delta_replay_decision: "held".to_string(),
            replay_decision: "held-for-bundle-replay".to_string(),
            replay_action: "keep-held-until-repair-delta-mutates".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-bundles.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let bundle_rows = vec![NationalSegmentBundleRow {
            segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            bundle_role: "stitched-service".to_string(),
            member_segment_ids: "US.HWYSEG.I295".to_string(),
            member_count: 1,
            stitch_group_ids: "US.HWYSTITCH.I295".to_string(),
            current_tiers: "T2".to_string(),
            current_zone_ids: "component-0".to_string(),
            route_labels: "I295".to_string(),
            state_scope: "FL".to_string(),
            evidence_state_scope: "FL".to_string(),
            geometry_state_scope: "FL".to_string(),
            bundle_aliases: "route:I295".to_string(),
            source_artifacts: "data/tier-segment-candidates.csv".to_string(),
            bundle_status: "needs-stitched-members".to_string(),
            bundle_action: "add ordered member segments before promotion or stitched service"
                .to_string(),
            qualification_effects: String::new(),
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_national_bundle_readiness_audit_rows(&replay_rows, &bundle_rows);
        let failures = t2_national_bundle_readiness_audit_gate_failures(&rows, &replay_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].audit_decision, "held-for-structural-bundle-repair");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].bundle_status, "needs-stitched-members");
        assert_eq!(
            rows[0].blocked_claims_after,
            "game;incident;publication;upgrade"
        );
    }

    #[test]
    fn t2_stitched_member_registry_handoff_preserves_claim_blockers() {
        let replay_rows = vec![T2BundleReadinessReplayDecisionRow {
            replay_id: "T2BUNDLEREADINESSREPLAY-I295".to_string(),
            evidence_id: "T2BUNDLEREADINESSEVIDENCE-I295".to_string(),
            delta_id: "T2OVERLAYDELTA-I295".to_string(),
            route: "I295".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            readiness_class: "stitched-member".to_string(),
            evidence_status: "candidate-evidence-found".to_string(),
            delta_replay_decision: "held".to_string(),
            replay_decision: "held-for-bundle-replay".to_string(),
            replay_action: "keep-held-until-repair-delta-mutates".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-bundles.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let bundle_rows = vec![NationalSegmentBundleRow {
            segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            bundle_role: "stitched-service".to_string(),
            member_segment_ids: "US.HWYSEG.I295".to_string(),
            member_count: 1,
            stitch_group_ids: "US.HWYSTITCH.I295".to_string(),
            current_tiers: "T2".to_string(),
            current_zone_ids: "component-0".to_string(),
            route_labels: "I295".to_string(),
            state_scope: "FL".to_string(),
            evidence_state_scope: "FL".to_string(),
            geometry_state_scope: "FL".to_string(),
            bundle_aliases: "route:I295".to_string(),
            source_artifacts: "data/tier-segment-candidates.csv".to_string(),
            bundle_status: "needs-stitched-members".to_string(),
            bundle_action: "add ordered member segments before promotion or stitched service"
                .to_string(),
            qualification_effects: String::new(),
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let registry_rows = vec![NationalSegmentRegistryRow {
            national_segment_id: "US.HWYSEG.I295".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            stitch_group_id: "US.HWYSTITCH.I295".to_string(),
            current_zone_id: "component-0".to_string(),
            current_tier: "T2".to_string(),
            route_label: "I295".to_string(),
            zone_id: "component-0".to_string(),
            route: "I295".to_string(),
            state_scope: "FL".to_string(),
            evidence_state_scope: "FL".to_string(),
            geometry_state_scope: "FL".to_string(),
            segment_aliases: "route:I295".to_string(),
            bundle_aliases: "route:I295".to_string(),
            board_layers: "tier-segment-candidate".to_string(),
            source_artifacts: "data/tier-segment-candidates.csv".to_string(),
            stop_placement_status: "member-role:stitched-member".to_string(),
            bundle_role: "stitched-service".to_string(),
            member_segment_ids: "US.HWYSEG.I295".to_string(),
            registry_action: "eligible-for-service-bundle".to_string(),
            qualification_effects: String::new(),
            validation_status: "pass".to_string(),
        }];
        let candidate_rows = vec![TierSegmentCandidateRow {
            tier: "T2".to_string(),
            source_selector: "t2-service-selection".to_string(),
            region_id: "component-0".to_string(),
            route: "I295".to_string(),
            edge_id: 1,
            edge_sequence: 1,
            national_segment_id: "US.HWYSEG.I295".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            stitch_group_id: "US.HWYSTITCH.I295".to_string(),
            member_role: "stitched-member".to_string(),
            state: "FL".to_string(),
            length_miles: 1.0,
            aadt: "source-needed".to_string(),
            lane_count: "source-needed".to_string(),
            route_aliases: "route:I295".to_string(),
            selector_basis: "test".to_string(),
            candidate_action: "candidate".to_string(),
            qualification_effects: String::new(),
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let audit_rows = t2_national_bundle_readiness_audit_rows(&replay_rows, &bundle_rows);
        let rows =
            t2_stitched_member_registry_handoff_rows(&audit_rows, &registry_rows, &candidate_rows);
        let failures = t2_stitched_member_registry_handoff_gate_failures(&rows, &audit_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].handoff_decision, "held-for-member-expansion");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].current_registry_member_count, 1);
        assert_eq!(
            rows[0].blocked_claims_after,
            "game;incident;publication;upgrade"
        );
    }

    #[test]
    fn t2_stitched_member_candidate_scope_review_does_not_promote_route_evidence() {
        let handoff_rows = vec![T2StitchedMemberRegistryHandoffRow {
            handoff_id: "T2STITCHEDREGISTRYHANDOFF-I295".to_string(),
            audit_id: "T2NATIONALBUNDLEAUDIT-I295".to_string(),
            route: "I295".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            stitch_group_id: "US.HWYSTITCH.I295".to_string(),
            current_registry_member_count: 1,
            candidate_bundle_member_count: 1,
            candidate_route_member_count: 2,
            required_member_min: 2,
            handoff_decision: "held-for-member-expansion".to_string(),
            handoff_action: "expand-stitch-group-before-bundle-replay".to_string(),
            qualification_effects: String::new(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/tier-segment-candidates.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let candidate_rows = vec![
            TierSegmentCandidateRow {
                tier: "T2".to_string(),
                source_selector: "t2-service-selection".to_string(),
                region_id: "component-0".to_string(),
                route: "I295".to_string(),
                edge_id: 1,
                edge_sequence: 1,
                national_segment_id: "US.HWYSEG.I295A".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
                stitch_group_id: "US.HWYSTITCH.I295".to_string(),
                member_role: "stitched-member".to_string(),
                state: "FL".to_string(),
                length_miles: 1.0,
                aadt: "source-needed".to_string(),
                lane_count: "source-needed".to_string(),
                route_aliases: "route:I295".to_string(),
                selector_basis: "test".to_string(),
                candidate_action: "candidate".to_string(),
                qualification_effects: String::new(),
                next_artifact: "data/national-segment-registry.csv".to_string(),
                validation_status: "review".to_string(),
            },
            TierSegmentCandidateRow {
                tier: "T2".to_string(),
                source_selector: "t2-service-selection".to_string(),
                region_id: "component-0".to_string(),
                route: "I295".to_string(),
                edge_id: 2,
                edge_sequence: 2,
                national_segment_id: "US.HWYSEG.I295B".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.I295OTHER".to_string(),
                stitch_group_id: "US.HWYSTITCH.I295OTHER".to_string(),
                member_role: "stitched-member".to_string(),
                state: "SC".to_string(),
                length_miles: 1.0,
                aadt: "source-needed".to_string(),
                lane_count: "source-needed".to_string(),
                route_aliases: "route:I295".to_string(),
                selector_basis: "test".to_string(),
                candidate_action: "candidate".to_string(),
                qualification_effects: String::new(),
                next_artifact: "data/national-segment-registry.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t2_stitched_member_candidate_scope_review_rows(&handoff_rows, &candidate_rows);
        let failures =
            t2_stitched_member_candidate_scope_review_gate_failures(&rows, &handoff_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].scope_decision, "held-for-scope-review");
        assert_eq!(rows[0].route_candidate_bundle_count, 2);
        assert_eq!(
            rows[0].blocked_claims_after,
            "game;incident;publication;upgrade"
        );
    }

    #[test]
    fn t2_stitched_member_decision_docket_keeps_split_as_review() {
        let scope_rows = vec![T2StitchedMemberCandidateScopeReviewRow {
            scope_review_id: "T2STITCHEDSCOPE-I295".to_string(),
            handoff_id: "T2STITCHEDREGISTRYHANDOFF-I295".to_string(),
            route: "I295".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            blocked_bundle_candidate_count: 1,
            route_candidate_count: 84,
            route_candidate_bundle_count: 9,
            route_candidate_state_scope: "FL;GA;SC".to_string(),
            route_candidate_bundle_ids: "US.HWYBUNDLE.I295;US.HWYBUNDLE.I295SC".to_string(),
            scope_decision: "held-for-scope-review".to_string(),
            scope_action: "review-route-family-state-scope-before-member-expansion".to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/tier-segment-candidates.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_stitched_member_decision_docket_rows(&scope_rows);
        let failures = t2_stitched_member_decision_docket_gate_failures(&rows, &scope_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].decision, "split");
        assert_eq!(rows[0].validation_status, "review");
        assert_eq!(
            rows[0].blocked_claims_after,
            "game;incident;publication;upgrade"
        );
    }

    #[test]
    fn t2_stitched_member_split_plan_keeps_candidates_as_review() {
        let decision_rows = vec![T2StitchedMemberDecisionDocketRow {
            decision_docket_id: "T2STITCHEDDECISION-I295".to_string(),
            scope_review_id: "T2STITCHEDSCOPE-I295".to_string(),
            route: "I295".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            candidate_bundle_count: 2,
            candidate_state_scope: "FL;VA".to_string(),
            decision: "split".to_string(),
            decision_action: "split-route-family-scope-before-member-expansion".to_string(),
            repair_instruction: "choose the state-scoped bundle ids".to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/tier-segment-candidates.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let candidate_rows = vec![
            TierSegmentCandidateRow {
                tier: "T2".to_string(),
                source_selector: "t2-service-selection".to_string(),
                region_id: "component-0".to_string(),
                route: "I295".to_string(),
                edge_id: 1,
                edge_sequence: 1,
                national_segment_id: "US.HWYSEG.1".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.FL".to_string(),
                stitch_group_id: "US.HWYSTITCH.FL".to_string(),
                member_role: "stitched-member".to_string(),
                state: "FL".to_string(),
                length_miles: 4.0,
                aadt: "1000".to_string(),
                lane_count: "4".to_string(),
                route_aliases: "I95".to_string(),
                selector_basis: "route-family-scope:FL".to_string(),
                candidate_action: "missing-beck-t2-diagnostic".to_string(),
                qualification_effects: String::new(),
                next_artifact: "data/national-segment-registry.csv".to_string(),
                validation_status: "review".to_string(),
            },
            TierSegmentCandidateRow {
                tier: "T2".to_string(),
                source_selector: "t2-service-selection".to_string(),
                region_id: "component-0".to_string(),
                route: "I295".to_string(),
                edge_id: 2,
                edge_sequence: 2,
                national_segment_id: "US.HWYSEG.2".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.VA".to_string(),
                stitch_group_id: "US.HWYSTITCH.VA".to_string(),
                member_role: "stitched-member".to_string(),
                state: "VA".to_string(),
                length_miles: 3.0,
                aadt: "1000".to_string(),
                lane_count: "4".to_string(),
                route_aliases: "I95".to_string(),
                selector_basis: "route-family-scope:VA".to_string(),
                candidate_action: "missing-beck-t2-diagnostic".to_string(),
                qualification_effects: String::new(),
                next_artifact: "data/national-segment-registry.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t2_stitched_member_split_plan_rows(&decision_rows, &candidate_rows);
        let failures = t2_stitched_member_split_plan_gate_failures(&rows, &decision_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.validation_status == "review"));
        assert!(rows.iter().all(|row| row.blocker_delta == 0));
        assert!(rows
            .iter()
            .all(|row| row.blocked_claims_after == "game;incident;publication;upgrade"));
    }

    #[test]
    fn t2_stitched_member_selection_docket_keeps_rows_evidence_needed() {
        let split_rows = vec![T2StitchedMemberSplitPlanRow {
            split_plan_id: "T2STITCHEDSPLIT-I295-FL".to_string(),
            decision_docket_id: "T2STITCHEDDECISION-I295".to_string(),
            route: "I295".to_string(),
            blocked_segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            candidate_segment_bundle_id: "US.HWYBUNDLE.FL".to_string(),
            candidate_stitch_group_id: "US.HWYSTITCH.FL".to_string(),
            state_scope: "FL".to_string(),
            candidate_member_count: 1,
            candidate_length_miles: 4.0,
            split_action: "review-state-scoped-candidate-before-membership-mutation".to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_stitched_member_selection_docket_rows(&split_rows);
        let failures = t2_stitched_member_selection_docket_gate_failures(&rows, &split_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].selection_decision, "evidence-needed");
        assert_eq!(rows[0].validation_status, "review");
        assert_eq!(rows[0].blocker_delta, 0);
        assert_eq!(
            rows[0].blocked_claims_after,
            "game;incident;publication;upgrade"
        );
    }

    #[test]
    fn t2_stitched_member_evidence_contract_keeps_rows_source_needed() {
        let selection_rows = vec![T2StitchedMemberSelectionDocketRow {
            selection_docket_id: "T2STITCHEDSELECT-I295-FL".to_string(),
            split_plan_id: "T2STITCHEDSPLIT-I295-FL".to_string(),
            route: "I295".to_string(),
            blocked_segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            candidate_segment_bundle_id: "US.HWYBUNDLE.FL".to_string(),
            state_scope: "FL".to_string(),
            candidate_member_count: 1,
            candidate_length_miles: 4.0,
            selection_decision: "evidence-needed".to_string(),
            selection_action: "collect-state-scope-evidence-before-decision".to_string(),
            evidence_requirement:
                "manual route-family service continuity evidence before in-scope or rejected status"
                    .to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_stitched_member_evidence_contract_rows(&selection_rows);
        let failures = t2_stitched_member_evidence_contract_gate_failures(&rows, &selection_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].evidence_status, "source-needed");
        assert_eq!(rows[0].validation_status, "review");
        assert_eq!(rows[0].blocker_delta, 0);
        assert_eq!(
            rows[0].blocked_claims_after,
            "game;incident;publication;upgrade"
        );
    }

    #[test]
    fn t2_stitched_member_evidence_acquisition_keeps_rows_source_needed() {
        let contract_rows = vec![T2StitchedMemberEvidenceContractRow {
            evidence_contract_id: "T2STITCHEDEVIDENCE-I295-FL".to_string(),
            selection_docket_id: "T2STITCHEDSELECT-I295-FL".to_string(),
            route: "I295".to_string(),
            candidate_segment_bundle_id: "US.HWYBUNDLE.FL".to_string(),
            state_scope: "FL".to_string(),
            required_continuity_proof:
                "document continuous service relationship between candidate bundle and blocked stitched route"
                    .to_string(),
            required_scope_proof:
                "document why the state-scoped candidate belongs in or outside the blocked service"
                    .to_string(),
            required_source_proof:
                "cite authoritative route geometry or agency source before in-scope or rejected status"
                    .to_string(),
            evidence_status: "source-needed".to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_stitched_member_evidence_acquisition_rows(&contract_rows);
        let failures = t2_stitched_member_evidence_acquisition_gate_failures(&rows, &contract_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].acquisition_status, "source-needed");
        assert_eq!(rows[0].validation_status, "review");
        assert_eq!(rows[0].blocker_delta, 0);
        assert_eq!(
            rows[0].blocked_claims_after,
            "game;incident;publication;upgrade"
        );
    }

    #[test]
    fn t2_stitched_member_source_access_policy_blocks_live_fetch() {
        let acquisition_rows = vec![T2StitchedMemberEvidenceAcquisitionRow {
            acquisition_docket_id: "T2STITCHEDACQUIRE-I295-FL".to_string(),
            evidence_contract_id: "T2STITCHEDEVIDENCE-I295-FL".to_string(),
            route: "I295".to_string(),
            candidate_segment_bundle_id: "US.HWYBUNDLE.FL".to_string(),
            state_scope: "FL".to_string(),
            source_owner: "FL DOT".to_string(),
            source_target:
                "FL DOT route log, GIS centerline, or official route description for I295 FL"
                    .to_string(),
            acquisition_action:
                "manual-source-request-or-cache-official-route-geometry-before-decision".to_string(),
            acquisition_status: "source-needed".to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_stitched_member_source_access_policy_rows(&acquisition_rows);
        let failures =
            t2_stitched_member_source_access_policy_gate_failures(&rows, &acquisition_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].access_mode, "manual-or-cached-source-needed");
        assert_eq!(
            rows[0].live_fetch_status,
            "unsupported-no-safe-stitched-member-fetcher"
        );
        assert_eq!(rows[0].evidence_artifact, "source-needed");
        assert_eq!(rows[0].blocker_delta, 0);
    }

    #[test]
    fn t2_stitched_member_proof_intake_requires_artifact_without_acceptance() {
        let access_rows = vec![T2StitchedMemberSourceAccessPolicyRow {
            access_policy_id: "T2STITCHEDACCESS-I295-FL".to_string(),
            acquisition_docket_id: "T2STITCHEDACQUIRE-I295-FL".to_string(),
            route: "I295".to_string(),
            candidate_segment_bundle_id: "US.HWYBUNDLE.FL".to_string(),
            state_scope: "FL".to_string(),
            source_owner: "FL DOT".to_string(),
            access_mode: "manual-or-cached-source-needed".to_string(),
            live_fetch_status: "unsupported-no-safe-stitched-member-fetcher".to_string(),
            required_source_metadata:
                "source title; source url or cached artifact; capture date; route; state scope; route geometry statement"
                    .to_string(),
            cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                .to_string(),
            source_access_blocker:
                "no safe live stitched-member route geometry fetch command exists; use manual/cached proof artifact or add policy-compliant fetcher"
                    .to_string(),
            evidence_artifact: "source-needed".to_string(),
            acquisition_status: "source-needed".to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_stitched_member_proof_intake_rows(&access_rows);
        let failures = t2_stitched_member_proof_intake_gate_failures(&rows, &access_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].proof_artifact, "source-needed");
        assert_eq!(rows[0].proof_status, "source-needed");
        assert_eq!(rows[0].validation_status, "review");
        assert_eq!(rows[0].blocker_delta, 0);
        assert_eq!(
            rows[0].blocked_claims_after,
            "game;incident;publication;upgrade"
        );
    }

    #[test]
    fn t2_stitched_member_proof_source_capture_stays_source_needed() {
        let intake_rows = vec![T2StitchedMemberProofIntakeRow {
            proof_intake_id: "T2STITCHEDPROOF-I295-FL".to_string(),
            access_policy_id: "T2STITCHEDACCESS-I295-FL".to_string(),
            route: "I295".to_string(),
            candidate_segment_bundle_id: "US.HWYBUNDLE.FL".to_string(),
            state_scope: "FL".to_string(),
            required_artifact_fields:
                "source title; source url or cached artifact; capture date; route; state scope; source owner"
                    .to_string(),
            required_geometry_statement:
                "route geometry statement explaining continuity with the blocked stitched service"
                    .to_string(),
            proof_artifact: "source-needed".to_string(),
            proof_status: "source-needed".to_string(),
            proof_blocker:
                "manual or cached route-geometry proof artifact has not been captured or reviewed"
                    .to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_stitched_member_proof_source_capture_rows(&intake_rows);
        let failures = t2_stitched_member_proof_source_capture_gate_failures(&rows, &intake_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].source_artifact_reference, "source-needed");
        assert_eq!(rows[0].capture_status, "source-needed");
        assert_eq!(rows[0].evidence_acceptance_status, "not-reviewed");
        assert_eq!(rows[0].validation_status, "review");
        assert_eq!(rows[0].blocker_delta, 0);
    }

    #[test]
    fn t2_stitched_member_proof_artifact_attachment_stays_unattached() {
        let capture_rows = vec![T2StitchedMemberProofSourceCaptureRow {
            source_capture_id: "T2STITCHEDSOURCE-I295-FL".to_string(),
            proof_intake_id: "T2STITCHEDPROOF-I295-FL".to_string(),
            route: "I295".to_string(),
            candidate_segment_bundle_id: "US.HWYBUNDLE.FL".to_string(),
            state_scope: "FL".to_string(),
            source_artifact_reference: "source-needed".to_string(),
            source_artifact_type: "manual-or-cached-route-geometry".to_string(),
            capture_status: "source-needed".to_string(),
            evidence_acceptance_status: "not-reviewed".to_string(),
            capture_blocker:
                "manual or cached DOT route-geometry source artifact has not been attached"
                    .to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_stitched_member_proof_artifact_attachment_rows(&capture_rows);
        let failures =
            t2_stitched_member_proof_artifact_attachment_gate_failures(&rows, &capture_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].source_artifact_reference, "source-needed");
        assert_eq!(rows[0].attachment_status, "source-needed");
        assert_eq!(rows[0].evidence_review_status, "not-reviewed");
        assert_eq!(rows[0].proof_acceptance_status, "not-accepted");
        assert_eq!(rows[0].validation_status, "review");
        assert_eq!(rows[0].blocker_delta, 0);
    }

    #[test]
    fn t2_stitched_member_proof_review_returns_to_optimizer_hold() {
        let attachment_rows = vec![T2StitchedMemberProofArtifactAttachmentRow {
            artifact_attachment_id: "T2STITCHEDATTACH-I295-FL".to_string(),
            source_capture_id: "T2STITCHEDSOURCE-I295-FL".to_string(),
            route: "I295".to_string(),
            candidate_segment_bundle_id: "US.HWYBUNDLE.FL".to_string(),
            state_scope: "FL".to_string(),
            source_artifact_reference: "source-needed".to_string(),
            attachment_status: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            attachment_blocker:
                "manual or cached DOT route-geometry artifact reference has not been attached"
                    .to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_stitched_member_proof_review_docket_rows(&attachment_rows);
        let failures =
            t2_stitched_member_proof_review_docket_gate_failures(&rows, &attachment_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].review_decision, "held-no-source-artifact");
        assert_eq!(rows[0].proof_acceptance_status, "not-accepted");
        assert_eq!(
            rows[0].candidate_disposition_status,
            "not-ready-for-disposition"
        );
        assert_eq!(
            rows[0].optimization_return_status,
            "return-to-optimizer-held-known"
        );
        assert_eq!(rows[0].next_artifact, "data/tier-optimizer-runs.csv");
        assert_eq!(rows[0].blocker_delta, 0);
    }

    #[test]
    fn t2_bundle_overlay_repair_delta_preserves_claim_blocks() {
        let decision_rows = vec![T2GameOpsBindingDecisionRow {
            decision_id: "T2GAMEOPSDECISION-3".to_string(),
            intake_id: "T2GAMEOPSINTAKE-3".to_string(),
            subject_id: "US.HWYBUNDLE.HELD".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.HELD".to_string(),
            route: "I195".to_string(),
            service_class: "unclassified".to_string(),
            bundle_status: "bundle-ready".to_string(),
            binding_status: "service-class-held-known".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            qualification_gate_policy: "accepted when structural diagnostics pass".to_string(),
            qualification_game_use:
                "default playable service for incidents, upgrades, and restitches".to_string(),
            decision: "held".to_string(),
            decision_reason: "service class overlay is missing or held".to_string(),
            blocks_claims: "game;incident;publication;upgrade".to_string(),
            next_artifact: "data/game/t2-service-overlays.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let target_rows = t2_bundle_overlay_repair_target_rows(&decision_rows, &[]);
        let service_rows = t2_service_class_repair_docket_rows(&target_rows, &[]);
        let readiness_rows = t2_bundle_readiness_disposition_rows(&target_rows);

        let rows = t2_bundle_overlay_repair_delta_rows(
            &decision_rows,
            &target_rows,
            &service_rows,
            &readiness_rows,
        );
        let failures = t2_bundle_overlay_repair_delta_gate_failures(&rows, &decision_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].replay_decision, "held");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].blocker_delta, 0);
        assert_eq!(rows[0].blocked_claims_before, rows[0].blocked_claims_after);
    }

    #[test]
    fn t2_overlay_optimizer_action_docket_routes_delta_without_promotion() {
        let delta_rows = vec![T2BundleOverlayRepairDeltaRow {
            delta_id: "T2OVERLAYDELTA-1".to_string(),
            decision_id: "T2GAMEOPSDECISION-1".to_string(),
            route: "I195".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.HELD".to_string(),
            previous_decision: "held".to_string(),
            target_status: "held".to_string(),
            service_action: "repair-service-overlay-before-game-ops-binding".to_string(),
            readiness_disposition: "no-readiness-disposition-required".to_string(),
            replay_decision: "held".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/game/t2-service-overlays.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_overlay_optimizer_action_docket_rows(&delta_rows);
        let failures = t2_overlay_optimizer_action_docket_gate_failures(&rows, &delta_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(
            rows[0].optimizer_action,
            "service-overlay-diagnostic-review"
        );
        assert_eq!(rows[0].priority_class, "P2-service-overlay");
        assert_eq!(rows[0].action_status, "optimizer-held-known");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].blocker_delta, 0);
        assert_eq!(rows[0].blocked_claims_before, rows[0].blocked_claims_after);
    }

    #[test]
    fn t2_overlay_p1_structural_readiness_review_keeps_actions_held() {
        let action_rows = vec![T2OverlayOptimizerActionDocketRow {
            action_id: "T2OVERLAYACTION-I295".to_string(),
            delta_id: "T2OVERLAYDELTA-I295".to_string(),
            route: "I295".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I295".to_string(),
            replay_decision: "held".to_string(),
            service_action: "repair-service-overlay-before-game-ops-binding".to_string(),
            readiness_disposition: "repair-needed".to_string(),
            optimizer_action: "bundle-readiness-repair-review".to_string(),
            priority_class: "P1-structural-readiness".to_string(),
            action_status: "optimizer-held-known".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-bundles.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_overlay_p1_structural_readiness_review_rows(&action_rows);
        let failures = t2_overlay_p1_structural_readiness_review_gate_failures(&rows, &action_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].priority_class, "P1-structural-readiness");
        assert_eq!(rows[0].action_status, "optimizer-held-known");
        assert_eq!(rows[0].readiness_decision, "held-stitched-proof-returned");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].blocker_delta, 0);
        assert_eq!(rows[0].blocked_claims_before, rows[0].blocked_claims_after);
    }

    #[test]
    fn t2_overlay_p2_service_overlay_review_keeps_actions_held() {
        let action_rows = vec![T2OverlayOptimizerActionDocketRow {
            action_id: "T2OVERLAYACTION-I195".to_string(),
            delta_id: "T2OVERLAYDELTA-I195".to_string(),
            route: "I195".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I195".to_string(),
            replay_decision: "held".to_string(),
            service_action: "repair-service-overlay-before-game-ops-binding".to_string(),
            readiness_disposition: "repair-needed".to_string(),
            optimizer_action: "service-overlay-diagnostic-review".to_string(),
            priority_class: "P2-service-overlay".to_string(),
            action_status: "optimizer-held-known".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/t2-overlay-optimizer-action-docket.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_overlay_p2_service_overlay_review_rows(&action_rows);
        let failures = t2_overlay_p2_service_overlay_review_gate_failures(&rows, &action_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].priority_class, "P2-service-overlay");
        assert_eq!(rows[0].action_status, "optimizer-held-known");
        assert_eq!(
            rows[0].service_overlay_decision,
            "held-service-overlay-diagnostic-needed"
        );
        assert_eq!(
            rows[0].downstream_action,
            "route-to-service-overlay-diagnostic-review"
        );
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].blocker_delta, 0);
        assert_eq!(rows[0].blocked_claims_before, rows[0].blocked_claims_after);
    }

    #[test]
    fn t2_overlay_p3_local_zone_overlay_review_keeps_actions_held() {
        let action_rows = vec![T2OverlayOptimizerActionDocketRow {
            action_id: "T2OVERLAYACTION-I205".to_string(),
            delta_id: "T2OVERLAYDELTA-I205".to_string(),
            route: "I205".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I205".to_string(),
            replay_decision: "held".to_string(),
            service_action: "hold-local-relief-below-national-game-overlay".to_string(),
            readiness_disposition: "no-readiness-disposition-required".to_string(),
            optimizer_action: "local-zone-overlay-review".to_string(),
            priority_class: "P3-local-zone-overlay".to_string(),
            action_status: "optimizer-held-known".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            blocked_claims_before: "game;incident;publication;upgrade".to_string(),
            blocked_claims_after: "game;incident;publication;upgrade".to_string(),
            blocker_delta: 0,
            next_artifact: "data/t3-zone-render-board.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_overlay_p3_local_zone_overlay_review_rows(&action_rows);
        let failures = t2_overlay_p3_local_zone_overlay_review_gate_failures(&rows, &action_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].priority_class, "P3-local-zone-overlay");
        assert_eq!(rows[0].action_status, "optimizer-held-known");
        assert_eq!(
            rows[0].local_zone_decision,
            "held-local-zone-overlay-review-needed"
        );
        assert_eq!(
            rows[0].downstream_action,
            "route-to-local-zone-overlay-review"
        );
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
        assert_eq!(rows[0].blocker_delta, 0);
        assert_eq!(rows[0].blocked_claims_before, rows[0].blocked_claims_after);
    }

    #[test]
    fn tier_pavement_docket_marks_pass_repair_and_source_needed_segments() {
        let mut graph = HighwayGraph::new();
        let a = graph.graph.add_node(HighwayNode {
            id: 1,
            coord: coord! { x: 0.0, y: 0.0 },
            is_interchange: false,
        });
        let b = graph.graph.add_node(HighwayNode {
            id: 2,
            coord: coord! { x: 1.0, y: 0.0 },
            is_interchange: true,
        });
        let c = graph.graph.add_node(HighwayNode {
            id: 3,
            coord: coord! { x: 2.0, y: 0.0 },
            is_interchange: false,
        });
        let d = graph.graph.add_node(HighwayNode {
            id: 4,
            coord: coord! { x: 3.0, y: 0.0 },
            is_interchange: false,
        });
        graph.graph.add_edge(
            a,
            b,
            HighwayEdge {
                id: 10,
                route_id: "I80".to_string(),
                state: "NE".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: 0.0, y: 0.0 },
                    coord! { x: 1.0, y: 0.0 },
                ]),
                length_miles: 50.0,
                lane_count: Some(4),
                aadt: Some(40_000),
                pct_truck: None,
                iri: Some(64.0),
                tti: None,
                pti: None,
                speed_limit: None,
            },
        );
        graph.graph.add_edge(
            b,
            c,
            HighwayEdge {
                id: 11,
                route_id: "I80".to_string(),
                state: "IA".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: 1.0, y: 0.0 },
                    coord! { x: 2.0, y: 0.0 },
                ]),
                length_miles: 60.0,
                lane_count: Some(4),
                aadt: Some(45_000),
                pct_truck: None,
                iri: Some(133.0),
                tti: None,
                pti: None,
                speed_limit: None,
            },
        );
        graph.graph.add_edge(
            c,
            d,
            HighwayEdge {
                id: 12,
                route_id: "I80".to_string(),
                state: "IL".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: 2.0, y: 0.0 },
                    coord! { x: 3.0, y: 0.0 },
                ]),
                length_miles: 70.0,
                lane_count: Some(4),
                aadt: Some(50_000),
                pct_truck: None,
                iri: None,
                tti: None,
                pti: None,
                speed_limit: None,
            },
        );

        let segment_rows = [10_u64, 11, 12]
            .into_iter()
            .enumerate()
            .map(|(idx, edge_id)| TierSegmentCandidateRow {
                tier: "T1".to_string(),
                source_selector: "fixture".to_string(),
                region_id: "national".to_string(),
                route: "I80".to_string(),
                edge_id,
                edge_sequence: idx + 1,
                national_segment_id: format!("US.HWYSEG.{edge_id:016X}"),
                segment_bundle_id: "US.HWYBUNDLE.FIXTURE".to_string(),
                stitch_group_id: "US.HWYSTITCH.FIXTURE".to_string(),
                member_role: "stitched-member".to_string(),
                state: "fixture".to_string(),
                length_miles: 10.0,
                aadt: "unknown".to_string(),
                lane_count: "unknown".to_string(),
                route_aliases: "route:I80".to_string(),
                selector_basis: "fixture".to_string(),
                candidate_action: "fixture".to_string(),
                qualification_effects: String::new(),
                next_artifact: "data/national-segment-registry.csv".to_string(),
                validation_status: "review".to_string(),
            })
            .collect::<Vec<_>>();
        let standards = vec![PavementStandardRow {
            tier: "T1".to_string(),
            road_role: "national timed-freight spine".to_string(),
            max_iri_m_per_km: 1.5,
            target_pavement_condition: "good".to_string(),
            freight_ride_requirement: "no roughness padding".to_string(),
            transit_ride_requirement: "coach-speed ride quality".to_string(),
            inspection_interval_months: 6,
            repair_trigger: "repair above threshold".to_string(),
            allowed_exception: "temporary construction only".to_string(),
            source_contract: "HPMS IRI plus state pavement feeds".to_string(),
            validation_status: "pass".to_string(),
        }];

        let rows: Vec<TierPavementDocketRow> =
            tier_pavement_docket_rows(&graph, &segment_rows, &standards);
        let failures = tier_pavement_docket_gate_failures(&rows, &segment_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows[0].pavement_status, "pavement-floor-pass");
        assert_eq!(rows[1].pavement_status, "pavement-repair-required");
        assert_eq!(rows[2].pavement_status, "pavement-source-needed");
        assert_eq!(rows[0].iri_m_per_km, "1.01");
        assert_eq!(rows[1].repair_action, "repair above threshold");
        assert!((normalized_iri_m_per_km(Some(64.0)).unwrap() - 1.010).abs() < 0.01);
    }

    #[test]
    fn tier_pavement_source_gaps_roll_up_blocked_members_by_bundle() {
        let docket_rows = vec![
            TierPavementDocketRow {
                tier: "T2".to_string(),
                source_selector: "t2-service-selection".to_string(),
                region_id: "component-1".to_string(),
                route: "US30".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.US30".to_string(),
                stitch_group_id: "US.HWYSTITCH.US30".to_string(),
                national_segment_id: "US.HWYSEG.0000000000000001".to_string(),
                edge_id: 1,
                edge_sequence: 1,
                state: "NE".to_string(),
                length_miles: 10.0,
                iri_m_per_km: "1.20".to_string(),
                max_iri_m_per_km: "1.90".to_string(),
                pavement_status: "pavement-floor-pass".to_string(),
                repair_action: "no pavement debt payment required for this member".to_string(),
                freight_ride_requirement: "regional freight ride quality".to_string(),
                transit_ride_requirement: "regional coach ride quality".to_string(),
                source_contract: "HPMS IRI".to_string(),
                qualification_effects: String::new(),
                next_artifact: "data/national-segment-registry.csv".to_string(),
                validation_status: "pass".to_string(),
            },
            TierPavementDocketRow {
                tier: "T2".to_string(),
                source_selector: "t2-service-selection".to_string(),
                region_id: "component-1".to_string(),
                route: "US30".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.US30".to_string(),
                stitch_group_id: "US.HWYSTITCH.US30".to_string(),
                national_segment_id: "US.HWYSEG.0000000000000002".to_string(),
                edge_id: 2,
                edge_sequence: 2,
                state: "IA".to_string(),
                length_miles: 12.0,
                iri_m_per_km: "unknown".to_string(),
                max_iri_m_per_km: "1.90".to_string(),
                pavement_status: "pavement-source-needed".to_string(),
                repair_action: "record pavement evidence debt".to_string(),
                freight_ride_requirement: "regional freight ride quality".to_string(),
                transit_ride_requirement: "regional coach ride quality".to_string(),
                source_contract: "HPMS IRI plus state pavement feeds".to_string(),
                qualification_effects: String::new(),
                next_artifact: "data/standards-l1-inventory.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows: Vec<TierPavementSourceGapRow> = tier_pavement_source_gap_rows(None, &docket_rows);
        let failures = tier_pavement_source_gap_gate_failures(&rows, &docket_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].route, "US30");
        assert_eq!(rows[0].member_count, 2);
        assert_eq!(rows[0].blocker_count, 1);
        assert_eq!(rows[0].affected_states, "IA");
        assert!(rows[0]
            .source_action
            .contains("price pavement evidence debt"));
        assert_eq!(tier_pavement_route_state_scope(None, "US30"), "");
    }

    #[test]
    fn tier_pavement_debt_budget_prices_source_and_repair_debt() {
        let gap_rows = vec![
            TierPavementSourceGapRow {
                tier: "T2".to_string(),
                route: "US30".to_string(),
                region_id: "component-1".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.US30".to_string(),
                stitch_group_id: "US.HWYSTITCH.US30".to_string(),
                member_count: 100,
                blocker_count: 90,
                blocker_statuses: "pavement-source-needed".to_string(),
                affected_states: "IA;NE;WY".to_string(),
                affected_edge_ids: "1;2;3".to_string(),
                source_contract: "HPMS IRI plus state pavement feeds".to_string(),
                source_action: "price pavement evidence debt".to_string(),
                next_artifact: "data/standards-l1-inventory.csv".to_string(),
                optimizer_effect: "bundle remains service-addressable".to_string(),
                validation_status: "review".to_string(),
            },
            TierPavementSourceGapRow {
                tier: "T2".to_string(),
                route: "I220".to_string(),
                region_id: "component-1".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
                stitch_group_id: "US.HWYSTITCH.I220".to_string(),
                member_count: 10,
                blocker_count: 4,
                blocker_statuses: "pavement-repair-required".to_string(),
                affected_states: "TX".to_string(),
                affected_edge_ids: "4;5;6;7".to_string(),
                source_contract: "HPMS IRI plus state pavement feeds".to_string(),
                source_action: "price pavement repair debt".to_string(),
                next_artifact: "data/tier-pavement-docket.csv".to_string(),
                optimizer_effect: "bundle remains service-addressable".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows: Vec<TierPavementDebtBudgetRow> =
            tier_pavement_debt_budget_rows_with_exclusions(&gap_rows, &[], &[]);
        let failures = crate::support::pavement::tier_pavement_debt_budget_gate_failures::tier_pavement_debt_budget_gate_failures(&rows, &gap_rows, &[], &[]);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].debt_class, "evidence-debt");
        assert_eq!(rows[0].evidence_debt_units, 90);
        assert_eq!(rows[0].total_debt_cost_m, 4.5);
        assert_eq!(rows[1].debt_class, "repair-debt");
        assert_eq!(rows[1].repair_debt_units, 4);
        assert_eq!(rows[1].total_debt_cost_m, 10.0);
        assert!(rows[1].optimizer_penalty.contains("budget-cost"));
    }

    #[test]
    fn tier_pavement_debt_budget_applies_route_state_exclusions() {
        let gap_rows = vec![TierPavementSourceGapRow {
            tier: "T2".to_string(),
            route: "I220".to_string(),
            region_id: "component-0".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220TX".to_string(),
            stitch_group_id: "US.HWYSTITCH.I220TX".to_string(),
            member_count: 4,
            blocker_count: 4,
            blocker_statuses: "pavement-repair-required".to_string(),
            affected_states: "TX".to_string(),
            affected_edge_ids: "1;2;3;4".to_string(),
            source_contract: "HPMS IRI plus state pavement feeds".to_string(),
            source_action: "price pavement repair debt".to_string(),
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            optimizer_effect: "bundle remains service-addressable".to_string(),
            validation_status: "review".to_string(),
        }];
        let exclusion_rows = vec![TierPavementRouteStateExclusionRow {
            exclusion_id: "PAVEMENTROUTESTATEEXCLUSION-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220TX".to_string(),
            source_title: "FHWA Interstate Route Log and Finders List Table 2".to_string(),
            source_url_or_cache_artifact: "https://www.fhwa.dot.gov/planning/national_highway_system/interstate_highway_system/routefinder/table02.cfm".to_string(),
            capture_date: "2026-05-15".to_string(),
            excluded_member_count: 4,
            exclusion_basis: "FHWA table omits I-220 Texas".to_string(),
            exclusion_status: "route-state-not-supported".to_string(),
            validation_status: "pass".to_string(),
        }];

        let rows = tier_pavement_debt_budget_rows_with_exclusions(&gap_rows, &exclusion_rows, &[]);
        let failures =
            crate::support::pavement::tier_pavement_debt_budget_gate_failures::tier_pavement_debt_budget_gate_failures(&rows, &gap_rows, &exclusion_rows, &[]);

        assert!(failures.is_empty(), "{failures:?}");
        assert!(rows.is_empty());
    }

    #[test]
    fn tier_pavement_debt_budget_applies_full_cost_repair_funding() {
        let gap_rows = vec![TierPavementSourceGapRow {
            tier: "T2".to_string(),
            route: "I110".to_string(),
            region_id: "component-0".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I110LA".to_string(),
            stitch_group_id: "US.HWYSTITCH.I110LA".to_string(),
            member_count: 10,
            blocker_count: 10,
            blocker_statuses: "pavement-repair-required".to_string(),
            affected_states: "LA".to_string(),
            affected_edge_ids: "1;2;3;4;5;6;7;8;9;10".to_string(),
            source_contract: "HPMS IRI plus state pavement feeds".to_string(),
            source_action: "price pavement repair debt".to_string(),
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            optimizer_effect: "bundle remains service-addressable".to_string(),
            validation_status: "review".to_string(),
        }];
        let funding_rows = vec![TierPavementRepairFundingAcceptanceRow {
            acceptance_id: "PAVEMENTREPAIRFUNDINGACCEPT-LA-I110".to_string(),
            state: "LA".to_string(),
            tier: "T2".to_string(),
            route: "I110".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I110LA".to_string(),
            source_title: "Louisiana DOTD STIP".to_string(),
            source_url_or_cache_artifact: "https://dotd.louisiana.gov/stip.pdf".to_string(),
            capture_date: "2026-05-15".to_string(),
            committed_amount_m: 26.4,
            covered_repair_cost_m: 25.0,
            funding_basis: "STIP reconstruction covers full planning repair debt".to_string(),
            acceptance_status: "accepted-full-cost-repair-funding".to_string(),
            validation_status: "pass".to_string(),
        }];

        let rows = tier_pavement_debt_budget_rows_with_exclusions(&gap_rows, &[], &funding_rows);
        let failures =
            crate::support::pavement::tier_pavement_debt_budget_gate_failures::tier_pavement_debt_budget_gate_failures(&rows, &gap_rows, &[], &funding_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert!(rows.is_empty());
    }

    #[test]
    fn optimizer_constraint_ledger_normalizes_first_source_families() {
        let pavement_rows = vec![TierPavementDebtBudgetRow {
            tier: "T2".to_string(),
            route: "US30".to_string(),
            region_id: "component-1".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.US30".to_string(),
            stitch_group_id: "US.HWYSTITCH.US30".to_string(),
            debt_class: "repair-debt".to_string(),
            blocked_member_count: 4,
            affected_states: "IA".to_string(),
            evidence_debt_units: 0,
            repair_debt_units: 4,
            estimated_evidence_cost_m: 0.0,
            estimated_repair_cost_m: 10.0,
            total_debt_cost_m: 10.0,
            budget_basis: "fixture pavement debt".to_string(),
            optimizer_penalty: "subtract 10 budget-cost units".to_string(),
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let topology_rows = vec![T1TopologyRepairRow {
            route: "I84".to_string(),
            selected: true,
            design_role: "score-backbone-exception".to_string(),
            design_status: "policy-review".to_string(),
            beck_review_flag: "ok".to_string(),
            overlap_corridors: String::new(),
            repair_type: "national-relay-justification".to_string(),
            repair_basis: "selected-score-exception-needs-national-role-proof".to_string(),
            next_artifact: "data/t1-score-exceptions.csv".to_string(),
            next_action: "justify-as-national-relay-or-demote-to-t2".to_string(),
            validation_status: "review".to_string(),
        }];
        let parallel_rows = vec![T2ParallelServiceQueueRow {
            route: "__all_t2_parallel_services__".to_string(),
            region_id: String::new(),
            beck_corridor: String::new(),
            service_class: String::new(),
            close_parallel_count: 0,
            close_parallel_corridors: String::new(),
            selection_action: "clear".to_string(),
            selection_basis: "no-close-parallel-t2-services".to_string(),
            parallel_action: "no-parallel-service-work-needed".to_string(),
            required_artifact: "data/t2-service-selection.csv".to_string(),
            next_artifact: "data/game/t2-bundle-overlays.csv".to_string(),
            optimizer_effect: "all T2 service rows clear close-parallel review".to_string(),
            qualification_effects: String::new(),
            validation_status: "pass".to_string(),
        }];
        let access_gap_rows = vec![T3T4AccessGapRow {
            gap_id: "T3GAP-T3SOUTHEAST-US90Z".to_string(),
            source_surface: "t3-zone-route-columns".to_string(),
            route: "US90Z".to_string(),
            zone_id: "t3-southeast".to_string(),
            current_score: 29.9,
            constraint_adjusted_score: 29.9,
            hard_blocker_count: 0,
            claim_blocker_count: 0,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 0.0,
            top_constraint_classes: "none".to_string(),
            constraint_ledger_artifact: String::new(),
            promise_horizon_hours: 6,
            gap_class: "below-threshold-feeder".to_string(),
            gap_reason: "candidate is below T3 threshold for a 6h feeder obligation".to_string(),
            required_evidence: "score-or-terminal-evidence-required".to_string(),
            repair_action: "prove-terminal-evidence-or-keep-t4".to_string(),
            next_artifact: "data/t3-zone-map-diagnostics.csv".to_string(),
            upward_pressure_allowed: false,
            validation_status: "review".to_string(),
        }];
        let beck_t1_rows = vec![route_map::BeckT1DiagnosticRow {
            corridor: "I-40",
            endpoint_start: "BARSTOW",
            endpoint_end: "BEN",
            endpoint_status: "qualified",
            stop_count: 16,
            drawn_stop_count: 16,
            transfer_stop_count: 12,
            shared_stop_count: 4,
            shared_stop_corridors: "I-20;I-35;I-75;I-95".to_string(),
            shared_segment_count: 1,
            shared_segment_corridors: "I-95".to_string(),
            service_action: "overlap-review",
            qualification_basis: "shared-backbone-segment-needs-policy",
            review_flag: "overlap-review",
        }];
        let beck_t2_rows = vec![route_map::BeckT2DiagnosticRow {
            corridor: "I-25",
            trunk: "I-90",
            start_trunk: "I-90",
            end_trunk: "I-40",
            color_mode: "split-parent",
            service_class: "transfer-spine",
            split_anchor: "DEN",
            split_anchor_offset_pct: 1.0,
            unstopped_t1_contact_count: 0,
            unstopped_t1_contacts: String::new(),
            close_parallel_count: 0,
            close_parallel_corridors: String::new(),
            duplicate_service_count: 0,
            duplicate_service_corridors: String::new(),
            unique_duplicate_stop_count: 0,
            service_action: "keep",
            qualification_basis: "distinct-parent-service",
            service_label: "High Plains",
            stop_count: 9,
            drawn_stop_count: 9,
            transfer_stop_count: 6,
            schematic_length_px: 762.0,
            min_x: 624.0,
            min_y: 232.0,
            max_x: 776.0,
            max_y: 867.0,
            label_density_per_100px: 1.18,
            review_flag: "dense-transfer-review",
        }];
        let source_policy_rows = vec![source_fetch_policy_row(
            "t1-live-event-snapshots",
            "route t1-fetch-iowa511",
            "data/cache/*events.json",
            "live-snapshot-preserve",
            "failed fetches preserve the last usable snapshot",
            "atomic_write_text",
            "HTTP success",
        )];
        let scenario_hook_rows = vec![T2ScenarioHookRow {
            scenario_id: "atlanta-managed-lane-stress".to_string(),
            service_class: "transfer-spine".to_string(),
            t2_map_id: "beck-schematic-t2-only".to_string(),
            player_decision: "choose whether to harden a T2 merge spine".to_string(),
            evidence_hold: "managed-lane merge validation missing".to_string(),
        }];
        let bundle_overlay_rows = vec![T2BundleOverlayRow {
            tier: "T2".to_string(),
            region_id: "component-0".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            bundle_status: "needs-stop-chain".to_string(),
            service_class: "unclassified".to_string(),
            map_id: String::new(),
            scenario_hook: String::new(),
            incident_lever: String::new(),
            upgrade_lever: String::new(),
            restitch_lever: String::new(),
            release_gate: String::new(),
            qualification_map_treatment: String::new(),
            qualification_gate_policy: String::new(),
            qualification_game_use: String::new(),
            qualification_effects: String::new(),
            pavement_debt_cost_m: 35.0,
            pavement_debt_class: "repair-debt".to_string(),
            pavement_debt_basis: "route-level pavement debt rollup".to_string(),
            source_artifacts: "data/t2-service-selection.csv".to_string(),
            binding_status: "service-class-overlay-pending".to_string(),
            next_artifact: "data/game/t2-service-overlays.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = optimizer_constraint_ledger_rows(
            &pavement_rows,
            &[],
            &topology_rows,
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &parallel_rows,
            &access_gap_rows,
            &[],
            &beck_t1_rows,
            &beck_t2_rows,
            &source_policy_rows,
            &[],
            &scenario_hook_rows,
            &bundle_overlay_rows,
        );
        let failures = optimizer_constraint_ledger_gate_failures(&rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 9);
        assert!(rows
            .iter()
            .any(|row| row.constraint_class == "asset_condition_debt"
                && row.behavior_type == "budget-debt"));
        assert!(rows
            .iter()
            .any(|row| row.constraint_class == "promise_portfolio"
                && row.behavior_type == "selection-hard"
                && !row.exception_artifact.is_empty()));
        assert!(rows.iter().any(
            |row| row.constraint_class == "duplication_and_parallel_service"
                && row.constraint_status == "pass"
        ));
        assert!(rows
            .iter()
            .any(|row| row.constraint_class == "lower_tier_feeder_gap"
                && row.behavior_type == "claim-blocker"
                && row.source_artifact == "data/t3-t4-access-gaps.csv"));
        assert!(rows
            .iter()
            .any(|row| row.constraint_class == "beck_schematic_geometry"
                && row.source_artifact == "data/beck-t1-diagnostics.csv"));
        assert!(rows
            .iter()
            .any(|row| row.constraint_class == "beck_label_density"
                && row.source_artifact == "data/beck-t2-diagnostics.csv"));
        assert!(rows.iter().any(
            |row| row.constraint_class == "source_acquisition_snapshot_guard"
                && row.source_artifact == "data/source-fetch-policy.csv"
        ));
        assert!(rows.iter().any(
            |row| row.constraint_class == "game_ops_publication_readiness"
                && row.source_artifact == "data/game/t2-scenario-hooks.csv"
        ));
        assert!(rows
            .iter()
            .any(|row| row.constraint_class == "game_ops_bundle_binding"
                && row.source_artifact == "data/game/t2-bundle-overlays.csv"));
    }

    #[test]
    fn optimizer_constraint_ledger_replays_t1_schematic_relief() {
        let topology_rows = vec![T1TopologyRepairRow {
            route: "I40".to_string(),
            selected: true,
            design_role: "promise-spine".to_string(),
            design_status: "policy-review".to_string(),
            beck_review_flag: "overlap-review".to_string(),
            overlap_corridors: "I-95".to_string(),
            repair_type: "shared-backbone-policy".to_string(),
            repair_basis: "selected-t1-route-shares-beck-segment".to_string(),
            next_artifact: "data/t1-design-policy-actions.csv".to_string(),
            next_action: "resolve-shared-segment-map-policy".to_string(),
            validation_status: "review".to_string(),
        }];
        let relief_rows = vec![T1SchematicGeometryBlockerReliefRow {
            relief_id: "T1SCHEMATICRELIEF-I40I95".to_string(),
            acceptance_id: "T1SHAREDACCEPT-I40I95".to_string(),
            policy_id: "T1SHAREDSEG-I40I95".to_string(),
            route_pair: "I40-I95".to_string(),
            affected_routes: "I40;I95".to_string(),
            accepted_render_treatment:
                "represent as interlined trunk service or split at selected transfer stops"
                    .to_string(),
            relief_decision: "relief-ready-for-constraint-ledger-replay".to_string(),
            blocker_claims_before: "map;publication".to_string(),
            blocker_claims_after: String::new(),
            blocker_count_before: 4,
            blocker_count_after: 0,
            claim_blocker_delta: -4,
            ledger_replay_status: "pending-optimizer-constraint-ledger-replay".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let beck_t1_rows = vec![route_map::BeckT1DiagnosticRow {
            corridor: "I-40",
            endpoint_start: "BARSTOW",
            endpoint_end: "BEN",
            endpoint_status: "qualified",
            stop_count: 16,
            drawn_stop_count: 16,
            transfer_stop_count: 12,
            shared_stop_count: 4,
            shared_stop_corridors: "I-20;I-35;I-75;I-95".to_string(),
            shared_segment_count: 1,
            shared_segment_corridors: "I-95".to_string(),
            service_action: "overlap-review",
            qualification_basis: "shared-backbone-segment-needs-policy",
            review_flag: "overlap-review",
        }];

        let rows = optimizer_constraint_ledger_rows(
            &[],
            &[],
            &topology_rows,
            &relief_rows,
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &beck_t1_rows,
            &[],
            &[],
            &[],
            &[],
            &[],
        );
        let failures = optimizer_constraint_ledger_gate_failures(&rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert!(rows
            .iter()
            .any(|row| row.constraint_class == "schematic_geometry_relief"
                && row.constraint_status == "pass"));
        assert!(!rows
            .iter()
            .any(|row| row.constraint_class == "schematic_geometry"
                || row.constraint_class == "beck_schematic_geometry"));
    }

    #[test]
    fn optimizer_constraint_ledger_replays_t2_transfer_relief() {
        let relief_rows = vec![T2BeckTransferComplexityBlockerReliefRow {
            relief_id: "T2TRANSFERRELIEF-US80".to_string(),
            acceptance_id: "T2TRANSFERACCEPT-US80".to_string(),
            policy_id: "T2TRANSFERPOLICY-US80".to_string(),
            route: "US80".to_string(),
            complexity_band: "severe-transfer-complexity".to_string(),
            accepted_render_treatment:
                "compress transfer emphasis to trunk interfaces and preserve local stops as unlabeled service beads"
                    .to_string(),
            relief_decision: "relief-ready-for-constraint-ledger-replay".to_string(),
            blocker_claims_before: "map;promotion;publication".to_string(),
            blocker_claims_after: String::new(),
            blocker_count_before: 1,
            blocker_count_after: 0,
            claim_blocker_delta: -1,
            ledger_replay_status: "pending-optimizer-constraint-ledger-replay".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let beck_t2_rows = vec![route_map::BeckT2DiagnosticRow {
            corridor: "US80",
            trunk: "I-35",
            start_trunk: "I-35",
            end_trunk: "I-20",
            color_mode: "split-parent",
            service_class: "transfer-spine",
            split_anchor: "LITTLE_ROCK",
            split_anchor_offset_pct: 0.0,
            unstopped_t1_contact_count: 0,
            unstopped_t1_contacts: String::new(),
            close_parallel_count: 0,
            close_parallel_corridors: String::new(),
            duplicate_service_count: 0,
            duplicate_service_corridors: String::new(),
            unique_duplicate_stop_count: 8,
            service_action: "keep",
            qualification_basis: "distinct-parent-service",
            service_label: "Old South",
            stop_count: 8,
            drawn_stop_count: 8,
            transfer_stop_count: 7,
            schematic_length_px: 1164.0,
            min_x: 0.0,
            min_y: 0.0,
            max_x: 1.0,
            max_y: 1.0,
            label_density_per_100px: 0.69,
            review_flag: "transfer-complexity-review",
        }];

        let rows = optimizer_constraint_ledger_rows(
            &[],
            &[],
            &[],
            &[],
            &relief_rows,
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &beck_t2_rows,
            &[],
            &[],
            &[],
            &[],
        );
        let failures = optimizer_constraint_ledger_gate_failures(&rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert!(rows.iter().any(
            |row| row.constraint_class == "beck_transfer_complexity_relief"
                && row.constraint_status == "pass"
        ));
        assert!(!rows
            .iter()
            .any(|row| row.constraint_class == "beck_transfer_complexity"));
    }

    #[test]
    fn optimizer_constraint_ledger_replays_t2_label_density_relief() {
        let relief_rows = vec![T2BeckLabelDensityBlockerReliefRow {
            relief_id: "T2LABELRELIEF-I405".to_string(),
            acceptance_id: "T2LABELACCEPT-I405".to_string(),
            policy_id: "T2LABELPOLICY-I405".to_string(),
            route: "I405".to_string(),
            density_band: "severe-label-density".to_string(),
            accepted_render_treatment:
                "compress labels to trunk interfaces and preserve intermediate stops as unlabeled service beads"
                    .to_string(),
            relief_decision: "relief-ready-for-constraint-ledger-replay".to_string(),
            blocker_claims_before: "map;promotion;publication".to_string(),
            blocker_claims_after: String::new(),
            blocker_count_before: 1,
            blocker_count_after: 0,
            claim_blocker_delta: -1,
            ledger_replay_status: "pending-optimizer-constraint-ledger-replay".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let beck_t2_rows = vec![route_map::BeckT2DiagnosticRow {
            corridor: "I405",
            trunk: "I-5",
            start_trunk: "I-5",
            end_trunk: "I-10",
            color_mode: "split-parent",
            service_class: "transfer-spine",
            split_anchor: "LOS_ANGELES",
            split_anchor_offset_pct: 0.0,
            unstopped_t1_contact_count: 0,
            unstopped_t1_contacts: String::new(),
            close_parallel_count: 0,
            close_parallel_corridors: String::new(),
            duplicate_service_count: 0,
            duplicate_service_corridors: String::new(),
            unique_duplicate_stop_count: 5,
            service_action: "keep",
            qualification_basis: "distinct-parent-service",
            service_label: "LA Basin Relief",
            stop_count: 5,
            drawn_stop_count: 5,
            transfer_stop_count: 5,
            schematic_length_px: 386.3,
            min_x: 0.0,
            min_y: 0.0,
            max_x: 1.0,
            max_y: 1.0,
            label_density_per_100px: 1.29,
            review_flag: "dense-transfer-review",
        }];

        let rows = optimizer_constraint_ledger_rows(
            &[],
            &[],
            &[],
            &[],
            &[],
            &relief_rows,
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &beck_t2_rows,
            &[],
            &[],
            &[],
            &[],
        );
        let failures = optimizer_constraint_ledger_gate_failures(&rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert!(rows
            .iter()
            .any(|row| row.constraint_class == "beck_label_density_relief"
                && row.constraint_status == "pass"));
        assert!(!rows
            .iter()
            .any(|row| row.constraint_class == "beck_label_density"));
    }

    #[test]
    fn optimizer_constraint_ledger_replays_t2_long_connector_relief() {
        let relief_rows = vec![T2BeckLongConnectorBlockerReliefRow {
            relief_id: "T2LONGRELIEF-US83".to_string(),
            acceptance_id: "T2LONGACCEPT-US83".to_string(),
            policy_id: "T2LONGPOLICY-US83".to_string(),
            route: "US83".to_string(),
            connector_band: "severe-long-connector".to_string(),
            accepted_render_treatment:
                "preserve connector service but require trunk-interface labeling and explicit local-service beads"
                    .to_string(),
            relief_decision: "relief-ready-for-constraint-ledger-replay".to_string(),
            blocker_claims_before: "map;promotion;publication".to_string(),
            blocker_claims_after: String::new(),
            blocker_count_before: 1,
            blocker_count_after: 0,
            claim_blocker_delta: -1,
            ledger_replay_status: "pending-optimizer-constraint-ledger-replay".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let beck_t2_rows = vec![route_map::BeckT2DiagnosticRow {
            corridor: "US83",
            trunk: "I-90",
            start_trunk: "I-90",
            end_trunk: "I-10",
            color_mode: "split-parent",
            service_class: "long-connector",
            split_anchor: "ODESSA",
            split_anchor_offset_pct: 21.0,
            unstopped_t1_contact_count: 0,
            unstopped_t1_contacts: String::new(),
            close_parallel_count: 0,
            close_parallel_corridors: String::new(),
            duplicate_service_count: 0,
            duplicate_service_corridors: String::new(),
            unique_duplicate_stop_count: 5,
            service_action: "keep",
            qualification_basis: "long-connector-service",
            service_label: "Plains Connector",
            stop_count: 5,
            drawn_stop_count: 5,
            transfer_stop_count: 1,
            schematic_length_px: 1446.0,
            min_x: 0.0,
            min_y: 0.0,
            max_x: 1.0,
            max_y: 1.0,
            label_density_per_100px: 0.35,
            review_flag: "long-connector-review",
        }];

        let rows = optimizer_constraint_ledger_rows(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &relief_rows,
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &beck_t2_rows,
            &[],
            &[],
            &[],
            &[],
        );
        let failures = optimizer_constraint_ledger_gate_failures(&rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert!(rows
            .iter()
            .any(|row| row.constraint_class == "beck_long_connector_relief"
                && row.constraint_status == "pass"));
        assert!(!rows
            .iter()
            .any(|row| row.constraint_class == "beck_long_connector"));
    }

    #[test]
    fn optimizer_constraint_ledger_replays_t2_game_publication_relief() {
        let relief_rows = vec![T2GamePublicationEvidenceBlockerReliefRow {
            relief_id: "T2GAMERELIEF-HOUSTONPORTSURGE".to_string(),
            acceptance_id: "T2GAMEACCEPT-HOUSTONPORTSURGE".to_string(),
            policy_id: "T2GAMEPOLICY-HOUSTONPORTSURGE".to_string(),
            scenario_id: "houston-port-surge".to_string(),
            service_class: "long-connector".to_string(),
            accepted_required_evidence: "port-surge-demand-and-flood-closure-evidence".to_string(),
            relief_decision: "relief-ready-for-constraint-ledger-replay".to_string(),
            blocker_claims_before: "game;publication;upgrade".to_string(),
            blocker_claims_after: String::new(),
            blocker_count_before: 1,
            blocker_count_after: 0,
            claim_blocker_delta: -1,
            ledger_replay_status: "pending-optimizer-constraint-ledger-replay".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let scenario_hook_rows = vec![T2ScenarioHookRow {
            scenario_id: "houston-port-surge".to_string(),
            service_class: "long-connector".to_string(),
            t2_map_id: "beck-schematic-t2-only".to_string(),
            player_decision: "choose whether to harden a port surge route".to_string(),
            evidence_hold: "port surge demand and flood closure evidence missing".to_string(),
        }];

        let rows = optimizer_constraint_ledger_rows(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &relief_rows,
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &scenario_hook_rows,
            &[],
        );
        let failures = optimizer_constraint_ledger_gate_failures(&rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert!(rows.iter().any(|row| row.constraint_class
            == "game_ops_publication_readiness_relief"
            && row.constraint_status == "pass"));
        assert!(!rows
            .iter()
            .any(|row| row.constraint_class == "game_ops_publication_readiness"));
    }

    #[test]
    fn optimizer_constraint_ledger_replays_t2_game_ops_bundle_relief() {
        let relief_rows = vec![T2GameOpsBundleEvidenceBlockerReliefRow {
            relief_id: "T2GAMEOPSRELIEF-I110-LA".to_string(),
            acceptance_id: "T2GAMEOPSACCEPT-I110-LA".to_string(),
            policy_id: "T2GAMEOPSPOLICY-I110-LA".to_string(),
            route: "I-110".to_string(),
            segment_bundle_id: "i110-la".to_string(),
            accepted_required_evidence: "game-ops-bundle-binding-evidence".to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            qualification_gate_policy: "accepted when structural diagnostics pass".to_string(),
            qualification_game_use:
                "default playable service for incidents, upgrades, and restitches".to_string(),
            relief_decision: "relief-ready-for-constraint-ledger-replay".to_string(),
            blocker_claims_before: "game;incident;publication;sla;transit;upgrade".to_string(),
            blocker_claims_after: String::new(),
            blocker_count_before: 6,
            blocker_count_after: 0,
            claim_blocker_delta: -6,
            ledger_replay_status: "pending-optimizer-constraint-ledger-replay".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let bundle_rows = vec![T2BundleOverlayRow {
            tier: "T2".to_string(),
            region_id: "game-campaign".to_string(),
            route: "I-110".to_string(),
            segment_bundle_id: "i110-la".to_string(),
            bundle_status: "candidate".to_string(),
            service_class: "long-connector".to_string(),
            map_id: "beck-schematic-t2-only".to_string(),
            scenario_hook: "los-angeles-port-access".to_string(),
            incident_lever: "incident-response".to_string(),
            upgrade_lever: "port-access-upgrade".to_string(),
            restitch_lever: "restitch-route".to_string(),
            release_gate: "review".to_string(),
            qualification_map_treatment: String::new(),
            qualification_gate_policy: String::new(),
            qualification_game_use: String::new(),
            qualification_effects: String::new(),
            pavement_debt_cost_m: 0.0,
            pavement_debt_class: String::new(),
            pavement_debt_basis: String::new(),
            source_artifacts: "docs/game/interstate-tycoon-plan.md".to_string(),
            binding_status: "bundle-binding-pending".to_string(),
            next_artifact: "data/t2-game-ops-bundle-evidence-policy.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = optimizer_constraint_ledger_rows(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &relief_rows,
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &bundle_rows,
        );
        let failures = optimizer_constraint_ledger_gate_failures(&rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert!(rows.iter().any(
            |row| row.constraint_class == "game_ops_bundle_binding_relief"
                && row.constraint_status == "pass"
                && row.subject_id == "i110-la"
                && row.optimizer_effect.contains(
                    "qualification_effects=qualification_game_use=default-play|qualification_gate_policy=stop-first"
                )
                && row.optimizer_effect.contains("qualification_gate_policy=")
        ));
        assert!(!rows
            .iter()
            .any(|row| row.constraint_class == "game_ops_bundle_binding"));
    }

    #[test]
    fn optimizer_constraint_ledger_preserves_parallel_service_qualification_effects() {
        let parallel_rows = vec![T2ParallelServiceQueueRow {
            route: "I59".to_string(),
            region_id: "component-1".to_string(),
            beck_corridor: "I-59".to_string(),
            service_class: "connector".to_string(),
            close_parallel_count: 1,
            close_parallel_corridors: "I-65".to_string(),
            selection_action: "split-parallel-service".to_string(),
            selection_basis: "close-parallel-beck-service".to_string(),
            parallel_action: "review-spacing-or-split-service-before-promotion".to_string(),
            required_artifact: "data/t2-service-selection.csv".to_string(),
            next_artifact: "docs/t2-regional-treatment.md".to_string(),
            optimizer_effect:
                "keeps close-parallel T2 line visible but below automatic keep/promotion; qualification_effects=qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            qualification_effects:
                "qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = optimizer_constraint_ledger_rows(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &parallel_rows,
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        );
        let failures = optimizer_constraint_ledger_gate_failures(&rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].constraint_class, "duplication_and_parallel_service");
        assert_eq!(rows[0].constraint_status, "review");
        assert!(rows[0]
            .optimizer_effect
            .contains("qualification_gate_policy=stop-first"));
    }

    #[test]
    fn optimizer_constraint_ledger_replays_t3_feeder_relief() {
        let relief_rows = vec![T3LowerTierFeederGapBlockerReliefRow {
            relief_id: "T3FEEDERRELIEF-I135".to_string(),
            acceptance_id: "T3FEEDERACCEPT-I135".to_string(),
            policy_id: "T3FEEDERPOLICY-I135".to_string(),
            route: "I-135".to_string(),
            zone_id: "t3-mountain-west".to_string(),
            score_band: "near-threshold-feeder".to_string(),
            accepted_map_treatment: "keep route below T3 feeder promotion".to_string(),
            relief_decision: "relief-ready-for-constraint-ledger-replay".to_string(),
            blocker_claims_before: "map;publication;upgrade".to_string(),
            blocker_claims_after: String::new(),
            blocker_count_before: 1,
            blocker_count_after: 0,
            claim_blocker_delta: -1,
            ledger_replay_status: "pending-optimizer-constraint-ledger-replay".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let access_gap_rows = vec![T3T4AccessGapRow {
            gap_id: "T3GAP-T3MOUNTAINWEST-I135".to_string(),
            source_surface: "t3-zone-route-columns".to_string(),
            route: "I-135".to_string(),
            zone_id: "t3-mountain-west".to_string(),
            current_score: 29.8,
            constraint_adjusted_score: 27.8,
            hard_blocker_count: 0,
            claim_blocker_count: 1,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 1.0,
            top_constraint_classes: "lower_tier_feeder_gap".to_string(),
            constraint_ledger_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            promise_horizon_hours: 6,
            gap_class: "below-threshold-feeder".to_string(),
            gap_reason: "candidate is below T3 threshold for a 6h feeder obligation".to_string(),
            required_evidence: "score-or-terminal-evidence-required".to_string(),
            repair_action: "prove-terminal-evidence-or-keep-t4".to_string(),
            next_artifact: "data/t3-lower-tier-feeder-gap-policy.csv".to_string(),
            upward_pressure_allowed: false,
            validation_status: "review".to_string(),
        }];

        let rows = optimizer_constraint_ledger_rows(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &relief_rows,
            &[],
            &access_gap_rows,
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        );
        let failures = optimizer_constraint_ledger_gate_failures(&rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert!(rows
            .iter()
            .any(|row| row.constraint_class == "lower_tier_feeder_gap_relief"
                && row.constraint_status == "pass"));
        assert!(!rows
            .iter()
            .any(|row| row.constraint_class == "lower_tier_feeder_gap"));
    }

    #[test]
    fn optimizer_constraint_ledger_applies_t4_terminal_access_map_exclusion() {
        let access_gap_rows = vec![T3T4AccessGapRow {
            gap_id: "T4GAP-T3GREATLAKES-I115".to_string(),
            source_surface: "t4-terminal-access-columns".to_string(),
            route: "I-115".to_string(),
            zone_id: "t3-great-lakes".to_string(),
            current_score: 21.0,
            constraint_adjusted_score: 20.0,
            hard_blocker_count: 0,
            claim_blocker_count: 1,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 1.0,
            top_constraint_classes: "terminal_access_evidence_gap".to_string(),
            constraint_ledger_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            promise_horizon_hours: 1,
            gap_class: "terminal-evidence-needed".to_string(),
            gap_reason: "route-to-terminal contact proof is seed-only".to_string(),
            required_evidence: "non-seed route-to-terminal contact proof".to_string(),
            repair_action: "attach-non-seed-terminal-access-proof".to_string(),
            next_artifact: "data/t4-terminal-access-proof-artifact-source-access.csv".to_string(),
            upward_pressure_allowed: false,
            validation_status: "review".to_string(),
        }];
        let exclusion_rows = vec![T4TerminalAccessMapExclusionRow {
            decision_id: "T4TERM-MAPEXCL-TEST".to_string(),
            decision_scope: "terminal-access-overlay-publication-scope".to_string(),
            source_artifact: "data/t3-t4-access-gaps.csv".to_string(),
            affected_constraint_class: "terminal_access_evidence_gap".to_string(),
            affected_gap_class: "terminal-evidence-needed".to_string(),
            affected_tier: "T4".to_string(),
            affected_claims_before: "upgrade|map|publication".to_string(),
            excluded_claims: "map|publication".to_string(),
            preserved_claims_after: "upgrade".to_string(),
            affected_route_count: 1,
            decision: "exclude-terminal-access-overlay-from-map-publication".to_string(),
            decision_basis: "render map without claiming terminal-access proof".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "accepted".to_string(),
        }];

        let rows = optimizer_constraint_ledger_rows(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &access_gap_rows,
            &exclusion_rows,
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        );

        let terminal_row = rows
            .iter()
            .find(|row| row.constraint_class == "terminal_access_evidence_gap")
            .expect("terminal access ledger row");

        assert_eq!(terminal_row.blocks_claims, "upgrade");
        assert_eq!(terminal_row.evidence_status, "exception");
        assert_eq!(
            terminal_row.exception_artifact,
            "data/t4-terminal-access-map-exclusion.csv"
        );
        assert!(!terminal_row.blocks_claims.contains("map"));
        assert!(!terminal_row.blocks_claims.contains("publication"));
    }

    #[test]
    fn optimizer_constraint_ledger_applies_source_snapshot_publication_exclusion() {
        let source_policy_rows = vec![source_fetch_policy_row(
            "t1-live-event-snapshots",
            "source-fetch --family t1-live-event-snapshots",
            "data/source-cache/live-events",
            "live-snapshot-preserve",
            "live event snapshots require repeat windows before evidence claims",
            "preserve live-source mutation history",
            "repeat-window-required",
        )];
        let exclusion_rows = vec![SourceSnapshotPublicationExclusionRow {
            decision_id: "SOURCE-SNAPSHOT-PUBEXCL-TEST".to_string(),
            decision_scope: "source-snapshot-publication-scope".to_string(),
            source_artifact: "data/source-fetch-policy.csv".to_string(),
            affected_constraint_class: "source_acquisition_snapshot_guard".to_string(),
            affected_fetch_family: "t1-live-event-snapshots".to_string(),
            affected_claims_before: "evidence|publication".to_string(),
            excluded_claims: "publication".to_string(),
            preserved_claims_after: "evidence".to_string(),
            decision: "exclude-live-snapshot-guard-from-map-publication".to_string(),
            decision_basis: "render map without claiming live-event snapshot evidence".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "accepted".to_string(),
        }];

        let rows = optimizer_constraint_ledger_rows(
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &source_policy_rows,
            &exclusion_rows,
            &[],
            &[],
        );

        let snapshot_row = rows
            .iter()
            .find(|row| row.constraint_class == "source_acquisition_snapshot_guard")
            .expect("source snapshot guard ledger row");

        assert_eq!(snapshot_row.blocks_claims, "evidence");
        assert_eq!(
            snapshot_row.exception_artifact,
            "data/source-snapshot-publication-exclusion.csv"
        );
        assert_eq!(snapshot_row.exception_id, "SOURCE-SNAPSHOT-PUBEXCL-TEST");
        assert!(!snapshot_row.blocks_claims.contains("publication"));
    }

    #[test]
    fn optimizer_constraint_ledger_applies_t2_asset_condition_map_publication_exclusion() {
        let pavement_rows = vec![TierPavementDebtBudgetRow {
            tier: "T2".to_string(),
            route: "US30".to_string(),
            region_id: "midwest".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.US30".to_string(),
            stitch_group_id: "US.HWYSTITCH.US30".to_string(),
            debt_class: "repair-debt".to_string(),
            blocked_member_count: 2,
            affected_states: "IA".to_string(),
            evidence_debt_units: 0,
            repair_debt_units: 2,
            estimated_evidence_cost_m: 0.0,
            estimated_repair_cost_m: 12.0,
            total_debt_cost_m: 12.0,
            budget_basis: "route-level pavement debt rollup".to_string(),
            optimizer_penalty: "repair debt remains payable".to_string(),
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let exclusion_rows = vec![T2AssetConditionMapPublicationExclusionRow {
            decision_id: "T2ASSET-PUBEXCL-TEST".to_string(),
            decision_scope: "asset-condition-map-publication-scope".to_string(),
            source_artifact: "data/tier-pavement-debt-budget.csv".to_string(),
            affected_constraint_class: "asset_condition_debt".to_string(),
            affected_tier: "T2".to_string(),
            affected_claims_before: "sla|transit|upgrade|publication".to_string(),
            excluded_claims: "publication".to_string(),
            preserved_claims_after: "sla|transit|upgrade".to_string(),
            affected_bundle_count: 1,
            total_debt_cost_m: 12.0,
            decision: "exclude-asset-condition-debt-from-map-publication".to_string(),
            decision_basis: "render map without claiming pavement repair completion".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "accepted".to_string(),
        }];

        let rows = optimizer_constraint_ledger_rows(
            &pavement_rows,
            &exclusion_rows,
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
        );

        let asset_row = rows
            .iter()
            .find(|row| row.constraint_class == "asset_condition_debt")
            .expect("asset condition ledger row");

        assert_eq!(asset_row.blocks_claims, "sla|transit|upgrade");
        assert_eq!(
            asset_row.exception_artifact,
            "data/t2-asset-condition-map-publication-exclusion.csv"
        );
        assert_eq!(asset_row.exception_id, "T2ASSET-PUBEXCL-TEST");
        assert!(!asset_row.blocks_claims.contains("publication"));
    }

    #[test]
    fn optimizer_constraint_budget_rolls_up_ledger_subjects() {
        let ledger_rows = vec![
            OptimizerConstraintLedgerRow {
                constraint_id: "CON-PAVEMENT-US30".to_string(),
                optimizer_run_id: "run-1".to_string(),
                tier: "T2".to_string(),
                region_id: "component-1".to_string(),
                constraint_order: 8,
                constraint_class: "asset_condition_debt".to_string(),
                behavior_type: "budget-debt".to_string(),
                constraint_scope: "bundle".to_string(),
                subject_id: "US.HWYBUNDLE.US30".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.US30".to_string(),
                national_segment_id: String::new(),
                stitch_group_id: "US.HWYSTITCH.US30".to_string(),
                route: "US30".to_string(),
                stop_id: String::new(),
                pair_id: String::new(),
                map_id: String::new(),
                source_artifact: "data/tier-pavement-debt-budget.csv".to_string(),
                source_row_id: "US.HWYBUNDLE.US30".to_string(),
                standard_artifact: "docs/tier-pavement-standards.md".to_string(),
                evidence_status: "accepted".to_string(),
                constraint_status: "debt".to_string(),
                observed_value: "4".to_string(),
                threshold_value: "0".to_string(),
                measurement_unit: "blocked_members".to_string(),
                blocks_claims: "sla|publication".to_string(),
                budget_cost_m: 10.0,
                cost_category: "capital_repair".to_string(),
                cost_basis: "fixture".to_string(),
                cost_confidence: "planning_proxy".to_string(),
                budget_units: "repair_members=4".to_string(),
                penalty_score: 10.0,
                repair_action: "pay_debt".to_string(),
                payment_action: "fund_pavement_repair".to_string(),
                owner_jurisdiction: "IA".to_string(),
                funding_program: "state_dot_hpms_or_nhpp".to_string(),
                delivery_risk: "medium".to_string(),
                exception_id: String::new(),
                exception_artifact: String::new(),
                next_artifact: "data/tier-pavement-docket.csv".to_string(),
                optimizer_effect:
                    "subtract budget cost; qualification_effects=qualification_game_use=default-play|qualification_gate_policy=stop-first; qualification_gate_policy=accepted when structural diagnostics pass"
                        .to_string(),
                validation_status: "review".to_string(),
            },
            OptimizerConstraintLedgerRow {
                constraint_id: "CON-T1TOPO-I84".to_string(),
                optimizer_run_id: "run-1".to_string(),
                tier: "T1".to_string(),
                region_id: "national".to_string(),
                constraint_order: 1,
                constraint_class: "promise_portfolio".to_string(),
                behavior_type: "selection-hard".to_string(),
                constraint_scope: "route".to_string(),
                subject_id: "I84".to_string(),
                segment_bundle_id: String::new(),
                national_segment_id: String::new(),
                stitch_group_id: String::new(),
                route: "I84".to_string(),
                stop_id: String::new(),
                pair_id: String::new(),
                map_id: "beck-schematic".to_string(),
                source_artifact: "data/t1-topology-repairs.csv".to_string(),
                source_row_id: "I84".to_string(),
                standard_artifact: "docs/tier-optimizer-design.md".to_string(),
                evidence_status: "exception".to_string(),
                constraint_status: "review".to_string(),
                observed_value: "policy-review".to_string(),
                threshold_value: "accepted".to_string(),
                measurement_unit: "design_status".to_string(),
                blocks_claims: "sla|publication".to_string(),
                budget_cost_m: 0.0,
                cost_category: String::new(),
                cost_basis: String::new(),
                cost_confidence: String::new(),
                budget_units: String::new(),
                penalty_score: 1.0,
                repair_action: "justify-as-national-relay-or-demote-to-t2".to_string(),
                payment_action: String::new(),
                owner_jurisdiction: "route-program".to_string(),
                funding_program: String::new(),
                delivery_risk: "unknown".to_string(),
                exception_id: "justify-as-national-relay-or-demote-to-t2".to_string(),
                exception_artifact: "data/t1-score-exceptions.csv".to_string(),
                next_artifact: "data/t1-score-exceptions.csv".to_string(),
                optimizer_effect: "selected-score-exception-needs-national-role-proof".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = optimizer_constraint_budget_rows(&ledger_rows);
        let failures = optimizer_constraint_budget_gate_failures(&rows, &ledger_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        let bundle_row = rows
            .iter()
            .find(|row| row.subject_scope == "bundle")
            .expect("bundle budget row");
        assert_eq!(bundle_row.constraint_debt_cost_m, 10.0);
        assert_eq!(bundle_row.claim_blocker_count, 0);
        assert_eq!(bundle_row.blocking_claims, "publication;sla");
        assert_eq!(
            bundle_row.qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=accepted when structural diagnostics pass|qualification_gate_policy=stop-first"
        );
        let route_row = rows
            .iter()
            .find(|row| row.subject_id == "I84")
            .expect("route budget row");
        assert_eq!(route_row.hard_blocker_count, 1);
        assert_eq!(route_row.validation_status, "blocked");
        assert_eq!(route_row.constraint_penalty_score, 1.0);
    }

    #[test]
    fn optimizer_constraint_budget_extracts_parallel_qualification_effects() {
        let ledger_rows = vec![OptimizerConstraintLedgerRow {
            constraint_id: "CON-T2PAR-I59".to_string(),
            optimizer_run_id: "run-1".to_string(),
            tier: "T2".to_string(),
            region_id: "component-1".to_string(),
            constraint_order: 11,
            constraint_class: "duplication_and_parallel_service".to_string(),
            behavior_type: "penalty-soft".to_string(),
            constraint_scope: "route".to_string(),
            subject_id: "I59".to_string(),
            segment_bundle_id: String::new(),
            national_segment_id: String::new(),
            stitch_group_id: String::new(),
            route: "I59".to_string(),
            stop_id: String::new(),
            pair_id: String::new(),
            map_id: "beck-schematic-t2-only".to_string(),
            source_artifact: "data/t2-parallel-service-queue.csv".to_string(),
            source_row_id: "I59".to_string(),
            standard_artifact: "docs/t2-regional-treatment.md".to_string(),
            evidence_status: "accepted".to_string(),
            constraint_status: "review".to_string(),
            observed_value: "1".to_string(),
            threshold_value: "0".to_string(),
            measurement_unit: "close_parallel_services".to_string(),
            blocks_claims: "promotion|map|publication".to_string(),
            budget_cost_m: 0.0,
            cost_category: String::new(),
            cost_basis: String::new(),
            cost_confidence: String::new(),
            budget_units: String::new(),
            penalty_score: 1.0,
            repair_action: "review-spacing-or-split-service-before-promotion".to_string(),
            payment_action: String::new(),
            owner_jurisdiction: "route-program".to_string(),
            funding_program: String::new(),
            delivery_risk: "unknown".to_string(),
            exception_id: String::new(),
            exception_artifact: String::new(),
            next_artifact: "docs/t2-regional-treatment.md".to_string(),
            optimizer_effect:
                "keeps close-parallel T2 line visible but below automatic keep/promotion; qualification_effects=qualification_game_use=default-play|qualification_gate_policy=stop-first"
                    .to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = optimizer_constraint_budget_rows(&ledger_rows);
        let failures = optimizer_constraint_budget_gate_failures(&rows, &ledger_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].subject_scope, "route");
        assert_eq!(rows[0].subject_id, "I59");
        assert_eq!(
            rows[0].qualification_effects,
            "qualification_game_use=default-play|qualification_gate_policy=stop-first"
        );
    }

    #[test]
    fn optimizer_residual_blocker_backlog_groups_without_relief() {
        let budget_rows = vec![
            OptimizerConstraintBudgetRow {
                budget_id: "CB-T2-BUNDLE-A".to_string(),
                optimizer_run_id: "tier-optimizer-current".to_string(),
                tier: "T2".to_string(),
                region_id: "component-0".to_string(),
                subject_scope: "bundle".to_string(),
                subject_id: "US.HWYBUNDLE.A".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.A".to_string(),
                route: "I205".to_string(),
                ledger_row_count: 1,
                hard_blocker_count: 0,
                claim_blocker_count: 1,
                review_count: 1,
                budget_debt_count: 0,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 1.0,
                top_constraint_classes: "game_ops_bundle_binding".to_string(),
                blocking_claims: "game;incident;publication;upgrade".to_string(),
                qualification_effects: String::new(),
                next_artifacts: "data/game/t2-service-overlays.csv".to_string(),
                constraint_ledger_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
                validation_status: "review".to_string(),
            },
            OptimizerConstraintBudgetRow {
                budget_id: "CB-T2-BUNDLE-B".to_string(),
                optimizer_run_id: "tier-optimizer-current".to_string(),
                tier: "T2".to_string(),
                region_id: "component-0".to_string(),
                subject_scope: "bundle".to_string(),
                subject_id: "US.HWYBUNDLE.B".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.B".to_string(),
                route: "US30".to_string(),
                ledger_row_count: 1,
                hard_blocker_count: 0,
                claim_blocker_count: 0,
                review_count: 1,
                budget_debt_count: 1,
                constraint_debt_cost_m: 5.85,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 5.85,
                top_constraint_classes: "asset_condition_debt".to_string(),
                blocking_claims: "publication;sla;transit;upgrade".to_string(),
                qualification_effects: String::new(),
                next_artifacts: "data/tier-pavement-acquisition-plan.csv".to_string(),
                constraint_ledger_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
                validation_status: "review".to_string(),
            },
            OptimizerConstraintBudgetRow {
                budget_id: "CB-T2-BUNDLE-C".to_string(),
                optimizer_run_id: "tier-optimizer-current".to_string(),
                tier: "T2".to_string(),
                region_id: "component-0".to_string(),
                subject_scope: "bundle".to_string(),
                subject_id: "US.HWYBUNDLE.C".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.C".to_string(),
                route: "I110".to_string(),
                ledger_row_count: 2,
                hard_blocker_count: 0,
                claim_blocker_count: 0,
                review_count: 1,
                budget_debt_count: 1,
                constraint_debt_cost_m: 5.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 5.0,
                top_constraint_classes: "asset_condition_debt|game_ops_bundle_binding_relief"
                    .to_string(),
                blocking_claims: "publication;sla;transit;upgrade".to_string(),
                qualification_effects: String::new(),
                next_artifacts:
                    "data/optimizer-constraint-budget.csv;data/tier-pavement-docket.csv".to_string(),
                constraint_ledger_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = optimizer_residual_blocker_backlog_rows(&budget_rows);
        let failures = optimizer_residual_blocker_backlog_gate_failures(&rows, &budget_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert_eq!(
            rows.iter()
                .filter(|row| row.priority_class == "P1-game-claim")
                .count(),
            1
        );
        assert_eq!(
            rows.iter()
                .map(|row| row.total_claim_blockers)
                .sum::<usize>(),
            1
        );
        assert_eq!(
            rows.iter()
                .map(|row| row.total_budget_debt_count)
                .sum::<usize>(),
            2
        );
        assert!(rows
            .iter()
            .all(|row| row.backlog_decision == "triage-only-no-blocker-relief"));
    }

    #[test]
    fn optimizer_claim_review_preserves_p1_claim_blockers() {
        let backlog_rows = vec![
            OptimizerResidualBlockerBacklogRow {
                backlog_id: "ORB-P1-claim-blocker-T2-BECKTRANSFERCOMPLEXITY".to_string(),
                priority_class: "P1-claim-blocker".to_string(),
                blocker_family: "beck_transfer_complexity".to_string(),
                tier: "T2".to_string(),
                blocked_claims: "map;promotion;publication".to_string(),
                subject_count: 6,
                route_count: 6,
                total_hard_blockers: 0,
                total_claim_blockers: 6,
                total_budget_debt_count: 0,
                total_constraint_debt_cost_m: 0.0,
                total_constraint_penalty_score: 15.0,
                representative_routes: "I65;I81;US30".to_string(),
                representative_subjects: "I65;I81;US30".to_string(),
                next_artifacts: "data/beck-t2-diagnostics.csv".to_string(),
                backlog_decision: "triage-only-no-blocker-relief".to_string(),
                next_wave: "optimizer-claim-review".to_string(),
                validation_status: "review".to_string(),
            },
            OptimizerResidualBlockerBacklogRow {
                backlog_id: "ORB-P1-game-claim-T2-GAMEOPSBUNDLEBINDING".to_string(),
                priority_class: "P1-game-claim".to_string(),
                blocker_family: "game_ops_bundle_binding".to_string(),
                tier: "T2".to_string(),
                blocked_claims: "game;incident;publication;upgrade".to_string(),
                subject_count: 16,
                route_count: 16,
                total_hard_blockers: 0,
                total_claim_blockers: 16,
                total_budget_debt_count: 1,
                total_constraint_debt_cost_m: 5.0,
                total_constraint_penalty_score: 21.0,
                representative_routes: "I110;I195".to_string(),
                representative_subjects: "US.HWYBUNDLE.A".to_string(),
                next_artifacts: "data/game/t2-service-overlays.csv".to_string(),
                backlog_decision: "triage-only-no-blocker-relief".to_string(),
                next_wave: "game-ops-blocker-evidence-review".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = optimizer_claim_review_rows(&backlog_rows);
        let failures = optimizer_claim_review_gate_failures(&rows, &backlog_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].priority_class, "P1-claim-blocker");
        assert_eq!(rows[0].blocker_claims_before, "map;promotion;publication");
        assert_eq!(rows[0].blocker_claims_after, "map;promotion;publication");
        assert_eq!(rows[0].claim_blocker_delta, 0);
        assert_eq!(
            rows[0].review_decision,
            "held-for-source-specific-claim-review"
        );
    }

    #[test]
    fn t2_game_publication_evidence_review_preserves_scenario_blockers() {
        let claim_rows = vec![OptimizerClaimReviewRow {
            claim_review_id: "OCR-GAMEPUB".to_string(),
            backlog_id: "ORB-P1-claim-blocker-T2-GAMEOPSPUBLICATIONREADINESS".to_string(),
            priority_class: "P1-claim-blocker".to_string(),
            blocker_family: "game_ops_publication_readiness".to_string(),
            tier: "T2".to_string(),
            blocked_claims: "game;publication;upgrade".to_string(),
            subject_count: 2,
            route_count: 0,
            total_claim_blockers: 2,
            representative_routes: String::new(),
            representative_subjects: "atlanta-managed-lane-stress;houston-port-surge".to_string(),
            evidence_artifacts: "data/game/t2-scenario-hooks.csv".to_string(),
            review_decision: "held-for-source-specific-claim-review".to_string(),
            blocker_claims_before: "game;publication;upgrade".to_string(),
            blocker_claims_after: "game;publication;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_artifact: "data/game/t2-scenario-hooks.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let hook_rows = vec![
            T2ScenarioHookRow {
                scenario_id: "atlanta-managed-lane-stress".to_string(),
                service_class: "transfer-spine".to_string(),
                t2_map_id: "beck-schematic-t2-only".to_string(),
                player_decision: "choose whether to harden a throughput-sensitive T2 merge spine"
                    .to_string(),
                evidence_hold: "managed-lane merge and spillback validation missing".to_string(),
            },
            T2ScenarioHookRow {
                scenario_id: "houston-port-surge".to_string(),
                service_class: "long-connector".to_string(),
                t2_map_id: "beck-schematic-t2-only".to_string(),
                player_decision: "choose whether to restitch a long connector".to_string(),
                evidence_hold: "port surge demand and flood closure evidence missing".to_string(),
            },
        ];

        let rows = t2_game_publication_evidence_review_rows(&claim_rows, &hook_rows);
        let failures =
            t2_game_publication_evidence_review_gate_failures(&rows, &claim_rows, &hook_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.claim_blocker_delta == 0));
        assert!(rows
            .iter()
            .all(|row| row.review_decision == "publication-evidence-policy-required"));
        assert!(rows
            .iter()
            .all(|row| row.next_artifact == "data/t2-game-publication-evidence-policy.csv"));
    }

    #[test]
    fn t2_game_publication_evidence_policy_preserves_blockers() {
        let review_rows = vec![T2GamePublicationEvidenceReviewRow {
            game_review_id: "T2GAMEPUB-HOUSTONPORTSURGE".to_string(),
            claim_review_id: "OCR-GAMEPUB".to_string(),
            scenario_id: "houston-port-surge".to_string(),
            service_class: "long-connector".to_string(),
            t2_map_id: "beck-schematic-t2-only".to_string(),
            player_decision: "choose whether to restitch a long connector".to_string(),
            evidence_hold: "port surge demand and flood closure evidence missing".to_string(),
            review_decision: "publication-evidence-policy-required".to_string(),
            blocker_claims_before: "game;publication;upgrade".to_string(),
            blocker_claims_after: "game;publication;upgrade".to_string(),
            blocker_count_before: 1,
            blocker_count_after: 1,
            claim_blocker_delta: 0,
            required_evidence: "port-surge-demand-and-flood-closure-evidence".to_string(),
            next_artifact: "data/t2-game-publication-evidence-policy.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_game_publication_evidence_policy_rows(&review_rows);
        let failures = t2_game_publication_evidence_policy_gate_failures(&rows, &review_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].claim_blocker_delta, 0);
        assert_eq!(
            rows[0].evidence_policy_decision,
            "publication-evidence-policy-authored-review"
        );
        assert_eq!(
            rows[0].next_artifact,
            "data/t2-game-publication-evidence-policy-acceptance.csv"
        );
    }

    #[test]
    fn t2_game_publication_evidence_policy_acceptance_preserves_blockers() {
        let policy_rows = vec![T2GamePublicationEvidencePolicyRow {
            policy_id: "T2GAMEPOLICY-HOUSTONPORTSURGE".to_string(),
            game_review_id: "T2GAMEPUB-HOUSTONPORTSURGE".to_string(),
            scenario_id: "houston-port-surge".to_string(),
            service_class: "long-connector".to_string(),
            t2_map_id: "beck-schematic-t2-only".to_string(),
            evidence_policy_basis: "port surge demand and flood closure evidence missing"
                .to_string(),
            required_evidence: "port-surge-demand-and-flood-closure-evidence".to_string(),
            evidence_policy_decision: "publication-evidence-policy-authored-review".to_string(),
            policy_treatment:
                "require port surge demand evidence and flood closure source before scenario publication"
                    .to_string(),
            publication_treatment: "hold game publication until accepted evidence policy is replayed"
                .to_string(),
            blocker_claims_before: "game;publication;upgrade".to_string(),
            blocker_claims_after: "game;publication;upgrade".to_string(),
            blocker_count_before: 1,
            blocker_count_after: 1,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-game-publication-evidence-policy-acceptance.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_game_publication_evidence_policy_acceptance_rows(&policy_rows);
        let failures =
            t2_game_publication_evidence_policy_acceptance_gate_failures(&rows, &policy_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].claim_blocker_delta, 0);
        assert_eq!(
            rows[0].acceptance_decision,
            "publication-evidence-policy-accepted"
        );
        assert_eq!(
            rows[0].next_artifact,
            "data/t2-game-publication-evidence-blocker-relief.csv"
        );
    }

    #[test]
    fn t2_game_publication_evidence_blocker_relief_reduces_accepted_blockers() {
        let acceptance_rows = vec![T2GamePublicationEvidencePolicyAcceptanceRow {
            acceptance_id: "T2GAMEACCEPT-HOUSTONPORTSURGE".to_string(),
            policy_id: "T2GAMEPOLICY-HOUSTONPORTSURGE".to_string(),
            scenario_id: "houston-port-surge".to_string(),
            service_class: "long-connector".to_string(),
            t2_map_id: "beck-schematic-t2-only".to_string(),
            accepted_required_evidence: "port-surge-demand-and-flood-closure-evidence".to_string(),
            accepted_policy_treatment:
                "require port surge demand evidence and flood closure source before scenario publication"
                    .to_string(),
            acceptance_decision: "publication-evidence-policy-accepted".to_string(),
            publication_treatment: "hold game publication until accepted evidence policy is replayed"
                .to_string(),
            blocker_claims_before: "game;publication;upgrade".to_string(),
            blocker_claims_after: "game;publication;upgrade".to_string(),
            blocker_count_before: 1,
            blocker_count_after: 1,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-game-publication-evidence-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_game_publication_evidence_blocker_relief_rows(&acceptance_rows);
        let failures =
            t2_game_publication_evidence_blocker_relief_gate_failures(&rows, &acceptance_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].blocker_count_before, 1);
        assert_eq!(rows[0].blocker_count_after, 0);
        assert_eq!(rows[0].claim_blocker_delta, -1);
        assert_eq!(
            rows[0].next_artifact,
            "data/optimizer-constraint-ledger.csv"
        );
    }

    #[test]
    fn t1_schematic_geometry_claim_review_preserves_shared_segment_blockers() {
        let claim_rows = vec![OptimizerClaimReviewRow {
            claim_review_id: "OCR-T1SCHEMATIC".to_string(),
            backlog_id: "ORB-P1-claim-blocker-T1-BECKSCHEMATICGEOMETRYSCHEMATICGEOMETRY"
                .to_string(),
            priority_class: "P1-claim-blocker".to_string(),
            blocker_family: "beck_schematic_geometry|schematic_geometry".to_string(),
            tier: "T1".to_string(),
            blocked_claims: "map;publication".to_string(),
            subject_count: 2,
            route_count: 2,
            total_claim_blockers: 4,
            representative_routes: "I40;I95".to_string(),
            representative_subjects: "I40;I95".to_string(),
            evidence_artifacts: "data/t1-design-policy-actions.csv".to_string(),
            review_decision: "held-for-source-specific-claim-review".to_string(),
            blocker_claims_before: "map;publication".to_string(),
            blocker_claims_after: "map;publication".to_string(),
            claim_blocker_delta: 0,
            next_artifact: "data/t1-design-policy-actions.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let design_rows = vec![
            T1DesignReviewCsvRow {
                route: "I40".to_string(),
                selected: true,
                design_role: "promise-spine".to_string(),
                promise_count: 3,
                selected_stop_count: 10,
                top_city_stop_count: 2,
                selector_reason: "sla-required-budget-fit".to_string(),
                beck_action: "overlap-review".to_string(),
                beck_review_flag: "overlap-review".to_string(),
                overlap_corridors: "I-95".to_string(),
                design_status: "policy-review".to_string(),
                next_design_action: "resolve-shared-segment-map-policy".to_string(),
            },
            T1DesignReviewCsvRow {
                route: "I95".to_string(),
                selected: true,
                design_role: "promise-spine".to_string(),
                promise_count: 11,
                selected_stop_count: 11,
                top_city_stop_count: 5,
                selector_reason: "sla-required-budget-fit".to_string(),
                beck_action: "overlap-review".to_string(),
                beck_review_flag: "overlap-review".to_string(),
                overlap_corridors: "I-40".to_string(),
                design_status: "policy-review".to_string(),
                next_design_action: "resolve-shared-segment-map-policy".to_string(),
            },
        ];
        let policy_rows = vec![T1DesignPolicyActionRow {
            action: "resolve-shared-segment-map-policy".to_string(),
            applies_to_status: "policy-review".to_string(),
            definition: "Selected T1 route shares a Beck backbone segment".to_string(),
            required_policy:
                "Represent shared segment as interlined trunk service or split at transfer stops"
                    .to_string(),
            design_treatment: "Keep both routes selected while policy resolves treatment"
                .to_string(),
            gate_policy: "Policy review blocks final Beck replacement".to_string(),
            next_selector_use: "Map engine consumes overlap corridors".to_string(),
        }];

        let rows = t1_schematic_geometry_claim_review_rows(&claim_rows, &design_rows, &policy_rows);
        let failures = t1_schematic_geometry_claim_review_gate_failures(
            &rows,
            &claim_rows,
            &design_rows,
            &policy_rows,
        );

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert_eq!(
            rows.iter()
                .map(|row| row.blocker_count_after)
                .sum::<usize>(),
            4
        );
        assert!(rows.iter().all(|row| row.claim_blocker_delta == 0));
        assert!(rows
            .iter()
            .all(|row| row.review_decision == "shared-segment-map-policy-required"));
    }

    #[test]
    fn t2_beck_transfer_complexity_review_preserves_blockers() {
        let claim_rows = vec![OptimizerClaimReviewRow {
            claim_review_id: "OCR-T2TRANSFER".to_string(),
            backlog_id: "ORB-P1-claim-blocker-T2-BECKTRANSFERCOMPLEXITY".to_string(),
            priority_class: "P1-claim-blocker".to_string(),
            blocker_family: "beck_transfer_complexity".to_string(),
            tier: "T2".to_string(),
            blocked_claims: "map;promotion;publication".to_string(),
            subject_count: 2,
            route_count: 2,
            total_claim_blockers: 2,
            representative_routes: "I65;US30".to_string(),
            representative_subjects: "I65;US30".to_string(),
            evidence_artifacts: "data/beck-t2-diagnostics.csv".to_string(),
            review_decision: "held-for-source-specific-claim-review".to_string(),
            blocker_claims_before: "map;promotion;publication".to_string(),
            blocker_claims_after: "map;promotion;publication".to_string(),
            claim_blocker_delta: 0,
            next_artifact: "data/beck-t2-diagnostics.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let diagnostics = vec![
            route_map::BeckT2DiagnosticRow {
                corridor: "I-65",
                trunk: "I-10",
                start_trunk: "I-10",
                end_trunk: "I-75",
                color_mode: "split-parent",
                service_class: "transfer-spine",
                split_anchor: "NASHVILLE",
                split_anchor_offset_pct: 15.0,
                unstopped_t1_contact_count: 0,
                unstopped_t1_contacts: String::new(),
                close_parallel_count: 0,
                close_parallel_corridors: String::new(),
                duplicate_service_count: 0,
                duplicate_service_corridors: String::new(),
                unique_duplicate_stop_count: 5,
                service_action: "keep",
                qualification_basis: "distinct-parent-service",
                service_label: "Birmingham",
                stop_count: 5,
                drawn_stop_count: 5,
                transfer_stop_count: 5,
                schematic_length_px: 624.0,
                min_x: 0.0,
                min_y: 0.0,
                max_x: 1.0,
                max_y: 1.0,
                label_density_per_100px: 0.80,
                review_flag: "transfer-complexity-review",
            },
            route_map::BeckT2DiagnosticRow {
                corridor: "US30",
                trunk: "I-70",
                start_trunk: "I-70",
                end_trunk: "I-80",
                color_mode: "split-parent",
                service_class: "transfer-spine",
                split_anchor: "DSM",
                split_anchor_offset_pct: 6.0,
                unstopped_t1_contact_count: 0,
                unstopped_t1_contacts: String::new(),
                close_parallel_count: 0,
                close_parallel_corridors: String::new(),
                duplicate_service_count: 0,
                duplicate_service_corridors: String::new(),
                unique_duplicate_stop_count: 9,
                service_action: "keep",
                qualification_basis: "distinct-parent-service",
                service_label: "Lincoln Highway",
                stop_count: 9,
                drawn_stop_count: 9,
                transfer_stop_count: 5,
                schematic_length_px: 1764.0,
                min_x: 0.0,
                min_y: 0.0,
                max_x: 1.0,
                max_y: 1.0,
                label_density_per_100px: 0.51,
                review_flag: "transfer-complexity-review",
            },
        ];

        let rows = t2_beck_transfer_complexity_review_rows(&claim_rows, &diagnostics);
        let failures =
            t2_beck_transfer_complexity_review_gate_failures(&rows, &claim_rows, &diagnostics);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.claim_blocker_delta == 0));
        assert!(rows
            .iter()
            .all(|row| row.review_decision == "transfer-complexity-policy-required"));
    }

    #[test]
    fn t2_beck_label_density_review_preserves_blockers() {
        let claim_rows = vec![OptimizerClaimReviewRow {
            claim_review_id: "OCR-T2LABEL".to_string(),
            backlog_id: "ORB-P1-claim-blocker-T2-BECKLABELDENSITY".to_string(),
            priority_class: "P1-claim-blocker".to_string(),
            blocker_family: "beck_label_density".to_string(),
            tier: "T2".to_string(),
            blocked_claims: "map;promotion;publication".to_string(),
            subject_count: 2,
            route_count: 2,
            total_claim_blockers: 2,
            representative_routes: "I25;I405".to_string(),
            representative_subjects: "I25;I405".to_string(),
            evidence_artifacts: "data/beck-t2-diagnostics.csv".to_string(),
            review_decision: "held-for-source-specific-claim-review".to_string(),
            blocker_claims_before: "map;promotion;publication".to_string(),
            blocker_claims_after: "map;promotion;publication".to_string(),
            claim_blocker_delta: 0,
            next_artifact: "data/beck-t2-diagnostics.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let diagnostics = vec![
            route_map::BeckT2DiagnosticRow {
                corridor: "I-25",
                trunk: "I-90",
                start_trunk: "I-90",
                end_trunk: "I-40",
                color_mode: "split-parent",
                service_class: "transfer-spine",
                split_anchor: "DEN",
                split_anchor_offset_pct: 1.0,
                unstopped_t1_contact_count: 0,
                unstopped_t1_contacts: String::new(),
                close_parallel_count: 0,
                close_parallel_corridors: String::new(),
                duplicate_service_count: 0,
                duplicate_service_corridors: String::new(),
                unique_duplicate_stop_count: 0,
                service_action: "keep",
                qualification_basis: "distinct-parent-service",
                service_label: "High Plains",
                stop_count: 9,
                drawn_stop_count: 9,
                transfer_stop_count: 6,
                schematic_length_px: 762.0,
                min_x: 0.0,
                min_y: 0.0,
                max_x: 1.0,
                max_y: 1.0,
                label_density_per_100px: 1.18,
                review_flag: "dense-transfer-review",
            },
            route_map::BeckT2DiagnosticRow {
                corridor: "I-405",
                trunk: "I-5",
                start_trunk: "I-5",
                end_trunk: "I-10",
                color_mode: "split-parent",
                service_class: "transfer-spine",
                split_anchor: "LA_BASIN_W",
                split_anchor_offset_pct: 0.0,
                unstopped_t1_contact_count: 0,
                unstopped_t1_contacts: String::new(),
                close_parallel_count: 0,
                close_parallel_corridors: String::new(),
                duplicate_service_count: 0,
                duplicate_service_corridors: String::new(),
                unique_duplicate_stop_count: 5,
                service_action: "keep",
                qualification_basis: "distinct-parent-service",
                service_label: "LA Basin Relief",
                stop_count: 5,
                drawn_stop_count: 5,
                transfer_stop_count: 5,
                schematic_length_px: 386.0,
                min_x: 0.0,
                min_y: 0.0,
                max_x: 1.0,
                max_y: 1.0,
                label_density_per_100px: 1.29,
                review_flag: "dense-transfer-review",
            },
        ];

        let rows = t2_beck_label_density_review_rows(&claim_rows, &diagnostics);
        let failures = t2_beck_label_density_review_gate_failures(&rows, &claim_rows, &diagnostics);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.claim_blocker_delta == 0));
        assert!(rows
            .iter()
            .all(|row| row.review_decision == "label-density-policy-required"));
    }

    #[test]
    fn t2_beck_long_connector_review_preserves_blockers() {
        let claim_rows = vec![OptimizerClaimReviewRow {
            claim_review_id: "OCR-T2LONG".to_string(),
            backlog_id: "ORB-P1-claim-blocker-T2-BECKLONGCONNECTOR".to_string(),
            priority_class: "P1-claim-blocker".to_string(),
            blocker_family: "beck_long_connector".to_string(),
            tier: "T2".to_string(),
            blocked_claims: "map;promotion;publication".to_string(),
            subject_count: 2,
            route_count: 2,
            total_claim_blockers: 2,
            representative_routes: "I44;US83".to_string(),
            representative_subjects: "I44;US83".to_string(),
            evidence_artifacts: "data/beck-t2-diagnostics.csv".to_string(),
            review_decision: "held-for-source-specific-claim-review".to_string(),
            blocker_claims_before: "map;promotion;publication".to_string(),
            blocker_claims_after: "map;promotion;publication".to_string(),
            claim_blocker_delta: 0,
            next_artifact: "data/beck-t2-diagnostics.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let diagnostics = vec![
            route_map::BeckT2DiagnosticRow {
                corridor: "I-44",
                trunk: "I-40",
                start_trunk: "I-40",
                end_trunk: "I-90",
                color_mode: "split-parent",
                service_class: "long-connector",
                split_anchor: "STL",
                split_anchor_offset_pct: 6.0,
                unstopped_t1_contact_count: 0,
                unstopped_t1_contacts: String::new(),
                close_parallel_count: 0,
                close_parallel_corridors: String::new(),
                duplicate_service_count: 0,
                duplicate_service_corridors: String::new(),
                unique_duplicate_stop_count: 5,
                service_action: "keep",
                qualification_basis: "distinct-parent-service",
                service_label: "St. Louis Link",
                stop_count: 5,
                drawn_stop_count: 5,
                transfer_stop_count: 4,
                schematic_length_px: 945.0,
                min_x: 0.0,
                min_y: 0.0,
                max_x: 1.0,
                max_y: 1.0,
                label_density_per_100px: 0.53,
                review_flag: "long-connector-review",
            },
            route_map::BeckT2DiagnosticRow {
                corridor: "US83",
                trunk: "I-90",
                start_trunk: "I-90",
                end_trunk: "I-10",
                color_mode: "split-parent",
                service_class: "long-connector",
                split_anchor: "RAPID_CITY",
                split_anchor_offset_pct: 21.0,
                unstopped_t1_contact_count: 0,
                unstopped_t1_contacts: String::new(),
                close_parallel_count: 0,
                close_parallel_corridors: String::new(),
                duplicate_service_count: 0,
                duplicate_service_corridors: String::new(),
                unique_duplicate_stop_count: 5,
                service_action: "keep",
                qualification_basis: "distinct-parent-service",
                service_label: "Dakota Spine",
                stop_count: 5,
                drawn_stop_count: 5,
                transfer_stop_count: 4,
                schematic_length_px: 1446.0,
                min_x: 0.0,
                min_y: 0.0,
                max_x: 1.0,
                max_y: 1.0,
                label_density_per_100px: 0.35,
                review_flag: "long-connector-review",
            },
        ];

        let rows = t2_beck_long_connector_review_rows(&claim_rows, &diagnostics);
        let failures =
            t2_beck_long_connector_review_gate_failures(&rows, &claim_rows, &diagnostics);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.claim_blocker_delta == 0));
        assert!(rows
            .iter()
            .all(|row| row.next_artifact == "data/t2-beck-long-connector-policy.csv"));
    }

    #[test]
    fn t2_beck_long_connector_policy_preserves_blockers() {
        let review_rows = vec![
            T2BeckLongConnectorReviewRow {
                connector_review_id: "T2BECKLONG-US83".to_string(),
                claim_review_id: "OCR-T2LONG".to_string(),
                route: "US83".to_string(),
                trunk: "I90".to_string(),
                start_trunk: "I90".to_string(),
                end_trunk: "I10".to_string(),
                service_class: "long-connector".to_string(),
                service_label: "Dakota Spine".to_string(),
                stop_count: 5,
                transfer_stop_count: 4,
                schematic_length_px: 1446.0,
                split_anchor: "RAPID_CITY".to_string(),
                split_anchor_offset_pct: 21.0,
                review_flag: "long-connector-review".to_string(),
                connector_basis: "schematic_length_px=1446;stops=5;split_anchor_offset_pct=21"
                    .to_string(),
                review_decision: "long-connector-policy-required".to_string(),
                blocker_claims_before: "map;promotion;publication".to_string(),
                blocker_claims_after: "map;promotion;publication".to_string(),
                blocker_count_before: 1,
                blocker_count_after: 1,
                claim_blocker_delta: 0,
                next_artifact: "data/t2-beck-long-connector-policy.csv".to_string(),
                validation_status: "review".to_string(),
            },
            T2BeckLongConnectorReviewRow {
                connector_review_id: "T2BECKLONG-US90".to_string(),
                claim_review_id: "OCR-T2LONG".to_string(),
                route: "US90".to_string(),
                trunk: "I10".to_string(),
                start_trunk: "I10".to_string(),
                end_trunk: "I95".to_string(),
                service_class: "long-connector".to_string(),
                service_label: "Gulf Local".to_string(),
                stop_count: 4,
                transfer_stop_count: 4,
                schematic_length_px: 980.0,
                split_anchor: "GULFPORT".to_string(),
                split_anchor_offset_pct: 0.0,
                review_flag: "long-connector-review".to_string(),
                connector_basis: "schematic_length_px=980;stops=4;split_anchor_offset_pct=0"
                    .to_string(),
                review_decision: "long-connector-policy-required".to_string(),
                blocker_claims_before: "map;promotion;publication".to_string(),
                blocker_claims_after: "map;promotion;publication".to_string(),
                blocker_count_before: 1,
                blocker_count_after: 1,
                claim_blocker_delta: 0,
                next_artifact: "data/t2-beck-long-connector-policy.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t2_beck_long_connector_policy_rows(&review_rows);
        let failures = t2_beck_long_connector_policy_gate_failures(&rows, &review_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.claim_blocker_delta == 0));
        assert!(rows
            .iter()
            .any(|row| row.connector_band == "severe-long-connector"));
        assert!(rows
            .iter()
            .any(|row| row.connector_band == "high-long-connector"));
        assert!(rows
            .iter()
            .all(|row| row.publication_status == "held-pending-policy-acceptance"));
    }

    #[test]
    fn t2_beck_long_connector_policy_acceptance_preserves_blockers() {
        let policy_rows = vec![T2BeckLongConnectorPolicyRow {
            policy_id: "T2LONGPOLICY-US83".to_string(),
            connector_review_id: "T2BECKLONG-US83".to_string(),
            route: "US83".to_string(),
            trunk_pair: "I90-I10".to_string(),
            service_class: "long-connector".to_string(),
            schematic_length_px: 1446.0,
            connector_band: "severe-long-connector".to_string(),
            policy_basis: "schematic_length_px=1446;stops=5;split_anchor_offset_pct=21"
                .to_string(),
            connector_policy_decision: "long-connector-policy-authored-review".to_string(),
            render_treatment:
                "preserve connector service but require trunk-interface labeling and explicit local-service beads"
                    .to_string(),
            promotion_treatment:
                "hold map promotion until accepted long-connector treatment is replayed".to_string(),
            publication_status: "held-pending-policy-acceptance".to_string(),
            blocker_claims_before: "map;promotion;publication".to_string(),
            blocker_claims_after: "map;promotion;publication".to_string(),
            blocker_count_before: 1,
            blocker_count_after: 1,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-beck-long-connector-policy-acceptance.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_beck_long_connector_policy_acceptance_rows(&policy_rows);
        let failures = t2_beck_long_connector_policy_acceptance_gate_failures(&rows, &policy_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].claim_blocker_delta, 0);
        assert_eq!(
            rows[0].acceptance_decision,
            "long-connector-policy-accepted"
        );
        assert_eq!(
            rows[0].next_artifact,
            "data/t2-beck-long-connector-blocker-relief.csv"
        );
    }

    #[test]
    fn t2_beck_long_connector_blocker_relief_reduces_accepted_blockers() {
        let acceptance_rows = vec![T2BeckLongConnectorPolicyAcceptanceRow {
            acceptance_id: "T2LONGACCEPT-US83".to_string(),
            policy_id: "T2LONGPOLICY-US83".to_string(),
            route: "US83".to_string(),
            connector_band: "severe-long-connector".to_string(),
            accepted_render_treatment:
                "preserve connector service but require trunk-interface labeling and explicit local-service beads"
                    .to_string(),
            accepted_promotion_treatment:
                "hold map promotion until accepted long-connector treatment is replayed".to_string(),
            acceptance_decision: "long-connector-policy-accepted".to_string(),
            blocker_claims_before: "map;promotion;publication".to_string(),
            blocker_claims_after: "map;promotion;publication".to_string(),
            blocker_count_before: 1,
            blocker_count_after: 1,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-beck-long-connector-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_beck_long_connector_blocker_relief_rows(&acceptance_rows);
        let failures = t2_beck_long_connector_blocker_relief_gate_failures(&rows, &acceptance_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].blocker_count_before, 1);
        assert_eq!(rows[0].blocker_count_after, 0);
        assert_eq!(rows[0].claim_blocker_delta, -1);
        assert_eq!(
            rows[0].ledger_replay_status,
            "pending-optimizer-constraint-ledger-replay"
        );
    }

    #[test]
    fn t2_beck_label_density_policy_preserves_blockers() {
        let review_rows = vec![
            T2BeckLabelDensityReviewRow {
                label_review_id: "T2BECKLABEL-I405".to_string(),
                claim_review_id: "OCR-T2LABEL".to_string(),
                route: "I405".to_string(),
                trunk: "I5".to_string(),
                start_trunk: "I5".to_string(),
                end_trunk: "I10".to_string(),
                service_class: "transfer-spine".to_string(),
                service_label: "LA Basin Relief".to_string(),
                stop_count: 5,
                transfer_stop_count: 5,
                label_density_per_100px: 1.29,
                review_flag: "dense-transfer-review".to_string(),
                density_basis: "label_density_per_100px=1.29;stops=5;transfers=5".to_string(),
                review_decision: "label-density-policy-required".to_string(),
                blocker_claims_before: "map;promotion;publication".to_string(),
                blocker_claims_after: "map;promotion;publication".to_string(),
                blocker_count_before: 1,
                blocker_count_after: 1,
                claim_blocker_delta: 0,
                next_artifact: "data/t2-beck-label-density-policy.csv".to_string(),
                validation_status: "review".to_string(),
            },
            T2BeckLabelDensityReviewRow {
                label_review_id: "T2BECKLABEL-I49".to_string(),
                claim_review_id: "OCR-T2LABEL".to_string(),
                route: "I49".to_string(),
                trunk: "I20".to_string(),
                start_trunk: "I20".to_string(),
                end_trunk: "I35".to_string(),
                service_class: "transfer-spine".to_string(),
                service_label: "Ozarks".to_string(),
                stop_count: 5,
                transfer_stop_count: 5,
                label_density_per_100px: 1.01,
                review_flag: "dense-transfer-review".to_string(),
                density_basis: "label_density_per_100px=1.01;stops=5;transfers=5".to_string(),
                review_decision: "label-density-policy-required".to_string(),
                blocker_claims_before: "map;promotion;publication".to_string(),
                blocker_claims_after: "map;promotion;publication".to_string(),
                blocker_count_before: 1,
                blocker_count_after: 1,
                claim_blocker_delta: 0,
                next_artifact: "data/t2-beck-label-density-policy.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t2_beck_label_density_policy_rows(&review_rows);
        let failures = t2_beck_label_density_policy_gate_failures(&rows, &review_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.claim_blocker_delta == 0));
        assert!(rows
            .iter()
            .any(|row| row.density_band == "severe-label-density"));
        assert!(rows
            .iter()
            .any(|row| row.density_band == "moderate-label-density"));
        assert!(rows
            .iter()
            .all(|row| row.publication_status == "held-pending-policy-acceptance"));
    }

    #[test]
    fn t2_beck_transfer_complexity_policy_preserves_blockers() {
        let review_rows = vec![
            T2BeckTransferComplexityReviewRow {
                transfer_review_id: "T2BECKTRANSFER-I65".to_string(),
                claim_review_id: "OCR-T2TRANSFER".to_string(),
                route: "I65".to_string(),
                trunk: "I10".to_string(),
                start_trunk: "I10".to_string(),
                end_trunk: "I75".to_string(),
                service_class: "transfer-spine".to_string(),
                service_label: "Birmingham".to_string(),
                stop_count: 5,
                transfer_stop_count: 5,
                unique_duplicate_stop_count: 5,
                label_density_per_100px: 0.80,
                review_flag: "transfer-complexity-review".to_string(),
                complexity_basis: "transfers=5;stops=5;service_class=transfer-spine".to_string(),
                review_decision: "transfer-complexity-policy-required".to_string(),
                blocker_claims_before: "map;promotion;publication".to_string(),
                blocker_claims_after: "map;promotion;publication".to_string(),
                blocker_count_before: 1,
                blocker_count_after: 1,
                claim_blocker_delta: 0,
                next_artifact: "data/t2-beck-transfer-complexity-policy.csv".to_string(),
                validation_status: "review".to_string(),
            },
            T2BeckTransferComplexityReviewRow {
                transfer_review_id: "T2BECKTRANSFER-US80".to_string(),
                claim_review_id: "OCR-T2TRANSFER".to_string(),
                route: "US80".to_string(),
                trunk: "I35".to_string(),
                start_trunk: "I35".to_string(),
                end_trunk: "I20".to_string(),
                service_class: "transfer-spine".to_string(),
                service_label: "Old South".to_string(),
                stop_count: 8,
                transfer_stop_count: 7,
                unique_duplicate_stop_count: 8,
                label_density_per_100px: 0.69,
                review_flag: "transfer-complexity-review".to_string(),
                complexity_basis: "transfers=7;stops=8;service_class=transfer-spine".to_string(),
                review_decision: "transfer-complexity-policy-required".to_string(),
                blocker_claims_before: "map;promotion;publication".to_string(),
                blocker_claims_after: "map;promotion;publication".to_string(),
                blocker_count_before: 1,
                blocker_count_after: 1,
                claim_blocker_delta: 0,
                next_artifact: "data/t2-beck-transfer-complexity-policy.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t2_beck_transfer_complexity_policy_rows(&review_rows);
        let failures = t2_beck_transfer_complexity_policy_gate_failures(&rows, &review_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert_eq!(
            rows.iter()
                .map(|row| row.blocker_count_after)
                .sum::<usize>(),
            2
        );
        assert!(rows.iter().all(|row| row.claim_blocker_delta == 0));
        assert!(rows
            .iter()
            .any(|row| row.complexity_band == "severe-transfer-complexity"));
        assert!(rows
            .iter()
            .all(|row| row.publication_status == "held-pending-policy-acceptance"));
    }

    #[test]
    fn t2_beck_label_density_policy_acceptance_preserves_blockers() {
        let policy_rows = vec![T2BeckLabelDensityPolicyRow {
            policy_id: "T2LABELPOLICY-I405".to_string(),
            label_review_id: "T2BECKLABEL-I405".to_string(),
            route: "I405".to_string(),
            trunk_pair: "I5-I10".to_string(),
            service_class: "transfer-spine".to_string(),
            label_density_per_100px: 1.29,
            density_band: "severe-label-density".to_string(),
            policy_basis: "label_density_per_100px=1.29;stops=5;transfers=5".to_string(),
            label_policy_decision: "label-density-policy-authored-review".to_string(),
            render_treatment:
                "compress labels to trunk interfaces and preserve intermediate stops as unlabeled service beads"
                    .to_string(),
            promotion_treatment:
                "hold map promotion until accepted label-density simplification is replayed"
                    .to_string(),
            publication_status: "held-pending-policy-acceptance".to_string(),
            blocker_claims_before: "map;promotion;publication".to_string(),
            blocker_claims_after: "map;promotion;publication".to_string(),
            blocker_count_before: 1,
            blocker_count_after: 1,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-beck-label-density-policy-acceptance.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_beck_label_density_policy_acceptance_rows(&policy_rows);
        let failures = t2_beck_label_density_policy_acceptance_gate_failures(&rows, &policy_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].claim_blocker_delta, 0);
        assert_eq!(rows[0].acceptance_decision, "label-density-policy-accepted");
        assert_eq!(
            rows[0].next_artifact,
            "data/t2-beck-label-density-blocker-relief.csv"
        );
    }

    #[test]
    fn t2_beck_label_density_blocker_relief_reduces_accepted_blockers() {
        let acceptance_rows = vec![T2BeckLabelDensityPolicyAcceptanceRow {
            acceptance_id: "T2LABELACCEPT-I405".to_string(),
            policy_id: "T2LABELPOLICY-I405".to_string(),
            route: "I405".to_string(),
            density_band: "severe-label-density".to_string(),
            accepted_render_treatment:
                "compress labels to trunk interfaces and preserve intermediate stops as unlabeled service beads"
                    .to_string(),
            accepted_promotion_treatment:
                "hold map promotion until accepted label-density simplification is replayed"
                    .to_string(),
            acceptance_decision: "label-density-policy-accepted".to_string(),
            blocker_claims_before: "map;promotion;publication".to_string(),
            blocker_claims_after: "map;promotion;publication".to_string(),
            blocker_count_before: 1,
            blocker_count_after: 1,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-beck-label-density-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_beck_label_density_blocker_relief_rows(&acceptance_rows);
        let failures = t2_beck_label_density_blocker_relief_gate_failures(&rows, &acceptance_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].blocker_count_before, 1);
        assert_eq!(rows[0].blocker_count_after, 0);
        assert_eq!(rows[0].claim_blocker_delta, -1);
        assert_eq!(
            rows[0].ledger_replay_status,
            "pending-optimizer-constraint-ledger-replay"
        );
    }

    #[test]
    fn t2_beck_transfer_complexity_policy_acceptance_preserves_blockers() {
        let policy_rows = vec![T2BeckTransferComplexityPolicyRow {
            policy_id: "T2TRANSFERPOLICY-US80".to_string(),
            transfer_review_id: "T2BECKTRANSFER-US80".to_string(),
            route: "US80".to_string(),
            trunk_pair: "I35-I20".to_string(),
            service_class: "transfer-spine".to_string(),
            transfer_stop_count: 7,
            stop_count: 8,
            complexity_band: "severe-transfer-complexity".to_string(),
            policy_basis: "transfers=7;stops=8;service_class=transfer-spine".to_string(),
            transfer_policy_decision: "transfer-simplification-policy-authored-review".to_string(),
            render_treatment: "compress transfer emphasis to trunk interfaces".to_string(),
            promotion_treatment:
                "hold map promotion until accepted transfer simplification is replayed".to_string(),
            publication_status: "held-pending-policy-acceptance".to_string(),
            blocker_claims_before: "map;promotion;publication".to_string(),
            blocker_claims_after: "map;promotion;publication".to_string(),
            blocker_count_before: 1,
            blocker_count_after: 1,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-beck-transfer-complexity-policy-acceptance.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_beck_transfer_complexity_policy_acceptance_rows(&policy_rows);
        let failures =
            t2_beck_transfer_complexity_policy_acceptance_gate_failures(&rows, &policy_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].claim_blocker_delta, 0);
        assert_eq!(
            rows[0].acceptance_decision,
            "transfer-simplification-policy-accepted"
        );
        assert_eq!(
            rows[0].next_artifact,
            "data/t2-beck-transfer-complexity-blocker-relief.csv"
        );
    }

    #[test]
    fn t2_beck_transfer_complexity_blocker_relief_reduces_accepted_blockers() {
        let acceptance_rows = vec![T2BeckTransferComplexityPolicyAcceptanceRow {
            acceptance_id: "T2TRANSFERACCEPT-US80".to_string(),
            policy_id: "T2TRANSFERPOLICY-US80".to_string(),
            route: "US80".to_string(),
            complexity_band: "severe-transfer-complexity".to_string(),
            accepted_render_treatment: "compress transfer emphasis to trunk interfaces".to_string(),
            accepted_promotion_treatment:
                "hold map promotion until accepted transfer simplification is replayed".to_string(),
            acceptance_decision: "transfer-simplification-policy-accepted".to_string(),
            blocker_claims_before: "map;promotion;publication".to_string(),
            blocker_claims_after: "map;promotion;publication".to_string(),
            blocker_count_before: 1,
            blocker_count_after: 1,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-beck-transfer-complexity-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t2_beck_transfer_complexity_blocker_relief_rows(&acceptance_rows);
        let failures =
            t2_beck_transfer_complexity_blocker_relief_gate_failures(&rows, &acceptance_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].blocker_count_before, 1);
        assert_eq!(rows[0].blocker_count_after, 0);
        assert_eq!(rows[0].claim_blocker_delta, -1);
        assert_eq!(
            rows[0].ledger_replay_status,
            "pending-optimizer-constraint-ledger-replay"
        );
    }

    #[test]
    fn t3_lower_tier_feeder_gap_review_preserves_blockers() {
        let backlog_rows = vec![OptimizerResidualBlockerBacklogRow {
            backlog_id: "ORB-P1-claim-blocker-T3-LOWERTIERFEEDERGAP".to_string(),
            priority_class: "P1-claim-blocker".to_string(),
            blocker_family: "lower_tier_feeder_gap".to_string(),
            tier: "T3".to_string(),
            blocked_claims: "map;publication;upgrade".to_string(),
            subject_count: 2,
            route_count: 2,
            total_hard_blockers: 0,
            total_claim_blockers: 2,
            total_budget_debt_count: 2,
            total_constraint_debt_cost_m: 0.0,
            total_constraint_penalty_score: 2.0,
            representative_routes: "I-135;US22".to_string(),
            representative_subjects: "I-135;US22".to_string(),
            next_artifacts: "data/t3-lower-tier-feeder-gap-policy.csv".to_string(),
            backlog_decision: "held-for-feeder-policy-review".to_string(),
            next_wave: "T3 lower-tier feeder gap policy".to_string(),
            validation_status: "review".to_string(),
        }];
        let access_gap_rows = vec![
            T3T4AccessGapRow {
                gap_id: "T3T4GAP-I135".to_string(),
                source_surface: "t3-zone-map".to_string(),
                route: "I-135".to_string(),
                zone_id: "Z-MW".to_string(),
                current_score: 0.42,
                constraint_adjusted_score: 0.25,
                hard_blocker_count: 0,
                claim_blocker_count: 2,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 1.0,
                top_constraint_classes: "lower_tier_feeder_gap;terminal_access_evidence_gap"
                    .to_string(),
                constraint_ledger_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
                promise_horizon_hours: 6,
                gap_class: "below-threshold-feeder".to_string(),
                gap_reason: "route below feeder threshold".to_string(),
                required_evidence: "policy exception or feeder proof".to_string(),
                repair_action: "write feeder policy".to_string(),
                next_artifact: "data/t3-lower-tier-feeder-gap-policy.csv".to_string(),
                upward_pressure_allowed: false,
                validation_status: "review".to_string(),
            },
            T3T4AccessGapRow {
                gap_id: "T3T4GAP-US22".to_string(),
                source_surface: "t3-zone-map".to_string(),
                route: "US22".to_string(),
                zone_id: "Z-NE".to_string(),
                current_score: 0.48,
                constraint_adjusted_score: 0.27,
                hard_blocker_count: 0,
                claim_blocker_count: 2,
                constraint_debt_cost_m: 0.0,
                lifecycle_debt_cost_m: 0.0,
                constraint_penalty_score: 1.0,
                top_constraint_classes: "lower_tier_feeder_gap;terminal_access_evidence_gap"
                    .to_string(),
                constraint_ledger_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
                promise_horizon_hours: 6,
                gap_class: "below-threshold-feeder".to_string(),
                gap_reason: "route below feeder threshold".to_string(),
                required_evidence: "policy exception or feeder proof".to_string(),
                repair_action: "write feeder policy".to_string(),
                next_artifact: "data/t3-lower-tier-feeder-gap-policy.csv".to_string(),
                upward_pressure_allowed: false,
                validation_status: "review".to_string(),
            },
        ];

        let rows = t3_lower_tier_feeder_gap_review_rows(&backlog_rows, &access_gap_rows);
        let failures =
            t3_lower_tier_feeder_gap_review_gate_failures(&rows, &backlog_rows, &access_gap_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.claim_blocker_delta == 0));
        assert_eq!(
            rows.iter()
                .map(|row| row.blocker_count_after)
                .sum::<usize>(),
            2
        );
        assert!(rows
            .iter()
            .all(|row| row.review_decision == "lower-tier-feeder-policy-required"));
    }

    #[test]
    fn t3_lower_tier_feeder_gap_policy_preserves_blockers() {
        let review_rows = vec![
            T3LowerTierFeederGapReviewRow {
                feeder_review_id: "T3FEEDERREVIEW-I135".to_string(),
                backlog_id: "ORB-P1-claim-blocker-T3-LOWERTIERFEEDERGAP".to_string(),
                gap_id: "T3GAP-T3MOUNTAINWEST-I135".to_string(),
                route: "I-135".to_string(),
                zone_id: "t3-mountain-west".to_string(),
                current_score: 29.8,
                constraint_adjusted_score: 27.8,
                promise_horizon_hours: 6,
                gap_class: "below-threshold-feeder".to_string(),
                gap_reason: "candidate is below T3 threshold for a 6h feeder obligation"
                    .to_string(),
                required_evidence: "score-or-terminal-evidence-required".to_string(),
                repair_action: "prove-terminal-evidence-or-keep-t4".to_string(),
                review_decision: "lower-tier-feeder-policy-required".to_string(),
                blocker_claims_before: "map;publication;upgrade".to_string(),
                blocker_claims_after: "map;publication;upgrade".to_string(),
                blocker_count_before: 1,
                blocker_count_after: 1,
                claim_blocker_delta: 0,
                next_artifact: "data/t3-lower-tier-feeder-gap-policy.csv".to_string(),
                validation_status: "review".to_string(),
            },
            T3LowerTierFeederGapReviewRow {
                feeder_review_id: "T3FEEDERREVIEW-US281".to_string(),
                backlog_id: "ORB-P1-claim-blocker-T3-LOWERTIERFEEDERGAP".to_string(),
                gap_id: "T3GAP-T3TEXASBORDER-US281".to_string(),
                route: "US281".to_string(),
                zone_id: "t3-texas-border".to_string(),
                current_score: 25.8,
                constraint_adjusted_score: 23.8,
                promise_horizon_hours: 6,
                gap_class: "below-threshold-feeder".to_string(),
                gap_reason: "candidate is below T3 threshold for a 6h feeder obligation"
                    .to_string(),
                required_evidence: "score-or-terminal-evidence-required".to_string(),
                repair_action: "prove-terminal-evidence-or-keep-t4".to_string(),
                review_decision: "lower-tier-feeder-policy-required".to_string(),
                blocker_claims_before: "map;publication;upgrade".to_string(),
                blocker_claims_after: "map;publication;upgrade".to_string(),
                blocker_count_before: 1,
                blocker_count_after: 1,
                claim_blocker_delta: 0,
                next_artifact: "data/t3-lower-tier-feeder-gap-policy.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t3_lower_tier_feeder_gap_policy_rows(&review_rows);
        let failures = t3_lower_tier_feeder_gap_policy_gate_failures(&rows, &review_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.claim_blocker_delta == 0));
        assert_eq!(
            rows.iter()
                .map(|row| row.blocker_count_after)
                .sum::<usize>(),
            2
        );
        assert!(rows
            .iter()
            .all(|row| row.publication_status == "held-pending-policy-acceptance"));
        assert!(rows
            .iter()
            .any(|row| row.score_band == "near-threshold-feeder"));
        assert!(rows
            .iter()
            .any(|row| row.score_band == "low-threshold-feeder"));
    }

    #[test]
    fn t3_lower_tier_feeder_gap_policy_acceptance_preserves_blockers() {
        let policy_rows = vec![
            T3LowerTierFeederGapPolicyRow {
                policy_id: "T3FEEDERPOLICY-I135".to_string(),
                feeder_review_id: "T3FEEDERREVIEW-I135".to_string(),
                gap_id: "T3GAP-T3MOUNTAINWEST-I135".to_string(),
                route: "I-135".to_string(),
                zone_id: "t3-mountain-west".to_string(),
                score_band: "near-threshold-feeder".to_string(),
                policy_basis: "candidate is below T3 threshold".to_string(),
                feeder_policy_decision: "lower-tier-feeder-policy-authored-review".to_string(),
                map_treatment: "keep route below T3 feeder promotion".to_string(),
                evidence_treatment: "require score-threshold proof".to_string(),
                upgrade_treatment: "hold upgrade framing".to_string(),
                publication_status: "held-pending-policy-acceptance".to_string(),
                blocker_claims_before: "map;publication;upgrade".to_string(),
                blocker_claims_after: "map;publication;upgrade".to_string(),
                blocker_count_before: 1,
                blocker_count_after: 1,
                claim_blocker_delta: 0,
                next_artifact: "data/t3-lower-tier-feeder-gap-policy-acceptance.csv".to_string(),
                validation_status: "review".to_string(),
            },
            T3LowerTierFeederGapPolicyRow {
                policy_id: "T3FEEDERPOLICY-US281".to_string(),
                feeder_review_id: "T3FEEDERREVIEW-US281".to_string(),
                gap_id: "T3GAP-T3TEXASBORDER-US281".to_string(),
                route: "US281".to_string(),
                zone_id: "t3-texas-border".to_string(),
                score_band: "low-threshold-feeder".to_string(),
                policy_basis: "candidate is below T3 threshold".to_string(),
                feeder_policy_decision: "lower-tier-feeder-policy-authored-review".to_string(),
                map_treatment: "keep route below T3 feeder promotion".to_string(),
                evidence_treatment: "require score-threshold proof".to_string(),
                upgrade_treatment: "hold upgrade framing".to_string(),
                publication_status: "held-pending-policy-acceptance".to_string(),
                blocker_claims_before: "map;publication;upgrade".to_string(),
                blocker_claims_after: "map;publication;upgrade".to_string(),
                blocker_count_before: 1,
                blocker_count_after: 1,
                claim_blocker_delta: 0,
                next_artifact: "data/t3-lower-tier-feeder-gap-policy-acceptance.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t3_lower_tier_feeder_gap_policy_acceptance_rows(&policy_rows);
        let failures =
            t3_lower_tier_feeder_gap_policy_acceptance_gate_failures(&rows, &policy_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().all(|row| row.claim_blocker_delta == 0));
        assert_eq!(
            rows.iter()
                .map(|row| row.blocker_count_after)
                .sum::<usize>(),
            2
        );
        assert!(rows
            .iter()
            .all(|row| row.acceptance_decision == "lower-tier-feeder-policy-accepted"));
        assert!(rows
            .iter()
            .all(|row| row.next_artifact == "data/t3-lower-tier-feeder-gap-blocker-relief.csv"));
    }

    #[test]
    fn t3_lower_tier_feeder_gap_blocker_relief_reduces_accepted_blockers() {
        let acceptance_rows = vec![T3LowerTierFeederGapPolicyAcceptanceRow {
            acceptance_id: "T3FEEDERACCEPT-I135".to_string(),
            policy_id: "T3FEEDERPOLICY-I135".to_string(),
            route: "I-135".to_string(),
            zone_id: "t3-mountain-west".to_string(),
            score_band: "near-threshold-feeder".to_string(),
            accepted_map_treatment: "keep route below T3 feeder promotion".to_string(),
            accepted_evidence_treatment: "require score-threshold proof".to_string(),
            accepted_upgrade_treatment: "hold upgrade framing".to_string(),
            acceptance_decision: "lower-tier-feeder-policy-accepted".to_string(),
            blocker_claims_before: "map;publication;upgrade".to_string(),
            blocker_claims_after: "map;publication;upgrade".to_string(),
            blocker_count_before: 1,
            blocker_count_after: 1,
            claim_blocker_delta: 0,
            next_artifact: "data/t3-lower-tier-feeder-gap-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t3_lower_tier_feeder_gap_blocker_relief_rows(&acceptance_rows);
        let failures =
            t3_lower_tier_feeder_gap_blocker_relief_gate_failures(&rows, &acceptance_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].blocker_count_before, 1);
        assert_eq!(rows[0].blocker_count_after, 0);
        assert_eq!(rows[0].claim_blocker_delta, -1);
        assert_eq!(
            rows[0].ledger_replay_status,
            "pending-optimizer-constraint-ledger-replay"
        );
    }

    #[test]
    fn t1_shared_segment_map_policy_pairs_routes_without_relief() {
        let schematic_rows = vec![
            T1SchematicGeometryClaimReviewRow {
                schematic_review_id: "T1SCHEMATIC-I40".to_string(),
                claim_review_id: "OCR-T1SCHEMATIC".to_string(),
                route: "I40".to_string(),
                design_role: "promise-spine".to_string(),
                design_status: "policy-review".to_string(),
                beck_review_flag: "overlap-review".to_string(),
                overlap_corridors: "I-95".to_string(),
                policy_action: "resolve-shared-segment-map-policy".to_string(),
                required_policy: "Shared segment must be represented as interlined trunk service or split at selected transfer stops without deleting either real route".to_string(),
                design_treatment: "Keep both routes selected while map policy resolves shared schematic treatment".to_string(),
                gate_policy: "Policy review does not block release but must remain visible in design review".to_string(),
                blocker_claims_before: "map;publication".to_string(),
                blocker_claims_after: "map;publication".to_string(),
                blocker_count_before: 2,
                blocker_count_after: 2,
                claim_blocker_delta: 0,
                review_decision: "shared-segment-map-policy-required".to_string(),
                next_artifact: "data/t1-shared-segment-map-policy.csv".to_string(),
                validation_status: "review".to_string(),
            },
            T1SchematicGeometryClaimReviewRow {
                schematic_review_id: "T1SCHEMATIC-I95".to_string(),
                claim_review_id: "OCR-T1SCHEMATIC".to_string(),
                route: "I95".to_string(),
                design_role: "promise-spine".to_string(),
                design_status: "policy-review".to_string(),
                beck_review_flag: "overlap-review".to_string(),
                overlap_corridors: "I-40".to_string(),
                policy_action: "resolve-shared-segment-map-policy".to_string(),
                required_policy: "Shared segment must be represented as interlined trunk service or split at selected transfer stops without deleting either real route".to_string(),
                design_treatment: "Keep both routes selected while map policy resolves shared schematic treatment".to_string(),
                gate_policy: "Policy review does not block release but must remain visible in design review".to_string(),
                blocker_claims_before: "map;publication".to_string(),
                blocker_claims_after: "map;publication".to_string(),
                blocker_count_before: 2,
                blocker_count_after: 2,
                claim_blocker_delta: 0,
                review_decision: "shared-segment-map-policy-required".to_string(),
                next_artifact: "data/t1-shared-segment-map-policy.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows = t1_shared_segment_map_policy_rows(&schematic_rows);
        let failures = t1_shared_segment_map_policy_gate_failures(&rows, &schematic_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].route_pair, "I40-I95");
        assert_eq!(rows[0].affected_routes, "I40;I95");
        assert_eq!(rows[0].blocker_count_before, 4);
        assert_eq!(rows[0].blocker_count_after, 4);
        assert_eq!(rows[0].claim_blocker_delta, 0);
        assert_eq!(rows[0].publication_status, "held-pending-policy-acceptance");
    }

    #[test]
    fn t1_shared_segment_policy_acceptance_preserves_blockers_for_replay() {
        let policy_rows = vec![T1SharedSegmentMapPolicyRow {
            policy_id: "T1SHAREDSEG-I40I95".to_string(),
            route_pair: "I40-I95".to_string(),
            primary_route: "I40".to_string(),
            overlap_route: "I95".to_string(),
            affected_routes: "I40;I95".to_string(),
            source_review_ids: "T1SCHEMATIC-I40;T1SCHEMATIC-I95".to_string(),
            policy_basis: "Shared segment must be represented as interlined trunk service"
                .to_string(),
            map_policy_decision: "shared-segment-policy-authored-review".to_string(),
            render_treatment:
                "represent as interlined trunk service or split at selected transfer stops"
                    .to_string(),
            selector_treatment: "keep both selected promise-spine routes pending acceptance"
                .to_string(),
            publication_status: "held-pending-policy-acceptance".to_string(),
            blocker_claims_before: "map;publication".to_string(),
            blocker_claims_after: "map;publication".to_string(),
            blocker_count_before: 4,
            blocker_count_after: 4,
            claim_blocker_delta: 0,
            next_artifact: "data/t1-shared-segment-policy-acceptance.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t1_shared_segment_policy_acceptance_rows(&policy_rows);
        let failures = t1_shared_segment_policy_acceptance_gate_failures(&rows, &policy_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(
            rows[0].acceptance_status,
            "accepted-policy-ready-for-relief-replay"
        );
        assert_eq!(
            rows[0].publication_status_after,
            "held-pending-blocker-relief-replay"
        );
        assert_eq!(rows[0].blocker_count_before, 4);
        assert_eq!(rows[0].blocker_count_after, 4);
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn t1_schematic_geometry_blocker_relief_reduces_accepted_policy_blockers() {
        let acceptance_rows = vec![T1SharedSegmentPolicyAcceptanceRow {
            acceptance_id: "T1SHAREDACCEPT-T1SHAREDSEGI40I95".to_string(),
            policy_id: "T1SHAREDSEG-I40I95".to_string(),
            route_pair: "I40-I95".to_string(),
            affected_routes: "I40;I95".to_string(),
            map_policy_decision: "shared-segment-policy-authored-review".to_string(),
            accepted_render_treatment:
                "represent as interlined trunk service or split at selected transfer stops"
                    .to_string(),
            acceptance_status: "accepted-policy-ready-for-relief-replay".to_string(),
            acceptance_basis:
                "policy uses allowed interlined trunk or selected-transfer split treatment"
                    .to_string(),
            publication_status_before: "held-pending-policy-acceptance".to_string(),
            publication_status_after: "held-pending-blocker-relief-replay".to_string(),
            blocker_claims_before: "map;publication".to_string(),
            blocker_claims_after: "map;publication".to_string(),
            blocker_count_before: 4,
            blocker_count_after: 4,
            claim_blocker_delta: 0,
            next_artifact: "data/t1-schematic-geometry-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = t1_schematic_geometry_blocker_relief_rows(&acceptance_rows);
        let failures = t1_schematic_geometry_blocker_relief_gate_failures(&rows, &acceptance_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].blocker_count_before, 4);
        assert_eq!(rows[0].blocker_count_after, 0);
        assert_eq!(rows[0].claim_blocker_delta, -4);
        assert_eq!(
            rows[0].ledger_replay_status,
            "pending-optimizer-constraint-ledger-replay"
        );
    }

    #[test]
    fn tier_pavement_acquisition_plan_groups_source_gaps_by_state() {
        let gap_rows = vec![
            TierPavementSourceGapRow {
                tier: "T2".to_string(),
                route: "US30".to_string(),
                region_id: "component-1".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.US30".to_string(),
                stitch_group_id: "US.HWYSTITCH.US30".to_string(),
                member_count: 100,
                blocker_count: 90,
                blocker_statuses: "pavement-source-needed".to_string(),
                affected_states: "IA;NE;WY".to_string(),
                affected_edge_ids: "1;2;3".to_string(),
                source_contract: "HPMS IRI plus state pavement feeds".to_string(),
                source_action: "price pavement evidence debt".to_string(),
                next_artifact: "data/standards-l1-inventory.csv".to_string(),
                optimizer_effect: "bundle remains service-addressable".to_string(),
                validation_status: "review".to_string(),
            },
            TierPavementSourceGapRow {
                tier: "T2".to_string(),
                route: "I29".to_string(),
                region_id: "component-1".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.I29".to_string(),
                stitch_group_id: "US.HWYSTITCH.I29".to_string(),
                member_count: 50,
                blocker_count: 30,
                blocker_statuses: "pavement-source-needed".to_string(),
                affected_states: "IA;NE".to_string(),
                affected_edge_ids: "4;5".to_string(),
                source_contract: "HPMS IRI plus state pavement feeds".to_string(),
                source_action: "price pavement evidence debt".to_string(),
                next_artifact: "data/standards-l1-inventory.csv".to_string(),
                optimizer_effect: "bundle remains service-addressable".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows: Vec<TierPavementAcquisitionPlanRow> =
            tier_pavement_acquisition_plan_rows(&gap_rows);
        let failures = tier_pavement_acquisition_plan_gate_failures(&rows, &gap_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 3);
        let ia = rows.iter().find(|row| row.state == "IA").unwrap();
        assert_eq!(ia.route_count, 2);
        assert_eq!(ia.bundle_count, 2);
        assert_eq!(ia.blocked_member_count, 45);
        assert_eq!(ia.source_priority, "B");
    }

    #[test]
    fn tier_pavement_acquisition_docket_emits_runnable_state_tasks() {
        let plan_rows = vec![TierPavementAcquisitionPlanRow {
            state: "TX".to_string(),
            tier: "T2".to_string(),
            source_family: "HPMS IRI plus state pavement condition feed".to_string(),
            route_count: 2,
            affected_routes: "US80;US90".to_string(),
            bundle_count: 2,
            affected_bundles: "US.HWYBUNDLE.US80;US.HWYBUNDLE.US90".to_string(),
            blocked_member_count: 62,
            source_priority: "A".to_string(),
            acquisition_action: "refresh HPMS/state pavement feed".to_string(),
            required_fields: "route id; IRI; observation year".to_string(),
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            optimizer_effect: "populate member pavement evidence".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows: Vec<TierPavementAcquisitionDocketRow> =
            tier_pavement_acquisition_docket_rows(&plan_rows);
        let failures = tier_pavement_acquisition_docket_gate_failures(&rows, &plan_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].task_id, "PAVEMENT-A-TX");
        assert_eq!(rows[0].fetch_command, "route fetch-hpms --states TX");
        assert_eq!(rows[0].rebuild_command, "route build --all-roads");
        assert!(rows[0]
            .verify_command
            .contains("route tier-pavement-docket --gate"));
    }

    #[test]
    fn tier_pavement_source_access_preserves_blockers_for_priority_fetches() {
        let docket_rows = vec![
            TierPavementAcquisitionDocketRow {
                task_id: "PAVEMENT-A-TX".to_string(),
                state: "TX".to_string(),
                source_priority: "A".to_string(),
                affected_routes: "US80;US287".to_string(),
                affected_bundles: "US.HWYBUNDLE.US80;US.HWYBUNDLE.US287".to_string(),
                blocked_member_count: 49,
                fetch_command: "route fetch-hpms --states TX".to_string(),
                rebuild_command: "route build --all-roads".to_string(),
                verify_command:
                    "route tier-pavement-docket --gate && route tier-pavement-source-gaps --gate"
                        .to_string(),
                source_contract: "route id; IRI; observation year".to_string(),
                next_artifact: "data/tier-pavement-docket.csv".to_string(),
                validation_status: "review".to_string(),
            },
            TierPavementAcquisitionDocketRow {
                task_id: "PAVEMENT-B-CO".to_string(),
                state: "CO".to_string(),
                source_priority: "B".to_string(),
                affected_routes: "US6".to_string(),
                affected_bundles: "US.HWYBUNDLE.US6".to_string(),
                blocked_member_count: 12,
                fetch_command: "route fetch-hpms --states CO".to_string(),
                rebuild_command: "route build --all-roads".to_string(),
                verify_command:
                    "route tier-pavement-docket --gate && route tier-pavement-source-gaps --gate"
                        .to_string(),
                source_contract: "route id; IRI; observation year".to_string(),
                next_artifact: "data/tier-pavement-docket.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];

        let rows: Vec<TierPavementSourceAccessRow> =
            tier_pavement_source_access_rows(&docket_rows, "A");
        let failures = tier_pavement_source_access_gate_failures(&rows, &docket_rows, "A");

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].state, "TX");
        assert_eq!(rows[0].mutation_mode, "scoped-cache-merge");
        assert_eq!(rows[0].claim_blocker_delta, 0);
        assert_eq!(rows[0].blocker_claims_before, rows[0].blocker_claims_after);
    }

    #[test]
    fn tier_pavement_source_fetch_attempt_records_empty_cache_without_relief() {
        let source_access_rows = vec![TierPavementSourceAccessRow {
            access_policy_id: "PAVEMENTACCESS-PAVEMENTANM".to_string(),
            task_id: "PAVEMENT-A-NM".to_string(),
            state: "NM".to_string(),
            source_priority: "A".to_string(),
            source_access_mode: "hpms-scoped-fetch".to_string(),
            mutation_mode: "scoped-cache-merge".to_string(),
            cache_targets: "data/cache/hpms_2018.csv;data/cache/hpms_nonexistent_test_state.csv"
                .to_string(),
            fetch_command: "route fetch-hpms --states NM".to_string(),
            preflight_gate: "route source-fetch-policy --gate".to_string(),
            postfetch_gate:
                "route tier-pavement-docket --gate && route tier-pavement-source-gaps --gate"
                    .to_string(),
            blocker_claims_before: "publication;sla;transit;upgrade".to_string(),
            blocker_claims_after: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows: Vec<TierPavementSourceFetchAttemptRow> =
            tier_pavement_source_fetch_attempt_rows(&source_access_rows).unwrap();
        let failures = tier_pavement_source_fetch_attempt_gate_failures(&rows, &source_access_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].cache_record_count, 0);
        assert_eq!(rows[0].fetch_result_status, "fetch-failed-or-empty-cache");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_source_fetch_review_preserves_open_gaps_and_failed_fetches() {
        let fetch_attempt_rows = vec![
            TierPavementSourceFetchAttemptRow {
                fetch_attempt_id: "PAVEMENTFETCH-PAVEMENTATX".to_string(),
                access_policy_id: "PAVEMENTACCESS-PAVEMENTATX".to_string(),
                task_id: "PAVEMENT-A-TX".to_string(),
                state: "TX".to_string(),
                source_priority: "A".to_string(),
                fetch_command: "route fetch-hpms --states TX".to_string(),
                cache_target: "data/cache/hpms_tx.csv".to_string(),
                cache_record_count: 12,
                fetch_result_status: "cache-populated-unreviewed".to_string(),
                evidence_acceptance_status: "not-accepted".to_string(),
                blocker_claims_before: "publication;sla;transit;upgrade".to_string(),
                blocker_claims_after: "publication;sla;transit;upgrade".to_string(),
                claim_blocker_delta: 0,
                next_artifact: "data/tier-pavement-docket.csv".to_string(),
                validation_status: "review".to_string(),
            },
            TierPavementSourceFetchAttemptRow {
                fetch_attempt_id: "PAVEMENTFETCH-PAVEMENTANM".to_string(),
                access_policy_id: "PAVEMENTACCESS-PAVEMENTANM".to_string(),
                task_id: "PAVEMENT-A-NM".to_string(),
                state: "NM".to_string(),
                source_priority: "A".to_string(),
                fetch_command: "route fetch-hpms --states NM".to_string(),
                cache_target: "data/cache/hpms_nm.csv".to_string(),
                cache_record_count: 0,
                fetch_result_status: "fetch-failed-or-empty-cache".to_string(),
                evidence_acceptance_status: "not-accepted".to_string(),
                blocker_claims_before: "publication;sla;transit;upgrade".to_string(),
                blocker_claims_after: "publication;sla;transit;upgrade".to_string(),
                claim_blocker_delta: 0,
                next_artifact: "data/tier-pavement-docket.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let docket_rows = vec![
            TierPavementAcquisitionDocketRow {
                task_id: "PAVEMENT-A-TX".to_string(),
                state: "TX".to_string(),
                source_priority: "A".to_string(),
                affected_routes: "US80".to_string(),
                affected_bundles: "US.HWYBUNDLE.US80".to_string(),
                blocked_member_count: 7,
                fetch_command: "route fetch-hpms --states TX".to_string(),
                rebuild_command: "route build --all-roads".to_string(),
                verify_command:
                    "route tier-pavement-docket --gate && route tier-pavement-source-gaps --gate"
                        .to_string(),
                source_contract: "route id; IRI; observation year".to_string(),
                next_artifact: "data/tier-pavement-docket.csv".to_string(),
                validation_status: "review".to_string(),
            },
            TierPavementAcquisitionDocketRow {
                task_id: "PAVEMENT-A-NM".to_string(),
                state: "NM".to_string(),
                source_priority: "A".to_string(),
                affected_routes: "US70".to_string(),
                affected_bundles: "US.HWYBUNDLE.US70".to_string(),
                blocked_member_count: 5,
                fetch_command: "route fetch-hpms --states NM".to_string(),
                rebuild_command: "route build --all-roads".to_string(),
                verify_command:
                    "route tier-pavement-docket --gate && route tier-pavement-source-gaps --gate"
                        .to_string(),
                source_contract: "route id; IRI; observation year".to_string(),
                next_artifact: "data/tier-pavement-docket.csv".to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let source_gap_rows = vec![TierPavementSourceGapRow {
            tier: "T2".to_string(),
            route: "US80".to_string(),
            region_id: "component-0".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.US80".to_string(),
            stitch_group_id: "US.HWYSTITCH.US80".to_string(),
            member_count: 7,
            blocker_count: 7,
            blocker_statuses: "pavement-source-needed".to_string(),
            affected_states: "TX".to_string(),
            affected_edge_ids: "1;2;3".to_string(),
            source_contract: "HPMS IRI joined to T2 segment candidates".to_string(),
            source_action: "price pavement evidence debt for affected member edges".to_string(),
            next_artifact: "data/standards-l1-inventory.csv".to_string(),
            optimizer_effect:
                "bundle remains service-addressable while pavement source debt is acquired"
                    .to_string(),
            validation_status: "review".to_string(),
        }];

        let rows: Vec<TierPavementSourceFetchReviewRow> = tier_pavement_source_fetch_review_rows(
            &fetch_attempt_rows,
            &docket_rows,
            &source_gap_rows,
        );
        let failures = tier_pavement_source_fetch_review_gate_failures(
            &rows,
            &fetch_attempt_rows,
            &docket_rows,
        );

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 2);
        assert_eq!(
            rows[0].join_review_status,
            "cache-populated-source-gap-still-open"
        );
        assert_eq!(rows[0].postfetch_unresolved_member_count, 7);
        assert_eq!(rows[0].claim_blocker_delta, 0);
        assert_eq!(rows[1].join_review_status, "fetch-repair-needed");
        assert_eq!(rows[1].evidence_acceptance_status, "not-accepted");
    }

    #[test]
    fn tier_pavement_unmatched_join_review_splits_source_needed_from_repair_debt() {
        let cache_dir =
            std::env::temp_dir().join(format!("route-pavement-join-review-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&cache_dir);
        std::fs::create_dir_all(&cache_dir).expect("create cache dir");
        std::fs::write(
            cache_dir.join("hpms_tx.csv"),
            "STATE,ROUTE_ID,AADT,PCT_TRUCK,LANE_COUNT,IRI,SPEED_LIMIT\nTX,I220,10000,0.10,4,90.0,60\n",
        )
        .expect("write hpms cache");

        let fetch_review_rows = vec![TierPavementSourceFetchReviewRow {
            review_id: "PAVEMENTFETCHREVIEW-TX".to_string(),
            fetch_attempt_id: "PAVEMENTFETCH-TX".to_string(),
            task_id: "PAVEMENT-A-TX".to_string(),
            state: "TX".to_string(),
            source_priority: "A".to_string(),
            cache_record_count: 1,
            fetch_result_status: "cache-populated-unreviewed".to_string(),
            pre_review_blocked_member_count: 3,
            postfetch_unresolved_member_count: 3,
            join_review_status: "cache-populated-source-gap-still-open".to_string(),
            evidence_acceptance_status: "not-accepted".to_string(),
            blocker_claims_before: "publication;sla;transit;upgrade".to_string(),
            blocker_claims_after: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "review unmatched HPMS joins".to_string(),
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let source_gap_rows = vec![
            TierPavementSourceGapRow {
                tier: "T2".to_string(),
                route: "US80".to_string(),
                region_id: "component-0".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.US80".to_string(),
                stitch_group_id: "US.HWYSTITCH.US80".to_string(),
                member_count: 2,
                blocker_count: 2,
                blocker_statuses: "pavement-source-needed".to_string(),
                affected_states: "TX".to_string(),
                affected_edge_ids: "1;2".to_string(),
                source_contract: "HPMS IRI joined to T2 segment candidates".to_string(),
                source_action: "price pavement evidence debt".to_string(),
                next_artifact: "data/standards-l1-inventory.csv".to_string(),
                optimizer_effect: "preserve source debt".to_string(),
                validation_status: "review".to_string(),
            },
            TierPavementSourceGapRow {
                tier: "T2".to_string(),
                route: "I220".to_string(),
                region_id: "component-0".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
                stitch_group_id: "US.HWYSTITCH.I220".to_string(),
                member_count: 1,
                blocker_count: 1,
                blocker_statuses: "pavement-repair-required".to_string(),
                affected_states: "TX".to_string(),
                affected_edge_ids: "3".to_string(),
                source_contract: "HPMS IRI joined to T2 segment candidates".to_string(),
                source_action: "price pavement repair debt".to_string(),
                next_artifact: "data/tier-pavement-docket.csv".to_string(),
                optimizer_effect: "preserve repair debt".to_string(),
                validation_status: "review".to_string(),
            },
        ];
        let docket_rows = vec![
            pavement_docket_fixture("US80", "US.HWYBUNDLE.US80", 1, "pavement-source-needed"),
            pavement_docket_fixture("US80", "US.HWYBUNDLE.US80", 2, "pavement-source-needed"),
            pavement_docket_fixture("I220", "US.HWYBUNDLE.I220", 3, "pavement-repair-required"),
        ];

        let rows: Vec<TierPavementUnmatchedJoinReviewRow> =
            tier_pavement_unmatched_join_review_rows(
                &fetch_review_rows,
                &source_gap_rows,
                &docket_rows,
                &cache_dir,
            )
            .unwrap();
        let failures = tier_pavement_unmatched_join_review_gate_failures(&rows, &fetch_review_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].source_needed_member_count, 2);
        assert_eq!(rows[0].repair_required_member_count, 1);
        assert_eq!(rows[0].hpms_records_for_source_needed_routes, 0);
        assert_eq!(
            rows[0].join_review_status,
            "hpms-scope-misses-source-needed-routes"
        );
        assert_eq!(rows[0].claim_blocker_delta, 0);

        let _ = std::fs::remove_dir_all(&cache_dir);
    }

    fn pavement_docket_fixture(
        route: &str,
        segment_bundle_id: &str,
        edge_id: u64,
        pavement_status: &str,
    ) -> TierPavementDocketRow {
        TierPavementDocketRow {
            tier: "T2".to_string(),
            source_selector: "test".to_string(),
            region_id: "component-0".to_string(),
            route: route.to_string(),
            segment_bundle_id: segment_bundle_id.to_string(),
            stitch_group_id: format!("US.HWYSTITCH.{route}"),
            national_segment_id: format!("US.HWYSEG.{edge_id}"),
            edge_id,
            edge_sequence: edge_id as usize,
            state: "TX".to_string(),
            length_miles: 1.0,
            iri_m_per_km: "unknown".to_string(),
            max_iri_m_per_km: "1.50".to_string(),
            pavement_status: pavement_status.to_string(),
            repair_action: "preserve blocker".to_string(),
            freight_ride_requirement: "freight ride requirement".to_string(),
            transit_ride_requirement: "transit ride requirement".to_string(),
            source_contract: "HPMS IRI joined to T2 segment candidates".to_string(),
            qualification_effects: String::new(),
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            validation_status: "review".to_string(),
        }
    }

    #[test]
    fn tier_pavement_hpms_scope_broadening_preserves_blockers() {
        let unmatched_rows = vec![TierPavementUnmatchedJoinReviewRow {
            join_review_id: "PAVEMENTJOINREVIEW-TX".to_string(),
            state: "TX".to_string(),
            source_priority: "A".to_string(),
            cache_record_count: 43_381,
            source_gap_member_count: 2,
            source_needed_member_count: 2,
            repair_required_member_count: 0,
            source_needed_routes: "US80;US287".to_string(),
            repair_required_routes: String::new(),
            hpms_records_for_source_needed_routes: 0,
            hpms_source_route_coverage: "none".to_string(),
            join_review_status: "hpms-scope-misses-source-needed-routes".to_string(),
            evidence_acceptance_status: "not-accepted".to_string(),
            blocker_claims_before: "publication;sla;transit;upgrade".to_string(),
            blocker_claims_after: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "broaden HPMS scope".to_string(),
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows: Vec<TierPavementHpmsScopeBroadeningRow> =
            tier_pavement_hpms_scope_broadening_rows(&unmatched_rows, &[1, 2, 3]);
        let failures = tier_pavement_hpms_scope_broadening_gate_failures(&rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].broadened_functional_systems, "1,2,3");
        assert!(rows[0]
            .broadened_fetch_command
            .contains("--functional-systems 1,2,3"));
        assert_eq!(rows[0].claim_blocker_delta, 0);
        assert_eq!(rows[0].evidence_acceptance_status, "not-accepted");
    }

    #[test]
    fn tier_pavement_repair_debt_review_preserves_blockers() {
        let unmatched_rows = vec![TierPavementUnmatchedJoinReviewRow {
            join_review_id: "PAVEMENTJOINREVIEW-TX".to_string(),
            state: "TX".to_string(),
            source_priority: "A".to_string(),
            cache_record_count: 208_285,
            source_gap_member_count: 4,
            source_needed_member_count: 0,
            repair_required_member_count: 4,
            source_needed_routes: String::new(),
            repair_required_routes: "I220".to_string(),
            hpms_records_for_source_needed_routes: 0,
            hpms_source_route_coverage: "not-needed".to_string(),
            join_review_status: "repair-debt-not-source-join".to_string(),
            evidence_acceptance_status: "not-accepted".to_string(),
            blocker_claims_before: "publication;sla;transit;upgrade".to_string(),
            blocker_claims_after: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "route repair debt to funding review".to_string(),
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            validation_status: "review".to_string(),
        }];
        let debt_rows = vec![TierPavementDebtBudgetRow {
            tier: "T2".to_string(),
            route: "I220".to_string(),
            region_id: "component-0".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            stitch_group_id: "US.HWYSTITCH.I220".to_string(),
            debt_class: "repair-debt".to_string(),
            blocked_member_count: 4,
            affected_states: "TX".to_string(),
            evidence_debt_units: 0,
            repair_debt_units: 4,
            estimated_evidence_cost_m: 0.0,
            estimated_repair_cost_m: 10.0,
            total_debt_cost_m: 10.0,
            budget_basis: "planning proxy".to_string(),
            optimizer_penalty: "subtract 10.00 budget-cost units".to_string(),
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = tier_pavement_repair_debt_review_rows(&unmatched_rows, &debt_rows, &[], &[]);
        let failures =
            tier_pavement_repair_debt_review_gate_failures(&rows, &unmatched_rows, &[], &[]);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].repair_debt_status, "confirmed-repair-debt");
        assert_eq!(rows[0].claim_blocker_delta, 0);
        assert_eq!(rows[0].blocker_claims_after, rows[0].blocker_claims_before);
        assert_eq!(rows[0].evidence_acceptance_status, "not-accepted");
    }

    #[test]
    fn tier_pavement_repair_disposition_keeps_relief_ineligible() {
        let repair_rows = vec![TierPavementRepairDebtReviewRow {
            repair_review_id: "PAVEMENTREPAIRREVIEW-TX-I220".to_string(),
            state: "TX".to_string(),
            source_priority: "A".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            stitch_group_id: "US.HWYSTITCH.I220".to_string(),
            blocked_member_count: 4,
            repair_debt_units: 4,
            estimated_repair_cost_m: 10.0,
            repair_debt_status: "confirmed-repair-debt".to_string(),
            repair_decision: "hold-claims-until-repair-funded-or-design-downgraded".to_string(),
            evidence_acceptance_status: "not-accepted".to_string(),
            blocker_claims_before: "publication;sla;transit;upgrade".to_string(),
            blocker_claims_after: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "prepare repair funding".to_string(),
            next_artifact: "data/tier-pavement-repair-debt-review.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = tier_pavement_repair_disposition_rows(&repair_rows);
        let failures = tier_pavement_repair_disposition_gate_failures(&rows, &repair_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].disposition, "repair-funding-required");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
        assert_eq!(rows[0].validation_status, "held");
    }

    #[test]
    fn tier_pavement_repair_funding_package_remains_unfunded() {
        let disposition_rows = vec![TierPavementRepairDispositionRow {
            disposition_id: "PAVEMENTREPAIRDISPOSITION-TX-I220".to_string(),
            repair_review_id: "PAVEMENTREPAIRREVIEW-TX-I220".to_string(),
            state: "TX".to_string(),
            source_priority: "A".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            blocked_member_count: 4,
            estimated_repair_cost_m: 10.0,
            disposition: "repair-funding-required".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "create repair funding package".to_string(),
            next_artifact: "data/tier-pavement-repair-disposition.csv".to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_repair_funding_package_rows(&disposition_rows);
        let failures = tier_pavement_repair_funding_package_gate_failures(&rows, &disposition_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].funding_package_status, "package-required");
        assert_eq!(rows[0].funding_commitment_status, "unfunded");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_commitment_review_preserves_unaccepted_commitments() {
        let package_rows = vec![TierPavementRepairFundingPackageRow {
            funding_package_id: "PAVEMENTREPAIRFUNDING-TX-I220".to_string(),
            disposition_id: "PAVEMENTREPAIRDISPOSITION-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            blocked_member_count: 4,
            estimated_repair_cost_m: 10.0,
            funding_package_status: "package-required".to_string(),
            funding_commitment_status: "unfunded".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "attach accepted funding commitment".to_string(),
            next_artifact: "data/tier-pavement-repair-funding-package.csv".to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_commitment_review_rows(&package_rows);
        let failures = tier_pavement_funding_commitment_review_gate_failures(&rows, &package_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(
            rows[0].funding_commitment_status,
            "no-accepted-commitment-attached"
        );
        assert_eq!(rows[0].accepted_commitment_artifact, "none");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_downgrade_exclusion_decision_keeps_service_held() {
        let commitment_rows = vec![TierPavementFundingCommitmentReviewRow {
            commitment_review_id: "PAVEMENTFUNDINGCOMMITMENT-TX-I220".to_string(),
            funding_package_id: "PAVEMENTREPAIRFUNDING-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            estimated_repair_cost_m: 10.0,
            funding_commitment_status: "no-accepted-commitment-attached".to_string(),
            accepted_commitment_artifact: "none".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "choose downgrade/exclusion".to_string(),
            next_artifact: "data/tier-pavement-funding-commitment-review.csv".to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_downgrade_exclusion_decision_rows(&commitment_rows);
        let failures =
            tier_pavement_downgrade_exclusion_decision_gate_failures(&rows, &commitment_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].downgrade_decision, "no-downgrade-selected");
        assert_eq!(rows[0].exclusion_decision, "no-exclusion-selected");
        assert_eq!(rows[0].service_status, "held-at-current-tier");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_contract_requires_source() {
        let decision_rows = vec![TierPavementDowngradeExclusionDecisionRow {
            downgrade_exclusion_decision_id: "PAVEMENTDOWNGRADEEXCLUSION-TX-I220".to_string(),
            commitment_review_id: "PAVEMENTFUNDINGCOMMITMENT-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            estimated_repair_cost_m: 10.0,
            downgrade_decision: "no-downgrade-selected".to_string(),
            exclusion_decision: "no-exclusion-selected".to_string(),
            service_status: "held-at-current-tier".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "attach accepted funding evidence".to_string(),
            next_artifact: "data/tier-pavement-downgrade-exclusion-decision.csv".to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_contract_rows(&decision_rows);
        let failures = tier_pavement_funding_evidence_contract_gate_failures(&rows, &decision_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].accepted_evidence_status, "source-needed");
        assert_eq!(rows[0].minimum_commitment_amount_m, 10.0);
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_source_capture_preserves_missing_source() {
        let contract_rows = vec![TierPavementFundingEvidenceContractRow {
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            downgrade_exclusion_decision_id: "PAVEMENTDOWNGRADEEXCLUSION-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            estimated_repair_cost_m: 10.0,
            required_evidence: "accepted-programming-document".to_string(),
            minimum_commitment_amount_m: 10.0,
            accepted_evidence_status: "source-needed".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "attach accepted funding evidence".to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-contract.csv".to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_source_capture_rows(&contract_rows);
        let failures =
            tier_pavement_funding_evidence_source_capture_gate_failures(&rows, &contract_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].source_capture_status, "source-needed");
        assert_eq!(rows[0].captured_artifact, "none");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_artifact_attachment_preserves_missing_artifact() {
        let capture_rows = vec![TierPavementFundingEvidenceSourceCaptureRow {
            source_capture_id: "PAVEMENTFUNDINGSOURCE-TX-I220".to_string(),
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            minimum_commitment_amount_m: 10.0,
            source_capture_status: "source-needed".to_string(),
            captured_artifact: "none".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "attach accepted funding artifact for review before relief replay"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-source-capture.csv".to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_artifact_attachment_rows(&capture_rows);
        let failures =
            tier_pavement_funding_evidence_artifact_attachment_gate_failures(&rows, &capture_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].attachment_status, "source-needed");
        assert_eq!(rows[0].attached_artifact, "none");
        assert_eq!(rows[0].evidence_review_status, "not-reviewed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_review_docket_preserves_unattached_hold() {
        let attachment_rows = vec![TierPavementFundingEvidenceArtifactAttachmentRow {
            artifact_attachment_id: "PAVEMENTFUNDINGATTACH-TX-I220".to_string(),
            source_capture_id: "PAVEMENTFUNDINGSOURCE-TX-I220".to_string(),
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            minimum_commitment_amount_m: 10.0,
            attachment_status: "source-needed".to_string(),
            attached_artifact: "none".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims_before: "publication;sla;transit;upgrade".to_string(),
            blocked_claims_after: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            attachment_blocker:
                "accepted full-cost programming or DOT commitment artifact has not been attached"
                    .to_string(),
            next_action: "attach accepted funding artifact for review before relief replay"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-artifact-attachment.csv"
                .to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_review_docket_rows(&attachment_rows);
        let failures =
            tier_pavement_funding_evidence_review_docket_gate_failures(&rows, &attachment_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].review_decision, "held-no-attached-artifact");
        assert_eq!(rows[0].evidence_review_status, "not-reviewed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_acquisition_targets_accepted_artifact() {
        let review_rows = vec![TierPavementFundingEvidenceReviewDocketRow {
            funding_evidence_review_id: "PAVEMENTFUNDINGREVIEW-TX-I220".to_string(),
            artifact_attachment_id: "PAVEMENTFUNDINGATTACH-TX-I220".to_string(),
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            minimum_commitment_amount_m: 10.0,
            attached_artifact: "none".to_string(),
            review_decision: "held-no-attached-artifact".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            review_reason: "artifact attachment remains source-needed".to_string(),
            blocked_claims_before: "publication;sla;transit;upgrade".to_string(),
            blocked_claims_after: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "attach accepted funding artifact before evidence review".to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-review-docket.csv".to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_acquisition_rows(&review_rows);
        let failures =
            tier_pavement_funding_evidence_acquisition_gate_failures(&rows, &review_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(
            rows[0].required_artifact_type,
            "accepted-full-cost-programming-or-dot-commitment"
        );
        assert_eq!(rows[0].acquisition_status, "source-needed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_source_access_blocks_live_fetch() {
        let acquisition_rows = vec![TierPavementFundingEvidenceAcquisitionRow {
            funding_evidence_acquisition_id: "PAVEMENTFUNDINGACQUIRE-TX-I220".to_string(),
            funding_evidence_review_id: "PAVEMENTFUNDINGREVIEW-TX-I220".to_string(),
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            minimum_commitment_amount_m: 10.0,
            required_artifact_type: "accepted-full-cost-programming-or-dot-commitment".to_string(),
            acquisition_status: "source-needed".to_string(),
            candidate_source_owner: "TX DOT or accepted programming authority".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            acquisition_reason: "funding evidence review is held".to_string(),
            next_action: "acquire accepted full-cost funding artifact".to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-acquisition.csv".to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_source_access_rows(&acquisition_rows);
        let failures =
            tier_pavement_funding_evidence_source_access_gate_failures(&rows, &acquisition_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].access_mode, "manual-or-cached-source-needed");
        assert_eq!(
            rows[0].live_fetch_status,
            "unsupported-no-safe-funding-commitment-fetcher"
        );
        assert_eq!(rows[0].evidence_artifact, "source-needed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_intake_requires_metadata() {
        let access_rows = vec![TierPavementFundingEvidenceSourceAccessRow {
            source_access_id: "PAVEMENTFUNDINGACCESS-TX-I220".to_string(),
            funding_evidence_acquisition_id: "PAVEMENTFUNDINGACQUIRE-TX-I220".to_string(),
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            source_owner: "TX DOT or accepted programming authority".to_string(),
            access_mode: "manual-or-cached-source-needed".to_string(),
            live_fetch_status: "unsupported-no-safe-funding-commitment-fetcher".to_string(),
            required_source_metadata:
                "source title; source url or cached artifact; capture date; issuing agency; committed amount; covered route and state"
                    .to_string(),
            cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                .to_string(),
            source_access_blocker: "no safe live funding-commitment fetch command exists"
                .to_string(),
            evidence_artifact: "source-needed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "collect manual or cached accepted funding artifact".to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-source-access.csv".to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_intake_rows(&access_rows);
        let failures = tier_pavement_funding_evidence_intake_gate_failures(&rows, &access_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].intake_status, "artifact-required");
        assert_eq!(rows[0].evidence_artifact, "source-needed");
        assert_eq!(rows[0].evidence_review_status, "not-reviewed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_metadata_capture_preserves_source_needed() {
        let intake_rows = vec![TierPavementFundingEvidenceIntakeRow {
            funding_evidence_intake_id: "PAVEMENTFUNDINGINTAKE-TX-I220".to_string(),
            source_access_id: "PAVEMENTFUNDINGACCESS-TX-I220".to_string(),
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            required_artifact_type: "accepted-full-cost-programming-or-dot-commitment"
                .to_string(),
            required_source_metadata:
                "source title; source url or cached artifact; capture date; issuing agency; committed amount; covered route and state"
                    .to_string(),
            intake_status: "artifact-required".to_string(),
            evidence_artifact: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            intake_blocker: "accepted funding artifact metadata has not been captured".to_string(),
            blocked_claims: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "capture accepted funding artifact metadata".to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-intake.csv".to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_metadata_capture_rows(&intake_rows);
        let failures =
            tier_pavement_funding_evidence_metadata_capture_gate_failures(&rows, &intake_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].metadata_capture_status, "source-needed");
        assert_eq!(rows[0].captured_artifact, "none");
        assert_eq!(rows[0].captured_source_title, "source-needed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_accepted_artifact_attachment_preserves_hold() {
        let metadata_rows = vec![TierPavementFundingEvidenceMetadataCaptureRow {
            metadata_capture_id: "PAVEMENTFUNDINGMETADATA-TX-I220".to_string(),
            funding_evidence_intake_id: "PAVEMENTFUNDINGINTAKE-TX-I220".to_string(),
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            required_artifact_type: "accepted-full-cost-programming-or-dot-commitment".to_string(),
            metadata_capture_status: "source-needed".to_string(),
            captured_artifact: "none".to_string(),
            captured_source_title: "source-needed".to_string(),
            captured_source_url: "source-needed".to_string(),
            captured_commitment_amount_m: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "capture accepted funding artifact metadata before attachment and review"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-metadata-capture.csv".to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_accepted_artifact_attachment_rows(&metadata_rows);
        let failures = tier_pavement_funding_evidence_accepted_artifact_attachment_gate_failures(
            &rows,
            &metadata_rows,
        );

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].attachment_status, "source-needed");
        assert_eq!(rows[0].attached_artifact, "none");
        assert_eq!(rows[0].captured_source_title, "source-needed");
        assert_eq!(rows[0].evidence_review_status, "not-reviewed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_accepted_attachment_review_preserves_hold() {
        let attachment_rows = vec![TierPavementFundingEvidenceAcceptedArtifactAttachmentRow {
            accepted_artifact_attachment_id: "PAVEMENTFUNDINGACCEPTEDATTACH-TX-I220".to_string(),
            metadata_capture_id: "PAVEMENTFUNDINGMETADATA-TX-I220".to_string(),
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            required_artifact_type: "accepted-full-cost-programming-or-dot-commitment".to_string(),
            attachment_status: "source-needed".to_string(),
            attached_artifact: "none".to_string(),
            captured_source_title: "source-needed".to_string(),
            captured_source_url: "source-needed".to_string(),
            captured_commitment_amount_m: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims_before: "publication;sla;transit;upgrade".to_string(),
            blocked_claims_after: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            attachment_blocker:
                "accepted full-cost programming or DOT commitment artifact has not been attached"
                    .to_string(),
            next_action: "attach accepted funding artifact before review or relief replay"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-accepted-artifact-attachment.csv"
                .to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_accepted_attachment_review_rows(&attachment_rows);
        let failures = tier_pavement_funding_evidence_accepted_attachment_review_gate_failures(
            &rows,
            &attachment_rows,
        );

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].review_decision, "held-no-attached-artifact");
        assert_eq!(rows[0].attached_artifact, "none");
        assert_eq!(rows[0].evidence_review_status, "not-reviewed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_accepted_artifact_acquisition_preserves_hold() {
        let review_rows = vec![TierPavementFundingEvidenceAcceptedAttachmentReviewRow {
            accepted_attachment_review_id: "PAVEMENTFUNDINGACCEPTEDREVIEW-TX-I220".to_string(),
            accepted_artifact_attachment_id: "PAVEMENTFUNDINGACCEPTEDATTACH-TX-I220".to_string(),
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            required_artifact_type: "accepted-full-cost-programming-or-dot-commitment".to_string(),
            attached_artifact: "none".to_string(),
            review_decision: "held-no-attached-artifact".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            review_reason:
                "accepted artifact attachment remains source-needed; funding evidence cannot be reviewed or accepted"
                    .to_string(),
            blocked_claims_before: "publication;sla;transit;upgrade".to_string(),
            blocked_claims_after: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "attach accepted funding artifact before evidence review or relief replay"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-accepted-attachment-review.csv"
                .to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_accepted_artifact_acquisition_rows(&review_rows);
        let failures = tier_pavement_funding_evidence_accepted_artifact_acquisition_gate_failures(
            &rows,
            &review_rows,
        );

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].acquisition_status, "source-needed");
        assert_eq!(rows[0].cache_status, "not-cached");
        assert_eq!(
            rows[0].candidate_source_owner,
            "TX DOT or accepted programming authority"
        );
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_accepted_source_access_preserves_hold() {
        let acquisition_rows = vec![TierPavementFundingEvidenceAcceptedArtifactAcquisitionRow {
            accepted_artifact_acquisition_id: "PAVEMENTFUNDINGACCEPTEDACQUIRE-TX-I220".to_string(),
            accepted_attachment_review_id: "PAVEMENTFUNDINGACCEPTEDREVIEW-TX-I220".to_string(),
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            required_artifact_type: "accepted-full-cost-programming-or-dot-commitment".to_string(),
            acquisition_status: "source-needed".to_string(),
            cache_status: "not-cached".to_string(),
            candidate_source_owner: "TX DOT or accepted programming authority".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            acquisition_reason: "accepted funding artifact is not attached and cannot be reviewed"
                .to_string(),
            next_action: "acquire or cache accepted full-cost funding artifact".to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-accepted-artifact-acquisition.csv"
                .to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_accepted_source_access_rows(&acquisition_rows);
        let failures = tier_pavement_funding_evidence_accepted_source_access_gate_failures(
            &rows,
            &acquisition_rows,
        );

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].access_mode, "manual-or-cached-source-needed");
        assert_eq!(rows[0].cache_status, "not-cached");
        assert_eq!(
            rows[0].live_fetch_status,
            "unsupported-no-safe-funding-commitment-fetcher"
        );
        assert_eq!(rows[0].evidence_artifact, "source-needed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_accepted_intake_preserves_hold() {
        let access_rows = vec![TierPavementFundingEvidenceAcceptedSourceAccessRow {
            accepted_source_access_id: "PAVEMENTFUNDINGACCEPTEDACCESS-TX-I220".to_string(),
            accepted_artifact_acquisition_id: "PAVEMENTFUNDINGACCEPTEDACQUIRE-TX-I220".to_string(),
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            source_owner: "TX DOT or accepted programming authority".to_string(),
            access_mode: "manual-or-cached-source-needed".to_string(),
            cache_status: "not-cached".to_string(),
            live_fetch_status: "unsupported-no-safe-funding-commitment-fetcher".to_string(),
            required_source_metadata: "source_url;publication_date;program_amount;covered_scope"
                .to_string(),
            cache_policy_artifact: "none".to_string(),
            source_access_blocker: "accepted artifact source must be manually located or cached"
                .to_string(),
            evidence_artifact: "source-needed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "capture accepted funding artifact source metadata".to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-accepted-source-access.csv"
                .to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_accepted_intake_rows(&access_rows);
        let failures =
            tier_pavement_funding_evidence_accepted_intake_gate_failures(&rows, &access_rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].intake_status, "artifact-required");
        assert_eq!(rows[0].cache_status, "not-cached");
        assert_eq!(rows[0].evidence_artifact, "source-needed");
        assert_eq!(rows[0].evidence_review_status, "not-reviewed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_accepted_metadata_capture_preserves_hold() {
        let intake_rows = vec![TierPavementFundingEvidenceAcceptedIntakeRow {
            accepted_intake_id: "PAVEMENTFUNDINGACCEPTEDINTAKE-TX-I220".to_string(),
            accepted_source_access_id: "PAVEMENTFUNDINGACCEPTEDACCESS-TX-I220".to_string(),
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            required_artifact_type: "accepted-full-cost-programming-or-dot-commitment".to_string(),
            required_source_metadata: "source_url;publication_date;program_amount;covered_scope"
                .to_string(),
            intake_status: "artifact-required".to_string(),
            cache_status: "not-cached".to_string(),
            evidence_artifact: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            intake_blocker: "accepted funding artifact metadata has not been captured".to_string(),
            blocked_claims: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "capture accepted funding artifact metadata".to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-accepted-intake.csv".to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_accepted_metadata_capture_rows(&intake_rows);
        let failures = tier_pavement_funding_evidence_accepted_metadata_capture_gate_failures(
            &rows,
            &intake_rows,
        );

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].metadata_capture_status, "source-needed");
        assert_eq!(rows[0].captured_artifact, "none");
        assert_eq!(rows[0].captured_source_title, "source-needed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_accepted_metadata_artifact_attachment_preserves_hold() {
        let metadata_rows = vec![TierPavementFundingEvidenceAcceptedMetadataCaptureRow {
            accepted_metadata_capture_id: "PAVEMENTFUNDINGACCEPTEDMETADATA-TX-I220".to_string(),
            accepted_intake_id: "PAVEMENTFUNDINGACCEPTEDINTAKE-TX-I220".to_string(),
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            required_artifact_type: "accepted-full-cost-programming-or-dot-commitment".to_string(),
            metadata_capture_status: "source-needed".to_string(),
            captured_artifact: "none".to_string(),
            captured_source_title: "source-needed".to_string(),
            captured_source_url: "source-needed".to_string(),
            captured_commitment_amount_m: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "attach accepted funding artifact only after metadata is captured"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-accepted-metadata-capture.csv"
                .to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_accepted_metadata_artifact_attachment_rows(
            &metadata_rows,
        );
        let failures =
            tier_pavement_funding_evidence_accepted_metadata_artifact_attachment_gate_failures(
                &rows,
                &metadata_rows,
            );

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].attachment_status, "source-needed");
        assert_eq!(rows[0].attached_artifact, "none");
        assert_eq!(rows[0].captured_source_title, "source-needed");
        assert_eq!(rows[0].evidence_review_status, "not-reviewed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_accepted_metadata_attachment_review_preserves_hold() {
        let attachment_rows =
            vec![TierPavementFundingEvidenceAcceptedMetadataArtifactAttachmentRow {
                accepted_metadata_artifact_attachment_id:
                    "PAVEMENTFUNDINGACCEPTEDMETAATTACH-TX-I220".to_string(),
                accepted_metadata_capture_id: "PAVEMENTFUNDINGACCEPTEDMETADATA-TX-I220"
                    .to_string(),
                evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
                state: "TX".to_string(),
                tier: "T2".to_string(),
                route: "I220".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
                required_artifact_type: "accepted-full-cost-programming-or-dot-commitment"
                    .to_string(),
                attachment_status: "source-needed".to_string(),
                attached_artifact: "none".to_string(),
                captured_source_title: "source-needed".to_string(),
                captured_source_url: "source-needed".to_string(),
                captured_commitment_amount_m: "source-needed".to_string(),
                evidence_review_status: "not-reviewed".to_string(),
                accepted_evidence_status: "not-accepted".to_string(),
                relief_eligibility: "not-eligible-for-relief".to_string(),
                blocked_claims_before: "publication;sla;transit;upgrade".to_string(),
                blocked_claims_after: "publication;sla;transit;upgrade".to_string(),
                claim_blocker_delta: 0,
                attachment_blocker:
                    "accepted full-cost programming or DOT commitment artifact has not been attached"
                        .to_string(),
                next_action: "review accepted funding artifact only after attachment".to_string(),
                next_artifact:
                    "data/tier-pavement-funding-evidence-accepted-metadata-artifact-attachment.csv"
                        .to_string(),
                validation_status: "held".to_string(),
            }];

        let rows = tier_pavement_funding_evidence_accepted_metadata_attachment_review_rows(
            &attachment_rows,
        );
        let failures =
            tier_pavement_funding_evidence_accepted_metadata_attachment_review_gate_failures(
                &rows,
                &attachment_rows,
            );

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].review_decision, "held-no-attached-artifact");
        assert_eq!(rows[0].attached_artifact, "none");
        assert_eq!(rows[0].evidence_review_status, "not-reviewed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition_preserves_hold() {
        let review_rows = vec![TierPavementFundingEvidenceAcceptedMetadataAttachmentReviewRow {
            accepted_metadata_attachment_review_id:
                "PAVEMENTFUNDINGACCEPTEDMETAREVIEW-TX-I220".to_string(),
            accepted_metadata_artifact_attachment_id:
                "PAVEMENTFUNDINGACCEPTEDMETAATTACH-TX-I220".to_string(),
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            required_artifact_type: "accepted-full-cost-programming-or-dot-commitment".to_string(),
            attached_artifact: "none".to_string(),
            review_decision: "held-no-attached-artifact".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            review_reason:
                "accepted metadata artifact attachment remains source-needed; funding evidence cannot be reviewed or accepted"
                    .to_string(),
            blocked_claims_before: "publication;sla;transit;upgrade".to_string(),
            blocked_claims_after: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "attach accepted funding artifact before evidence review or relief replay"
                .to_string(),
            next_artifact:
                "data/tier-pavement-funding-evidence-accepted-metadata-attachment-review.csv"
                    .to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition_rows(
            &review_rows,
        );
        let failures =
            tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition_gate_failures(
                &rows,
                &review_rows,
            );

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].acquisition_status, "source-needed");
        assert_eq!(rows[0].cache_status, "not-cached");
        assert_eq!(
            rows[0].candidate_source_owner,
            "TX DOT or accepted programming authority"
        );
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_accepted_metadata_source_access_preserves_hold() {
        let acquisition_rows =
            vec![TierPavementFundingEvidenceAcceptedMetadataArtifactAcquisitionRow {
                accepted_metadata_artifact_acquisition_id:
                    "PAVEMENTFUNDINGACCEPTEDMETAACQUIRE-TX-I220".to_string(),
                accepted_metadata_attachment_review_id:
                    "PAVEMENTFUNDINGACCEPTEDMETAREVIEW-TX-I220".to_string(),
                evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
                state: "TX".to_string(),
                tier: "T2".to_string(),
                route: "I220".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
                required_artifact_type: "accepted-full-cost-programming-or-dot-commitment"
                    .to_string(),
                acquisition_status: "source-needed".to_string(),
                cache_status: "not-cached".to_string(),
                candidate_source_owner: "TX DOT or accepted programming authority".to_string(),
                accepted_evidence_status: "not-accepted".to_string(),
                relief_eligibility: "not-eligible-for-relief".to_string(),
                blocked_claims: "publication;sla;transit;upgrade".to_string(),
                claim_blocker_delta: 0,
                acquisition_reason:
                    "accepted funding artifact is not attached and cannot be reviewed".to_string(),
                next_action: "acquire or cache accepted full-cost funding artifact".to_string(),
                next_artifact:
                    "data/tier-pavement-funding-evidence-accepted-metadata-artifact-acquisition.csv"
                        .to_string(),
                validation_status: "held".to_string(),
            }];

        let rows =
            tier_pavement_funding_evidence_accepted_metadata_source_access_rows(&acquisition_rows);
        let failures = tier_pavement_funding_evidence_accepted_metadata_source_access_gate_failures(
            &rows,
            &acquisition_rows,
        );

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].access_mode, "manual-or-cached-source-needed");
        assert_eq!(rows[0].cache_status, "not-cached");
        assert_eq!(
            rows[0].live_fetch_status,
            "unsupported-no-safe-funding-commitment-fetcher"
        );
        assert_eq!(rows[0].evidence_artifact, "source-needed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_accepted_metadata_intake_preserves_hold() {
        let access_rows = vec![TierPavementFundingEvidenceAcceptedMetadataSourceAccessRow {
            accepted_metadata_source_access_id: "PAVEMENTFUNDINGACCEPTEDMETAACCESS-TX-I220"
                .to_string(),
            accepted_metadata_artifact_acquisition_id:
                "PAVEMENTFUNDINGACCEPTEDMETAACQUIRE-TX-I220".to_string(),
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            source_owner: "TX DOT or accepted programming authority".to_string(),
            access_mode: "manual-or-cached-source-needed".to_string(),
            cache_status: "not-cached".to_string(),
            live_fetch_status: "unsupported-no-safe-funding-commitment-fetcher".to_string(),
            required_source_metadata: "source_url;publication_date;program_amount;covered_scope"
                .to_string(),
            cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                .to_string(),
            source_access_blocker:
                "accepted funding artifact is source-needed and not cached; live fetch is unsupported"
                    .to_string(),
            evidence_artifact: "source-needed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "collect manual or cached accepted funding artifact before intake and review"
                .to_string(),
            next_artifact:
                "data/tier-pavement-funding-evidence-accepted-metadata-source-access.csv"
                    .to_string(),
            validation_status: "held".to_string(),
        }];

        let rows = tier_pavement_funding_evidence_accepted_metadata_intake_rows(&access_rows);
        let failures = tier_pavement_funding_evidence_accepted_metadata_intake_gate_failures(
            &rows,
            &access_rows,
        );

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].intake_status, "artifact-required");
        assert_eq!(rows[0].cache_status, "not-cached");
        assert_eq!(rows[0].evidence_artifact, "source-needed");
        assert_eq!(rows[0].evidence_review_status, "not-reviewed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_accepted_metadata_source_capture_preserves_hold() {
        let intake_rows = vec![TierPavementFundingEvidenceAcceptedMetadataIntakeRow {
            accepted_metadata_intake_id: "PAVEMENTFUNDINGACCEPTEDMETAINTAKE-TX-I220".to_string(),
            accepted_metadata_source_access_id: "PAVEMENTFUNDINGACCEPTEDMETAACCESS-TX-I220"
                .to_string(),
            evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
            state: "TX".to_string(),
            tier: "T2".to_string(),
            route: "I220".to_string(),
            segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
            required_artifact_type: "accepted-full-cost-programming-or-dot-commitment".to_string(),
            required_source_metadata: "source_url;publication_date;program_amount;covered_scope"
                .to_string(),
            intake_status: "artifact-required".to_string(),
            cache_status: "not-cached".to_string(),
            evidence_artifact: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            intake_blocker: "accepted funding artifact metadata has not been captured".to_string(),
            blocked_claims: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_action: "capture accepted funding artifact metadata".to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-accepted-metadata-intake.csv"
                .to_string(),
            validation_status: "held".to_string(),
        }];

        let rows =
            tier_pavement_funding_evidence_accepted_metadata_source_capture_rows(&intake_rows);
        let failures =
            tier_pavement_funding_evidence_accepted_metadata_source_capture_gate_failures(
                &rows,
                &intake_rows,
            );

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].source_capture_status, "source-needed");
        assert_eq!(rows[0].captured_artifact, "none");
        assert_eq!(rows[0].captured_source_title, "source-needed");
        assert_eq!(rows[0].captured_source_url, "source-needed");
        assert_eq!(rows[0].captured_commitment_amount_m, "source-needed");
        assert_eq!(rows[0].evidence_review_status, "not-reviewed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn tier_pavement_funding_evidence_accepted_metadata_source_capture_artifact_attachment_preserves_hold(
    ) {
        let capture_rows = vec![
            TierPavementFundingEvidenceAcceptedMetadataSourceCaptureRow {
                accepted_metadata_source_capture_id: "PAVEMENTFUNDINGACCEPTEDMETASOURCE-TX-I220"
                    .to_string(),
                accepted_metadata_intake_id: "PAVEMENTFUNDINGACCEPTEDMETAINTAKE-TX-I220"
                    .to_string(),
                evidence_contract_id: "PAVEMENTFUNDINGEVIDENCE-TX-I220".to_string(),
                state: "TX".to_string(),
                tier: "T2".to_string(),
                route: "I220".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.I220".to_string(),
                required_artifact_type: "accepted-full-cost-programming-or-dot-commitment"
                    .to_string(),
                required_source_metadata:
                    "source_url;publication_date;program_amount;covered_scope".to_string(),
                source_capture_status: "source-needed".to_string(),
                captured_artifact: "none".to_string(),
                captured_source_title: "source-needed".to_string(),
                captured_source_url: "source-needed".to_string(),
                captured_commitment_amount_m: "source-needed".to_string(),
                evidence_review_status: "not-reviewed".to_string(),
                accepted_evidence_status: "not-accepted".to_string(),
                relief_eligibility: "not-eligible-for-relief".to_string(),
                blocked_claims: "publication;sla;transit;upgrade".to_string(),
                claim_blocker_delta: 0,
                next_action:
                    "attach accepted funding artifact only after source metadata is captured"
                        .to_string(),
                next_artifact:
                    "data/tier-pavement-funding-evidence-accepted-metadata-source-capture.csv"
                        .to_string(),
                validation_status: "held".to_string(),
            },
        ];

        let rows =
            tier_pavement_funding_evidence_accepted_metadata_source_capture_artifact_attachment_rows(
                &capture_rows,
            );
        let failures =
            tier_pavement_funding_evidence_accepted_metadata_source_capture_artifact_attachment_gate_failures(
                &rows,
                &capture_rows,
            );

        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].attachment_status, "source-needed");
        assert_eq!(rows[0].attached_artifact, "none");
        assert_eq!(rows[0].captured_source_title, "source-needed");
        assert_eq!(rows[0].captured_source_url, "source-needed");
        assert_eq!(rows[0].captured_commitment_amount_m, "source-needed");
        assert_eq!(rows[0].evidence_review_status, "not-reviewed");
        assert_eq!(rows[0].accepted_evidence_status, "not-accepted");
        assert_eq!(rows[0].relief_eligibility, "not-eligible-for-relief");
        assert_eq!(rows[0].blocked_claims_before, rows[0].blocked_claims_after);
        assert_eq!(rows[0].claim_blocker_delta, 0);
    }

    #[test]
    fn hpms_state_fetch_merge_replaces_only_requested_states() {
        let mut states = std::collections::BTreeSet::new();
        states.insert("TX".to_string());

        let existing = vec![
            route_data::HpmsRecord {
                state: "IA".to_string(),
                route_id: "US30".to_string(),
                aadt: Some(1000),
                pct_truck: Some(0.12),
                lane_count: Some(4),
                iri: Some(70.0),
                speed_limit: Some(65),
            },
            route_data::HpmsRecord {
                state: "TX".to_string(),
                route_id: "US80".to_string(),
                aadt: Some(2000),
                pct_truck: Some(0.18),
                lane_count: Some(2),
                iri: None,
                speed_limit: Some(55),
            },
        ];
        let fetched = vec![route_data::HpmsRecord {
            state: "TX".to_string(),
            route_id: "US80".to_string(),
            aadt: Some(3000),
            pct_truck: Some(0.2),
            lane_count: Some(4),
            iri: Some(55.0),
            speed_limit: Some(70),
        }];

        let merged = merge_hpms_state_records(existing, fetched, &states);

        assert_eq!(merged.len(), 2);
        assert!(merged
            .iter()
            .any(|row| row.state == "IA" && row.route_id == "US30"));
        assert!(merged
            .iter()
            .any(|row| row.state == "TX" && row.route_id == "US80" && row.aadt == Some(3000)));
        assert!(!merged
            .iter()
            .any(|row| row.state == "TX" && row.route_id == "US80" && row.aadt == Some(2000)));
    }

    #[test]
    fn hpms_functional_system_parser_deduplicates_broadened_scope() {
        let systems = parse_hpms_functional_systems("1,2,3,3").unwrap();
        assert_eq!(systems, vec![1, 2, 3]);
        assert!(parse_hpms_functional_systems("").is_err());
        assert!(parse_hpms_functional_systems("8").is_err());
    }

    #[test]
    fn source_fetch_policy_covers_scoped_full_and_live_fetch_modes() {
        let rows: Vec<SourceFetchPolicyRow> = source_fetch_policy_rows();
        let failures = source_fetch_policy_gate_failures(&rows);

        assert!(failures.is_empty(), "{failures:?}");
        assert!(rows.iter().any(
            |row| row.fetch_family == "hpms-state-scope" && row.mutation_mode == "scoped-merge"
        ));
        assert!(rows
            .iter()
            .any(|row| row.mutation_mode == "full-replace-after-validation"));
        assert!(rows
            .iter()
            .any(|row| row.mutation_mode == "live-snapshot-preserve"));
        assert!(rows
            .iter()
            .all(|row| row.policy_doc == "docs/source-fetch-cache-policy.md"));
        for command in known_source_fetch_commands() {
            assert!(
                rows.iter()
                    .any(|row| source_fetch_policy_row_covers_command(row, command)),
                "{command} missing from source fetch policy"
            );
        }
    }

    #[test]
    fn lower_tier_pressure_witnesses_emit_upgrade_and_demotion_rows() {
        let tier_rows = vec![
            TierTableScoreRow {
                tier: "T3".to_string(),
                route: "I-57".to_string(),
                score: 49.6,
                confidence: 0.6,
                confidence_label: "Low".to_string(),
            },
            TierTableScoreRow {
                tier: "T4".to_string(),
                route: "US-11".to_string(),
                score: 29.8,
                confidence: 0.5,
                confidence_label: "Low".to_string(),
            },
        ];
        let candidate_rows = vec![TierCandidateColumnRow {
            tier: "T2".to_string(),
            route: "I220".to_string(),
            candidate_type: "route-service-column".to_string(),
            graph_kind: "dual-route-graph".to_string(),
            split_objective: "route-mile-workload".to_string(),
            node_class: "local_spur".to_string(),
            route_miles: 58.0,
            observed_t1_node_count: 0,
            observed_dual_contacts: 0,
            parent_trunks: String::new(),
            component_id: 5,
            component_route_count: 1,
            component_status: "component-bridged:21".to_string(),
            witness_type: "tier-demotion-needed".to_string(),
            repair_action: "demote-to-t3-t4".to_string(),
            repair_basis: "local-spur".to_string(),
            segment_bundle_id: String::new(),
            bundle_status: String::new(),
            bundle_action: String::new(),
            pavement_debt_cost_m: 0.0,
            pavement_debt_class: "none".to_string(),
            pavement_debt_basis: "no pavement debt row joined".to_string(),
            pavement_debt_artifact: String::new(),
            hard_blocker_count: 0,
            claim_blocker_count: 0,
            constraint_debt_cost_m: 0.0,
            lifecycle_debt_cost_m: 0.0,
            constraint_penalty_score: 0.0,
            top_constraint_classes: "none".to_string(),
            qualification_effects: String::new(),
            constraint_ledger_artifact: String::new(),
            column_decision: "demote".to_string(),
            evidence_status: "policy-action".to_string(),
            required_artifact: "data/tier-table.csv".to_string(),
            validation_status: "review".to_string(),
        }];

        let rows = lower_tier_pressure_witness_rows(
            &tier_rows,
            &candidate_rows,
            &[],
            &std::collections::HashMap::new(),
        );
        let failures = lower_tier_pressure_witness_gate_failures(&rows);
        let actions = rows
            .iter()
            .map(|row| row.witness_action.as_str())
            .collect::<std::collections::BTreeSet<_>>();

        assert!(actions.contains("demote-to-lower-tier-treatment"));
        assert!(actions.contains("evaluate-for-t2-upgrade-candidate"));
        assert!(actions.contains("evaluate-for-t3-access-candidate"));
        assert!(failures.is_empty());
    }

    #[test]
    fn lower_tier_pressure_witnesses_include_contact_resolution_demotions() {
        let tier_rows = vec![TierTableScoreRow {
            tier: "T2".to_string(),
            route: "I-110".to_string(),
            score: 65.6,
            confidence: 0.75,
            confidence_label: "Medium".to_string(),
        }];
        let resolution_rows = vec![T2ContactResolutionRow {
            route: "I110".to_string(),
            witness_type: "graph-contact-needed".to_string(),
            node_class: "missing_graph_data".to_string(),
            repair_action: "fix-graph-contact-or-demote".to_string(),
            required_artifact: "data/tier-contact-witnesses.csv".to_string(),
            exception_type: "demote".to_string(),
            exception_evidence_level: "heuristic".to_string(),
            resolution_action: "move-to-lower-tier-pressure".to_string(),
            resolution_basis: "endpoint-exception-demotion".to_string(),
            next_artifact: "data/lower-tier-pressure-witnesses.csv".to_string(),
            validation_status: "pass".to_string(),
        }];

        let rows = lower_tier_pressure_witness_rows(
            &tier_rows,
            &[],
            &resolution_rows,
            &std::collections::HashMap::new(),
        );

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].route, "I110");
        assert_eq!(rows[0].source_artifact, "data/t2-contact-resolutions.csv");
        assert_eq!(rows[0].selection_basis, "endpoint-exception-demotion");
    }

    #[test]
    fn tier_optimizer_run_gate_allows_known_held_rows() {
        let pass_artifact = write_optimizer_manifest_fixture("gate-pass", 10);
        let held_artifact = write_optimizer_manifest_fixture("gate-held", 40);
        let rows = vec![
            TierOptimizerRunRow {
                step: 1,
                optimizer_stage: "t1-stop-selector".to_string(),
                command: "route t1-stop-selector --gate".to_string(),
                artifact: pass_artifact.display().to_string(),
                row_count: 10,
                gate_status: "pass".to_string(),
                blocker_count: 0,
                blocker_summary: String::new(),
                validation_status: "pass".to_string(),
            },
            TierOptimizerRunRow {
                step: 2,
                optimizer_stage: "t2-contact-witnesses".to_string(),
                command: "route tier-contact-witnesses --gate".to_string(),
                artifact: held_artifact.display().to_string(),
                row_count: 40,
                gate_status: "held-known".to_string(),
                blocker_count: 1,
                blocker_summary: "source/contact rows remain unresolved".to_string(),
                validation_status: "held".to_string(),
            },
        ];

        assert!(tier_optimizer_run_gate_failures(true, &rows).is_empty());
        assert_eq!(
            tier_optimizer_run_gate_failures(false, &rows),
            vec!["tier-optimize bundle gate requires --all-tiers".to_string()]
        );

        let _ = std::fs::remove_dir_all(pass_artifact.parent().expect("fixture parent"));
        let _ = std::fs::remove_dir_all(held_artifact.parent().expect("fixture parent"));
    }

    #[test]
    fn optimizer_manifest_gate_requires_held_blocker_contract() {
        let artifact = write_optimizer_manifest_fixture("held-contract", 40);
        let rows = vec![TierOptimizerRunRow {
            step: 1,
            optimizer_stage: "t2-contact-witnesses".to_string(),
            command: "route tier-contact-witnesses --gate".to_string(),
            artifact: artifact.display().to_string(),
            row_count: 40,
            gate_status: "held-known".to_string(),
            blocker_count: 0,
            blocker_summary: String::new(),
            validation_status: "held".to_string(),
        }];

        let failures = optimizer_manifest_gate_failures(&rows);

        assert!(failures.contains(&"t2-contact-witnesses held without blocker count".to_string()));
        assert!(failures.contains(&"t2-contact-witnesses held without blocker summary".to_string()));
        let _ = std::fs::remove_dir_all(artifact.parent().expect("fixture parent"));
    }

    #[test]
    fn optimizer_manifest_gate_detects_stale_row_count() {
        let artifact = write_optimizer_manifest_fixture("stale-row-count", 2);
        let rows = vec![TierOptimizerRunRow {
            step: 1,
            optimizer_stage: "t1-stop-selector".to_string(),
            command: "route t1-stop-selector --gate".to_string(),
            artifact: artifact.display().to_string(),
            row_count: 3,
            gate_status: "pass".to_string(),
            blocker_count: 0,
            blocker_summary: String::new(),
            validation_status: "pass".to_string(),
        }];

        let failures = optimizer_manifest_gate_failures(&rows);

        assert!(
            failures
                .iter()
                .any(|failure| failure
                    .contains("row_count 3 does not match current artifact count 2"))
        );
        let _ = std::fs::remove_dir_all(artifact.parent().expect("fixture parent"));
    }

    #[test]
    fn optimizer_manifest_gate_requires_gateable_route_command() {
        let artifact = write_optimizer_manifest_fixture("command-contract", 1);
        let rows = vec![TierOptimizerRunRow {
            step: 1,
            optimizer_stage: "t1-stop-selector".to_string(),
            command: "manual spreadsheet check".to_string(),
            artifact: artifact.display().to_string(),
            row_count: 1,
            gate_status: "pass".to_string(),
            blocker_count: 0,
            blocker_summary: String::new(),
            validation_status: "pass".to_string(),
        }];

        let failures = optimizer_manifest_gate_failures(&rows);

        assert!(failures.contains(
            &"t1-stop-selector has non-gate optimizer command manual spreadsheet check".to_string()
        ));
        let _ = std::fs::remove_dir_all(artifact.parent().expect("fixture parent"));
    }

    #[test]
    fn optimizer_map_hook_gate_requires_pass_status() {
        let optimizer_artifact = write_optimizer_manifest_fixture("hook-optimizer", 1);
        let consumer_artifact = write_optimizer_manifest_fixture("hook-consumer", 1);
        let rows = vec![
            OptimizerMapHookRow {
                hook_id: "ok-hook".to_string(),
                optimizer_artifact: optimizer_artifact.display().to_string(),
                consumer_artifact: consumer_artifact.display().to_string(),
                consumer_type: "map".to_string(),
                gate_command: "route example --gate".to_string(),
                link_basis: "test".to_string(),
                validation_status: "pass".to_string(),
            },
            OptimizerMapHookRow {
                hook_id: "missing-hook".to_string(),
                optimizer_artifact: "data/b.csv".to_string(),
                consumer_artifact: "maps/b.png".to_string(),
                consumer_type: "map".to_string(),
                gate_command: "manual check".to_string(),
                link_basis: "test".to_string(),
                validation_status: "missing-artifact".to_string(),
            },
        ];

        assert_eq!(
            optimizer_map_hook_gate_failures(&rows),
            vec![
                "missing-hook has non-gate consumer command manual check".to_string(),
                "missing-hook optimizer artifact missing or empty".to_string(),
                "missing-hook consumer artifact missing or empty".to_string(),
                "missing-hook has non-pass validation status missing-artifact".to_string(),
            ]
        );
        let _ = std::fs::remove_dir_all(optimizer_artifact.parent().expect("fixture parent"));
        let _ = std::fs::remove_dir_all(consumer_artifact.parent().expect("fixture parent"));
    }

    #[test]
    fn bundle_architecture_gate_requires_all_crate_entrypoints() {
        let rows = bundle_architecture_rows();
        let failures = bundle_architecture_gate_failures(&rows);

        assert!(rows.iter().any(|row| row.crate_name == "route-network"
            && row.bundle_entrypoint == "route_network::build_segment_bundles"));
        assert!(rows.iter().any(|row| row.crate_name == "route-score"
            && row.bundle_entrypoint == "route_score::score_bundle"));
        assert!(rows.iter().any(|row| row.crate_name == "route-map"
            && row.bundle_entrypoint == "route_map::build_bundle_svg"));
        assert!(rows.iter().any(|row| row.crate_name == "route-sim"
            && row.bundle_entrypoint == "route_sim::BundleIncidentSpec"));
        assert!(rows.iter().any(|row| row.crate_name == "route-report"
            && row.bundle_entrypoint == "route_report::write_bundle_corpus_entry"));
        assert!(failures.is_empty());
    }

    #[test]
    fn t1_stop_selector_assigns_metis_regions_to_selected_route_stops() {
        let selector_rows = vec![T1LineSelectorInputRow {
            route: "I5".to_string(),
            selected: true,
            selected_stops: "STOP-A;STOP-B;STOP-C;STOP-D".to_string(),
        }];
        let stop_rows = ["STOP-A", "STOP-B", "STOP-C", "STOP-D"]
            .iter()
            .enumerate()
            .map(|(idx, stop_id)| StopCandidateRow {
                stop_id: (*stop_id).to_string(),
                name: format!("Stop {idx}"),
                state: "CA".to_string(),
                lat: (30.0 + idx as f64).to_string(),
                lon: "-120.0".to_string(),
                requested_class: if idx == 0 || idx == 3 { "S1" } else { "S3" }.to_string(),
                route_refs: "I-5".to_string(),
                stop_role: "service_stop".to_string(),
                transfer_value: "medium".to_string(),
                freight_volume: "medium".to_string(),
                spacing_need: "met".to_string(),
                resilience_value: "medium".to_string(),
                energy_service: "planned".to_string(),
                land_ops_feasibility: "medium".to_string(),
                equity_community: "review_needed".to_string(),
                evidence_status: "heuristic".to_string(),
                source_artifact: "fixture".to_string(),
                next_step: "validate".to_string(),
            })
            .collect::<Vec<_>>();

        let rows = t1_stop_selector_rows(&selector_rows, &stop_rows, 2).unwrap();
        let failures = t1_stop_selector_gate_failures(&rows);

        assert_eq!(rows.len(), 4);
        assert_eq!(rows.iter().filter(|row| row.boundary_after).count(), 1);
        assert!(rows
            .iter()
            .all(|row| row.split_objective == "hybrid-service"));
        assert!(failures.is_empty());
    }

    #[test]
    fn t1_topology_repairs_emit_overlap_policy_rows() {
        let review_rows = vec![
            T1DesignReviewCsvRow {
                route: "I95".to_string(),
                selected: true,
                design_role: "promise-spine".to_string(),
                promise_count: 11,
                selected_stop_count: 11,
                top_city_stop_count: 5,
                selector_reason: "sla-required-budget-fit".to_string(),
                beck_action: "overlap-review".to_string(),
                beck_review_flag: "overlap-review".to_string(),
                overlap_corridors: "I-40".to_string(),
                design_status: "policy-review".to_string(),
                next_design_action: "resolve-shared-segment-map-policy".to_string(),
            },
            T1DesignReviewCsvRow {
                route: "I5".to_string(),
                selected: true,
                design_role: "promise-spine".to_string(),
                promise_count: 7,
                selected_stop_count: 6,
                top_city_stop_count: 3,
                selector_reason: "sla-required-budget-fit".to_string(),
                beck_action: "keep".to_string(),
                beck_review_flag: "ok".to_string(),
                overlap_corridors: String::new(),
                design_status: "accepted".to_string(),
                next_design_action: "keep-in-t1-design".to_string(),
            },
        ];

        let repairs = t1_topology_repair_rows(&review_rows);
        let failures = t1_topology_repair_gate_failures(&repairs);

        assert_eq!(repairs.len(), 1);
        assert_eq!(repairs[0].repair_type, "shared-backbone-policy");
        assert!(failures.is_empty());
    }

    #[test]
    fn t1_beck_alignment_accepts_selector_routes_with_sufficient_beck_stops() {
        let rows = vec![
            T1StopSelectorInputRow {
                route: "I5".to_string(),
                stop_sequence: 1,
                stop_id: "STOP-SD-TJ".to_string(),
                stop_name: "San Diego/Tijuana".to_string(),
                requested_class: "S1".to_string(),
                selector_weight: 599,
                split_objective: "hybrid-service".to_string(),
                target_regions: 2,
                metis_region: 0,
                boundary_after: true,
                evidence_status: "heuristic".to_string(),
                validation_status: "pass".to_string(),
            },
            T1StopSelectorInputRow {
                route: "I5".to_string(),
                stop_sequence: 2,
                stop_id: "STOP-BLAINE".to_string(),
                stop_name: "Blaine/Vancouver".to_string(),
                requested_class: "S1".to_string(),
                selector_weight: 607,
                split_objective: "hybrid-service".to_string(),
                target_regions: 2,
                metis_region: 1,
                boundary_after: false,
                evidence_status: "heuristic".to_string(),
                validation_status: "pass".to_string(),
            },
        ];

        let alignment = t1_beck_alignment_rows(&rows);
        let failures = t1_beck_alignment_gate_failures(&alignment);

        assert_eq!(alignment.len(), 1);
        assert_eq!(alignment[0].alignment_status, "aligned");
        assert!(failures.is_empty());
    }

    #[test]
    fn beck_t2_gate_only_fails_structural_layout_defects() {
        for accepted in [
            "ok",
            "dense-transfer-review",
            "transfer-complexity-review",
            "long-connector-review",
        ] {
            assert!(
                !beck_t2_diagnostics_gate_failure(accepted),
                "{accepted} should not fail the structural gate"
            );
        }

        for rejected in [
            "unstopped-t1-contact-review",
            "parallel-spacing-review",
            "split-anchor-review",
            "dense-label-review",
        ] {
            assert!(
                beck_t2_diagnostics_gate_failure(rejected),
                "{rejected} should fail the structural gate"
            );
        }
    }

    #[test]
    fn bottleneck_signal_prefers_stress_then_topology() {
        assert_eq!(
            super::bottleneck_signal_label(&ScoreSignalRow {
                a1: 8.0,
                a3: 1.0,
                b2: 10.0,
            }),
            "corridor_stress"
        );
        assert_eq!(
            super::bottleneck_signal_label(&ScoreSignalRow {
                a1: 1.0,
                a3: 1.0,
                b2: 10.0,
            }),
            "topology_chokepoint"
        );
        assert_eq!(
            super::bottleneck_signal_label(&ScoreSignalRow {
                a1: 1.0,
                a3: 1.0,
                b2: 2.0,
            }),
            "capacity_needs_flow"
        );
    }

    #[test]
    fn score_all_csv_dimension_values_cover_full_rubric() {
        let scores = score_corridor(
            &CorridorAttributes::default(),
            &ScoringConfig::default_config(),
        );

        assert_eq!(dimension_score_values(&scores).len(), 16);
        assert_eq!(dimension_estimated_values(&scores).len(), 16);
        assert_eq!(dimension_confidence_values(&scores).len(), 16);
    }

    #[test]
    fn confidence_risk_dimensions_prioritizes_scored_low_confidence_dimensions() {
        let mut scores = [0.0; 16];
        let mut confidences = [0.9; 16];
        scores[0] = 9.0;
        scores[1] = 6.0;
        scores[2] = 10.0;
        scores[13] = 5.0;
        confidences[1] = 0.45;
        confidences[2] = 0.55;
        confidences[13] = 0.50;

        assert_eq!(
            confidence_risk_dimensions(&scores, &confidences),
            "A3:10.0@0.55;A2:6.0@0.45;D1:5.0@0.50"
        );
    }

    #[test]
    fn dimension_confidence_risks_clamps_confidence_to_valid_range() {
        let mut scores = [0.0; 16];
        let mut confidences = [1.0; 16];
        scores[0] = 10.0;
        scores[1] = 10.0;
        confidences[0] = -0.5;
        confidences[1] = 1.5;

        let risks = dimension_confidence_risks(&scores, &confidences);

        assert_eq!(risks[0], 10.0);
        assert_eq!(risks[1], 0.0);
    }

    #[test]
    fn tier_artifacts_sort_by_tier_then_descending_score() {
        let rows = vec![
            score_row("I2", 55.0, "T2"),
            score_row("I1", 75.0, "T1"),
            score_row("I3", 82.0, "T1"),
        ];

        let dir = std::env::temp_dir().join(format!("route-tier-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);

        write_tier_artifacts_to(&rows, &dir).expect("write tier artifacts");

        let csv = std::fs::read_to_string(dir.join("tier-table.csv")).expect("read tier csv");
        let route_order: Vec<&str> = csv
            .lines()
            .skip(1)
            .filter_map(|line| line.split(',').nth(1))
            .take(3)
            .collect();
        assert_eq!(route_order, ["I3", "I1", "I2"]);

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn t1_line_selector_prioritizes_sla_required_routes() {
        let dir =
            std::env::temp_dir().join(format!("route-t1-selector-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create selector test dir");
        let tier_path = dir.join("tier-table.csv");
        let stops_path = dir.join("tier-stop-candidates.csv");
        let sla_path = dir.join("t1-sla-pairs.csv");
        let exceptions_path = dir.join("t1-score-exceptions.csv");
        let constraint_budget_path = dir.join("optimizer-constraint-budget.csv");
        std::fs::write(
            &tier_path,
            "\
tier,route,score,rubric_version,estimated,confidence,score_confidence,confidence_label,score_confidence_label
T1,I95,98.0,v,true,0.8,0.8,Medium,Medium
T1,I10,96.0,v,true,0.8,0.8,Medium,Medium
T1,I20,75.0,v,true,0.8,0.8,Medium,Medium
",
        )
        .expect("write tier");
        std::fs::write(
            &stops_path,
            "\
stop_id,name,state,lat,lon,requested_class,route_refs,stop_role,transfer_value,freight_volume,spacing_need,resilience_value,energy_service,land_ops_feasibility,equity_community,evidence_status,source_artifact,next_step
STOP-NY,New York,NY,1,1,S1,I95,terminal,high,high,met,high,planned,medium,required,heuristic,artifact,next
STOP-LA,Los Angeles,CA,2,2,S1,I10; I20,terminal,high,high,met,high,planned,medium,required,heuristic,artifact,next
STOP-ATL,Atlanta,GA,3,3,S2,I20,hub,high,high,met,high,planned,medium,required,heuristic,artifact,next
",
        )
        .expect("write stops");
        std::fs::write(
            &sla_path,
            "\
pair_id,origin_id,dest_id,target_hours,priority,market_class,required_routes,required_stops,evidence_basis
ATL-LA-48,STOP-ATL,STOP-LA,48,10,air-substitution,I20,STOP-ATL;STOP-LA,C.3
",
        )
        .expect("write sla");
        std::fs::write(
            &exceptions_path,
            "\
route,decision,exception_type,rationale,evidence_status,artifact,replacement_candidate,next_selector_action
I95,keep,fixture-score-backbone,Fixture route kept by explicit score exception so the selector can test SLA priority ahead of score-only T1 rows,heuristic,fixture,,keep in fixture selector
",
        )
        .expect("write exceptions");

        let rows = crate::support::tier::t1_line_selector_rows::t1_line_selector_rows(
            &tier_path,
            &stops_path,
            &sla_path,
            &exceptions_path,
            &constraint_budget_path,
            2,
            2,
            10,
        )
        .expect("selector rows");
        let selected = rows
            .iter()
            .filter(|row| row.selected)
            .map(|row| row.route.as_str())
            .collect::<Vec<_>>();
        assert_eq!(selected, vec!["I20", "I95"]);
        assert!(rows
            .iter()
            .find(|row| row.route == "I20")
            .unwrap()
            .reason
            .contains("sla-required"));
        assert!(t1_line_selector_gate_failures(&rows, 2, 10).is_empty());

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn standards_proof_ledger_parses_and_gates_unresolved_rows() {
        let csv = "\
standard_id,tier,standard_family,standard,outcome,mechanism,primary_stressor,acceptance_gate,evidence_level,current_artifact,blocking_gap,next_command_or_test,owner_track
T1-DIAMOND-K,T1,resilience,k >= 3,outcome,mechanism,closure,gate,Heuristic,artifact,manual validation needed,next,B.4
T3-COVERAGE,T3,access,coverage,outcome,mechanism,gap,gate,Implemented,artifact,,next,B.1
";

        let rows = parse_standards_proof_ledger(csv.as_bytes()).expect("parse proof ledger");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].standard_id, "T1-DIAMOND-K");
        assert_eq!(rows[1].evidence_level, "Implemented");

        let failures = standards_blueprint_gate_failures(&rows);
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].standard_id, "T1-DIAMOND-K");
    }

    #[test]
    fn standards_proof_evidence_levels_use_blueprint_vocabulary() {
        for level in ["Implemented", "Heuristic", "Stub", "Planned", "Deprecated"] {
            assert!(standards_evidence_level_is_allowed(level));
        }
        assert!(standards_evidence_level_is_allowed(" heuristic "));
        assert!(!standards_evidence_level_is_allowed("unknown"));
        assert!(!standards_evidence_level_is_allowed(""));
    }

    #[test]
    fn standards_blueprint_gate_rejects_unknown_evidence_levels() {
        let csv = "\
standard_id,tier,standard_family,standard,outcome,mechanism,primary_stressor,acceptance_gate,evidence_level,current_artifact,blocking_gap,next_command_or_test,owner_track
T1-UNKNOWN,T1,resilience,claim,outcome,mechanism,closure,gate,Unlabeled,artifact,,next,B.4
";

        let rows = parse_standards_proof_ledger(csv.as_bytes()).expect("parse proof ledger");
        let failures = standards_blueprint_gate_failures(&rows);

        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].standard_id, "T1-UNKNOWN");
    }

    #[test]
    fn standards_pressure_gate_accepts_labeled_unresolved_proof_records() {
        let csv = "\
standard_id,tier,standard_family,standard,outcome,mechanism,primary_stressor,acceptance_gate,evidence_level,current_artifact,blocking_gap,next_command_or_test,owner_track
T1-OPS-PTI,T1,throughput,pti,outcome,mechanism,peak,gate,Heuristic,artifact,NPMRDS direct evidence missing,next,C.1
T1-BRIDGE,T1,safety,bridge,outcome,mechanism,posting,gate,Planned,artifact,clearance join missing,next,E.2
BAD,T1,throughput,pti,outcome,mechanism,peak,gate,unknown,artifact,gap,next,C.1
";

        let rows = parse_standards_proof_ledger(csv.as_bytes()).expect("parse proof ledger");
        let failures = standards_pressure_gate_failures(&rows);

        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].standard_id, "BAD");
    }

    #[test]
    fn standards_pressure_gate_canonical_ledger_passes_contract_check() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/standards-proof-ledger.csv");
        let file = std::fs::File::open(path).expect("open canonical standards proof ledger");
        let rows = parse_standards_proof_ledger(file).expect("parse canonical standards");

        assert!(!rows.is_empty());
        assert!(standards_pressure_gate_failures(&rows).is_empty());
    }

    #[test]
    fn forum_docket_gate_requires_review_contracts() {
        let csv = "\
review_id,artifact,review_type,status,roles,claim_target,blocking_question,next_action,output_artifact
F5-01,docs/milepost-4-closeout.md,parliament,complete,traffic-engineer,claim,question,next,docs/forum/review.md
BAD,,unknown,maybe,,claim,,next,
";

        let rows = parse_forum_docket(csv.as_bytes()).expect("parse forum docket");

        assert!(forum_docket_row_failure(&rows[0]).is_none());
        assert!(forum_docket_row_failure(&rows[1]).is_some());
        let failures = forum_docket_gate_failures(&rows);
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("BAD"));
    }

    #[test]
    fn forum_docket_canonical_file_passes_gate() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/forum-docket.csv");
        let file = std::fs::File::open(path).expect("open canonical forum docket");
        let rows = parse_forum_docket(file).expect("parse canonical forum docket");

        assert!(!rows.is_empty());
        assert!(rows.iter().any(|row| row.review_id == "F5-01"));
        assert!(forum_docket_gate_failures(&rows).is_empty());
    }

    #[test]
    fn significant_moments_gate_requires_complete_contracts() {
        let csv = "\
moment_id,date,flair,kind,summary,why_it_mattered,primary_artifacts,commit,next_thread
MOM-2026-05-12-GOOD,2026-05-12,Good Moment,conceptual_breakthrough,summary,why,docs/SPEC_INDEX.md,44f92db,next
BAD,2026/05/12,,note,summary,,missing.md,notsha,
";

        let rows = parse_significant_moments(csv.as_bytes()).expect("parse moments");

        assert!(significant_moment_row_failure(&rows[0]).is_none());
        assert!(significant_moment_row_failure(&rows[1]).is_some());
        let failures = significant_moment_gate_failures(&rows);
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("BAD"));
    }

    #[test]
    fn significant_moments_canonical_file_passes_gate() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/significant-moments.csv");
        let file = std::fs::File::open(path).expect("open canonical moments ledger");
        let rows = parse_significant_moments(file).expect("parse canonical moments");

        assert!(!rows.is_empty());
        assert!(rows.iter().any(|row| row.flair == "Promise Horizon"));
        assert!(significant_moment_gate_failures(&rows).is_empty());
    }

    #[test]
    fn release_manifest_gate_requires_valid_metadata_and_existing_artifacts() {
        let csv = "\
artifact_path,artifact_class,owner_milepost,release_status,public_status,verification_command,notes
docs/SPEC_INDEX.md,index,M7,release_candidate,public,manual review,index
data/release-manifest.csv,release_manifest,M7,release_candidate,public,route release-manifest --gate,self
docs/source-fetch-cache-policy.md,source_policy,M10,release_candidate,public,manual review,policy
data/source-fetch-policy.csv,source_policy,M10,release_candidate,public,route source-fetch-policy --gate,ledger
data/map-atlas.csv,map_manifest,M2,release_candidate,public,spreadsheet check,bad command
missing.md,doc,Mx,bad_status,bad_public,manual review,missing
";

        let rows = parse_release_manifest(csv.as_bytes()).expect("parse release manifest");
        let failures = release_manifest_gate_failures(&rows);

        assert!(failures
            .iter()
            .any(|failure| failure.contains("missing.md")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("invalid owner milepost")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("invalid release status")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("invalid public status")));
        assert!(failures.iter().any(|failure| failure.contains(
            "data/map-atlas.csv has unsupported verification command spreadsheet check"
        )));
    }

    #[test]
    fn release_manifest_canonical_file_passes_gate() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/release-manifest.csv");
        let file = std::fs::File::open(path).expect("open canonical release manifest");
        let rows = parse_release_manifest(file).expect("parse canonical release manifest");

        assert!(!rows.is_empty());
        assert!(rows
            .iter()
            .any(|row| row.artifact_path == "data/source-fetch-policy.csv"));
        assert!(release_manifest_gate_failures(&rows).is_empty());
    }

    #[test]
    fn blueprint_gate_requires_forum_intake_contracts() {
        let csv = "\
package_id,phase,feature_package,stakeholder_class,standards,evidence_level,status,cost_range,value_case,source_label,pressure_artifact,forum_constraint,mitigation_companion,row_complexity,maintenance_burden,community_exposure_check,rural_access_exception,blueprint_action,blocking_gap,next_evidence_step
B6-GOOD,Phase 0,Relay operations,operational_must_have,T1-REST,Heuristic,blueprint_candidate,$40M-$250M,heuristic reliability seed,Forum F5-07,S-L2-RELAY-HUB,claims remain heuristic,not_applicable_no_new_footprint,existing hubs,low operations,no new lane footprint,not_applicable,include with heuristic label,NPMRDS absent,calibrate PTI
B6-BAD,Phase 1,Managed lanes,conditional_expansion,T1-OPS-PTI,Unknown,candidate,$1B,benefit claim,Forum F5-07,S-L2-MANAGED-LANE,needs mitigation,not_applicable,not_applicable,not_applicable,not_applicable,,promote as proven,,none
";

        let rows = parse_blueprint_packages(csv.as_bytes()).expect("parse blueprint packages");

        assert!(blueprint_row_contract_failure(&rows[0]).is_none());
        assert!(blueprint_row_contract_failure(&rows[1]).is_some());
        let failures = blueprint_gate_failures(&rows);
        assert!(failures.iter().any(|failure| failure.contains("B6-BAD")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("conditional expansion lacks required")));
    }

    #[test]
    fn blueprint_canonical_ledger_passes_gate() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/blueprint-feature-packages.csv");
        let file = std::fs::File::open(path).expect("open canonical blueprint ledger");
        let rows = parse_blueprint_packages(file).expect("parse canonical blueprint ledger");

        assert!(!rows.is_empty());
        assert!(rows.iter().any(|row| row.package_id == "B6-P0-RELAY-OPS"));
        assert!(blueprint_gate_failures(&rows).is_empty());
    }

    #[test]
    fn blueprint_evidence_gate_links_packages_and_standards() {
        let packages_csv = "\
package_id,phase,feature_package,stakeholder_class,standards,evidence_level,status,cost_range,value_case,source_label,pressure_artifact,forum_constraint,mitigation_companion,row_complexity,maintenance_burden,community_exposure_check,rural_access_exception,blueprint_action,blocking_gap,next_evidence_step
B6-GOOD,Phase 0,Relay operations,operational_must_have,T1-REST; T1-RECOVERY,Heuristic,blueprint_candidate,$40M-$250M,heuristic reliability seed,Forum F5-07,S-L2-RELAY-HUB,claims remain heuristic,not_applicable_no_new_footprint,existing hubs,low operations,no new lane footprint,not_applicable,include with heuristic label,NPMRDS absent,calibrate PTI
";
        let standards_csv = "\
standard_id,tier,standard_family,standard,outcome,mechanism,primary_stressor,acceptance_gate,evidence_level,current_artifact,blocking_gap,next_command_or_test,owner_track
T1-REST,T1,operations,rest,outcome,mechanism,parking,gate,Planned,artifact,gap,next,F
T1-RECOVERY,T1,resilience,recovery,outcome,mechanism,closure,gate,Heuristic,artifact,gap,next,C.2
";
        let evidence_csv = "\
package_id,standard_id,proof_evidence_level,blueprint_claim_status,promotion_rule,proof_artifact,forum_hold,blocking_gap,required_next_evidence
B6-GOOD,T1-REST,Planned,planned,source row required before promotion,artifact,Forum F5-07,gap,next
B6-GOOD,T1-RECOVERY,Heuristic,heuristic,keep scenario label visible,artifact,Forum F5-01,gap,next
";

        let packages = parse_blueprint_packages(packages_csv.as_bytes()).expect("packages");
        let standards = parse_standards_proof_ledger(standards_csv.as_bytes()).expect("standards");
        let evidence = parse_blueprint_evidence_map(evidence_csv.as_bytes()).expect("evidence");
        let package_ids = packages
            .iter()
            .map(|row| row.package_id.as_str())
            .collect::<std::collections::HashSet<_>>();
        let standard_evidence = standards
            .iter()
            .map(|row| (row.standard_id.as_str(), row.evidence_level.as_str()))
            .collect::<std::collections::HashMap<_, _>>();

        assert!(
            blueprint_evidence_row_failure(&evidence[0], &package_ids, &standard_evidence)
                .is_none()
        );
        assert!(blueprint_evidence_gate_failures(&evidence, &packages, &standards).is_empty());
    }

    #[test]
    fn blueprint_evidence_canonical_map_passes_gate() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let packages_file = std::fs::File::open(root.join("data/blueprint-feature-packages.csv"))
            .expect("open canonical blueprint packages");
        let standards_file = std::fs::File::open(root.join("data/standards-proof-ledger.csv"))
            .expect("open canonical standards proof ledger");
        let evidence_file = std::fs::File::open(root.join("data/blueprint-evidence-map.csv"))
            .expect("open canonical blueprint evidence map");
        let packages = parse_blueprint_packages(packages_file).expect("parse packages");
        let standards = parse_standards_proof_ledger(standards_file).expect("parse standards");
        let evidence = parse_blueprint_evidence_map(evidence_file).expect("parse evidence");

        assert!(!evidence.is_empty());
        assert!(evidence
            .iter()
            .any(|row| row.package_id == "B6-P1-MANAGED-LANE-PILOT"
                && row.standard_id == "T1-OPS-PTI"));
        assert!(blueprint_evidence_gate_failures(&evidence, &packages, &standards).is_empty());
    }

    #[test]
    fn blueprint_cost_gate_links_packages_and_rejects_premature_source_claims() {
        let packages_csv = "\
package_id,phase,feature_package,stakeholder_class,standards,evidence_level,status,cost_range,value_case,source_label,pressure_artifact,forum_constraint,mitigation_companion,row_complexity,maintenance_burden,community_exposure_check,rural_access_exception,blueprint_action,blocking_gap,next_evidence_step
B6-GOOD,Phase 0,Relay operations,operational_must_have,T1-REST,Heuristic,blueprint_candidate,$40M-$250M,heuristic reliability seed,Forum F5-07,S-L2-RELAY-HUB,claims remain heuristic,not_applicable_no_new_footprint,existing hubs,low operations,no new lane footprint,not_applicable,include with heuristic label,NPMRDS absent,calibrate PTI
";
        let costs_csv = "\
package_id,cost_basis,capital_range_2026_usd,lifecycle_burden,source_status,source_artifact,cost_claim_status,risk_note,next_cost_step
B6-GOOD,planning seed,$40M-$250M,low operations,planning_range,docs/blueprint/feature-packages.md,planning_only,wide proxy range,collect source costs
B6-BAD,pretend source,$1B,high,planning_range,artifact,source_backed,risk,next
";
        let packages = parse_blueprint_packages(packages_csv.as_bytes()).expect("packages");
        let costs = parse_blueprint_cost_ranges(costs_csv.as_bytes()).expect("costs");
        let package_ids = packages
            .iter()
            .map(|row| row.package_id.as_str())
            .collect::<std::collections::HashSet<_>>();

        assert!(blueprint_cost_row_failure(&costs[0], &package_ids).is_none());
        assert!(blueprint_cost_row_failure(&costs[1], &package_ids).is_some());
        let failures = blueprint_cost_gate_failures(&costs, &packages);
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("B6-BAD"));
    }

    #[test]
    fn blueprint_cost_canonical_ledger_passes_gate() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let packages_file = std::fs::File::open(root.join("data/blueprint-feature-packages.csv"))
            .expect("open canonical blueprint packages");
        let costs_file = std::fs::File::open(root.join("data/blueprint-cost-ranges.csv"))
            .expect("open canonical blueprint costs");
        let packages = parse_blueprint_packages(packages_file).expect("parse packages");
        let costs = parse_blueprint_cost_ranges(costs_file).expect("parse costs");

        assert!(!costs.is_empty());
        assert!(costs.iter().any(|row| row.package_id == "B6-P0-RELAY-OPS"));
        assert!(blueprint_cost_gate_failures(&costs, &packages).is_empty());
    }

    #[test]
    fn standards_inventory_requires_l1_source_contracts() {
        let csv = "\
standard_id,inventory_name,source_kind,source_status,current_artifact,coverage_scope,blocking_gap,next_step
T1-BRIDGE,bridge ledger,FHWA NBI,partial,data/cache/nbi_bridges.csv,T1 bridges,gap,next
T1-REST,rest ledger,state DOT,unknown,,T1 rest areas,gap,
";

        let rows = parse_standards_inventory(csv.as_bytes()).expect("parse standards inventory");

        assert!(standards_inventory_row_has_contract(&rows[0]));
        assert!(!standards_inventory_row_has_contract(&rows[1]));
        let failures = standards_inventory_gate_failures(&rows);
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].standard_id, "T1-REST");
    }

    #[test]
    fn planned_standard_inventory_gate_requires_rows_for_planned_standards() {
        let standards_csv = "\
standard_id,tier,standard_family,standard,outcome,mechanism,primary_stressor,acceptance_gate,evidence_level,current_artifact,blocking_gap,next_command_or_test,owner_track
T1-REST,T1,operations,rest,outcome,mechanism,outage,gate,Planned,artifact,gap,next,F
T1-BRIDGE,T1,safety,bridge,outcome,mechanism,posting,gate,Planned,artifact,gap,next,E.2
T1-OPS-PTI,T1,throughput,pti,outcome,mechanism,peak,gate,Heuristic,artifact,gap,next,C.1
";
        let inventory_csv = "\
standard_id,inventory_name,source_kind,source_status,current_artifact,coverage_scope,blocking_gap,next_step
T1-REST,rest ledger,state DOT,source_needed,artifact,T1 rest areas,gap,next
";

        let standards =
            parse_standards_proof_ledger(standards_csv.as_bytes()).expect("parse standards");
        let inventory =
            parse_standards_inventory(inventory_csv.as_bytes()).expect("parse inventory");
        let missing = planned_standard_inventory_missing(&standards, &inventory);

        assert_eq!(missing.len(), 1);
        assert_eq!(missing[0].standard_id, "T1-BRIDGE");
    }

    #[test]
    fn standards_inventory_canonical_ledger_covers_planned_standards() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let standards_file = std::fs::File::open(root.join("data/standards-proof-ledger.csv"))
            .expect("open canonical standards proof ledger");
        let inventory_file = std::fs::File::open(root.join("data/standards-l1-inventory.csv"))
            .expect("open canonical standards inventory");
        let standards =
            parse_standards_proof_ledger(standards_file).expect("parse canonical standards");
        let inventory =
            parse_standards_inventory(inventory_file).expect("parse canonical inventory");

        assert!(standards_inventory_gate_failures(&inventory).is_empty());
        assert!(planned_standard_inventory_missing(&standards, &inventory).is_empty());
    }

    #[test]
    fn pavement_standards_require_tier_thresholds_and_contracts() {
        let rows = vec![
            PavementStandardRow {
                tier: "T1".to_string(),
                road_role: "national timed-freight spine".to_string(),
                max_iri_m_per_km: 1.5,
                target_pavement_condition: "good".to_string(),
                freight_ride_requirement: "no roughness padding".to_string(),
                transit_ride_requirement: "coach-speed ride quality".to_string(),
                inspection_interval_months: 6,
                repair_trigger: "repair above threshold".to_string(),
                allowed_exception: "temporary construction only".to_string(),
                source_contract: "HPMS IRI plus state pavement feeds".to_string(),
                validation_status: "pass".to_string(),
            },
            PavementStandardRow {
                tier: "T2".to_string(),
                road_role: "regional connector".to_string(),
                max_iri_m_per_km: 0.0,
                target_pavement_condition: String::new(),
                freight_ride_requirement: String::new(),
                transit_ride_requirement: "regional coach ride quality".to_string(),
                inspection_interval_months: 48,
                repair_trigger: String::new(),
                allowed_exception: String::new(),
                source_contract: String::new(),
                validation_status: "unknown".to_string(),
            },
        ];

        let failures = pavement_standard_gate_failures(&rows);

        assert!(failures.iter().any(|failure| failure.contains("T2")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("T3 missing")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("T4 missing")));
    }

    #[test]
    fn bridge_standards_gate_flags_missing_nbi_routes() {
        let routes = vec!["I10".to_string(), "I95".to_string()];
        let mut nbi = std::collections::HashMap::new();
        nbi.insert(
            "I10".to_string(),
            NbiBridgeRecord {
                pct_bridges_poor: 0.01,
                mean_year_built: 1980.0,
                bridge_count: 100,
            },
        );

        assert_eq!(bridge_standard_missing_routes(&routes, &nbi), vec!["I95"]);
    }

    #[test]
    fn tier_route_loader_normalizes_generated_tier_table_routes() {
        let path =
            std::env::temp_dir().join(format!("route-tier-routes-{}.csv", std::process::id()));
        std::fs::write(
            &path,
            "tier,route,score\nT1,I-95,98.3\nT1,I-10,95.9\nT2,I-64,55.0\n",
        )
        .expect("write tier table fixture");

        let routes = load_tier_routes(&path, "T1").expect("load tier routes");

        assert_eq!(routes, vec!["I10".to_string(), "I95".to_string()]);
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn endpoint_exception_terminal_worthiness_requires_contract_and_role() {
        let csv = "\
route,requested_tier,endpoint_name,endpoint_role,exception_type,evidence_level,artifact,next_step
I65,T2,Mobile,t2_terminal_exception,port_terminal,heuristic,data/ports.csv,validate terminal
I270,T2,Beltway,local_access_end,metro_beltway_relief,heuristic,data/atri-bottlenecks.csv,validate relief
";
        let rows = parse_endpoint_exceptions(csv.as_bytes()).expect("parse endpoint exceptions");

        assert!(endpoint_exception_is_terminal_worthy(&rows[0]));
        assert!(!endpoint_exception_is_terminal_worthy(&rows[1]));
    }

    #[test]
    fn endpoint_exception_filter_and_gate_find_incomplete_rows() {
        let csv = "\
route,requested_tier,endpoint_name,endpoint_role,exception_type,evidence_level,artifact,next_step
I65,T2,Mobile,t2_terminal_exception,port_terminal,heuristic,data/ports.csv,validate terminal
I25,T2,Denver,t2_terminal_exception,regional_terminal,unknown,,validate graph
";
        let rows = parse_endpoint_exceptions(csv.as_bytes()).expect("parse endpoint exceptions");
        let filtered = filter_endpoint_exceptions(&rows, Some("T2"), Some("I-25"));
        let failures = endpoint_exception_gate_failures(&filtered, false);

        assert_eq!(filtered.len(), 1);
        assert_eq!(failures.len(), 2);
        assert!(failures
            .iter()
            .any(|failure| failure.contains("incomplete endpoint exception")));

        let promotion_failures = endpoint_exception_gate_failures(&filtered, true);
        assert!(promotion_failures
            .iter()
            .any(|failure| failure.contains("not terminal-worthy")));
    }

    #[test]
    fn one_ended_t2_can_pass_with_terminal_exception_but_missing_graph_stays_blocked() {
        let csv = "\
route,requested_tier,endpoint_name,endpoint_role,exception_type,evidence_level,artifact,next_step
I65,T2,Mobile,t2_terminal_exception,port_terminal,heuristic,data/ports.csv,validate terminal
I25,T2,Denver,t2_terminal_exception,regional_terminal,heuristic,data/tier-table.csv,validate graph
";
        let exceptions =
            parse_endpoint_exceptions(csv.as_bytes()).expect("parse endpoint exceptions");
        let rows = vec![
            route_network::TierConnectivityRow {
                route: "I65".to_string(),
                route_miles: 400.0,
                t1_node_count: 1,
                t1_routes: vec!["I10".to_string()],
                touch_nodes: Vec::new(),
                classification: route_network::TierNodeClass::OneEndedFeeder,
            },
            route_network::TierConnectivityRow {
                route: "I25".to_string(),
                route_miles: 1200.0,
                t1_node_count: 0,
                t1_routes: Vec::new(),
                touch_nodes: Vec::new(),
                classification: route_network::TierNodeClass::MissingGraphData,
            },
        ];

        let failures = tier_connectivity_gate_failures_with_exceptions(&rows, &exceptions, "T2");

        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].row.route, "I25");
        assert!(failures[0].reason.contains("graph/contact data"));
    }

    #[test]
    fn stop_candidates_parse_and_filter_by_route() {
        let csv = "\
stop_id,name,state,lat,lon,requested_class,route_refs,stop_role,transfer_value,freight_volume,spacing_need,resilience_value,energy_service,land_ops_feasibility,equity_community,evidence_status,source_artifact,next_step
STOP-ATL,Atlanta,GA,33.75,-84.35,S2,\"I-20; I-75; I-85\",major_interchange_hub,high,high,met,high,planned,low,required,heuristic,data/relay-hubs.toml,validate site
STOP-LOCAL,Local Spur,GA,33.1,-84.1,S5,I-285,local_access_stop,low,low,met,low,planned,medium,review_needed,heuristic,data/tier-table.csv,demote if needed
";
        let rows = parse_stop_candidates(csv.as_bytes()).expect("parse stop candidates");
        let filtered = filter_stop_candidates(&rows, Some("S2"), Some("I-75"));

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].stop_id, "STOP-ATL");
        assert!(stop_candidate_gate_failures(&filtered).is_empty());
    }

    #[test]
    fn stop_candidate_gate_rejects_unreviewable_major_hub() {
        let csv = "\
stop_id,name,state,lat,lon,requested_class,route_refs,stop_role,transfer_value,freight_volume,spacing_need,resilience_value,energy_service,land_ops_feasibility,equity_community,evidence_status,source_artifact,next_step
STOP-BAD,Bad Hub,GA,33.75,-84.35,S2,I-20,local_access_stop,low,low,met,low,planned,medium,review_needed,unknown,,
";
        let rows = parse_stop_candidates(csv.as_bytes()).expect("parse stop candidates");
        let refs = rows.iter().collect::<Vec<_>>();
        let failures = stop_candidate_gate_failures(&refs);

        assert!(failures
            .iter()
            .any(|failure| failure.contains("unsupported evidence_status")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("S2 needs at least two route_refs")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("missing source_artifact")));
    }

    #[test]
    fn s1_border_terminal_can_have_one_route_ref() {
        let csv = "\
stop_id,name,state,lat,lon,requested_class,route_refs,stop_role,transfer_value,freight_volume,spacing_need,resilience_value,energy_service,land_ops_feasibility,equity_community,evidence_status,source_artifact,next_step
STOP-BORDER,Border Terminal,WA,48.99,-122.75,S1,I-5,national_terminal; border_gateway,high,high,met,high,planned,medium,required,heuristic,data/ports.csv,validate border
";
        let rows = parse_stop_candidates(csv.as_bytes()).expect("parse stop candidates");
        let refs = rows.iter().collect::<Vec<_>>();

        assert!(stop_candidate_gate_failures(&refs).is_empty());
    }

    #[test]
    fn stop_plan_sorts_i5_like_chain_south_to_north() {
        let csv = "\
stop_id,name,state,lat,lon,requested_class,route_refs,stop_role,transfer_value,freight_volume,spacing_need,resilience_value,energy_service,land_ops_feasibility,equity_community,evidence_status,source_artifact,next_step
STOP-SEA,Seattle,WA,47.58,-122.33,S2,\"I-5; I-90\",major_interchange_hub,high,medium,met,high,planned,medium,review_needed,heuristic,data/relay-hubs.toml,next
STOP-LA-LB,Los Angeles/Long Beach,CA,33.95,-118.20,S1,\"I-5; I-10\",national_terminal,high,high,met,high,planned,medium,required,heuristic,data/ports.csv,next
STOP-SAC,Sacramento,CA,38.58,-121.49,S2,\"I-5; I-80\",major_interchange_hub,high,medium,met,high,planned,medium,review_needed,heuristic,data/relay-hubs.toml,next
";
        let rows = parse_stop_candidates(csv.as_bytes()).expect("parse stop candidates");
        let plan = stop_plan_for_route(&rows, "I5");

        assert_eq!(
            plan.iter()
                .map(|row| row.stop_id.as_str())
                .collect::<Vec<_>>(),
            vec!["STOP-LA-LB", "STOP-SAC", "STOP-SEA"]
        );
        assert!(stop_plan_gate_failures("I5", &plan).is_empty());
    }

    #[test]
    fn stop_coverage_flags_routes_without_visible_stop_chains() {
        let csv = "\
stop_id,name,state,lat,lon,requested_class,route_refs,stop_role,transfer_value,freight_volume,spacing_need,resilience_value,energy_service,land_ops_feasibility,equity_community,evidence_status,source_artifact,next_step
STOP-A,A,AA,1.0,1.0,S1,I-10,national_terminal; border_gateway,high,high,met,high,planned,medium,required,heuristic,data/ports.csv,next
STOP-B,B,BB,2.0,2.0,S2,\"I-10; I-20\",major_interchange_hub,high,medium,met,high,planned,medium,review_needed,heuristic,data/relay-hubs.toml,next
STOP-C,C,CC,3.0,3.0,S1,I-10,national_terminal; port_gateway,high,high,met,high,planned,medium,required,heuristic,data/ports.csv,next
";
        let rows = parse_stop_candidates(csv.as_bytes()).expect("parse stop candidates");
        let coverage =
            stop_coverage_for_routes(&rows, &["I10".to_string(), "I95".to_string()], "T1");
        let failures = stop_coverage_gate_failures(&coverage);

        assert_eq!(coverage[0].stop_count, 3);
        assert!(coverage[0].failures.is_empty());
        assert_eq!(coverage[1].stop_count, 0);
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("I95"));
    }

    #[test]
    fn t3_stop_coverage_accepts_two_stop_regional_chain() {
        let csv = "\
stop_id,name,state,lat,lon,requested_class,route_refs,stop_role,transfer_value,freight_volume,spacing_need,resilience_value,energy_service,land_ops_feasibility,equity_community,evidence_status,source_artifact,next_step
STOP-XFER,Transfer Hub,AA,1.0,1.0,S3,\"I-57; I-70\",transfer_stop,medium,medium,met,medium,planned,medium,review_needed,heuristic,data/tier-table.csv,next
STOP-TERM,Regional Terminal,BB,2.0,2.0,S4,I-57,regional_terminal,medium,medium,met,medium,planned,medium,review_needed,heuristic,data/tier-table.csv,next
";
        let rows = parse_stop_candidates(csv.as_bytes()).expect("parse stop candidates");
        let coverage = stop_coverage_for_routes(&rows, &["I57".to_string()], "T3");

        assert_eq!(coverage[0].stop_count, 2);
        assert!(coverage[0].failures.is_empty());
    }

    #[test]
    fn map_atlas_manifest_requires_existing_png_contracts() {
        let csv = "\
map_id,path,map_type,render_command,expected_width,expected_height,min_bytes,tier_role,game_use
missing,maps/does-not-exist.png,national,route map all,2400,1350,100,tier overview,campaign atlas
";

        let rows = parse_map_atlas(csv.as_bytes()).expect("parse map atlas");
        let failures = map_atlas_gate_failures(&rows);

        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("missing maps/does-not-exist.png"));
    }

    #[test]
    fn map_atlas_canonical_manifest_passes_contract_gate() {
        let path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/map-atlas.csv");
        let file = std::fs::File::open(path).expect("open canonical map atlas");
        let rows = parse_map_atlas(file).expect("parse canonical map atlas");

        assert!(!rows.is_empty());
        assert!(map_atlas_gate_failures(&rows).is_empty());
    }

    #[test]
    fn map_publication_readiness_passes_without_publication_blockers() {
        let atlas_path =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/map-atlas.csv");
        let atlas_file = std::fs::File::open(atlas_path).expect("open canonical map atlas");
        let atlas_rows = parse_map_atlas(atlas_file).expect("parse canonical map atlas");
        let backlog_rows = vec![OptimizerResidualBlockerBacklogRow {
            backlog_id: "ORB-T4-UPGRADE".to_string(),
            priority_class: "P1-terminal-evidence".to_string(),
            blocker_family: "terminal_access_evidence_gap".to_string(),
            tier: "T4".to_string(),
            blocked_claims: "upgrade".to_string(),
            subject_count: 69,
            route_count: 69,
            total_hard_blockers: 0,
            total_claim_blockers: 69,
            total_budget_debt_count: 0,
            total_constraint_debt_cost_m: 0.0,
            total_constraint_penalty_score: 69.0,
            representative_routes: "I-115".to_string(),
            representative_subjects: "I-115".to_string(),
            next_artifacts: "data/t4-terminal-contact-evidence.csv".to_string(),
            backlog_decision: "triage-only-no-blocker-relief".to_string(),
            next_wave: "terminal-access-evidence-review".to_string(),
            validation_status: "review".to_string(),
        }];
        let scope_rows = vec![MapPublicationScopeDecisionRow {
            decision_id: "MAPPUB-FULL-T1T4".to_string(),
            decision_scope: "full-t1-t4-publication".to_string(),
            map_surface: "data/map-atlas.csv".to_string(),
            render_gate_status: "pass".to_string(),
            evidence_gate_status: "held".to_string(),
            claim_status: "publication-valid-with-held-nonpublication-claims".to_string(),
            blocked_claims: "evidence|upgrade".to_string(),
            claim_blocker_count: 69,
            budget_debt_count: 0,
            blocking_artifacts: "data/optimizer-residual-blocker-backlog.csv".to_string(),
            scope_treatment: "publish structural maps with held-claim labels".to_string(),
            next_action: "hold non-publication claims".to_string(),
            next_artifact: "data/map-publication-scope-decision.csv".to_string(),
            validation_status: "pass".to_string(),
        }];

        let rows = map_publication_readiness_rows(
            &atlas_rows,
            &backlog_rows,
            &scope_rows,
            std::path::Path::new("data/map-publication-scope-decision.csv"),
            std::path::Path::new("data/optimizer-residual-blocker-backlog.csv"),
        );

        assert_eq!(rows[0].publication_blocker_count, 0);
        assert_eq!(rows[0].validation_status, "pass");
        assert_eq!(rows[0].held_claims, "upgrade");
        assert!(map_publication_readiness_gate_failures(&rows).is_empty());
    }

    #[test]
    fn map_publication_inventory_matches_atlas_and_readiness() {
        let atlas_rows = vec![MapAtlasRow {
            map_id: "national-tier".to_string(),
            path: "maps/all-tiers.png".to_string(),
            map_type: "national-tier".to_string(),
            render_command: "route map all --output maps/all-tiers.png".to_string(),
            expected_width: 2400,
            expected_height: 1350,
            min_bytes: 1,
            tier_role: "T1/T2/T3/T4 national tier overview".to_string(),
            game_use: "publication atlas".to_string(),
        }];
        let readiness_rows = vec![MapPublicationReadinessRow {
            readiness_id: "MAPPUB-READY-T1T4-STRUCTURAL".to_string(),
            map_surface: "T1-T4 structural maps".to_string(),
            map_count: 1,
            map_types: "national-tier".to_string(),
            render_gate_status: "pass".to_string(),
            scope_decision_status: "pass".to_string(),
            publication_blocker_count: 0,
            publication_blocker_families: String::new(),
            held_claims: "evidence;sla;transit;upgrade".to_string(),
            held_claim_family_count: 3,
            budget_debt_count: 9,
            scope_decision_artifact: "data/map-publication-scope-decision.csv".to_string(),
            backlog_artifact: "data/optimizer-residual-blocker-backlog.csv".to_string(),
            readiness_decision: "publish-structural-t1-t4-maps-with-held-claim-labels".to_string(),
            next_artifact: "docs/map-publication-scope.md".to_string(),
            validation_status: "pass".to_string(),
        }];
        let inventory_rows = vec![MapPublicationInventoryRow {
            map_id: "national-tier".to_string(),
            map_path: "maps/all-tiers.png".to_string(),
            map_type: "national-tier".to_string(),
            publication_status: "publication-ready-held-claims".to_string(),
            render_gate_status: "pass".to_string(),
            readiness_artifact: "data/map-publication-readiness.csv".to_string(),
            held_claims: "evidence;sla;transit;upgrade".to_string(),
            required_label: "Structural T1-T4 map; evidence/SLA/transit/upgrade claims held"
                .to_string(),
            allowed_use: "public project overview and campaign atlas".to_string(),
            not_allowed_claims:
                "evidence-valid|sla-valid|transit-ready|upgrade-ready|asset-condition-repaired"
                    .to_string(),
            next_artifact: "docs/map-publication-scope.md".to_string(),
            validation_status: "pass".to_string(),
        }];

        assert!(map_publication_inventory_gate_failures(
            &inventory_rows,
            &atlas_rows,
            &readiness_rows
        )
        .is_empty());
    }

    #[test]
    fn pressure_scenarios_require_bounded_l2_contracts() {
        let csv = "\
scenario_id,scenario_name,adversity_class,standards_tested,current_status,existing_artifact,blocking_gap,next_evidence_step
S-L2-DES-MOINES,des-moines-interchange,T1/T1 closure,T1-DIAMOND-K; T1-FLYOVER,Heuristic,scenario.toml,gap,next
BAD,unnamed,,T1-DIAMOND-K,unknown,,gap,
";

        let rows = parse_pressure_scenarios(csv.as_bytes()).expect("parse pressure scenarios");

        assert_eq!(rows.len(), 2);
        assert!(pressure_scenario_has_bounded_contract(&rows[0]));
        assert!(!pressure_scenario_has_bounded_contract(&rows[1]));
        let failures = pressure_scenario_gate_failures(&rows);
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].scenario_id, "BAD");
    }

    #[test]
    fn pressure_scenarios_cover_required_adversity_classes() {
        let csv = "\
scenario_id,scenario_name,adversity_class,standards_tested,current_status,existing_artifact,blocking_gap,next_evidence_step
S-L2-DES-MOINES,des-moines-interchange,T1/T1 closure,T1-DIAMOND-K,Heuristic,scenario.toml,gap,next
S-L2-DONNER,donner-closure,corridor segment weather closure,T1-SPURS,Heuristic,scenario.toml,gap,next
S-L2-HOUSTON,houston-surge,hurricane/flood disruption and port surge,T1-RECOVERY,Heuristic,scenario.toml,gap,next
S-L2-ATLANTA,atlanta-peak,urban peak and managed-lane stress,T1-OPS-PTI,Heuristic,scenario.toml,gap,next
S-L2-RELAY-HUB,relay-hub-outage,relay hub outage,T1-TRANSIT-HUB,Planned,route sla-matrix,gap,next
S-L2-EV-REST,ev-rest-area-outage,EV/rest-area outage,T1-EV-TRUCK,Planned,route ev-analysis,gap,next
";

        let rows = parse_pressure_scenarios(csv.as_bytes()).expect("parse pressure scenarios");
        assert!(pressure_scenario_missing_required_adversity(&rows).is_empty());

        let missing = pressure_scenario_missing_required_adversity(&rows[..5]);
        assert_eq!(missing, vec!["EV/rest-area outage"]);
    }

    #[test]
    fn pressure_scenario_readiness_gate_rejects_planned_rows() {
        let csv = "\
scenario_id,scenario_name,adversity_class,standards_tested,current_status,existing_artifact,blocking_gap,next_evidence_step
S-L2-DES-MOINES,des-moines-interchange,T1/T1 closure,T1-DIAMOND-K,Heuristic,scenario.toml,gap,next
S-L2-RELAY-HUB,relay-hub-outage,relay hub outage,T1-TRANSIT-HUB,Planned,route hub-staff,gap,next
";

        let rows = parse_pressure_scenarios(csv.as_bytes()).expect("parse pressure scenarios");

        assert!(pressure_scenario_is_executable(&rows[0]));
        assert!(!pressure_scenario_is_executable(&rows[1]));
        let failures = pressure_scenario_readiness_gate_failures(&rows);
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].scenario_id, "S-L2-RELAY-HUB");
    }

    #[test]
    fn pressure_standard_coverage_focuses_high_stakes_t1_hooks() {
        let standards_csv = "\
standard_id,tier,standard_family,standard,outcome,mechanism,primary_stressor,acceptance_gate,evidence_level,current_artifact,blocking_gap,next_command_or_test,owner_track
T1-OPS-PTI,T1,throughput,pti,outcome,mechanism,peak,gate,Heuristic,artifact,gap,next,C.1
T1-OPS-SPEED,T1,throughput,speed,outcome,mechanism,weather,gate,Heuristic,artifact,gap,next,C.1
T1-DIAMOND-K,T1,resilience,k,outcome,mechanism,closure,gate,Heuristic,artifact,gap,next,B.4
T1-REST,T1,operations,rest,outcome,mechanism,outage,gate,Planned,artifact,gap,next,F
T3-COVERAGE,T3,access,coverage,outcome,mechanism,gap,gate,Implemented,artifact,,next,B.1
";
        let scenarios_csv = "\
scenario_id,scenario_name,adversity_class,standards_tested,current_status,existing_artifact,blocking_gap,next_evidence_step
S-L2-SLA,ny-la-sla,long-haul SLA,T1-OPS-PTI; T1-OPS-SPEED,Heuristic,route od,gap,next
S-L2-DES-MOINES,des-moines,T1/T1 closure,T1-DIAMOND-K; T1-UNKNOWN,Heuristic,scenario.toml,gap,next
";

        let standards =
            parse_standards_proof_ledger(standards_csv.as_bytes()).expect("parse standards");
        let scenarios =
            parse_pressure_scenarios(scenarios_csv.as_bytes()).expect("parse scenarios");

        assert!(pressure_standard_coverage_failures(&standards, &scenarios).is_empty());
        assert_eq!(
            pressure_scenario_unknown_standard_refs(&standards, &scenarios),
            vec!["T1-UNKNOWN".to_string()]
        );
    }

    #[test]
    fn pressure_scenarios_canonical_ledger_passes_l2_and_readiness_gates() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/pressure-test-scenarios.csv");
        let file = std::fs::File::open(path).expect("open canonical pressure scenarios");
        let rows = parse_pressure_scenarios(file).expect("parse canonical pressure scenarios");

        assert!(!rows.is_empty());
        assert!(pressure_scenario_gate_failures(&rows).is_empty());
        assert!(pressure_scenario_missing_required_adversity(&rows).is_empty());
        assert!(pressure_scenario_readiness_gate_failures(&rows).is_empty());
    }

    #[test]
    fn pressure_scenarios_cover_canonical_high_stakes_t1_standards() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let scenarios_file = std::fs::File::open(root.join("data/pressure-test-scenarios.csv"))
            .expect("open canonical pressure scenarios");
        let standards_file = std::fs::File::open(root.join("data/standards-proof-ledger.csv"))
            .expect("open canonical standards proof ledger");
        let scenarios =
            parse_pressure_scenarios(scenarios_file).expect("parse canonical pressure scenarios");
        let standards =
            parse_standards_proof_ledger(standards_file).expect("parse canonical standards");

        assert!(pressure_standard_coverage_failures(&standards, &scenarios).is_empty());
        assert!(pressure_scenario_unknown_standard_refs(&standards, &scenarios).is_empty());
    }

    #[test]
    fn throughput_proof_matrix_separates_congestion_and_resilience_contracts() {
        let csv = "\
proof_id,proof_name,binding_type,stressor,primary_metric,existing_artifact,current_status,blocking_gap,next_evidence_step
TP-CONG-I80,I-80 bottleneck,congestion_binding,peak demand,max_flow_vpd,route flow I-80,Heuristic,gap,next
TP-RES-DM,Des Moines closure,resilience_binding,T1 closure,k_connectivity; t90_hours,route diamond,Heuristic,gap,next
BAD,Missing binding,unknown,peak demand,,artifact,unknown,gap,
";

        let rows = parse_throughput_proof_matrix(csv.as_bytes()).expect("parse throughput proof");

        assert_eq!(rows.len(), 3);
        assert!(throughput_proof_has_bounded_contract(&rows[0]));
        assert!(throughput_proof_has_bounded_contract(&rows[1]));
        assert!(!throughput_proof_has_bounded_contract(&rows[2]));
        let failures = throughput_proof_gate_failures(&rows);
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].proof_id, "BAD");
    }

    #[test]
    fn throughput_proof_canonical_matrix_passes_contract_gate() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/throughput-proof-matrix.csv");
        let file = std::fs::File::open(path).expect("open canonical throughput proof matrix");
        let rows = parse_throughput_proof_matrix(file).expect("parse canonical throughput proof");

        assert!(!rows.is_empty());
        assert!(throughput_proof_gate_failures(&rows).is_empty());
    }

    #[test]
    fn t1_failure_ledger_parses_optional_empirical_fields() {
        let csv = "\
site_id,intersection,location,failure_mode,annual_probability,duration_p50_hours,duration_p95_hours,throughput_retention_current,throughput_retention_i2,reroute_time_p50_hours,reroute_time_p95_hours,source_status,confidence,current_artifact,blocking_gap,next_evidence_step
T1X-I35-I80,I-35 x I-80,Des Moines IA,closure,,,,0.962,1.000,0.9,,modeled,low,artifact,gap,next
T1X-I40-I75,I-40 x I-75,Knoxville TN,closure,,,,,,,,source_needed,unknown,artifact,gap,next
";

        let rows = parse_t1_failure_ledger(csv.as_bytes()).expect("parse T1 failure ledger");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].site_id, "T1X-I35-I80");
        assert_eq!(rows[0].throughput_retention_current, Some(0.962));
        assert_eq!(rows[0].annual_probability, None);
        assert_eq!(rows[1].source_status, "source_needed");
    }

    #[test]
    fn t1_failure_evidence_gate_requires_labeled_source_status_and_next_steps() {
        let csv = "\
site_id,intersection,location,failure_mode,annual_probability,duration_p50_hours,duration_p95_hours,throughput_retention_current,throughput_retention_i2,reroute_time_p50_hours,reroute_time_p95_hours,source_status,confidence,current_artifact,blocking_gap,next_evidence_step
T1X-I35-I80,I-35 x I-80,Des Moines IA,closure,,,,0.962,1.000,0.9,,modeled,low,artifact,gap,next
T1X-I40-I75,I-40 x I-75,Knoxville TN,closure,,,,,,,,source_needed,unknown,artifact,gap,next
T1X-BAD,I-5 x I-10,Los Angeles CA,closure,,,,,,,,maybe,unknown,artifact,gap,
";

        let rows = parse_t1_failure_ledger(csv.as_bytes()).expect("parse T1 failure ledger");

        assert!(t1_failure_row_has_evidence_contract(&rows[0]));
        assert!(t1_failure_row_has_evidence_contract(&rows[1]));
        assert!(!t1_failure_row_has_evidence_contract(&rows[2]));
        let failures = t1_failure_evidence_gate_failures(&rows);
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].site_id, "T1X-BAD");
    }

    #[test]
    fn t1_diamond_validation_gate_requires_complete_catalog_contracts() {
        let csv = "\
site_id,intersection,location,priority_band,anchor_lon,anchor_lat,analyzer_status,manual_geometry_status,alternate_capacity_status,observed_failure_status,validation_status,current_artifact,blocking_gap,next_validation_step
T1X-I35-I80,I-35 x I-80,Des Moines IA,A,-93.573,41.659,recognized,heuristic,heuristic,modeled,heuristic,artifact,gap,next
T1X-BAD,I-5 x I-10,Los Angeles CA,B,-118.230,34.050,unknown,heuristic,pending,source_needed,heuristic,artifact,gap,next
";

        let rows = parse_t1_diamond_validation(csv.as_bytes()).expect("parse validation");
        let failures = super::t1_diamond_validation_gate_failures(&rows);
        let missing = super::t1_diamond_validation_missing_sites(&rows);

        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].site_id, "T1X-BAD");
        assert!(missing.contains(&"T1X-I80-I90".to_string()));
    }

    #[test]
    fn t1_diamond_validation_canonical_catalog_passes_gate() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/t1-diamond-validation.csv");
        let file = std::fs::File::open(path).expect("open canonical validation ledger");
        let rows = parse_t1_diamond_validation(file).expect("parse validation ledger");

        assert_eq!(rows.len(), super::EXPECTED_T1_DIAMOND_SITES.len());
        assert!(super::t1_diamond_validation_gate_failures(&rows).is_empty());
        assert!(super::t1_diamond_validation_missing_sites(&rows).is_empty());
    }

    #[test]
    fn t1_diamond_validation_tasks_split_unresolved_dimensions() {
        let csv = "\
site_id,intersection,location,priority_band,anchor_lon,anchor_lat,analyzer_status,manual_geometry_status,alternate_capacity_status,observed_failure_status,validation_status,current_artifact,blocking_gap,next_validation_step
T1X-I35-I80,I-35 x I-80,Des Moines IA,A,-93.573,41.659,recognized,validated,heuristic,modeled,heuristic,artifact,gap,next
T1X-I35-I40,I-35 x I-40,Oklahoma City OK,B,-97.530,35.460,recognized,heuristic,pending,source_needed,heuristic,artifact,gap,next
";

        let rows = parse_t1_diamond_validation(csv.as_bytes()).expect("parse validation");
        let all_tasks = super::t1_diamond_validation_tasks(&rows, None, None);
        let a_tasks = super::t1_diamond_validation_tasks(&rows, Some("A"), None);

        assert_eq!(all_tasks.len(), 5);
        assert_eq!(a_tasks.len(), 2);
        assert!(a_tasks
            .iter()
            .any(|task| task.category == "alternate_capacity"));
        assert!(a_tasks
            .iter()
            .any(|task| task.category == "observed_failure"));
    }

    #[test]
    fn t1_diamond_validation_observed_failure_tasks_include_source_access() {
        let validation_csv = "\
site_id,intersection,location,priority_band,anchor_lon,anchor_lat,analyzer_status,manual_geometry_status,alternate_capacity_status,observed_failure_status,validation_status,current_artifact,blocking_gap,next_validation_step
T1X-I10-I35,I-10 x I-35,San Antonio TX,A,-98.500,29.430,recognized,validated,validated,source_needed,heuristic,artifact,gap,next
";
        let source_csv = "\
site_id,source_name,source_url,source_kind,access_health,ingestion_status,history_status,last_checked,blocking_gap,next_step
T1X-I10-I35,DriveTexas API,https://example.invalid,live_event_feed,requires_key,not_started,unknown,2026-05-09,gap,Obtain DriveTexas API credentials
";

        let validation_rows =
            parse_t1_diamond_validation(validation_csv.as_bytes()).expect("parse validation");
        let source_rows = parse_t1_source_health(source_csv.as_bytes()).expect("parse source");
        let tasks =
            super::t1_diamond_validation_tasks(&validation_rows, Some("A"), Some(&source_rows));

        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].category, "observed_failure");
        assert!(tasks[0]
            .source_action
            .as_ref()
            .expect("source action")
            .contains("DriveTexas API [requires_key / unknown]"));
    }

    #[test]
    fn t1_failure_source_plan_parses_source_targets() {
        let csv = "\
site_id,intersection,location,primary_state_sources,national_sources,fields_to_populate,access_status,source_url,notes
T1X-I35-I80,I-35 x I-80,Des Moines IA,Iowa DOT 511,NPMRDS,annual_probability,identified,https://example.invalid,notes
T1X-I35-I40,I-35 x I-40,Oklahoma City OK,Oklahoma 511,NPMRDS,duration,lookup_needed,,notes
";

        let rows = parse_t1_failure_source_plan(csv.as_bytes()).expect("parse source plan");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].access_status, "identified");
        assert_eq!(rows[1].source_url, "");
    }

    #[test]
    fn t1_source_health_parses_and_flags_blockers() {
        let csv = "\
site_id,source_name,source_url,source_kind,access_health,ingestion_status,history_status,last_checked,blocking_gap,next_step
T1X-I35-I80,Iowa DOT 511,https://example.invalid,live_event_feed,live,implemented,snapshot_only,2026-05-09,gap,next
T1X-I40-I75,TDOT SmartWay,https://example.invalid,live_event_feed,blocked_query,scaffolded,unknown,2026-05-09,gap,next
";

        let rows = parse_t1_source_health(csv.as_bytes()).expect("parse source health");
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].access_health, "live");
        assert!(!super::t1_source_health_is_blocked(&rows[0]));
        assert!(super::t1_source_health_is_blocked(&rows[1]));
        assert_eq!(super::t1_source_health_blockers(&rows).len(), 1);
    }

    #[test]
    fn t1_access_docket_groups_blockers_by_action_type() {
        let csv = "\
site_id,source_name,source_url,source_kind,access_health,ingestion_status,history_status,last_checked,blocking_gap,next_step
T1X-I35-I80,Iowa DOT 511,https://example.invalid,live_event_feed,live,implemented,snapshot_only,2026-05-09,gap,next
T1X-I40-I75,TDOT SmartWay,https://example.invalid,live_event_feed,blocked_query,scaffolded,unknown,2026-05-09,gap,next
ALL,FHWA NPMRDS,https://example.invalid,travel_time_reliability,requires_access,not_started,historical_available,2026-05-09,gap,next
";

        let rows = parse_t1_source_health(csv.as_bytes()).expect("parse source health");
        let items = rows
            .iter()
            .filter(|row| super::t1_source_health_is_blocked(row))
            .map(super::t1_access_docket_item)
            .collect::<Vec<_>>();

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].category, "endpoint_tuning");
        assert_eq!(items[0].priority, "high");
        assert_eq!(items[1].category, "access_request");
        assert_eq!(items[1].priority, "critical");
    }

    #[test]
    fn t1_snapshot_plan_requires_executable_polling_contract() {
        let csv = "\
site_id,intersection,priority_band,source_name,source_health,cadence,fetch_command,import_command,accumulate_command,raw_output,normalized_output,accumulated_output,blocking_gap,next_step
T1X-I35-I80,I-35 x I-80,A,Iowa DOT 511,live/implemented/snapshot_only,daily,route t1-fetch-iowa511,route t1-import-iowa511,route t1-accumulate-events,data/cache/iowa.json,data/cache/iowa.csv,data/t1-failure-events.csv,gap,next
T1X-BAD,I-35 x I-40,B,Bad Source,live/implemented/snapshot_only,eventually,fetch,import,accumulate,raw.txt,norm.txt,out.txt,gap,next
";

        let rows = parse_t1_snapshot_plan(csv.as_bytes()).expect("parse snapshot plan");
        let failures = super::t1_snapshot_plan_gate_failures(&rows);

        assert_eq!(rows.len(), 2);
        assert!(super::t1_snapshot_plan_row_has_contract(&rows[0]));
        assert_eq!(failures.len(), 1);
        assert_eq!(failures[0].site_id, "T1X-BAD");
    }

    #[test]
    fn t1_snapshot_plan_canonical_plan_passes_gate() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/t1-snapshot-plan.csv");
        let file = std::fs::File::open(path).expect("open canonical snapshot plan");
        let rows = parse_t1_snapshot_plan(file).expect("parse snapshot plan");

        assert_eq!(rows.len(), 2);
        assert!(super::t1_snapshot_plan_gate_failures(&rows).is_empty());
    }

    #[test]
    fn t1_snapshot_plan_priority_filter_drives_script_scope() {
        let csv = "\
site_id,intersection,priority_band,source_name,source_health,cadence,fetch_command,import_command,accumulate_command,raw_output,normalized_output,accumulated_output,blocking_gap,next_step
T1X-I35-I80,I-35 x I-80,A,Iowa DOT 511,live/implemented/snapshot_only,daily,route t1-fetch-iowa511,route t1-import-iowa511,route t1-accumulate-events,data/cache/iowa.json,data/cache/iowa.csv,data/t1-failure-events.csv,gap,next
T1X-I5-I10,I-5 x I-10,B,Caltrans PeMS,live/implemented/snapshot_only,daily,route t1-fetch-caltrans,route t1-import-caltrans,route t1-accumulate-events,data/cache/caltrans.json,data/cache/caltrans.csv,data/t1-failure-events.csv,gap,next
";

        let rows = parse_t1_snapshot_plan(csv.as_bytes()).expect("parse snapshot plan");
        let filtered = super::filtered_t1_snapshot_rows(&rows, Some("A"));

        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].site_id, "T1X-I35-I80");
    }

    #[test]
    fn t1_evidence_windows_require_window_metadata_and_snapshot_guard() {
        let csv = "\
window_id,site_id,source_name,evidence_mode,capture_started_at,capture_ended_at,observation_start,observation_end,raw_artifact,normalized_artifact,event_count,freight_relevant_count,promotion_eligible,blocking_gap,next_step,review_artifact
W1,T1X-I35-I80,Iowa DOT 511 ArcGIS,snapshot_only,2026-05-10,2026-05-10,2026-01-15,2026-05-04,data/cache/iowa511-events.json,data/t1-failure-events.csv,25,25,false,Snapshot only,next,docs/reviews/milepost-8-t1-failure-evidence-review.md
W2,T1X-I35-I80,Iowa DOT 511 ArcGIS,snapshot_only,2026-05-10,2026-05-10,2026-01-15,2026-05-04,data/cache/iowa511-events.json,data/t1-failure-events.csv,25,25,true,Snapshot only,next,docs/reviews/milepost-8-t1-failure-evidence-review.md
";
        let rows = parse_t1_evidence_windows(csv.as_bytes()).expect("parse evidence windows");

        assert!(super::t1_evidence_window_has_contract(&rows[0]));
        assert!(super::t1_evidence_window_gate_failures(&rows)
            .iter()
            .any(|failure| failure.contains("snapshot-only evidence as promotion eligible")));
    }

    #[test]
    fn t1_evidence_windows_canonical_ledger_passes_gate() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/t1-evidence-windows.csv");
        let file = std::fs::File::open(path).expect("open canonical evidence windows");
        let rows = parse_t1_evidence_windows(file).expect("parse canonical evidence windows");

        assert!(super::t1_evidence_window_gate_failures(&rows).is_empty());
        assert!(rows.iter().any(|row| row.evidence_mode == "snapshot_only"));
    }

    #[test]
    fn t1_failure_events_summarize_rates_and_durations() {
        let csv = "\
site_id,event_id,source,source_event_id,observation_year,start_time,end_time,duration_hours,event_type,full_closure,lanes_closed,freight_relevant,confidence,notes
T1X-I35-I80,e1,Iowa 511,100,2023,2023-01-01T00:00:00Z,2023-01-01T02:00:00Z,2.0,incident,true,2,true,medium,first
T1X-I35-I80,e2,Iowa 511,101,2023,2023-03-01T00:00:00Z,2023-03-01T04:00:00Z,4.0,work_zone,false,1,true,medium,second
T1X-I35-I80,e3,Iowa 511,102,2024,2024-06-01T00:00:00Z,2024-06-01T10:00:00Z,10.0,incident,true,3,true,high,third
T1X-I35-I80,e4,Iowa 511,103,2024,2024-07-01T00:00:00Z,2024-07-01T08:00:00Z,8.0,incident,true,3,false,low,non-freight
";

        let rows = parse_t1_failure_events(csv.as_bytes()).expect("parse event rows");
        let summaries = summarize_t1_failure_events(&rows);

        assert_eq!(rows.len(), 4);
        assert_eq!(summaries.len(), 1);
        let summary = &summaries[0];
        assert_eq!(summary.site_id, "T1X-I35-I80");
        assert_eq!(summary.observed_years, 2);
        assert_eq!(summary.event_count, 3);
        assert_eq!(summary.annual_rate, 1.5);
        assert!((summary.annual_probability - 0.77686984).abs() < 1e-6);
        assert_eq!(summary.duration_p50_hours, Some(4.0));
        assert_eq!(summary.duration_p95_hours, Some(10.0));
        assert_eq!(summary.confidence, "medium");
    }

    #[test]
    fn t1_failure_event_observation_gate_requires_normalized_evidence_fields() {
        let csv = "\
site_id,event_id,source,source_event_id,observation_year,start_time,end_time,duration_hours,event_type,full_closure,lanes_closed,freight_relevant,confidence,notes
T1X-I35-I80,e1,Iowa 511,100,2023,2023-01-01T00:00:00Z,2023-01-01T02:00:00Z,,incident,true,2,true,medium,first
T1X-I35-I80,,Iowa 511,,2023,,,,incident,true,2,false,unknown,missing id and timing
";

        let rows = parse_t1_failure_events(csv.as_bytes()).expect("parse event rows");

        assert!(t1_failure_event_has_observation_contract(&rows[0]));
        assert!(!t1_failure_event_has_observation_contract(&rows[1]));
        let failures = t1_failure_event_observation_gate_failures(&rows);
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("<missing-event-id>"));
        assert_eq!(
            t1_failure_event_observation_gate_failures(&[]),
            vec!["event ledger has no observation rows".to_string()]
        );
    }

    #[test]
    fn t1_failure_events_canonical_ledger_passes_observation_gate() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../data/t1-failure-events.csv");
        let file = std::fs::File::open(path).expect("open canonical T1 failure events");
        let rows = parse_t1_failure_events(file).expect("parse canonical T1 failure events");

        assert!(!rows.is_empty());
        assert!(t1_failure_event_observation_gate_failures(&rows).is_empty());
        assert!(!summarize_t1_failure_events(&rows).is_empty());
    }

    #[test]
    fn t1_failure_events_apply_empirical_fields_to_ledger() {
        let ledger_csv = "\
site_id,intersection,location,failure_mode,annual_probability,duration_p50_hours,duration_p95_hours,throughput_retention_current,throughput_retention_i2,reroute_time_p50_hours,reroute_time_p95_hours,source_status,confidence,current_artifact,blocking_gap,next_evidence_step
T1X-I35-I80,I-35 x I-80,Des Moines IA,closure,,,,0.962,1.000,0.9,,modeled,low,artifact,gap,next
T1X-I35-I40,I-35 x I-40,Oklahoma City OK,closure,,,,,,,,source_needed,unknown,artifact,gap,next
";
        let events_csv = "\
site_id,event_id,source,source_event_id,observation_year,start_time,end_time,duration_hours,event_type,full_closure,lanes_closed,freight_relevant,confidence,notes
T1X-I35-I80,e1,Iowa 511,100,2023,2023-01-01T00:00:00Z,2023-01-01T02:00:00Z,2.0,incident,true,2,true,medium,first
T1X-I35-I80,e2,Iowa 511,101,2024,2024-01-01T00:00:00Z,2024-01-01T06:00:00Z,6.0,incident,true,2,true,medium,second
";

        let ledger_rows = parse_t1_failure_ledger(ledger_csv.as_bytes()).expect("parse ledger");
        let event_rows = parse_t1_failure_events(events_csv.as_bytes()).expect("parse events");
        let updated = super::apply_t1_failure_events_to_ledger(
            &ledger_rows,
            &event_rows,
            std::path::Path::new("data/t1-failure-events.csv"),
        );

        assert_eq!(updated[0].source_status, "empirical");
        assert_eq!(updated[0].duration_p50_hours, Some(6.0));
        assert_eq!(updated[0].duration_p95_hours, Some(6.0));
        assert_eq!(updated[0].throughput_retention_current, Some(0.962));
        assert!(updated[0]
            .current_artifact
            .contains("data/t1-failure-events.csv"));
        assert_eq!(updated[1].source_status, "source_needed");
    }

    #[test]
    fn t1_failure_events_merge_dedupes_repeated_snapshots() {
        let existing_csv = "\
site_id,event_id,source,source_event_id,observation_year,start_time,end_time,duration_hours,event_type,full_closure,lanes_closed,freight_relevant,confidence,notes
T1X-I35-I80,IOWA511-1,Iowa DOT 511 ArcGIS,1,2026,2026-05-01 08:00 AM,2026-05-01 10:00 AM,2.0,work_zone,false,,true,medium,first
";
        let incoming_csv = "\
site_id,event_id,source,source_event_id,observation_year,start_time,end_time,duration_hours,event_type,full_closure,lanes_closed,freight_relevant,confidence,notes
T1X-I35-I80,IOWA511-1,Iowa DOT 511 ArcGIS,1,2026,2026-05-01 08:00 AM,2026-05-01 10:00 AM,2.0,work_zone,false,,true,medium,duplicate
T1X-I35-I80,IOWA511-2,Iowa DOT 511 ArcGIS,2,2026,2026-05-02 08:00 AM,2026-05-02 11:00 AM,3.0,closure,true,,true,medium,second
";

        let existing = parse_t1_failure_events(existing_csv.as_bytes()).expect("parse existing");
        let incoming = parse_t1_failure_events(incoming_csv.as_bytes()).expect("parse incoming");
        let merged = super::merge_t1_failure_events(&existing, &incoming);

        assert_eq!(merged.len(), 2);
        assert_eq!(merged[0].event_id, "IOWA511-1");
        assert_eq!(merged[0].notes, "first");
        assert_eq!(merged[1].event_id, "IOWA511-2");
    }

    #[test]
    fn iowa511_import_filters_radius_and_normalizes_events() {
        let json = r#"{
  "features": [
    {
      "attributes": {
        "ID": "IADOT-1",
        "Route": "I-80 WB",
        "StartTime": "08:00 AM",
        "EndTime": "10:30 AM",
        "IssueDate": "20260330",
        "IssueTime": "170433",
        "headline": "I-80 WB: Crash, left lane closed",
        "cause": "due to crash.",
        "Restrict_": "Lane closed",
        "Desc0": "near Des Moines"
      },
      "geometry": { "x": -93.80, "y": 41.66 }
    },
    {
      "attributes": {
        "ID": "IADOT-2",
        "Route": "US 218",
        "StartTime": "08:00 AM",
        "EndTime": "10:30 AM",
        "IssueDate": "20260330",
        "IssueTime": "170433",
        "headline": "US 218: Road Construction",
        "cause": "due to road construction.",
        "Restrict_": "",
        "Desc0": "not a T1 route"
      },
      "geometry": { "x": -93.80, "y": 41.66 }
    },
    {
      "attributes": {
        "ID": "IADOT-3",
        "Route": "I-80 WB",
        "StartTime": "08:00 AM",
        "EndTime": "10:30 AM",
        "IssueDate": "20260330",
        "IssueTime": "170433",
        "headline": "I-80 WB: Entrance Ramp Closed",
        "cause": "due to road construction.",
        "Restrict_": "",
        "Desc0": "Council Bluffs"
      },
      "geometry": { "x": -95.85, "y": 41.26 }
    }
  ]
}"#;

        let rows = parse_iowa511_events(json, "T1X-I35-I80", 41.658, -93.800, 30.0)
            .expect("parse Iowa 511 fixture");

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].site_id, "T1X-I35-I80");
        assert_eq!(rows[0].source_event_id, "IADOT-1");
        assert_eq!(rows[0].observation_year, 2026);
        assert_eq!(rows[0].duration_hours, Some(2.5));
        assert_eq!(rows[0].event_type, "crash");
        assert!(!rows[0].full_closure);
    }

    #[test]
    fn tdot_smartway_import_filters_radius_and_normalizes_events() {
        let json = r#"{
  "features": [
    {
      "attributes": {
        "ID": "TDOT-1",
        "START_DATE": 1777636800000,
        "END_DATE": 1777651200000,
        "CD_ROAD_NAMES": "I-40 / I-75",
        "CD_DIRECTION": "Eastbound",
        "EVENT_TYPE": "Roadway Closure",
        "EVENT_SUBTYPE": "Construction",
        "DESCRIPTION": "I-40/I-75 lane closure in Knox County",
        "HAS_CLOSURE": 1,
        "MIDPOINT_LATITUDE_DD": 35.90,
        "MIDPOINT_LONGITUDE_DD": -84.16,
        "COUNTY_NAME": "Knox"
      }
    },
    {
      "attributes": {
        "ID": "TDOT-2",
        "START_DATE": 1777636800000,
        "END_DATE": 1777651200000,
        "CD_ROAD_NAMES": "SR-1",
        "CD_DIRECTION": "Eastbound",
        "EVENT_TYPE": "Roadway Closure",
        "EVENT_SUBTYPE": "Construction",
        "DESCRIPTION": "not a T1 route",
        "HAS_CLOSURE": 1,
        "MIDPOINT_LATITUDE_DD": 35.90,
        "MIDPOINT_LONGITUDE_DD": -84.16,
        "COUNTY_NAME": "Knox"
      }
    },
    {
      "attributes": {
        "ID": "TDOT-3",
        "START_DATE": 1777636800000,
        "END_DATE": 1777651200000,
        "CD_ROAD_NAMES": "I-75",
        "CD_DIRECTION": "Southbound",
        "EVENT_TYPE": "Roadway Closure",
        "EVENT_SUBTYPE": "Construction",
        "DESCRIPTION": "near Chattanooga",
        "HAS_CLOSURE": 1,
        "MIDPOINT_LATITUDE_DD": 35.05,
        "MIDPOINT_LONGITUDE_DD": -85.20,
        "COUNTY_NAME": "Hamilton"
      }
    }
  ]
}"#;

        let rows =
            parse_tdot_smartway_events(json, "T1X-I40-I75", 35.90, -84.16, 35.0).expect("parse");

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].site_id, "T1X-I40-I75");
        assert_eq!(rows[0].source_event_id, "TDOT-1");
        assert_eq!(rows[0].observation_year, 2026);
        assert_eq!(rows[0].duration_hours, Some(4.0));
        assert_eq!(rows[0].event_type, "work_zone");
        assert!(rows[0].full_closure);
    }

    #[test]
    fn mdot_midrive_import_filters_radius_and_normalizes_events() {
        let json = r#"[
  {
    "latitude": 42.31,
    "longitude": -83.08,
    "id": 1092974,
    "title": "Crash on NB  I-75",
    "message": "<div><strong>Location: </strong>NB I-75 at I-94</div><div><strong>Lanes Blocked: </strong>Left Lane</div><div><strong>Event Type: </strong> Crash</div><div><strong>County: </strong>Wayne</div><div><strong>Reported:</strong> 5:14 PM</div>"
  },
  {
    "latitude": 42.31,
    "longitude": -83.08,
    "id": 1092975,
    "title": "Crash on US-23",
    "message": "<div><strong>Event Type: </strong> Crash</div>"
  },
  {
    "latitude": 43.60,
    "longitude": -84.20,
    "id": 1092976,
    "title": "Crash on SB I-75",
    "message": "<div><strong>Event Type: </strong> Crash</div>"
  }
]"#;

        let rows = parse_mdot_midrive_events(json, "T1X-I75-I90", 42.31, -83.07, 60.0, 2026)
            .expect("parse MDOT Mi Drive fixture");

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].site_id, "T1X-I75-I90");
        assert_eq!(rows[0].source_event_id, "1092974");
        assert_eq!(rows[0].observation_year, 2026);
        assert_eq!(rows[0].start_time, "5:14 PM");
        assert_eq!(rows[0].event_type, "crash");
        assert_eq!(rows[0].lanes_closed, Some(1));
        assert_eq!(rows[0].confidence, "low");
    }

    #[test]
    fn indot_trafficwise_import_filters_events_and_normalizes_rows() {
        let json = r#"{
  "data": {
    "mapFeaturesQuery": {
      "mapFeatures": [
        {
          "title": "I-80 westbound: Entrance ramp closed.",
          "tooltip": "I-80 westbound: Entrance ramp closed, because of roadwork.",
          "uri": "event/CARSx-333174",
          "__typename": "Event",
          "_eventReport": {
            "beginTime": {"time": 1778065200000},
            "endTime": {"time": 1778079600000}
          }
        },
        {
          "title": "US 30 in both directions: Paving operations.",
          "tooltip": "US 30 in both directions: Paving operations, left lane closed.",
          "uri": "event/incars-178325",
          "__typename": "Event"
        },
        {
          "title": "Show six events",
          "tooltip": "",
          "uri": "cluster/-87371644160212",
          "__typename": "Cluster"
        }
      ],
      "error": null
    }
  }
}"#;

        let rows = parse_indot_trafficwise_events(json, "T1X-I80-I90", 2026).expect("parse INDOT");

        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].site_id, "T1X-I80-I90");
        assert_eq!(rows[0].source_event_id, "CARSx-333174");
        assert_eq!(rows[0].observation_year, 2026);
        assert_eq!(rows[0].start_time, "2026-05-06");
        assert_eq!(rows[0].end_time, "2026-05-06");
        assert_eq!(rows[0].duration_hours, Some(4.0));
        assert_eq!(rows[0].event_type, "work_zone");
        assert!(rows[0].full_closure);
        assert_eq!(rows[0].confidence, "medium");
    }

    #[test]
    fn indot_trafficwise_import_skips_untimed_rows() {
        let json = r#"{
  "data": {
    "mapFeaturesQuery": {
      "mapFeatures": [
        {
          "title": "I-80 westbound: Entrance ramp closed.",
          "tooltip": "I-80 westbound: Entrance ramp closed, because of roadwork.",
          "uri": "event/CARSx-333174",
          "__typename": "Event"
        }
      ],
      "error": null
    }
  }
}"#;

        let rows = parse_indot_trafficwise_events(json, "T1X-I80-I90", 2026).expect("parse INDOT");

        assert!(rows.is_empty());
    }

    #[test]
    fn scenario_edge_candidates_return_nearest_stable_edge_ids() {
        let mut graph = HighwayGraph::new();
        let a = graph.graph.add_node(HighwayNode {
            id: 1,
            coord: coord! { x: -96.0, y: 41.0 },
            is_interchange: false,
        });
        let b = graph.graph.add_node(HighwayNode {
            id: 2,
            coord: coord! { x: -95.9, y: 41.0 },
            is_interchange: false,
        });
        let c = graph.graph.add_node(HighwayNode {
            id: 3,
            coord: coord! { x: -90.0, y: 41.0 },
            is_interchange: false,
        });
        let d = graph.graph.add_node(HighwayNode {
            id: 4,
            coord: coord! { x: -89.9, y: 41.0 },
            is_interchange: false,
        });
        let near = graph.graph.add_edge(
            a,
            b,
            HighwayEdge {
                id: 101,
                route_id: "I80".to_string(),
                state: "NE".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: -96.0, y: 41.0 },
                    coord! { x: -95.9, y: 41.0 },
                ]),
                length_miles: 6.0,
                lane_count: Some(4),
                aadt: Some(50_000),
                pct_truck: None,
                iri: None,
                tti: None,
                pti: None,
                speed_limit: None,
            },
        );
        let far = graph.graph.add_edge(
            c,
            d,
            HighwayEdge {
                id: 202,
                route_id: "I80".to_string(),
                state: "IA".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: -90.0, y: 41.0 },
                    coord! { x: -89.9, y: 41.0 },
                ]),
                length_miles: 6.0,
                lane_count: Some(4),
                aadt: Some(40_000),
                pct_truck: None,
                iri: None,
                tti: None,
                pti: None,
                speed_limit: None,
            },
        );
        graph.route_index.insert("I80".to_string(), vec![far, near]);

        let candidates = scenario_edge_candidates(&graph, "I80", 41.0, -95.95, 20.0, 5);

        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].edge_id, 101);
        assert_eq!(candidates[0].state, "NE");
    }

    #[test]
    fn normalise_designation_strips_cache_separators() {
        assert_eq!(super::normalise_designation("I_66"), "I66");
        assert_eq!(super::normalise_designation("us-287"), "US287");
    }

    #[test]
    fn a2_freight_proxy_uses_mean_aadt_when_p90_missing() {
        let mut attrs = CorridorAttributes {
            p90_aadt: None,
            mean_aadt: Some(20_000.0),
            mean_pct_truck: Some(0.10),
            ..Default::default()
        };

        super::join_a2_freight_proxy(&mut attrs, 100.0);

        let freight_b = attrs.annual_freight_value_b.expect("A2 proxy should fill");
        assert!((freight_b - 11.68).abs() < 0.001);
        assert!(attrs.freight_value_is_hpms_proxy);
    }

    #[test]
    fn atlas_candidates_include_us_highway_promotions_but_not_state_routes() {
        let mut graph = HighwayGraph::new();
        let a = graph.graph.add_node(HighwayNode {
            id: 1,
            coord: coord! { x: 0.0, y: 0.0 },
            is_interchange: false,
        });
        let b = graph.graph.add_node(HighwayNode {
            id: 2,
            coord: coord! { x: 1.0, y: 1.0 },
            is_interchange: false,
        });
        graph.route_index = ["I80", "US30", "SR99"]
            .into_iter()
            .map(|id| {
                let edge = graph.graph.add_edge(
                    a,
                    b,
                    HighwayEdge {
                        id: 1,
                        route_id: id.to_string(),
                        state: "TS".to_string(),
                        road_class: route_data::RoadClass::Interstate,
                        geometry: LineString::from(vec![
                            coord! { x: 0.0, y: 0.0 },
                            coord! { x: 1.0, y: 1.0 },
                        ]),
                        length_miles: 1.0,
                        lane_count: None,
                        aadt: None,
                        pct_truck: None,
                        iri: None,
                        tti: None,
                        pti: None,
                        speed_limit: None,
                    },
                );
                (id.to_string(), vec![edge])
            })
            .collect::<HashMap<_, _>>();

        assert_eq!(atlas_candidate_ids(&graph), vec!["I80", "US30"]);
    }

    #[test]
    fn fema_d1_join_uses_route_edge_boxes_not_whole_corridor_box() {
        let mut graph = HighwayGraph::new();
        let a = graph.graph.add_node(HighwayNode {
            id: 1,
            coord: coord! { x: 0.0, y: 0.0 },
            is_interchange: false,
        });
        let b = graph.graph.add_node(HighwayNode {
            id: 2,
            coord: coord! { x: 1.0, y: 0.0 },
            is_interchange: false,
        });
        let c = graph.graph.add_node(HighwayNode {
            id: 3,
            coord: coord! { x: 10.0, y: 0.0 },
            is_interchange: false,
        });
        let d = graph.graph.add_node(HighwayNode {
            id: 4,
            coord: coord! { x: 11.0, y: 0.0 },
            is_interchange: false,
        });
        let edge_a = graph.graph.add_edge(
            a,
            b,
            HighwayEdge {
                id: 1,
                route_id: "I1".to_string(),
                state: "TS".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: 0.0, y: 0.0 },
                    coord! { x: 1.0, y: 0.0 },
                ]),
                length_miles: 1.0,
                lane_count: None,
                aadt: None,
                pct_truck: None,
                iri: None,
                tti: None,
                pti: None,
                speed_limit: None,
            },
        );
        let edge_b = graph.graph.add_edge(
            c,
            d,
            HighwayEdge {
                id: 2,
                route_id: "I1".to_string(),
                state: "TS".to_string(),
                road_class: route_data::RoadClass::Interstate,
                geometry: LineString::from(vec![
                    coord! { x: 10.0, y: 0.0 },
                    coord! { x: 11.0, y: 0.0 },
                ]),
                length_miles: 1.0,
                lane_count: None,
                aadt: None,
                pct_truck: None,
                iri: None,
                tti: None,
                pti: None,
                speed_limit: None,
            },
        );
        graph
            .route_index
            .insert("I1".to_string(), vec![edge_a, edge_b]);
        graph
            .route_index
            .insert("I80".to_string(), vec![edge_a, edge_b]);

        let tiles = vec![
            FemaTile {
                name: "fixture-a".to_string(),
                xmin: 5.0,
                ymin: -0.5,
                xmax: 6.0,
                ymax: 0.5,
                sfha_count: 100,
                status: "ok".to_string(),
            },
            FemaTile {
                name: "fixture-b".to_string(),
                xmin: 0.25,
                ymin: -0.5,
                xmax: 0.75,
                ymax: 0.5,
                sfha_count: 7,
                status: "ok".to_string(),
            },
        ];
        let mut attrs = CorridorAttributes::default();

        join_fema_d1_to_corridor(&graph, "I1", &mut attrs, &tiles);

        assert_eq!(attrs.fema_sfha_miles, Some(2.1));
        assert_eq!(attrs.max_consecutive_sfha_miles, Some(1.47));

        let mut i80_attrs = CorridorAttributes::default();
        join_fema_d1_to_corridor(&graph, "I80", &mut i80_attrs, &tiles);
        assert_eq!(i80_attrs.fema_sfha_miles, None);
    }

    #[test]
    fn hazard_zone_loader_skips_comment_preamble_and_merges_segments() {
        let zones = super::load_hazard_zones();
        let i5 = zones
            .get("I5")
            .expect("I-5 hazard rows should load from commented CSV");
        let i80 = zones
            .get("I80")
            .expect("I-80 hazard row should load from commented CSV");

        assert_eq!(zones.len(), 12);
        assert_eq!(i5.wildfire, 8.5);
        assert_eq!(i5.seismic, 8.5);
        assert_eq!(i80.wildfire, 3.5);
        assert_eq!(i80.tornado, 0.5);
    }

    #[test]
    fn fema_tile_parser_requires_explicit_ok_status() {
        let legacy = "tile,xmin,ymin,xmax,ymax,sfha_count\nlegacy,0,0,1,1,4\n";
        let failed = "tile,xmin,ymin,xmax,ymax,sfha_count,status\nfailed,0,0,1,1,4,error\n";
        let healthy = "tile,xmin,ymin,xmax,ymax,sfha_count,status\nhealthy,0,0,1,1,4,ok\n";

        assert!(super::parse_fema_tiles(legacy.as_bytes()).is_empty());
        assert!(super::parse_fema_tiles(failed.as_bytes()).is_empty());
        assert_eq!(super::parse_fema_tiles(healthy.as_bytes()).len(), 1);
    }

    fn score_row(route: &str, score: f64, tier: &'static str) -> ScoreAllRow {
        ScoreAllRow {
            route: route.to_string(),
            score,
            tier,
            rubric_version: "test".to_string(),
            estimated: false,
            confidence: 0.9,
            score_confidence: 0.8,
            dimensions: [0.0; 16],
            dimension_confidences: [0.0; 16],
        }
    }
