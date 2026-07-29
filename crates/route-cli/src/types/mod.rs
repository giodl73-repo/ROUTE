//! Shared CLI data types peeled from `main`.
#![allow(dead_code)]

// --- struct ConfidenceRisk ---
pub(crate) struct ConfidenceRisk {
    pub(crate) route: String,
    pub(crate) score: f64,
    pub(crate) tier: &'static str,
    pub(crate) mean_confidence: f32,
    pub(crate) score_confidence: f32,
    pub(crate) risk_dimensions: String,
}

// --- struct ScoreAllRow ---
pub(crate) struct ScoreAllRow {
    pub(crate) route: String,
    pub(crate) score: f64,
    pub(crate) tier: &'static str,
    pub(crate) rubric_version: String,
    pub(crate) estimated: bool,
    pub(crate) confidence: f32,
    pub(crate) score_confidence: f32,
    pub(crate) dimensions: [f64; 16],
    pub(crate) dimension_confidences: [f32; 16],
}

// --- struct EvProfileRecord ---
/// Local deserialization record for ev-profiles.toml (CLI-only; uses String for name).
#[derive(serde::Deserialize)]
pub(crate) struct EvProfileRecord {
    pub(crate) name: String,
    pub(crate) highway_range_miles: f64,
    pub(crate) charge_rate_kw: f64,
    pub(crate) battery_kwh: f64,
    pub(crate) kwh_per_mile: f64,
}

// --- struct EvProfilesFile ---
#[derive(serde::Deserialize)]
pub(crate) struct EvProfilesFile {
    pub(crate) vehicles: Vec<EvProfileRecord>,
}

// --- struct StopSlaRow ---
#[derive(Debug, serde::Deserialize)]
pub(crate) struct StopSlaRow {
    pub(crate) origin_id: String,
    pub(crate) origin_label: String,
    pub(crate) dest_id: String,
    pub(crate) dest_label: String,
    pub(crate) network_miles: f64,
    pub(crate) max_stop_gap_miles: f64,
    pub(crate) stop_gap_status: String,
    pub(crate) route_path: String,
    pub(crate) stop_path: String,
    pub(crate) freight_sla_window: String,
    pub(crate) passenger_competitive_with_air: String,
    pub(crate) rail_competition_note: String,
    pub(crate) evidence_status: String,
}

// --- struct RecurringStopGap ---
#[derive(Debug)]
pub(crate) struct RecurringStopGap {
    pub(crate) segment_id: String,
    pub(crate) labels: String,
    pub(crate) miles: f64,
    pub(crate) row_count: usize,
    pub(crate) route_path: String,
}

// --- struct StopSlaCandidateRecommendation ---
#[derive(Debug)]
pub(crate) struct StopSlaCandidateRecommendation {
    pub(crate) gap: RecurringStopGap,
    pub(crate) candidates: Vec<StopSlaCandidateScore>,
}

// --- struct StopSlaCandidateScore ---
#[derive(Debug)]
pub(crate) struct StopSlaCandidateScore {
    pub(crate) stop_id: String,
    pub(crate) name: String,
    pub(crate) lat: f64,
    pub(crate) lon: f64,
    pub(crate) requested_class: String,
    pub(crate) route_refs: String,
    pub(crate) evidence_status: String,
    pub(crate) source_type: String,
    pub(crate) basis: String,
    pub(crate) spacing_gain_miles: f64,
    pub(crate) largest_resulting_gap_miles: f64,
    pub(crate) distance_from_segment_miles: f64,
    pub(crate) intersection_route_count: usize,
    pub(crate) score: f64,
}

// --- struct CitySeedFile ---
#[derive(Debug, serde::Deserialize)]
pub(crate) struct CitySeedFile {
    pub(crate) cities: Vec<CitySeedRow>,
}

// --- struct CitySeedRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct CitySeedRow {
    pub(crate) name: String,
    pub(crate) abbr: String,
    pub(crate) lat: f64,
    pub(crate) lon: f64,
}

// --- struct StopSlaCandidateDocketRow ---
#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
pub(crate) struct StopSlaCandidateDocketRow {
    pub(crate) gap_segment: String,
    pub(crate) gap_labels: String,
    pub(crate) gap_miles: f64,
    pub(crate) gap_row_count: usize,
    pub(crate) gap_routes: String,
    pub(crate) candidate_rank: usize,
    pub(crate) candidate_id: String,
    pub(crate) candidate_name: String,
    pub(crate) candidate_class: String,
    pub(crate) candidate_lat: String,
    pub(crate) candidate_lon: String,
    pub(crate) candidate_source_type: String,
    pub(crate) candidate_evidence_status: String,
    pub(crate) candidate_route_refs: String,
    pub(crate) candidate_basis: String,
    pub(crate) largest_resulting_gap_miles: f64,
    pub(crate) spacing_gain_miles: f64,
    pub(crate) offset_miles: f64,
    pub(crate) intersection_route_count: usize,
    pub(crate) score: f64,
}

// --- struct ScoreSignalRow ---
#[derive(Debug)]
pub(crate) struct ScoreSignalRow {
    pub(crate) a1: f64,
    pub(crate) a3: f64,
    pub(crate) b2: f64,
}

// --- struct MapAtlasRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct MapAtlasRow {
    pub(crate) map_id: String,
    pub(crate) path: String,
    pub(crate) map_type: String,
    pub(crate) render_command: String,
    pub(crate) expected_width: u32,
    pub(crate) expected_height: u32,
    pub(crate) min_bytes: u64,
    pub(crate) tier_role: String,
    pub(crate) game_use: String,
}

// --- struct MapPublicationScopeDecisionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct MapPublicationScopeDecisionRow {
    pub(crate) decision_id: String,
    pub(crate) decision_scope: String,
    pub(crate) map_surface: String,
    pub(crate) render_gate_status: String,
    pub(crate) evidence_gate_status: String,
    pub(crate) claim_status: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_count: usize,
    pub(crate) budget_debt_count: usize,
    pub(crate) blocking_artifacts: String,
    pub(crate) scope_treatment: String,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct MapPublicationReadinessRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct MapPublicationReadinessRow {
    pub(crate) readiness_id: String,
    pub(crate) map_surface: String,
    pub(crate) map_count: usize,
    pub(crate) map_types: String,
    pub(crate) render_gate_status: String,
    pub(crate) scope_decision_status: String,
    pub(crate) publication_blocker_count: usize,
    pub(crate) publication_blocker_families: String,
    pub(crate) held_claims: String,
    pub(crate) held_claim_family_count: usize,
    pub(crate) budget_debt_count: usize,
    pub(crate) scope_decision_artifact: String,
    pub(crate) backlog_artifact: String,
    pub(crate) readiness_decision: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct MapPublicationInventoryRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct MapPublicationInventoryRow {
    pub(crate) map_id: String,
    pub(crate) map_path: String,
    pub(crate) map_type: String,
    pub(crate) publication_status: String,
    pub(crate) render_gate_status: String,
    pub(crate) readiness_artifact: String,
    pub(crate) held_claims: String,
    pub(crate) required_label: String,
    pub(crate) allowed_use: String,
    pub(crate) not_allowed_claims: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct ScenarioEdgeCandidate ---
#[derive(Debug, Clone)]
pub(crate) struct ScenarioEdgeCandidate {
    pub(crate) edge_id: u64,
    pub(crate) distance_miles: f64,
    pub(crate) length_miles: f64,
    pub(crate) aadt: Option<u32>,
    pub(crate) lanes: Option<u8>,
    pub(crate) state: String,
    pub(crate) mid_lat: f64,
    pub(crate) mid_lon: f64,
}

// --- struct StandardsProofRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct StandardsProofRow {
    pub(crate) standard_id: String,
    pub(crate) tier: String,
    pub(crate) standard_family: String,
    pub(crate) standard: String,
    pub(crate) outcome: String,
    pub(crate) mechanism: String,
    pub(crate) primary_stressor: String,
    pub(crate) acceptance_gate: String,
    pub(crate) evidence_level: String,
    pub(crate) current_artifact: String,
    pub(crate) blocking_gap: String,
    pub(crate) next_command_or_test: String,
    pub(crate) owner_track: String,
}

// --- struct ForumDocketRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct ForumDocketRow {
    pub(crate) review_id: String,
    pub(crate) artifact: String,
    pub(crate) review_type: String,
    pub(crate) status: String,
    pub(crate) roles: String,
    pub(crate) claim_target: String,
    pub(crate) blocking_question: String,
    pub(crate) next_action: String,
    pub(crate) output_artifact: String,
}

// --- struct SignificantMomentRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct SignificantMomentRow {
    pub(crate) moment_id: String,
    pub(crate) date: String,
    pub(crate) flair: String,
    pub(crate) kind: String,
    pub(crate) summary: String,
    pub(crate) why_it_mattered: String,
    pub(crate) primary_artifacts: String,
    pub(crate) commit: String,
    pub(crate) next_thread: String,
}

// --- struct ReleaseManifestRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct ReleaseManifestRow {
    pub(crate) artifact_path: String,
    pub(crate) artifact_class: String,
    pub(crate) owner_milepost: String,
    pub(crate) release_status: String,
    pub(crate) public_status: String,
    pub(crate) verification_command: String,
    pub(crate) notes: String,
}

// --- struct BlueprintPackageRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct BlueprintPackageRow {
    pub(crate) package_id: String,
    pub(crate) phase: String,
    pub(crate) feature_package: String,
    pub(crate) stakeholder_class: String,
    pub(crate) standards: String,
    pub(crate) evidence_level: String,
    pub(crate) status: String,
    pub(crate) cost_range: String,
    pub(crate) value_case: String,
    pub(crate) source_label: String,
    pub(crate) pressure_artifact: String,
    pub(crate) forum_constraint: String,
    pub(crate) mitigation_companion: String,
    pub(crate) row_complexity: String,
    pub(crate) maintenance_burden: String,
    pub(crate) community_exposure_check: String,
    pub(crate) rural_access_exception: String,
    pub(crate) blueprint_action: String,
    pub(crate) blocking_gap: String,
    pub(crate) next_evidence_step: String,
}

// --- struct BlueprintEvidenceRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct BlueprintEvidenceRow {
    pub(crate) package_id: String,
    pub(crate) standard_id: String,
    pub(crate) proof_evidence_level: String,
    pub(crate) blueprint_claim_status: String,
    pub(crate) promotion_rule: String,
    pub(crate) proof_artifact: String,
    pub(crate) forum_hold: String,
    pub(crate) blocking_gap: String,
    pub(crate) required_next_evidence: String,
}

// --- struct BlueprintCostRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct BlueprintCostRow {
    pub(crate) package_id: String,
    pub(crate) cost_basis: String,
    pub(crate) capital_range_2026_usd: String,
    pub(crate) lifecycle_burden: String,
    pub(crate) source_status: String,
    pub(crate) source_artifact: String,
    pub(crate) cost_claim_status: String,
    pub(crate) risk_note: String,
    pub(crate) next_cost_step: String,
}

// --- struct StandardsInventoryRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct StandardsInventoryRow {
    pub(crate) standard_id: String,
    pub(crate) inventory_name: String,
    pub(crate) source_kind: String,
    pub(crate) source_status: String,
    pub(crate) current_artifact: String,
    pub(crate) coverage_scope: String,
    pub(crate) blocking_gap: String,
    pub(crate) next_step: String,
}

// --- struct PavementStandardRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct PavementStandardRow {
    pub(crate) tier: String,
    pub(crate) road_role: String,
    pub(crate) max_iri_m_per_km: f64,
    pub(crate) target_pavement_condition: String,
    pub(crate) freight_ride_requirement: String,
    pub(crate) transit_ride_requirement: String,
    pub(crate) inspection_interval_months: u16,
    pub(crate) repair_trigger: String,
    pub(crate) allowed_exception: String,
    pub(crate) source_contract: String,
    pub(crate) validation_status: String,
}

// --- struct TierRegionWorkloadRow ---
#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct TierRegionWorkloadRow {
    pub(crate) tier: String,
    pub(crate) graph_kind: String,
    pub(crate) split_objective: String,
    pub(crate) requested_regions: usize,
    pub(crate) region_id: usize,
    pub(crate) route: String,
    pub(crate) node_class: String,
    pub(crate) route_weight: i32,
    pub(crate) route_miles: f64,
    pub(crate) t1_node_count: usize,
    pub(crate) parent_trunk_count: usize,
    pub(crate) parent_trunks: String,
    pub(crate) contact_route_count: usize,
    pub(crate) component_id: usize,
    pub(crate) component_route_count: usize,
    pub(crate) component_status: String,
    pub(crate) repair_action: String,
    pub(crate) repair_basis: String,
    pub(crate) validation_status: String,
}

// --- struct TierRegionRepairRow ---
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct TierRegionRepairRow {
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) node_class: String,
    pub(crate) route_miles: f64,
    pub(crate) t1_node_count: usize,
    pub(crate) parent_trunks: String,
    pub(crate) contact_route_count: usize,
    pub(crate) component_id: usize,
    pub(crate) component_route_count: usize,
    pub(crate) component_status: String,
    pub(crate) repair_action: String,
    pub(crate) repair_basis: String,
    pub(crate) next_artifact: String,
}

// --- struct TierRegionRepairInputRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct TierRegionRepairInputRow {
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) node_class: String,
    pub(crate) route_miles: f64,
    pub(crate) t1_node_count: usize,
    pub(crate) parent_trunks: String,
    pub(crate) contact_route_count: usize,
    pub(crate) component_id: usize,
    pub(crate) component_route_count: usize,
    pub(crate) component_status: String,
    pub(crate) repair_action: String,
    pub(crate) repair_basis: String,
    pub(crate) next_artifact: String,
}

// --- struct TierContactWitnessRow ---
#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct TierContactWitnessRow {
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) witness_type: String,
    pub(crate) node_class: String,
    pub(crate) route_miles: f64,
    pub(crate) observed_t1_node_count: usize,
    pub(crate) observed_parent_trunks: String,
    pub(crate) observed_dual_contacts: usize,
    pub(crate) component_id: usize,
    pub(crate) component_route_count: usize,
    pub(crate) component_status: String,
    pub(crate) repair_action: String,
    pub(crate) repair_basis: String,
    pub(crate) evidence_status: String,
    pub(crate) required_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierContactWitnessInputRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct TierContactWitnessInputRow {
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) witness_type: String,
    pub(crate) node_class: String,
    pub(crate) route_miles: f64,
    pub(crate) observed_t1_node_count: usize,
    pub(crate) observed_parent_trunks: String,
    pub(crate) observed_dual_contacts: usize,
    pub(crate) component_id: usize,
    pub(crate) component_route_count: usize,
    pub(crate) component_status: String,
    pub(crate) repair_action: String,
    pub(crate) repair_basis: String,
    pub(crate) evidence_status: String,
    pub(crate) required_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2ContactResolutionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2ContactResolutionRow {
    pub(crate) route: String,
    pub(crate) witness_type: String,
    pub(crate) node_class: String,
    pub(crate) repair_action: String,
    pub(crate) required_artifact: String,
    pub(crate) exception_type: String,
    pub(crate) exception_evidence_level: String,
    pub(crate) resolution_action: String,
    pub(crate) resolution_basis: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2HeldContactActionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2HeldContactActionRow {
    pub(crate) route: String,
    pub(crate) held_action_type: String,
    pub(crate) source_resolution_action: String,
    pub(crate) exception_type: String,
    pub(crate) required_evidence: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct T2GraphContactRepairRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2GraphContactRepairRow {
    pub(crate) route: String,
    pub(crate) repair_class: String,
    pub(crate) source_exception_type: String,
    pub(crate) repair_action: String,
    pub(crate) required_evidence: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct T2ParentContactValidationRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2ParentContactValidationRow {
    pub(crate) route: String,
    pub(crate) parent_trunks: String,
    pub(crate) observed_dual_contacts: usize,
    pub(crate) validation_action: String,
    pub(crate) required_evidence: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct AtriBottleneckRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct AtriBottleneckRow {
    #[serde(rename = "RANK")]
    pub(crate) rank: usize,
    #[serde(rename = "LOCATION")]
    pub(crate) location: String,
    #[serde(rename = "ROUTE")]
    pub(crate) route: String,
    #[serde(rename = "STATE")]
    pub(crate) state: String,
    #[serde(rename = "ANNUAL_COST_M")]
    pub(crate) annual_cost_m: f64,
    #[serde(rename = "LAT")]
    pub(crate) lat: f64,
    #[serde(rename = "LON")]
    pub(crate) lon: f64,
}

// --- struct T2ReliefEvidenceRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2ReliefEvidenceRow {
    pub(crate) route: String,
    pub(crate) source_exception_type: String,
    pub(crate) bottleneck_match_count: usize,
    pub(crate) top_bottleneck_rank: usize,
    pub(crate) top_bottleneck_location: String,
    pub(crate) annual_cost_m: f64,
    pub(crate) relief_action: String,
    pub(crate) evidence_basis: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct T2TerminalContactValidationRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2TerminalContactValidationRow {
    pub(crate) route: String,
    pub(crate) held_action_type: String,
    pub(crate) endpoint_name: String,
    pub(crate) endpoint_role: String,
    pub(crate) exception_type: String,
    pub(crate) terminal_worthy: bool,
    pub(crate) observed_t1_node_count: usize,
    pub(crate) observed_dual_contacts: usize,
    pub(crate) terminal_action: String,
    pub(crate) required_evidence: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct T2BlockerClosureRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BlockerClosureRow {
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) bundle_status: String,
    pub(crate) bundle_action: String,
    pub(crate) source_surface: String,
    pub(crate) blocker_class: String,
    pub(crate) blocker_action: String,
    pub(crate) required_evidence: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) closure_status: String,
    pub(crate) validation_status: String,
}

// --- struct T2RouteFamilySplitRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2RouteFamilySplitRow {
    pub(crate) route: String,
    pub(crate) endpoint_name: String,
    pub(crate) endpoint_role: String,
    pub(crate) exception_type: String,
    pub(crate) source_artifact: String,
    pub(crate) family_action: String,
    pub(crate) disposition: String,
    pub(crate) required_evidence: String,
    pub(crate) next_artifact: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct T2GraphContactValidationRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2GraphContactValidationRow {
    pub(crate) route: String,
    pub(crate) observed_t1_node_count: usize,
    pub(crate) observed_dual_contacts: usize,
    pub(crate) observed_parent_trunks: String,
    pub(crate) contact_action: String,
    pub(crate) disposition: String,
    pub(crate) required_evidence: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct T2ContactClosureRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2ContactClosureRow {
    pub(crate) route: String,
    pub(crate) blocker_class: String,
    pub(crate) observed_t1_node_count: usize,
    pub(crate) observed_dual_contacts: usize,
    pub(crate) observed_parent_trunks: String,
    pub(crate) contact_action: String,
    pub(crate) disposition: String,
    pub(crate) required_evidence: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct T2EndpointClosureRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2EndpointClosureRow {
    pub(crate) route: String,
    pub(crate) endpoint_name: String,
    pub(crate) endpoint_role: String,
    pub(crate) exception_type: String,
    pub(crate) evidence_level: String,
    pub(crate) terminal_worthy: bool,
    pub(crate) endpoint_action: String,
    pub(crate) disposition: String,
    pub(crate) required_evidence: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct T2ClosureDisposition ---
#[derive(Debug, Clone)]
pub(crate) struct T2ClosureDisposition {
    pub(crate) route: String,
    pub(crate) disposition: String,
    pub(crate) action: String,
    pub(crate) basis: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) bundle_status: String,
    pub(crate) bundle_action: String,
    pub(crate) qualification_effects: String,
    pub(crate) source_artifact: String,
    pub(crate) next_artifact: String,
}

// --- struct TierCandidateColumnRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierCandidateColumnRow {
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) candidate_type: String,
    pub(crate) graph_kind: String,
    pub(crate) split_objective: String,
    pub(crate) node_class: String,
    pub(crate) route_miles: f64,
    pub(crate) observed_t1_node_count: usize,
    pub(crate) observed_dual_contacts: usize,
    pub(crate) parent_trunks: String,
    pub(crate) component_id: usize,
    pub(crate) component_route_count: usize,
    pub(crate) component_status: String,
    pub(crate) witness_type: String,
    pub(crate) repair_action: String,
    pub(crate) repair_basis: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) bundle_status: String,
    pub(crate) bundle_action: String,
    pub(crate) pavement_debt_cost_m: f64,
    pub(crate) pavement_debt_class: String,
    pub(crate) pavement_debt_basis: String,
    pub(crate) pavement_debt_artifact: String,
    pub(crate) hard_blocker_count: usize,
    pub(crate) claim_blocker_count: usize,
    pub(crate) constraint_debt_cost_m: f64,
    pub(crate) lifecycle_debt_cost_m: f64,
    pub(crate) constraint_penalty_score: f64,
    pub(crate) top_constraint_classes: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) constraint_ledger_artifact: String,
    pub(crate) column_decision: String,
    pub(crate) evidence_status: String,
    pub(crate) required_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2RegionalizerRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2RegionalizerRow {
    pub(crate) tier: String,
    pub(crate) region_id: String,
    pub(crate) component_id: usize,
    pub(crate) route: String,
    pub(crate) parent_trunks: String,
    pub(crate) route_miles: f64,
    pub(crate) column_decision: String,
    pub(crate) treatment_status: String,
    pub(crate) evidence_status: String,
    pub(crate) pavement_debt_cost_m: f64,
    pub(crate) pavement_debt_class: String,
    pub(crate) pavement_debt_basis: String,
    pub(crate) hard_blocker_count: usize,
    pub(crate) claim_blocker_count: usize,
    pub(crate) constraint_debt_cost_m: f64,
    pub(crate) lifecycle_debt_cost_m: f64,
    pub(crate) constraint_penalty_score: f64,
    pub(crate) top_constraint_classes: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) constraint_ledger_artifact: String,
    pub(crate) regionalizer_action: String,
    pub(crate) validation_status: String,
}

// --- struct T2ServiceSelectionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2ServiceSelectionRow {
    pub(crate) tier: String,
    pub(crate) region_id: String,
    pub(crate) route: String,
    pub(crate) parent_trunks: String,
    pub(crate) column_decision: String,
    pub(crate) treatment_status: String,
    pub(crate) beck_corridor: String,
    pub(crate) beck_service_class: String,
    pub(crate) beck_color_mode: String,
    pub(crate) beck_start_trunk: String,
    pub(crate) beck_end_trunk: String,
    pub(crate) duplicate_service_count: usize,
    pub(crate) duplicate_service_corridors: String,
    pub(crate) close_parallel_count: usize,
    pub(crate) close_parallel_corridors: String,
    pub(crate) unstopped_t1_contact_count: usize,
    pub(crate) unstopped_t1_contacts: String,
    pub(crate) pavement_debt_cost_m: f64,
    pub(crate) pavement_debt_class: String,
    pub(crate) pavement_debt_basis: String,
    pub(crate) hard_blocker_count: usize,
    pub(crate) claim_blocker_count: usize,
    pub(crate) constraint_debt_cost_m: f64,
    pub(crate) lifecycle_debt_cost_m: f64,
    pub(crate) constraint_penalty_score: f64,
    pub(crate) top_constraint_classes: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) constraint_ledger_artifact: String,
    pub(crate) beck_service_action: String,
    pub(crate) qualification_basis: String,
    pub(crate) qualification_map_treatment: String,
    pub(crate) qualification_gate_policy: String,
    pub(crate) qualification_game_use: String,
    pub(crate) selection_action: String,
    pub(crate) selection_basis: String,
    pub(crate) validation_status: String,
}

// --- struct GameT2ServiceOverlayRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct GameT2ServiceOverlayRow {
    pub(crate) service_class: String,
    pub(crate) map_id: String,
    pub(crate) scenario_hook: String,
    pub(crate) incident_lever: String,
    pub(crate) upgrade_lever: String,
    pub(crate) restitch_lever: String,
    pub(crate) release_gate: String,
}

// --- struct T2BundleOverlayRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BundleOverlayRow {
    pub(crate) tier: String,
    pub(crate) region_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) bundle_status: String,
    pub(crate) service_class: String,
    pub(crate) map_id: String,
    pub(crate) scenario_hook: String,
    pub(crate) incident_lever: String,
    pub(crate) upgrade_lever: String,
    pub(crate) restitch_lever: String,
    pub(crate) release_gate: String,
    pub(crate) qualification_map_treatment: String,
    pub(crate) qualification_gate_policy: String,
    pub(crate) qualification_game_use: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) pavement_debt_cost_m: f64,
    pub(crate) pavement_debt_class: String,
    pub(crate) pavement_debt_basis: String,
    pub(crate) source_artifacts: String,
    pub(crate) binding_status: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2ScenarioHookRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct T2ScenarioHookRow {
    pub(crate) scenario_id: String,
    pub(crate) service_class: String,
    pub(crate) t2_map_id: String,
    pub(crate) player_decision: String,
    pub(crate) evidence_hold: String,
}

// --- struct T2ServiceDiagnosticQueueRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2ServiceDiagnosticQueueRow {
    pub(crate) route: String,
    pub(crate) region_id: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) bundle_status: String,
    pub(crate) selection_action: String,
    pub(crate) selection_basis: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) diagnostic_status: String,
    pub(crate) service_diagnostic_action: String,
    pub(crate) required_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct T2ParallelServiceQueueRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2ParallelServiceQueueRow {
    pub(crate) route: String,
    pub(crate) region_id: String,
    pub(crate) beck_corridor: String,
    pub(crate) service_class: String,
    pub(crate) close_parallel_count: usize,
    pub(crate) close_parallel_corridors: String,
    pub(crate) selection_action: String,
    pub(crate) selection_basis: String,
    pub(crate) parallel_action: String,
    pub(crate) required_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) validation_status: String,
}

// --- struct OptimizerConstraintLedgerRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct OptimizerConstraintLedgerRow {
    pub(crate) constraint_id: String,
    pub(crate) optimizer_run_id: String,
    pub(crate) tier: String,
    pub(crate) region_id: String,
    pub(crate) constraint_order: u8,
    pub(crate) constraint_class: String,
    pub(crate) behavior_type: String,
    pub(crate) constraint_scope: String,
    pub(crate) subject_id: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) national_segment_id: String,
    pub(crate) stitch_group_id: String,
    pub(crate) route: String,
    pub(crate) stop_id: String,
    pub(crate) pair_id: String,
    pub(crate) map_id: String,
    pub(crate) source_artifact: String,
    pub(crate) source_row_id: String,
    pub(crate) standard_artifact: String,
    pub(crate) evidence_status: String,
    pub(crate) constraint_status: String,
    pub(crate) observed_value: String,
    pub(crate) threshold_value: String,
    pub(crate) measurement_unit: String,
    pub(crate) blocks_claims: String,
    pub(crate) budget_cost_m: f64,
    pub(crate) cost_category: String,
    pub(crate) cost_basis: String,
    pub(crate) cost_confidence: String,
    pub(crate) budget_units: String,
    pub(crate) penalty_score: f64,
    pub(crate) repair_action: String,
    pub(crate) payment_action: String,
    pub(crate) owner_jurisdiction: String,
    pub(crate) funding_program: String,
    pub(crate) delivery_risk: String,
    pub(crate) exception_id: String,
    pub(crate) exception_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct OptimizerConstraintBudgetRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct OptimizerConstraintBudgetRow {
    pub(crate) budget_id: String,
    pub(crate) optimizer_run_id: String,
    pub(crate) tier: String,
    pub(crate) region_id: String,
    pub(crate) subject_scope: String,
    pub(crate) subject_id: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) route: String,
    pub(crate) ledger_row_count: usize,
    pub(crate) hard_blocker_count: usize,
    pub(crate) claim_blocker_count: usize,
    pub(crate) review_count: usize,
    pub(crate) budget_debt_count: usize,
    pub(crate) constraint_debt_cost_m: f64,
    pub(crate) lifecycle_debt_cost_m: f64,
    pub(crate) constraint_penalty_score: f64,
    pub(crate) top_constraint_classes: String,
    pub(crate) blocking_claims: String,
    pub(crate) qualification_effects: String,
    pub(crate) next_artifacts: String,
    pub(crate) constraint_ledger_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct OptimizerResidualBlockerBacklogRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct OptimizerResidualBlockerBacklogRow {
    pub(crate) backlog_id: String,
    pub(crate) priority_class: String,
    pub(crate) blocker_family: String,
    pub(crate) tier: String,
    pub(crate) blocked_claims: String,
    pub(crate) subject_count: usize,
    pub(crate) route_count: usize,
    pub(crate) total_hard_blockers: usize,
    pub(crate) total_claim_blockers: usize,
    pub(crate) total_budget_debt_count: usize,
    pub(crate) total_constraint_debt_cost_m: f64,
    pub(crate) total_constraint_penalty_score: f64,
    pub(crate) representative_routes: String,
    pub(crate) representative_subjects: String,
    pub(crate) next_artifacts: String,
    pub(crate) backlog_decision: String,
    pub(crate) next_wave: String,
    pub(crate) validation_status: String,
}

// --- struct OptimizerClaimReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct OptimizerClaimReviewRow {
    pub(crate) claim_review_id: String,
    pub(crate) backlog_id: String,
    pub(crate) priority_class: String,
    pub(crate) blocker_family: String,
    pub(crate) tier: String,
    pub(crate) blocked_claims: String,
    pub(crate) subject_count: usize,
    pub(crate) route_count: usize,
    pub(crate) total_claim_blockers: usize,
    pub(crate) representative_routes: String,
    pub(crate) representative_subjects: String,
    pub(crate) evidence_artifacts: String,
    pub(crate) review_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2GamePublicationEvidenceReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2GamePublicationEvidenceReviewRow {
    pub(crate) game_review_id: String,
    pub(crate) claim_review_id: String,
    pub(crate) scenario_id: String,
    pub(crate) service_class: String,
    pub(crate) t2_map_id: String,
    pub(crate) player_decision: String,
    pub(crate) evidence_hold: String,
    pub(crate) review_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) required_evidence: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2GamePublicationEvidencePolicyRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2GamePublicationEvidencePolicyRow {
    pub(crate) policy_id: String,
    pub(crate) game_review_id: String,
    pub(crate) scenario_id: String,
    pub(crate) service_class: String,
    pub(crate) t2_map_id: String,
    pub(crate) evidence_policy_basis: String,
    pub(crate) required_evidence: String,
    pub(crate) evidence_policy_decision: String,
    pub(crate) policy_treatment: String,
    pub(crate) publication_treatment: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2GamePublicationEvidencePolicyAcceptanceRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2GamePublicationEvidencePolicyAcceptanceRow {
    pub(crate) acceptance_id: String,
    pub(crate) policy_id: String,
    pub(crate) scenario_id: String,
    pub(crate) service_class: String,
    pub(crate) t2_map_id: String,
    pub(crate) accepted_required_evidence: String,
    pub(crate) accepted_policy_treatment: String,
    pub(crate) acceptance_decision: String,
    pub(crate) publication_treatment: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2GamePublicationEvidenceBlockerReliefRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2GamePublicationEvidenceBlockerReliefRow {
    pub(crate) relief_id: String,
    pub(crate) acceptance_id: String,
    pub(crate) policy_id: String,
    pub(crate) scenario_id: String,
    pub(crate) service_class: String,
    pub(crate) accepted_required_evidence: String,
    pub(crate) relief_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) ledger_replay_status: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T1SchematicGeometryClaimReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T1SchematicGeometryClaimReviewRow {
    pub(crate) schematic_review_id: String,
    pub(crate) claim_review_id: String,
    pub(crate) route: String,
    pub(crate) design_role: String,
    pub(crate) design_status: String,
    pub(crate) beck_review_flag: String,
    pub(crate) overlap_corridors: String,
    pub(crate) policy_action: String,
    pub(crate) required_policy: String,
    pub(crate) design_treatment: String,
    pub(crate) gate_policy: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) review_decision: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BeckTransferComplexityReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BeckTransferComplexityReviewRow {
    pub(crate) transfer_review_id: String,
    pub(crate) claim_review_id: String,
    pub(crate) route: String,
    pub(crate) trunk: String,
    pub(crate) start_trunk: String,
    pub(crate) end_trunk: String,
    pub(crate) service_class: String,
    pub(crate) service_label: String,
    pub(crate) stop_count: usize,
    pub(crate) transfer_stop_count: usize,
    pub(crate) unique_duplicate_stop_count: usize,
    pub(crate) label_density_per_100px: f64,
    pub(crate) review_flag: String,
    pub(crate) complexity_basis: String,
    pub(crate) review_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BeckLabelDensityReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BeckLabelDensityReviewRow {
    pub(crate) label_review_id: String,
    pub(crate) claim_review_id: String,
    pub(crate) route: String,
    pub(crate) trunk: String,
    pub(crate) start_trunk: String,
    pub(crate) end_trunk: String,
    pub(crate) service_class: String,
    pub(crate) service_label: String,
    pub(crate) stop_count: usize,
    pub(crate) transfer_stop_count: usize,
    pub(crate) label_density_per_100px: f64,
    pub(crate) review_flag: String,
    pub(crate) density_basis: String,
    pub(crate) review_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BeckLongConnectorReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BeckLongConnectorReviewRow {
    pub(crate) connector_review_id: String,
    pub(crate) claim_review_id: String,
    pub(crate) route: String,
    pub(crate) trunk: String,
    pub(crate) start_trunk: String,
    pub(crate) end_trunk: String,
    pub(crate) service_class: String,
    pub(crate) service_label: String,
    pub(crate) stop_count: usize,
    pub(crate) transfer_stop_count: usize,
    pub(crate) schematic_length_px: f64,
    pub(crate) split_anchor: String,
    pub(crate) split_anchor_offset_pct: f64,
    pub(crate) review_flag: String,
    pub(crate) connector_basis: String,
    pub(crate) review_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BeckLongConnectorPolicyRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BeckLongConnectorPolicyRow {
    pub(crate) policy_id: String,
    pub(crate) connector_review_id: String,
    pub(crate) route: String,
    pub(crate) trunk_pair: String,
    pub(crate) service_class: String,
    pub(crate) schematic_length_px: f64,
    pub(crate) connector_band: String,
    pub(crate) policy_basis: String,
    pub(crate) connector_policy_decision: String,
    pub(crate) render_treatment: String,
    pub(crate) promotion_treatment: String,
    pub(crate) publication_status: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BeckLongConnectorPolicyAcceptanceRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BeckLongConnectorPolicyAcceptanceRow {
    pub(crate) acceptance_id: String,
    pub(crate) policy_id: String,
    pub(crate) route: String,
    pub(crate) connector_band: String,
    pub(crate) accepted_render_treatment: String,
    pub(crate) accepted_promotion_treatment: String,
    pub(crate) acceptance_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BeckLongConnectorBlockerReliefRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BeckLongConnectorBlockerReliefRow {
    pub(crate) relief_id: String,
    pub(crate) acceptance_id: String,
    pub(crate) policy_id: String,
    pub(crate) route: String,
    pub(crate) connector_band: String,
    pub(crate) accepted_render_treatment: String,
    pub(crate) relief_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) ledger_replay_status: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BeckLabelDensityPolicyRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BeckLabelDensityPolicyRow {
    pub(crate) policy_id: String,
    pub(crate) label_review_id: String,
    pub(crate) route: String,
    pub(crate) trunk_pair: String,
    pub(crate) service_class: String,
    pub(crate) label_density_per_100px: f64,
    pub(crate) density_band: String,
    pub(crate) policy_basis: String,
    pub(crate) label_policy_decision: String,
    pub(crate) render_treatment: String,
    pub(crate) promotion_treatment: String,
    pub(crate) publication_status: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BeckLabelDensityPolicyAcceptanceRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BeckLabelDensityPolicyAcceptanceRow {
    pub(crate) acceptance_id: String,
    pub(crate) policy_id: String,
    pub(crate) route: String,
    pub(crate) density_band: String,
    pub(crate) accepted_render_treatment: String,
    pub(crate) accepted_promotion_treatment: String,
    pub(crate) acceptance_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BeckLabelDensityBlockerReliefRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BeckLabelDensityBlockerReliefRow {
    pub(crate) relief_id: String,
    pub(crate) acceptance_id: String,
    pub(crate) policy_id: String,
    pub(crate) route: String,
    pub(crate) density_band: String,
    pub(crate) accepted_render_treatment: String,
    pub(crate) relief_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) ledger_replay_status: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BeckTransferComplexityPolicyRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BeckTransferComplexityPolicyRow {
    pub(crate) policy_id: String,
    pub(crate) transfer_review_id: String,
    pub(crate) route: String,
    pub(crate) trunk_pair: String,
    pub(crate) service_class: String,
    pub(crate) transfer_stop_count: usize,
    pub(crate) stop_count: usize,
    pub(crate) complexity_band: String,
    pub(crate) policy_basis: String,
    pub(crate) transfer_policy_decision: String,
    pub(crate) render_treatment: String,
    pub(crate) promotion_treatment: String,
    pub(crate) publication_status: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BeckTransferComplexityPolicyAcceptanceRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BeckTransferComplexityPolicyAcceptanceRow {
    pub(crate) acceptance_id: String,
    pub(crate) policy_id: String,
    pub(crate) route: String,
    pub(crate) complexity_band: String,
    pub(crate) accepted_render_treatment: String,
    pub(crate) accepted_promotion_treatment: String,
    pub(crate) acceptance_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BeckTransferComplexityBlockerReliefRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BeckTransferComplexityBlockerReliefRow {
    pub(crate) relief_id: String,
    pub(crate) acceptance_id: String,
    pub(crate) policy_id: String,
    pub(crate) route: String,
    pub(crate) complexity_band: String,
    pub(crate) accepted_render_treatment: String,
    pub(crate) relief_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) ledger_replay_status: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T3LowerTierFeederGapReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T3LowerTierFeederGapReviewRow {
    pub(crate) feeder_review_id: String,
    pub(crate) backlog_id: String,
    pub(crate) gap_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) current_score: f64,
    pub(crate) constraint_adjusted_score: f64,
    pub(crate) promise_horizon_hours: u8,
    pub(crate) gap_class: String,
    pub(crate) gap_reason: String,
    pub(crate) required_evidence: String,
    pub(crate) repair_action: String,
    pub(crate) review_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T3LowerTierFeederGapPolicyRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T3LowerTierFeederGapPolicyRow {
    pub(crate) policy_id: String,
    pub(crate) feeder_review_id: String,
    pub(crate) gap_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) score_band: String,
    pub(crate) policy_basis: String,
    pub(crate) feeder_policy_decision: String,
    pub(crate) map_treatment: String,
    pub(crate) evidence_treatment: String,
    pub(crate) upgrade_treatment: String,
    pub(crate) publication_status: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T3LowerTierFeederGapPolicyAcceptanceRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T3LowerTierFeederGapPolicyAcceptanceRow {
    pub(crate) acceptance_id: String,
    pub(crate) policy_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) score_band: String,
    pub(crate) accepted_map_treatment: String,
    pub(crate) accepted_evidence_treatment: String,
    pub(crate) accepted_upgrade_treatment: String,
    pub(crate) acceptance_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T3LowerTierFeederGapBlockerReliefRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T3LowerTierFeederGapBlockerReliefRow {
    pub(crate) relief_id: String,
    pub(crate) acceptance_id: String,
    pub(crate) policy_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) score_band: String,
    pub(crate) accepted_map_treatment: String,
    pub(crate) relief_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) ledger_replay_status: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T1SharedSegmentMapPolicyRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T1SharedSegmentMapPolicyRow {
    pub(crate) policy_id: String,
    pub(crate) route_pair: String,
    pub(crate) primary_route: String,
    pub(crate) overlap_route: String,
    pub(crate) affected_routes: String,
    pub(crate) source_review_ids: String,
    pub(crate) policy_basis: String,
    pub(crate) map_policy_decision: String,
    pub(crate) render_treatment: String,
    pub(crate) selector_treatment: String,
    pub(crate) publication_status: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T1SharedSegmentPolicyAcceptanceRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T1SharedSegmentPolicyAcceptanceRow {
    pub(crate) acceptance_id: String,
    pub(crate) policy_id: String,
    pub(crate) route_pair: String,
    pub(crate) affected_routes: String,
    pub(crate) map_policy_decision: String,
    pub(crate) accepted_render_treatment: String,
    pub(crate) acceptance_status: String,
    pub(crate) acceptance_basis: String,
    pub(crate) publication_status_before: String,
    pub(crate) publication_status_after: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T1SchematicGeometryBlockerReliefRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T1SchematicGeometryBlockerReliefRow {
    pub(crate) relief_id: String,
    pub(crate) acceptance_id: String,
    pub(crate) policy_id: String,
    pub(crate) route_pair: String,
    pub(crate) affected_routes: String,
    pub(crate) accepted_render_treatment: String,
    pub(crate) relief_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) ledger_replay_status: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2GameOpsBindingIntakeRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2GameOpsBindingIntakeRow {
    pub(crate) intake_id: String,
    pub(crate) budget_id: String,
    pub(crate) subject_id: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) route: String,
    pub(crate) claim_blocker_count: usize,
    pub(crate) blocked_claims: String,
    pub(crate) top_constraint_classes: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) next_artifacts: String,
    pub(crate) constraint_ledger_artifact: String,
    pub(crate) intake_status: String,
    pub(crate) decision_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2GameOpsBindingDecisionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2GameOpsBindingDecisionRow {
    pub(crate) decision_id: String,
    pub(crate) intake_id: String,
    pub(crate) subject_id: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) route: String,
    pub(crate) service_class: String,
    pub(crate) bundle_status: String,
    pub(crate) binding_status: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) qualification_gate_policy: String,
    pub(crate) qualification_game_use: String,
    pub(crate) decision: String,
    pub(crate) decision_reason: String,
    pub(crate) blocks_claims: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BundleOverlayRepairTargetRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BundleOverlayRepairTargetRow {
    pub(crate) target_id: String,
    pub(crate) decision_id: String,
    pub(crate) subject_id: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) route: String,
    pub(crate) decision: String,
    pub(crate) binding_status: String,
    pub(crate) bundle_status: String,
    pub(crate) service_class: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) qualification_gate_policy: String,
    pub(crate) qualification_game_use: String,
    pub(crate) pavement_debt_cost_m: f64,
    pub(crate) pavement_debt_class: String,
    pub(crate) blocks_claims: String,
    pub(crate) repair_class: String,
    pub(crate) repair_action: String,
    pub(crate) required_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) target_status: String,
    pub(crate) validation_status: String,
}

// --- struct T2ServiceClassRepairDocketRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2ServiceClassRepairDocketRow {
    pub(crate) docket_id: String,
    pub(crate) target_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) bundle_status: String,
    pub(crate) service_class: String,
    pub(crate) service_repair_class: String,
    pub(crate) service_action: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) required_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct T2GameOpsBundleEvidenceReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2GameOpsBundleEvidenceReviewRow {
    pub(crate) review_id: String,
    pub(crate) decision_id: String,
    pub(crate) target_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) decision: String,
    pub(crate) binding_status: String,
    pub(crate) bundle_status: String,
    pub(crate) service_class: String,
    pub(crate) repair_class: String,
    pub(crate) repair_action: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) qualification_gate_policy: String,
    pub(crate) qualification_game_use: String,
    pub(crate) evidence_artifact: String,
    pub(crate) service_repair_class: String,
    pub(crate) evidence_status: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2GameOpsBundleEvidencePolicyRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2GameOpsBundleEvidencePolicyRow {
    pub(crate) policy_id: String,
    pub(crate) review_id: String,
    pub(crate) decision_id: String,
    pub(crate) target_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) repair_class: String,
    pub(crate) service_repair_class: String,
    pub(crate) evidence_artifact: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) qualification_gate_policy: String,
    pub(crate) qualification_game_use: String,
    pub(crate) required_evidence: String,
    pub(crate) evidence_policy_decision: String,
    pub(crate) policy_treatment: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2GameOpsBundleEvidencePolicyAcceptanceRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2GameOpsBundleEvidencePolicyAcceptanceRow {
    pub(crate) acceptance_id: String,
    pub(crate) policy_id: String,
    pub(crate) review_id: String,
    pub(crate) decision_id: String,
    pub(crate) target_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) accepted_required_evidence: String,
    pub(crate) accepted_policy_treatment: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) qualification_gate_policy: String,
    pub(crate) qualification_game_use: String,
    pub(crate) acceptance_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2GameOpsBundleEvidenceBlockerReliefRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2GameOpsBundleEvidenceBlockerReliefRow {
    pub(crate) relief_id: String,
    pub(crate) acceptance_id: String,
    pub(crate) policy_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) accepted_required_evidence: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) qualification_gate_policy: String,
    pub(crate) qualification_game_use: String,
    pub(crate) relief_decision: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) blocker_count_before: usize,
    pub(crate) blocker_count_after: usize,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) ledger_replay_status: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2ServiceOverlayDiagnosticDecisionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2ServiceOverlayDiagnosticDecisionRow {
    pub(crate) decision_id: String,
    pub(crate) docket_id: String,
    pub(crate) target_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) bundle_status: String,
    pub(crate) current_service_class: String,
    pub(crate) diagnostic_status: String,
    pub(crate) diagnostic_action: String,
    pub(crate) overlay_decision: String,
    pub(crate) decision_reason: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) blocks_claims: String,
    pub(crate) required_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2LocalZoneOverlayHandoffRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2LocalZoneOverlayHandoffRow {
    pub(crate) handoff_id: String,
    pub(crate) docket_id: String,
    pub(crate) target_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) zone_id: String,
    pub(crate) zone_name: String,
    pub(crate) zone_role: String,
    pub(crate) column_decision: String,
    pub(crate) map_treatment: String,
    pub(crate) handoff_decision: String,
    pub(crate) handoff_reason: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) blocks_claims: String,
    pub(crate) required_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BundleReadinessDispositionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BundleReadinessDispositionRow {
    pub(crate) disposition_id: String,
    pub(crate) target_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) bundle_status: String,
    pub(crate) service_class: String,
    pub(crate) readiness_class: String,
    pub(crate) disposition: String,
    pub(crate) disposition_action: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) required_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) blocks_claims: String,
    pub(crate) validation_status: String,
}

// --- struct T2BundleReadinessRepairDocketRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BundleReadinessRepairDocketRow {
    pub(crate) repair_id: String,
    pub(crate) disposition_id: String,
    pub(crate) target_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) readiness_class: String,
    pub(crate) repair_decision: String,
    pub(crate) repair_action: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) required_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) blocks_claims: String,
    pub(crate) validation_status: String,
}

// --- struct T2BundleReadinessRepairEvidenceRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BundleReadinessRepairEvidenceRow {
    pub(crate) evidence_id: String,
    pub(crate) repair_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) readiness_class: String,
    pub(crate) evidence_artifact: String,
    pub(crate) evidence_status: String,
    pub(crate) evidence_row_count: usize,
    pub(crate) evidence_summary: String,
    pub(crate) evidence_decision: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) next_artifact: String,
    pub(crate) blocks_claims: String,
    pub(crate) validation_status: String,
}

// --- struct T2BundleReadinessReplayDecisionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BundleReadinessReplayDecisionRow {
    pub(crate) replay_id: String,
    pub(crate) evidence_id: String,
    pub(crate) delta_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) readiness_class: String,
    pub(crate) evidence_status: String,
    pub(crate) delta_replay_decision: String,
    pub(crate) replay_decision: String,
    pub(crate) replay_action: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2NationalBundleReadinessAuditRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2NationalBundleReadinessAuditRow {
    pub(crate) audit_id: String,
    pub(crate) replay_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) readiness_class: String,
    pub(crate) replay_decision: String,
    pub(crate) bundle_status: String,
    pub(crate) bundle_validation_status: String,
    pub(crate) bundle_member_count: usize,
    pub(crate) audit_decision: String,
    pub(crate) audit_action: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2StitchedMemberRegistryHandoffRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2StitchedMemberRegistryHandoffRow {
    pub(crate) handoff_id: String,
    pub(crate) audit_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) stitch_group_id: String,
    pub(crate) current_registry_member_count: usize,
    pub(crate) candidate_bundle_member_count: usize,
    pub(crate) candidate_route_member_count: usize,
    pub(crate) required_member_min: usize,
    pub(crate) handoff_decision: String,
    pub(crate) handoff_action: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2StitchedMemberCandidateScopeReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2StitchedMemberCandidateScopeReviewRow {
    pub(crate) scope_review_id: String,
    pub(crate) handoff_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) blocked_bundle_candidate_count: usize,
    pub(crate) route_candidate_count: usize,
    pub(crate) route_candidate_bundle_count: usize,
    pub(crate) route_candidate_state_scope: String,
    pub(crate) route_candidate_bundle_ids: String,
    pub(crate) scope_decision: String,
    pub(crate) scope_action: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2StitchedMemberDecisionDocketRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2StitchedMemberDecisionDocketRow {
    pub(crate) decision_docket_id: String,
    pub(crate) scope_review_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) candidate_bundle_count: usize,
    pub(crate) candidate_state_scope: String,
    pub(crate) decision: String,
    pub(crate) decision_action: String,
    pub(crate) repair_instruction: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2StitchedMemberSplitPlanRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2StitchedMemberSplitPlanRow {
    pub(crate) split_plan_id: String,
    pub(crate) decision_docket_id: String,
    pub(crate) route: String,
    pub(crate) blocked_segment_bundle_id: String,
    pub(crate) candidate_segment_bundle_id: String,
    pub(crate) candidate_stitch_group_id: String,
    pub(crate) state_scope: String,
    pub(crate) candidate_member_count: usize,
    pub(crate) candidate_length_miles: f64,
    pub(crate) split_action: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2StitchedMemberSelectionDocketRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2StitchedMemberSelectionDocketRow {
    pub(crate) selection_docket_id: String,
    pub(crate) split_plan_id: String,
    pub(crate) route: String,
    pub(crate) blocked_segment_bundle_id: String,
    pub(crate) candidate_segment_bundle_id: String,
    pub(crate) state_scope: String,
    pub(crate) candidate_member_count: usize,
    pub(crate) candidate_length_miles: f64,
    pub(crate) selection_decision: String,
    pub(crate) selection_action: String,
    pub(crate) evidence_requirement: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2StitchedMemberEvidenceContractRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2StitchedMemberEvidenceContractRow {
    pub(crate) evidence_contract_id: String,
    pub(crate) selection_docket_id: String,
    pub(crate) route: String,
    pub(crate) candidate_segment_bundle_id: String,
    pub(crate) state_scope: String,
    pub(crate) required_continuity_proof: String,
    pub(crate) required_scope_proof: String,
    pub(crate) required_source_proof: String,
    pub(crate) evidence_status: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2StitchedMemberEvidenceAcquisitionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2StitchedMemberEvidenceAcquisitionRow {
    pub(crate) acquisition_docket_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) route: String,
    pub(crate) candidate_segment_bundle_id: String,
    pub(crate) state_scope: String,
    pub(crate) source_owner: String,
    pub(crate) source_target: String,
    pub(crate) acquisition_action: String,
    pub(crate) acquisition_status: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2StitchedMemberSourceAccessPolicyRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2StitchedMemberSourceAccessPolicyRow {
    pub(crate) access_policy_id: String,
    pub(crate) acquisition_docket_id: String,
    pub(crate) route: String,
    pub(crate) candidate_segment_bundle_id: String,
    pub(crate) state_scope: String,
    pub(crate) source_owner: String,
    pub(crate) access_mode: String,
    pub(crate) live_fetch_status: String,
    pub(crate) required_source_metadata: String,
    pub(crate) cache_policy_artifact: String,
    pub(crate) source_access_blocker: String,
    pub(crate) evidence_artifact: String,
    pub(crate) acquisition_status: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2StitchedMemberProofIntakeRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2StitchedMemberProofIntakeRow {
    pub(crate) proof_intake_id: String,
    pub(crate) access_policy_id: String,
    pub(crate) route: String,
    pub(crate) candidate_segment_bundle_id: String,
    pub(crate) state_scope: String,
    pub(crate) required_artifact_fields: String,
    pub(crate) required_geometry_statement: String,
    pub(crate) proof_artifact: String,
    pub(crate) proof_status: String,
    pub(crate) proof_blocker: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2StitchedMemberProofSourceCaptureRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2StitchedMemberProofSourceCaptureRow {
    pub(crate) source_capture_id: String,
    pub(crate) proof_intake_id: String,
    pub(crate) route: String,
    pub(crate) candidate_segment_bundle_id: String,
    pub(crate) state_scope: String,
    pub(crate) source_artifact_reference: String,
    pub(crate) source_artifact_type: String,
    pub(crate) capture_status: String,
    pub(crate) evidence_acceptance_status: String,
    pub(crate) capture_blocker: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2StitchedMemberProofArtifactAttachmentRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2StitchedMemberProofArtifactAttachmentRow {
    pub(crate) artifact_attachment_id: String,
    pub(crate) source_capture_id: String,
    pub(crate) route: String,
    pub(crate) candidate_segment_bundle_id: String,
    pub(crate) state_scope: String,
    pub(crate) source_artifact_reference: String,
    pub(crate) attachment_status: String,
    pub(crate) evidence_review_status: String,
    pub(crate) proof_acceptance_status: String,
    pub(crate) attachment_blocker: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2StitchedMemberProofReviewDocketRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2StitchedMemberProofReviewDocketRow {
    pub(crate) proof_review_id: String,
    pub(crate) artifact_attachment_id: String,
    pub(crate) route: String,
    pub(crate) candidate_segment_bundle_id: String,
    pub(crate) state_scope: String,
    pub(crate) source_artifact_reference: String,
    pub(crate) review_decision: String,
    pub(crate) proof_acceptance_status: String,
    pub(crate) candidate_disposition_status: String,
    pub(crate) optimization_return_status: String,
    pub(crate) review_reason: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BundleOverlayRepairDeltaRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BundleOverlayRepairDeltaRow {
    pub(crate) delta_id: String,
    pub(crate) decision_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) previous_decision: String,
    pub(crate) target_status: String,
    pub(crate) service_action: String,
    pub(crate) readiness_disposition: String,
    pub(crate) replay_decision: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2OverlayOptimizerActionDocketRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2OverlayOptimizerActionDocketRow {
    pub(crate) action_id: String,
    pub(crate) delta_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) replay_decision: String,
    pub(crate) service_action: String,
    pub(crate) readiness_disposition: String,
    pub(crate) optimizer_action: String,
    pub(crate) priority_class: String,
    pub(crate) action_status: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2OverlayP1StructuralReadinessReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2OverlayP1StructuralReadinessReviewRow {
    pub(crate) p1_review_id: String,
    pub(crate) action_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) optimizer_action: String,
    pub(crate) priority_class: String,
    pub(crate) readiness_decision: String,
    pub(crate) readiness_reason: String,
    pub(crate) downstream_action: String,
    pub(crate) action_status: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2OverlayP2ServiceOverlayReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2OverlayP2ServiceOverlayReviewRow {
    pub(crate) p2_review_id: String,
    pub(crate) action_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) optimizer_action: String,
    pub(crate) priority_class: String,
    pub(crate) service_overlay_decision: String,
    pub(crate) service_overlay_reason: String,
    pub(crate) downstream_action: String,
    pub(crate) action_status: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2OverlayP3LocalZoneOverlayReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2OverlayP3LocalZoneOverlayReviewRow {
    pub(crate) p3_review_id: String,
    pub(crate) action_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) optimizer_action: String,
    pub(crate) priority_class: String,
    pub(crate) local_zone_decision: String,
    pub(crate) local_zone_reason: String,
    pub(crate) downstream_action: String,
    pub(crate) action_status: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BundleRepairQueueRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BundleRepairQueueRow {
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) bundle_status: String,
    pub(crate) bundle_action: String,
    pub(crate) contact_evidence_status: String,
    pub(crate) candidate_decision: String,
    pub(crate) repair_class: String,
    pub(crate) repair_action: String,
    pub(crate) required_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) validation_status: String,
}

// --- struct TierSegmentCandidateRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierSegmentCandidateRow {
    pub(crate) tier: String,
    pub(crate) source_selector: String,
    pub(crate) region_id: String,
    pub(crate) route: String,
    pub(crate) edge_id: u64,
    pub(crate) edge_sequence: usize,
    pub(crate) national_segment_id: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) stitch_group_id: String,
    pub(crate) member_role: String,
    pub(crate) state: String,
    pub(crate) length_miles: f64,
    pub(crate) aadt: String,
    pub(crate) lane_count: String,
    pub(crate) route_aliases: String,
    pub(crate) selector_basis: String,
    pub(crate) candidate_action: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementDocketRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementDocketRow {
    pub(crate) tier: String,
    pub(crate) source_selector: String,
    pub(crate) region_id: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) stitch_group_id: String,
    pub(crate) national_segment_id: String,
    pub(crate) edge_id: u64,
    pub(crate) edge_sequence: usize,
    pub(crate) state: String,
    pub(crate) length_miles: f64,
    pub(crate) iri_m_per_km: String,
    pub(crate) max_iri_m_per_km: String,
    pub(crate) pavement_status: String,
    pub(crate) repair_action: String,
    pub(crate) freight_ride_requirement: String,
    pub(crate) transit_ride_requirement: String,
    pub(crate) source_contract: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementSourceGapRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementSourceGapRow {
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) region_id: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) stitch_group_id: String,
    pub(crate) member_count: usize,
    pub(crate) blocker_count: usize,
    pub(crate) blocker_statuses: String,
    pub(crate) affected_states: String,
    pub(crate) affected_edge_ids: String,
    pub(crate) source_contract: String,
    pub(crate) source_action: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementDebtBudgetRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementDebtBudgetRow {
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) region_id: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) stitch_group_id: String,
    pub(crate) debt_class: String,
    pub(crate) blocked_member_count: usize,
    pub(crate) affected_states: String,
    pub(crate) evidence_debt_units: usize,
    pub(crate) repair_debt_units: usize,
    pub(crate) estimated_evidence_cost_m: f64,
    pub(crate) estimated_repair_cost_m: f64,
    pub(crate) total_debt_cost_m: f64,
    pub(crate) budget_basis: String,
    pub(crate) optimizer_penalty: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementRouteStateExclusionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementRouteStateExclusionRow {
    pub(crate) exclusion_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) source_title: String,
    pub(crate) source_url_or_cache_artifact: String,
    pub(crate) capture_date: String,
    pub(crate) excluded_member_count: usize,
    pub(crate) exclusion_basis: String,
    pub(crate) exclusion_status: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementRepairFundingAcceptanceRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementRepairFundingAcceptanceRow {
    pub(crate) acceptance_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) source_title: String,
    pub(crate) source_url_or_cache_artifact: String,
    pub(crate) capture_date: String,
    pub(crate) committed_amount_m: f64,
    pub(crate) covered_repair_cost_m: f64,
    pub(crate) funding_basis: String,
    pub(crate) acceptance_status: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementAcquisitionPlanRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementAcquisitionPlanRow {
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) source_family: String,
    pub(crate) route_count: usize,
    pub(crate) affected_routes: String,
    pub(crate) bundle_count: usize,
    pub(crate) affected_bundles: String,
    pub(crate) blocked_member_count: usize,
    pub(crate) source_priority: String,
    pub(crate) acquisition_action: String,
    pub(crate) required_fields: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementAcquisitionDocketRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementAcquisitionDocketRow {
    pub(crate) task_id: String,
    pub(crate) state: String,
    pub(crate) source_priority: String,
    pub(crate) affected_routes: String,
    pub(crate) affected_bundles: String,
    pub(crate) blocked_member_count: usize,
    pub(crate) fetch_command: String,
    pub(crate) rebuild_command: String,
    pub(crate) verify_command: String,
    pub(crate) source_contract: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementSourceAccessRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementSourceAccessRow {
    pub(crate) access_policy_id: String,
    pub(crate) task_id: String,
    pub(crate) state: String,
    pub(crate) source_priority: String,
    pub(crate) source_access_mode: String,
    pub(crate) mutation_mode: String,
    pub(crate) cache_targets: String,
    pub(crate) fetch_command: String,
    pub(crate) preflight_gate: String,
    pub(crate) postfetch_gate: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementSourceFetchAttemptRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementSourceFetchAttemptRow {
    pub(crate) fetch_attempt_id: String,
    pub(crate) access_policy_id: String,
    pub(crate) task_id: String,
    pub(crate) state: String,
    pub(crate) source_priority: String,
    pub(crate) fetch_command: String,
    pub(crate) cache_target: String,
    pub(crate) cache_record_count: usize,
    pub(crate) fetch_result_status: String,
    pub(crate) evidence_acceptance_status: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementSourceFetchReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementSourceFetchReviewRow {
    pub(crate) review_id: String,
    pub(crate) fetch_attempt_id: String,
    pub(crate) task_id: String,
    pub(crate) state: String,
    pub(crate) source_priority: String,
    pub(crate) cache_record_count: usize,
    pub(crate) fetch_result_status: String,
    pub(crate) pre_review_blocked_member_count: usize,
    pub(crate) postfetch_unresolved_member_count: usize,
    pub(crate) join_review_status: String,
    pub(crate) evidence_acceptance_status: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementUnmatchedJoinReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementUnmatchedJoinReviewRow {
    pub(crate) join_review_id: String,
    pub(crate) state: String,
    pub(crate) source_priority: String,
    pub(crate) cache_record_count: usize,
    pub(crate) source_gap_member_count: usize,
    pub(crate) source_needed_member_count: usize,
    pub(crate) repair_required_member_count: usize,
    pub(crate) source_needed_routes: String,
    pub(crate) repair_required_routes: String,
    pub(crate) hpms_records_for_source_needed_routes: usize,
    pub(crate) hpms_source_route_coverage: String,
    pub(crate) join_review_status: String,
    pub(crate) evidence_acceptance_status: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementHpmsScopeBroadeningRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementHpmsScopeBroadeningRow {
    pub(crate) broadening_id: String,
    pub(crate) state: String,
    pub(crate) source_priority: String,
    pub(crate) source_needed_routes: String,
    pub(crate) source_needed_member_count: usize,
    pub(crate) current_hpms_records_for_source_needed_routes: usize,
    pub(crate) current_coverage_status: String,
    pub(crate) broadened_functional_systems: String,
    pub(crate) broadened_fetch_command: String,
    pub(crate) preflight_gate: String,
    pub(crate) postfetch_gate: String,
    pub(crate) evidence_acceptance_status: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementRepairDebtReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementRepairDebtReviewRow {
    pub(crate) repair_review_id: String,
    pub(crate) state: String,
    pub(crate) source_priority: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) stitch_group_id: String,
    pub(crate) blocked_member_count: usize,
    pub(crate) repair_debt_units: usize,
    pub(crate) estimated_repair_cost_m: f64,
    pub(crate) repair_debt_status: String,
    pub(crate) repair_decision: String,
    pub(crate) evidence_acceptance_status: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementRepairDispositionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementRepairDispositionRow {
    pub(crate) disposition_id: String,
    pub(crate) repair_review_id: String,
    pub(crate) state: String,
    pub(crate) source_priority: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) blocked_member_count: usize,
    pub(crate) estimated_repair_cost_m: f64,
    pub(crate) disposition: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementRepairFundingPackageRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementRepairFundingPackageRow {
    pub(crate) funding_package_id: String,
    pub(crate) disposition_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) blocked_member_count: usize,
    pub(crate) estimated_repair_cost_m: f64,
    pub(crate) funding_package_status: String,
    pub(crate) funding_commitment_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingCommitmentReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingCommitmentReviewRow {
    pub(crate) commitment_review_id: String,
    pub(crate) funding_package_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) estimated_repair_cost_m: f64,
    pub(crate) funding_commitment_status: String,
    pub(crate) accepted_commitment_artifact: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementDowngradeExclusionDecisionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementDowngradeExclusionDecisionRow {
    pub(crate) downgrade_exclusion_decision_id: String,
    pub(crate) commitment_review_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) estimated_repair_cost_m: f64,
    pub(crate) downgrade_decision: String,
    pub(crate) exclusion_decision: String,
    pub(crate) service_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceContractRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceContractRow {
    pub(crate) evidence_contract_id: String,
    pub(crate) downgrade_exclusion_decision_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) estimated_repair_cost_m: f64,
    pub(crate) required_evidence: String,
    pub(crate) minimum_commitment_amount_m: f64,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceSourceCaptureRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceSourceCaptureRow {
    pub(crate) source_capture_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) minimum_commitment_amount_m: f64,
    pub(crate) source_capture_status: String,
    pub(crate) captured_artifact: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceArtifactAttachmentRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceArtifactAttachmentRow {
    pub(crate) artifact_attachment_id: String,
    pub(crate) source_capture_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) minimum_commitment_amount_m: f64,
    pub(crate) attachment_status: String,
    pub(crate) attached_artifact: String,
    pub(crate) evidence_review_status: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) attachment_blocker: String,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceReviewDocketRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceReviewDocketRow {
    pub(crate) funding_evidence_review_id: String,
    pub(crate) artifact_attachment_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) minimum_commitment_amount_m: f64,
    pub(crate) attached_artifact: String,
    pub(crate) review_decision: String,
    pub(crate) evidence_review_status: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) review_reason: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceAcquisitionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceAcquisitionRow {
    pub(crate) funding_evidence_acquisition_id: String,
    pub(crate) funding_evidence_review_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) minimum_commitment_amount_m: f64,
    pub(crate) required_artifact_type: String,
    pub(crate) acquisition_status: String,
    pub(crate) candidate_source_owner: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) acquisition_reason: String,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceSourceAccessRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceSourceAccessRow {
    pub(crate) source_access_id: String,
    pub(crate) funding_evidence_acquisition_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) source_owner: String,
    pub(crate) access_mode: String,
    pub(crate) live_fetch_status: String,
    pub(crate) required_source_metadata: String,
    pub(crate) cache_policy_artifact: String,
    pub(crate) source_access_blocker: String,
    pub(crate) evidence_artifact: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceIntakeRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceIntakeRow {
    pub(crate) funding_evidence_intake_id: String,
    pub(crate) source_access_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) required_artifact_type: String,
    pub(crate) required_source_metadata: String,
    pub(crate) intake_status: String,
    pub(crate) evidence_artifact: String,
    pub(crate) evidence_review_status: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) intake_blocker: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceMetadataCaptureRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceMetadataCaptureRow {
    pub(crate) metadata_capture_id: String,
    pub(crate) funding_evidence_intake_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) required_artifact_type: String,
    pub(crate) metadata_capture_status: String,
    pub(crate) captured_artifact: String,
    pub(crate) captured_source_title: String,
    pub(crate) captured_source_url: String,
    pub(crate) captured_commitment_amount_m: String,
    pub(crate) evidence_review_status: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceAcceptedArtifactAttachmentRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceAcceptedArtifactAttachmentRow {
    pub(crate) accepted_artifact_attachment_id: String,
    pub(crate) metadata_capture_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) required_artifact_type: String,
    pub(crate) attachment_status: String,
    pub(crate) attached_artifact: String,
    pub(crate) captured_source_title: String,
    pub(crate) captured_source_url: String,
    pub(crate) captured_commitment_amount_m: String,
    pub(crate) evidence_review_status: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) attachment_blocker: String,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceAcceptedAttachmentReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceAcceptedAttachmentReviewRow {
    pub(crate) accepted_attachment_review_id: String,
    pub(crate) accepted_artifact_attachment_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) required_artifact_type: String,
    pub(crate) attached_artifact: String,
    pub(crate) review_decision: String,
    pub(crate) evidence_review_status: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) review_reason: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceAcceptedArtifactAcquisitionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceAcceptedArtifactAcquisitionRow {
    pub(crate) accepted_artifact_acquisition_id: String,
    pub(crate) accepted_attachment_review_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) required_artifact_type: String,
    pub(crate) acquisition_status: String,
    pub(crate) cache_status: String,
    pub(crate) candidate_source_owner: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) acquisition_reason: String,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceAcceptedSourceAccessRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceAcceptedSourceAccessRow {
    pub(crate) accepted_source_access_id: String,
    pub(crate) accepted_artifact_acquisition_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) source_owner: String,
    pub(crate) access_mode: String,
    pub(crate) cache_status: String,
    pub(crate) live_fetch_status: String,
    pub(crate) required_source_metadata: String,
    pub(crate) cache_policy_artifact: String,
    pub(crate) source_access_blocker: String,
    pub(crate) evidence_artifact: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceAcceptedIntakeRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceAcceptedIntakeRow {
    pub(crate) accepted_intake_id: String,
    pub(crate) accepted_source_access_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) required_artifact_type: String,
    pub(crate) required_source_metadata: String,
    pub(crate) intake_status: String,
    pub(crate) cache_status: String,
    pub(crate) evidence_artifact: String,
    pub(crate) evidence_review_status: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) intake_blocker: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceAcceptedMetadataCaptureRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceAcceptedMetadataCaptureRow {
    pub(crate) accepted_metadata_capture_id: String,
    pub(crate) accepted_intake_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) required_artifact_type: String,
    pub(crate) metadata_capture_status: String,
    pub(crate) captured_artifact: String,
    pub(crate) captured_source_title: String,
    pub(crate) captured_source_url: String,
    pub(crate) captured_commitment_amount_m: String,
    pub(crate) evidence_review_status: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceAcceptedMetadataArtifactAttachmentRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceAcceptedMetadataArtifactAttachmentRow {
    pub(crate) accepted_metadata_artifact_attachment_id: String,
    pub(crate) accepted_metadata_capture_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) required_artifact_type: String,
    pub(crate) attachment_status: String,
    pub(crate) attached_artifact: String,
    pub(crate) captured_source_title: String,
    pub(crate) captured_source_url: String,
    pub(crate) captured_commitment_amount_m: String,
    pub(crate) evidence_review_status: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) attachment_blocker: String,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceAcceptedMetadataAttachmentReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceAcceptedMetadataAttachmentReviewRow {
    pub(crate) accepted_metadata_attachment_review_id: String,
    pub(crate) accepted_metadata_artifact_attachment_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) required_artifact_type: String,
    pub(crate) attached_artifact: String,
    pub(crate) review_decision: String,
    pub(crate) evidence_review_status: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) review_reason: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceAcceptedMetadataArtifactAcquisitionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceAcceptedMetadataArtifactAcquisitionRow {
    pub(crate) accepted_metadata_artifact_acquisition_id: String,
    pub(crate) accepted_metadata_attachment_review_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) required_artifact_type: String,
    pub(crate) acquisition_status: String,
    pub(crate) cache_status: String,
    pub(crate) candidate_source_owner: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) acquisition_reason: String,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceAcceptedMetadataSourceAccessRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceAcceptedMetadataSourceAccessRow {
    pub(crate) accepted_metadata_source_access_id: String,
    pub(crate) accepted_metadata_artifact_acquisition_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) source_owner: String,
    pub(crate) access_mode: String,
    pub(crate) cache_status: String,
    pub(crate) live_fetch_status: String,
    pub(crate) required_source_metadata: String,
    pub(crate) cache_policy_artifact: String,
    pub(crate) source_access_blocker: String,
    pub(crate) evidence_artifact: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceAcceptedMetadataIntakeRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceAcceptedMetadataIntakeRow {
    pub(crate) accepted_metadata_intake_id: String,
    pub(crate) accepted_metadata_source_access_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) required_artifact_type: String,
    pub(crate) required_source_metadata: String,
    pub(crate) intake_status: String,
    pub(crate) cache_status: String,
    pub(crate) evidence_artifact: String,
    pub(crate) evidence_review_status: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) intake_blocker: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceAcceptedMetadataSourceCaptureRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceAcceptedMetadataSourceCaptureRow {
    pub(crate) accepted_metadata_source_capture_id: String,
    pub(crate) accepted_metadata_intake_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) required_artifact_type: String,
    pub(crate) required_source_metadata: String,
    pub(crate) source_capture_status: String,
    pub(crate) captured_artifact: String,
    pub(crate) captured_source_title: String,
    pub(crate) captured_source_url: String,
    pub(crate) captured_commitment_amount_m: String,
    pub(crate) evidence_review_status: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementFundingEvidenceAcceptedMetadataSourceCaptureArtifactAttachmentRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierPavementFundingEvidenceAcceptedMetadataSourceCaptureArtifactAttachmentRow {
    pub(crate) accepted_metadata_source_capture_artifact_attachment_id: String,
    pub(crate) accepted_metadata_source_capture_id: String,
    pub(crate) evidence_contract_id: String,
    pub(crate) state: String,
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) required_artifact_type: String,
    pub(crate) attachment_status: String,
    pub(crate) attached_artifact: String,
    pub(crate) captured_source_title: String,
    pub(crate) captured_source_url: String,
    pub(crate) captured_commitment_amount_m: String,
    pub(crate) evidence_review_status: String,
    pub(crate) accepted_evidence_status: String,
    pub(crate) relief_eligibility: String,
    pub(crate) blocked_claims_before: String,
    pub(crate) blocked_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) attachment_blocker: String,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierTableScoreRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct TierTableScoreRow {
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) score: f64,
    pub(crate) confidence: f64,
    pub(crate) confidence_label: String,
}

// --- struct LowerTierPressureWitnessRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct LowerTierPressureWitnessRow {
    pub(crate) route: String,
    pub(crate) current_tier: String,
    pub(crate) current_score: f64,
    pub(crate) confidence: f64,
    pub(crate) confidence_label: String,
    pub(crate) pressure_type: String,
    pub(crate) witness_action: String,
    pub(crate) target_tier: String,
    pub(crate) selection_basis: String,
    pub(crate) source_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T3T4PressureIntakeRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T3T4PressureIntakeRow {
    pub(crate) route: String,
    pub(crate) source_pressure_type: String,
    pub(crate) current_tier: String,
    pub(crate) current_score: f64,
    pub(crate) target_tier: String,
    pub(crate) intake_class: String,
    pub(crate) intake_action: String,
    pub(crate) selection_basis: String,
    pub(crate) source_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct T3ZoneAccessObligationRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T3ZoneAccessObligationRow {
    pub(crate) zone_id: String,
    pub(crate) zone_name: String,
    pub(crate) obligation_class: String,
    pub(crate) access_target: String,
    pub(crate) promise_horizon_hours: u8,
    pub(crate) source_route_count: usize,
    pub(crate) candidate_routes: String,
    pub(crate) source_intake_classes: String,
    pub(crate) map_id: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct T3ZoneRouteColumnRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T3ZoneRouteColumnRow {
    pub(crate) zone_id: String,
    pub(crate) zone_name: String,
    pub(crate) obligation_class: String,
    pub(crate) route: String,
    pub(crate) current_tier: String,
    pub(crate) current_score: f64,
    pub(crate) constraint_adjusted_score: f64,
    pub(crate) hard_blocker_count: usize,
    pub(crate) claim_blocker_count: usize,
    pub(crate) constraint_debt_cost_m: f64,
    pub(crate) lifecycle_debt_cost_m: f64,
    pub(crate) constraint_penalty_score: f64,
    pub(crate) top_constraint_classes: String,
    pub(crate) constraint_ledger_artifact: String,
    pub(crate) promise_horizon_hours: u8,
    pub(crate) column_decision: String,
    pub(crate) zone_role: String,
    pub(crate) contact_requirement: String,
    pub(crate) map_treatment: String,
    pub(crate) selection_basis: String,
    pub(crate) source_obligation: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalAccessColumnRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalAccessColumnRow {
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) current_score: f64,
    pub(crate) constraint_adjusted_score: f64,
    pub(crate) hard_blocker_count: usize,
    pub(crate) claim_blocker_count: usize,
    pub(crate) constraint_debt_cost_m: f64,
    pub(crate) lifecycle_debt_cost_m: f64,
    pub(crate) constraint_penalty_score: f64,
    pub(crate) top_constraint_classes: String,
    pub(crate) constraint_ledger_artifact: String,
    pub(crate) access_class: String,
    pub(crate) terminal_obligation: String,
    pub(crate) promise_horizon_hours: u8,
    pub(crate) column_decision: String,
    pub(crate) evidence_required: String,
    pub(crate) map_treatment: String,
    pub(crate) selection_basis: String,
    pub(crate) source_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalContactEvidenceRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalContactEvidenceRow {
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) terminal_district_seed: String,
    pub(crate) terminal_district_seed_source: String,
    pub(crate) contact_basis: String,
    pub(crate) contact_proof_source: String,
    pub(crate) evidence_status: String,
    pub(crate) selected_higher_tier_attachment: String,
    pub(crate) decision: String,
    pub(crate) next_artifact: String,
    pub(crate) source_column_artifact: String,
    pub(crate) source_column_decision: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalAccessEvidenceReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalAccessEvidenceReviewRow {
    pub(crate) review_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) terminal_district_seed: String,
    pub(crate) terminal_district_seed_source: String,
    pub(crate) evidence_status_before: String,
    pub(crate) review_decision: String,
    pub(crate) review_reason: String,
    pub(crate) source_action: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalScenarioReadinessRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalScenarioReadinessRow {
    pub(crate) docket_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) terminal_district: String,
    pub(crate) contact_basis: String,
    pub(crate) contact_proof_source: String,
    pub(crate) selected_higher_tier_attachment: String,
    pub(crate) freight_access_rationale: String,
    pub(crate) scenario_decision: String,
    pub(crate) scenario_artifact: String,
    pub(crate) source_evidence_status: String,
    pub(crate) release_status: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalContactSourcePlanRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalContactSourcePlanRow {
    pub(crate) plan_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) terminal_district: String,
    pub(crate) terminal_district_seed_source: String,
    pub(crate) contact_proof_source_family: String,
    pub(crate) contact_proof_source_artifact: String,
    pub(crate) required_proof_fields: String,
    pub(crate) acquisition_status: String,
    pub(crate) proof_blocker: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalAccessProofAcquisitionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalAccessProofAcquisitionRow {
    pub(crate) acquisition_id: String,
    pub(crate) review_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) terminal_district_seed: String,
    pub(crate) required_proof: String,
    pub(crate) prohibited_seed_source: String,
    pub(crate) acquisition_status: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) proof_artifact_status: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalAccessProofArtifactRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalAccessProofArtifactRow {
    pub(crate) proof_artifact_id: String,
    pub(crate) acquisition_id: String,
    pub(crate) review_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) terminal_district_seed: String,
    pub(crate) required_proof: String,
    pub(crate) source_artifact_reference: String,
    pub(crate) attachment_status: String,
    pub(crate) evidence_review_status: String,
    pub(crate) proof_acceptance_status: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalAccessProofReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalAccessProofReviewRow {
    pub(crate) proof_review_id: String,
    pub(crate) proof_artifact_id: String,
    pub(crate) acquisition_id: String,
    pub(crate) review_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) source_artifact_reference: String,
    pub(crate) review_decision: String,
    pub(crate) proof_acceptance_status: String,
    pub(crate) optimization_return_status: String,
    pub(crate) review_reason: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalAccessSourceAccessRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalAccessSourceAccessRow {
    pub(crate) source_access_id: String,
    pub(crate) proof_review_id: String,
    pub(crate) proof_artifact_id: String,
    pub(crate) acquisition_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) source_owner: String,
    pub(crate) access_mode: String,
    pub(crate) live_fetch_status: String,
    pub(crate) required_source_metadata: String,
    pub(crate) cache_policy_artifact: String,
    pub(crate) source_access_blocker: String,
    pub(crate) evidence_artifact: String,
    pub(crate) proof_acceptance_status: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalAccessProofIntakeRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalAccessProofIntakeRow {
    pub(crate) proof_intake_id: String,
    pub(crate) source_access_id: String,
    pub(crate) proof_review_id: String,
    pub(crate) proof_artifact_id: String,
    pub(crate) acquisition_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) required_artifact_fields: String,
    pub(crate) required_contact_statement: String,
    pub(crate) proof_artifact: String,
    pub(crate) proof_status: String,
    pub(crate) proof_blocker: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalAccessProofSourceCaptureRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalAccessProofSourceCaptureRow {
    pub(crate) source_capture_id: String,
    pub(crate) proof_intake_id: String,
    pub(crate) source_access_id: String,
    pub(crate) proof_artifact_id: String,
    pub(crate) acquisition_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) source_artifact_reference: String,
    pub(crate) source_artifact_type: String,
    pub(crate) capture_status: String,
    pub(crate) evidence_acceptance_status: String,
    pub(crate) capture_blocker: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalAccessProofArtifactAttachmentRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalAccessProofArtifactAttachmentRow {
    pub(crate) artifact_attachment_id: String,
    pub(crate) source_capture_id: String,
    pub(crate) proof_intake_id: String,
    pub(crate) proof_artifact_id: String,
    pub(crate) acquisition_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) source_artifact_reference: String,
    pub(crate) attachment_status: String,
    pub(crate) evidence_review_status: String,
    pub(crate) proof_acceptance_status: String,
    pub(crate) attachment_blocker: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalAccessProofAttachmentReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalAccessProofAttachmentReviewRow {
    pub(crate) attachment_review_id: String,
    pub(crate) artifact_attachment_id: String,
    pub(crate) source_capture_id: String,
    pub(crate) proof_intake_id: String,
    pub(crate) proof_artifact_id: String,
    pub(crate) acquisition_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) source_artifact_reference: String,
    pub(crate) review_decision: String,
    pub(crate) evidence_review_status: String,
    pub(crate) proof_acceptance_status: String,
    pub(crate) optimization_return_status: String,
    pub(crate) review_reason: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalAccessProofArtifactAcquisitionTargetRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalAccessProofArtifactAcquisitionTargetRow {
    pub(crate) acquisition_target_id: String,
    pub(crate) attachment_review_id: String,
    pub(crate) artifact_attachment_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) candidate_source_owner: String,
    pub(crate) required_artifact_fields: String,
    pub(crate) prohibited_seed_source: String,
    pub(crate) acquisition_status: String,
    pub(crate) cache_status: String,
    pub(crate) source_artifact_reference: String,
    pub(crate) proof_acceptance_status: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalAccessProofArtifactSourceAccessRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalAccessProofArtifactSourceAccessRow {
    pub(crate) source_access_id: String,
    pub(crate) acquisition_target_id: String,
    pub(crate) attachment_review_id: String,
    pub(crate) artifact_attachment_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) source_owner: String,
    pub(crate) access_mode: String,
    pub(crate) cache_status: String,
    pub(crate) live_fetch_status: String,
    pub(crate) required_source_metadata: String,
    pub(crate) cache_policy_artifact: String,
    pub(crate) source_access_blocker: String,
    pub(crate) evidence_artifact: String,
    pub(crate) proof_acceptance_status: String,
    pub(crate) blocker_claims_before: String,
    pub(crate) blocker_claims_after: String,
    pub(crate) claim_blocker_delta: isize,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalContactSourceCatalogRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalContactSourceCatalogRow {
    pub(crate) catalog_id: String,
    pub(crate) terminal_district: String,
    pub(crate) route_task_count: usize,
    pub(crate) source_family: String,
    pub(crate) source_access_mode: String,
    pub(crate) required_proof_fields: String,
    pub(crate) acquisition_status: String,
    pub(crate) proof_blocker: String,
    pub(crate) cache_policy_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalContactProofArtifactContractRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalContactProofArtifactContractRow {
    pub(crate) contract_id: String,
    pub(crate) source_family: String,
    pub(crate) accepted_proof_status: String,
    pub(crate) required_fields: String,
    pub(crate) allowed_artifact_modes: String,
    pub(crate) prohibited_sources: String,
    pub(crate) promotion_rule: String,
    pub(crate) source_needed_decision: String,
    pub(crate) blocked_decision: String,
    pub(crate) rejected_decision: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalContactProofSourceRegistryRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalContactProofSourceRegistryRow {
    pub(crate) registry_id: String,
    pub(crate) task_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) terminal_district: String,
    pub(crate) source_family: String,
    pub(crate) source_artifact_mode: String,
    pub(crate) source_title: String,
    pub(crate) source_url_or_cache_artifact: String,
    pub(crate) capture_date: String,
    pub(crate) contact_statement_status: String,
    pub(crate) selected_higher_tier_attachment_status: String,
    pub(crate) registry_status: String,
    pub(crate) proof_source_artifact: String,
    pub(crate) registry_blocker: String,
    pub(crate) contract_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalContactAcceptedProofSourceRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalContactAcceptedProofSourceRow {
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) terminal_district: String,
    pub(crate) source_artifact_mode: String,
    pub(crate) source_title: String,
    pub(crate) source_url_or_cache_artifact: String,
    pub(crate) capture_date: String,
    pub(crate) contact_statement: String,
    pub(crate) selected_higher_tier_attachment: String,
    pub(crate) proof_source_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalContactRejectedProofSourceRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalContactRejectedProofSourceRow {
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) terminal_district: String,
    pub(crate) source_title: String,
    pub(crate) source_url_or_cache_artifact: String,
    pub(crate) capture_date: String,
    pub(crate) listed_terminal_access_routes: String,
    pub(crate) rejection_basis: String,
    pub(crate) rejection_status: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalContactDistrictProofImportRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalContactDistrictProofImportRow {
    pub(crate) import_id: String,
    pub(crate) registry_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) terminal_district: String,
    pub(crate) source_artifact_mode: String,
    pub(crate) proof_source_artifact: String,
    pub(crate) contact_statement_status: String,
    pub(crate) selected_higher_tier_attachment_status: String,
    pub(crate) import_status: String,
    pub(crate) proof_decision: String,
    pub(crate) import_blocker: String,
    pub(crate) selection_rule: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalContactProofDocketRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalContactProofDocketRow {
    pub(crate) task_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) terminal_district: String,
    pub(crate) source_family: String,
    pub(crate) required_proof_field: String,
    pub(crate) selected_higher_tier_attachment_requirement: String,
    pub(crate) contact_proof_source_artifact: String,
    pub(crate) proof_status: String,
    pub(crate) proof_blocker: String,
    pub(crate) scenario_effect: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalColumbusProofIntakeRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalColumbusProofIntakeRow {
    pub(crate) intake_id: String,
    pub(crate) task_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) terminal_district: String,
    pub(crate) source_family: String,
    pub(crate) required_proof_field: String,
    pub(crate) selected_higher_tier_attachment_requirement: String,
    pub(crate) contact_proof_source_artifact: String,
    pub(crate) proof_status: String,
    pub(crate) proof_blocker: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalColumbusSourceAccessRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalColumbusSourceAccessRow {
    pub(crate) access_id: String,
    pub(crate) intake_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) terminal_district: String,
    pub(crate) source_family: String,
    pub(crate) access_mode: String,
    pub(crate) live_fetch_status: String,
    pub(crate) required_source_metadata: String,
    pub(crate) contact_proof_source_artifact: String,
    pub(crate) acquisition_status: String,
    pub(crate) source_access_blocker: String,
    pub(crate) cache_policy_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalColumbusProofAttemptRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalColumbusProofAttemptRow {
    pub(crate) attempt_id: String,
    pub(crate) access_id: String,
    pub(crate) intake_id: String,
    pub(crate) queue_id: String,
    pub(crate) route: String,
    pub(crate) terminal_district: String,
    pub(crate) source_family: String,
    pub(crate) source_artifact: String,
    pub(crate) capture_status: String,
    pub(crate) contact_statement_status: String,
    pub(crate) selected_higher_tier_attachment_status: String,
    pub(crate) proof_attempt_status: String,
    pub(crate) proof_decision: String,
    pub(crate) proof_blocker: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T3T4AccessGapRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T3T4AccessGapRow {
    pub(crate) gap_id: String,
    pub(crate) source_surface: String,
    pub(crate) route: String,
    pub(crate) zone_id: String,
    pub(crate) current_score: f64,
    pub(crate) constraint_adjusted_score: f64,
    pub(crate) hard_blocker_count: usize,
    pub(crate) claim_blocker_count: usize,
    pub(crate) constraint_debt_cost_m: f64,
    pub(crate) lifecycle_debt_cost_m: f64,
    pub(crate) constraint_penalty_score: f64,
    pub(crate) top_constraint_classes: String,
    pub(crate) constraint_ledger_artifact: String,
    pub(crate) promise_horizon_hours: u8,
    pub(crate) gap_class: String,
    pub(crate) gap_reason: String,
    pub(crate) required_evidence: String,
    pub(crate) repair_action: String,
    pub(crate) next_artifact: String,
    pub(crate) upward_pressure_allowed: bool,
    pub(crate) validation_status: String,
}

// --- struct T4TerminalAccessMapExclusionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T4TerminalAccessMapExclusionRow {
    pub(crate) decision_id: String,
    pub(crate) decision_scope: String,
    pub(crate) source_artifact: String,
    pub(crate) affected_constraint_class: String,
    pub(crate) affected_gap_class: String,
    pub(crate) affected_tier: String,
    pub(crate) affected_claims_before: String,
    pub(crate) excluded_claims: String,
    pub(crate) preserved_claims_after: String,
    pub(crate) affected_route_count: usize,
    pub(crate) decision: String,
    pub(crate) decision_basis: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T3ZoneMapDiagnosticRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T3ZoneMapDiagnosticRow {
    pub(crate) zone_id: String,
    pub(crate) zone_name: String,
    pub(crate) map_id: String,
    pub(crate) map_path: String,
    pub(crate) selected_route_count: usize,
    pub(crate) selected_routes: String,
    pub(crate) review_connector_count: usize,
    pub(crate) review_connectors: String,
    pub(crate) access_gap_count: usize,
    pub(crate) below_threshold_feeder_count: usize,
    pub(crate) terminal_evidence_gap_count: usize,
    pub(crate) zone_assignment_gap_count: usize,
    pub(crate) map_readiness: String,
    pub(crate) diagnostic_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T3ZoneRenderBoardRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T3ZoneRenderBoardRow {
    pub(crate) zone_id: String,
    pub(crate) zone_name: String,
    pub(crate) map_id: String,
    pub(crate) map_path: String,
    pub(crate) board_layer: String,
    pub(crate) route: String,
    pub(crate) national_segment_id: String,
    pub(crate) stitch_group_id: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) segment_aliases: String,
    pub(crate) route_status: String,
    pub(crate) map_treatment: String,
    pub(crate) selected_route_count: usize,
    pub(crate) access_gap_count: usize,
    pub(crate) source_artifact: String,
    pub(crate) render_action: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T3ZoneStopPlacementRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T3ZoneStopPlacementRow {
    pub(crate) zone_id: String,
    pub(crate) zone_name: String,
    pub(crate) route: String,
    pub(crate) national_segment_id: String,
    pub(crate) stitch_group_id: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) segment_aliases: String,
    pub(crate) state_scope: String,
    pub(crate) stop_count: usize,
    pub(crate) transfer_grade_stop_count: usize,
    pub(crate) stop_chain: String,
    pub(crate) stop_classes: String,
    pub(crate) placement_status: String,
    pub(crate) placement_action: String,
    pub(crate) source_artifact: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct NationalSegmentRegistryRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct NationalSegmentRegistryRow {
    pub(crate) national_segment_id: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) stitch_group_id: String,
    pub(crate) current_zone_id: String,
    pub(crate) current_tier: String,
    pub(crate) route_label: String,
    pub(crate) zone_id: String,
    pub(crate) route: String,
    pub(crate) state_scope: String,
    pub(crate) evidence_state_scope: String,
    pub(crate) geometry_state_scope: String,
    pub(crate) segment_aliases: String,
    pub(crate) bundle_aliases: String,
    pub(crate) board_layers: String,
    pub(crate) source_artifacts: String,
    pub(crate) stop_placement_status: String,
    pub(crate) bundle_role: String,
    pub(crate) member_segment_ids: String,
    pub(crate) registry_action: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) validation_status: String,
}

// --- struct NationalSegmentBundleRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct NationalSegmentBundleRow {
    pub(crate) segment_bundle_id: String,
    pub(crate) bundle_role: String,
    pub(crate) member_segment_ids: String,
    pub(crate) member_count: usize,
    pub(crate) stitch_group_ids: String,
    pub(crate) current_tiers: String,
    pub(crate) current_zone_ids: String,
    pub(crate) route_labels: String,
    pub(crate) state_scope: String,
    pub(crate) evidence_state_scope: String,
    pub(crate) geometry_state_scope: String,
    pub(crate) bundle_aliases: String,
    pub(crate) source_artifacts: String,
    pub(crate) bundle_status: String,
    pub(crate) bundle_action: String,
    #[serde(default)]
    pub(crate) qualification_effects: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct T2BubbleUpReviewRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2BubbleUpReviewRow {
    pub(crate) route: String,
    pub(crate) source_intake_class: String,
    pub(crate) current_score: f64,
    pub(crate) review_action: String,
    pub(crate) required_evidence: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct T1FeedbackDocketRow ---
#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct T1FeedbackDocketRow {
    pub(crate) route: String,
    pub(crate) source_surface: String,
    pub(crate) source_action: String,
    pub(crate) current_score: f64,
    pub(crate) t1_feedback_class: String,
    pub(crate) t1_feedback_action: String,
    pub(crate) t1_sla_pair_count: usize,
    pub(crate) t1_sla_pairs: String,
    pub(crate) required_evidence: String,
    pub(crate) next_artifact: String,
    pub(crate) optimizer_effect: String,
    pub(crate) validation_status: String,
}

// --- struct TierOptimizerRunRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct TierOptimizerRunRow {
    pub(crate) step: usize,
    pub(crate) optimizer_stage: String,
    pub(crate) command: String,
    pub(crate) artifact: String,
    pub(crate) row_count: usize,
    pub(crate) gate_status: String,
    pub(crate) blocker_count: usize,
    pub(crate) blocker_summary: String,
    pub(crate) validation_status: String,
}

// --- struct SourceFetchPolicyRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct SourceFetchPolicyRow {
    pub(crate) fetch_family: String,
    pub(crate) commands: String,
    pub(crate) cache_targets: String,
    pub(crate) mutation_mode: String,
    pub(crate) preservation_contract: String,
    pub(crate) implementation_guard: String,
    pub(crate) validation_floor: String,
    pub(crate) policy_doc: String,
    pub(crate) validation_status: String,
}

// --- struct T2AssetConditionMapPublicationExclusionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T2AssetConditionMapPublicationExclusionRow {
    pub(crate) decision_id: String,
    pub(crate) decision_scope: String,
    pub(crate) source_artifact: String,
    pub(crate) affected_constraint_class: String,
    pub(crate) affected_tier: String,
    pub(crate) affected_claims_before: String,
    pub(crate) excluded_claims: String,
    pub(crate) preserved_claims_after: String,
    pub(crate) affected_bundle_count: u32,
    pub(crate) total_debt_cost_m: f64,
    pub(crate) decision: String,
    pub(crate) decision_basis: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct SourceSnapshotPublicationExclusionRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct SourceSnapshotPublicationExclusionRow {
    pub(crate) decision_id: String,
    pub(crate) decision_scope: String,
    pub(crate) source_artifact: String,
    pub(crate) affected_constraint_class: String,
    pub(crate) affected_fetch_family: String,
    pub(crate) affected_claims_before: String,
    pub(crate) excluded_claims: String,
    pub(crate) preserved_claims_after: String,
    pub(crate) decision: String,
    pub(crate) decision_basis: String,
    pub(crate) next_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct OptimizerMapHookRow ---
#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct OptimizerMapHookRow {
    pub(crate) hook_id: String,
    pub(crate) optimizer_artifact: String,
    pub(crate) consumer_artifact: String,
    pub(crate) consumer_type: String,
    pub(crate) gate_command: String,
    pub(crate) link_basis: String,
    pub(crate) validation_status: String,
}

// --- struct BundleArchitectureRow ---
#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct BundleArchitectureRow {
    pub(crate) crate_name: String,
    pub(crate) role: String,
    pub(crate) bundle_entrypoint: String,
    pub(crate) source_path: String,
    pub(crate) required_tokens: String,
    pub(crate) architecture_status: String,
    pub(crate) next_action: String,
    pub(crate) validation_status: String,
}

// --- struct T2BeckContactWitness ---
pub(crate) struct T2BeckContactWitness {
    pub(crate) witness_type: String,
    pub(crate) node_class: String,
    pub(crate) observed_t1_node_count: usize,
    pub(crate) observed_parent_trunks: String,
    pub(crate) observed_dual_contacts: usize,
    pub(crate) repair_action: String,
    pub(crate) repair_basis: String,
    pub(crate) evidence_status: String,
    pub(crate) required_artifact: String,
    pub(crate) validation_status: String,
}

// --- struct TierPavementSourceGapBuilder ---
#[derive(Default)]
pub(crate) struct TierPavementSourceGapBuilder {
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) region_id: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) stitch_group_id: String,
    pub(crate) member_count: usize,
    pub(crate) blocker_count: usize,
    pub(crate) blocker_statuses: std::collections::BTreeSet<String>,
    pub(crate) affected_states: std::collections::BTreeSet<String>,
    pub(crate) affected_edge_ids: std::collections::BTreeSet<u64>,
    pub(crate) source_contracts: std::collections::BTreeSet<String>,
}

// --- struct PavementDebtBudgetIndex ---
#[derive(Debug, Clone, Default)]
pub(crate) struct PavementDebtBudgetIndex {
    pub(crate) by_bundle: std::collections::HashMap<String, TierPavementDebtBudgetRow>,
    pub(crate) by_route: std::collections::HashMap<String, TierPavementDebtBudgetRollup>,
}

// --- struct TierPavementDebtBudgetRollup ---
#[derive(Debug, Clone, Default)]
pub(crate) struct TierPavementDebtBudgetRollup {
    pub(crate) total_debt_cost_m: f64,
    pub(crate) debt_classes: std::collections::BTreeSet<String>,
    pub(crate) affected_bundles: std::collections::BTreeSet<String>,
}

// --- struct OptimizerConstraintBudgetIndex ---
#[derive(Debug, Clone, Default)]
pub(crate) struct OptimizerConstraintBudgetIndex {
    pub(crate) by_bundle: std::collections::HashMap<String, OptimizerConstraintBudgetRow>,
    pub(crate) by_route: std::collections::HashMap<String, OptimizerConstraintBudgetRollup>,
}

// --- struct OptimizerConstraintBudgetRollup ---
#[derive(Debug, Clone, Default)]
pub(crate) struct OptimizerConstraintBudgetRollup {
    pub(crate) hard_blocker_count: usize,
    pub(crate) claim_blocker_count: usize,
    pub(crate) constraint_debt_cost_m: f64,
    pub(crate) lifecycle_debt_cost_m: f64,
    pub(crate) constraint_penalty_score: f64,
    pub(crate) top_constraint_classes: std::collections::BTreeSet<String>,
    pub(crate) qualification_effects: std::collections::BTreeSet<String>,
    pub(crate) constraint_ledger_artifact: String,
}

// --- struct OptimizerConstraintBudgetBuilder ---
#[derive(Debug, Default)]
pub(crate) struct OptimizerConstraintBudgetBuilder {
    pub(crate) optimizer_run_id: String,
    pub(crate) tier: String,
    pub(crate) region_id: String,
    pub(crate) subject_scope: String,
    pub(crate) subject_id: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) route: String,
    pub(crate) ledger_row_count: usize,
    pub(crate) hard_blocker_count: usize,
    pub(crate) claim_blocker_count: usize,
    pub(crate) review_count: usize,
    pub(crate) budget_debt_count: usize,
    pub(crate) constraint_debt_cost_m: f64,
    pub(crate) lifecycle_debt_cost_m: f64,
    pub(crate) constraint_penalty_score: f64,
    pub(crate) class_counts: std::collections::BTreeMap<String, usize>,
    pub(crate) blocking_claims: std::collections::BTreeSet<String>,
    pub(crate) qualification_effects: std::collections::BTreeSet<String>,
    pub(crate) next_artifacts: std::collections::BTreeSet<String>,
}

// --- struct ResidualBacklogBuilder ---
#[derive(Debug, Clone, Default)]
pub(crate) struct ResidualBacklogBuilder {
    pub(crate) priority_class: String,
    pub(crate) blocker_family: String,
    pub(crate) tier: String,
    pub(crate) blocked_claims: std::collections::BTreeSet<String>,
    pub(crate) subject_ids: std::collections::BTreeSet<String>,
    pub(crate) routes: std::collections::BTreeSet<String>,
    pub(crate) total_hard_blockers: usize,
    pub(crate) total_claim_blockers: usize,
    pub(crate) total_budget_debt_count: usize,
    pub(crate) total_constraint_debt_cost_m: f64,
    pub(crate) total_constraint_penalty_score: f64,
    pub(crate) next_artifacts: std::collections::BTreeSet<String>,
    pub(crate) next_wave: String,
}

// --- struct T1SharedSegmentPolicyBuilder ---
#[derive(Default)]
pub(crate) struct T1SharedSegmentPolicyBuilder {
    pub(crate) routes: std::collections::BTreeSet<String>,
    pub(crate) source_review_ids: std::collections::BTreeSet<String>,
    pub(crate) blocker_claims: std::collections::BTreeSet<String>,
    pub(crate) blocker_count: usize,
    pub(crate) policy_basis: std::collections::BTreeSet<String>,
    pub(crate) design_treatments: std::collections::BTreeSet<String>,
}

// --- struct TierPavementAcquisitionBuilder ---
#[derive(Default)]
pub(crate) struct TierPavementAcquisitionBuilder {
    pub(crate) state: String,
    pub(crate) tiers: std::collections::BTreeSet<String>,
    pub(crate) routes: std::collections::BTreeSet<String>,
    pub(crate) bundles: std::collections::BTreeSet<String>,
    pub(crate) blocked_member_count: usize,
}

// --- struct NationalSegmentRegistryBuilder ---
#[derive(Default)]
pub(crate) struct NationalSegmentRegistryBuilder {
    pub(crate) national_segment_id: String,
    pub(crate) segment_bundle_id: String,
    pub(crate) bundle_role: String,
    pub(crate) stitch_group_id: String,
    pub(crate) zone_id: String,
    pub(crate) current_tier: String,
    pub(crate) route: String,
    pub(crate) evidence_state_scope: std::collections::BTreeSet<String>,
    pub(crate) geometry_state_scope: std::collections::BTreeSet<String>,
    pub(crate) segment_aliases: std::collections::BTreeSet<String>,
    pub(crate) bundle_aliases: std::collections::BTreeSet<String>,
    pub(crate) board_layers: std::collections::BTreeSet<String>,
    pub(crate) source_artifacts: std::collections::BTreeSet<String>,
    pub(crate) stop_placement_status: std::collections::BTreeSet<String>,
    pub(crate) qualification_effects: std::collections::BTreeSet<String>,
    pub(crate) validation_statuses: std::collections::BTreeSet<String>,
}

// --- struct EndpointExceptionRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct EndpointExceptionRow {
    pub(crate) route: String,
    pub(crate) requested_tier: String,
    pub(crate) endpoint_name: String,
    pub(crate) endpoint_role: String,
    pub(crate) exception_type: String,
    pub(crate) evidence_level: String,
    pub(crate) artifact: String,
    pub(crate) next_step: String,
}

// --- struct TierConnectivityGateFailure ---
#[derive(Debug)]
pub(crate) struct TierConnectivityGateFailure<'a> {
    pub(crate) row: &'a route_network::TierConnectivityRow,
    pub(crate) reason: String,
}

// --- struct StopCandidateRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct StopCandidateRow {
    pub(crate) stop_id: String,
    pub(crate) name: String,
    pub(crate) state: String,
    pub(crate) lat: String,
    pub(crate) lon: String,
    pub(crate) requested_class: String,
    pub(crate) route_refs: String,
    pub(crate) stop_role: String,
    pub(crate) transfer_value: String,
    pub(crate) freight_volume: String,
    pub(crate) spacing_need: String,
    pub(crate) resilience_value: String,
    pub(crate) energy_service: String,
    pub(crate) land_ops_feasibility: String,
    pub(crate) equity_community: String,
    pub(crate) evidence_status: String,
    pub(crate) source_artifact: String,
    pub(crate) next_step: String,
}

// --- struct T1LineSelectorRow ---
#[derive(Debug)]
pub(crate) struct T1LineSelectorRow {
    pub(crate) route: String,
    pub(crate) tier: String,
    pub(crate) score: f64,
    pub(crate) constraint_adjusted_score: f64,
    pub(crate) rank: usize,
    pub(crate) selected: bool,
    pub(crate) selected_stop_count: usize,
    pub(crate) top_city_stop_count: usize,
    pub(crate) sla_pair_count: usize,
    pub(crate) budget_cost: usize,
    pub(crate) hard_blocker_count: usize,
    pub(crate) claim_blocker_count: usize,
    pub(crate) constraint_debt_cost_m: f64,
    pub(crate) lifecycle_debt_cost_m: f64,
    pub(crate) constraint_penalty_score: f64,
    pub(crate) top_constraint_classes: String,
    pub(crate) constraint_ledger_artifact: String,
    pub(crate) decision: &'static str,
    pub(crate) reason: &'static str,
    pub(crate) selected_stops: String,
    pub(crate) top_city_stops: String,
    pub(crate) sla_pairs: String,
}

// --- struct T1SlaCandidateUniverseRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct T1SlaCandidateUniverseRow {
    pub(crate) pair_id: String,
    pub(crate) origin_id: String,
    pub(crate) dest_id: String,
    pub(crate) target_hours: f64,
    pub(crate) market_class: String,
    pub(crate) required_routes: String,
    pub(crate) required_stops: String,
    pub(crate) evidence_basis: String,
    pub(crate) market_score: f64,
    pub(crate) conversion_score: f64,
    pub(crate) coverage_score: f64,
    pub(crate) reuse_score: f64,
    pub(crate) resilience_score: f64,
    pub(crate) evidence_score: f64,
    pub(crate) budget_penalty: f64,
    pub(crate) drop_reason_hint: String,
    pub(crate) covered_by_selected_pair: String,
}

// --- struct T1SlaCandidatePairRow ---
#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct T1SlaCandidatePairRow {
    pub(crate) rank: usize,
    pub(crate) pair_id: String,
    pub(crate) origin_id: String,
    pub(crate) dest_id: String,
    pub(crate) target_hours: f64,
    pub(crate) market_class: String,
    pub(crate) total_score: f64,
    pub(crate) market_score: f64,
    pub(crate) conversion_score: f64,
    pub(crate) coverage_score: f64,
    pub(crate) reuse_score: f64,
    pub(crate) resilience_score: f64,
    pub(crate) evidence_score: f64,
    pub(crate) budget_penalty: f64,
    pub(crate) portfolio_selected: bool,
    pub(crate) selected_budget: usize,
    pub(crate) cutline_status: String,
    pub(crate) cutline_reason: String,
    pub(crate) covered_by_selected_pair: String,
    pub(crate) required_routes: String,
    pub(crate) required_stops: String,
    pub(crate) evidence_basis: String,
    pub(crate) validation_status: String,
}

// --- struct T1LineSelectorInputRow ---
#[derive(Debug, serde::Deserialize)]
pub(crate) struct T1LineSelectorInputRow {
    pub(crate) route: String,
    pub(crate) selected: bool,
    pub(crate) selected_stops: String,
}

// --- struct T1StopSelectorRow ---
#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct T1StopSelectorRow {
    pub(crate) route: String,
    pub(crate) stop_sequence: usize,
    pub(crate) stop_id: String,
    pub(crate) stop_name: String,
    pub(crate) requested_class: String,
    pub(crate) selector_weight: i32,
    pub(crate) split_objective: String,
    pub(crate) target_regions: usize,
    pub(crate) metis_region: usize,
    pub(crate) boundary_after: bool,
    pub(crate) evidence_status: String,
    pub(crate) validation_status: String,
}

// --- struct T1StopSelectorInputRow ---
#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
pub(crate) struct T1StopSelectorInputRow {
    pub(crate) route: String,
    pub(crate) stop_sequence: usize,
    pub(crate) stop_id: String,
    pub(crate) stop_name: String,
    pub(crate) requested_class: String,
    pub(crate) selector_weight: i32,
    pub(crate) split_objective: String,
    pub(crate) target_regions: usize,
    pub(crate) metis_region: usize,
    pub(crate) boundary_after: bool,
    pub(crate) evidence_status: String,
    pub(crate) validation_status: String,
}

// --- struct T1BeckAlignmentRow ---
#[derive(Debug, Clone, serde::Serialize)]
pub(crate) struct T1BeckAlignmentRow {
    pub(crate) route: String,
    pub(crate) selector_stop_count: usize,
    pub(crate) selector_boundary_count: usize,
    pub(crate) selector_regions: usize,
    pub(crate) beck_stop_count: usize,
    pub(crate) beck_drawn_stop_count: usize,
    pub(crate) beck_transfer_stop_count: usize,
    pub(crate) beck_action: String,
    pub(crate) beck_review_flag: String,
    pub(crate) alignment_status: String,
    pub(crate) validation_status: String,
}

// --- struct TierTableInputRow ---
#[derive(Debug, serde::Deserialize)]
pub(crate) struct TierTableInputRow {
    pub(crate) tier: String,
    pub(crate) route: String,
    pub(crate) score: f64,
}

// --- struct T1SlaPairRow ---
#[derive(Debug, serde::Deserialize)]
pub(crate) struct T1SlaPairRow {
    pub(crate) pair_id: String,
    pub(crate) origin_id: String,
    pub(crate) dest_id: String,
    pub(crate) target_hours: f64,
    pub(crate) priority: u8,
    pub(crate) market_class: String,
    pub(crate) required_routes: String,
    pub(crate) required_stops: String,
    pub(crate) evidence_basis: String,
}

// --- struct T1DesignReviewRow ---
#[derive(Debug, Clone)]
pub(crate) struct T1DesignReviewRow {
    pub(crate) route: String,
    pub(crate) selected: bool,
    pub(crate) design_role: &'static str,
    pub(crate) promise_count: usize,
    pub(crate) selected_stop_count: usize,
    pub(crate) top_city_stop_count: usize,
    pub(crate) selector_reason: String,
    pub(crate) beck_action: String,
    pub(crate) beck_review_flag: String,
    pub(crate) overlap_corridors: String,
    pub(crate) design_status: &'static str,
    pub(crate) next_design_action: &'static str,
}

// --- struct T1DesignReviewCsvRow ---
#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
pub(crate) struct T1DesignReviewCsvRow {
    pub(crate) route: String,
    pub(crate) selected: bool,
    pub(crate) design_role: String,
    pub(crate) promise_count: usize,
    pub(crate) selected_stop_count: usize,
    pub(crate) top_city_stop_count: usize,
    pub(crate) selector_reason: String,
    pub(crate) beck_action: String,
    pub(crate) beck_review_flag: String,
    pub(crate) overlap_corridors: String,
    pub(crate) design_status: String,
    pub(crate) next_design_action: String,
}

// --- struct T1TopologyRepairRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T1TopologyRepairRow {
    pub(crate) route: String,
    pub(crate) selected: bool,
    pub(crate) design_role: String,
    pub(crate) design_status: String,
    pub(crate) beck_review_flag: String,
    pub(crate) overlap_corridors: String,
    pub(crate) repair_type: String,
    pub(crate) repair_basis: String,
    pub(crate) next_artifact: String,
    pub(crate) next_action: String,
    pub(crate) validation_status: String,
}

// --- struct T1DesignPolicyActionRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct T1DesignPolicyActionRow {
    pub(crate) action: String,
    pub(crate) applies_to_status: String,
    pub(crate) definition: String,
    pub(crate) required_policy: String,
    pub(crate) design_treatment: String,
    pub(crate) gate_policy: String,
    pub(crate) next_selector_use: String,
}

// --- struct T1ScoreExceptionRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct T1ScoreExceptionRow {
    pub(crate) route: String,
    pub(crate) decision: String,
    pub(crate) exception_type: String,
    pub(crate) rationale: String,
    pub(crate) evidence_status: String,
    pub(crate) artifact: String,
    pub(crate) replacement_candidate: String,
    pub(crate) next_selector_action: String,
}

// --- struct StopCoverageRow ---
#[derive(Debug)]
pub(crate) struct StopCoverageRow {
    pub(crate) route: String,
    pub(crate) stop_count: usize,
    pub(crate) major_stop_count: usize,
    pub(crate) classes: String,
    pub(crate) failures: Vec<String>,
}

// --- struct PressureScenarioRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct PressureScenarioRow {
    pub(crate) scenario_id: String,
    pub(crate) scenario_name: String,
    pub(crate) adversity_class: String,
    pub(crate) standards_tested: String,
    pub(crate) current_status: String,
    pub(crate) existing_artifact: String,
    pub(crate) blocking_gap: String,
    pub(crate) next_evidence_step: String,
}

// --- struct ThroughputProofRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct ThroughputProofRow {
    pub(crate) proof_id: String,
    pub(crate) proof_name: String,
    pub(crate) binding_type: String,
    pub(crate) stressor: String,
    pub(crate) primary_metric: String,
    pub(crate) existing_artifact: String,
    pub(crate) current_status: String,
    pub(crate) blocking_gap: String,
    pub(crate) next_evidence_step: String,
}

// --- struct T1FailureRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T1FailureRow {
    pub(crate) site_id: String,
    pub(crate) intersection: String,
    pub(crate) location: String,
    pub(crate) failure_mode: String,
    pub(crate) annual_probability: Option<f64>,
    pub(crate) duration_p50_hours: Option<f64>,
    pub(crate) duration_p95_hours: Option<f64>,
    pub(crate) throughput_retention_current: Option<f64>,
    pub(crate) throughput_retention_i2: Option<f64>,
    pub(crate) reroute_time_p50_hours: Option<f64>,
    pub(crate) reroute_time_p95_hours: Option<f64>,
    pub(crate) source_status: String,
    pub(crate) confidence: String,
    pub(crate) current_artifact: String,
    pub(crate) blocking_gap: String,
    pub(crate) next_evidence_step: String,
}

// --- struct T1DiamondValidationRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct T1DiamondValidationRow {
    pub(crate) site_id: String,
    pub(crate) intersection: String,
    pub(crate) location: String,
    pub(crate) priority_band: String,
    pub(crate) anchor_lon: f64,
    pub(crate) anchor_lat: f64,
    pub(crate) analyzer_status: String,
    pub(crate) manual_geometry_status: String,
    pub(crate) alternate_capacity_status: String,
    pub(crate) observed_failure_status: String,
    pub(crate) validation_status: String,
    pub(crate) current_artifact: String,
    pub(crate) blocking_gap: String,
    pub(crate) next_validation_step: String,
}

// --- struct T1DiamondValidationTask ---
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct T1DiamondValidationTask {
    pub(crate) priority_band: String,
    pub(crate) category: &'static str,
    pub(crate) site_id: String,
    pub(crate) intersection: String,
    pub(crate) location: String,
    pub(crate) action: String,
    pub(crate) source_action: Option<String>,
}

// --- struct T1FailureSourceRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct T1FailureSourceRow {
    pub(crate) site_id: String,
    pub(crate) intersection: String,
    pub(crate) location: String,
    pub(crate) primary_state_sources: String,
    pub(crate) national_sources: String,
    pub(crate) fields_to_populate: String,
    pub(crate) access_status: String,
    pub(crate) source_url: String,
    pub(crate) notes: String,
}

// --- struct T1SourceHealthRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct T1SourceHealthRow {
    pub(crate) site_id: String,
    pub(crate) source_name: String,
    pub(crate) source_url: String,
    pub(crate) source_kind: String,
    pub(crate) access_health: String,
    pub(crate) ingestion_status: String,
    pub(crate) history_status: String,
    pub(crate) last_checked: String,
    pub(crate) blocking_gap: String,
    pub(crate) next_step: String,
}

// --- struct T1AccessDocketItem ---
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct T1AccessDocketItem {
    pub(crate) site_id: String,
    pub(crate) source_name: String,
    pub(crate) source_url: String,
    pub(crate) access_health: String,
    pub(crate) history_status: String,
    pub(crate) blocking_gap: String,
    pub(crate) category: String,
    pub(crate) priority: String,
    pub(crate) action: String,
}

// --- struct T1SnapshotPlanRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct T1SnapshotPlanRow {
    pub(crate) site_id: String,
    pub(crate) intersection: String,
    pub(crate) priority_band: String,
    pub(crate) source_name: String,
    pub(crate) source_health: String,
    pub(crate) cadence: String,
    pub(crate) fetch_command: String,
    pub(crate) import_command: String,
    pub(crate) accumulate_command: String,
    pub(crate) raw_output: String,
    pub(crate) normalized_output: String,
    pub(crate) accumulated_output: String,
    pub(crate) blocking_gap: String,
    pub(crate) next_step: String,
}

// --- struct T1EvidenceWindowRow ---
#[derive(Debug, Clone, serde::Deserialize)]
pub(crate) struct T1EvidenceWindowRow {
    pub(crate) window_id: String,
    pub(crate) site_id: String,
    pub(crate) source_name: String,
    pub(crate) evidence_mode: String,
    pub(crate) capture_started_at: String,
    pub(crate) capture_ended_at: String,
    pub(crate) observation_start: String,
    pub(crate) observation_end: String,
    pub(crate) raw_artifact: String,
    pub(crate) normalized_artifact: String,
    pub(crate) event_count: usize,
    pub(crate) freight_relevant_count: usize,
    pub(crate) promotion_eligible: bool,
    pub(crate) blocking_gap: String,
    pub(crate) next_step: String,
    pub(crate) review_artifact: String,
}

// --- struct T1FailureEventRow ---
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub(crate) struct T1FailureEventRow {
    pub(crate) site_id: String,
    pub(crate) event_id: String,
    pub(crate) source: String,
    pub(crate) source_event_id: String,
    pub(crate) observation_year: u16,
    pub(crate) start_time: String,
    pub(crate) end_time: String,
    pub(crate) duration_hours: Option<f64>,
    pub(crate) event_type: String,
    pub(crate) full_closure: bool,
    pub(crate) lanes_closed: Option<u8>,
    pub(crate) freight_relevant: bool,
    pub(crate) confidence: String,
    pub(crate) notes: String,
}

// --- struct T1FailureEventSummary ---
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct T1FailureEventSummary {
    pub(crate) site_id: String,
    pub(crate) observed_years: usize,
    pub(crate) event_count: usize,
    pub(crate) annual_rate: f64,
    pub(crate) annual_probability: f64,
    pub(crate) duration_p50_hours: Option<f64>,
    pub(crate) duration_p95_hours: Option<f64>,
    pub(crate) confidence: String,
}

// --- struct I80SourcePolicyRow ---
#[derive(serde::Deserialize)]
pub(crate) struct I80SourcePolicyRow {
    pub(crate) source_id: String,
    pub(crate) acquisition_status: String,
}

// --- struct PortLocation ---
pub(crate) struct PortLocation {
    pub(crate) lat: f64,
    pub(crate) lon: f64,
    pub(crate) _rank: u32,
    pub(crate) is_border: bool,
}

// --- struct FemaTile ---
/// A 1°×1° FEMA NFHL tile with an SFHA feature count.
pub(crate) struct FemaTile {
    pub(crate) name: String,
    pub(crate) xmin: f64,
    pub(crate) ymin: f64,
    pub(crate) xmax: f64,
    pub(crate) ymax: f64,
    pub(crate) sfha_count: u32,
    pub(crate) status: String,
}

// --- struct NbiBridgeRecord ---
// NBI data record for joining
pub(crate) struct NbiBridgeRecord {
    pub(crate) pct_bridges_poor: f32,
    pub(crate) mean_year_built: f32,
    pub(crate) bridge_count: u32,
}

// --- struct HazardZone ---
pub(crate) struct HazardZone {
    pub(crate) wildfire: f32,
    pub(crate) tornado: f32,
    pub(crate) seismic: f32,
}

