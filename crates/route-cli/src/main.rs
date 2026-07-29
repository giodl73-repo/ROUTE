use anyhow::{Context, Result};
use clap::Parser;
use std::path::{Path, PathBuf};

mod cli;
mod game;
mod commands;
mod support;
pub(crate) use support::*;

use cli::{
    Cli, Commands, GapType, GameCommand, InterventionCorridorArg, OdCorridorCmd, SimMode,
    TierRegionGraphArg,
};

const T1_THRESHOLD: f64 = route_network::T1_SCORE_THRESHOLD;
const T2_THRESHOLD: f64 = route_network::T2_SCORE_THRESHOLD;
const T3_THRESHOLD: f64 = route_network::T3_SCORE_THRESHOLD;
const DIMENSION_CODES: [&str; 16] = [
    "A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "C1", "C2", "C3", "C4", "D1", "D2", "D3",
];

struct ConfidenceRisk {
    route: String,
    score: f64,
    tier: &'static str,
    mean_confidence: f32,
    score_confidence: f32,
    risk_dimensions: String,
}

struct ScoreAllRow {
    route: String,
    score: f64,
    tier: &'static str,
    rubric_version: String,
    estimated: bool,
    confidence: f32,
    score_confidence: f32,
    dimensions: [f64; 16],
    dimension_confidences: [f32; 16],
}


pub(crate) fn main() -> Result<()> {
    std::thread::Builder::new()
        .name("route-cli".to_string())
        .stack_size(32 * 1024 * 1024)
        .spawn(run_cli)
        .context("spawning route CLI thread")?
        .join()
        .map_err(|panic| {
            if let Some(message) = panic.downcast_ref::<&str>() {
                anyhow::anyhow!("route CLI thread panicked: {message}")
            } else if let Some(message) = panic.downcast_ref::<String>() {
                anyhow::anyhow!("route CLI thread panicked: {message}")
            } else {
                anyhow::anyhow!("route CLI thread panicked")
            }
        })?
}

pub(crate) fn run_cli() -> Result<()> {
    let cli = Cli::parse();

    // Load scoring config
    let scoring_config_path = cli
        .scoring_config
        .clone()
        .unwrap_or_else(|| PathBuf::from("config/scoring.toml"));
    let scoring_cfg = {
        if scoring_config_path.exists() {
            route_score::ScoringConfig::load(&scoring_config_path)
                .context("loading scoring config")?
        } else {
            eprintln!("note: config/scoring.toml not found — using built-in defaults");
            route_score::ScoringConfig::default_config()
        }
    };

    // Load manifest — check data/manifest.json in project root first, then ~/.route/manifest.json
    let manifest_path = cli.manifest.clone().unwrap_or_else(|| {
        let local = std::path::PathBuf::from("data/manifest.json");
        if local.exists() {
            local
        } else {
            route_data::Manifest::default_path()
        }
    });

    // Initialize strategic designation data from CSV (no-op if file not found)
    route_network::strategic::init_designations(std::path::Path::new("data"));

    let cmd_ctx = commands::ctx::Ctx {
        manifest_path: &manifest_path,
        scoring_cfg: &scoring_cfg,
        scoring_config_path: &scoring_config_path,
    };

    match cli.command {
        Commands::Fetch { force, year: _ } => commands::data::fetch::run(&cmd_ctx, force)?,

        Commands::Build {
            all_roads,
            hpms: hpms_path,
            fpm: fpm_path,
        } => commands::core::build::run(&cmd_ctx, all_roads, hpms_path, fpm_path)?,

        Commands::FetchHpms {
            output,
            states,
            functional_systems,
        } => commands::data::fetch_hpms::run(&cmd_ctx, output, states, functional_systems)?,

        Commands::Score {
            designation,
            estimated,
            proposed,
        } => commands::core::score::run(&cmd_ctx, designation, estimated, proposed)?,

        Commands::ScoreAll { workers } => commands::core::score_all::run(&cmd_ctx, workers)?,

        Commands::Gap { r#type, slug } => commands::core::gap::run(&cmd_ctx, r#type, slug)?,

        Commands::Map {
            designation,
            output,
            color_by,
        } => commands::map::map::run(&cmd_ctx, designation, output, color_by)?,

        Commands::MapAtlas {
            ledger,
            details,
            gate,
        } => commands::map::map_atlas::run(&cmd_ctx, ledger, details, gate)?,

        Commands::MapPublicationReadiness {
            map_atlas,
            backlog,
            scope_decision,
            output,
            details,
            gate,
        } => commands::map::map_publication_readiness::run(&cmd_ctx, map_atlas, backlog, scope_decision, output, details, gate)?,

        Commands::MapPublicationInventory {
            inventory,
            map_atlas,
            readiness,
            details,
            gate,
        } => commands::map::map_publication_inventory::run(&cmd_ctx, inventory, map_atlas, readiness, details, gate)?,

        Commands::BeckT2Diagnostics { output, gate } => commands::t2::beck_t2_diagnostics::run(&cmd_ctx, output, gate)?,

        Commands::BeckT1Diagnostics { output, gate } => commands::t1::beck_t1_diagnostics::run(&cmd_ctx, output, gate)?,

        Commands::T1SlaCandidatePairs {
            candidates,
            selected_pairs,
            output,
            selected_budget,
            gate,
        } => commands::t1::t1_sla_candidate_pairs::run(&cmd_ctx, candidates, selected_pairs, output, selected_budget, gate)?,

        Commands::T1LineSelector {
            tier_table,
            stop_candidates,
            sla_pairs,
            score_exceptions,
            constraint_budget,
            output,
            route_budget,
            city_budget,
            stop_budget,
            gate,
        } => commands::t1::t1_line_selector::run(&cmd_ctx, tier_table, stop_candidates, sla_pairs, score_exceptions, constraint_budget, output, route_budget, city_budget, stop_budget, gate)?,

        Commands::T1StopSelector {
            selector,
            stop_candidates,
            output,
            target_regions,
            gate,
        } => commands::t1::t1_stop_selector::run(&cmd_ctx, selector, stop_candidates, output, target_regions, gate)?,

        Commands::T1DesignReview {
            tier_table,
            stop_candidates,
            sla_pairs,
            score_exceptions,
            constraint_budget,
            output,
            route_budget,
            city_budget,
            stop_budget,
            gate,
        } => commands::t1::t1_design_review::run(&cmd_ctx, tier_table, stop_candidates, sla_pairs, score_exceptions, constraint_budget, output, route_budget, city_budget, stop_budget, gate)?,

        Commands::T1TopologyRepairs {
            design_review,
            output,
            gate,
        } => commands::t1::t1_topology_repairs::run(&cmd_ctx, design_review, output, gate)?,

        Commands::T1BeckAlignment {
            stop_selector,
            output,
            gate,
        } => commands::t1::t1_beck_alignment::run(&cmd_ctx, stop_selector, output, gate)?,

        Commands::T1DesignPolicy {
            review,
            policy,
            details,
            gate,
        } => commands::t1::t1_design_policy::run(&cmd_ctx, review, policy, details, gate)?,

        Commands::T1ScoreExceptions {
            review,
            exceptions,
            details,
            gate,
        } => commands::t1::t1_score_exceptions::run(&cmd_ctx, review, exceptions, details, gate)?,

        Commands::BeckT2ServiceStandards { output, gate } => commands::t2::beck_t2_service_standards::run(&cmd_ctx, output, gate)?,

        Commands::BeckT2QualificationActions { output, gate } => commands::t2::beck_t2_qualification_actions::run(&cmd_ctx, output, gate)?,

        Commands::Report {
            designation,
            output,
            allow_partial,
        } => commands::core::report::run(&cmd_ctx, designation, output, allow_partial)?,

        Commands::Flow { designation } => commands::core::flow::run(&cmd_ctx, designation)?,

        Commands::Invest {
            budget,
            include_upgrades,
            top,
        } => commands::core::invest::run(&cmd_ctx, budget, include_upgrades, top)?,

        Commands::FetchAcs => commands::data::fetch_acs::run(&cmd_ctx)?,

        Commands::FetchAcsIncome => commands::data::fetch_acs_income::run(&cmd_ctx)?,

        Commands::FetchFemaD1 => commands::data::fetch_fema_d1::run(&cmd_ctx)?,

        Commands::FetchFema { output } => commands::data::fetch_fema::run(&cmd_ctx, output)?,

        Commands::SourceFetchPolicy { output, gate } => commands::data::source_fetch_policy::run(&cmd_ctx, output, gate)?,

        Commands::FletchSources {
            registry,
            source_policy,
            output,
            details,
            gate,
        } => commands::data::fletch_sources::run(&cmd_ctx, registry, source_policy, output, details, gate)?,

        Commands::FletchCacheIndex {
            registry,
            cache_manifest,
            output,
            details,
            gate,
        } => commands::data::fletch_cache_index::run(&cmd_ctx, registry, cache_manifest, output, details, gate)?,

        Commands::Coverage {
            threshold,
            grid,
            t1_only,
            top_gaps,
            grid_mode,
        } => commands::core::coverage::run(&cmd_ctx, threshold, grid, t1_only, top_gaps, grid_mode)?,

        Commands::Standards { tier } => commands::standards::standards::run(&cmd_ctx, tier)?,
        Commands::StandardsProof {
            ledger,
            tier,
            family,
            details,
            gate_blueprint,
            gate_pressure,
        } => commands::standards::standards_proof::run(&cmd_ctx, ledger, tier, family, details, gate_blueprint, gate_pressure)?,

        Commands::Forum {
            docket,
            blockers,
            details,
            gate,
        } => commands::governance::forum::run(&cmd_ctx, docket, blockers, details, gate)?,

        Commands::SignificantMoments {
            ledger,
            blockers,
            details,
            gate,
        } => commands::governance::significant_moments::run(&cmd_ctx, ledger, blockers, details, gate)?,

        Commands::ReleaseManifest {
            manifest,
            blockers,
            details,
            gate,
        } => commands::governance::release_manifest::run(&cmd_ctx, manifest, blockers, details, gate)?,

        Commands::Blueprint {
            ledger,
            blockers,
            details,
            gate,
        } => commands::governance::blueprint::run(&cmd_ctx, ledger, blockers, details, gate)?,

        Commands::BlueprintEvidence {
            ledger,
            evidence_map,
            standards_ledger,
            blockers,
            details,
            gate,
        } => commands::governance::blueprint_evidence::run(&cmd_ctx, ledger, evidence_map, standards_ledger, blockers, details, gate)?,

        Commands::BlueprintCosts {
            ledger,
            costs,
            blockers,
            details,
            gate,
        } => commands::governance::blueprint_costs::run(&cmd_ctx, ledger, costs, blockers, details, gate)?,

        Commands::StandardsInventory {
            ledger,
            standards_ledger,
            blockers,
            details,
            gate,
            gate_planned,
        } => commands::standards::standards_inventory::run(&cmd_ctx, ledger, standards_ledger, blockers, details, gate, gate_planned)?,

        Commands::StandardsPavement {
            ledger,
            blockers,
            details,
            gate,
        } => commands::standards::standards_pavement::run(&cmd_ctx, ledger, blockers, details, gate)?,

        Commands::TierPavementDocket {
            segments,
            standards,
            output,
            details,
            gate,
        } => commands::pavement::tier_pavement_docket::run(&cmd_ctx, segments, standards, output, details, gate)?,

        Commands::TierPavementSourceGaps {
            docket,
            output,
            details,
            gate,
        } => commands::pavement::tier_pavement_source_gaps::run(&cmd_ctx, docket, output, details, gate)?,

        Commands::TierPavementDebtBudget {
            source_gaps,
            route_state_exclusions,
            repair_funding_acceptance,
            output,
            details,
            gate,
        } => commands::pavement::tier_pavement_debt_budget::run(&cmd_ctx, source_gaps, route_state_exclusions, repair_funding_acceptance, output, details, gate)?,

        Commands::TierPavementAcquisitionPlan {
            source_gaps,
            output,
            details,
            gate,
        } => commands::pavement::tier_pavement_acquisition_plan::run(&cmd_ctx, source_gaps, output, details, gate)?,

        Commands::TierPavementAcquisitionDocket {
            acquisition_plan,
            output,
            priority,
            script,
            gate,
        } => commands::pavement::tier_pavement_acquisition_docket::run(&cmd_ctx, acquisition_plan, output, priority, script, gate)?,

        Commands::TierPavementSourceAccess {
            acquisition_docket,
            output,
            priority,
            gate,
        } => commands::pavement::tier_pavement_source_access::run(&cmd_ctx, acquisition_docket, output, priority, gate)?,

        Commands::TierPavementSourceFetchAttempt {
            source_access,
            output,
            gate,
        } => commands::pavement::tier_pavement_source_fetch_attempt::run(&cmd_ctx, source_access, output, gate)?,

        Commands::TierPavementSourceFetchReview {
            fetch_attempt,
            acquisition_docket,
            source_gaps,
            output,
            gate,
        } => commands::pavement::tier_pavement_source_fetch_review::run(&cmd_ctx, fetch_attempt, acquisition_docket, source_gaps, output, gate)?,

        Commands::TierPavementUnmatchedJoinReview {
            fetch_review,
            source_gaps,
            pavement_docket,
            cache_dir,
            output,
            gate,
        } => commands::pavement::tier_pavement_unmatched_join_review::run(&cmd_ctx, fetch_review, source_gaps, pavement_docket, cache_dir, output, gate)?,

        Commands::TierPavementHpmsScopeBroadening {
            unmatched_join_review,
            output,
            functional_systems,
            gate,
        } => commands::pavement::tier_pavement_hpms_scope_broadening::run(&cmd_ctx, unmatched_join_review, output, functional_systems, gate)?,

        Commands::TierPavementRepairDebtReview {
            unmatched_join_review,
            pavement_debt_budget,
            route_state_exclusions,
            repair_funding_acceptance,
            output,
            gate,
        } => commands::pavement::tier_pavement_repair_debt_review::run(&cmd_ctx, unmatched_join_review, pavement_debt_budget, route_state_exclusions, repair_funding_acceptance, output, gate)?,

        Commands::TierPavementRepairDisposition {
            repair_debt_review,
            output,
            gate,
        } => commands::pavement::tier_pavement_repair_disposition::run(&cmd_ctx, repair_debt_review, output, gate)?,

        Commands::TierPavementRepairFundingPackage {
            repair_disposition,
            output,
            gate,
        } => commands::pavement::tier_pavement_repair_funding_package::run(&cmd_ctx, repair_disposition, output, gate)?,

        Commands::TierPavementFundingCommitmentReview {
            repair_funding_package,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_commitment_review::run(&cmd_ctx, repair_funding_package, output, gate)?,

        Commands::TierPavementDowngradeExclusionDecision {
            funding_commitment_review,
            output,
            gate,
        } => commands::pavement::tier_pavement_downgrade_exclusion_decision::run(&cmd_ctx, funding_commitment_review, output, gate)?,

        Commands::TierPavementFundingEvidenceContract {
            downgrade_exclusion_decision,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_contract::run(&cmd_ctx, downgrade_exclusion_decision, output, gate)?,

        Commands::TierPavementFundingEvidenceSourceCapture {
            funding_evidence_contract,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_source_capture::run(&cmd_ctx, funding_evidence_contract, output, gate)?,

        Commands::TierPavementFundingEvidenceArtifactAttachment {
            source_capture,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_artifact_attachment::run(&cmd_ctx, source_capture, output, gate)?,

        Commands::TierPavementFundingEvidenceReviewDocket {
            artifact_attachment,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_review_docket::run(&cmd_ctx, artifact_attachment, output, gate)?,

        Commands::TierPavementFundingEvidenceAcquisition {
            review_docket,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_acquisition::run(&cmd_ctx, review_docket, output, gate)?,

        Commands::TierPavementFundingEvidenceSourceAccess {
            evidence_acquisition,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_source_access::run(&cmd_ctx, evidence_acquisition, output, gate)?,

        Commands::TierPavementFundingEvidenceIntake {
            source_access,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_intake::run(&cmd_ctx, source_access, output, gate)?,

        Commands::TierPavementFundingEvidenceMetadataCapture {
            evidence_intake,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_metadata_capture::run(&cmd_ctx, evidence_intake, output, gate)?,

        Commands::TierPavementFundingEvidenceAcceptedArtifactAttachment {
            metadata_capture,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_accepted_artifact_attachment::run(&cmd_ctx, metadata_capture, output, gate)?,

        Commands::TierPavementFundingEvidenceAcceptedAttachmentReview {
            accepted_artifact_attachment,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_accepted_attachment_review::run(&cmd_ctx, accepted_artifact_attachment, output, gate)?,

        Commands::TierPavementFundingEvidenceAcceptedArtifactAcquisition {
            accepted_attachment_review,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_accepted_artifact_acquisition::run(&cmd_ctx, accepted_attachment_review, output, gate)?,

        Commands::TierPavementFundingEvidenceAcceptedSourceAccess {
            accepted_artifact_acquisition,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_accepted_source_access::run(&cmd_ctx, accepted_artifact_acquisition, output, gate)?,

        Commands::TierPavementFundingEvidenceAcceptedIntake {
            accepted_source_access,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_accepted_intake::run(&cmd_ctx, accepted_source_access, output, gate)?,

        Commands::TierPavementFundingEvidenceAcceptedMetadataCapture {
            accepted_intake,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_accepted_metadata_capture::run(&cmd_ctx, accepted_intake, output, gate)?,

        Commands::TierPavementFundingEvidenceAcceptedMetadataArtifactAttachment {
            accepted_metadata_capture,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_accepted_metadata_artifact_attachment::run(&cmd_ctx, accepted_metadata_capture, output, gate)?,

        Commands::TierPavementFundingEvidenceAcceptedMetadataAttachmentReview {
            accepted_metadata_artifact_attachment,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_accepted_metadata_attachment_review::run(&cmd_ctx, accepted_metadata_artifact_attachment, output, gate)?,

        Commands::TierPavementFundingEvidenceAcceptedMetadataArtifactAcquisition {
            accepted_metadata_attachment_review,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition::run(&cmd_ctx, accepted_metadata_attachment_review, output, gate)?,

        Commands::TierPavementFundingEvidenceAcceptedMetadataSourceAccess {
            accepted_metadata_artifact_acquisition,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_accepted_metadata_source_access::run(&cmd_ctx, accepted_metadata_artifact_acquisition, output, gate)?,

        Commands::TierPavementFundingEvidenceAcceptedMetadataIntake {
            accepted_metadata_source_access,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_accepted_metadata_intake::run(&cmd_ctx, accepted_metadata_source_access, output, gate)?,

        Commands::TierPavementFundingEvidenceAcceptedMetadataSourceCapture {
            accepted_metadata_intake,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_accepted_metadata_source_capture::run(&cmd_ctx, accepted_metadata_intake, output, gate)?,

        Commands::TierPavementFundingEvidenceAcceptedMetadataSourceCaptureArtifactAttachment {
            accepted_metadata_source_capture,
            output,
            gate,
        } => commands::pavement::tier_pavement_funding_evidence_accepted_metadata_source_capture_artifact_attachment::run(&cmd_ctx, accepted_metadata_source_capture, output, gate)?,

        Commands::StandardsBridges {
            tier_table,
            tier,
            details,
            gate_l1,
        } => commands::standards::standards_bridges::run(&cmd_ctx, tier_table, tier, details, gate_l1)?,

        Commands::PressureScenarios {
            ledger,
            blockers,
            details,
            coverage,
            standards_ledger,
            gate_l2,
            gate_readiness,
            gate_coverage,
        } => commands::analysis::pressure_scenarios::run(&cmd_ctx, ledger, blockers, details, coverage, standards_ledger, gate_l2, gate_readiness, gate_coverage)?,

        Commands::ThroughputProof {
            matrix,
            blockers,
            details,
            gate,
        } => commands::analysis::throughput_proof::run(&cmd_ctx, matrix, blockers, details, gate)?,

        Commands::T1Failures {
            ledger,
            needs_sources,
            details,
            gate_evidence,
        } => commands::t1::t1_failures::run(&cmd_ctx, ledger, needs_sources, details, gate_evidence)?,

        Commands::T1DiamondValidation {
            ledger,
            blockers,
            priority,
            docket,
            with_access,
            source_health,
            details,
            gate_catalog,
        } => commands::t1::t1_diamond_validation::run(&cmd_ctx, ledger, blockers, priority, docket, with_access, source_health, details, gate_catalog)?,

        Commands::T1FailureSources {
            ledger,
            lookup_needed,
        } => commands::t1::t1_failure_sources::run(&cmd_ctx, ledger, lookup_needed)?,

        Commands::T1SourceHealth {
            ledger,
            blockers,
            details,
            gate_ingestion,
        } => commands::t1::t1_source_health::run(&cmd_ctx, ledger, blockers, details, gate_ingestion)?,

        Commands::T1AccessDocket {
            ledger,
            category,
            details,
        } => commands::t1::t1_access_docket::run(&cmd_ctx, ledger, category, details)?,

        Commands::T1SnapshotPlan {
            ledger,
            priority,
            details,
            script,
            gate_plan,
        } => commands::t1::t1_snapshot_plan::run(&cmd_ctx, ledger, priority, details, script, gate_plan)?,

        Commands::T1EvidenceWindows {
            ledger,
            blockers,
            details,
            gate_windows,
        } => commands::t1::t1_evidence_windows::run(&cmd_ctx, ledger, blockers, details, gate_windows)?,

        Commands::T1FailureEvents {
            events,
            ledger,
            write_ledger,
            gate_observations,
        } => commands::t1::t1_failure_events::run(&cmd_ctx, events, ledger, write_ledger, gate_observations)?,

        Commands::T1FetchIowa511 { output } => commands::t1::t1_fetch_iowa511::run(&cmd_ctx, output)?,

        Commands::T1ImportIowa511 {
            input,
            output,
            site_id,
            lat,
            lon,
            radius_miles,
        } => commands::t1::t1_import_iowa511::run(&cmd_ctx, input, output, site_id, lat, lon, radius_miles)?,

        Commands::T1FetchTdotSmartway {
            output,
            timeout_seconds,
        } => commands::t1::t1_fetch_tdot_smartway::run(&cmd_ctx, output, timeout_seconds)?,

        Commands::T1ImportTdotSmartway {
            input,
            output,
            site_id,
            lat,
            lon,
            radius_miles,
        } => commands::t1::t1_import_tdot_smartway::run(&cmd_ctx, input, output, site_id, lat, lon, radius_miles)?,

        Commands::T1FetchMdotMidrive { output } => commands::t1::t1_fetch_mdot_midrive::run(&cmd_ctx, output)?,

        Commands::T1ImportMdotMidrive {
            input,
            output,
            site_id,
            lat,
            lon,
            radius_miles,
            observation_year,
        } => commands::t1::t1_import_mdot_midrive::run(&cmd_ctx, input, output, site_id, lat, lon, radius_miles, observation_year)?,

        Commands::T1FetchIndotTrafficwise {
            output,
            north,
            south,
            east,
            west,
            zoom,
        } => commands::t1::t1_fetch_indot_trafficwise::run(&cmd_ctx, output, north, south, east, west, zoom)?,

        Commands::T1ImportIndotTrafficwise {
            input,
            output,
            site_id,
            observation_year,
        } => commands::t1::t1_import_indot_trafficwise::run(&cmd_ctx, input, output, site_id, observation_year)?,

        Commands::T1AccumulateEvents {
            events,
            input,
            output,
        } => commands::t1::t1_accumulate_events::run(&cmd_ctx, events, input, output)?,

        Commands::Game { command } => commands::game::game_cmd::run(&cmd_ctx, command)?,
        Commands::Sim { mode } => commands::core::sim::run(&cmd_ctx, mode)?,

        Commands::Diamond { at } => commands::core::diamond::run(&cmd_ctx, at)?,

        Commands::Connectivity { all_pairs } => commands::core::connectivity::run(&cmd_ctx, all_pairs)?,

        Commands::TierConnectivity {
            tier_table,
            exceptions,
            tier,
            details,
            gate,
        } => commands::network::tier_connectivity::run(&cmd_ctx, tier_table, exceptions, tier, details, gate)?,

        Commands::TierRegions {
            tier_table,
            tier,
            regions,
            graph,
            output,
            repairs,
            gate,
        } => commands::network::tier_regions::run(&cmd_ctx, tier_table, tier, regions, graph, output, repairs, gate)?,

        Commands::TierContactWitnesses {
            repairs,
            output,
            gate,
        } => commands::network::tier_contact_witnesses::run(&cmd_ctx, repairs, output, gate)?,

        Commands::T2ContactResolutions {
            witnesses,
            exceptions,
            output,
            gate,
        } => commands::t2::t2_contact_resolutions::run(&cmd_ctx, witnesses, exceptions, output, gate)?,

        Commands::T2HeldContactActions {
            resolutions,
            output,
            gate,
        } => commands::t2::t2_held_contact_actions::run(&cmd_ctx, resolutions, output, gate)?,

        Commands::T2GraphContactRepairs {
            held_actions,
            output,
            gate,
        } => commands::t2::t2_graph_contact_repairs::run(&cmd_ctx, held_actions, output, gate)?,

        Commands::T2ParentContactValidation {
            held_actions,
            witnesses,
            output,
            gate,
        } => commands::t2::t2_parent_contact_validation::run(&cmd_ctx, held_actions, witnesses, output, gate)?,

        Commands::T2ReliefEvidenceDocket {
            held_actions,
            bottlenecks,
            output,
            gate,
        } => commands::t2::t2_relief_evidence_docket::run(&cmd_ctx, held_actions, bottlenecks, output, gate)?,

        Commands::T2TerminalContactValidation {
            held_actions,
            exceptions,
            witnesses,
            output,
            gate,
        } => commands::t2::t2_terminal_contact_validation::run(&cmd_ctx, held_actions, exceptions, witnesses, output, gate)?,

        Commands::T2BlockerClosure {
            graph_repairs,
            parent_validation,
            relief_evidence,
            terminal_validation,
            bundles,
            output,
            gate,
        } => commands::t2::t2_blocker_closure::run(&cmd_ctx, graph_repairs, parent_validation, relief_evidence, terminal_validation, bundles, output, gate)?,

        Commands::T2RouteFamilySplits {
            closure,
            service_diagnostics,
            bundles,
            exceptions,
            output,
            gate,
        } => commands::t2::t2_route_family_splits::run(&cmd_ctx, closure, service_diagnostics, bundles, exceptions, output, gate)?,

        Commands::T2GraphContactValidation {
            closure,
            witnesses,
            output,
            gate,
        } => commands::t2::t2_graph_contact_validation::run(&cmd_ctx, closure, witnesses, output, gate)?,

        Commands::T2ContactClosure {
            closure,
            witnesses,
            output,
            gate,
        } => commands::t2::t2_contact_closure::run(&cmd_ctx, closure, witnesses, output, gate)?,

        Commands::T2EndpointClosure {
            closure,
            exceptions,
            output,
            gate,
        } => commands::t2::t2_endpoint_closure::run(&cmd_ctx, closure, exceptions, output, gate)?,

        Commands::TierCandidateColumns {
            witnesses,
            route_family_splits,
            graph_contact_validation,
            contact_closure,
            endpoint_closure,
            blocker_closure,
            pavement_debt_budget,
            constraint_budget,
            output,
            gate,
        } => commands::network::tier_candidate_columns::run(&cmd_ctx, witnesses, route_family_splits, graph_contact_validation, contact_closure, endpoint_closure, blocker_closure, pavement_debt_budget, constraint_budget, output, gate)?,

        Commands::T2Regionalizer {
            candidates,
            output,
            gate,
        } => commands::t2::t2_regionalizer::run(&cmd_ctx, candidates, output, gate)?,

        Commands::T2ServiceSelection {
            regionalizer,
            output,
            gate,
        } => commands::t2::t2_service_selection::run(&cmd_ctx, regionalizer, output, gate)?,

        Commands::T2ServiceDiagnosticQueue {
            service_selection,
            bundles,
            output,
            gate,
        } => commands::t2::t2_service_diagnostic_queue::run(&cmd_ctx, service_selection, bundles, output, gate)?,

        Commands::T2ParallelServiceQueue {
            service_selection,
            output,
            gate,
        } => commands::t2::t2_parallel_service_queue::run(&cmd_ctx, service_selection, output, gate)?,

        Commands::T2BundleOverlays {
            service_selection,
            bundles,
            game_overlays,
            output,
            gate,
        } => commands::t2::t2_bundle_overlays::run(&cmd_ctx, service_selection, bundles, game_overlays, output, gate)?,

        Commands::T2BundleRepairQueue {
            candidates,
            blocker_closure,
            output,
            gate,
        } => commands::t2::t2_bundle_repair_queue::run(&cmd_ctx, candidates, blocker_closure, output, gate)?,

        Commands::TierSegmentCandidates {
            t1_selector,
            t2_service_selection,
            t2_bundle_repair_queue,
            t2_route_family_splits,
            output,
            gate,
        } => commands::network::tier_segment_candidates::run(&cmd_ctx, t1_selector, t2_service_selection, t2_bundle_repair_queue, t2_route_family_splits, output, gate)?,

        Commands::LowerTierPressureWitnesses {
            tier_table,
            candidates,
            resolutions,
            route_family_splits,
            graph_contact_validation,
            contact_closure,
            endpoint_closure,
            output,
            gate,
        } => commands::network::lower_tier_pressure_witnesses::run(&cmd_ctx, tier_table, candidates, resolutions, route_family_splits, graph_contact_validation, contact_closure, endpoint_closure, output, gate)?,

        Commands::T3T4PressureIntake {
            pressure,
            output,
            gate,
        } => commands::t3::t3_t4_pressure_intake::run(&cmd_ctx, pressure, output, gate)?,

        Commands::T3ZoneAccessObligations {
            intake,
            map_atlas,
            output,
            gate,
        } => commands::t3::t3_zone_access_obligations::run(&cmd_ctx, intake, map_atlas, output, gate)?,

        Commands::T3ZoneRouteColumns {
            obligations,
            intake,
            constraint_budget,
            output,
            gate,
        } => commands::t3::t3_zone_route_columns::run(&cmd_ctx, obligations, intake, constraint_budget, output, gate)?,

        Commands::T4TerminalAccessColumns {
            intake,
            constraint_budget,
            output,
            gate,
        } => commands::t4::t4_terminal_access_columns::run(&cmd_ctx, intake, constraint_budget, output, gate)?,

        Commands::T4TerminalContactEvidence {
            terminal_columns,
            output,
            gate,
        } => commands::t4::t4_terminal_contact_evidence::run(&cmd_ctx, terminal_columns, output, gate)?,

        Commands::T4TerminalAccessEvidenceReview {
            contact_evidence,
            output,
            gate,
        } => commands::t4::t4_terminal_access_evidence_review::run(&cmd_ctx, contact_evidence, output, gate)?,

        Commands::T4TerminalAccessProofAcquisition {
            evidence_review,
            output,
            gate,
        } => commands::t4::t4_terminal_access_proof_acquisition::run(&cmd_ctx, evidence_review, output, gate)?,

        Commands::T4TerminalAccessProofArtifacts {
            proof_acquisition,
            output,
            gate,
        } => commands::t4::t4_terminal_access_proof_artifacts::run(&cmd_ctx, proof_acquisition, output, gate)?,

        Commands::T4TerminalAccessProofReview {
            proof_artifacts,
            output,
            gate,
        } => commands::t4::t4_terminal_access_proof_review::run(&cmd_ctx, proof_artifacts, output, gate)?,

        Commands::T4TerminalAccessSourceAccess {
            proof_review,
            output,
            gate,
        } => commands::t4::t4_terminal_access_source_access::run(&cmd_ctx, proof_review, output, gate)?,

        Commands::T4TerminalAccessProofIntake {
            source_access,
            output,
            gate,
        } => commands::t4::t4_terminal_access_proof_intake::run(&cmd_ctx, source_access, output, gate)?,

        Commands::T4TerminalAccessProofSourceCapture {
            proof_intake,
            output,
            gate,
        } => commands::t4::t4_terminal_access_proof_source_capture::run(&cmd_ctx, proof_intake, output, gate)?,

        Commands::T4TerminalAccessProofArtifactAttachment {
            source_capture,
            output,
            gate,
        } => commands::t4::t4_terminal_access_proof_artifact_attachment::run(&cmd_ctx, source_capture, output, gate)?,

        Commands::T4TerminalAccessProofAttachmentReview {
            artifact_attachment,
            output,
            gate,
        } => commands::t4::t4_terminal_access_proof_attachment_review::run(&cmd_ctx, artifact_attachment, output, gate)?,

        Commands::T4TerminalAccessProofArtifactAcquisitionTargets {
            attachment_review,
            output,
            gate,
        } => commands::t4::t4_terminal_access_proof_artifact_acquisition_targets::run(&cmd_ctx, attachment_review, output, gate)?,

        Commands::T4TerminalAccessProofArtifactSourceAccess {
            acquisition_targets,
            output,
            gate,
        } => commands::t4::t4_terminal_access_proof_artifact_source_access::run(&cmd_ctx, acquisition_targets, output, gate)?,

        Commands::T4TerminalScenarioReadiness {
            contact_evidence,
            output,
            gate,
        } => commands::t4::t4_terminal_scenario_readiness::run(&cmd_ctx, contact_evidence, output, gate)?,

        Commands::T4TerminalContactSourcePlan {
            contact_evidence,
            output,
            catalog_output,
            proof_docket_output,
            gate,
        } => commands::t4::t4_terminal_contact_source_plan::run(&cmd_ctx, contact_evidence, output, catalog_output, proof_docket_output, gate)?,

        Commands::T4TerminalContactProofArtifactContract { output, gate } => commands::t4::t4_terminal_contact_proof_artifact_contract::run(&cmd_ctx, output, gate)?,

        Commands::T4TerminalContactProofSourceRegistry {
            proof_docket,
            accepted_sources,
            output,
            gate,
        } => commands::t4::t4_terminal_contact_proof_source_registry::run(&cmd_ctx, proof_docket, accepted_sources, output, gate)?,

        Commands::T4TerminalContactDistrictProofImport {
            source_registry,
            output,
            gate,
        } => commands::t4::t4_terminal_contact_district_proof_import::run(&cmd_ctx, source_registry, output, gate)?,

        Commands::T4TerminalColumbusProofIntake {
            proof_docket,
            output,
            gate,
        } => commands::t4::t4_terminal_columbus_proof_intake::run(&cmd_ctx, proof_docket, output, gate)?,

        Commands::T4TerminalColumbusSourceAccess {
            columbus_intake,
            output,
            gate,
        } => commands::t4::t4_terminal_columbus_source_access::run(&cmd_ctx, columbus_intake, output, gate)?,

        Commands::T4TerminalColumbusProofAttempts {
            source_access,
            output,
            gate,
        } => commands::t4::t4_terminal_columbus_proof_attempts::run(&cmd_ctx, source_access, output, gate)?,

        Commands::T3T4AccessGaps {
            route_columns,
            terminal_columns,
            output,
            gate,
        } => commands::t3::t3_t4_access_gaps::run(&cmd_ctx, route_columns, terminal_columns, output, gate)?,

        Commands::T3ZoneMapDiagnostics {
            route_columns,
            access_gaps,
            map_atlas,
            output,
            gate,
        } => commands::t3::t3_zone_map_diagnostics::run(&cmd_ctx, route_columns, access_gaps, map_atlas, output, gate)?,

        Commands::T3ZoneRenderBoard {
            diagnostics,
            route_columns,
            access_gaps,
            map_atlas,
            output,
            gate,
        } => commands::t3::t3_zone_render_board::run(&cmd_ctx, diagnostics, route_columns, access_gaps, map_atlas, output, gate)?,

        Commands::T3ZoneStopPlacement {
            render_board,
            stop_candidates,
            output,
            gate,
        } => commands::t3::t3_zone_stop_placement::run(&cmd_ctx, render_board, stop_candidates, output, gate)?,

        Commands::NationalSegmentRegistry {
            render_board,
            stop_placement,
            segment_candidates,
            pavement_docket,
            output,
            gate,
        } => commands::network::national_segment_registry::run(&cmd_ctx, render_board, stop_placement, segment_candidates, pavement_docket, output, gate)?,

        Commands::NationalSegmentBundles {
            registry,
            output,
            gate,
        } => commands::network::national_segment_bundles::run(&cmd_ctx, registry, output, gate)?,

        Commands::T2BubbleUpReview {
            intake,
            output,
            gate,
        } => commands::t2::t2_bubble_up_review::run(&cmd_ctx, intake, output, gate)?,

        Commands::T1FeedbackDocket {
            service_selection,
            bubble_up,
            intake,
            sla_pairs,
            output,
            gate,
        } => commands::t1::t1_feedback_docket::run(&cmd_ctx, service_selection, bubble_up, intake, sla_pairs, output, gate)?,

        Commands::TierOptimize {
            all_tiers,
            output,
            gate,
        } => commands::network::tier_optimize::run(&cmd_ctx, all_tiers, output, gate)?,

        Commands::OptimizerManifest { manifest, gate } => commands::optimizer::optimizer_manifest::run(&cmd_ctx, manifest, gate)?,

        Commands::OptimizerConstraintLedger {
            pavement_debt_budget,
            t2_asset_condition_map_publication_exclusion,
            t1_topology_repairs,
            t1_schematic_geometry_blocker_relief,
            t2_beck_transfer_complexity_blocker_relief,
            t2_beck_label_density_blocker_relief,
            t2_beck_long_connector_blocker_relief,
            t2_game_publication_evidence_blocker_relief,
            t2_game_ops_bundle_evidence_blocker_relief,
            t3_lower_tier_feeder_gap_blocker_relief,
            t2_parallel_service_queue,
            t3_t4_access_gaps,
            t4_terminal_access_map_exclusion,
            t4_terminal_contact_district_proof_import,
            t4_terminal_contact_rejected_proof_sources,
            source_fetch_policy,
            source_snapshot_publication_exclusion,
            t2_scenario_hooks,
            t2_bundle_overlays,
            output,
            details,
            gate,
        } => commands::optimizer::optimizer_constraint_ledger::run(&cmd_ctx, pavement_debt_budget, t2_asset_condition_map_publication_exclusion, t1_topology_repairs, t1_schematic_geometry_blocker_relief, t2_beck_transfer_complexity_blocker_relief, t2_beck_label_density_blocker_relief, t2_beck_long_connector_blocker_relief, t2_game_publication_evidence_blocker_relief, t2_game_ops_bundle_evidence_blocker_relief, t3_lower_tier_feeder_gap_blocker_relief, t2_parallel_service_queue, t3_t4_access_gaps, t4_terminal_access_map_exclusion, t4_terminal_contact_district_proof_import, t4_terminal_contact_rejected_proof_sources, source_fetch_policy, source_snapshot_publication_exclusion, t2_scenario_hooks, t2_bundle_overlays, output, details, gate)?,

        Commands::OptimizerConstraintBudget {
            ledger,
            output,
            details,
            gate,
        } => commands::optimizer::optimizer_constraint_budget::run(&cmd_ctx, ledger, output, details, gate)?,

        Commands::OptimizerResidualBlockerBacklog {
            budget,
            output,
            details,
            gate,
        } => commands::optimizer::optimizer_residual_blocker_backlog::run(&cmd_ctx, budget, output, details, gate)?,

        Commands::OptimizerClaimReview {
            backlog,
            output,
            gate,
        } => commands::optimizer::optimizer_claim_review::run(&cmd_ctx, backlog, output, gate)?,

        Commands::T2GamePublicationEvidenceReview {
            claim_review,
            scenario_hooks,
            output,
            gate,
        } => commands::t2::t2_game_publication_evidence_review::run(&cmd_ctx, claim_review, scenario_hooks, output, gate)?,

        Commands::T2GamePublicationEvidencePolicy {
            review,
            output,
            gate,
        } => commands::t2::t2_game_publication_evidence_policy::run(&cmd_ctx, review, output, gate)?,

        Commands::T2GamePublicationEvidencePolicyAcceptance {
            policy,
            output,
            gate,
        } => commands::t2::t2_game_publication_evidence_policy_acceptance::run(&cmd_ctx, policy, output, gate)?,

        Commands::T2GamePublicationEvidenceBlockerRelief {
            acceptance,
            output,
            gate,
        } => commands::t2::t2_game_publication_evidence_blocker_relief::run(&cmd_ctx, acceptance, output, gate)?,

        Commands::T1SchematicGeometryClaimReview {
            claim_review,
            design_review,
            policy_actions,
            output,
            gate,
        } => commands::t1::t1_schematic_geometry_claim_review::run(&cmd_ctx, claim_review, design_review, policy_actions, output, gate)?,

        Commands::T2BeckTransferComplexityReview {
            claim_review,
            output,
            gate,
        } => commands::t2::t2_beck_transfer_complexity_review::run(&cmd_ctx, claim_review, output, gate)?,

        Commands::T2BeckLabelDensityReview {
            claim_review,
            output,
            gate,
        } => commands::t2::t2_beck_label_density_review::run(&cmd_ctx, claim_review, output, gate)?,

        Commands::T2BeckLongConnectorReview {
            claim_review,
            output,
            gate,
        } => commands::t2::t2_beck_long_connector_review::run(&cmd_ctx, claim_review, output, gate)?,

        Commands::T2BeckLongConnectorPolicy {
            connector_review,
            output,
            gate,
        } => commands::t2::t2_beck_long_connector_policy::run(&cmd_ctx, connector_review, output, gate)?,

        Commands::T2BeckLongConnectorPolicyAcceptance {
            policy,
            output,
            gate,
        } => commands::t2::t2_beck_long_connector_policy_acceptance::run(&cmd_ctx, policy, output, gate)?,

        Commands::T2BeckLongConnectorBlockerRelief {
            acceptance,
            output,
            gate,
        } => commands::t2::t2_beck_long_connector_blocker_relief::run(&cmd_ctx, acceptance, output, gate)?,

        Commands::T2BeckLabelDensityPolicy {
            label_review,
            output,
            gate,
        } => commands::t2::t2_beck_label_density_policy::run(&cmd_ctx, label_review, output, gate)?,

        Commands::T2BeckLabelDensityPolicyAcceptance {
            policy,
            output,
            gate,
        } => commands::t2::t2_beck_label_density_policy_acceptance::run(&cmd_ctx, policy, output, gate)?,

        Commands::T2BeckTransferComplexityPolicy {
            transfer_review,
            output,
            gate,
        } => commands::t2::t2_beck_transfer_complexity_policy::run(&cmd_ctx, transfer_review, output, gate)?,

        Commands::T2BeckTransferComplexityPolicyAcceptance {
            policy,
            output,
            gate,
        } => commands::t2::t2_beck_transfer_complexity_policy_acceptance::run(&cmd_ctx, policy, output, gate)?,

        Commands::T1SharedSegmentMapPolicy {
            schematic_review,
            output,
            gate,
        } => commands::t1::t1_shared_segment_map_policy::run(&cmd_ctx, schematic_review, output, gate)?,

        Commands::T1SharedSegmentPolicyAcceptance {
            policy,
            output,
            gate,
        } => commands::t1::t1_shared_segment_policy_acceptance::run(&cmd_ctx, policy, output, gate)?,

        Commands::T1SchematicGeometryBlockerRelief {
            acceptance,
            output,
            gate,
        } => commands::t1::t1_schematic_geometry_blocker_relief::run(&cmd_ctx, acceptance, output, gate)?,

        Commands::T2BeckTransferComplexityBlockerRelief {
            acceptance,
            output,
            gate,
        } => commands::t2::t2_beck_transfer_complexity_blocker_relief::run(&cmd_ctx, acceptance, output, gate)?,

        Commands::T2BeckLabelDensityBlockerRelief {
            acceptance,
            output,
            gate,
        } => commands::t2::t2_beck_label_density_blocker_relief::run(&cmd_ctx, acceptance, output, gate)?,

        Commands::T3LowerTierFeederGapReview {
            backlog,
            access_gaps,
            output,
            gate,
        } => commands::t3::t3_lower_tier_feeder_gap_review::run(&cmd_ctx, backlog, access_gaps, output, gate)?,

        Commands::T3LowerTierFeederGapPolicy {
            feeder_review,
            output,
            gate,
        } => commands::t3::t3_lower_tier_feeder_gap_policy::run(&cmd_ctx, feeder_review, output, gate)?,

        Commands::T3LowerTierFeederGapPolicyAcceptance {
            policy,
            output,
            gate,
        } => commands::t3::t3_lower_tier_feeder_gap_policy_acceptance::run(&cmd_ctx, policy, output, gate)?,

        Commands::T3LowerTierFeederGapBlockerRelief {
            acceptance,
            output,
            gate,
        } => commands::t3::t3_lower_tier_feeder_gap_blocker_relief::run(&cmd_ctx, acceptance, output, gate)?,

        Commands::T2GameOpsBindingIntake {
            budget,
            output,
            gate,
        } => commands::t2::t2_game_ops_binding_intake::run(&cmd_ctx, budget, output, gate)?,

        Commands::T2GameOpsBindingDecisions {
            intake,
            bundle_overlays,
            output,
            gate,
        } => commands::t2::t2_game_ops_binding_decisions::run(&cmd_ctx, intake, bundle_overlays, output, gate)?,

        Commands::T2BundleOverlayRepairTargets {
            decisions,
            bundle_overlays,
            output,
            gate,
        } => commands::t2::t2_bundle_overlay_repair_targets::run(&cmd_ctx, decisions, bundle_overlays, output, gate)?,

        Commands::T2ServiceClassRepairDocket {
            targets,
            service_diagnostics,
            output,
            gate,
        } => commands::t2::t2_service_class_repair_docket::run(&cmd_ctx, targets, service_diagnostics, output, gate)?,

        Commands::T2GameOpsBundleEvidenceReview {
            decisions,
            targets,
            service_docket,
            output,
            gate,
        } => commands::t2::t2_game_ops_bundle_evidence_review::run(&cmd_ctx, decisions, targets, service_docket, output, gate)?,

        Commands::T2GameOpsBundleEvidencePolicy {
            review,
            output,
            gate,
        } => commands::t2::t2_game_ops_bundle_evidence_policy::run(&cmd_ctx, review, output, gate)?,

        Commands::T2GameOpsBundleEvidencePolicyAcceptance {
            policy,
            output,
            gate,
        } => commands::t2::t2_game_ops_bundle_evidence_policy_acceptance::run(&cmd_ctx, policy, output, gate)?,

        Commands::T2GameOpsBundleEvidenceBlockerRelief {
            acceptance,
            output,
            gate,
        } => commands::t2::t2_game_ops_bundle_evidence_blocker_relief::run(&cmd_ctx, acceptance, output, gate)?,

        Commands::T2ServiceOverlayDiagnosticDecisions {
            service_docket,
            targets,
            service_diagnostics,
            output,
            gate,
        } => commands::t2::t2_service_overlay_diagnostic_decisions::run(&cmd_ctx, service_docket, targets, service_diagnostics, output, gate)?,

        Commands::T2LocalZoneOverlayHandoff {
            service_docket,
            zone_route_columns,
            zone_render_board,
            output,
            gate,
        } => commands::t2::t2_local_zone_overlay_handoff::run(&cmd_ctx, service_docket, zone_route_columns, zone_render_board, output, gate)?,

        Commands::T2BundleReadinessDisposition {
            targets,
            output,
            gate,
        } => commands::t2::t2_bundle_readiness_disposition::run(&cmd_ctx, targets, output, gate)?,

        Commands::T2BundleReadinessRepairDocket {
            readiness,
            output,
            gate,
        } => commands::t2::t2_bundle_readiness_repair_docket::run(&cmd_ctx, readiness, output, gate)?,

        Commands::T2BundleReadinessRepairEvidence {
            repair_docket,
            registry,
            segment_candidates,
            service_selection,
            output,
            gate,
        } => commands::t2::t2_bundle_readiness_repair_evidence::run(&cmd_ctx, repair_docket, registry, segment_candidates, service_selection, output, gate)?,

        Commands::T2BundleReadinessReplayDecisions {
            evidence,
            repair_delta,
            output,
            gate,
        } => commands::t2::t2_bundle_readiness_replay_decisions::run(&cmd_ctx, evidence, repair_delta, output, gate)?,

        Commands::T2NationalBundleReadinessAudit {
            replay_decisions,
            bundles,
            output,
            gate,
        } => commands::t2::t2_national_bundle_readiness_audit::run(&cmd_ctx, replay_decisions, bundles, output, gate)?,

        Commands::T2StitchedMemberRegistryHandoff {
            audit,
            registry,
            segment_candidates,
            output,
            gate,
        } => commands::t2::t2_stitched_member_registry_handoff::run(&cmd_ctx, audit, registry, segment_candidates, output, gate)?,

        Commands::T2StitchedMemberCandidateScopeReview {
            handoff,
            segment_candidates,
            output,
            gate,
        } => commands::t2::t2_stitched_member_candidate_scope_review::run(&cmd_ctx, handoff, segment_candidates, output, gate)?,

        Commands::T2StitchedMemberDecisionDocket {
            scope_review,
            output,
            gate,
        } => commands::t2::t2_stitched_member_decision_docket::run(&cmd_ctx, scope_review, output, gate)?,

        Commands::T2StitchedMemberSplitPlan {
            decision_docket,
            segment_candidates,
            output,
            gate,
        } => commands::t2::t2_stitched_member_split_plan::run(&cmd_ctx, decision_docket, segment_candidates, output, gate)?,

        Commands::T2StitchedMemberSelectionDocket {
            split_plan,
            output,
            gate,
        } => commands::t2::t2_stitched_member_selection_docket::run(&cmd_ctx, split_plan, output, gate)?,

        Commands::T2StitchedMemberEvidenceContract {
            selection_docket,
            output,
            gate,
        } => commands::t2::t2_stitched_member_evidence_contract::run(&cmd_ctx, selection_docket, output, gate)?,

        Commands::T2StitchedMemberEvidenceAcquisition {
            evidence_contract,
            output,
            gate,
        } => commands::t2::t2_stitched_member_evidence_acquisition::run(&cmd_ctx, evidence_contract, output, gate)?,

        Commands::T2StitchedMemberSourceAccessPolicy {
            evidence_acquisition,
            output,
            gate,
        } => commands::t2::t2_stitched_member_source_access_policy::run(&cmd_ctx, evidence_acquisition, output, gate)?,

        Commands::T2StitchedMemberProofIntake {
            source_access,
            output,
            gate,
        } => commands::t2::t2_stitched_member_proof_intake::run(&cmd_ctx, source_access, output, gate)?,

        Commands::T2StitchedMemberProofSourceCapture {
            proof_intake,
            output,
            gate,
        } => commands::t2::t2_stitched_member_proof_source_capture::run(&cmd_ctx, proof_intake, output, gate)?,

        Commands::T2StitchedMemberProofArtifactAttachment {
            source_capture,
            output,
            gate,
        } => commands::t2::t2_stitched_member_proof_artifact_attachment::run(&cmd_ctx, source_capture, output, gate)?,

        Commands::T2StitchedMemberProofReviewDocket {
            artifact_attachment,
            output,
            gate,
        } => commands::t2::t2_stitched_member_proof_review_docket::run(&cmd_ctx, artifact_attachment, output, gate)?,

        Commands::T2BundleOverlayRepairDelta {
            decisions,
            targets,
            service_docket,
            readiness,
            output,
            gate,
        } => commands::t2::t2_bundle_overlay_repair_delta::run(&cmd_ctx, decisions, targets, service_docket, readiness, output, gate)?,

        Commands::T2OverlayOptimizerActionDocket {
            repair_delta,
            output,
            gate,
        } => commands::t2::t2_overlay_optimizer_action_docket::run(&cmd_ctx, repair_delta, output, gate)?,

        Commands::T2OverlayP1StructuralReadinessReview {
            action_docket,
            output,
            gate,
        } => commands::t2::t2_overlay_p1_structural_readiness_review::run(&cmd_ctx, action_docket, output, gate)?,

        Commands::T2OverlayP2ServiceOverlayReview {
            action_docket,
            output,
            gate,
        } => commands::t2::t2_overlay_p2_service_overlay_review::run(&cmd_ctx, action_docket, output, gate)?,

        Commands::T2OverlayP3LocalZoneOverlayReview {
            action_docket,
            output,
            gate,
        } => commands::t2::t2_overlay_p3_local_zone_overlay_review::run(&cmd_ctx, action_docket, output, gate)?,

        Commands::OptimizerMapHooks { output, gate } => commands::optimizer::optimizer_map_hooks::run(&cmd_ctx, output, gate)?,

        Commands::BundleArchitecture { output, gate } => commands::network::bundle_architecture::run(&cmd_ctx, output, gate)?,

        Commands::EndpointExceptions {
            ledger,
            tier,
            route,
            blockers,
            details,
            gate,
        } => commands::core::endpoint_exceptions::run(&cmd_ctx, ledger, tier, route, blockers, details, gate)?,

        Commands::StopCandidates {
            ledger,
            stop_class,
            route,
            details,
            gate,
        } => commands::stop::stop_candidates::run(&cmd_ctx, ledger, stop_class, route, details, gate)?,

        Commands::StopPlan {
            route,
            ledger,
            details,
            gate,
        } => commands::stop::stop_plan::run(&cmd_ctx, route, ledger, details, gate)?,

        Commands::StopCoverage {
            tier_table,
            ledger,
            tier,
            blockers,
            gate,
        } => commands::stop::stop_coverage::run(&cmd_ctx, tier_table, ledger, tier, blockers, gate)?,

        Commands::Calibrate => commands::core::calibrate::run(&cmd_ctx)?,

        Commands::Od { corridor, month } => commands::analysis::od::run(&cmd_ctx, corridor, month)?,

        Commands::HubStaff { include_proposed } => commands::analysis::hub_staff::run(&cmd_ctx, include_proposed)?,

        Commands::HubOutage {
            include_proposed,
            outage_hours,
            reserve_driver_fraction,
            adjacent_absorption_fraction,
        } => commands::analysis::hub_outage::run(&cmd_ctx, include_proposed, outage_hours, reserve_driver_fraction, adjacent_absorption_fraction)?,

        Commands::EvAnalysis => commands::analysis::ev_analysis::run(&cmd_ctx)?,

        Commands::EvRestOutage {
            outage_station_fraction,
            backup_power_fraction,
            queue_delay_minutes,
        } => commands::analysis::ev_rest_outage::run(&cmd_ctx, outage_station_fraction, backup_power_fraction, queue_delay_minutes)?,

        Commands::PassengerMatrix { trips, seed } => commands::analysis::passenger_matrix::run(&cmd_ctx, trips, seed)?,

        Commands::SlaMatrix { trips, seed } => commands::analysis::sla_matrix::run(&cmd_ctx, trips, seed)?,

        Commands::StopSlaSurface { output } => commands::stop::stop_sla_surface::run(&cmd_ctx, output)?,

        Commands::StopSlaSummary {
            input,
            top,
            gate_max_gap,
        } => commands::stop::stop_sla_summary::run(&cmd_ctx, input, top, gate_max_gap)?,

        Commands::StopSlaCandidates {
            input,
            ledger,
            cities,
            target_gap,
            top,
            candidates_per_gap,
            output,
            gate,
            gate_no_algorithmic,
        } => commands::stop::stop_sla_candidates::run(&cmd_ctx, input, ledger, cities, target_gap, top, candidates_per_gap, output, gate, gate_no_algorithmic)?,

        Commands::StopSlaPromotions {
            input,
            output,
            include_ledger,
            include_alternates,
            gate,
        } => commands::stop::stop_sla_promotions::run(&cmd_ctx, input, output, include_ledger, include_alternates, gate)?,

        Commands::Interventions {
            corridor,
            trips,
            seed,
        } => commands::analysis::interventions::run(&cmd_ctx, corridor, trips, seed)?,

        Commands::StandardsTest { tier, trips, seed } => commands::standards::standards_test::run(&cmd_ctx, tier, trips, seed)?,
    }

    Ok(())
}

// `print_od_comparison` moved to support::print

// `print_hub_staffing` moved to support::print

// `print_hub_outage` moved to support::print

// `print_ev_analysis` moved to support::print

// `print_ev_rest_outage` moved to support::print

// `print_passenger_matrix` moved to `print_passenger_matrix.rs`

// `print_sla_matrix` moved to support::print::print_sla_matrix

/// Load Amtrak schedules from data/amtrak-schedules.csv.
/// Returns corridor_slug -> scheduled_hours mapping.
/// Falls back to empty HashMap if file not found or unparseable.
pub(crate) fn load_amtrak_schedules(data_dir: &std::path::Path) -> std::collections::HashMap<String, f64> {
    let path = data_dir.join("amtrak-schedules.csv");
    let mut map = std::collections::HashMap::new();
    let Ok(file) = std::fs::File::open(&path) else {
        return map;
    };
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let Ok(record) = result else { continue };
        let slug = record.get(0).unwrap_or("").trim().to_string();
        let hours: f64 = match record.get(2).unwrap_or("").trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if !slug.is_empty() {
            map.entry(slug).or_insert(hours);
        }
    }
    map
}

/// Local deserialization record for ev-profiles.toml (CLI-only; uses String for name).
#[derive(serde::Deserialize)]
struct EvProfileRecord {
    name: String,
    highway_range_miles: f64,
    charge_rate_kw: f64,
    battery_kwh: f64,
    kwh_per_mile: f64,
}

#[derive(serde::Deserialize)]
struct EvProfilesFile {
    vehicles: Vec<EvProfileRecord>,
}

/// Load EV profiles from data/ev-profiles.toml.
/// Falls back to the three built-in profiles if the file is missing or unparseable.
pub(crate) fn load_ev_profiles(data_dir: &std::path::Path) -> Vec<route_sim::EvProfile> {
    let path = data_dir.join("ev-profiles.toml");
    if let Ok(text) = std::fs::read_to_string(&path) {
        if let Ok(file) = toml::from_str::<EvProfilesFile>(&text) {
            if !file.vehicles.is_empty() {
                return file
                    .vehicles
                    .into_iter()
                    .map(|r| {
                        // Box::leak turns an owned String into a &'static str for the lifetime of the
                        // process. Acceptable in a CLI binary that doesn't free profiles at runtime.
                        let name: &'static str = Box::leak(r.name.into_boxed_str());
                        route_sim::EvProfile {
                            name,
                            highway_range_miles: r.highway_range_miles,
                            charge_rate_kw: r.charge_rate_kw,
                            battery_kwh: r.battery_kwh,
                            kwh_per_mile: r.kwh_per_mile,
                        }
                    })
                    .collect();
            }
        }
    }
    // Fall back to built-in profiles
    vec![
        route_sim::average_ev_2026(),
        route_sim::tesla_model_y(),
        route_sim::tesla_semi(),
    ]
}

// `print_intervention_benchmark` moved to support::print::print_intervention_benchmark

pub(crate) fn pct_under(d: &route_sim::TransitDistribution, threshold_h: f64) -> f64 {
    // We only have percentile snapshots; approximate from distribution shape
    if threshold_h >= d.p99_hours {
        return 99.0;
    }
    if threshold_h >= d.p95_hours {
        return 95.0;
    }
    if threshold_h >= d.p90_hours {
        return 90.0;
    }
    if threshold_h >= d.p75_hours {
        return 75.0;
    }
    if threshold_h >= d.p50_hours {
        return 50.0;
    }
    0.0
}

pub(crate) fn pad_center(s: &str, width: usize) -> String {
    if s.len() >= width {
        return s[..width].to_string();
    }
    let pad = width - s.len();
    let left = pad / 2;
    let right = pad - left;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
}

pub(crate) fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

pub(crate) fn tier_for_score(score: f64) -> &'static str {
    route_network::RouteTier::from_score(score).as_str()
}

#[derive(Debug, serde::Deserialize)]
struct StopSlaRow {
    origin_id: String,
    origin_label: String,
    dest_id: String,
    dest_label: String,
    network_miles: f64,
    max_stop_gap_miles: f64,
    stop_gap_status: String,
    route_path: String,
    stop_path: String,
    freight_sla_window: String,
    passenger_competitive_with_air: String,
    rail_competition_note: String,
    evidence_status: String,
}

pub(crate) fn parse_stop_sla_rows<R: std::io::Read>(reader: R) -> Result<Vec<StopSlaRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    rdr.deserialize()
        .collect::<std::result::Result<Vec<_>, _>>()
        .context("parsing stop SLA surface")
}

pub(crate) fn stop_sla_gap_failures(rows: &[StopSlaRow], max_gap: f64) -> Vec<&StopSlaRow> {
    let mut failures = rows
        .iter()
        .filter(|row| row.max_stop_gap_miles > max_gap)
        .collect::<Vec<_>>();
    failures.sort_by(|a, b| b.max_stop_gap_miles.total_cmp(&a.max_stop_gap_miles));
    failures
}

// `print_stop_sla_summary` moved to support::print

#[derive(Debug)]
struct RecurringStopGap {
    segment_id: String,
    labels: String,
    miles: f64,
    row_count: usize,
    route_path: String,
}

pub(crate) fn recurring_stop_gaps(rows: &[StopSlaRow]) -> Vec<RecurringStopGap> {
    let mut direct_pairs = std::collections::HashMap::<String, (&StopSlaRow, f64)>::new();
    for row in rows {
        let stops = row.stop_path.split(';').collect::<Vec<_>>();
        if stops.len() != 2 {
            continue;
        }
        direct_pairs.insert(
            normalized_stop_pair(&row.origin_id, &row.dest_id),
            (row, row.network_miles),
        );
    }

    let mut counts = std::collections::HashMap::<String, usize>::new();
    for row in rows {
        let stops = row.stop_path.split(';').collect::<Vec<_>>();
        for pair in stops.windows(2) {
            *counts
                .entry(normalized_stop_pair(pair[0], pair[1]))
                .or_default() += 1;
        }
    }

    let mut gaps = counts
        .into_iter()
        .filter_map(|(segment_id, row_count)| {
            let (direct, miles) = direct_pairs.get(&segment_id)?;
            Some(RecurringStopGap {
                segment_id,
                labels: format!("{} to {}", direct.origin_label, direct.dest_label),
                miles: *miles,
                row_count,
                route_path: direct.route_path.clone(),
            })
        })
        .collect::<Vec<_>>();
    gaps.sort_by(|a, b| {
        b.miles
            .total_cmp(&a.miles)
            .then_with(|| b.row_count.cmp(&a.row_count))
            .then_with(|| a.segment_id.cmp(&b.segment_id))
    });
    gaps
}

pub(crate) fn normalized_stop_pair(a: &str, b: &str) -> String {
    if a <= b {
        format!("{a}->{b}")
    } else {
        format!("{b}->{a}")
    }
}

#[derive(Debug)]
struct StopSlaCandidateRecommendation {
    gap: RecurringStopGap,
    candidates: Vec<StopSlaCandidateScore>,
}

#[derive(Debug)]
struct StopSlaCandidateScore {
    stop_id: String,
    name: String,
    lat: f64,
    lon: f64,
    requested_class: String,
    route_refs: String,
    evidence_status: String,
    source_type: String,
    basis: String,
    spacing_gain_miles: f64,
    largest_resulting_gap_miles: f64,
    distance_from_segment_miles: f64,
    intersection_route_count: usize,
    score: f64,
}

pub(crate) fn stop_sla_candidate_recommendations(
    rows: &[StopSlaRow],
    stop_rows: &[StopCandidateRow],
    city_rows: &[CitySeedRow],
    target_gap: f64,
    top: usize,
) -> Vec<StopSlaCandidateRecommendation> {
    let catalog = route_map::beck_stop_catalog()
        .into_iter()
        .map(|stop| (stop.id.to_string(), stop))
        .collect::<std::collections::HashMap<_, _>>();
    recurring_stop_gaps(rows)
        .into_iter()
        .filter(|gap| gap.miles > target_gap)
        .take(top)
        .map(|gap| {
            let candidates = score_stop_candidates_for_gap(&gap, stop_rows, city_rows, &catalog);
            StopSlaCandidateRecommendation { gap, candidates }
        })
        .collect()
}

// `score_stop_candidates_for_gap` moved to support::misc::score_stop_candidates_for_gap

pub(crate) fn algorithmic_midpoint_candidate(
    gap: &RecurringStopGap,
    from: &route_map::BeckStopCatalogRow,
    to: &route_map::BeckStopCatalogRow,
    route_set: &std::collections::BTreeSet<String>,
) -> StopSlaCandidateScore {
    let midpoint_gap = gap.miles / 2.0;
    let midpoint_lat = (from.lat + to.lat) / 2.0;
    let midpoint_lon = midpoint_lon(from.lon, to.lon);
    let route_refs = if route_set.is_empty() {
        gap.route_path.clone()
    } else {
        route_set.iter().cloned().collect::<Vec<_>>().join(";")
    };
    StopSlaCandidateScore {
        stop_id: format!("DRAFT-MID-{}-{}", from.id, to.id),
        name: format!("{} / {} midpoint", from.label, to.label),
        lat: midpoint_lat,
        lon: midpoint_lon,
        requested_class: "S4?".to_string(),
        route_refs,
        evidence_status: "draft-algorithmic-midpoint".to_string(),
        source_type: "algorithmic-midpoint".to_string(),
        basis: "computed midpoint for spacing only; choose nearest real interchange/service city"
            .to_string(),
        spacing_gain_miles: gap.miles - midpoint_gap,
        largest_resulting_gap_miles: midpoint_gap,
        distance_from_segment_miles: 0.0,
        intersection_route_count: route_set.len().max(1),
        score: gap.miles - midpoint_gap,
    }
}

pub(crate) fn midpoint_lon(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

#[derive(Debug, serde::Deserialize)]
struct CitySeedFile {
    cities: Vec<CitySeedRow>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct CitySeedRow {
    name: String,
    abbr: String,
    lat: f64,
    lon: f64,
}

pub(crate) fn load_city_rows(path: &Path) -> Result<Vec<CitySeedRow>> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let file: CitySeedFile =
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
    Ok(file.cities)
}

pub(crate) fn print_stop_sla_candidate_recommendations(
    recommendations: &[StopSlaCandidateRecommendation],
    target_gap: f64,
    candidates_per_gap: usize,
) {
    println!("  target gap: >{target_gap:.0} mi");
    println!("  inspected gaps: {}", recommendations.len());
    println!();
    for rec in recommendations {
        println!(
            "{}  {:.0} mi  rows={}  routes={}",
            rec.gap.segment_id, rec.gap.miles, rec.gap.row_count, rec.gap.route_path
        );
        println!("  {}", rec.gap.labels);
        if rec.candidates.is_empty() {
            println!("  no ledger candidates near this segment");
            println!();
            continue;
        }
        println!(
            "  {:<16} {:<24} {:<5} {:>7} {:>7} {:>7} {:>5} {:<12} Routes",
            "Stop", "Name", "Class", "NewMax", "Gain", "Offset", "Xfer", "Source"
        );
        for candidate in rec.candidates.iter().take(candidates_per_gap.max(1)) {
            println!(
                "  {:<16} {:<24} {:<5} {:>7.0} {:>7.0} {:>7.0} {:>5} {:<12} {}",
                candidate.stop_id,
                truncate_for_table(&candidate.name, 24),
                candidate.requested_class,
                candidate.largest_resulting_gap_miles,
                candidate.spacing_gain_miles,
                candidate.distance_from_segment_miles,
                candidate.intersection_route_count,
                truncate_for_table(&candidate.source_type, 12),
                truncate_for_table(&candidate.route_refs, 28)
            );
            println!(
                "    score={:.1} evidence={}",
                candidate.score, candidate.evidence_status
            );
        }
        println!();
    }
}

// `write_stop_sla_candidate_recommendations` moved to support::misc

#[allow(dead_code)]
#[derive(Debug, serde::Deserialize)]
struct StopSlaCandidateDocketRow {
    gap_segment: String,
    gap_labels: String,
    gap_miles: f64,
    gap_row_count: usize,
    gap_routes: String,
    candidate_rank: usize,
    candidate_id: String,
    candidate_name: String,
    candidate_class: String,
    candidate_lat: String,
    candidate_lon: String,
    candidate_source_type: String,
    candidate_evidence_status: String,
    candidate_route_refs: String,
    candidate_basis: String,
    largest_resulting_gap_miles: f64,
    spacing_gain_miles: f64,
    offset_miles: f64,
    intersection_route_count: usize,
    score: f64,
}

pub(crate) fn parse_stop_sla_candidate_docket<R: std::io::Read>(
    reader: R,
) -> Result<Vec<StopSlaCandidateDocketRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    rdr.deserialize()
        .collect::<std::result::Result<Vec<_>, _>>()
        .context("parsing stop SLA candidate docket")
}

pub(crate) fn stop_sla_promotion_rows(
    docket: &[StopSlaCandidateDocketRow],
    include_ledger: bool,
    include_alternates: bool,
) -> Vec<StopCandidateRow> {
    let mut seen_gaps = std::collections::BTreeSet::new();
    let mut rows = Vec::new();
    for row in docket {
        if !include_ledger && row.candidate_source_type == "stop-ledger" {
            continue;
        }
        if !include_alternates && !seen_gaps.insert(row.gap_segment.clone()) {
            continue;
        }
        rows.push(stop_sla_promotion_row(row));
    }
    rows
}

// `stop_sla_promotion_row` moved to support::misc

pub(crate) fn denormalized_route_refs(routes: &str) -> String {
    routes
        .split([';', ','])
        .map(str::trim)
        .filter(|route| !route.is_empty())
        .map(|route| {
            let norm = normalise_designation(route);
            if let Some(rest) = norm.strip_prefix('I') {
                format!("I-{rest}")
            } else if let Some(rest) = norm.strip_prefix("US") {
                format!("US{rest}")
            } else {
                norm
            }
        })
        .collect::<Vec<_>>()
        .join("; ")
}

// `write_stop_sla_promotions` moved to support::misc

pub(crate) fn geo_distance_miles(a_lat: f64, a_lon: f64, b_lat: f64, b_lon: f64) -> f64 {
    let earth_radius_miles = 3958.8_f64;
    let dlat = (b_lat - a_lat).to_radians();
    let dlon = (b_lon - a_lon).to_radians();
    let h = (dlat / 2.0).sin().powi(2)
        + a_lat.to_radians().cos() * b_lat.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    2.0 * earth_radius_miles * h.sqrt().asin() * 1.18
}

pub(crate) fn projection_fraction(
    a_lat: f64,
    a_lon: f64,
    b_lat: f64,
    b_lon: f64,
    p_lat: f64,
    p_lon: f64,
) -> f64 {
    let lat0 = ((a_lat + b_lat + p_lat) / 3.0).to_radians();
    let ax = a_lon * lat0.cos();
    let ay = a_lat;
    let bx = b_lon * lat0.cos();
    let by = b_lat;
    let px = p_lon * lat0.cos();
    let py = p_lat;
    let dx = bx - ax;
    let dy = by - ay;
    let len2 = dx * dx + dy * dy;
    if len2 <= f64::EPSILON {
        0.0
    } else {
        ((px - ax) * dx + (py - ay) * dy) / len2
    }
}

pub(crate) fn distance_to_geo_segment_miles(
    a_lat: f64,
    a_lon: f64,
    b_lat: f64,
    b_lon: f64,
    p_lat: f64,
    p_lon: f64,
) -> f64 {
    let t = projection_fraction(a_lat, a_lon, b_lat, b_lon, p_lat, p_lon).clamp(0.0, 1.0);
    let lat = a_lat + (b_lat - a_lat) * t;
    let lon = a_lon + (b_lon - a_lon) * t;
    geo_distance_miles(lat, lon, p_lat, p_lon)
}

pub(crate) fn rounded_score(score: f64) -> f64 {
    (score * 10.0).round() / 10.0
}

pub(crate) fn dimension_score_values(scores: &route_score::DimensionScores) -> [f64; 16] {
    [
        scores.a1.score,
        scores.a2.score,
        scores.a3.score,
        scores.a4.score,
        scores.a5.score,
        scores.b1.score,
        scores.b2.score,
        scores.b3.score,
        scores.b4.score,
        scores.c1.score,
        scores.c2.score,
        scores.c3.score,
        scores.c4.score,
        scores.d1.score,
        scores.d2.score,
        scores.d3.score,
    ]
}

pub(crate) fn dimension_estimated_values(scores: &route_score::DimensionScores) -> [bool; 16] {
    [
        scores.a1.estimated,
        scores.a2.estimated,
        scores.a3.estimated,
        scores.a4.estimated,
        scores.a5.estimated,
        scores.b1.estimated,
        scores.b2.estimated,
        scores.b3.estimated,
        scores.b4.estimated,
        scores.c1.estimated,
        scores.c2.estimated,
        scores.c3.estimated,
        scores.c4.estimated,
        scores.d1.estimated,
        scores.d2.estimated,
        scores.d3.estimated,
    ]
}

pub(crate) fn dimension_confidence_values(scores: &route_score::DimensionScores) -> [f32; 16] {
    [
        scores.a1.confidence,
        scores.a2.confidence,
        scores.a3.confidence,
        scores.a4.confidence,
        scores.a5.confidence,
        scores.b1.confidence,
        scores.b2.confidence,
        scores.b3.confidence,
        scores.b4.confidence,
        scores.c1.confidence,
        scores.c2.confidence,
        scores.c3.confidence,
        scores.c4.confidence,
        scores.d1.confidence,
        scores.d2.confidence,
        scores.d3.confidence,
    ]
}

pub(crate) fn dimension_confidence_risks(scores: &[f64; 16], confidences: &[f32; 16]) -> [f64; 16] {
    let mut risks = [0.0; 16];
    for d in 0..16 {
        risks[d] = scores[d] * (1.0 - confidences[d].clamp(0.0, 1.0) as f64);
    }
    risks
}

pub(crate) fn confidence_risk_dimensions(scores: &[f64; 16], confidences: &[f32; 16]) -> String {
    let contribution = dimension_confidence_risks(scores, confidences);
    let mut risks: Vec<(&str, f64, f32, f64)> = DIMENSION_CODES
        .iter()
        .zip(scores.iter())
        .zip(confidences.iter())
        .zip(contribution.iter())
        .filter_map(|(((code, score), confidence), risk)| {
            let score = *score;
            let confidence = confidence.clamp(0.0, 1.0);
            if *risk >= 1.0 {
                Some((*code, score, confidence, *risk))
            } else {
                None
            }
        })
        .collect();

    risks.sort_by(|a, b| {
        b.3.total_cmp(&a.3)
            .then_with(|| b.1.total_cmp(&a.1))
            .then_with(|| a.0.cmp(b.0))
    });

    risks
        .into_iter()
        .take(3)
        .map(|(code, score, confidence, _)| format!("{code}:{score:.1}@{confidence:.2}"))
        .collect::<Vec<_>>()
        .join(";")
}

pub(crate) fn write_tier_artifacts(score_rows: &[ScoreAllRow]) -> Result<()> {
    write_tier_artifacts_to(score_rows, Path::new("data"))
}

pub(crate) fn gap_type_slug(gap_type: &GapType) -> &'static str {
    match gap_type {
        GapType::MissingLink => "missing-link",
        GapType::Bottleneck => "bottleneck",
        GapType::Resilience => "resilience",
        GapType::Intermodal => "intermodal",
    }
}

pub(crate) fn write_gap_report(gap_type: &GapType, output_path: &Path) -> Result<()> {
    if let Some(parent) = output_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut md = String::new();
    md.push_str(&format!("# Fault Lines — {}\n\n", gap_type_slug(gap_type)));
    md.push_str(&format!(
        "Generated by `route gap --type {}` on {}.\n\n",
        gap_type_slug(gap_type),
        route_date()
    ));

    match gap_type {
        GapType::MissingLink => write_missing_link_gap_section(&mut md)?,
        GapType::Bottleneck => write_bottleneck_gap_section(&mut md)?,
        GapType::Resilience => write_score_dimension_gap_section(
            &mut md,
            "Resilience Holes",
            "Routes with high D1 climate-resilience exposure under the current score ledger.",
            "D1",
            "D1 climate-resilience exposure",
            true,
        )?,
        GapType::Intermodal => write_intermodal_gap_section(&mut md)?,
    }

    std::fs::write(output_path, md)
        .with_context(|| format!("writing gap report {}", output_path.display()))
}

pub(crate) fn write_missing_link_gap_section(md: &mut String) -> Result<()> {
    md.push_str(
        "Source: `data/coverage-gaps.csv`, regenerated by `route coverage --threshold 30`.\n\n",
    );
    let mut rdr = csv::Reader::from_path("data/coverage-gaps.csv")
        .context("reading data/coverage-gaps.csv")?;
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    let mut rows = Vec::new();
    for record in rdr.records() {
        let record = record?;
        let class = csv_get(&record, 8).to_string();
        *counts.entry(class.clone()).or_default() += 1;
        if class == "candidate_access_gap" && rows.len() < 20 {
            rows.push(record);
        }
    }

    md.push_str("| Gap class | Counties |\n|---|---:|\n");
    for (class, count) in counts {
        md.push_str(&format!("| {class} | {count} |\n"));
    }
    md.push_str("\n## Top Candidate Access Gaps\n\n");
    md.push_str(
        "| County | State | Nearest mi | Population | Land sq mi |\n|---|---|---:|---:|---:|\n",
    );
    for row in rows {
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            csv_get(&row, 1),
            csv_get(&row, 2),
            csv_get(&row, 5),
            csv_get(&row, 6),
            csv_get(&row, 7)
        ));
    }
    md.push_str("\nInterpretation: `candidate_access_gap` rows are not automatic construction recommendations; they are counties whose Census internal point is beyond the threshold after separating non-CONUS and large-county centroid-risk rows.\n");
    Ok(())
}

pub(crate) fn write_bottleneck_gap_section(md: &mut String) -> Result<()> {
    md.push_str("Source: `data/atri-bottlenecks.csv` hand-curated from ATRI truck bottleneck reporting.\n\n");
    let score_rows = load_score_signal_rows()?;
    let mut rdr =
        csv::Reader::from_path("data/atri-bottlenecks.csv").context("reading ATRI bottlenecks")?;
    md.push_str("| Rank | Location | Route | State | Annual cost $M | A1 | A3 | B2 | Signal |\n|---:|---|---|---|---:|---:|---:|---:|---|\n");
    for record in rdr.records().take(20) {
        let row = record?;
        let route = normalise_designation(csv_get(&row, 2));
        let signal = score_rows.get(&route);
        let (a1, a3, b2, label) = signal
            .map(|s| {
                (
                    format!("{:.1}", s.a1),
                    format!("{:.1}", s.a3),
                    format!("{:.1}", s.b2),
                    bottleneck_signal_label(s),
                )
            })
            .unwrap_or_else(|| ("".to_string(), "".to_string(), "".to_string(), "data_gap"));
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            csv_get(&row, 0),
            csv_get(&row, 1),
            csv_get(&row, 2),
            csv_get(&row, 3),
            csv_get(&row, 4),
            a1,
            a3,
            b2,
            label
        ));
    }
    md.push_str("\nInterpretation: ATRI bottlenecks are observed freight congestion seeds. `corridor_stress` means A1/A3 confirms broad congestion or reliability stress; `topology_chokepoint` means the route is central but the congestion is likely local/interchange-specific; `capacity_needs_flow` needs `route flow` or segment-level validation before being labeled structural capacity.\n");
    Ok(())
}

#[derive(Debug)]
struct ScoreSignalRow {
    a1: f64,
    a3: f64,
    b2: f64,
}

pub(crate) fn load_score_signal_rows() -> Result<std::collections::HashMap<String, ScoreSignalRow>> {
    let mut rdr = csv::Reader::from_path("data/scores-all.csv").context("reading scores-all")?;
    let headers = rdr.headers()?.clone();
    let route_idx = headers.iter().position(|h| h == "route").unwrap_or(0);
    let a1_idx = headers.iter().position(|h| h == "A1").unwrap_or(9);
    let a3_idx = headers.iter().position(|h| h == "A3").unwrap_or(11);
    let b2_idx = headers.iter().position(|h| h == "B2").unwrap_or(15);
    let mut rows = std::collections::HashMap::new();
    for record in rdr.records() {
        let row = record?;
        rows.insert(
            normalise_designation(csv_get(&row, route_idx)),
            ScoreSignalRow {
                a1: csv_get(&row, a1_idx).parse().unwrap_or(0.0),
                a3: csv_get(&row, a3_idx).parse().unwrap_or(0.0),
                b2: csv_get(&row, b2_idx).parse().unwrap_or(0.0),
            },
        );
    }
    Ok(rows)
}

pub(crate) fn bottleneck_signal_label(row: &ScoreSignalRow) -> &'static str {
    if row.a1 >= 7.0 || row.a3 >= 7.0 {
        "corridor_stress"
    } else if row.b2 >= 8.0 {
        "topology_chokepoint"
    } else {
        "capacity_needs_flow"
    }
}

// `write_score_dimension_gap_section` moved to support::misc

pub(crate) fn write_intermodal_gap_section(md: &mut String) -> Result<()> {
    md.push_str("Source: `data/scores-all.csv`; candidates here have high B3 port/border access but low D2 multimodal integration.\n\n");
    let mut rdr = csv::Reader::from_path("data/scores-all.csv").context("reading scores-all")?;
    let headers = rdr.headers()?.clone();
    let b3_idx = headers.iter().position(|h| h == "B3").unwrap_or(16);
    let d2_idx = headers.iter().position(|h| h == "D2").unwrap_or(23);
    let b3_conf_idx = headers.iter().position(|h| h == "B3_conf").unwrap_or(32);
    let d2_conf_idx = headers.iter().position(|h| h == "D2_conf").unwrap_or(39);
    let mut rows = Vec::new();
    for record in rdr.records() {
        let row = record?;
        let b3 = csv_get(&row, b3_idx).parse::<f64>().unwrap_or(0.0);
        let d2 = csv_get(&row, d2_idx).parse::<f64>().unwrap_or(0.0);
        if b3 >= 8.0 && d2 <= 5.0 {
            rows.push((b3 - d2, row));
        }
    }
    rows.sort_by(|a, b| b.0.total_cmp(&a.0));
    md.push_str("| Route | Score | Tier | B3 port/border | B3 conf | D2 multimodal | D2 conf | Claim label |\n|---|---:|---|---:|---:|---:|---:|---|\n");
    for (_, row) in rows.into_iter().take(20) {
        let b3_conf = csv_get(&row, b3_conf_idx).parse::<f32>().unwrap_or(0.0);
        let d2_conf = csv_get(&row, d2_conf_idx).parse::<f32>().unwrap_or(0.0);
        let claim_conf = b3_conf.min(d2_conf);
        md.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            csv_get(&row, 0),
            csv_get(&row, 1),
            csv_get(&row, 2),
            csv_get(&row, b3_idx),
            csv_get(&row, b3_conf_idx),
            csv_get(&row, d2_idx),
            csv_get(&row, d2_conf_idx),
            route_score::confidence_label(claim_conf)
        ));
    }
    md.push_str("\nInterpretation: these are port/border-adjacent corridors whose multimodal support is weak under the current source model; validate terminal and connector data before elevating them to projects.\n");
    Ok(())
}

pub(crate) fn csv_get(record: &csv::StringRecord, idx: usize) -> &str {
    record.get(idx).unwrap_or("")
}

pub(crate) fn route_date() -> String {
    std::env::var("ROUTE_DATE").unwrap_or_else(|_| "2026-05-06".to_string())
}

// `write_tier_artifacts_to` moved to support::misc

pub(crate) fn atlas_candidate_ids(graph: &route_network::HighwayGraph) -> Vec<String> {
    let mut ids = graph.interstate_ids();
    ids.extend(graph.us_highway_ids());
    ids.sort();
    ids.dedup();
    ids
}

#[derive(Debug, Clone, serde::Deserialize)]
struct MapAtlasRow {
    map_id: String,
    path: String,
    map_type: String,
    render_command: String,
    expected_width: u32,
    expected_height: u32,
    min_bytes: u64,
    tier_role: String,
    game_use: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct MapPublicationScopeDecisionRow {
    decision_id: String,
    decision_scope: String,
    map_surface: String,
    render_gate_status: String,
    evidence_gate_status: String,
    claim_status: String,
    blocked_claims: String,
    claim_blocker_count: usize,
    budget_debt_count: usize,
    blocking_artifacts: String,
    scope_treatment: String,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct MapPublicationReadinessRow {
    readiness_id: String,
    map_surface: String,
    map_count: usize,
    map_types: String,
    render_gate_status: String,
    scope_decision_status: String,
    publication_blocker_count: usize,
    publication_blocker_families: String,
    held_claims: String,
    held_claim_family_count: usize,
    budget_debt_count: usize,
    scope_decision_artifact: String,
    backlog_artifact: String,
    readiness_decision: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct MapPublicationInventoryRow {
    map_id: String,
    map_path: String,
    map_type: String,
    publication_status: String,
    render_gate_status: String,
    readiness_artifact: String,
    held_claims: String,
    required_label: String,
    allowed_use: String,
    not_allowed_claims: String,
    next_artifact: String,
    validation_status: String,
}

pub(crate) fn load_map_atlas(path: &Path) -> Result<Vec<MapAtlasRow>> {
    let file = std::fs::File::open(path)?;
    parse_map_atlas(file)
}

pub(crate) fn parse_map_atlas<R: std::io::Read>(reader: R) -> Result<Vec<MapAtlasRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

pub(crate) fn print_map_atlas(rows: &[MapAtlasRow], details: bool) {
    let failures = map_atlas_gate_failures(rows);
    let mut by_type = std::collections::BTreeMap::new();
    for row in rows {
        *by_type.entry(row.map_type.clone()).or_insert(0usize) += 1;
    }

    println!("route map-atlas");
    println!("  maps: {}", rows.len());
    println!("  types: {}", format_count_map(&by_type));
    println!("  gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<18} {:<22} {:<14} {:<12} {}",
        "Map", "Path", "Type", "Contract", "Use"
    );
    println!("{}", "-".repeat(112));
    for row in rows {
        let contract = match png_dimensions(&map_atlas_artifact_path(&row.path)) {
            Some((width, height)) => format!("{width}x{height}"),
            None => "missing".to_string(),
        };
        println!(
            "{:<18} {:<22} {:<14} {:<12} {}",
            row.map_id,
            truncate_for_table(&row.path, 22),
            row.map_type,
            contract,
            row.tier_role
        );
        if details {
            println!("  command: {}", row.render_command);
            println!("  game: {}", row.game_use);
        }
    }
}

pub(crate) fn map_atlas_gate_failures(rows: &[MapAtlasRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("map atlas has no rows".to_string());
    }
    for row in rows {
        if row.map_id.trim().is_empty()
            || row.path.trim().is_empty()
            || row.map_type.trim().is_empty()
            || row.render_command.trim().is_empty()
            || row.tier_role.trim().is_empty()
            || row.game_use.trim().is_empty()
        {
            failures.push(format!("{} has empty manifest fields", row.map_id));
            continue;
        }
        let path = map_atlas_artifact_path(&row.path);
        let Ok(metadata) = std::fs::metadata(&path) else {
            failures.push(format!("{} missing {}", row.map_id, row.path));
            continue;
        };
        if metadata.len() < row.min_bytes {
            failures.push(format!(
                "{} too small: {} bytes < {}",
                row.map_id,
                metadata.len(),
                row.min_bytes
            ));
        }
        match png_dimensions(&path) {
            Some((width, height))
                if width == row.expected_width && height == row.expected_height => {}
            Some((width, height)) => failures.push(format!(
                "{} dimensions {}x{} != {}x{}",
                row.map_id, width, height, row.expected_width, row.expected_height
            )),
            None => failures.push(format!("{} is not a readable PNG", row.map_id)),
        }
    }
    failures
}

pub(crate) fn png_dimensions(path: &Path) -> Option<(u32, u32)> {
    let bytes = std::fs::read(path).ok()?;
    if bytes.len() < 24 || &bytes[0..8] != b"\x89PNG\r\n\x1a\n" {
        return None;
    }
    let width = u32::from_be_bytes(bytes[16..20].try_into().ok()?);
    let height = u32::from_be_bytes(bytes[20..24].try_into().ok()?);
    Some((width, height))
}

pub(crate) fn map_atlas_artifact_path(path: &str) -> PathBuf {
    let direct = PathBuf::from(path);
    if direct.exists() || direct.is_absolute() {
        direct
    } else {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(path)
    }
}

pub(crate) fn load_map_publication_scope_decision(path: &Path) -> Result<Vec<MapPublicationScopeDecisionRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `map_publication_readiness_rows` moved to support::misc

pub(crate) fn split_claim_tokens(claims: &str) -> Vec<&str> {
    claims
        .split(['|', ';', ','])
        .map(str::trim)
        .filter(|claim| !claim.is_empty())
        .collect()
}

pub(crate) fn write_map_publication_readiness(path: &Path, rows: &[MapPublicationReadinessRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_map_publication_readiness_summary(
    output: &Path,
    rows: &[MapPublicationReadinessRow],
    details: bool,
) {
    println!(
        "  wrote {} readiness rows to {}",
        rows.len(),
        output.display()
    );
    if let Some(row) = rows.first() {
        println!("  maps: {} ({})", row.map_count, row.map_types);
        println!("  render gate: {}", row.render_gate_status);
        println!("  publication blockers: {}", row.publication_blocker_count);
        println!("  held claims: {}", row.held_claims);
        println!("  status: {}", row.validation_status);
        if details {
            println!("  decision: {}", row.readiness_decision);
            println!(
                "  publication families: {}",
                row.publication_blocker_families
            );
        }
    }
}

pub(crate) fn map_publication_readiness_gate_failures(rows: &[MapPublicationReadinessRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("map publication readiness has no rows".to_string());
        return failures;
    }
    for row in rows {
        if row.map_count == 0 {
            failures.push(format!("{} has no map atlas rows", row.readiness_id));
        }
        if row.render_gate_status != "pass" {
            failures.push(format!("{} render gate is not pass", row.readiness_id));
        }
        if row.scope_decision_status != "pass" {
            failures.push(format!(
                "{} scope decision status is {}",
                row.readiness_id, row.scope_decision_status
            ));
        }
        if row.publication_blocker_count > 0 {
            failures.push(format!(
                "{} still has {} publication blockers ({})",
                row.readiness_id, row.publication_blocker_count, row.publication_blocker_families
            ));
        }
        if row.validation_status != "pass" {
            failures.push(format!("{} validation is not pass", row.readiness_id));
        }
        if split_claim_tokens(&row.held_claims)
            .iter()
            .any(|claim| *claim == "publication")
        {
            failures.push(format!("{} still holds publication", row.readiness_id));
        }
    }
    failures
}

pub(crate) fn load_map_publication_readiness(path: &Path) -> Result<Vec<MapPublicationReadinessRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_map_publication_inventory(path: &Path) -> Result<Vec<MapPublicationInventoryRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn print_map_publication_inventory_summary(
    path: &Path,
    rows: &[MapPublicationInventoryRow],
    details: bool,
) {
    let mut by_type = std::collections::BTreeMap::new();
    for row in rows {
        *by_type.entry(row.map_type.clone()).or_insert(0usize) += 1;
    }
    println!("  inventory: {}", path.display());
    println!("  maps: {}", rows.len());
    println!("  types: {}", format_count_map(&by_type));
    if details {
        for row in rows {
            println!(
                "  {} -> {} ({}) {}",
                row.map_id, row.map_path, row.map_type, row.publication_status
            );
        }
    }
}

// `map_publication_inventory_gate_failures` moved to support::gates

pub(crate) fn merge_hpms_state_records(
    mut existing: Vec<route_data::HpmsRecord>,
    fetched: Vec<route_data::HpmsRecord>,
    states: &std::collections::BTreeSet<String>,
) -> Vec<route_data::HpmsRecord> {
    existing.retain(|row| !states.contains(&row.state.to_ascii_uppercase()));
    existing.extend(fetched);
    existing.sort_by(|a, b| {
        a.state
            .cmp(&b.state)
            .then_with(|| a.route_id.cmp(&b.route_id))
            .then_with(|| a.aadt.cmp(&b.aadt))
    });
    existing
}

pub(crate) fn parse_hpms_functional_systems(value: &str) -> Result<Vec<u8>> {
    let systems = value
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            part.parse::<u8>()
                .with_context(|| format!("invalid HPMS functional system {part}"))
        })
        .collect::<Result<Vec<_>>>()?;
    if systems.is_empty() {
        anyhow::bail!("at least one HPMS functional system is required");
    }
    let systems = systems
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if systems.iter().any(|system| !(1..=7).contains(system)) {
        anyhow::bail!("HPMS functional systems must be in 1..=7");
    }
    Ok(systems)
}

// `write_hpms_records` moved to support::misc

pub(crate) fn atomic_write_text(path: &Path, text: impl AsRef<str>) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let tmp = temp_path_for_atomic_write(path);
    std::fs::write(&tmp, text.as_ref()).with_context(|| format!("writing {}", tmp.display()))?;
    replace_with_atomic_write(&tmp, path)
}

pub(crate) fn temp_path_for_atomic_write(path: &Path) -> PathBuf {
    let mut file_name = path
        .file_name()
        .map(|value| value.to_os_string())
        .unwrap_or_else(|| "route-cache".into());
    file_name.push(format!(".{}.tmp", std::process::id()));
    path.with_file_name(file_name)
}

pub(crate) fn replace_with_atomic_write(tmp: &Path, path: &Path) -> Result<()> {
    if path.exists() {
        std::fs::remove_file(path)
            .with_context(|| format!("removing previous {}", path.display()))?;
    }
    std::fs::rename(tmp, path)
        .with_context(|| format!("replacing {} with {}", path.display(), tmp.display()))?;
    Ok(())
}

#[derive(Debug, Clone)]
struct ScenarioEdgeCandidate {
    edge_id: u64,
    distance_miles: f64,
    length_miles: f64,
    aadt: Option<u32>,
    lanes: Option<u8>,
    state: String,
    mid_lat: f64,
    mid_lon: f64,
}

pub(crate) fn scenario_edge_candidates(
    graph: &route_network::HighwayGraph,
    route: &str,
    lat: f64,
    lon: f64,
    radius_miles: f64,
    top: usize,
) -> Vec<ScenarioEdgeCandidate> {
    let mut candidates: Vec<ScenarioEdgeCandidate> = graph
        .route_edges(route)
        .iter()
        .filter_map(|&ei| {
            let edge = &graph.graph[ei];
            let (mid_lat, mid_lon) = edge_midpoint(edge)?;
            let distance_miles = haversine_miles(lat, lon, mid_lat, mid_lon);
            (distance_miles <= radius_miles).then(|| ScenarioEdgeCandidate {
                edge_id: edge.id,
                distance_miles,
                length_miles: edge.length_miles,
                aadt: edge.aadt,
                lanes: edge.lane_count,
                state: edge.state.clone(),
                mid_lat,
                mid_lon,
            })
        })
        .collect();

    candidates.sort_by(|a, b| {
        a.distance_miles
            .partial_cmp(&b.distance_miles)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.edge_id.cmp(&b.edge_id))
    });
    candidates.truncate(top);
    candidates
}

pub(crate) fn edge_midpoint(edge: &route_network::HighwayEdge) -> Option<(f64, f64)> {
    let coords = edge.geometry.0.as_slice();
    if coords.is_empty() {
        return None;
    }
    let idx = coords.len() / 2;
    let coord = coords[idx];
    Some((coord.y, coord.x))
}

pub(crate) fn haversine_miles(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let r = 3958.8_f64;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    2.0 * r * a.sqrt().asin()
}

#[derive(Debug, Clone, serde::Deserialize)]
struct StandardsProofRow {
    standard_id: String,
    tier: String,
    standard_family: String,
    standard: String,
    outcome: String,
    mechanism: String,
    primary_stressor: String,
    acceptance_gate: String,
    evidence_level: String,
    current_artifact: String,
    blocking_gap: String,
    next_command_or_test: String,
    owner_track: String,
}

pub(crate) fn load_standards_proof_ledger(path: &Path) -> Result<Vec<StandardsProofRow>> {
    let file = std::fs::File::open(path)?;
    parse_standards_proof_ledger(file)
}

pub(crate) fn parse_standards_proof_ledger<R: std::io::Read>(reader: R) -> Result<Vec<StandardsProofRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

pub(crate) fn standards_evidence_level_is_allowed(level: &str) -> bool {
    matches!(
        level.trim().to_ascii_lowercase().as_str(),
        "implemented" | "heuristic" | "stub" | "planned" | "deprecated"
    )
}

pub(crate) fn standards_blueprint_gate_failures(rows: &[StandardsProofRow]) -> Vec<&StandardsProofRow> {
    rows.iter()
        .filter(|row| {
            !standards_evidence_level_is_allowed(&row.evidence_level)
                || !row.evidence_level.eq_ignore_ascii_case("Implemented")
                || !row.blocking_gap.trim().is_empty()
        })
        .collect()
}

pub(crate) fn standards_pressure_gate_failures(rows: &[StandardsProofRow]) -> Vec<&StandardsProofRow> {
    rows.iter()
        .filter(|row| !standards_pressure_row_has_contract(row))
        .collect()
}

pub(crate) fn standards_pressure_row_has_contract(row: &StandardsProofRow) -> bool {
    !row.standard_id.trim().is_empty()
        && !row.tier.trim().is_empty()
        && !row.standard_family.trim().is_empty()
        && !row.standard.trim().is_empty()
        && !row.outcome.trim().is_empty()
        && !row.mechanism.trim().is_empty()
        && !row.primary_stressor.trim().is_empty()
        && !row.acceptance_gate.trim().is_empty()
        && standards_evidence_level_is_allowed(&row.evidence_level)
        && !row.current_artifact.trim().is_empty()
        && !row.blocking_gap.trim().is_empty()
        && !row.next_command_or_test.trim().is_empty()
        && !row.owner_track.trim().is_empty()
}

// `print_standards_proof` moved to support::print

#[derive(Debug, Clone, serde::Deserialize)]
struct ForumDocketRow {
    review_id: String,
    artifact: String,
    review_type: String,
    status: String,
    roles: String,
    claim_target: String,
    blocking_question: String,
    next_action: String,
    output_artifact: String,
}

pub(crate) fn load_forum_docket(path: &Path) -> Result<Vec<ForumDocketRow>> {
    let file = std::fs::File::open(path)?;
    parse_forum_docket(file)
}

pub(crate) fn parse_forum_docket<R: std::io::Read>(reader: R) -> Result<Vec<ForumDocketRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

// `print_forum_docket` moved to support::print

pub(crate) fn forum_docket_gate_failures(rows: &[ForumDocketRow]) -> Vec<String> {
    if rows.is_empty() {
        return vec!["forum docket has no review rows".to_string()];
    }

    rows.iter()
        .filter_map(|row| forum_docket_row_failure(row))
        .collect()
}

pub(crate) fn forum_docket_row_failure(row: &ForumDocketRow) -> Option<String> {
    let review_type = row.review_type.trim().to_ascii_lowercase();
    let status = row.status.trim().to_ascii_lowercase();
    let type_ok = matches!(
        review_type.as_str(),
        "parliament" | "stakeholder" | "editorial" | "panel" | "owner"
    );
    let status_ok = matches!(status.as_str(), "planned" | "complete" | "held");
    let required_filled = !row.review_id.trim().is_empty()
        && !row.artifact.trim().is_empty()
        && !row.roles.trim().is_empty()
        && !row.claim_target.trim().is_empty()
        && !row.blocking_question.trim().is_empty()
        && !row.next_action.trim().is_empty()
        && !row.output_artifact.trim().is_empty();

    if !type_ok || !status_ok || !required_filled {
        Some(format!(
            "{} invalid contract: type={} status={} artifact={} output={}",
            if row.review_id.trim().is_empty() {
                "<missing-review-id>"
            } else {
                row.review_id.as_str()
            },
            row.review_type,
            row.status,
            row.artifact,
            row.output_artifact
        ))
    } else {
        None
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct SignificantMomentRow {
    moment_id: String,
    date: String,
    flair: String,
    kind: String,
    summary: String,
    why_it_mattered: String,
    primary_artifacts: String,
    commit: String,
    next_thread: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ReleaseManifestRow {
    artifact_path: String,
    artifact_class: String,
    owner_milepost: String,
    release_status: String,
    public_status: String,
    verification_command: String,
    notes: String,
}

pub(crate) fn load_significant_moments(path: &Path) -> Result<Vec<SignificantMomentRow>> {
    let file = std::fs::File::open(path)?;
    parse_significant_moments(file)
}

pub(crate) fn load_release_manifest(path: &Path) -> Result<Vec<ReleaseManifestRow>> {
    let file = std::fs::File::open(path)?;
    parse_release_manifest(file)
}

pub(crate) fn parse_release_manifest<R: std::io::Read>(reader: R) -> Result<Vec<ReleaseManifestRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

pub(crate) fn parse_significant_moments<R: std::io::Read>(reader: R) -> Result<Vec<SignificantMomentRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

// `print_significant_moments` moved to support::print

pub(crate) fn significant_moment_gate_failures(rows: &[SignificantMomentRow]) -> Vec<String> {
    if rows.is_empty() {
        return vec!["significant moment ledger has no rows".to_string()];
    }

    let mut seen_ids = std::collections::HashSet::new();
    let mut seen_flairs = std::collections::HashSet::new();
    let mut failures = Vec::new();

    for row in rows {
        if let Some(failure) = significant_moment_row_failure(row) {
            failures.push(failure);
        }

        let id = row.moment_id.trim();
        if !id.is_empty() && !seen_ids.insert(id.to_string()) {
            failures.push(format!("{id} duplicate moment_id"));
        }

        let flair = row.flair.trim().to_ascii_lowercase();
        if !flair.is_empty() && !seen_flairs.insert(flair) {
            failures.push(format!("{} duplicate flair '{}'", row.moment_id, row.flair));
        }
    }

    failures
}

// `print_release_manifest` moved to support::print

// `release_manifest_gate_failures` moved to support::gates

pub(crate) fn release_manifest_verification_command_allowed(row: &ReleaseManifestRow) -> bool {
    let command = row.verification_command.trim();
    if command == "manual review" {
        return release_manifest_manual_review_allowed(row);
    }
    if command == "cargo test --workspace" {
        return true;
    }
    if command.starts_with("powershell -ExecutionPolicy Bypass -File ") {
        return true;
    }
    let route_args = command
        .strip_prefix("cargo run -q -p route -- ")
        .or_else(|| command.strip_prefix("route "));
    let Some(route_args) = route_args else {
        return false;
    };
    let parts: Vec<&str> = route_args.split_whitespace().collect();
    if parts.iter().any(|part| part.starts_with("--gate")) {
        return true;
    }
    matches!(
        parts.as_slice(),
        ["score-all"] | ["beck-t1-diagnostics"] | ["gap", "--type", _]
    )
}

pub(crate) fn release_manifest_manual_review_allowed(row: &ReleaseManifestRow) -> bool {
    let path = row.artifact_path.trim();
    let class = row.artifact_class.trim();
    path.starts_with("docs/")
        || path.starts_with("specs/")
        || path == "TRACKER.md"
        || path.ends_with("phase-sequence.csv")
        || class.contains("doc")
        || class.contains("plan")
        || class.contains("review")
        || class.contains("closeout")
        || class.contains("spec")
        || class.contains("standard")
        || class.contains("policy")
        || class.contains("roadmap")
        || class.contains("index")
        || class.contains("status")
        || class.contains("script")
}

pub(crate) fn release_manifest_artifact_exists(path: &str) -> bool {
    repo_relative_artifact_path(path).exists()
}

pub(crate) fn release_manifest_artifact_path(path: &str) -> PathBuf {
    repo_relative_artifact_path(path)
}

pub(crate) fn repo_relative_artifact_path(path: &str) -> PathBuf {
    let direct = std::path::PathBuf::from(path);
    if direct.exists() || direct.is_absolute() {
        direct
    } else {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(path)
    }
}

pub(crate) fn release_manifest_optimizer_bundle_failures() -> Vec<String> {
    let manifest_path = release_manifest_artifact_path("data/tier-optimizer-runs.csv");
    let rows = match load_tier_optimizer_runs(&manifest_path) {
        Ok(rows) => rows,
        Err(error) => {
            return vec![format!(
                "data/tier-optimizer-runs.csv could not be loaded for release coverage: {error}"
            )];
        }
    };
    rows.iter()
        .filter(|row| matches!(row.gate_status.as_str(), "pass" | "held-known"))
        .filter(|row| !release_manifest_artifact_exists(&row.artifact))
        .map(|row| {
            format!(
                "{} optimizer artifact is missing from release coverage bundle",
                row.artifact
            )
        })
        .collect()
}

// `significant_moment_row_failure` moved to support::misc

pub(crate) fn looks_like_iso_date(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(idx, byte)| idx == 4 || idx == 7 || byte.is_ascii_digit())
}

pub(crate) fn looks_like_commit_ref(value: &str) -> bool {
    (7..=40).contains(&value.len()) && value.chars().all(|ch| ch.is_ascii_hexdigit())
}

pub(crate) fn missing_moment_artifacts(primary_artifacts: &str) -> Vec<String> {
    primary_artifacts
        .split(';')
        .map(str::trim)
        .filter(|artifact| !artifact.is_empty())
        .filter(|artifact| !moment_artifact_exists(artifact))
        .map(str::to_string)
        .collect()
}

pub(crate) fn moment_artifact_exists(artifact: &str) -> bool {
    let path = Path::new(artifact);
    if path.exists() {
        return true;
    }
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
        .exists()
}

#[derive(Debug, Clone, serde::Deserialize)]
struct BlueprintPackageRow {
    package_id: String,
    phase: String,
    feature_package: String,
    stakeholder_class: String,
    standards: String,
    evidence_level: String,
    status: String,
    cost_range: String,
    value_case: String,
    source_label: String,
    pressure_artifact: String,
    forum_constraint: String,
    mitigation_companion: String,
    row_complexity: String,
    maintenance_burden: String,
    community_exposure_check: String,
    rural_access_exception: String,
    blueprint_action: String,
    blocking_gap: String,
    next_evidence_step: String,
}

pub(crate) fn load_blueprint_packages(path: &Path) -> Result<Vec<BlueprintPackageRow>> {
    let file = std::fs::File::open(path)?;
    parse_blueprint_packages(file)
}

pub(crate) fn parse_blueprint_packages<R: std::io::Read>(reader: R) -> Result<Vec<BlueprintPackageRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

// `print_blueprint_packages` moved to support::print

// `blueprint_gate_failures` moved to support::gates

// `blueprint_row_contract_failure` moved to support::misc

pub(crate) fn blueprint_field_is_not_applicable(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "" | "n/a" | "not_applicable" | "none"
    )
}

#[derive(Debug, Clone, serde::Deserialize)]
struct BlueprintEvidenceRow {
    package_id: String,
    standard_id: String,
    proof_evidence_level: String,
    blueprint_claim_status: String,
    promotion_rule: String,
    proof_artifact: String,
    forum_hold: String,
    blocking_gap: String,
    required_next_evidence: String,
}

pub(crate) fn load_blueprint_evidence_map(path: &Path) -> Result<Vec<BlueprintEvidenceRow>> {
    let file = std::fs::File::open(path)?;
    parse_blueprint_evidence_map(file)
}

pub(crate) fn parse_blueprint_evidence_map<R: std::io::Read>(reader: R) -> Result<Vec<BlueprintEvidenceRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

// `print_blueprint_evidence_map` moved to support::print

// `blueprint_evidence_gate_failures` moved to support::gates

// `blueprint_evidence_row_failure` moved to support::misc

#[derive(Debug, Clone, serde::Deserialize)]
struct BlueprintCostRow {
    package_id: String,
    cost_basis: String,
    capital_range_2026_usd: String,
    lifecycle_burden: String,
    source_status: String,
    source_artifact: String,
    cost_claim_status: String,
    risk_note: String,
    next_cost_step: String,
}

pub(crate) fn load_blueprint_cost_ranges(path: &Path) -> Result<Vec<BlueprintCostRow>> {
    let file = std::fs::File::open(path)?;
    parse_blueprint_cost_ranges(file)
}

pub(crate) fn parse_blueprint_cost_ranges<R: std::io::Read>(reader: R) -> Result<Vec<BlueprintCostRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

// `print_blueprint_cost_ranges` moved to support::print

pub(crate) fn blueprint_cost_gate_failures(
    rows: &[BlueprintCostRow],
    packages: &[BlueprintPackageRow],
) -> Vec<String> {
    if rows.is_empty() {
        return vec!["blueprint cost ledger has no rows".to_string()];
    }

    let package_ids = packages
        .iter()
        .map(|row| row.package_id.as_str())
        .collect::<std::collections::HashSet<_>>();
    let mut failures = Vec::new();
    for row in rows {
        if let Some(failure) = blueprint_cost_row_failure(row, &package_ids) {
            failures.push(failure);
        }
    }

    for package in packages {
        if !rows
            .iter()
            .any(|row| row.package_id.trim() == package.package_id.trim())
        {
            failures.push(format!("{} missing cost range row", package.package_id));
        }
    }

    failures
}

pub(crate) fn blueprint_cost_row_failure(
    row: &BlueprintCostRow,
    package_ids: &std::collections::HashSet<&str>,
) -> Option<String> {
    let source_status = row.source_status.trim().to_ascii_lowercase();
    let claim_status = row.cost_claim_status.trim().to_ascii_lowercase();
    let source_status_ok = matches!(
        source_status.as_str(),
        "source_backed" | "planning_range" | "corridor_specific" | "source_needed"
    );
    let claim_status_ok = matches!(
        claim_status.as_str(),
        "source_backed" | "planning_only" | "placeholder" | "held"
    );
    let no_premature_source_claim =
        source_status == "source_backed" || claim_status != "source_backed";
    let filled = !row.package_id.trim().is_empty()
        && package_ids.contains(row.package_id.trim())
        && !row.cost_basis.trim().is_empty()
        && !row.capital_range_2026_usd.trim().is_empty()
        && !row.lifecycle_burden.trim().is_empty()
        && source_status_ok
        && !row.source_artifact.trim().is_empty()
        && claim_status_ok
        && no_premature_source_claim
        && !row.risk_note.trim().is_empty()
        && !row.next_cost_step.trim().is_empty();

    if filled {
        None
    } else {
        Some(format!(
            "{} invalid cost row: source={} claim={}",
            if row.package_id.trim().is_empty() {
                "<missing-package-id>"
            } else {
                row.package_id.as_str()
            },
            row.source_status,
            row.cost_claim_status
        ))
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct StandardsInventoryRow {
    standard_id: String,
    inventory_name: String,
    source_kind: String,
    source_status: String,
    current_artifact: String,
    coverage_scope: String,
    blocking_gap: String,
    next_step: String,
}

pub(crate) fn load_standards_inventory(path: &Path) -> Result<Vec<StandardsInventoryRow>> {
    let file = std::fs::File::open(path)?;
    parse_standards_inventory(file)
}

pub(crate) fn parse_standards_inventory<R: std::io::Read>(reader: R) -> Result<Vec<StandardsInventoryRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

pub(crate) fn print_standards_inventory(rows: &[StandardsInventoryRow], blockers: bool, details: bool) {
    let failures = standards_inventory_gate_failures(rows);
    let filtered = if blockers {
        failures.clone()
    } else {
        rows.iter().collect::<Vec<_>>()
    };
    let mut by_status: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for row in rows {
        *by_status.entry(row.source_status.clone()).or_insert(0) += 1;
    }

    println!("route standards-inventory");
    println!("  rows: {} shown / {} total", filtered.len(), rows.len());
    println!("  source status: {}", format_count_map(&by_status));
    println!("  gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<24} {:<22} {:<14} {:<18} {}",
        "Standard", "Inventory", "Status", "Source", "Gap"
    );
    println!("{}", "-".repeat(122));
    for row in filtered {
        println!(
            "{:<24} {:<22} {:<14} {:<18} {}",
            row.standard_id,
            truncate_for_table(&row.inventory_name, 22),
            row.source_status,
            truncate_for_table(&row.source_kind, 18),
            row.blocking_gap
        );
        if details {
            println!("  artifact: {}", row.current_artifact);
            println!("  scope: {}", row.coverage_scope);
            println!("  next: {}", row.next_step);
        }
    }
}

pub(crate) fn standards_inventory_gate_failures(
    rows: &[StandardsInventoryRow],
) -> Vec<&StandardsInventoryRow> {
    rows.iter()
        .filter(|row| !standards_inventory_row_has_contract(row))
        .collect()
}

pub(crate) fn standards_inventory_row_has_contract(row: &StandardsInventoryRow) -> bool {
    let status = row.source_status.trim().to_ascii_lowercase();
    let status_is_labeled = matches!(
        status.as_str(),
        "implemented" | "partial" | "source_needed" | "access_gated" | "planned"
    );
    !row.standard_id.trim().is_empty()
        && !row.inventory_name.trim().is_empty()
        && !row.source_kind.trim().is_empty()
        && status_is_labeled
        && !row.current_artifact.trim().is_empty()
        && !row.coverage_scope.trim().is_empty()
        && !row.blocking_gap.trim().is_empty()
        && !row.next_step.trim().is_empty()
}

pub(crate) fn planned_standard_inventory_missing<'a>(
    standards: &'a [StandardsProofRow],
    inventories: &[StandardsInventoryRow],
) -> Vec<&'a StandardsProofRow> {
    let covered = inventories
        .iter()
        .map(|row| row.standard_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    standards
        .iter()
        .filter(|row| row.evidence_level.eq_ignore_ascii_case("Planned"))
        .filter(|row| !covered.contains(row.standard_id.as_str()))
        .collect()
}

#[derive(Debug, Clone, serde::Deserialize)]
struct PavementStandardRow {
    tier: String,
    road_role: String,
    max_iri_m_per_km: f64,
    target_pavement_condition: String,
    freight_ride_requirement: String,
    transit_ride_requirement: String,
    inspection_interval_months: u16,
    repair_trigger: String,
    allowed_exception: String,
    source_contract: String,
    validation_status: String,
}

pub(crate) fn load_pavement_standards(path: &Path) -> Result<Vec<PavementStandardRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn print_pavement_standards(rows: &[PavementStandardRow], blockers: bool, details: bool) {
    let failures = pavement_standard_gate_failures(rows);
    let failure_tiers = failures
        .iter()
        .filter_map(|failure| failure.split_whitespace().next())
        .collect::<std::collections::BTreeSet<_>>();
    let filtered = if blockers {
        rows.iter()
            .filter(|row| failure_tiers.contains(row.tier.as_str()))
            .collect::<Vec<_>>()
    } else {
        rows.iter().collect::<Vec<_>>()
    };

    println!("route standards-pavement");
    println!("  rows: {} shown / {} total", filtered.len(), rows.len());
    println!("  gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<4} {:<24} {:>7} {:<12} {}",
        "Tier", "Role", "IRI", "Condition", "Repair trigger"
    );
    println!("{}", "-".repeat(112));
    for row in filtered {
        println!(
            "{:<4} {:<24} {:>7.2} {:<12} {}",
            row.tier,
            truncate_for_table(&row.road_role, 24),
            row.max_iri_m_per_km,
            row.target_pavement_condition,
            row.repair_trigger
        );
        if details {
            println!("  freight: {}", row.freight_ride_requirement);
            println!("  transit: {}", row.transit_ride_requirement);
            println!("  inspection: {} months", row.inspection_interval_months);
            println!("  exception: {}", row.allowed_exception);
            println!("  source: {}", row.source_contract);
            println!("  status: {}", row.validation_status);
        }
    }
}

// `pavement_standard_gate_failures` moved to support::pavement

pub(crate) fn load_tier_routes(path: &Path, tier: &str) -> Result<Vec<String>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let headers = rdr.headers()?.clone();
    let tier_idx = headers
        .iter()
        .position(|value| value == "tier")
        .context("tier table missing tier column")?;
    let route_idx = headers
        .iter()
        .position(|value| value == "route")
        .context("tier table missing route column")?;
    let mut routes = Vec::new();
    for result in rdr.records() {
        let row = result?;
        if row
            .get(tier_idx)
            .unwrap_or("")
            .trim()
            .eq_ignore_ascii_case(tier)
        {
            let route = normalise_designation(row.get(route_idx).unwrap_or("").trim());
            if !route.is_empty() {
                routes.push(route);
            }
        }
    }
    routes.sort();
    routes.dedup();
    Ok(routes)
}

#[derive(Debug, Clone, serde::Serialize)]
struct TierRegionWorkloadRow {
    tier: String,
    graph_kind: String,
    split_objective: String,
    requested_regions: usize,
    region_id: usize,
    route: String,
    node_class: String,
    route_weight: i32,
    route_miles: f64,
    t1_node_count: usize,
    parent_trunk_count: usize,
    parent_trunks: String,
    contact_route_count: usize,
    component_id: usize,
    component_route_count: usize,
    component_status: String,
    repair_action: String,
    repair_basis: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct TierRegionRepairRow {
    tier: String,
    route: String,
    node_class: String,
    route_miles: f64,
    t1_node_count: usize,
    parent_trunks: String,
    contact_route_count: usize,
    component_id: usize,
    component_route_count: usize,
    component_status: String,
    repair_action: String,
    repair_basis: String,
    next_artifact: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct TierRegionRepairInputRow {
    tier: String,
    route: String,
    node_class: String,
    route_miles: f64,
    t1_node_count: usize,
    parent_trunks: String,
    contact_route_count: usize,
    component_id: usize,
    component_route_count: usize,
    component_status: String,
    repair_action: String,
    repair_basis: String,
    next_artifact: String,
}

#[derive(Debug, Clone, serde::Serialize)]
struct TierContactWitnessRow {
    tier: String,
    route: String,
    witness_type: String,
    node_class: String,
    route_miles: f64,
    observed_t1_node_count: usize,
    observed_parent_trunks: String,
    observed_dual_contacts: usize,
    component_id: usize,
    component_route_count: usize,
    component_status: String,
    repair_action: String,
    repair_basis: String,
    evidence_status: String,
    required_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct TierContactWitnessInputRow {
    tier: String,
    route: String,
    witness_type: String,
    node_class: String,
    route_miles: f64,
    observed_t1_node_count: usize,
    observed_parent_trunks: String,
    observed_dual_contacts: usize,
    component_id: usize,
    component_route_count: usize,
    component_status: String,
    repair_action: String,
    repair_basis: String,
    evidence_status: String,
    required_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2ContactResolutionRow {
    route: String,
    witness_type: String,
    node_class: String,
    repair_action: String,
    required_artifact: String,
    exception_type: String,
    exception_evidence_level: String,
    resolution_action: String,
    resolution_basis: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2HeldContactActionRow {
    route: String,
    held_action_type: String,
    source_resolution_action: String,
    exception_type: String,
    required_evidence: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2GraphContactRepairRow {
    route: String,
    repair_class: String,
    source_exception_type: String,
    repair_action: String,
    required_evidence: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2ParentContactValidationRow {
    route: String,
    parent_trunks: String,
    observed_dual_contacts: usize,
    validation_action: String,
    required_evidence: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct AtriBottleneckRow {
    #[serde(rename = "RANK")]
    rank: usize,
    #[serde(rename = "LOCATION")]
    location: String,
    #[serde(rename = "ROUTE")]
    route: String,
    #[serde(rename = "STATE")]
    state: String,
    #[serde(rename = "ANNUAL_COST_M")]
    annual_cost_m: f64,
    #[serde(rename = "LAT")]
    lat: f64,
    #[serde(rename = "LON")]
    lon: f64,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2ReliefEvidenceRow {
    route: String,
    source_exception_type: String,
    bottleneck_match_count: usize,
    top_bottleneck_rank: usize,
    top_bottleneck_location: String,
    annual_cost_m: f64,
    relief_action: String,
    evidence_basis: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2TerminalContactValidationRow {
    route: String,
    held_action_type: String,
    endpoint_name: String,
    endpoint_role: String,
    exception_type: String,
    terminal_worthy: bool,
    observed_t1_node_count: usize,
    observed_dual_contacts: usize,
    terminal_action: String,
    required_evidence: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BlockerClosureRow {
    route: String,
    segment_bundle_id: String,
    bundle_status: String,
    bundle_action: String,
    source_surface: String,
    blocker_class: String,
    blocker_action: String,
    required_evidence: String,
    next_artifact: String,
    optimizer_effect: String,
    #[serde(default)]
    qualification_effects: String,
    closure_status: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2RouteFamilySplitRow {
    route: String,
    endpoint_name: String,
    endpoint_role: String,
    exception_type: String,
    source_artifact: String,
    family_action: String,
    disposition: String,
    required_evidence: String,
    next_artifact: String,
    #[serde(default)]
    qualification_effects: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2GraphContactValidationRow {
    route: String,
    observed_t1_node_count: usize,
    observed_dual_contacts: usize,
    observed_parent_trunks: String,
    contact_action: String,
    disposition: String,
    required_evidence: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2ContactClosureRow {
    route: String,
    blocker_class: String,
    observed_t1_node_count: usize,
    observed_dual_contacts: usize,
    observed_parent_trunks: String,
    contact_action: String,
    disposition: String,
    required_evidence: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2EndpointClosureRow {
    route: String,
    endpoint_name: String,
    endpoint_role: String,
    exception_type: String,
    evidence_level: String,
    terminal_worthy: bool,
    endpoint_action: String,
    disposition: String,
    required_evidence: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone)]
struct T2ClosureDisposition {
    route: String,
    disposition: String,
    action: String,
    basis: String,
    segment_bundle_id: String,
    bundle_status: String,
    bundle_action: String,
    qualification_effects: String,
    source_artifact: String,
    next_artifact: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierCandidateColumnRow {
    tier: String,
    route: String,
    candidate_type: String,
    graph_kind: String,
    split_objective: String,
    node_class: String,
    route_miles: f64,
    observed_t1_node_count: usize,
    observed_dual_contacts: usize,
    parent_trunks: String,
    component_id: usize,
    component_route_count: usize,
    component_status: String,
    witness_type: String,
    repair_action: String,
    repair_basis: String,
    segment_bundle_id: String,
    bundle_status: String,
    bundle_action: String,
    pavement_debt_cost_m: f64,
    pavement_debt_class: String,
    pavement_debt_basis: String,
    pavement_debt_artifact: String,
    hard_blocker_count: usize,
    claim_blocker_count: usize,
    constraint_debt_cost_m: f64,
    lifecycle_debt_cost_m: f64,
    constraint_penalty_score: f64,
    top_constraint_classes: String,
    #[serde(default)]
    qualification_effects: String,
    constraint_ledger_artifact: String,
    column_decision: String,
    evidence_status: String,
    required_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2RegionalizerRow {
    tier: String,
    region_id: String,
    component_id: usize,
    route: String,
    parent_trunks: String,
    route_miles: f64,
    column_decision: String,
    treatment_status: String,
    evidence_status: String,
    pavement_debt_cost_m: f64,
    pavement_debt_class: String,
    pavement_debt_basis: String,
    hard_blocker_count: usize,
    claim_blocker_count: usize,
    constraint_debt_cost_m: f64,
    lifecycle_debt_cost_m: f64,
    constraint_penalty_score: f64,
    top_constraint_classes: String,
    #[serde(default)]
    qualification_effects: String,
    constraint_ledger_artifact: String,
    regionalizer_action: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2ServiceSelectionRow {
    tier: String,
    region_id: String,
    route: String,
    parent_trunks: String,
    column_decision: String,
    treatment_status: String,
    beck_corridor: String,
    beck_service_class: String,
    beck_color_mode: String,
    beck_start_trunk: String,
    beck_end_trunk: String,
    duplicate_service_count: usize,
    duplicate_service_corridors: String,
    close_parallel_count: usize,
    close_parallel_corridors: String,
    unstopped_t1_contact_count: usize,
    unstopped_t1_contacts: String,
    pavement_debt_cost_m: f64,
    pavement_debt_class: String,
    pavement_debt_basis: String,
    hard_blocker_count: usize,
    claim_blocker_count: usize,
    constraint_debt_cost_m: f64,
    lifecycle_debt_cost_m: f64,
    constraint_penalty_score: f64,
    top_constraint_classes: String,
    #[serde(default)]
    qualification_effects: String,
    constraint_ledger_artifact: String,
    beck_service_action: String,
    qualification_basis: String,
    qualification_map_treatment: String,
    qualification_gate_policy: String,
    qualification_game_use: String,
    selection_action: String,
    selection_basis: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct GameT2ServiceOverlayRow {
    service_class: String,
    map_id: String,
    scenario_hook: String,
    incident_lever: String,
    upgrade_lever: String,
    restitch_lever: String,
    release_gate: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BundleOverlayRow {
    tier: String,
    region_id: String,
    route: String,
    segment_bundle_id: String,
    bundle_status: String,
    service_class: String,
    map_id: String,
    scenario_hook: String,
    incident_lever: String,
    upgrade_lever: String,
    restitch_lever: String,
    release_gate: String,
    qualification_map_treatment: String,
    qualification_gate_policy: String,
    qualification_game_use: String,
    #[serde(default)]
    qualification_effects: String,
    pavement_debt_cost_m: f64,
    pavement_debt_class: String,
    pavement_debt_basis: String,
    source_artifacts: String,
    binding_status: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct T2ScenarioHookRow {
    scenario_id: String,
    service_class: String,
    t2_map_id: String,
    player_decision: String,
    evidence_hold: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2ServiceDiagnosticQueueRow {
    route: String,
    region_id: String,
    segment_bundle_id: String,
    bundle_status: String,
    selection_action: String,
    selection_basis: String,
    #[serde(default)]
    qualification_effects: String,
    diagnostic_status: String,
    service_diagnostic_action: String,
    required_artifact: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2ParallelServiceQueueRow {
    route: String,
    region_id: String,
    beck_corridor: String,
    service_class: String,
    close_parallel_count: usize,
    close_parallel_corridors: String,
    selection_action: String,
    selection_basis: String,
    parallel_action: String,
    required_artifact: String,
    next_artifact: String,
    optimizer_effect: String,
    #[serde(default)]
    qualification_effects: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct OptimizerConstraintLedgerRow {
    constraint_id: String,
    optimizer_run_id: String,
    tier: String,
    region_id: String,
    constraint_order: u8,
    constraint_class: String,
    behavior_type: String,
    constraint_scope: String,
    subject_id: String,
    segment_bundle_id: String,
    national_segment_id: String,
    stitch_group_id: String,
    route: String,
    stop_id: String,
    pair_id: String,
    map_id: String,
    source_artifact: String,
    source_row_id: String,
    standard_artifact: String,
    evidence_status: String,
    constraint_status: String,
    observed_value: String,
    threshold_value: String,
    measurement_unit: String,
    blocks_claims: String,
    budget_cost_m: f64,
    cost_category: String,
    cost_basis: String,
    cost_confidence: String,
    budget_units: String,
    penalty_score: f64,
    repair_action: String,
    payment_action: String,
    owner_jurisdiction: String,
    funding_program: String,
    delivery_risk: String,
    exception_id: String,
    exception_artifact: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct OptimizerConstraintBudgetRow {
    budget_id: String,
    optimizer_run_id: String,
    tier: String,
    region_id: String,
    subject_scope: String,
    subject_id: String,
    segment_bundle_id: String,
    route: String,
    ledger_row_count: usize,
    hard_blocker_count: usize,
    claim_blocker_count: usize,
    review_count: usize,
    budget_debt_count: usize,
    constraint_debt_cost_m: f64,
    lifecycle_debt_cost_m: f64,
    constraint_penalty_score: f64,
    top_constraint_classes: String,
    blocking_claims: String,
    qualification_effects: String,
    next_artifacts: String,
    constraint_ledger_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct OptimizerResidualBlockerBacklogRow {
    backlog_id: String,
    priority_class: String,
    blocker_family: String,
    tier: String,
    blocked_claims: String,
    subject_count: usize,
    route_count: usize,
    total_hard_blockers: usize,
    total_claim_blockers: usize,
    total_budget_debt_count: usize,
    total_constraint_debt_cost_m: f64,
    total_constraint_penalty_score: f64,
    representative_routes: String,
    representative_subjects: String,
    next_artifacts: String,
    backlog_decision: String,
    next_wave: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct OptimizerClaimReviewRow {
    claim_review_id: String,
    backlog_id: String,
    priority_class: String,
    blocker_family: String,
    tier: String,
    blocked_claims: String,
    subject_count: usize,
    route_count: usize,
    total_claim_blockers: usize,
    representative_routes: String,
    representative_subjects: String,
    evidence_artifacts: String,
    review_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2GamePublicationEvidenceReviewRow {
    game_review_id: String,
    claim_review_id: String,
    scenario_id: String,
    service_class: String,
    t2_map_id: String,
    player_decision: String,
    evidence_hold: String,
    review_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    required_evidence: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2GamePublicationEvidencePolicyRow {
    policy_id: String,
    game_review_id: String,
    scenario_id: String,
    service_class: String,
    t2_map_id: String,
    evidence_policy_basis: String,
    required_evidence: String,
    evidence_policy_decision: String,
    policy_treatment: String,
    publication_treatment: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2GamePublicationEvidencePolicyAcceptanceRow {
    acceptance_id: String,
    policy_id: String,
    scenario_id: String,
    service_class: String,
    t2_map_id: String,
    accepted_required_evidence: String,
    accepted_policy_treatment: String,
    acceptance_decision: String,
    publication_treatment: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2GamePublicationEvidenceBlockerReliefRow {
    relief_id: String,
    acceptance_id: String,
    policy_id: String,
    scenario_id: String,
    service_class: String,
    accepted_required_evidence: String,
    relief_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    ledger_replay_status: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T1SchematicGeometryClaimReviewRow {
    schematic_review_id: String,
    claim_review_id: String,
    route: String,
    design_role: String,
    design_status: String,
    beck_review_flag: String,
    overlap_corridors: String,
    policy_action: String,
    required_policy: String,
    design_treatment: String,
    gate_policy: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    review_decision: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BeckTransferComplexityReviewRow {
    transfer_review_id: String,
    claim_review_id: String,
    route: String,
    trunk: String,
    start_trunk: String,
    end_trunk: String,
    service_class: String,
    service_label: String,
    stop_count: usize,
    transfer_stop_count: usize,
    unique_duplicate_stop_count: usize,
    label_density_per_100px: f64,
    review_flag: String,
    complexity_basis: String,
    review_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BeckLabelDensityReviewRow {
    label_review_id: String,
    claim_review_id: String,
    route: String,
    trunk: String,
    start_trunk: String,
    end_trunk: String,
    service_class: String,
    service_label: String,
    stop_count: usize,
    transfer_stop_count: usize,
    label_density_per_100px: f64,
    review_flag: String,
    density_basis: String,
    review_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BeckLongConnectorReviewRow {
    connector_review_id: String,
    claim_review_id: String,
    route: String,
    trunk: String,
    start_trunk: String,
    end_trunk: String,
    service_class: String,
    service_label: String,
    stop_count: usize,
    transfer_stop_count: usize,
    schematic_length_px: f64,
    split_anchor: String,
    split_anchor_offset_pct: f64,
    review_flag: String,
    connector_basis: String,
    review_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BeckLongConnectorPolicyRow {
    policy_id: String,
    connector_review_id: String,
    route: String,
    trunk_pair: String,
    service_class: String,
    schematic_length_px: f64,
    connector_band: String,
    policy_basis: String,
    connector_policy_decision: String,
    render_treatment: String,
    promotion_treatment: String,
    publication_status: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BeckLongConnectorPolicyAcceptanceRow {
    acceptance_id: String,
    policy_id: String,
    route: String,
    connector_band: String,
    accepted_render_treatment: String,
    accepted_promotion_treatment: String,
    acceptance_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BeckLongConnectorBlockerReliefRow {
    relief_id: String,
    acceptance_id: String,
    policy_id: String,
    route: String,
    connector_band: String,
    accepted_render_treatment: String,
    relief_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    ledger_replay_status: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BeckLabelDensityPolicyRow {
    policy_id: String,
    label_review_id: String,
    route: String,
    trunk_pair: String,
    service_class: String,
    label_density_per_100px: f64,
    density_band: String,
    policy_basis: String,
    label_policy_decision: String,
    render_treatment: String,
    promotion_treatment: String,
    publication_status: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BeckLabelDensityPolicyAcceptanceRow {
    acceptance_id: String,
    policy_id: String,
    route: String,
    density_band: String,
    accepted_render_treatment: String,
    accepted_promotion_treatment: String,
    acceptance_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BeckLabelDensityBlockerReliefRow {
    relief_id: String,
    acceptance_id: String,
    policy_id: String,
    route: String,
    density_band: String,
    accepted_render_treatment: String,
    relief_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    ledger_replay_status: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BeckTransferComplexityPolicyRow {
    policy_id: String,
    transfer_review_id: String,
    route: String,
    trunk_pair: String,
    service_class: String,
    transfer_stop_count: usize,
    stop_count: usize,
    complexity_band: String,
    policy_basis: String,
    transfer_policy_decision: String,
    render_treatment: String,
    promotion_treatment: String,
    publication_status: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BeckTransferComplexityPolicyAcceptanceRow {
    acceptance_id: String,
    policy_id: String,
    route: String,
    complexity_band: String,
    accepted_render_treatment: String,
    accepted_promotion_treatment: String,
    acceptance_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BeckTransferComplexityBlockerReliefRow {
    relief_id: String,
    acceptance_id: String,
    policy_id: String,
    route: String,
    complexity_band: String,
    accepted_render_treatment: String,
    relief_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    ledger_replay_status: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T3LowerTierFeederGapReviewRow {
    feeder_review_id: String,
    backlog_id: String,
    gap_id: String,
    route: String,
    zone_id: String,
    current_score: f64,
    constraint_adjusted_score: f64,
    promise_horizon_hours: u8,
    gap_class: String,
    gap_reason: String,
    required_evidence: String,
    repair_action: String,
    review_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T3LowerTierFeederGapPolicyRow {
    policy_id: String,
    feeder_review_id: String,
    gap_id: String,
    route: String,
    zone_id: String,
    score_band: String,
    policy_basis: String,
    feeder_policy_decision: String,
    map_treatment: String,
    evidence_treatment: String,
    upgrade_treatment: String,
    publication_status: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T3LowerTierFeederGapPolicyAcceptanceRow {
    acceptance_id: String,
    policy_id: String,
    route: String,
    zone_id: String,
    score_band: String,
    accepted_map_treatment: String,
    accepted_evidence_treatment: String,
    accepted_upgrade_treatment: String,
    acceptance_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T3LowerTierFeederGapBlockerReliefRow {
    relief_id: String,
    acceptance_id: String,
    policy_id: String,
    route: String,
    zone_id: String,
    score_band: String,
    accepted_map_treatment: String,
    relief_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    ledger_replay_status: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T1SharedSegmentMapPolicyRow {
    policy_id: String,
    route_pair: String,
    primary_route: String,
    overlap_route: String,
    affected_routes: String,
    source_review_ids: String,
    policy_basis: String,
    map_policy_decision: String,
    render_treatment: String,
    selector_treatment: String,
    publication_status: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T1SharedSegmentPolicyAcceptanceRow {
    acceptance_id: String,
    policy_id: String,
    route_pair: String,
    affected_routes: String,
    map_policy_decision: String,
    accepted_render_treatment: String,
    acceptance_status: String,
    acceptance_basis: String,
    publication_status_before: String,
    publication_status_after: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T1SchematicGeometryBlockerReliefRow {
    relief_id: String,
    acceptance_id: String,
    policy_id: String,
    route_pair: String,
    affected_routes: String,
    accepted_render_treatment: String,
    relief_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    ledger_replay_status: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2GameOpsBindingIntakeRow {
    intake_id: String,
    budget_id: String,
    subject_id: String,
    segment_bundle_id: String,
    route: String,
    claim_blocker_count: usize,
    blocked_claims: String,
    top_constraint_classes: String,
    #[serde(default)]
    qualification_effects: String,
    next_artifacts: String,
    constraint_ledger_artifact: String,
    intake_status: String,
    decision_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2GameOpsBindingDecisionRow {
    decision_id: String,
    intake_id: String,
    subject_id: String,
    segment_bundle_id: String,
    route: String,
    service_class: String,
    bundle_status: String,
    binding_status: String,
    #[serde(default)]
    qualification_effects: String,
    qualification_gate_policy: String,
    qualification_game_use: String,
    decision: String,
    decision_reason: String,
    blocks_claims: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BundleOverlayRepairTargetRow {
    target_id: String,
    decision_id: String,
    subject_id: String,
    segment_bundle_id: String,
    route: String,
    decision: String,
    binding_status: String,
    bundle_status: String,
    service_class: String,
    #[serde(default)]
    qualification_effects: String,
    qualification_gate_policy: String,
    qualification_game_use: String,
    pavement_debt_cost_m: f64,
    pavement_debt_class: String,
    blocks_claims: String,
    repair_class: String,
    repair_action: String,
    required_artifact: String,
    next_artifact: String,
    target_status: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2ServiceClassRepairDocketRow {
    docket_id: String,
    target_id: String,
    route: String,
    segment_bundle_id: String,
    bundle_status: String,
    service_class: String,
    service_repair_class: String,
    service_action: String,
    #[serde(default)]
    qualification_effects: String,
    required_artifact: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2GameOpsBundleEvidenceReviewRow {
    review_id: String,
    decision_id: String,
    target_id: String,
    route: String,
    segment_bundle_id: String,
    decision: String,
    binding_status: String,
    bundle_status: String,
    service_class: String,
    repair_class: String,
    repair_action: String,
    #[serde(default)]
    qualification_effects: String,
    qualification_gate_policy: String,
    qualification_game_use: String,
    evidence_artifact: String,
    service_repair_class: String,
    evidence_status: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2GameOpsBundleEvidencePolicyRow {
    policy_id: String,
    review_id: String,
    decision_id: String,
    target_id: String,
    route: String,
    segment_bundle_id: String,
    repair_class: String,
    service_repair_class: String,
    evidence_artifact: String,
    #[serde(default)]
    qualification_effects: String,
    qualification_gate_policy: String,
    qualification_game_use: String,
    required_evidence: String,
    evidence_policy_decision: String,
    policy_treatment: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2GameOpsBundleEvidencePolicyAcceptanceRow {
    acceptance_id: String,
    policy_id: String,
    review_id: String,
    decision_id: String,
    target_id: String,
    route: String,
    segment_bundle_id: String,
    accepted_required_evidence: String,
    accepted_policy_treatment: String,
    #[serde(default)]
    qualification_effects: String,
    qualification_gate_policy: String,
    qualification_game_use: String,
    acceptance_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2GameOpsBundleEvidenceBlockerReliefRow {
    relief_id: String,
    acceptance_id: String,
    policy_id: String,
    route: String,
    segment_bundle_id: String,
    accepted_required_evidence: String,
    #[serde(default)]
    qualification_effects: String,
    qualification_gate_policy: String,
    qualification_game_use: String,
    relief_decision: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    blocker_count_before: usize,
    blocker_count_after: usize,
    claim_blocker_delta: isize,
    ledger_replay_status: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2ServiceOverlayDiagnosticDecisionRow {
    decision_id: String,
    docket_id: String,
    target_id: String,
    route: String,
    segment_bundle_id: String,
    bundle_status: String,
    current_service_class: String,
    diagnostic_status: String,
    diagnostic_action: String,
    overlay_decision: String,
    decision_reason: String,
    #[serde(default)]
    qualification_effects: String,
    blocks_claims: String,
    required_artifact: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2LocalZoneOverlayHandoffRow {
    handoff_id: String,
    docket_id: String,
    target_id: String,
    route: String,
    segment_bundle_id: String,
    zone_id: String,
    zone_name: String,
    zone_role: String,
    column_decision: String,
    map_treatment: String,
    handoff_decision: String,
    handoff_reason: String,
    #[serde(default)]
    qualification_effects: String,
    blocks_claims: String,
    required_artifact: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BundleReadinessDispositionRow {
    disposition_id: String,
    target_id: String,
    route: String,
    segment_bundle_id: String,
    bundle_status: String,
    service_class: String,
    readiness_class: String,
    disposition: String,
    disposition_action: String,
    #[serde(default)]
    qualification_effects: String,
    required_artifact: String,
    next_artifact: String,
    blocks_claims: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BundleReadinessRepairDocketRow {
    repair_id: String,
    disposition_id: String,
    target_id: String,
    route: String,
    segment_bundle_id: String,
    readiness_class: String,
    repair_decision: String,
    repair_action: String,
    #[serde(default)]
    qualification_effects: String,
    required_artifact: String,
    next_artifact: String,
    blocks_claims: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BundleReadinessRepairEvidenceRow {
    evidence_id: String,
    repair_id: String,
    route: String,
    segment_bundle_id: String,
    readiness_class: String,
    evidence_artifact: String,
    evidence_status: String,
    evidence_row_count: usize,
    evidence_summary: String,
    evidence_decision: String,
    #[serde(default)]
    qualification_effects: String,
    next_artifact: String,
    blocks_claims: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BundleReadinessReplayDecisionRow {
    replay_id: String,
    evidence_id: String,
    delta_id: String,
    route: String,
    segment_bundle_id: String,
    readiness_class: String,
    evidence_status: String,
    delta_replay_decision: String,
    replay_decision: String,
    replay_action: String,
    #[serde(default)]
    qualification_effects: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2NationalBundleReadinessAuditRow {
    audit_id: String,
    replay_id: String,
    route: String,
    segment_bundle_id: String,
    readiness_class: String,
    replay_decision: String,
    bundle_status: String,
    bundle_validation_status: String,
    bundle_member_count: usize,
    audit_decision: String,
    audit_action: String,
    #[serde(default)]
    qualification_effects: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2StitchedMemberRegistryHandoffRow {
    handoff_id: String,
    audit_id: String,
    route: String,
    segment_bundle_id: String,
    stitch_group_id: String,
    current_registry_member_count: usize,
    candidate_bundle_member_count: usize,
    candidate_route_member_count: usize,
    required_member_min: usize,
    handoff_decision: String,
    handoff_action: String,
    #[serde(default)]
    qualification_effects: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2StitchedMemberCandidateScopeReviewRow {
    scope_review_id: String,
    handoff_id: String,
    route: String,
    segment_bundle_id: String,
    blocked_bundle_candidate_count: usize,
    route_candidate_count: usize,
    route_candidate_bundle_count: usize,
    route_candidate_state_scope: String,
    route_candidate_bundle_ids: String,
    scope_decision: String,
    scope_action: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2StitchedMemberDecisionDocketRow {
    decision_docket_id: String,
    scope_review_id: String,
    route: String,
    segment_bundle_id: String,
    candidate_bundle_count: usize,
    candidate_state_scope: String,
    decision: String,
    decision_action: String,
    repair_instruction: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2StitchedMemberSplitPlanRow {
    split_plan_id: String,
    decision_docket_id: String,
    route: String,
    blocked_segment_bundle_id: String,
    candidate_segment_bundle_id: String,
    candidate_stitch_group_id: String,
    state_scope: String,
    candidate_member_count: usize,
    candidate_length_miles: f64,
    split_action: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2StitchedMemberSelectionDocketRow {
    selection_docket_id: String,
    split_plan_id: String,
    route: String,
    blocked_segment_bundle_id: String,
    candidate_segment_bundle_id: String,
    state_scope: String,
    candidate_member_count: usize,
    candidate_length_miles: f64,
    selection_decision: String,
    selection_action: String,
    evidence_requirement: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2StitchedMemberEvidenceContractRow {
    evidence_contract_id: String,
    selection_docket_id: String,
    route: String,
    candidate_segment_bundle_id: String,
    state_scope: String,
    required_continuity_proof: String,
    required_scope_proof: String,
    required_source_proof: String,
    evidence_status: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2StitchedMemberEvidenceAcquisitionRow {
    acquisition_docket_id: String,
    evidence_contract_id: String,
    route: String,
    candidate_segment_bundle_id: String,
    state_scope: String,
    source_owner: String,
    source_target: String,
    acquisition_action: String,
    acquisition_status: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2StitchedMemberSourceAccessPolicyRow {
    access_policy_id: String,
    acquisition_docket_id: String,
    route: String,
    candidate_segment_bundle_id: String,
    state_scope: String,
    source_owner: String,
    access_mode: String,
    live_fetch_status: String,
    required_source_metadata: String,
    cache_policy_artifact: String,
    source_access_blocker: String,
    evidence_artifact: String,
    acquisition_status: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2StitchedMemberProofIntakeRow {
    proof_intake_id: String,
    access_policy_id: String,
    route: String,
    candidate_segment_bundle_id: String,
    state_scope: String,
    required_artifact_fields: String,
    required_geometry_statement: String,
    proof_artifact: String,
    proof_status: String,
    proof_blocker: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2StitchedMemberProofSourceCaptureRow {
    source_capture_id: String,
    proof_intake_id: String,
    route: String,
    candidate_segment_bundle_id: String,
    state_scope: String,
    source_artifact_reference: String,
    source_artifact_type: String,
    capture_status: String,
    evidence_acceptance_status: String,
    capture_blocker: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2StitchedMemberProofArtifactAttachmentRow {
    artifact_attachment_id: String,
    source_capture_id: String,
    route: String,
    candidate_segment_bundle_id: String,
    state_scope: String,
    source_artifact_reference: String,
    attachment_status: String,
    evidence_review_status: String,
    proof_acceptance_status: String,
    attachment_blocker: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2StitchedMemberProofReviewDocketRow {
    proof_review_id: String,
    artifact_attachment_id: String,
    route: String,
    candidate_segment_bundle_id: String,
    state_scope: String,
    source_artifact_reference: String,
    review_decision: String,
    proof_acceptance_status: String,
    candidate_disposition_status: String,
    optimization_return_status: String,
    review_reason: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BundleOverlayRepairDeltaRow {
    delta_id: String,
    decision_id: String,
    route: String,
    segment_bundle_id: String,
    previous_decision: String,
    target_status: String,
    service_action: String,
    readiness_disposition: String,
    replay_decision: String,
    #[serde(default)]
    qualification_effects: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2OverlayOptimizerActionDocketRow {
    action_id: String,
    delta_id: String,
    route: String,
    segment_bundle_id: String,
    replay_decision: String,
    service_action: String,
    readiness_disposition: String,
    optimizer_action: String,
    priority_class: String,
    action_status: String,
    #[serde(default)]
    qualification_effects: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2OverlayP1StructuralReadinessReviewRow {
    p1_review_id: String,
    action_id: String,
    route: String,
    segment_bundle_id: String,
    optimizer_action: String,
    priority_class: String,
    readiness_decision: String,
    readiness_reason: String,
    downstream_action: String,
    action_status: String,
    #[serde(default)]
    qualification_effects: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2OverlayP2ServiceOverlayReviewRow {
    p2_review_id: String,
    action_id: String,
    route: String,
    segment_bundle_id: String,
    optimizer_action: String,
    priority_class: String,
    service_overlay_decision: String,
    service_overlay_reason: String,
    downstream_action: String,
    action_status: String,
    #[serde(default)]
    qualification_effects: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2OverlayP3LocalZoneOverlayReviewRow {
    p3_review_id: String,
    action_id: String,
    route: String,
    segment_bundle_id: String,
    optimizer_action: String,
    priority_class: String,
    local_zone_decision: String,
    local_zone_reason: String,
    downstream_action: String,
    action_status: String,
    #[serde(default)]
    qualification_effects: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BundleRepairQueueRow {
    route: String,
    segment_bundle_id: String,
    bundle_status: String,
    bundle_action: String,
    contact_evidence_status: String,
    candidate_decision: String,
    repair_class: String,
    repair_action: String,
    required_artifact: String,
    next_artifact: String,
    optimizer_effect: String,
    #[serde(default)]
    qualification_effects: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierSegmentCandidateRow {
    tier: String,
    source_selector: String,
    region_id: String,
    route: String,
    edge_id: u64,
    edge_sequence: usize,
    national_segment_id: String,
    segment_bundle_id: String,
    stitch_group_id: String,
    member_role: String,
    state: String,
    length_miles: f64,
    aadt: String,
    lane_count: String,
    route_aliases: String,
    selector_basis: String,
    candidate_action: String,
    #[serde(default)]
    qualification_effects: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementDocketRow {
    tier: String,
    source_selector: String,
    region_id: String,
    route: String,
    segment_bundle_id: String,
    stitch_group_id: String,
    national_segment_id: String,
    edge_id: u64,
    edge_sequence: usize,
    state: String,
    length_miles: f64,
    iri_m_per_km: String,
    max_iri_m_per_km: String,
    pavement_status: String,
    repair_action: String,
    freight_ride_requirement: String,
    transit_ride_requirement: String,
    source_contract: String,
    #[serde(default)]
    qualification_effects: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementSourceGapRow {
    tier: String,
    route: String,
    region_id: String,
    segment_bundle_id: String,
    stitch_group_id: String,
    member_count: usize,
    blocker_count: usize,
    blocker_statuses: String,
    affected_states: String,
    affected_edge_ids: String,
    source_contract: String,
    source_action: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementDebtBudgetRow {
    tier: String,
    route: String,
    region_id: String,
    segment_bundle_id: String,
    stitch_group_id: String,
    debt_class: String,
    blocked_member_count: usize,
    affected_states: String,
    evidence_debt_units: usize,
    repair_debt_units: usize,
    estimated_evidence_cost_m: f64,
    estimated_repair_cost_m: f64,
    total_debt_cost_m: f64,
    budget_basis: String,
    optimizer_penalty: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementRouteStateExclusionRow {
    exclusion_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    source_title: String,
    source_url_or_cache_artifact: String,
    capture_date: String,
    excluded_member_count: usize,
    exclusion_basis: String,
    exclusion_status: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementRepairFundingAcceptanceRow {
    acceptance_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    source_title: String,
    source_url_or_cache_artifact: String,
    capture_date: String,
    committed_amount_m: f64,
    covered_repair_cost_m: f64,
    funding_basis: String,
    acceptance_status: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementAcquisitionPlanRow {
    state: String,
    tier: String,
    source_family: String,
    route_count: usize,
    affected_routes: String,
    bundle_count: usize,
    affected_bundles: String,
    blocked_member_count: usize,
    source_priority: String,
    acquisition_action: String,
    required_fields: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementAcquisitionDocketRow {
    task_id: String,
    state: String,
    source_priority: String,
    affected_routes: String,
    affected_bundles: String,
    blocked_member_count: usize,
    fetch_command: String,
    rebuild_command: String,
    verify_command: String,
    source_contract: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementSourceAccessRow {
    access_policy_id: String,
    task_id: String,
    state: String,
    source_priority: String,
    source_access_mode: String,
    mutation_mode: String,
    cache_targets: String,
    fetch_command: String,
    preflight_gate: String,
    postfetch_gate: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementSourceFetchAttemptRow {
    fetch_attempt_id: String,
    access_policy_id: String,
    task_id: String,
    state: String,
    source_priority: String,
    fetch_command: String,
    cache_target: String,
    cache_record_count: usize,
    fetch_result_status: String,
    evidence_acceptance_status: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementSourceFetchReviewRow {
    review_id: String,
    fetch_attempt_id: String,
    task_id: String,
    state: String,
    source_priority: String,
    cache_record_count: usize,
    fetch_result_status: String,
    pre_review_blocked_member_count: usize,
    postfetch_unresolved_member_count: usize,
    join_review_status: String,
    evidence_acceptance_status: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementUnmatchedJoinReviewRow {
    join_review_id: String,
    state: String,
    source_priority: String,
    cache_record_count: usize,
    source_gap_member_count: usize,
    source_needed_member_count: usize,
    repair_required_member_count: usize,
    source_needed_routes: String,
    repair_required_routes: String,
    hpms_records_for_source_needed_routes: usize,
    hpms_source_route_coverage: String,
    join_review_status: String,
    evidence_acceptance_status: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementHpmsScopeBroadeningRow {
    broadening_id: String,
    state: String,
    source_priority: String,
    source_needed_routes: String,
    source_needed_member_count: usize,
    current_hpms_records_for_source_needed_routes: usize,
    current_coverage_status: String,
    broadened_functional_systems: String,
    broadened_fetch_command: String,
    preflight_gate: String,
    postfetch_gate: String,
    evidence_acceptance_status: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementRepairDebtReviewRow {
    repair_review_id: String,
    state: String,
    source_priority: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    stitch_group_id: String,
    blocked_member_count: usize,
    repair_debt_units: usize,
    estimated_repair_cost_m: f64,
    repair_debt_status: String,
    repair_decision: String,
    evidence_acceptance_status: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementRepairDispositionRow {
    disposition_id: String,
    repair_review_id: String,
    state: String,
    source_priority: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    blocked_member_count: usize,
    estimated_repair_cost_m: f64,
    disposition: String,
    relief_eligibility: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementRepairFundingPackageRow {
    funding_package_id: String,
    disposition_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    blocked_member_count: usize,
    estimated_repair_cost_m: f64,
    funding_package_status: String,
    funding_commitment_status: String,
    relief_eligibility: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingCommitmentReviewRow {
    commitment_review_id: String,
    funding_package_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    estimated_repair_cost_m: f64,
    funding_commitment_status: String,
    accepted_commitment_artifact: String,
    relief_eligibility: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementDowngradeExclusionDecisionRow {
    downgrade_exclusion_decision_id: String,
    commitment_review_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    estimated_repair_cost_m: f64,
    downgrade_decision: String,
    exclusion_decision: String,
    service_status: String,
    relief_eligibility: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceContractRow {
    evidence_contract_id: String,
    downgrade_exclusion_decision_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    estimated_repair_cost_m: f64,
    required_evidence: String,
    minimum_commitment_amount_m: f64,
    accepted_evidence_status: String,
    relief_eligibility: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceSourceCaptureRow {
    source_capture_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    minimum_commitment_amount_m: f64,
    source_capture_status: String,
    captured_artifact: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceArtifactAttachmentRow {
    artifact_attachment_id: String,
    source_capture_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    minimum_commitment_amount_m: f64,
    attachment_status: String,
    attached_artifact: String,
    evidence_review_status: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    claim_blocker_delta: isize,
    attachment_blocker: String,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceReviewDocketRow {
    funding_evidence_review_id: String,
    artifact_attachment_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    minimum_commitment_amount_m: f64,
    attached_artifact: String,
    review_decision: String,
    evidence_review_status: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    review_reason: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceAcquisitionRow {
    funding_evidence_acquisition_id: String,
    funding_evidence_review_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    minimum_commitment_amount_m: f64,
    required_artifact_type: String,
    acquisition_status: String,
    candidate_source_owner: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    acquisition_reason: String,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceSourceAccessRow {
    source_access_id: String,
    funding_evidence_acquisition_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    source_owner: String,
    access_mode: String,
    live_fetch_status: String,
    required_source_metadata: String,
    cache_policy_artifact: String,
    source_access_blocker: String,
    evidence_artifact: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceIntakeRow {
    funding_evidence_intake_id: String,
    source_access_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    required_artifact_type: String,
    required_source_metadata: String,
    intake_status: String,
    evidence_artifact: String,
    evidence_review_status: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    intake_blocker: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceMetadataCaptureRow {
    metadata_capture_id: String,
    funding_evidence_intake_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    required_artifact_type: String,
    metadata_capture_status: String,
    captured_artifact: String,
    captured_source_title: String,
    captured_source_url: String,
    captured_commitment_amount_m: String,
    evidence_review_status: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceAcceptedArtifactAttachmentRow {
    accepted_artifact_attachment_id: String,
    metadata_capture_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    required_artifact_type: String,
    attachment_status: String,
    attached_artifact: String,
    captured_source_title: String,
    captured_source_url: String,
    captured_commitment_amount_m: String,
    evidence_review_status: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    claim_blocker_delta: isize,
    attachment_blocker: String,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceAcceptedAttachmentReviewRow {
    accepted_attachment_review_id: String,
    accepted_artifact_attachment_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    required_artifact_type: String,
    attached_artifact: String,
    review_decision: String,
    evidence_review_status: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    review_reason: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceAcceptedArtifactAcquisitionRow {
    accepted_artifact_acquisition_id: String,
    accepted_attachment_review_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    required_artifact_type: String,
    acquisition_status: String,
    cache_status: String,
    candidate_source_owner: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    acquisition_reason: String,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceAcceptedSourceAccessRow {
    accepted_source_access_id: String,
    accepted_artifact_acquisition_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    source_owner: String,
    access_mode: String,
    cache_status: String,
    live_fetch_status: String,
    required_source_metadata: String,
    cache_policy_artifact: String,
    source_access_blocker: String,
    evidence_artifact: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceAcceptedIntakeRow {
    accepted_intake_id: String,
    accepted_source_access_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    required_artifact_type: String,
    required_source_metadata: String,
    intake_status: String,
    cache_status: String,
    evidence_artifact: String,
    evidence_review_status: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    intake_blocker: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceAcceptedMetadataCaptureRow {
    accepted_metadata_capture_id: String,
    accepted_intake_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    required_artifact_type: String,
    metadata_capture_status: String,
    captured_artifact: String,
    captured_source_title: String,
    captured_source_url: String,
    captured_commitment_amount_m: String,
    evidence_review_status: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceAcceptedMetadataArtifactAttachmentRow {
    accepted_metadata_artifact_attachment_id: String,
    accepted_metadata_capture_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    required_artifact_type: String,
    attachment_status: String,
    attached_artifact: String,
    captured_source_title: String,
    captured_source_url: String,
    captured_commitment_amount_m: String,
    evidence_review_status: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    claim_blocker_delta: isize,
    attachment_blocker: String,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceAcceptedMetadataAttachmentReviewRow {
    accepted_metadata_attachment_review_id: String,
    accepted_metadata_artifact_attachment_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    required_artifact_type: String,
    attached_artifact: String,
    review_decision: String,
    evidence_review_status: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    review_reason: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceAcceptedMetadataArtifactAcquisitionRow {
    accepted_metadata_artifact_acquisition_id: String,
    accepted_metadata_attachment_review_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    required_artifact_type: String,
    acquisition_status: String,
    cache_status: String,
    candidate_source_owner: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    acquisition_reason: String,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceAcceptedMetadataSourceAccessRow {
    accepted_metadata_source_access_id: String,
    accepted_metadata_artifact_acquisition_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    source_owner: String,
    access_mode: String,
    cache_status: String,
    live_fetch_status: String,
    required_source_metadata: String,
    cache_policy_artifact: String,
    source_access_blocker: String,
    evidence_artifact: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceAcceptedMetadataIntakeRow {
    accepted_metadata_intake_id: String,
    accepted_metadata_source_access_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    required_artifact_type: String,
    required_source_metadata: String,
    intake_status: String,
    cache_status: String,
    evidence_artifact: String,
    evidence_review_status: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    intake_blocker: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceAcceptedMetadataSourceCaptureRow {
    accepted_metadata_source_capture_id: String,
    accepted_metadata_intake_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    required_artifact_type: String,
    required_source_metadata: String,
    source_capture_status: String,
    captured_artifact: String,
    captured_source_title: String,
    captured_source_url: String,
    captured_commitment_amount_m: String,
    evidence_review_status: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    blocked_claims: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierPavementFundingEvidenceAcceptedMetadataSourceCaptureArtifactAttachmentRow {
    accepted_metadata_source_capture_artifact_attachment_id: String,
    accepted_metadata_source_capture_id: String,
    evidence_contract_id: String,
    state: String,
    tier: String,
    route: String,
    segment_bundle_id: String,
    required_artifact_type: String,
    attachment_status: String,
    attached_artifact: String,
    captured_source_title: String,
    captured_source_url: String,
    captured_commitment_amount_m: String,
    evidence_review_status: String,
    accepted_evidence_status: String,
    relief_eligibility: String,
    blocked_claims_before: String,
    blocked_claims_after: String,
    claim_blocker_delta: isize,
    attachment_blocker: String,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct TierTableScoreRow {
    tier: String,
    route: String,
    score: f64,
    confidence: f64,
    confidence_label: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct LowerTierPressureWitnessRow {
    route: String,
    current_tier: String,
    current_score: f64,
    confidence: f64,
    confidence_label: String,
    pressure_type: String,
    witness_action: String,
    target_tier: String,
    selection_basis: String,
    source_artifact: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T3T4PressureIntakeRow {
    route: String,
    source_pressure_type: String,
    current_tier: String,
    current_score: f64,
    target_tier: String,
    intake_class: String,
    intake_action: String,
    selection_basis: String,
    source_artifact: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T3ZoneAccessObligationRow {
    zone_id: String,
    zone_name: String,
    obligation_class: String,
    access_target: String,
    promise_horizon_hours: u8,
    source_route_count: usize,
    candidate_routes: String,
    source_intake_classes: String,
    map_id: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T3ZoneRouteColumnRow {
    zone_id: String,
    zone_name: String,
    obligation_class: String,
    route: String,
    current_tier: String,
    current_score: f64,
    constraint_adjusted_score: f64,
    hard_blocker_count: usize,
    claim_blocker_count: usize,
    constraint_debt_cost_m: f64,
    lifecycle_debt_cost_m: f64,
    constraint_penalty_score: f64,
    top_constraint_classes: String,
    constraint_ledger_artifact: String,
    promise_horizon_hours: u8,
    column_decision: String,
    zone_role: String,
    contact_requirement: String,
    map_treatment: String,
    selection_basis: String,
    source_obligation: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalAccessColumnRow {
    route: String,
    zone_id: String,
    current_score: f64,
    constraint_adjusted_score: f64,
    hard_blocker_count: usize,
    claim_blocker_count: usize,
    constraint_debt_cost_m: f64,
    lifecycle_debt_cost_m: f64,
    constraint_penalty_score: f64,
    top_constraint_classes: String,
    constraint_ledger_artifact: String,
    access_class: String,
    terminal_obligation: String,
    promise_horizon_hours: u8,
    column_decision: String,
    evidence_required: String,
    map_treatment: String,
    selection_basis: String,
    source_artifact: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalContactEvidenceRow {
    queue_id: String,
    route: String,
    zone_id: String,
    terminal_district_seed: String,
    terminal_district_seed_source: String,
    contact_basis: String,
    contact_proof_source: String,
    evidence_status: String,
    selected_higher_tier_attachment: String,
    decision: String,
    next_artifact: String,
    source_column_artifact: String,
    source_column_decision: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalAccessEvidenceReviewRow {
    review_id: String,
    queue_id: String,
    route: String,
    zone_id: String,
    terminal_district_seed: String,
    terminal_district_seed_source: String,
    evidence_status_before: String,
    review_decision: String,
    review_reason: String,
    source_action: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalScenarioReadinessRow {
    docket_id: String,
    route: String,
    zone_id: String,
    terminal_district: String,
    contact_basis: String,
    contact_proof_source: String,
    selected_higher_tier_attachment: String,
    freight_access_rationale: String,
    scenario_decision: String,
    scenario_artifact: String,
    source_evidence_status: String,
    release_status: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalContactSourcePlanRow {
    plan_id: String,
    queue_id: String,
    route: String,
    zone_id: String,
    terminal_district: String,
    terminal_district_seed_source: String,
    contact_proof_source_family: String,
    contact_proof_source_artifact: String,
    required_proof_fields: String,
    acquisition_status: String,
    proof_blocker: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalAccessProofAcquisitionRow {
    acquisition_id: String,
    review_id: String,
    queue_id: String,
    route: String,
    zone_id: String,
    terminal_district_seed: String,
    required_proof: String,
    prohibited_seed_source: String,
    acquisition_status: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    proof_artifact_status: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalAccessProofArtifactRow {
    proof_artifact_id: String,
    acquisition_id: String,
    review_id: String,
    queue_id: String,
    route: String,
    zone_id: String,
    terminal_district_seed: String,
    required_proof: String,
    source_artifact_reference: String,
    attachment_status: String,
    evidence_review_status: String,
    proof_acceptance_status: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalAccessProofReviewRow {
    proof_review_id: String,
    proof_artifact_id: String,
    acquisition_id: String,
    review_id: String,
    queue_id: String,
    route: String,
    zone_id: String,
    source_artifact_reference: String,
    review_decision: String,
    proof_acceptance_status: String,
    optimization_return_status: String,
    review_reason: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalAccessSourceAccessRow {
    source_access_id: String,
    proof_review_id: String,
    proof_artifact_id: String,
    acquisition_id: String,
    queue_id: String,
    route: String,
    zone_id: String,
    source_owner: String,
    access_mode: String,
    live_fetch_status: String,
    required_source_metadata: String,
    cache_policy_artifact: String,
    source_access_blocker: String,
    evidence_artifact: String,
    proof_acceptance_status: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalAccessProofIntakeRow {
    proof_intake_id: String,
    source_access_id: String,
    proof_review_id: String,
    proof_artifact_id: String,
    acquisition_id: String,
    queue_id: String,
    route: String,
    zone_id: String,
    required_artifact_fields: String,
    required_contact_statement: String,
    proof_artifact: String,
    proof_status: String,
    proof_blocker: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalAccessProofSourceCaptureRow {
    source_capture_id: String,
    proof_intake_id: String,
    source_access_id: String,
    proof_artifact_id: String,
    acquisition_id: String,
    queue_id: String,
    route: String,
    zone_id: String,
    source_artifact_reference: String,
    source_artifact_type: String,
    capture_status: String,
    evidence_acceptance_status: String,
    capture_blocker: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalAccessProofArtifactAttachmentRow {
    artifact_attachment_id: String,
    source_capture_id: String,
    proof_intake_id: String,
    proof_artifact_id: String,
    acquisition_id: String,
    queue_id: String,
    route: String,
    zone_id: String,
    source_artifact_reference: String,
    attachment_status: String,
    evidence_review_status: String,
    proof_acceptance_status: String,
    attachment_blocker: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalAccessProofAttachmentReviewRow {
    attachment_review_id: String,
    artifact_attachment_id: String,
    source_capture_id: String,
    proof_intake_id: String,
    proof_artifact_id: String,
    acquisition_id: String,
    queue_id: String,
    route: String,
    zone_id: String,
    source_artifact_reference: String,
    review_decision: String,
    evidence_review_status: String,
    proof_acceptance_status: String,
    optimization_return_status: String,
    review_reason: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalAccessProofArtifactAcquisitionTargetRow {
    acquisition_target_id: String,
    attachment_review_id: String,
    artifact_attachment_id: String,
    queue_id: String,
    route: String,
    zone_id: String,
    candidate_source_owner: String,
    required_artifact_fields: String,
    prohibited_seed_source: String,
    acquisition_status: String,
    cache_status: String,
    source_artifact_reference: String,
    proof_acceptance_status: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalAccessProofArtifactSourceAccessRow {
    source_access_id: String,
    acquisition_target_id: String,
    attachment_review_id: String,
    artifact_attachment_id: String,
    queue_id: String,
    route: String,
    zone_id: String,
    source_owner: String,
    access_mode: String,
    cache_status: String,
    live_fetch_status: String,
    required_source_metadata: String,
    cache_policy_artifact: String,
    source_access_blocker: String,
    evidence_artifact: String,
    proof_acceptance_status: String,
    blocker_claims_before: String,
    blocker_claims_after: String,
    claim_blocker_delta: isize,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalContactSourceCatalogRow {
    catalog_id: String,
    terminal_district: String,
    route_task_count: usize,
    source_family: String,
    source_access_mode: String,
    required_proof_fields: String,
    acquisition_status: String,
    proof_blocker: String,
    cache_policy_artifact: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalContactProofArtifactContractRow {
    contract_id: String,
    source_family: String,
    accepted_proof_status: String,
    required_fields: String,
    allowed_artifact_modes: String,
    prohibited_sources: String,
    promotion_rule: String,
    source_needed_decision: String,
    blocked_decision: String,
    rejected_decision: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalContactProofSourceRegistryRow {
    registry_id: String,
    task_id: String,
    queue_id: String,
    route: String,
    terminal_district: String,
    source_family: String,
    source_artifact_mode: String,
    source_title: String,
    source_url_or_cache_artifact: String,
    capture_date: String,
    contact_statement_status: String,
    selected_higher_tier_attachment_status: String,
    registry_status: String,
    proof_source_artifact: String,
    registry_blocker: String,
    contract_artifact: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalContactAcceptedProofSourceRow {
    queue_id: String,
    route: String,
    terminal_district: String,
    source_artifact_mode: String,
    source_title: String,
    source_url_or_cache_artifact: String,
    capture_date: String,
    contact_statement: String,
    selected_higher_tier_attachment: String,
    proof_source_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalContactRejectedProofSourceRow {
    queue_id: String,
    route: String,
    terminal_district: String,
    source_title: String,
    source_url_or_cache_artifact: String,
    capture_date: String,
    listed_terminal_access_routes: String,
    rejection_basis: String,
    rejection_status: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalContactDistrictProofImportRow {
    import_id: String,
    registry_id: String,
    queue_id: String,
    route: String,
    terminal_district: String,
    source_artifact_mode: String,
    proof_source_artifact: String,
    contact_statement_status: String,
    selected_higher_tier_attachment_status: String,
    import_status: String,
    proof_decision: String,
    import_blocker: String,
    selection_rule: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalContactProofDocketRow {
    task_id: String,
    queue_id: String,
    route: String,
    zone_id: String,
    terminal_district: String,
    source_family: String,
    required_proof_field: String,
    selected_higher_tier_attachment_requirement: String,
    contact_proof_source_artifact: String,
    proof_status: String,
    proof_blocker: String,
    scenario_effect: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalColumbusProofIntakeRow {
    intake_id: String,
    task_id: String,
    queue_id: String,
    route: String,
    zone_id: String,
    terminal_district: String,
    source_family: String,
    required_proof_field: String,
    selected_higher_tier_attachment_requirement: String,
    contact_proof_source_artifact: String,
    proof_status: String,
    proof_blocker: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalColumbusSourceAccessRow {
    access_id: String,
    intake_id: String,
    queue_id: String,
    route: String,
    terminal_district: String,
    source_family: String,
    access_mode: String,
    live_fetch_status: String,
    required_source_metadata: String,
    contact_proof_source_artifact: String,
    acquisition_status: String,
    source_access_blocker: String,
    cache_policy_artifact: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalColumbusProofAttemptRow {
    attempt_id: String,
    access_id: String,
    intake_id: String,
    queue_id: String,
    route: String,
    terminal_district: String,
    source_family: String,
    source_artifact: String,
    capture_status: String,
    contact_statement_status: String,
    selected_higher_tier_attachment_status: String,
    proof_attempt_status: String,
    proof_decision: String,
    proof_blocker: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T3T4AccessGapRow {
    gap_id: String,
    source_surface: String,
    route: String,
    zone_id: String,
    current_score: f64,
    constraint_adjusted_score: f64,
    hard_blocker_count: usize,
    claim_blocker_count: usize,
    constraint_debt_cost_m: f64,
    lifecycle_debt_cost_m: f64,
    constraint_penalty_score: f64,
    top_constraint_classes: String,
    constraint_ledger_artifact: String,
    promise_horizon_hours: u8,
    gap_class: String,
    gap_reason: String,
    required_evidence: String,
    repair_action: String,
    next_artifact: String,
    upward_pressure_allowed: bool,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T4TerminalAccessMapExclusionRow {
    decision_id: String,
    decision_scope: String,
    source_artifact: String,
    affected_constraint_class: String,
    affected_gap_class: String,
    affected_tier: String,
    affected_claims_before: String,
    excluded_claims: String,
    preserved_claims_after: String,
    affected_route_count: usize,
    decision: String,
    decision_basis: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T3ZoneMapDiagnosticRow {
    zone_id: String,
    zone_name: String,
    map_id: String,
    map_path: String,
    selected_route_count: usize,
    selected_routes: String,
    review_connector_count: usize,
    review_connectors: String,
    access_gap_count: usize,
    below_threshold_feeder_count: usize,
    terminal_evidence_gap_count: usize,
    zone_assignment_gap_count: usize,
    map_readiness: String,
    diagnostic_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T3ZoneRenderBoardRow {
    zone_id: String,
    zone_name: String,
    map_id: String,
    map_path: String,
    board_layer: String,
    route: String,
    national_segment_id: String,
    stitch_group_id: String,
    segment_bundle_id: String,
    segment_aliases: String,
    route_status: String,
    map_treatment: String,
    selected_route_count: usize,
    access_gap_count: usize,
    source_artifact: String,
    render_action: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T3ZoneStopPlacementRow {
    zone_id: String,
    zone_name: String,
    route: String,
    national_segment_id: String,
    stitch_group_id: String,
    segment_bundle_id: String,
    segment_aliases: String,
    state_scope: String,
    stop_count: usize,
    transfer_grade_stop_count: usize,
    stop_chain: String,
    stop_classes: String,
    placement_status: String,
    placement_action: String,
    source_artifact: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct NationalSegmentRegistryRow {
    national_segment_id: String,
    segment_bundle_id: String,
    stitch_group_id: String,
    current_zone_id: String,
    current_tier: String,
    route_label: String,
    zone_id: String,
    route: String,
    state_scope: String,
    evidence_state_scope: String,
    geometry_state_scope: String,
    segment_aliases: String,
    bundle_aliases: String,
    board_layers: String,
    source_artifacts: String,
    stop_placement_status: String,
    bundle_role: String,
    member_segment_ids: String,
    registry_action: String,
    #[serde(default)]
    qualification_effects: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct NationalSegmentBundleRow {
    segment_bundle_id: String,
    bundle_role: String,
    member_segment_ids: String,
    member_count: usize,
    stitch_group_ids: String,
    current_tiers: String,
    current_zone_ids: String,
    route_labels: String,
    state_scope: String,
    evidence_state_scope: String,
    geometry_state_scope: String,
    bundle_aliases: String,
    source_artifacts: String,
    bundle_status: String,
    bundle_action: String,
    #[serde(default)]
    qualification_effects: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2BubbleUpReviewRow {
    route: String,
    source_intake_class: String,
    current_score: f64,
    review_action: String,
    required_evidence: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Serialize)]
struct T1FeedbackDocketRow {
    route: String,
    source_surface: String,
    source_action: String,
    current_score: f64,
    t1_feedback_class: String,
    t1_feedback_action: String,
    t1_sla_pair_count: usize,
    t1_sla_pairs: String,
    required_evidence: String,
    next_artifact: String,
    optimizer_effect: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct TierOptimizerRunRow {
    step: usize,
    optimizer_stage: String,
    command: String,
    artifact: String,
    row_count: usize,
    gate_status: String,
    blocker_count: usize,
    blocker_summary: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct SourceFetchPolicyRow {
    fetch_family: String,
    commands: String,
    cache_targets: String,
    mutation_mode: String,
    preservation_contract: String,
    implementation_guard: String,
    validation_floor: String,
    policy_doc: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T2AssetConditionMapPublicationExclusionRow {
    decision_id: String,
    decision_scope: String,
    source_artifact: String,
    affected_constraint_class: String,
    affected_tier: String,
    affected_claims_before: String,
    excluded_claims: String,
    preserved_claims_after: String,
    affected_bundle_count: u32,
    total_debt_cost_m: f64,
    decision: String,
    decision_basis: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct SourceSnapshotPublicationExclusionRow {
    decision_id: String,
    decision_scope: String,
    source_artifact: String,
    affected_constraint_class: String,
    affected_fetch_family: String,
    affected_claims_before: String,
    excluded_claims: String,
    preserved_claims_after: String,
    decision: String,
    decision_basis: String,
    next_artifact: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Serialize)]
struct OptimizerMapHookRow {
    hook_id: String,
    optimizer_artifact: String,
    consumer_artifact: String,
    consumer_type: String,
    gate_command: String,
    link_basis: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Serialize)]
struct BundleArchitectureRow {
    crate_name: String,
    role: String,
    bundle_entrypoint: String,
    source_path: String,
    required_tokens: String,
    architecture_status: String,
    next_action: String,
    validation_status: String,
}

// `tier_region_workload_rows` moved to support::tier

// `dual_route_adjacency` moved to support::misc

pub(crate) fn route_region_weight(route_miles: f64) -> i32 {
    route_miles.round().clamp(1.0, i32::MAX as f64) as i32
}

pub(crate) fn connected_components(adjacency: &[Vec<usize>]) -> (Vec<usize>, usize) {
    let mut component_ids = vec![usize::MAX; adjacency.len()];
    let mut component_count = 0usize;
    for start in 0..adjacency.len() {
        if component_ids[start] != usize::MAX {
            continue;
        }
        let mut queue = std::collections::VecDeque::from([start]);
        component_ids[start] = component_count;
        while let Some(node) = queue.pop_front() {
            for &neighbor in &adjacency[node] {
                if component_ids[neighbor] == usize::MAX {
                    component_ids[neighbor] = component_count;
                    queue.push_back(neighbor);
                }
            }
        }
        component_count += 1;
    }
    (component_ids, component_count)
}

pub(crate) fn component_sizes(component_ids: &[usize], component_count: usize) -> Vec<usize> {
    let mut sizes = vec![0usize; component_count];
    for &component in component_ids {
        sizes[component] += 1;
    }
    sizes
}

pub(crate) fn bridge_components(
    adjacency: &mut [Vec<usize>],
    component_ids: &[usize],
    component_count: usize,
) {
    let mut representatives = vec![None; component_count];
    for (node, &component) in component_ids.iter().enumerate() {
        representatives[component].get_or_insert(node);
    }
    for pair in representatives.windows(2) {
        if let [Some(a), Some(b)] = pair {
            push_unique_neighbor(&mut adjacency[*a], *b);
            push_unique_neighbor(&mut adjacency[*b], *a);
        }
    }
}

pub(crate) fn push_unique_neighbor(neighbors: &mut Vec<usize>, neighbor: usize) {
    if !neighbors.contains(&neighbor) {
        neighbors.push(neighbor);
        neighbors.sort_unstable();
    }
}

pub(crate) fn validate_region_assignment(assignment: &[usize], requested_regions: usize) -> Result<()> {
    let mut counts = vec![0usize; requested_regions];
    for &region in assignment {
        if region >= requested_regions {
            anyhow::bail!("METIS assigned route to out-of-range region {region}");
        }
        counts[region] += 1;
    }
    for (region, count) in counts.into_iter().enumerate() {
        if count == 0 {
            anyhow::bail!("METIS produced empty region {region}");
        }
    }
    Ok(())
}

pub(crate) fn tier_region_repair_action(
    node_class: &route_network::TierNodeClass,
    contact_route_count: usize,
    component_route_count: usize,
) -> (&'static str, &'static str) {
    match node_class {
        route_network::TierNodeClass::TrunkConnector if component_route_count >= 2 => {
            ("keep-for-regionalizer", "touches-multiple-t1-trunks")
        }
        route_network::TierNodeClass::TrunkConnector => (
            "add-dual-contact-witness",
            "qualified-route-is-alone-in-dual-component",
        ),
        route_network::TierNodeClass::ReliefLoop if contact_route_count > 0 => (
            "keep-with-parent-region-review",
            "relief-loop-shares-parent-service-context",
        ),
        route_network::TierNodeClass::ReliefLoop => (
            "add-parent-contact-or-demote",
            "relief-loop-has-no-dual-route-contact",
        ),
        route_network::TierNodeClass::OneEndedFeeder => (
            "terminal-exception-or-demote",
            "one-ended-feeder-needs-terminal-worthy-endpoint",
        ),
        route_network::TierNodeClass::LocalSpur => ("demote-to-t3-t4", "local-spur"),
        route_network::TierNodeClass::MissingGraphData => {
            ("fix-graph-contact-or-demote", "missing-t1-contact-evidence")
        }
    }
}

pub(crate) fn write_tier_region_workloads(path: &Path, rows: &[TierRegionWorkloadRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn tier_region_repair_rows(rows: &[TierRegionWorkloadRow]) -> Vec<TierRegionRepairRow> {
    rows.iter()
        .map(|row| TierRegionRepairRow {
            tier: row.tier.clone(),
            route: row.route.clone(),
            node_class: row.node_class.clone(),
            route_miles: row.route_miles,
            t1_node_count: row.t1_node_count,
            parent_trunks: row.parent_trunks.clone(),
            contact_route_count: row.contact_route_count,
            component_id: row.component_id,
            component_route_count: row.component_route_count,
            component_status: row.component_status.clone(),
            repair_action: row.repair_action.clone(),
            repair_basis: row.repair_basis.clone(),
            next_artifact: tier_region_next_artifact(&row.repair_action).to_string(),
        })
        .collect()
}

pub(crate) fn tier_region_next_artifact(repair_action: &str) -> &'static str {
    match repair_action {
        "keep-for-regionalizer" => "data/tier-candidate-columns.csv",
        "keep-with-parent-region-review" => "data/tier-candidate-columns.csv",
        "add-dual-contact-witness" => "data/tier-contact-witnesses.csv",
        "add-parent-contact-or-demote" => "data/tier-contact-witnesses.csv",
        "terminal-exception-or-demote" => "data/tier-node-exceptions.csv",
        "demote-to-t3-t4" => "data/tier-table.csv",
        "fix-graph-contact-or-demote" => "data/tier-contact-witnesses.csv",
        _ => "data/tier-region-repairs.csv",
    }
}

pub(crate) fn write_tier_region_repairs(path: &Path, rows: &[TierRegionRepairRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn load_tier_region_repairs(path: &Path) -> Result<Vec<TierRegionRepairInputRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `tier_contact_witness_rows` moved to support::tier

struct T2BeckContactWitness {
    witness_type: String,
    node_class: String,
    observed_t1_node_count: usize,
    observed_parent_trunks: String,
    observed_dual_contacts: usize,
    repair_action: String,
    repair_basis: String,
    evidence_status: String,
    required_artifact: String,
    validation_status: String,
}

pub(crate) fn t2_beck_contact_witness(
    row: &TierRegionRepairInputRow,
    diagnostic: &route_map::BeckT2DiagnosticRow,
) -> Option<T2BeckContactWitness> {
    if !row.tier.eq_ignore_ascii_case("T2")
        || row.repair_action != "fix-graph-contact-or-demote"
        || diagnostic.service_action != "keep"
        || beck_t2_diagnostics_gate_failure(diagnostic.review_flag)
        || diagnostic.unstopped_t1_contact_count > 0
        || diagnostic.duplicate_service_count > 0
        || diagnostic.close_parallel_count > 0
    {
        return None;
    }

    let mut trunks = semicolon_values(&format!(
        "{};{}",
        diagnostic.start_trunk, diagnostic.end_trunk
    ))
    .into_iter()
    .map(|trunk| canonical_route_key(&trunk))
    .filter(|trunk| !trunk.is_empty())
    .collect::<Vec<_>>();
    trunks.sort();
    trunks.dedup();
    if trunks.len() < 2 {
        return None;
    }

    Some(T2BeckContactWitness {
        witness_type: "regionalizer-ready".to_string(),
        node_class: "trunk_connector".to_string(),
        observed_t1_node_count: trunks.len(),
        observed_parent_trunks: trunks.join(";"),
        observed_dual_contacts: trunks.len(),
        repair_action: "keep-for-regionalizer".to_string(),
        repair_basis: "beck-diagnostic-t1-contact".to_string(),
        evidence_status: "beck-contact-observed".to_string(),
        required_artifact: "data/tier-candidate-columns.csv".to_string(),
        validation_status: "pass".to_string(),
    })
}

pub(crate) fn tier_contact_witness_status(repair_action: &str) -> (&'static str, &'static str, &'static str) {
    match repair_action {
        "keep-for-regionalizer" => ("regionalizer-ready", "accepted", "pass"),
        "keep-with-parent-region-review" => ("parent-region-review", "review", "review"),
        "add-dual-contact-witness" => ("dual-contact-needed", "source-needed", "review"),
        "add-parent-contact-or-demote" => ("parent-contact-needed", "source-needed", "review"),
        "terminal-exception-or-demote" => ("terminal-exception-needed", "source-needed", "review"),
        "demote-to-t3-t4" => ("tier-demotion-needed", "policy-action", "review"),
        "fix-graph-contact-or-demote" => ("graph-contact-needed", "source-needed", "review"),
        _ => ("unknown-repair-action", "source-needed", "review"),
    }
}

pub(crate) fn write_tier_contact_witnesses(path: &Path, rows: &[TierContactWitnessRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_contact_witness_summary(output: &Path, rows: &[TierContactWitnessRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.witness_type.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} witness rows to {}",
        rows.len(),
        output.display()
    );
    for (witness_type, count) in counts {
        println!("  {witness_type}: {count}");
    }
}

pub(crate) fn tier_contact_witness_gate_failures(rows: &[TierContactWitnessRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no contact witness rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if tier_contact_witness_is_unresolved_blocker(
            row.witness_type.as_str(),
            row.required_artifact.as_str(),
            row.validation_status.as_str(),
        ) {
            failures.push(format!(
                "{} requires {} via {}",
                row.route, row.witness_type, row.required_artifact
            ));
        }
    }
    failures
}

pub(crate) fn tier_contact_witness_is_unresolved_blocker(
    witness_type: &str,
    required_artifact: &str,
    validation_status: &str,
) -> bool {
    if validation_status.eq_ignore_ascii_case("pass") {
        return false;
    }
    matches!(
        witness_type,
        "dual-contact-needed"
            | "parent-contact-needed"
            | "graph-contact-needed"
            | "unknown-repair-action"
    ) && !matches!(
        required_artifact,
        "data/tier-candidate-columns.csv" | "data/tier-table.csv"
    )
}

pub(crate) fn load_tier_contact_witnesses(path: &Path) -> Result<Vec<TierContactWitnessInputRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_contact_resolution_rows(
    rows: &[TierContactWitnessInputRow],
    exceptions: &[EndpointExceptionRow],
) -> Vec<T2ContactResolutionRow> {
    rows.iter()
        .filter(|row| row.tier.eq_ignore_ascii_case("T2"))
        .map(|row| {
            let route_exceptions = endpoint_exceptions_for_route(exceptions, &row.route, "T2");
            let exception = route_exceptions.first().copied();
            let (resolution_action, resolution_basis, next_artifact, validation_status) =
                t2_contact_resolution_decision(row, &route_exceptions);
            T2ContactResolutionRow {
                route: row.route.clone(),
                witness_type: row.witness_type.clone(),
                node_class: row.node_class.clone(),
                repair_action: row.repair_action.clone(),
                required_artifact: row.required_artifact.clone(),
                exception_type: exception
                    .map(|exception| exception.exception_type.clone())
                    .unwrap_or_default(),
                exception_evidence_level: exception
                    .map(|exception| exception.evidence_level.clone())
                    .unwrap_or_default(),
                resolution_action: resolution_action.to_string(),
                resolution_basis: resolution_basis.to_string(),
                next_artifact: next_artifact.to_string(),
                validation_status: validation_status.to_string(),
            }
        })
        .collect()
}

// `t2_contact_resolution_decision` moved to support::tier

pub(crate) fn write_t2_contact_resolutions(path: &Path, rows: &[T2ContactResolutionRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn load_t2_contact_resolutions(path: &Path) -> Result<Vec<T2ContactResolutionRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn print_t2_contact_resolution_summary(output: &Path, rows: &[T2ContactResolutionRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.resolution_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} contact resolution rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

pub(crate) fn t2_contact_resolution_gate_failures(rows: &[T2ContactResolutionRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T2 contact resolution rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.resolution_action.trim().is_empty()
            || row.resolution_basis.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || !matches!(row.validation_status.as_str(), "pass" | "review")
        {
            failures.push(format!(
                "{} has incomplete T2 contact resolution contract",
                row.route
            ));
        }
    }
    failures
}

pub(crate) fn t2_held_contact_action_rows(rows: &[T2ContactResolutionRow]) -> Vec<T2HeldContactActionRow> {
    rows.iter()
        .filter(|row| row.validation_status == "review")
        .map(|row| {
            let (held_action_type, required_evidence, next_artifact, optimizer_effect) =
                t2_held_contact_action_contract(row);
            T2HeldContactActionRow {
                route: row.route.clone(),
                held_action_type: held_action_type.to_string(),
                source_resolution_action: row.resolution_action.clone(),
                exception_type: row.exception_type.clone(),
                required_evidence: required_evidence.to_string(),
                next_artifact: next_artifact.to_string(),
                optimizer_effect: optimizer_effect.to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect()
}

pub(crate) fn t2_held_contact_action_contract(
    row: &T2ContactResolutionRow,
) -> (&'static str, &'static str, &'static str, &'static str) {
    match row.resolution_action.as_str() {
        "hold-for-terminal-contact-validation" => (
            "terminal-contact-validation",
            "prove terminal endpoint plus at least one T1/T2 contact chain",
            "data/t2-terminal-contact-validation.csv",
            "retain as T2 only if terminal contact validates; otherwise demote",
        ),
        "hold-for-terminal-exception" => (
            "terminal-exception-review",
            "split route family or validate terminal-worthy endpoint exception",
            "data/t2-terminal-contact-validation.csv",
            "retain only validated terminal segment; demote unsplit local loop behavior",
        ),
        "hold-for-relief-evidence-or-demotion" => (
            "relief-evidence-review",
            "source-backed bottleneck or resilience relief evidence plus T1/T2 contact",
            "data/t2-relief-evidence-docket.csv",
            "retain as relief service only with evidence; otherwise demote",
        ),
        "hold-for-parent-contact-or-demotion" => (
            "parent-contact-validation",
            "prove relief loop dual-route contact to parent trunk",
            "data/t2-parent-contact-validation.csv",
            "retain with parent contact; otherwise demote",
        ),
        _ => (
            "graph-contact-repair",
            "repair route geometry or split route family before tier decision",
            "data/tier-contact-witnesses.csv",
            "blocked from T2 regionalizer until contact evidence exists",
        ),
    }
}

pub(crate) fn write_t2_held_contact_actions(path: &Path, rows: &[T2HeldContactActionRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn load_t2_held_contact_actions(path: &Path) -> Result<Vec<T2HeldContactActionRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn print_t2_held_contact_action_summary(output: &Path, rows: &[T2HeldContactActionRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.held_action_type.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} held contact action rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

pub(crate) fn t2_held_contact_action_gate_failures(rows: &[T2HeldContactActionRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no held T2 contact action rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.held_action_type.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!("{} has incomplete held contact action", row.route));
        }
    }
    failures
}

pub(crate) fn t2_graph_contact_repair_rows(rows: &[T2HeldContactActionRow]) -> Vec<T2GraphContactRepairRow> {
    let mut repairs = rows
        .iter()
        .filter(|row| row.held_action_type == "graph-contact-repair")
        .map(|row| {
            let (repair_class, repair_action, required_evidence, next_artifact, optimizer_effect) =
                t2_graph_contact_repair_contract(row);
            T2GraphContactRepairRow {
                route: row.route.clone(),
                repair_class: repair_class.to_string(),
                source_exception_type: row.exception_type.clone(),
                repair_action: repair_action.to_string(),
                required_evidence: required_evidence.to_string(),
                next_artifact: next_artifact.to_string(),
                optimizer_effect: optimizer_effect.to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    if repairs.is_empty() {
        repairs.push(T2GraphContactRepairRow {
            route: "__all_t2_graph_contact_repairs__".to_string(),
            repair_class: "graph-contact-repair-clear".to_string(),
            source_exception_type: String::new(),
            repair_action: "graph-contact-repair-clear".to_string(),
            required_evidence: "no graph-contact repair blockers remain".to_string(),
            next_artifact: "data/tier-candidate-columns.csv".to_string(),
            optimizer_effect: "graph-contact repair lane is clear".to_string(),
            validation_status: "pass".to_string(),
        });
    }
    repairs
}

pub(crate) fn t2_graph_contact_repair_contract(
    row: &T2HeldContactActionRow,
) -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
) {
    if row
        .exception_type
        .trim()
        .eq_ignore_ascii_case("missing_graph_geometry")
    {
        (
            "route-family-split",
            "split-numbered-route-family-before-tier-decision",
            "identify represented segment and its T1/T2 contacts",
            "data/tier-node-exceptions.csv",
            "blocked until route family is disambiguated",
        )
    } else {
        (
            "graph-contact-repair",
            "repair-route-geometry-or-demote",
            "prove at least one T1/T2 graph contact or demotion basis",
            "data/tier-contact-witnesses.csv",
            "blocked until graph contact evidence exists",
        )
    }
}

pub(crate) fn write_t2_graph_contact_repairs(path: &Path, rows: &[T2GraphContactRepairRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_graph_contact_repair_summary(output: &Path, rows: &[T2GraphContactRepairRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.repair_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} graph contact repair rows to {}",
        rows.len(),
        output.display()
    );
    for (repair_class, count) in counts {
        println!("  {repair_class}: {count}");
    }
}

pub(crate) fn t2_graph_contact_repair_gate_failures(rows: &[T2GraphContactRepairRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.len() == 1 && rows[0].route == "__all_t2_graph_contact_repairs__" {
        let row = &rows[0];
        if row.repair_action != "graph-contact-repair-clear" || row.validation_status != "pass" {
            failures
                .push("graph contact repair clearance row has incomplete clear status".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.repair_class.trim().is_empty()
            || row.repair_action.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!("{} has incomplete graph contact repair", row.route));
        }
    }
    failures
}

// `t2_parent_contact_validation_rows` moved to support::tier

pub(crate) fn write_t2_parent_contact_validation(
    path: &Path,
    rows: &[T2ParentContactValidationRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_parent_contact_validation_summary(
    output: &Path,
    rows: &[T2ParentContactValidationRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.validation_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} parent contact validation rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

pub(crate) fn t2_parent_contact_validation_gate_failures(
    rows: &[T2ParentContactValidationRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.len() == 1 && rows[0].route == "__all_t2_parent_contacts__" {
        let row = &rows[0];
        if row.validation_action != "parent-contact-clear" || row.validation_status != "pass" {
            failures.push("parent contact clearance row has incomplete clear status".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.validation_action.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
            || !matches!(row.validation_status.as_str(), "pass" | "review")
        {
            failures.push(format!(
                "{} has incomplete parent contact validation",
                row.route
            ));
        }
    }
    failures
}

pub(crate) fn load_atri_bottlenecks(path: &Path) -> Result<Vec<AtriBottleneckRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_relief_evidence_rows` moved to support::tier

pub(crate) fn write_t2_relief_evidence_docket(path: &Path, rows: &[T2ReliefEvidenceRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_relief_evidence_summary(output: &Path, rows: &[T2ReliefEvidenceRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.relief_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} relief evidence rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

pub(crate) fn t2_relief_evidence_gate_failures(rows: &[T2ReliefEvidenceRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.len() == 1 && rows[0].route == "__all_t2_relief_evidence__" {
        let row = &rows[0];
        if row.relief_action != "relief-evidence-clear" || row.validation_status != "pass" {
            failures.push("relief evidence clearance row has incomplete clear status".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.relief_action.trim().is_empty()
            || row.evidence_basis.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
            || !matches!(row.validation_status.as_str(), "pass" | "review")
        {
            failures.push(format!(
                "{} has incomplete relief evidence docket",
                row.route
            ));
        }
    }
    failures
}

// `t2_terminal_contact_validation_rows` moved to support::tier

pub(crate) fn write_t2_terminal_contact_validation(
    path: &Path,
    rows: &[T2TerminalContactValidationRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_terminal_contact_validation_summary(
    output: &Path,
    rows: &[T2TerminalContactValidationRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.terminal_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} terminal contact validation rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

pub(crate) fn t2_terminal_contact_validation_gate_failures(
    rows: &[T2TerminalContactValidationRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.len() == 1 && rows[0].route == "__all_t2_terminal_contacts__" {
        let row = &rows[0];
        if row.terminal_action != "terminal-contact-clear" || row.validation_status != "pass" {
            failures.push("terminal contact clearance row has incomplete clear status".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.held_action_type.trim().is_empty()
            || row.terminal_action.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
            || !matches!(row.validation_status.as_str(), "pass" | "review")
        {
            failures.push(format!(
                "{} has incomplete terminal contact validation",
                row.route
            ));
        }
    }
    failures
}

pub(crate) fn load_t2_graph_contact_repairs(path: &Path) -> Result<Vec<T2GraphContactRepairRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t2_parent_contact_validation(path: &Path) -> Result<Vec<T2ParentContactValidationRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t2_relief_evidence_docket(path: &Path) -> Result<Vec<T2ReliefEvidenceRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t2_terminal_contact_validation(path: &Path) -> Result<Vec<T2TerminalContactValidationRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_blocker_closure_rows` moved to `t2_blocker_closure_rows.rs`

pub(crate) fn bundle_qualification_effects_by_route(
    bundle_rows: &[NationalSegmentBundleRow],
) -> std::collections::BTreeMap<String, String> {
    let mut effects_by_route =
        std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();
    for row in bundle_rows {
        if row.qualification_effects.trim().is_empty() {
            continue;
        }
        for route in semicolon_values(&row.route_labels) {
            insert_pipe_values(
                effects_by_route
                    .entry(canonical_route_key(&route))
                    .or_default(),
                &row.qualification_effects,
            );
        }
    }
    effects_by_route
        .into_iter()
        .map(|(route, effects)| (route, join_pipe_set(&effects)))
        .collect()
}

pub(crate) fn t2_blocker_bundle_fields(
    registry: &route_network::BundleRegistry,
    route: &str,
) -> (String, String, String) {
    registry
        .by_route_label(route)
        .first()
        .map(|bundle| {
            let (bundle_action, _) =
                route_network::bundle_action(bundle.bundle_status, &bundle.registry_actions);
            (
                bundle.segment_bundle_id.clone(),
                bundle.bundle_status.as_str().to_string(),
                bundle_action.to_string(),
            )
        })
        .unwrap_or_else(|| {
            (
                String::new(),
                "bundle-missing".to_string(),
                "resolve route family or add segment bundle".to_string(),
            )
        })
}

pub(crate) fn write_t2_blocker_closure(path: &Path, rows: &[T2BlockerClosureRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_blocker_closure_summary(output: &Path, rows: &[T2BlockerClosureRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.blocker_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 blocker closure rows to {}",
        rows.len(),
        output.display()
    );
    for (blocker_class, count) in counts {
        println!("  {blocker_class}: {count}");
    }
}

pub(crate) fn t2_blocker_closure_gate_failures(rows: &[T2BlockerClosureRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T2 blocker closure rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.bundle_status.trim().is_empty()
            || row.bundle_action.trim().is_empty()
            || row.source_surface.trim().is_empty()
            || row.blocker_class.trim().is_empty()
            || row.blocker_action.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
            || row.closure_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete blocker closure", row.route));
        }
        if row.bundle_status == "bundle-missing"
            && !matches!(
                row.blocker_class.as_str(),
                "route-family-split"
                    | "relief-contact-repair"
                    | "parent-contact-repair"
                    | "endpoint-exception-upgrade"
            )
        {
            failures.push(format!(
                "{} blocker closure lacks bundle binding for {}",
                row.route, row.blocker_class
            ));
        }
        if !row.qualification_effects.trim().is_empty() && row.segment_bundle_id.trim().is_empty() {
            failures.push(format!(
                "{} carries qualification effects without a segment bundle",
                row.route
            ));
        }
    }
    failures
}

pub(crate) fn load_t2_blocker_closure(path: &Path) -> Result<Vec<T2BlockerClosureRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_route_family_split_rows` moved to `t2_route_family_split_rows.rs`

pub(crate) fn route_family_split_optimizer_effect(effect: &str, qualification_effects: &str) -> String {
    if qualification_effects.trim().is_empty() {
        return effect.to_string();
    }
    format!("{effect}; qualification_effects={qualification_effects}")
}

pub(crate) fn is_three_digit_interstate(route: &str) -> bool {
    canonical_route_key(route)
        .strip_prefix('I')
        .and_then(|number| number.parse::<u16>().ok())
        .map(|number| number >= 100)
        .unwrap_or_default()
}

// `t2_route_family_split_decision` moved to support::tier

pub(crate) fn write_t2_route_family_splits(path: &Path, rows: &[T2RouteFamilySplitRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_route_family_split_summary(output: &Path, rows: &[T2RouteFamilySplitRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.family_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 route-family split rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

pub(crate) fn t2_route_family_split_gate_failures(rows: &[T2RouteFamilySplitRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.len() == 1 && rows[0].route == "__all_t2_route_family_splits__" {
        let row = &rows[0];
        if row.family_action != "route-family-split-clear" || row.validation_status != "pass" {
            failures
                .push("route-family split clearance row has incomplete clear status".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.family_action.trim().is_empty()
            || row.disposition.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!("{} has incomplete route-family split", row.route));
        }
        if !row.qualification_effects.trim().is_empty()
            && !row.optimizer_effect.contains("qualification")
        {
            failures.push(format!(
                "{} route-family split drops qualification effects",
                row.route
            ));
        }
    }
    failures
}

// `t2_graph_contact_validation_rows` moved to support::tier

pub(crate) fn write_t2_graph_contact_validation(
    path: &Path,
    rows: &[T2GraphContactValidationRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_graph_contact_validation_summary(output: &Path, rows: &[T2GraphContactValidationRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.contact_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 graph contact validation rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

pub(crate) fn t2_graph_contact_validation_gate_failures(rows: &[T2GraphContactValidationRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.len() == 1 && rows[0].route == "__all_t2_graph_contacts__" {
        let row = &rows[0];
        if row.contact_action != "graph-contact-clear" || row.validation_status != "pass" {
            failures.push("graph contact clearance row has incomplete clear status".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.contact_action.trim().is_empty()
            || row.disposition.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete graph contact validation",
                row.route
            ));
        }
    }
    failures
}

// `t2_contact_closure_rows` moved to support::tier

pub(crate) fn write_t2_contact_closure(path: &Path, rows: &[T2ContactClosureRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_contact_closure_summary(output: &Path, rows: &[T2ContactClosureRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.contact_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 contact closure rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

pub(crate) fn t2_contact_closure_gate_failures(rows: &[T2ContactClosureRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.len() == 1 && rows[0].route == "__all_t2_contact_closures__" {
        let row = &rows[0];
        if row.contact_action != "contact-closure-clear" || row.validation_status != "pass" {
            failures.push("contact closure clearance row has incomplete clear status".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.blocker_class.trim().is_empty()
            || row.contact_action.trim().is_empty()
            || row.disposition.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!("{} has incomplete contact closure", row.route));
        }
    }
    failures
}

// `t2_endpoint_closure_rows` moved to support::tier

pub(crate) fn write_t2_endpoint_closure(path: &Path, rows: &[T2EndpointClosureRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_endpoint_closure_summary(output: &Path, rows: &[T2EndpointClosureRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.endpoint_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 endpoint closure rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

pub(crate) fn t2_endpoint_closure_gate_failures(rows: &[T2EndpointClosureRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.len() == 1 && rows[0].route == "__all_t2_endpoint_closures__" {
        let row = &rows[0];
        if row.endpoint_action != "endpoint-closure-clear" || row.validation_status != "pass" {
            failures.push("endpoint closure clearance row has incomplete clear status".to_string());
        }
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.endpoint_action.trim().is_empty()
            || row.disposition.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!("{} has incomplete endpoint closure", row.route));
        }
    }
    failures
}

pub(crate) fn load_t2_route_family_splits(path: &Path) -> Result<Vec<T2RouteFamilySplitRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t2_graph_contact_validation(path: &Path) -> Result<Vec<T2GraphContactValidationRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t2_contact_closure(path: &Path) -> Result<Vec<T2ContactClosureRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t2_endpoint_closure(path: &Path) -> Result<Vec<T2EndpointClosureRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_closure_dispositions` moved to support::tier

pub(crate) fn t2_closure_bundle_posture(
    bundle_by_route: &std::collections::HashMap<String, (String, String, String, String)>,
    route: &str,
) -> (String, String, String, String) {
    bundle_by_route
        .get(&canonical_route_key(route))
        .cloned()
        .unwrap_or_else(|| {
            (
                String::new(),
                "bundle-unchecked".to_string(),
                "join t2-blocker-closure to bundle registry".to_string(),
                String::new(),
            )
        })
}

// `tier_candidate_column_rows` moved to support::tier

pub(crate) fn tier_candidate_column_decision(
    row: &TierContactWitnessInputRow,
    closure: Option<&T2ClosureDisposition>,
) -> &'static str {
    if row.tier.eq_ignore_ascii_case("T2")
        && closure
            .map(|closure| closure.disposition == "candidate-review")
            .unwrap_or_default()
    {
        if closure
            .map(|closure| closure.bundle_status.as_str() != "bundle-ready")
            .unwrap_or_default()
        {
            return "blocked";
        }
        return "review";
    }
    match row.witness_type.as_str() {
        "regionalizer-ready" if row.validation_status.eq_ignore_ascii_case("pass") => "selected",
        "parent-region-review" => "review",
        "tier-demotion-needed" => "demote",
        _ => "blocked",
    }
}

pub(crate) fn tier_candidate_column_evidence_status(
    row: &TierContactWitnessInputRow,
    closure: Option<&T2ClosureDisposition>,
) -> String {
    match closure {
        Some(closure)
            if closure.disposition == "candidate-review"
                && closure.bundle_status == "bundle-ready" =>
        {
            "closure-accepted-bundle-ready".to_string()
        }
        Some(closure) if closure.disposition == "candidate-review" => {
            "closure-bundle-pending".to_string()
        }
        _ => row.evidence_status.clone(),
    }
}

pub(crate) fn tier_candidate_column_required_artifact(
    row: &TierContactWitnessInputRow,
    closure: Option<&T2ClosureDisposition>,
) -> String {
    match closure {
        Some(closure)
            if closure.disposition == "candidate-review"
                && closure.bundle_status != "bundle-ready" =>
        {
            "data/t2-blocker-closure.csv".to_string()
        }
        Some(closure) if closure.disposition == "candidate-review" => {
            closure.source_artifact.clone()
        }
        _ => row.required_artifact.clone(),
    }
}

pub(crate) fn write_tier_candidate_columns(path: &Path, rows: &[TierCandidateColumnRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_candidate_column_summary(output: &Path, rows: &[TierCandidateColumnRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.column_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} candidate column rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

// `tier_candidate_column_gate_failures` moved to support::tier

pub(crate) fn load_tier_candidate_columns(path: &Path) -> Result<Vec<TierCandidateColumnRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_bundle_repair_queue_rows` moved to support::tier

pub(crate) fn t2_bundle_repair_queue_action(bundle_status: &str) -> (&'static str, &'static str) {
    match bundle_status {
        "bundle-missing" => (
            "add-or-split-segment-bundle-before-regionalizer",
            "data/national-segment-bundles.csv",
        ),
        "needs-stop-chain" => (
            "author-stop-chain-before-regionalizer",
            "data/tier-stop-candidates.csv",
        ),
        "needs-terminal-stop" => (
            "complete-terminal-stop-before-regionalizer",
            "data/tier-stop-candidates.csv",
        ),
        _ => (
            "resolve-bundle-readiness-before-regionalizer",
            "data/t2-blocker-closure.csv",
        ),
    }
}

pub(crate) fn write_t2_bundle_repair_queue(path: &Path, rows: &[T2BundleRepairQueueRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_bundle_repair_queue_summary(output: &Path, rows: &[T2BundleRepairQueueRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.bundle_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} bundle repair queue rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

// `t2_bundle_repair_queue_gate_failures` moved to support::tier

pub(crate) fn load_t2_bundle_repair_queue(path: &Path) -> Result<Vec<T2BundleRepairQueueRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_regionalizer_rows` moved to support::tier

pub(crate) fn write_t2_regionalizer(path: &Path, rows: &[T2RegionalizerRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_regionalizer_summary(output: &Path, rows: &[T2RegionalizerRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.treatment_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} regionalizer rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

pub(crate) fn t2_regionalizer_gate_failures(rows: &[T2RegionalizerRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T2 regionalizer rows emitted".to_string());
        return failures;
    }
    let selected = rows
        .iter()
        .filter(|row| row.treatment_status == "selected-treatment")
        .count();
    if selected == 0 {
        failures.push("no selected T2 regional treatments".to_string());
    }
    failures
}

pub(crate) fn load_t2_regionalizer(path: &Path) -> Result<Vec<T2RegionalizerRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_service_selection_rows` moved to support::tier

// `t2_service_selection_decision` moved to support::tier

pub(crate) fn t2_qualification_action_for(
    service_action: &str,
    qualification_basis: &str,
) -> Option<route_map::BeckT2QualificationActionRow> {
    route_map::beck_t2_qualification_actions()
        .into_iter()
        .find(|action| {
            action.service_action == service_action
                && action
                    .covered_bases
                    .iter()
                    .any(|basis| *basis == qualification_basis)
        })
}

pub(crate) fn canonical_route_key(route: &str) -> String {
    route
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(|ch| ch.to_uppercase())
        .collect()
}

pub(crate) fn write_t2_service_selection(path: &Path, rows: &[T2ServiceSelectionRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_service_selection_summary(output: &Path, rows: &[T2ServiceSelectionRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.selection_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} service selection rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

// `t2_service_selection_gate_failures` moved to support::tier

pub(crate) fn load_t2_service_selection(path: &Path) -> Result<Vec<T2ServiceSelectionRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t2_service_diagnostic_queue(path: &Path) -> Result<Vec<T2ServiceDiagnosticQueueRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_service_diagnostic_queue_rows` moved to support::tier

pub(crate) fn service_diagnostic_optimizer_effect(effect: &str, qualification_effects: &str) -> String {
    if qualification_effects.trim().is_empty() {
        return effect.to_string();
    }
    format!("{effect}; qualification_effects={qualification_effects}")
}

// `t2_service_diagnostic_contract` moved to support::tier

pub(crate) fn national_bundle_matches_route(bundle: &NationalSegmentBundleRow, route: &str) -> bool {
    let key = canonical_route_key(route);
    bundle
        .route_labels
        .split(';')
        .chain(bundle.bundle_aliases.split(';').filter_map(|alias| {
            alias
                .strip_prefix("route:")
                .or_else(|| alias.strip_prefix("route-label:"))
        }))
        .any(|candidate| canonical_route_key(candidate) == key)
}

pub(crate) fn write_t2_service_diagnostic_queue(
    path: &Path,
    rows: &[T2ServiceDiagnosticQueueRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_service_diagnostic_queue_summary(output: &Path, rows: &[T2ServiceDiagnosticQueueRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.diagnostic_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} service diagnostic queue rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

// `t2_service_diagnostic_queue_gate_failures` moved to support::tier

// `t2_parallel_service_queue_rows` moved to support::tier

pub(crate) fn write_t2_parallel_service_queue(path: &Path, rows: &[T2ParallelServiceQueueRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_parallel_service_queue_summary(output: &Path, rows: &[T2ParallelServiceQueueRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.validation_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} parallel service queue rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

// `t2_parallel_service_queue_gate_failures` moved to support::tier

pub(crate) fn load_national_segment_bundles(path: &Path) -> Result<Vec<NationalSegmentBundleRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_game_t2_service_overlays(path: &Path) -> Result<Vec<GameT2ServiceOverlayRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t2_bundle_overlays(path: &Path) -> Result<Vec<T2BundleOverlayRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t2_scenario_hooks(path: &Path) -> Result<Vec<T2ScenarioHookRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_bundle_overlay_rows` moved to support::tier::t2_bundle_overlay_rows

pub(crate) fn segment_bundle_from_national_row(
    row: &NationalSegmentBundleRow,
) -> route_network::SegmentBundle {
    route_network::SegmentBundle {
        segment_bundle_id: row.segment_bundle_id.clone(),
        bundle_role: row.bundle_role.clone(),
        member_segment_ids: semicolon_values(&row.member_segment_ids),
        stitch_group_ids: semicolon_values(&row.stitch_group_ids),
        current_tiers: semicolon_values(&row.current_tiers),
        current_zone_ids: semicolon_values(&row.current_zone_ids),
        route_labels: semicolon_values(&row.route_labels),
        state_scope: semicolon_values(&row.state_scope),
        evidence_state_scope: semicolon_values(&row.evidence_state_scope),
        geometry_state_scope: semicolon_values(&row.geometry_state_scope),
        bundle_aliases: semicolon_values(&row.bundle_aliases),
        source_artifacts: semicolon_values(&row.source_artifacts),
        registry_actions: Vec::new(),
        validation_statuses: vec![row.validation_status.clone()],
        bundle_status: route_network::BundleStatus::from_label(&row.bundle_status),
    }
}

pub(crate) fn write_t2_bundle_overlays(path: &Path, rows: &[T2BundleOverlayRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_bundle_overlay_summary(output: &Path, rows: &[T2BundleOverlayRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.binding_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 bundle overlay rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

// `t2_bundle_overlay_gate_failures` moved to support::tier

// `tier_segment_candidate_rows` moved to `tier_segment_candidate_rows.rs`

pub(crate) fn merge_qualification_effects(left: &str, right: &str) -> String {
    let mut values = std::collections::BTreeSet::new();
    for value in left.split('|').chain(right.split('|')).map(str::trim) {
        if !value.is_empty() {
            values.insert(value.to_string());
        }
    }
    join_pipe_set(&values)
}

pub(crate) fn tier_candidate_segment_id(edge: &route_network::HighwayEdge) -> String {
    let first = edge.geometry.0.first().copied();
    let last = edge.geometry.0.last().copied();
    let geometry_key = match (first, last) {
        (Some(first), Some(last)) => {
            format!("{:.5},{:.5}->{:.5},{:.5}", first.x, first.y, last.x, last.y)
        }
        _ => "missing-geometry".to_string(),
    };
    format!(
        "US.HWYSEG.{:016X}",
        stable_segment_hash(&format!(
            "edge|{}|{}|{}|{:.3}",
            edge.route_id, edge.state, geometry_key, edge.length_miles
        ))
    )
}

pub(crate) fn tier_candidate_bundle_id(
    tier: &str,
    region_id: &str,
    route: &str,
    bundle_scope: &str,
) -> String {
    let identity = if bundle_scope.trim().is_empty() {
        format!("candidate-bundle|{tier}|{region_id}|{route}")
    } else {
        format!("candidate-bundle|{tier}|{region_id}|{route}|{bundle_scope}")
    };
    format!("US.HWYBUNDLE.{:016X}", stable_segment_hash(&identity))
}

pub(crate) fn tier_candidate_stitch_group_id(
    tier: &str,
    region_id: &str,
    route: &str,
    bundle_scope: &str,
) -> String {
    let identity = if bundle_scope.trim().is_empty() {
        format!("candidate-stitch|{tier}|{region_id}|{route}")
    } else {
        format!("candidate-stitch|{tier}|{region_id}|{route}|{bundle_scope}")
    };
    format!("US.HWYSTITCH.{:016X}", stable_segment_hash(&identity))
}

pub(crate) fn tier_candidate_aliases(tier: &str, region_id: &str, route: &str, bundle_scope: &str) -> String {
    let mut aliases = vec![
        format!("current-tier:{tier}"),
        format!("current-zone:{region_id}"),
        format!("route:{route}"),
        format!("route-label:{route}"),
        "layer:segment-candidate".to_string(),
    ];
    if !bundle_scope.trim().is_empty() {
        aliases.push(format!("route-family-scope:{bundle_scope}"));
    }
    aliases.join(";")
}

pub(crate) fn write_tier_segment_candidates(path: &Path, rows: &[TierSegmentCandidateRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_segment_candidate_summary(output: &Path, rows: &[TierSegmentCandidateRow]) {
    let mut by_tier = std::collections::BTreeMap::<&str, usize>::new();
    let mut bundles = std::collections::BTreeSet::<&str>::new();
    for row in rows {
        *by_tier.entry(row.tier.as_str()).or_default() += 1;
        bundles.insert(row.segment_bundle_id.as_str());
    }
    println!(
        "  wrote {} segment candidate rows across {} bundle candidates to {}",
        rows.len(),
        bundles.len(),
        output.display()
    );
    for (tier, count) in by_tier {
        println!("  {tier}: {count}");
    }
}

// `tier_segment_candidate_gate_failures` moved to support::tier

pub(crate) fn load_tier_segment_candidates(path: &Path) -> Result<Vec<TierSegmentCandidateRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `tier_pavement_docket_rows` moved to support::pavement

// `tier_pavement_decision` moved to support::pavement

pub(crate) fn normalized_iri_m_per_km(raw_iri: Option<f32>) -> Option<f32> {
    raw_iri.map(|value| {
        if value > 20.0 {
            value * 0.015_782_8
        } else {
            value
        }
    })
}

pub(crate) fn write_tier_pavement_docket(path: &Path, rows: &[TierPavementDocketRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn load_tier_pavement_docket(path: &Path) -> Result<Vec<TierPavementDocketRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn print_tier_pavement_docket_summary(
    output: &Path,
    rows: &[TierPavementDocketRow],
    details: bool,
) {
    let mut by_status = std::collections::BTreeMap::<&str, usize>::new();
    let mut by_tier = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_status.entry(row.pavement_status.as_str()).or_default() += 1;
        *by_tier.entry(row.tier.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} pavement docket rows to {}",
        rows.len(),
        output.display()
    );
    for (tier, count) in by_tier {
        println!("  {tier}: {count}");
    }
    for (status, count) in by_status {
        println!("  {status}: {count}");
    }

    if details {
        println!();
        println!(
            "{:<4} {:<8} {:<8} {:>7} {:>7} {:<24} {}",
            "Tier", "Route", "State", "IRI", "Max", "Status", "Repair action"
        );
        println!("{}", "-".repeat(116));
        for row in rows.iter().filter(|row| row.validation_status == "review") {
            println!(
                "{:<4} {:<8} {:<8} {:>7} {:>7} {:<24} {}",
                row.tier,
                row.route,
                row.state,
                row.iri_m_per_km,
                row.max_iri_m_per_km,
                row.pavement_status,
                truncate_for_table(&row.repair_action, 48)
            );
        }
    }
}

// `tier_pavement_docket_gate_failures` moved to support::pavement

#[derive(Default)]
struct TierPavementSourceGapBuilder {
    tier: String,
    route: String,
    region_id: String,
    segment_bundle_id: String,
    stitch_group_id: String,
    member_count: usize,
    blocker_count: usize,
    blocker_statuses: std::collections::BTreeSet<String>,
    affected_states: std::collections::BTreeSet<String>,
    affected_edge_ids: std::collections::BTreeSet<u64>,
    source_contracts: std::collections::BTreeSet<String>,
}

// `tier_pavement_source_gap_rows` moved to support::pavement

pub(crate) fn tier_pavement_route_state_scope(
    graph: Option<&route_network::HighwayGraph>,
    route: &str,
) -> String {
    graph
        .and_then(|graph| route_network::aggregate_corridor(graph, route))
        .map(|corridor| corridor.states.join(";"))
        .unwrap_or_default()
}

pub(crate) fn tier_pavement_source_gap_decision(
    blocker_statuses: &str,
) -> (&'static str, &'static str, &'static str, &'static str) {
    if blocker_statuses.contains("pavement-repair-required") {
        return (
            "price pavement repair debt for failing member segments",
            "data/tier-pavement-docket.csv",
            "bundle remains service-addressable while pavement repair debt is priced and paid before SLA or transit readiness claims",
            "review",
        );
    }
    if blocker_statuses.contains("pavement-source-needed") {
        return (
            "price pavement evidence debt for affected member edges",
            "data/standards-l1-inventory.csv",
            "bundle remains service-addressable while pavement source debt is acquired and converted to pass or repair debt",
            "review",
        );
    }
    (
        "review pavement debt status",
        "data/tier-pavement-docket.csv",
        "bundle remains service-addressable while pavement debt is classified",
        "review",
    )
}

pub(crate) fn write_tier_pavement_source_gaps(path: &Path, rows: &[TierPavementSourceGapRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_source_gap_summary(
    output: &Path,
    rows: &[TierPavementSourceGapRow],
    details: bool,
) {
    let mut by_tier = std::collections::BTreeMap::<&str, usize>::new();
    let mut blocker_total = 0usize;
    for row in rows {
        *by_tier.entry(row.tier.as_str()).or_default() += 1;
        blocker_total += row.blocker_count;
    }
    println!(
        "  wrote {} pavement source-gap rows to {}",
        rows.len(),
        output.display()
    );
    println!("  pavement debt member segments: {blocker_total}");
    for (tier, count) in by_tier {
        println!("  {tier}: {count}");
    }

    if details {
        println!();
        println!(
            "{:<4} {:<8} {:>7} {:>7} {:<22} {}",
            "Tier", "Route", "Members", "Blocked", "States", "Action"
        );
        println!("{}", "-".repeat(112));
        for row in rows {
            println!(
                "{:<4} {:<8} {:>7} {:>7} {:<22} {}",
                row.tier,
                row.route,
                row.member_count,
                row.blocker_count,
                truncate_for_table(&row.affected_states, 22),
                truncate_for_table(&row.source_action, 52)
            );
        }
    }
}

// `tier_pavement_source_gap_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_source_gaps(path: &Path) -> Result<Vec<TierPavementSourceGapRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

const PAVEMENT_EVIDENCE_COST_PER_MEMBER_M: f64 = 0.05;
const PAVEMENT_REPAIR_COST_PER_MEMBER_M: f64 = 2.50;

#[derive(Debug, Clone, Default)]
struct PavementDebtBudgetIndex {
    by_bundle: std::collections::HashMap<String, TierPavementDebtBudgetRow>,
    by_route: std::collections::HashMap<String, TierPavementDebtBudgetRollup>,
}

#[derive(Debug, Clone, Default)]
struct TierPavementDebtBudgetRollup {
    total_debt_cost_m: f64,
    debt_classes: std::collections::BTreeSet<String>,
    affected_bundles: std::collections::BTreeSet<String>,
}

// `tier_pavement_debt_budget_rows_with_exclusions` moved to support::pavement

pub(crate) fn pavement_gap_has_accepted_repair_funding(
    gap_row: &TierPavementSourceGapRow,
    funding_rows: &[TierPavementRepairFundingAcceptanceRow],
) -> bool {
    if !gap_row
        .blocker_statuses
        .contains("pavement-repair-required")
    {
        return false;
    }
    let estimated_repair_cost_m =
        round_cost_m(gap_row.blocker_count as f64 * PAVEMENT_REPAIR_COST_PER_MEMBER_M);
    funding_rows.iter().any(|funding| {
        funding.validation_status == "pass"
            && funding.acceptance_status == "accepted-full-cost-repair-funding"
            && funding.tier == gap_row.tier
            && route_display_key(&funding.route) == route_display_key(&gap_row.route)
            && funding.segment_bundle_id == gap_row.segment_bundle_id
            && semicolon_values(&gap_row.affected_states)
                .iter()
                .any(|state| state == &funding.state)
            && funding.committed_amount_m + 1e-6 >= estimated_repair_cost_m
            && funding.covered_repair_cost_m + 1e-6 >= estimated_repair_cost_m
    })
}

pub(crate) fn pavement_gap_has_accepted_route_state_exclusion(
    gap_row: &TierPavementSourceGapRow,
    exclusion_rows: &[TierPavementRouteStateExclusionRow],
) -> bool {
    exclusion_rows.iter().any(|exclusion| {
        exclusion.validation_status == "pass"
            && exclusion.exclusion_status == "route-state-not-supported"
            && exclusion.tier == gap_row.tier
            && route_display_key(&exclusion.route) == route_display_key(&gap_row.route)
            && exclusion.segment_bundle_id == gap_row.segment_bundle_id
            && semicolon_values(&gap_row.affected_states)
                .iter()
                .any(|state| state == &exclusion.state)
    })
}

pub(crate) fn load_tier_pavement_route_state_exclusions(
    path: &Path,
) -> Result<Vec<TierPavementRouteStateExclusionRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_tier_pavement_repair_funding_acceptance(
    path: &Path,
) -> Result<Vec<TierPavementRepairFundingAcceptanceRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn round_cost_m(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

pub(crate) fn write_tier_pavement_debt_budget(path: &Path, rows: &[TierPavementDebtBudgetRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn load_tier_pavement_debt_budget(path: &Path) -> Result<Vec<TierPavementDebtBudgetRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t1_topology_repairs(path: &Path) -> Result<Vec<T1TopologyRepairRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t1_schematic_geometry_blocker_relief(
    path: &Path,
) -> Result<Vec<T1SchematicGeometryBlockerReliefRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t1_schematic_relief_route_set(
    rows: &[T1SchematicGeometryBlockerReliefRow],
) -> std::collections::BTreeSet<String> {
    rows.iter()
        .filter(|row| {
            row.relief_decision == "relief-ready-for-constraint-ledger-replay"
                && row.blocker_count_after == 0
                && row.claim_blocker_delta < 0
        })
        .flat_map(|row| row.affected_routes.split(';'))
        .filter(|route| !route.trim().is_empty())
        .map(route_display_key)
        .collect()
}

pub(crate) fn t2_transfer_relief_route_set(
    rows: &[T2BeckTransferComplexityBlockerReliefRow],
) -> std::collections::BTreeSet<String> {
    rows.iter()
        .filter(|row| {
            row.relief_decision == "relief-ready-for-constraint-ledger-replay"
                && row.blocker_count_after == 0
                && row.claim_blocker_delta < 0
        })
        .map(|row| route_display_key(&row.route))
        .collect()
}

pub(crate) fn load_t2_beck_label_density_blocker_relief(
    path: &Path,
) -> Result<Vec<T2BeckLabelDensityBlockerReliefRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_label_density_relief_route_set(
    rows: &[T2BeckLabelDensityBlockerReliefRow],
) -> std::collections::BTreeSet<String> {
    rows.iter()
        .filter(|row| {
            row.relief_decision == "relief-ready-for-constraint-ledger-replay"
                && row.blocker_count_after == 0
                && row.claim_blocker_delta < 0
        })
        .map(|row| route_display_key(&row.route))
        .collect()
}

pub(crate) fn load_t2_beck_long_connector_blocker_relief(
    path: &Path,
) -> Result<Vec<T2BeckLongConnectorBlockerReliefRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_long_connector_relief_route_set(
    rows: &[T2BeckLongConnectorBlockerReliefRow],
) -> std::collections::BTreeSet<String> {
    rows.iter()
        .filter(|row| {
            row.relief_decision == "relief-ready-for-constraint-ledger-replay"
                && row.blocker_count_after == 0
                && row.claim_blocker_delta < 0
        })
        .map(|row| route_display_key(&row.route))
        .collect()
}

pub(crate) fn load_t2_game_publication_evidence_blocker_relief(
    path: &Path,
) -> Result<Vec<T2GamePublicationEvidenceBlockerReliefRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_game_publication_relief_scenario_set(
    rows: &[T2GamePublicationEvidenceBlockerReliefRow],
) -> std::collections::BTreeSet<String> {
    rows.iter()
        .filter(|row| {
            row.relief_decision == "relief-ready-for-constraint-ledger-replay"
                && row.blocker_count_after == 0
                && row.claim_blocker_delta < 0
        })
        .map(|row| row.scenario_id.clone())
        .collect()
}

pub(crate) fn load_t2_game_ops_bundle_evidence_blocker_relief(
    path: &Path,
) -> Result<Vec<T2GameOpsBundleEvidenceBlockerReliefRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_game_ops_bundle_relief_bundle_set(
    rows: &[T2GameOpsBundleEvidenceBlockerReliefRow],
) -> std::collections::BTreeSet<String> {
    rows.iter()
        .filter(|row| {
            row.relief_decision == "relief-ready-for-constraint-ledger-replay"
                && row.blocker_count_after == 0
                && row.claim_blocker_delta < 0
        })
        .map(|row| row.segment_bundle_id.clone())
        .collect()
}

pub(crate) fn load_t3_lower_tier_feeder_gap_blocker_relief(
    path: &Path,
) -> Result<Vec<T3LowerTierFeederGapBlockerReliefRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t3_feeder_relief_route_set(
    rows: &[T3LowerTierFeederGapBlockerReliefRow],
) -> std::collections::BTreeSet<String> {
    rows.iter()
        .filter(|row| {
            row.relief_decision == "relief-ready-for-constraint-ledger-replay"
                && row.blocker_count_after == 0
                && row.claim_blocker_delta < 0
        })
        .map(|row| route_display_key(&row.route))
        .collect()
}

pub(crate) fn load_t2_parallel_service_queue(path: &Path) -> Result<Vec<T2ParallelServiceQueueRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `optimizer_constraint_ledger_rows` moved to support::optimizer

// `optimizer_constraint_ledger_rows_with_terminal_proof` moved to `optimizer_ledger.rs`

pub(crate) fn game_ops_bundle_relief_optimizer_effect(
    row: &T2GameOpsBundleEvidenceBlockerReliefRow,
) -> String {
    let base = "accepted game/ops bundle evidence policy removes bundle-binding blockers";
    let mut parts = Vec::new();
    if !row.qualification_effects.trim().is_empty() {
        parts.push(format!(
            "qualification_effects={}",
            row.qualification_effects
        ));
    }
    if !row.qualification_gate_policy.trim().is_empty() {
        parts.push(format!(
            "qualification_gate_policy={}",
            row.qualification_gate_policy
        ));
    }
    if !row.qualification_game_use.trim().is_empty() {
        parts.push(format!(
            "qualification_game_use={}",
            row.qualification_game_use
        ));
    }
    if parts.is_empty() {
        return base.to_string();
    }
    format!("{base}; {}", parts.join("; "))
}

pub(crate) fn t1_topology_constraint_mapping(
    row: &T1TopologyRepairRow,
) -> (u8, &'static str, &'static str, String, &'static str) {
    match row.repair_type.as_str() {
        "shared-backbone-policy" => (
            13,
            "schematic_geometry",
            "claim-blocker",
            "review".to_string(),
            "map|publication",
        ),
        "national-relay-justification" => (
            1,
            "promise_portfolio",
            "selection-hard",
            "review".to_string(),
            "sla|publication",
        ),
        "held-candidate" => (
            3,
            "route_budget",
            "review",
            row.validation_status.clone(),
            "",
        ),
        _ => (
            5,
            "topology_connectivity",
            "review",
            row.validation_status.clone(),
            "map|publication",
        ),
    }
}

pub(crate) fn stable_id_fragment(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .collect::<String>()
        .to_ascii_uppercase()
}

pub(crate) fn write_optimizer_constraint_ledger(
    path: &Path,
    rows: &[OptimizerConstraintLedgerRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

// `print_optimizer_constraint_ledger_summary` moved to support::print

pub(crate) fn beck_t2_constraint_mapping(review_flag: &str) -> (&'static str, &'static str, &'static str) {
    match review_flag {
        "unstopped-t1-contact-review" => (
            "beck_unstopped_contact",
            "add-transfer-stop-or-realign-contact",
            "data/beck-t2-diagnostics.csv",
        ),
        "parallel-spacing-review" => (
            "beck_parallel_spacing",
            "separate-merge-or-demote-parallel-service",
            "data/t2-parallel-service-queue.csv",
        ),
        "split-anchor-review" => (
            "beck_split_anchor",
            "add-split-anchor-stop-or-use-single-parent-color",
            "data/beck-t2-diagnostics.csv",
        ),
        "duplicate-service-review" => (
            "beck_duplicate_service",
            "merge-demote-or-prove-distinct-parent-service",
            "data/t2-service-selection.csv",
        ),
        "dense-label-review" | "dense-transfer-review" => (
            "beck_label_density",
            "space-labels-stops-or-split-service",
            "data/beck-t2-diagnostics.csv",
        ),
        "transfer-complexity-review" => (
            "beck_transfer_complexity",
            "simplify-transfer-spine-or-add-zone-map",
            "data/beck-t2-diagnostics.csv",
        ),
        "long-connector-review" => (
            "beck_long_connector",
            "review-long-connector-treatment",
            "data/beck-t2-diagnostics.csv",
        ),
        _ => (
            "beck_schematic_review",
            "review-beck-diagnostic",
            "data/beck-t2-diagnostics.csv",
        ),
    }
}

pub(crate) fn beck_t2_constraint_penalty(row: &route_map::BeckT2DiagnosticRow) -> f64 {
    match row.review_flag {
        "unstopped-t1-contact-review" => row.unstopped_t1_contact_count.max(1) as f64,
        "parallel-spacing-review" => row.close_parallel_count.max(1) as f64,
        "duplicate-service-review" => row.duplicate_service_count.max(1) as f64,
        "dense-label-review" | "dense-transfer-review" => {
            (row.label_density_per_100px - 0.95).max(0.25)
        }
        "transfer-complexity-review" => 1.0 + row.transfer_stop_count.saturating_sub(4) as f64,
        "long-connector-review" => 1.0,
        "split-anchor-review" => 1.0,
        _ => 1.0,
    }
}

// `optimizer_constraint_ledger_gate_failures` moved to support::gates

pub(crate) fn load_optimizer_constraint_ledger(path: &Path) -> Result<Vec<OptimizerConstraintLedgerRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_optimizer_constraint_budget(path: &Path) -> Result<Vec<OptimizerConstraintBudgetRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_game_ops_binding_intake_rows(
    budget_rows: &[OptimizerConstraintBudgetRow],
) -> Vec<T2GameOpsBindingIntakeRow> {
    let mut rows = budget_rows
        .iter()
        .filter(|row| {
            row.tier == "T2"
                && constraint_class_values(&row.top_constraint_classes)
                    .iter()
                    .any(|class| class == "game_ops_bundle_binding")
        })
        .map(|row| T2GameOpsBindingIntakeRow {
            intake_id: format!("T2GAMEOPSINTAKE-{}", stable_id_fragment(&row.budget_id)),
            budget_id: row.budget_id.clone(),
            subject_id: row.subject_id.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            route: row.route.clone(),
            claim_blocker_count: row.claim_blocker_count,
            blocked_claims: row.blocking_claims.clone(),
            top_constraint_classes: row.top_constraint_classes.clone(),
            qualification_effects: row.qualification_effects.clone(),
            next_artifacts: row.next_artifacts.clone(),
            constraint_ledger_artifact: row.constraint_ledger_artifact.clone(),
            intake_status: "decision-needed".to_string(),
            decision_artifact: "data/t2-game-ops-binding-decisions.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.subject_id.cmp(&right.subject_id))
    });
    rows
}

pub(crate) fn write_t2_game_ops_binding_intake(path: &Path, rows: &[T2GameOpsBindingIntakeRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_game_ops_binding_intake_summary(output: &Path, rows: &[T2GameOpsBindingIntakeRow]) {
    println!(
        "  wrote {} T2 game/ops binding intake rows to {}",
        rows.len(),
        output.display()
    );
}

// `t2_game_ops_binding_intake_gate_failures` moved to support::tier

pub(crate) fn load_t2_game_ops_binding_intake(path: &Path) -> Result<Vec<T2GameOpsBindingIntakeRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_game_ops_binding_decision_rows` moved to support::tier

pub(crate) fn write_t2_game_ops_binding_decisions(
    path: &Path,
    rows: &[T2GameOpsBindingDecisionRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_game_ops_binding_decision_summary(output: &Path, rows: &[T2GameOpsBindingDecisionRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 game/ops binding decision rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

// `t2_game_ops_binding_decision_gate_failures` moved to support::tier

pub(crate) fn load_t2_game_ops_binding_decisions(path: &Path) -> Result<Vec<T2GameOpsBindingDecisionRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_bundle_overlay_repair_target_rows` moved to support::tier

pub(crate) fn write_t2_bundle_overlay_repair_targets(
    path: &Path,
    rows: &[T2BundleOverlayRepairTargetRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_bundle_overlay_repair_target_summary(
    output: &Path,
    rows: &[T2BundleOverlayRepairTargetRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.repair_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 bundle overlay repair target rows to {}",
        rows.len(),
        output.display()
    );
    for (class, count) in counts {
        println!("  {class}: {count}");
    }
}

// `t2_bundle_overlay_repair_target_gate_failures` moved to support::tier

pub(crate) fn load_t2_bundle_overlay_repair_targets(
    path: &Path,
) -> Result<Vec<T2BundleOverlayRepairTargetRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_service_class_repair_docket_rows` moved to support::tier

pub(crate) fn service_repair_optimizer_effect(effect: &str, row: &T2BundleOverlayRepairTargetRow) -> String {
    if row.qualification_effects.trim().is_empty() {
        return effect.to_string();
    }
    format!(
        "{effect}; qualification_effects={}",
        row.qualification_effects
    )
}

pub(crate) fn write_t2_service_class_repair_docket(
    path: &Path,
    rows: &[T2ServiceClassRepairDocketRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_service_class_repair_docket_summary(
    output: &Path,
    rows: &[T2ServiceClassRepairDocketRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.service_repair_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 service-class repair docket rows to {}",
        rows.len(),
        output.display()
    );
    for (class, count) in counts {
        println!("  {class}: {count}");
    }
}

// `t2_service_class_repair_docket_gate_failures` moved to support::tier

pub(crate) fn load_t2_service_class_repair_docket(path: &Path) -> Result<Vec<T2ServiceClassRepairDocketRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_game_ops_bundle_evidence_review_rows` moved to support::tier

pub(crate) fn write_t2_game_ops_bundle_evidence_review(
    path: &Path,
    rows: &[T2GameOpsBundleEvidenceReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_game_ops_bundle_evidence_review_summary(
    output: &Path,
    rows: &[T2GameOpsBundleEvidenceReviewRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.repair_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 game/ops bundle evidence review rows to {}",
        rows.len(),
        output.display()
    );
    for (class, count) in counts {
        println!("  {class}: {count}");
    }
}

// `t2_game_ops_bundle_evidence_review_gate_failures` moved to support::tier

pub(crate) fn load_t2_game_ops_bundle_evidence_review(
    path: &Path,
) -> Result<Vec<T2GameOpsBundleEvidenceReviewRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_game_ops_bundle_evidence_policy_rows(
    review_rows: &[T2GameOpsBundleEvidenceReviewRow],
) -> Vec<T2GameOpsBundleEvidencePolicyRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| row.claim_blocker_delta == 0 && row.blocker_count_after > 0)
        .map(|row| T2GameOpsBundleEvidencePolicyRow {
            policy_id: format!(
                "T2GAMEOPSBUNDLEPOLICY-{}",
                stable_id_fragment(&row.review_id)
            ),
            review_id: row.review_id.clone(),
            decision_id: row.decision_id.clone(),
            target_id: row.target_id.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            repair_class: row.repair_class.clone(),
            service_repair_class: row.service_repair_class.clone(),
            evidence_artifact: row.evidence_artifact.clone(),
            qualification_effects: row.qualification_effects.clone(),
            qualification_gate_policy: row.qualification_gate_policy.clone(),
            qualification_game_use: row.qualification_game_use.clone(),
            required_evidence: t2_game_ops_bundle_required_evidence(row).to_string(),
            evidence_policy_decision: "bundle-evidence-policy-authored-review".to_string(),
            policy_treatment: t2_game_ops_bundle_policy_treatment(row).to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-game-ops-bundle-evidence-policy-acceptance.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.segment_bundle_id.cmp(&right.segment_bundle_id))
    });
    rows
}

pub(crate) fn t2_game_ops_bundle_required_evidence(row: &T2GameOpsBundleEvidenceReviewRow) -> &'static str {
    match row.repair_class.as_str() {
        "service-class" if row.service_repair_class == "local-zone" => {
            "accepted-local-zone-overlay-handoff"
        }
        "service-class" => "accepted-service-overlay-classification",
        "stitched-member" => "accepted-stitched-member-proof-review",
        "stop-chain" => "accepted-stop-chain-repair-or-demotion",
        "terminal-stop" => "accepted-terminal-stop-repair",
        _ => "accepted-bundle-binding-repair-evidence",
    }
}

pub(crate) fn t2_game_ops_bundle_policy_treatment(row: &T2GameOpsBundleEvidenceReviewRow) -> &'static str {
    match row.repair_class.as_str() {
        "service-class" if row.service_repair_class == "local-zone" => {
            "hold game/ops claims until local-zone overlay handoff is accepted or explicitly carried"
        }
        "service-class" => {
            "hold game/ops claims until service-overlay classification is accepted"
        }
        "stitched-member" => {
            "hold game/ops claims until stitched-member proof is accepted or explicitly carried"
        }
        "stop-chain" => "hold game/ops claims until stop-chain repair is accepted or demoted",
        "terminal-stop" => "hold game/ops claims until terminal-stop repair is accepted",
        _ => "hold game/ops claims until bundle-binding repair evidence is accepted",
    }
}

pub(crate) fn write_t2_game_ops_bundle_evidence_policy(
    path: &Path,
    rows: &[T2GameOpsBundleEvidencePolicyRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_game_ops_bundle_evidence_policy_summary(
    output: &Path,
    rows: &[T2GameOpsBundleEvidencePolicyRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 game/ops bundle evidence policy rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t2_game_ops_bundle_evidence_policy_gate_failures` moved to support::tier

pub(crate) fn load_t2_game_ops_bundle_evidence_policy(
    path: &Path,
) -> Result<Vec<T2GameOpsBundleEvidencePolicyRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_game_ops_bundle_evidence_policy_acceptance_rows(
    policy_rows: &[T2GameOpsBundleEvidencePolicyRow],
) -> Vec<T2GameOpsBundleEvidencePolicyAcceptanceRow> {
    let mut rows = policy_rows
        .iter()
        .filter(|row| {
            row.evidence_policy_decision == "bundle-evidence-policy-authored-review"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2GameOpsBundleEvidencePolicyAcceptanceRow {
            acceptance_id: format!(
                "T2GAMEOPSBUNDLEACCEPT-{}",
                stable_id_fragment(&row.policy_id)
            ),
            policy_id: row.policy_id.clone(),
            review_id: row.review_id.clone(),
            decision_id: row.decision_id.clone(),
            target_id: row.target_id.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            accepted_required_evidence: row.required_evidence.clone(),
            accepted_policy_treatment: row.policy_treatment.clone(),
            qualification_effects: row.qualification_effects.clone(),
            qualification_gate_policy: row.qualification_gate_policy.clone(),
            qualification_game_use: row.qualification_game_use.clone(),
            acceptance_decision: "bundle-evidence-policy-accepted".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-game-ops-bundle-evidence-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.segment_bundle_id.cmp(&right.segment_bundle_id))
    });
    rows
}

pub(crate) fn write_t2_game_ops_bundle_evidence_policy_acceptance(
    path: &Path,
    rows: &[T2GameOpsBundleEvidencePolicyAcceptanceRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_game_ops_bundle_evidence_policy_acceptance_summary(
    output: &Path,
    rows: &[T2GameOpsBundleEvidencePolicyAcceptanceRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 game/ops bundle evidence policy acceptance rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t2_game_ops_bundle_evidence_policy_acceptance_gate_failures` moved to support::tier

pub(crate) fn load_t2_game_ops_bundle_evidence_policy_acceptance(
    path: &Path,
) -> Result<Vec<T2GameOpsBundleEvidencePolicyAcceptanceRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_game_ops_bundle_evidence_blocker_relief_rows(
    acceptance_rows: &[T2GameOpsBundleEvidencePolicyAcceptanceRow],
) -> Vec<T2GameOpsBundleEvidenceBlockerReliefRow> {
    let mut rows = acceptance_rows
        .iter()
        .filter(|row| {
            row.acceptance_decision == "bundle-evidence-policy-accepted"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2GameOpsBundleEvidenceBlockerReliefRow {
            relief_id: format!(
                "T2GAMEOPSBUNDLERELIEF-{}",
                stable_id_fragment(&row.acceptance_id)
            ),
            acceptance_id: row.acceptance_id.clone(),
            policy_id: row.policy_id.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            accepted_required_evidence: row.accepted_required_evidence.clone(),
            qualification_effects: row.qualification_effects.clone(),
            qualification_gate_policy: row.qualification_gate_policy.clone(),
            qualification_game_use: row.qualification_game_use.clone(),
            relief_decision: "relief-ready-for-constraint-ledger-replay".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: String::new(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: 0,
            claim_blocker_delta: -(row.blocker_count_after as isize),
            ledger_replay_status: "pending-optimizer-constraint-ledger-replay".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.segment_bundle_id.cmp(&right.segment_bundle_id))
    });
    rows
}

pub(crate) fn write_t2_game_ops_bundle_evidence_blocker_relief(
    path: &Path,
    rows: &[T2GameOpsBundleEvidenceBlockerReliefRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_game_ops_bundle_evidence_blocker_relief_summary(
    output: &Path,
    rows: &[T2GameOpsBundleEvidenceBlockerReliefRow],
) {
    let before = rows
        .iter()
        .map(|row| row.blocker_count_before)
        .sum::<usize>();
    let after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 game/ops bundle evidence blocker relief rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers before: {before}");
    println!("  claim blockers after: {after}");
}

// `t2_game_ops_bundle_evidence_blocker_relief_gate_failures` moved to support::tier::t2_game_ops_bundle_evidence_blocker_relief_gate_failures

// `t2_service_overlay_diagnostic_decision_rows` moved to support::tier

pub(crate) fn write_t2_service_overlay_diagnostic_decisions(
    path: &Path,
    rows: &[T2ServiceOverlayDiagnosticDecisionRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_service_overlay_diagnostic_decision_summary(
    output: &Path,
    rows: &[T2ServiceOverlayDiagnosticDecisionRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.overlay_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 service overlay diagnostic decision rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

// `t2_service_overlay_diagnostic_decision_gate_failures` moved to support::tier

// `t2_local_zone_overlay_handoff_rows` moved to support::tier

pub(crate) fn write_t2_local_zone_overlay_handoff(
    path: &Path,
    rows: &[T2LocalZoneOverlayHandoffRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_local_zone_overlay_handoff_summary(
    output: &Path,
    rows: &[T2LocalZoneOverlayHandoffRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.handoff_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 local-zone overlay handoff rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

// `t2_local_zone_overlay_handoff_gate_failures` moved to support::tier

// `t2_bundle_readiness_disposition_rows` moved to support::tier

pub(crate) fn write_t2_bundle_readiness_disposition(
    path: &Path,
    rows: &[T2BundleReadinessDispositionRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_bundle_readiness_disposition_summary(
    output: &Path,
    rows: &[T2BundleReadinessDispositionRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.disposition.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 bundle readiness disposition rows to {}",
        rows.len(),
        output.display()
    );
    for (disposition, count) in counts {
        println!("  {disposition}: {count}");
    }
}

// `t2_bundle_readiness_disposition_gate_failures` moved to support::tier

pub(crate) fn load_t2_bundle_readiness_disposition(
    path: &Path,
) -> Result<Vec<T2BundleReadinessDispositionRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_bundle_readiness_repair_docket_rows(
    readiness_rows: &[T2BundleReadinessDispositionRow],
) -> Vec<T2BundleReadinessRepairDocketRow> {
    let mut rows = readiness_rows
        .iter()
        .filter(|row| row.disposition == "repair-needed")
        .map(|row| {
            let repair_action = match row.readiness_class.as_str() {
                "stop-chain" => "author-stop-chain-before-bundle-pass",
                "stitched-member" => "stitch-member-segments-before-bundle-pass",
                "terminal-stop" => "author-terminal-stop-before-bundle-pass",
                _ => "manual-bundle-readiness-repair",
            };
            T2BundleReadinessRepairDocketRow {
                repair_id: format!(
                    "T2BUNDLEREADINESSREPAIR-{}",
                    stable_id_fragment(&row.disposition_id)
                ),
                disposition_id: row.disposition_id.clone(),
                target_id: row.target_id.clone(),
                route: row.route.clone(),
                segment_bundle_id: row.segment_bundle_id.clone(),
                readiness_class: row.readiness_class.clone(),
                repair_decision: "repair-needed".to_string(),
                repair_action: repair_action.to_string(),
                qualification_effects: row.qualification_effects.clone(),
                required_artifact: row.required_artifact.clone(),
                next_artifact: row.next_artifact.clone(),
                blocks_claims: row.blocks_claims.clone(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.readiness_class
            .cmp(&right.readiness_class)
            .then(left.route.cmp(&right.route))
    });
    rows
}

pub(crate) fn write_t2_bundle_readiness_repair_docket(
    path: &Path,
    rows: &[T2BundleReadinessRepairDocketRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_bundle_readiness_repair_docket_summary(
    output: &Path,
    rows: &[T2BundleReadinessRepairDocketRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.readiness_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 bundle readiness repair docket rows to {}",
        rows.len(),
        output.display()
    );
    for (readiness_class, count) in counts {
        println!("  {readiness_class}: {count}");
    }
}

// `t2_bundle_readiness_repair_docket_gate_failures` moved to support::tier

pub(crate) fn load_t2_bundle_readiness_repair_docket(
    path: &Path,
) -> Result<Vec<T2BundleReadinessRepairDocketRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_bundle_readiness_repair_evidence_rows` moved to support::tier

pub(crate) fn write_t2_bundle_readiness_repair_evidence(
    path: &Path,
    rows: &[T2BundleReadinessRepairEvidenceRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_bundle_readiness_repair_evidence_summary(
    output: &Path,
    rows: &[T2BundleReadinessRepairEvidenceRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.evidence_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 bundle readiness repair evidence rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

// `t2_bundle_readiness_repair_evidence_gate_failures` moved to support::tier

pub(crate) fn load_t2_bundle_readiness_repair_evidence(
    path: &Path,
) -> Result<Vec<T2BundleReadinessRepairEvidenceRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t2_bundle_overlay_repair_delta(path: &Path) -> Result<Vec<T2BundleOverlayRepairDeltaRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_bundle_readiness_replay_decision_rows` moved to support::tier

pub(crate) fn write_t2_bundle_readiness_replay_decisions(
    path: &Path,
    rows: &[T2BundleReadinessReplayDecisionRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_bundle_readiness_replay_decision_summary(
    output: &Path,
    rows: &[T2BundleReadinessReplayDecisionRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.replay_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 bundle readiness replay decision rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

// `t2_bundle_readiness_replay_decision_gate_failures` moved to support::tier

pub(crate) fn load_t2_bundle_readiness_replay_decisions(
    path: &Path,
) -> Result<Vec<T2BundleReadinessReplayDecisionRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_national_bundle_readiness_audit_rows` moved to support::tier

pub(crate) fn write_t2_national_bundle_readiness_audit(
    path: &Path,
    rows: &[T2NationalBundleReadinessAuditRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_national_bundle_readiness_audit_summary(
    output: &Path,
    rows: &[T2NationalBundleReadinessAuditRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.bundle_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 national bundle readiness audit rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

// `t2_national_bundle_readiness_audit_gate_failures` moved to support::tier

pub(crate) fn load_t2_national_bundle_readiness_audit(
    path: &Path,
) -> Result<Vec<T2NationalBundleReadinessAuditRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_stitched_member_registry_handoff_rows` moved to support::tier

pub(crate) fn write_t2_stitched_member_registry_handoff(
    path: &Path,
    rows: &[T2StitchedMemberRegistryHandoffRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_stitched_member_registry_handoff_summary(
    output: &Path,
    rows: &[T2StitchedMemberRegistryHandoffRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.handoff_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member registry handoff rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

// `t2_stitched_member_registry_handoff_gate_failures` moved to support::tier

pub(crate) fn load_t2_stitched_member_registry_handoff(
    path: &Path,
) -> Result<Vec<T2StitchedMemberRegistryHandoffRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_stitched_member_candidate_scope_review_rows` moved to support::tier

pub(crate) fn write_t2_stitched_member_candidate_scope_review(
    path: &Path,
    rows: &[T2StitchedMemberCandidateScopeReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_stitched_member_candidate_scope_review_summary(
    output: &Path,
    rows: &[T2StitchedMemberCandidateScopeReviewRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.scope_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member candidate scope review rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

// `t2_stitched_member_candidate_scope_review_gate_failures` moved to support::tier

pub(crate) fn load_t2_stitched_member_candidate_scope_review(
    path: &Path,
) -> Result<Vec<T2StitchedMemberCandidateScopeReviewRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_stitched_member_decision_docket_rows` moved to support::tier

pub(crate) fn write_t2_stitched_member_decision_docket(
    path: &Path,
    rows: &[T2StitchedMemberDecisionDocketRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_stitched_member_decision_docket_summary(
    output: &Path,
    rows: &[T2StitchedMemberDecisionDocketRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member decision docket rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

// `t2_stitched_member_decision_docket_gate_failures` moved to support::tier

pub(crate) fn load_t2_stitched_member_decision_docket(
    path: &Path,
) -> Result<Vec<T2StitchedMemberDecisionDocketRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_stitched_member_split_plan_rows` moved to support::tier

pub(crate) fn write_t2_stitched_member_split_plan(
    path: &Path,
    rows: &[T2StitchedMemberSplitPlanRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_stitched_member_split_plan_summary(
    output: &Path,
    rows: &[T2StitchedMemberSplitPlanRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.route.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member split plan rows to {}",
        rows.len(),
        output.display()
    );
    for (route, count) in counts {
        println!("  {route}: {count}");
    }
}

// `t2_stitched_member_split_plan_gate_failures` moved to support::tier

pub(crate) fn load_t2_stitched_member_split_plan(path: &Path) -> Result<Vec<T2StitchedMemberSplitPlanRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_stitched_member_selection_docket_rows(
    split_rows: &[T2StitchedMemberSplitPlanRow],
) -> Vec<T2StitchedMemberSelectionDocketRow> {
    let mut rows = split_rows
        .iter()
        .map(|split| T2StitchedMemberSelectionDocketRow {
            selection_docket_id: format!(
                "T2STITCHEDSELECT-{}",
                stable_id_fragment(&split.split_plan_id)
            ),
            split_plan_id: split.split_plan_id.clone(),
            route: split.route.clone(),
            blocked_segment_bundle_id: split.blocked_segment_bundle_id.clone(),
            candidate_segment_bundle_id: split.candidate_segment_bundle_id.clone(),
            state_scope: split.state_scope.clone(),
            candidate_member_count: split.candidate_member_count,
            candidate_length_miles: split.candidate_length_miles,
            selection_decision: "evidence-needed".to_string(),
            selection_action: "collect-state-scope-evidence-before-decision".to_string(),
            evidence_requirement:
                "manual route-family service continuity evidence before in-scope or rejected status"
                    .to_string(),
            blocked_claims_before: split.blocked_claims_after.clone(),
            blocked_claims_after: split.blocked_claims_after.clone(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.state_scope.cmp(&right.state_scope))
            .then(
                left.candidate_segment_bundle_id
                    .cmp(&right.candidate_segment_bundle_id),
            )
    });
    rows
}

pub(crate) fn write_t2_stitched_member_selection_docket(
    path: &Path,
    rows: &[T2StitchedMemberSelectionDocketRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_stitched_member_selection_docket_summary(
    output: &Path,
    rows: &[T2StitchedMemberSelectionDocketRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.selection_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member selection docket rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

// `t2_stitched_member_selection_docket_gate_failures` moved to support::tier

pub(crate) fn load_t2_stitched_member_selection_docket(
    path: &Path,
) -> Result<Vec<T2StitchedMemberSelectionDocketRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_stitched_member_evidence_contract_rows(
    selection_rows: &[T2StitchedMemberSelectionDocketRow],
) -> Vec<T2StitchedMemberEvidenceContractRow> {
    let mut rows = selection_rows
        .iter()
        .filter(|row| row.selection_decision == "evidence-needed")
        .map(|selection| T2StitchedMemberEvidenceContractRow {
            evidence_contract_id: format!(
                "T2STITCHEDEVIDENCE-{}",
                stable_id_fragment(&selection.selection_docket_id)
            ),
            selection_docket_id: selection.selection_docket_id.clone(),
            route: selection.route.clone(),
            candidate_segment_bundle_id: selection.candidate_segment_bundle_id.clone(),
            state_scope: selection.state_scope.clone(),
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
            blocked_claims_before: selection.blocked_claims_after.clone(),
            blocked_claims_after: selection.blocked_claims_after.clone(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.state_scope.cmp(&right.state_scope))
            .then(
                left.candidate_segment_bundle_id
                    .cmp(&right.candidate_segment_bundle_id),
            )
    });
    rows
}

pub(crate) fn write_t2_stitched_member_evidence_contract(
    path: &Path,
    rows: &[T2StitchedMemberEvidenceContractRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_stitched_member_evidence_contract_summary(
    output: &Path,
    rows: &[T2StitchedMemberEvidenceContractRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.evidence_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member evidence contract rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

// `t2_stitched_member_evidence_contract_gate_failures` moved to support::tier

pub(crate) fn load_t2_stitched_member_evidence_contract(
    path: &Path,
) -> Result<Vec<T2StitchedMemberEvidenceContractRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_stitched_member_evidence_acquisition_rows` moved to support::tier

pub(crate) fn write_t2_stitched_member_evidence_acquisition(
    path: &Path,
    rows: &[T2StitchedMemberEvidenceAcquisitionRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_stitched_member_evidence_acquisition_summary(
    output: &Path,
    rows: &[T2StitchedMemberEvidenceAcquisitionRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.acquisition_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member evidence acquisition rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

// `t2_stitched_member_evidence_acquisition_gate_failures` moved to support::tier

pub(crate) fn load_t2_stitched_member_evidence_acquisition(
    path: &Path,
) -> Result<Vec<T2StitchedMemberEvidenceAcquisitionRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_stitched_member_source_access_policy_rows` moved to support::tier

pub(crate) fn write_t2_stitched_member_source_access_policy(
    path: &Path,
    rows: &[T2StitchedMemberSourceAccessPolicyRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_stitched_member_source_access_policy_summary(
    output: &Path,
    rows: &[T2StitchedMemberSourceAccessPolicyRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.access_mode.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member source access policy rows to {}",
        rows.len(),
        output.display()
    );
    for (mode, count) in counts {
        println!("  {mode}: {count}");
    }
}

// `t2_stitched_member_source_access_policy_gate_failures` moved to support::tier

pub(crate) fn load_t2_stitched_member_source_access_policy(
    path: &Path,
) -> Result<Vec<T2StitchedMemberSourceAccessPolicyRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_stitched_member_proof_intake_rows(
    access_rows: &[T2StitchedMemberSourceAccessPolicyRow],
) -> Vec<T2StitchedMemberProofIntakeRow> {
    let mut rows = access_rows
        .iter()
        .filter(|row| row.evidence_artifact == "source-needed")
        .map(|access| T2StitchedMemberProofIntakeRow {
            proof_intake_id: format!(
                "T2STITCHEDPROOF-{}",
                stable_id_fragment(&access.access_policy_id)
            ),
            access_policy_id: access.access_policy_id.clone(),
            route: access.route.clone(),
            candidate_segment_bundle_id: access.candidate_segment_bundle_id.clone(),
            state_scope: access.state_scope.clone(),
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
            blocked_claims_before: access.blocked_claims_after.clone(),
            blocked_claims_after: access.blocked_claims_after.clone(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.state_scope.cmp(&right.state_scope))
            .then(
                left.candidate_segment_bundle_id
                    .cmp(&right.candidate_segment_bundle_id),
            )
    });
    rows
}

pub(crate) fn write_t2_stitched_member_proof_intake(
    path: &Path,
    rows: &[T2StitchedMemberProofIntakeRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_stitched_member_proof_intake_summary(
    output: &Path,
    rows: &[T2StitchedMemberProofIntakeRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.proof_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member proof intake rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

// `t2_stitched_member_proof_intake_gate_failures` moved to support::tier

pub(crate) fn load_t2_stitched_member_proof_intake(
    path: &Path,
) -> Result<Vec<T2StitchedMemberProofIntakeRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_stitched_member_proof_source_capture_rows(
    intake_rows: &[T2StitchedMemberProofIntakeRow],
) -> Vec<T2StitchedMemberProofSourceCaptureRow> {
    let mut rows = intake_rows
        .iter()
        .filter(|row| row.proof_artifact == "source-needed")
        .map(|intake| T2StitchedMemberProofSourceCaptureRow {
            source_capture_id: format!(
                "T2STITCHEDSOURCE-{}",
                stable_id_fragment(&intake.proof_intake_id)
            ),
            proof_intake_id: intake.proof_intake_id.clone(),
            route: intake.route.clone(),
            candidate_segment_bundle_id: intake.candidate_segment_bundle_id.clone(),
            state_scope: intake.state_scope.clone(),
            source_artifact_reference: "source-needed".to_string(),
            source_artifact_type: "manual-or-cached-route-geometry".to_string(),
            capture_status: "source-needed".to_string(),
            evidence_acceptance_status: "not-reviewed".to_string(),
            capture_blocker:
                "manual or cached DOT route-geometry source artifact has not been attached"
                    .to_string(),
            blocked_claims_before: intake.blocked_claims_after.clone(),
            blocked_claims_after: intake.blocked_claims_after.clone(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.state_scope.cmp(&right.state_scope))
            .then(
                left.candidate_segment_bundle_id
                    .cmp(&right.candidate_segment_bundle_id),
            )
    });
    rows
}

pub(crate) fn write_t2_stitched_member_proof_source_capture(
    path: &Path,
    rows: &[T2StitchedMemberProofSourceCaptureRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_stitched_member_proof_source_capture_summary(
    output: &Path,
    rows: &[T2StitchedMemberProofSourceCaptureRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.capture_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member proof source-capture rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

// `t2_stitched_member_proof_source_capture_gate_failures` moved to support::tier

pub(crate) fn load_t2_stitched_member_proof_source_capture(
    path: &Path,
) -> Result<Vec<T2StitchedMemberProofSourceCaptureRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_stitched_member_proof_artifact_attachment_rows(
    capture_rows: &[T2StitchedMemberProofSourceCaptureRow],
) -> Vec<T2StitchedMemberProofArtifactAttachmentRow> {
    let mut rows = capture_rows
        .iter()
        .filter(|row| row.source_artifact_reference == "source-needed")
        .map(|capture| T2StitchedMemberProofArtifactAttachmentRow {
            artifact_attachment_id: format!(
                "T2STITCHEDATTACH-{}",
                stable_id_fragment(&capture.source_capture_id)
            ),
            source_capture_id: capture.source_capture_id.clone(),
            route: capture.route.clone(),
            candidate_segment_bundle_id: capture.candidate_segment_bundle_id.clone(),
            state_scope: capture.state_scope.clone(),
            source_artifact_reference: "source-needed".to_string(),
            attachment_status: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            attachment_blocker:
                "manual or cached DOT route-geometry artifact reference has not been attached"
                    .to_string(),
            blocked_claims_before: capture.blocked_claims_after.clone(),
            blocked_claims_after: capture.blocked_claims_after.clone(),
            blocker_delta: 0,
            next_artifact: "data/national-segment-registry.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.state_scope.cmp(&right.state_scope))
            .then(
                left.candidate_segment_bundle_id
                    .cmp(&right.candidate_segment_bundle_id),
            )
    });
    rows
}

pub(crate) fn write_t2_stitched_member_proof_artifact_attachment(
    path: &Path,
    rows: &[T2StitchedMemberProofArtifactAttachmentRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_stitched_member_proof_artifact_attachment_summary(
    output: &Path,
    rows: &[T2StitchedMemberProofArtifactAttachmentRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.attachment_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member proof artifact-attachment rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

// `t2_stitched_member_proof_artifact_attachment_gate_failures` moved to support::tier

pub(crate) fn load_t2_stitched_member_proof_artifact_attachment(
    path: &Path,
) -> Result<Vec<T2StitchedMemberProofArtifactAttachmentRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_stitched_member_proof_review_docket_rows(
    attachment_rows: &[T2StitchedMemberProofArtifactAttachmentRow],
) -> Vec<T2StitchedMemberProofReviewDocketRow> {
    let mut rows = attachment_rows
        .iter()
        .filter(|row| row.source_artifact_reference == "source-needed")
        .map(|attachment| T2StitchedMemberProofReviewDocketRow {
            proof_review_id: format!(
                "T2STITCHEDREVIEW-{}",
                stable_id_fragment(&attachment.artifact_attachment_id)
            ),
            artifact_attachment_id: attachment.artifact_attachment_id.clone(),
            route: attachment.route.clone(),
            candidate_segment_bundle_id: attachment.candidate_segment_bundle_id.clone(),
            state_scope: attachment.state_scope.clone(),
            source_artifact_reference: attachment.source_artifact_reference.clone(),
            review_decision: "held-no-source-artifact".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            candidate_disposition_status: "not-ready-for-disposition".to_string(),
            optimization_return_status: "return-to-optimizer-held-known".to_string(),
            review_reason:
                "artifact attachment remains source-needed; proof review cannot accept continuity evidence"
                    .to_string(),
            blocked_claims_before: attachment.blocked_claims_after.clone(),
            blocked_claims_after: attachment.blocked_claims_after.clone(),
            blocker_delta: 0,
            next_artifact: "data/tier-optimizer-runs.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.route
            .cmp(&right.route)
            .then(left.state_scope.cmp(&right.state_scope))
            .then(
                left.candidate_segment_bundle_id
                    .cmp(&right.candidate_segment_bundle_id),
            )
    });
    rows
}

pub(crate) fn write_t2_stitched_member_proof_review_docket(
    path: &Path,
    rows: &[T2StitchedMemberProofReviewDocketRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_stitched_member_proof_review_docket_summary(
    output: &Path,
    rows: &[T2StitchedMemberProofReviewDocketRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.review_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 stitched member proof review rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

// `t2_stitched_member_proof_review_docket_gate_failures` moved to support::tier

// `t2_bundle_overlay_repair_delta_rows` moved to support::tier

pub(crate) fn write_t2_bundle_overlay_repair_delta(
    path: &Path,
    rows: &[T2BundleOverlayRepairDeltaRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_bundle_overlay_repair_delta_summary(
    output: &Path,
    rows: &[T2BundleOverlayRepairDeltaRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.replay_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 bundle overlay repair delta rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

// `t2_bundle_overlay_repair_delta_gate_failures` moved to support::tier

// `t2_overlay_optimizer_action_docket_rows` moved to support::tier

pub(crate) fn write_t2_overlay_optimizer_action_docket(
    path: &Path,
    rows: &[T2OverlayOptimizerActionDocketRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_overlay_optimizer_action_docket_summary(
    output: &Path,
    rows: &[T2OverlayOptimizerActionDocketRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.optimizer_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 overlay optimizer action rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

// `t2_overlay_optimizer_action_docket_gate_failures` moved to support::tier

pub(crate) fn load_t2_overlay_optimizer_action_docket(
    path: &Path,
) -> Result<Vec<T2OverlayOptimizerActionDocketRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_overlay_p1_structural_readiness_review_rows` moved to support::tier

pub(crate) fn write_t2_overlay_p1_structural_readiness_review(
    path: &Path,
    rows: &[T2OverlayP1StructuralReadinessReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_overlay_p1_structural_readiness_review_summary(
    output: &Path,
    rows: &[T2OverlayP1StructuralReadinessReviewRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.readiness_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 overlay P1 structural-readiness review rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

// `t2_overlay_p1_structural_readiness_review_gate_failures` moved to support::tier

pub(crate) fn t2_overlay_p2_service_overlay_review_rows(
    action_rows: &[T2OverlayOptimizerActionDocketRow],
) -> Vec<T2OverlayP2ServiceOverlayReviewRow> {
    let mut rows = action_rows
        .iter()
        .filter(|row| row.priority_class == "P2-service-overlay")
        .map(|action| T2OverlayP2ServiceOverlayReviewRow {
            p2_review_id: format!("T2OVERLAYP2-{}", stable_id_fragment(&action.action_id)),
            action_id: action.action_id.clone(),
            route: action.route.clone(),
            segment_bundle_id: action.segment_bundle_id.clone(),
            optimizer_action: action.optimizer_action.clone(),
            priority_class: action.priority_class.clone(),
            service_overlay_decision: "held-service-overlay-diagnostic-needed".to_string(),
            service_overlay_reason:
                "service overlay remains diagnostic-only; no sourced evidence supports blocker reduction"
                    .to_string(),
            downstream_action: "route-to-service-overlay-diagnostic-review".to_string(),
            action_status: "optimizer-held-known".to_string(),
            qualification_effects: action.qualification_effects.clone(),
            blocked_claims_before: action.blocked_claims_after.clone(),
            blocked_claims_after: action.blocked_claims_after.clone(),
            blocker_delta: 0,
            next_artifact: action.next_artifact.clone(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t2_overlay_p2_service_overlay_review(
    path: &Path,
    rows: &[T2OverlayP2ServiceOverlayReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_overlay_p2_service_overlay_review_summary(
    output: &Path,
    rows: &[T2OverlayP2ServiceOverlayReviewRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts
            .entry(row.service_overlay_decision.as_str())
            .or_default() += 1;
    }
    println!(
        "  wrote {} T2 overlay P2 service-overlay review rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

// `t2_overlay_p2_service_overlay_review_gate_failures` moved to support::tier

pub(crate) fn t2_overlay_p3_local_zone_overlay_review_rows(
    action_rows: &[T2OverlayOptimizerActionDocketRow],
) -> Vec<T2OverlayP3LocalZoneOverlayReviewRow> {
    let mut rows = action_rows
        .iter()
        .filter(|row| row.priority_class == "P3-local-zone-overlay")
        .map(|action| T2OverlayP3LocalZoneOverlayReviewRow {
            p3_review_id: format!("T2OVERLAYP3-{}", stable_id_fragment(&action.action_id)),
            action_id: action.action_id.clone(),
            route: action.route.clone(),
            segment_bundle_id: action.segment_bundle_id.clone(),
            optimizer_action: action.optimizer_action.clone(),
            priority_class: action.priority_class.clone(),
            local_zone_decision: "held-local-zone-overlay-review-needed".to_string(),
            local_zone_reason:
                "local relief remains below national game overlay; no sourced evidence supports blocker reduction"
                    .to_string(),
            downstream_action: "route-to-local-zone-overlay-review".to_string(),
            action_status: "optimizer-held-known".to_string(),
            qualification_effects: action.qualification_effects.clone(),
            blocked_claims_before: action.blocked_claims_after.clone(),
            blocked_claims_after: action.blocked_claims_after.clone(),
            blocker_delta: 0,
            next_artifact: action.next_artifact.clone(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t2_overlay_p3_local_zone_overlay_review(
    path: &Path,
    rows: &[T2OverlayP3LocalZoneOverlayReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_overlay_p3_local_zone_overlay_review_summary(
    output: &Path,
    rows: &[T2OverlayP3LocalZoneOverlayReviewRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.local_zone_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 overlay P3 local-zone overlay review rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

// `t2_overlay_p3_local_zone_overlay_review_gate_failures` moved to support::tier

#[derive(Debug, Clone, Default)]
struct OptimizerConstraintBudgetIndex {
    by_bundle: std::collections::HashMap<String, OptimizerConstraintBudgetRow>,
    by_route: std::collections::HashMap<String, OptimizerConstraintBudgetRollup>,
}

#[derive(Debug, Clone, Default)]
struct OptimizerConstraintBudgetRollup {
    hard_blocker_count: usize,
    claim_blocker_count: usize,
    constraint_debt_cost_m: f64,
    lifecycle_debt_cost_m: f64,
    constraint_penalty_score: f64,
    top_constraint_classes: std::collections::BTreeSet<String>,
    qualification_effects: std::collections::BTreeSet<String>,
    constraint_ledger_artifact: String,
}

pub(crate) fn optimizer_constraint_budget_index(
    rows: &[OptimizerConstraintBudgetRow],
) -> OptimizerConstraintBudgetIndex {
    let mut index = OptimizerConstraintBudgetIndex::default();
    for row in rows {
        if !row.segment_bundle_id.trim().is_empty() {
            index
                .by_bundle
                .insert(row.segment_bundle_id.clone(), row.clone());
        }
        if !row.route.trim().is_empty() {
            let rollup = index
                .by_route
                .entry(canonical_route_key(&row.route))
                .or_default();
            rollup.hard_blocker_count += row.hard_blocker_count;
            rollup.claim_blocker_count += row.claim_blocker_count;
            rollup.constraint_debt_cost_m =
                round_cost_m(rollup.constraint_debt_cost_m + row.constraint_debt_cost_m);
            rollup.lifecycle_debt_cost_m =
                round_cost_m(rollup.lifecycle_debt_cost_m + row.lifecycle_debt_cost_m);
            rollup.constraint_penalty_score =
                round_cost_m(rollup.constraint_penalty_score + row.constraint_penalty_score);
            for class in row.top_constraint_classes.split('|').map(str::trim) {
                if !class.is_empty() {
                    rollup.top_constraint_classes.insert(class.to_string());
                }
            }
            for effect in row.qualification_effects.split('|').map(str::trim) {
                if !effect.is_empty() {
                    rollup.qualification_effects.insert(effect.to_string());
                }
            }
            if rollup.constraint_ledger_artifact.is_empty() {
                rollup.constraint_ledger_artifact = row.constraint_ledger_artifact.clone();
            }
        }
    }
    index
}

pub(crate) fn constraint_budget_for_candidate(
    route: &str,
    segment_bundle_id: &str,
    index: &OptimizerConstraintBudgetIndex,
) -> (usize, usize, f64, f64, f64, String, String, String) {
    if let Some(row) = index.by_bundle.get(segment_bundle_id) {
        return (
            row.hard_blocker_count,
            row.claim_blocker_count,
            row.constraint_debt_cost_m,
            row.lifecycle_debt_cost_m,
            row.constraint_penalty_score,
            row.top_constraint_classes.clone(),
            row.qualification_effects.clone(),
            row.constraint_ledger_artifact.clone(),
        );
    }
    if let Some(rollup) = index.by_route.get(&canonical_route_key(route)) {
        return (
            rollup.hard_blocker_count,
            rollup.claim_blocker_count,
            rollup.constraint_debt_cost_m,
            rollup.lifecycle_debt_cost_m,
            rollup.constraint_penalty_score,
            join_string_set(&rollup.top_constraint_classes),
            join_pipe_set(&rollup.qualification_effects),
            rollup.constraint_ledger_artifact.clone(),
        );
    }
    (
        0,
        0,
        0.0,
        0.0,
        0.0,
        "none".to_string(),
        String::new(),
        String::new(),
    )
}

#[derive(Debug, Default)]
struct OptimizerConstraintBudgetBuilder {
    optimizer_run_id: String,
    tier: String,
    region_id: String,
    subject_scope: String,
    subject_id: String,
    segment_bundle_id: String,
    route: String,
    ledger_row_count: usize,
    hard_blocker_count: usize,
    claim_blocker_count: usize,
    review_count: usize,
    budget_debt_count: usize,
    constraint_debt_cost_m: f64,
    lifecycle_debt_cost_m: f64,
    constraint_penalty_score: f64,
    class_counts: std::collections::BTreeMap<String, usize>,
    blocking_claims: std::collections::BTreeSet<String>,
    qualification_effects: std::collections::BTreeSet<String>,
    next_artifacts: std::collections::BTreeSet<String>,
}

// `optimizer_constraint_budget_rows` moved to support::optimizer

pub(crate) fn insert_optimizer_qualification_effects(
    target: &mut std::collections::BTreeSet<String>,
    optimizer_effect: &str,
) {
    for part in optimizer_effect.split(';').map(str::trim) {
        if let Some(effects) = part.strip_prefix("qualification_effects=") {
            insert_pipe_values(target, effects);
        } else if part.starts_with("qualification_gate_policy=")
            || part.starts_with("qualification_game_use=")
        {
            target.insert(part.to_string());
        }
    }
}

pub(crate) fn optimizer_constraint_budget_subject(row: &OptimizerConstraintLedgerRow) -> (String, String) {
    if !row.segment_bundle_id.trim().is_empty() {
        ("bundle".to_string(), row.segment_bundle_id.clone())
    } else if !row.route.trim().is_empty() {
        ("route".to_string(), row.route.clone())
    } else {
        (row.constraint_scope.clone(), row.subject_id.clone())
    }
}

pub(crate) fn top_constraint_classes(class_counts: &std::collections::BTreeMap<String, usize>) -> String {
    let mut classes = class_counts.iter().collect::<Vec<_>>();
    classes.sort_by(|left, right| right.1.cmp(left.1).then_with(|| left.0.cmp(right.0)));
    classes
        .into_iter()
        .take(3)
        .map(|(class, _)| class.as_str())
        .collect::<Vec<_>>()
        .join("|")
}

pub(crate) fn write_optimizer_constraint_budget(
    path: &Path,
    rows: &[OptimizerConstraintBudgetRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

// `print_optimizer_constraint_budget_summary` moved to support::print

// `optimizer_constraint_budget_gate_failures` moved to support::gates

#[derive(Debug, Clone, Default)]
struct ResidualBacklogBuilder {
    priority_class: String,
    blocker_family: String,
    tier: String,
    blocked_claims: std::collections::BTreeSet<String>,
    subject_ids: std::collections::BTreeSet<String>,
    routes: std::collections::BTreeSet<String>,
    total_hard_blockers: usize,
    total_claim_blockers: usize,
    total_budget_debt_count: usize,
    total_constraint_debt_cost_m: f64,
    total_constraint_penalty_score: f64,
    next_artifacts: std::collections::BTreeSet<String>,
    next_wave: String,
}

// `optimizer_residual_blocker_backlog_rows` moved to support::optimizer

// `optimizer_backlog_family` moved to support::optimizer

pub(crate) fn join_limited_set(values: &std::collections::BTreeSet<String>, limit: usize) -> String {
    values
        .iter()
        .take(limit)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(";")
}

pub(crate) fn write_optimizer_residual_blocker_backlog(
    path: &Path,
    rows: &[OptimizerResidualBlockerBacklogRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

// `print_optimizer_residual_blocker_backlog_summary` moved to support::print

// `optimizer_residual_blocker_backlog_gate_failures` moved to support::gates

pub(crate) fn load_optimizer_residual_blocker_backlog(
    path: &Path,
) -> Result<Vec<OptimizerResidualBlockerBacklogRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn optimizer_claim_review_rows(
    backlog_rows: &[OptimizerResidualBlockerBacklogRow],
) -> Vec<OptimizerClaimReviewRow> {
    let mut rows = backlog_rows
        .iter()
        .filter(|row| {
            row.priority_class == "P1-claim-blocker"
                && row.next_wave == "optimizer-claim-review"
                && row.total_claim_blockers > 0
        })
        .map(|row| OptimizerClaimReviewRow {
            claim_review_id: format!("OCR-{}", stable_id_fragment(&row.backlog_id)),
            backlog_id: row.backlog_id.clone(),
            priority_class: row.priority_class.clone(),
            blocker_family: row.blocker_family.clone(),
            tier: row.tier.clone(),
            blocked_claims: row.blocked_claims.clone(),
            subject_count: row.subject_count,
            route_count: row.route_count,
            total_claim_blockers: row.total_claim_blockers,
            representative_routes: row.representative_routes.clone(),
            representative_subjects: row.representative_subjects.clone(),
            evidence_artifacts: row.next_artifacts.clone(),
            review_decision: "held-for-source-specific-claim-review".to_string(),
            blocker_claims_before: row.blocked_claims.clone(),
            blocker_claims_after: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_artifact: row.next_artifacts.clone(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| {
        left.tier
            .cmp(&right.tier)
            .then_with(|| left.blocker_family.cmp(&right.blocker_family))
    });
    rows
}

pub(crate) fn write_optimizer_claim_review(path: &Path, rows: &[OptimizerClaimReviewRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_optimizer_claim_review_summary(output: &Path, rows: &[OptimizerClaimReviewRow]) {
    let blockers = rows
        .iter()
        .map(|row| row.total_claim_blockers)
        .sum::<usize>();
    println!(
        "  wrote {} optimizer claim review rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `optimizer_claim_review_gate_failures` moved to support::gates

pub(crate) fn load_optimizer_claim_review(path: &Path) -> Result<Vec<OptimizerClaimReviewRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_game_publication_evidence_review_rows(
    claim_rows: &[OptimizerClaimReviewRow],
    hook_rows: &[T2ScenarioHookRow],
) -> Vec<T2GamePublicationEvidenceReviewRow> {
    let Some(claim_row) = claim_rows.iter().find(|row| {
        row.priority_class == "P1-claim-blocker"
            && row.tier == "T2"
            && row.blocker_family == "game_ops_publication_readiness"
            && row.total_claim_blockers > 0
    }) else {
        return Vec::new();
    };
    let expected_scenarios = claim_row
        .representative_subjects
        .split(';')
        .filter(|scenario| !scenario.trim().is_empty())
        .map(str::to_string)
        .collect::<std::collections::BTreeSet<_>>();
    let mut rows = hook_rows
        .iter()
        .filter(|row| expected_scenarios.contains(row.scenario_id.as_str()))
        .map(|row| T2GamePublicationEvidenceReviewRow {
            game_review_id: format!("T2GAMEPUB-{}", stable_id_fragment(&row.scenario_id)),
            claim_review_id: claim_row.claim_review_id.clone(),
            scenario_id: row.scenario_id.clone(),
            service_class: row.service_class.clone(),
            t2_map_id: row.t2_map_id.clone(),
            player_decision: row.player_decision.clone(),
            evidence_hold: row.evidence_hold.clone(),
            review_decision: "publication-evidence-policy-required".to_string(),
            blocker_claims_before: claim_row.blocked_claims.clone(),
            blocker_claims_after: claim_row.blocked_claims.clone(),
            blocker_count_before: 1,
            blocker_count_after: 1,
            claim_blocker_delta: 0,
            required_evidence: t2_game_publication_required_evidence(&row.evidence_hold)
                .to_string(),
            next_artifact: "data/t2-game-publication-evidence-policy.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.scenario_id.cmp(&right.scenario_id));
    rows
}

pub(crate) fn t2_game_publication_required_evidence(evidence_hold: &str) -> &'static str {
    let hold = evidence_hold.to_ascii_lowercase();
    if hold.contains("port") || hold.contains("flood") {
        "port-surge-demand-and-flood-closure-evidence"
    } else if hold.contains("managed-lane") || hold.contains("spillback") {
        "managed-lane-merge-and-spillback-validation"
    } else {
        "standards-proof-and-scenario-promotion-record"
    }
}

pub(crate) fn write_t2_game_publication_evidence_review(
    path: &Path,
    rows: &[T2GamePublicationEvidenceReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_game_publication_evidence_review_summary(
    output: &Path,
    rows: &[T2GamePublicationEvidenceReviewRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 game publication evidence review rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t2_game_publication_evidence_review_gate_failures` moved to support::tier

pub(crate) fn load_t2_game_publication_evidence_review(
    path: &Path,
) -> Result<Vec<T2GamePublicationEvidenceReviewRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_game_publication_evidence_policy_rows(
    review_rows: &[T2GamePublicationEvidenceReviewRow],
) -> Vec<T2GamePublicationEvidencePolicyRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| {
            row.review_decision == "publication-evidence-policy-required"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2GamePublicationEvidencePolicyRow {
            policy_id: format!("T2GAMEPOLICY-{}", stable_id_fragment(&row.scenario_id)),
            game_review_id: row.game_review_id.clone(),
            scenario_id: row.scenario_id.clone(),
            service_class: row.service_class.clone(),
            t2_map_id: row.t2_map_id.clone(),
            evidence_policy_basis: row.evidence_hold.clone(),
            required_evidence: row.required_evidence.clone(),
            evidence_policy_decision: "publication-evidence-policy-authored-review".to_string(),
            policy_treatment: t2_game_publication_policy_treatment(&row.required_evidence)
                .to_string(),
            publication_treatment:
                "hold game publication until accepted evidence policy is replayed".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-game-publication-evidence-policy-acceptance.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.scenario_id.cmp(&right.scenario_id));
    rows
}

pub(crate) fn t2_game_publication_policy_treatment(required_evidence: &str) -> &'static str {
    match required_evidence {
        "port-surge-demand-and-flood-closure-evidence" => {
            "require port surge demand evidence and flood closure source before scenario publication"
        }
        "managed-lane-merge-and-spillback-validation" => {
            "require managed-lane merge and spillback validation before scenario publication"
        }
        _ => "require standards proof and scenario promotion record before scenario publication",
    }
}

pub(crate) fn write_t2_game_publication_evidence_policy(
    path: &Path,
    rows: &[T2GamePublicationEvidencePolicyRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_game_publication_evidence_policy_summary(
    output: &Path,
    rows: &[T2GamePublicationEvidencePolicyRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 game publication evidence policy rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t2_game_publication_evidence_policy_gate_failures` moved to support::tier

pub(crate) fn load_t2_game_publication_evidence_policy(
    path: &Path,
) -> Result<Vec<T2GamePublicationEvidencePolicyRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_game_publication_evidence_policy_acceptance_rows(
    policy_rows: &[T2GamePublicationEvidencePolicyRow],
) -> Vec<T2GamePublicationEvidencePolicyAcceptanceRow> {
    let mut rows = policy_rows
        .iter()
        .filter(|row| {
            row.evidence_policy_decision == "publication-evidence-policy-authored-review"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2GamePublicationEvidencePolicyAcceptanceRow {
            acceptance_id: format!("T2GAMEACCEPT-{}", stable_id_fragment(&row.scenario_id)),
            policy_id: row.policy_id.clone(),
            scenario_id: row.scenario_id.clone(),
            service_class: row.service_class.clone(),
            t2_map_id: row.t2_map_id.clone(),
            accepted_required_evidence: row.required_evidence.clone(),
            accepted_policy_treatment: row.policy_treatment.clone(),
            acceptance_decision: "publication-evidence-policy-accepted".to_string(),
            publication_treatment: row.publication_treatment.clone(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-game-publication-evidence-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.scenario_id.cmp(&right.scenario_id));
    rows
}

pub(crate) fn write_t2_game_publication_evidence_policy_acceptance(
    path: &Path,
    rows: &[T2GamePublicationEvidencePolicyAcceptanceRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_game_publication_evidence_policy_acceptance_summary(
    output: &Path,
    rows: &[T2GamePublicationEvidencePolicyAcceptanceRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 game publication evidence policy acceptance rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t2_game_publication_evidence_policy_acceptance_gate_failures` moved to support::tier

pub(crate) fn load_t2_game_publication_evidence_policy_acceptance(
    path: &Path,
) -> Result<Vec<T2GamePublicationEvidencePolicyAcceptanceRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_game_publication_evidence_blocker_relief_rows(
    acceptance_rows: &[T2GamePublicationEvidencePolicyAcceptanceRow],
) -> Vec<T2GamePublicationEvidenceBlockerReliefRow> {
    let mut rows = acceptance_rows
        .iter()
        .filter(|row| {
            row.acceptance_decision == "publication-evidence-policy-accepted"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2GamePublicationEvidenceBlockerReliefRow {
            relief_id: format!("T2GAMERELIEF-{}", stable_id_fragment(&row.scenario_id)),
            acceptance_id: row.acceptance_id.clone(),
            policy_id: row.policy_id.clone(),
            scenario_id: row.scenario_id.clone(),
            service_class: row.service_class.clone(),
            accepted_required_evidence: row.accepted_required_evidence.clone(),
            relief_decision: "relief-ready-for-constraint-ledger-replay".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: String::new(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: 0,
            claim_blocker_delta: -(row.blocker_count_after as isize),
            ledger_replay_status: "pending-optimizer-constraint-ledger-replay".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.scenario_id.cmp(&right.scenario_id));
    rows
}

pub(crate) fn write_t2_game_publication_evidence_blocker_relief(
    path: &Path,
    rows: &[T2GamePublicationEvidenceBlockerReliefRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_game_publication_evidence_blocker_relief_summary(
    output: &Path,
    rows: &[T2GamePublicationEvidenceBlockerReliefRow],
) {
    let before = rows
        .iter()
        .map(|row| row.blocker_count_before)
        .sum::<usize>();
    let after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 game publication evidence blocker relief rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers before: {before}");
    println!("  claim blockers after: {after}");
}

// `t2_game_publication_evidence_blocker_relief_gate_failures` moved to support::tier

// `t1_schematic_geometry_claim_review_rows` moved to support::tier

pub(crate) fn write_t1_schematic_geometry_claim_review(
    path: &Path,
    rows: &[T1SchematicGeometryClaimReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t1_schematic_geometry_claim_review_summary(
    output: &Path,
    rows: &[T1SchematicGeometryClaimReviewRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T1 schematic-geometry claim review rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t1_schematic_geometry_claim_review_gate_failures` moved to support::tier::t1_schematic_geometry_claim_review_gate_failures

pub(crate) fn load_t1_schematic_geometry_claim_review(
    path: &Path,
) -> Result<Vec<T1SchematicGeometryClaimReviewRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_beck_transfer_complexity_review_rows` moved to support::tier

pub(crate) fn write_t2_beck_transfer_complexity_review(
    path: &Path,
    rows: &[T2BeckTransferComplexityReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_beck_transfer_complexity_review_summary(
    output: &Path,
    rows: &[T2BeckTransferComplexityReviewRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 Beck transfer-complexity review rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t2_beck_transfer_complexity_review_gate_failures` moved to support::tier

pub(crate) fn load_t2_beck_transfer_complexity_review(
    path: &Path,
) -> Result<Vec<T2BeckTransferComplexityReviewRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_beck_label_density_review_rows` moved to support::tier

pub(crate) fn write_t2_beck_label_density_review(
    path: &Path,
    rows: &[T2BeckLabelDensityReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_beck_label_density_review_summary(output: &Path, rows: &[T2BeckLabelDensityReviewRow]) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 Beck label-density review rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t2_beck_label_density_review_gate_failures` moved to support::tier::t2_beck_label_density_review_gate_failures

pub(crate) fn load_t2_beck_label_density_review(path: &Path) -> Result<Vec<T2BeckLabelDensityReviewRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t2_beck_long_connector_review_rows` moved to support::tier

pub(crate) fn write_t2_beck_long_connector_review(
    path: &Path,
    rows: &[T2BeckLongConnectorReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_beck_long_connector_review_summary(
    output: &Path,
    rows: &[T2BeckLongConnectorReviewRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 Beck long-connector review rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t2_beck_long_connector_review_gate_failures` moved to support::tier

pub(crate) fn load_t2_beck_long_connector_review(path: &Path) -> Result<Vec<T2BeckLongConnectorReviewRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_long_connector_band(schematic_length_px: f64) -> &'static str {
    if schematic_length_px >= 1200.0 {
        "severe-long-connector"
    } else if schematic_length_px >= 900.0 {
        "high-long-connector"
    } else {
        "moderate-long-connector"
    }
}

pub(crate) fn t2_beck_long_connector_policy_rows(
    review_rows: &[T2BeckLongConnectorReviewRow],
) -> Vec<T2BeckLongConnectorPolicyRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| {
            row.review_decision == "long-connector-policy-required"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2BeckLongConnectorPolicyRow {
            policy_id: format!("T2LONGPOLICY-{}", stable_id_fragment(&row.route)),
            connector_review_id: row.connector_review_id.clone(),
            route: row.route.clone(),
            trunk_pair: format!("{}-{}", row.start_trunk, row.end_trunk),
            service_class: row.service_class.clone(),
            schematic_length_px: row.schematic_length_px,
            connector_band: t2_long_connector_band(row.schematic_length_px).to_string(),
            policy_basis: row.connector_basis.clone(),
            connector_policy_decision: "long-connector-policy-authored-review".to_string(),
            render_treatment:
                "preserve connector service but require trunk-interface labeling and explicit local-service beads"
                    .to_string(),
            promotion_treatment:
                "hold map promotion until accepted long-connector treatment is replayed"
                    .to_string(),
            publication_status: "held-pending-policy-acceptance".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-beck-long-connector-policy-acceptance.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t2_beck_long_connector_policy(
    path: &Path,
    rows: &[T2BeckLongConnectorPolicyRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_beck_long_connector_policy_summary(
    output: &Path,
    rows: &[T2BeckLongConnectorPolicyRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 Beck long-connector policy rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t2_beck_long_connector_policy_gate_failures` moved to support::tier

pub(crate) fn load_t2_beck_long_connector_policy(path: &Path) -> Result<Vec<T2BeckLongConnectorPolicyRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_beck_long_connector_policy_acceptance_rows(
    policy_rows: &[T2BeckLongConnectorPolicyRow],
) -> Vec<T2BeckLongConnectorPolicyAcceptanceRow> {
    let mut rows = policy_rows
        .iter()
        .filter(|row| {
            row.connector_policy_decision == "long-connector-policy-authored-review"
                && row.publication_status == "held-pending-policy-acceptance"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2BeckLongConnectorPolicyAcceptanceRow {
            acceptance_id: format!("T2LONGACCEPT-{}", stable_id_fragment(&row.route)),
            policy_id: row.policy_id.clone(),
            route: row.route.clone(),
            connector_band: row.connector_band.clone(),
            accepted_render_treatment: row.render_treatment.clone(),
            accepted_promotion_treatment: row.promotion_treatment.clone(),
            acceptance_decision: "long-connector-policy-accepted".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-beck-long-connector-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t2_beck_long_connector_policy_acceptance(
    path: &Path,
    rows: &[T2BeckLongConnectorPolicyAcceptanceRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_beck_long_connector_policy_acceptance_summary(
    output: &Path,
    rows: &[T2BeckLongConnectorPolicyAcceptanceRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 Beck long-connector policy acceptance rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t2_beck_long_connector_policy_acceptance_gate_failures` moved to support::tier

pub(crate) fn load_t2_beck_long_connector_policy_acceptance(
    path: &Path,
) -> Result<Vec<T2BeckLongConnectorPolicyAcceptanceRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_beck_long_connector_blocker_relief_rows(
    acceptance_rows: &[T2BeckLongConnectorPolicyAcceptanceRow],
) -> Vec<T2BeckLongConnectorBlockerReliefRow> {
    let mut rows = acceptance_rows
        .iter()
        .filter(|row| {
            row.acceptance_decision == "long-connector-policy-accepted"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2BeckLongConnectorBlockerReliefRow {
            relief_id: format!("T2LONGRELIEF-{}", stable_id_fragment(&row.route)),
            acceptance_id: row.acceptance_id.clone(),
            policy_id: row.policy_id.clone(),
            route: row.route.clone(),
            connector_band: row.connector_band.clone(),
            accepted_render_treatment: row.accepted_render_treatment.clone(),
            relief_decision: "relief-ready-for-constraint-ledger-replay".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: String::new(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: 0,
            claim_blocker_delta: -(row.blocker_count_after as isize),
            ledger_replay_status: "pending-optimizer-constraint-ledger-replay".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t2_beck_long_connector_blocker_relief(
    path: &Path,
    rows: &[T2BeckLongConnectorBlockerReliefRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_beck_long_connector_blocker_relief_summary(
    output: &Path,
    rows: &[T2BeckLongConnectorBlockerReliefRow],
) {
    let before = rows
        .iter()
        .map(|row| row.blocker_count_before)
        .sum::<usize>();
    let after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 Beck long-connector blocker relief rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers before: {before}");
    println!("  claim blockers after: {after}");
}

// `t2_beck_long_connector_blocker_relief_gate_failures` moved to support::tier

pub(crate) fn t2_label_density_band(label_density_per_100px: f64) -> &'static str {
    if label_density_per_100px >= 1.25 {
        "severe-label-density"
    } else if label_density_per_100px >= 1.10 {
        "high-label-density"
    } else {
        "moderate-label-density"
    }
}

pub(crate) fn t2_beck_label_density_policy_rows(
    review_rows: &[T2BeckLabelDensityReviewRow],
) -> Vec<T2BeckLabelDensityPolicyRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| {
            row.review_decision == "label-density-policy-required"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2BeckLabelDensityPolicyRow {
            policy_id: format!("T2LABELPOLICY-{}", stable_id_fragment(&row.route)),
            label_review_id: row.label_review_id.clone(),
            route: row.route.clone(),
            trunk_pair: format!("{}-{}", row.start_trunk, row.end_trunk),
            service_class: row.service_class.clone(),
            label_density_per_100px: row.label_density_per_100px,
            density_band: t2_label_density_band(row.label_density_per_100px).to_string(),
            policy_basis: row.density_basis.clone(),
            label_policy_decision: "label-density-policy-authored-review".to_string(),
            render_treatment:
                "compress labels to trunk interfaces and preserve intermediate stops as unlabeled service beads"
                    .to_string(),
            promotion_treatment:
                "hold map promotion until accepted label-density simplification is replayed"
                    .to_string(),
            publication_status: "held-pending-policy-acceptance".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-beck-label-density-policy-acceptance.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t2_beck_label_density_policy(
    path: &Path,
    rows: &[T2BeckLabelDensityPolicyRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_beck_label_density_policy_summary(output: &Path, rows: &[T2BeckLabelDensityPolicyRow]) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 Beck label-density policy rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t2_beck_label_density_policy_gate_failures` moved to support::tier

pub(crate) fn load_t2_beck_label_density_policy(path: &Path) -> Result<Vec<T2BeckLabelDensityPolicyRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_beck_label_density_policy_acceptance_rows(
    policy_rows: &[T2BeckLabelDensityPolicyRow],
) -> Vec<T2BeckLabelDensityPolicyAcceptanceRow> {
    let mut rows = policy_rows
        .iter()
        .filter(|row| {
            row.label_policy_decision == "label-density-policy-authored-review"
                && row.publication_status == "held-pending-policy-acceptance"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2BeckLabelDensityPolicyAcceptanceRow {
            acceptance_id: format!("T2LABELACCEPT-{}", stable_id_fragment(&row.route)),
            policy_id: row.policy_id.clone(),
            route: row.route.clone(),
            density_band: row.density_band.clone(),
            accepted_render_treatment: row.render_treatment.clone(),
            accepted_promotion_treatment: row.promotion_treatment.clone(),
            acceptance_decision: "label-density-policy-accepted".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-beck-label-density-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t2_beck_label_density_policy_acceptance(
    path: &Path,
    rows: &[T2BeckLabelDensityPolicyAcceptanceRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_beck_label_density_policy_acceptance_summary(
    output: &Path,
    rows: &[T2BeckLabelDensityPolicyAcceptanceRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 Beck label-density policy acceptance rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t2_beck_label_density_policy_acceptance_gate_failures` moved to support::tier

pub(crate) fn load_t2_beck_label_density_policy_acceptance(
    path: &Path,
) -> Result<Vec<T2BeckLabelDensityPolicyAcceptanceRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_beck_label_density_blocker_relief_rows(
    acceptance_rows: &[T2BeckLabelDensityPolicyAcceptanceRow],
) -> Vec<T2BeckLabelDensityBlockerReliefRow> {
    let mut rows = acceptance_rows
        .iter()
        .filter(|row| {
            row.acceptance_decision == "label-density-policy-accepted"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2BeckLabelDensityBlockerReliefRow {
            relief_id: format!("T2LABELRELIEF-{}", stable_id_fragment(&row.route)),
            acceptance_id: row.acceptance_id.clone(),
            policy_id: row.policy_id.clone(),
            route: row.route.clone(),
            density_band: row.density_band.clone(),
            accepted_render_treatment: row.accepted_render_treatment.clone(),
            relief_decision: "relief-ready-for-constraint-ledger-replay".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: String::new(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: 0,
            claim_blocker_delta: -(row.blocker_count_after as isize),
            ledger_replay_status: "pending-optimizer-constraint-ledger-replay".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t2_beck_label_density_blocker_relief(
    path: &Path,
    rows: &[T2BeckLabelDensityBlockerReliefRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_beck_label_density_blocker_relief_summary(
    output: &Path,
    rows: &[T2BeckLabelDensityBlockerReliefRow],
) {
    let before = rows
        .iter()
        .map(|row| row.blocker_count_before)
        .sum::<usize>();
    let after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 Beck label-density blocker relief rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers before: {before}");
    println!("  claim blockers after: {after}");
}

// `t2_beck_label_density_blocker_relief_gate_failures` moved to support::tier

pub(crate) fn t2_transfer_complexity_band(transfer_stop_count: usize) -> &'static str {
    if transfer_stop_count >= 7 {
        "severe-transfer-complexity"
    } else if transfer_stop_count >= 6 {
        "high-transfer-complexity"
    } else {
        "moderate-transfer-complexity"
    }
}

// `t2_beck_transfer_complexity_policy_rows` moved to support::tier

pub(crate) fn write_t2_beck_transfer_complexity_policy(
    path: &Path,
    rows: &[T2BeckTransferComplexityPolicyRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_beck_transfer_complexity_policy_summary(
    output: &Path,
    rows: &[T2BeckTransferComplexityPolicyRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 Beck transfer-complexity policy rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t2_beck_transfer_complexity_policy_gate_failures` moved to support::tier

pub(crate) fn load_t2_beck_transfer_complexity_policy(
    path: &Path,
) -> Result<Vec<T2BeckTransferComplexityPolicyRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_beck_transfer_complexity_policy_acceptance_rows(
    policy_rows: &[T2BeckTransferComplexityPolicyRow],
) -> Vec<T2BeckTransferComplexityPolicyAcceptanceRow> {
    let mut rows = policy_rows
        .iter()
        .filter(|row| {
            row.transfer_policy_decision == "transfer-simplification-policy-authored-review"
                && row.publication_status == "held-pending-policy-acceptance"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2BeckTransferComplexityPolicyAcceptanceRow {
            acceptance_id: format!("T2TRANSFERACCEPT-{}", stable_id_fragment(&row.route)),
            policy_id: row.policy_id.clone(),
            route: row.route.clone(),
            complexity_band: row.complexity_band.clone(),
            accepted_render_treatment: row.render_treatment.clone(),
            accepted_promotion_treatment: row.promotion_treatment.clone(),
            acceptance_decision: "transfer-simplification-policy-accepted".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t2-beck-transfer-complexity-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t2_beck_transfer_complexity_policy_acceptance(
    path: &Path,
    rows: &[T2BeckTransferComplexityPolicyAcceptanceRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_beck_transfer_complexity_policy_acceptance_summary(
    output: &Path,
    rows: &[T2BeckTransferComplexityPolicyAcceptanceRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 Beck transfer-complexity policy acceptance rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t2_beck_transfer_complexity_policy_acceptance_gate_failures` moved to support::tier

pub(crate) fn load_t2_beck_transfer_complexity_policy_acceptance(
    path: &Path,
) -> Result<Vec<T2BeckTransferComplexityPolicyAcceptanceRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t2_beck_transfer_complexity_blocker_relief_rows(
    acceptance_rows: &[T2BeckTransferComplexityPolicyAcceptanceRow],
) -> Vec<T2BeckTransferComplexityBlockerReliefRow> {
    let mut rows = acceptance_rows
        .iter()
        .filter(|row| {
            row.acceptance_decision == "transfer-simplification-policy-accepted"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T2BeckTransferComplexityBlockerReliefRow {
            relief_id: format!("T2TRANSFERRELIEF-{}", stable_id_fragment(&row.route)),
            acceptance_id: row.acceptance_id.clone(),
            policy_id: row.policy_id.clone(),
            route: row.route.clone(),
            complexity_band: row.complexity_band.clone(),
            accepted_render_treatment: row.accepted_render_treatment.clone(),
            relief_decision: "relief-ready-for-constraint-ledger-replay".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: String::new(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: 0,
            claim_blocker_delta: -(row.blocker_count_after as isize),
            ledger_replay_status: "pending-optimizer-constraint-ledger-replay".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t2_beck_transfer_complexity_blocker_relief(
    path: &Path,
    rows: &[T2BeckTransferComplexityBlockerReliefRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_beck_transfer_complexity_blocker_relief_summary(
    output: &Path,
    rows: &[T2BeckTransferComplexityBlockerReliefRow],
) {
    let before = rows
        .iter()
        .map(|row| row.blocker_count_before)
        .sum::<usize>();
    let after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T2 Beck transfer-complexity blocker relief rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers before: {before}");
    println!("  claim blockers after: {after}");
}

// `t2_beck_transfer_complexity_blocker_relief_gate_failures` moved to support::tier

pub(crate) fn load_t2_beck_transfer_complexity_blocker_relief(
    path: &Path,
) -> Result<Vec<T2BeckTransferComplexityBlockerReliefRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t3_lower_tier_feeder_gap_review_rows` moved to support::tier

pub(crate) fn write_t3_lower_tier_feeder_gap_review(
    path: &Path,
    rows: &[T3LowerTierFeederGapReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t3_lower_tier_feeder_gap_review_summary(
    output: &Path,
    rows: &[T3LowerTierFeederGapReviewRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T3 lower-tier feeder-gap review rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t3_lower_tier_feeder_gap_review_gate_failures` moved to support::tier

pub(crate) fn load_t3_lower_tier_feeder_gap_review(path: &Path) -> Result<Vec<T3LowerTierFeederGapReviewRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t3_feeder_score_band(current_score: f64) -> &'static str {
    if current_score >= 29.0 {
        "near-threshold-feeder"
    } else if current_score >= 25.0 {
        "low-threshold-feeder"
    } else {
        "out-of-band-feeder"
    }
}

pub(crate) fn t3_lower_tier_feeder_gap_policy_rows(
    review_rows: &[T3LowerTierFeederGapReviewRow],
) -> Vec<T3LowerTierFeederGapPolicyRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| {
            row.review_decision == "lower-tier-feeder-policy-required"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T3LowerTierFeederGapPolicyRow {
            policy_id: format!("T3FEEDERPOLICY-{}", stable_id_fragment(&row.route)),
            feeder_review_id: row.feeder_review_id.clone(),
            gap_id: row.gap_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            score_band: t3_feeder_score_band(row.current_score).to_string(),
            policy_basis: format!(
                "{};required_evidence={};repair_action={}",
                row.gap_reason, row.required_evidence, row.repair_action
            ),
            feeder_policy_decision: "lower-tier-feeder-policy-authored-review".to_string(),
            map_treatment:
                "keep route below T3 feeder promotion until accepted score or terminal evidence exists"
                    .to_string(),
            evidence_treatment:
                "require score-threshold proof or terminal-access evidence before any claim relief"
                    .to_string(),
            upgrade_treatment:
                "hold upgrade framing as T4 or evidence-needed unless policy acceptance authorizes T3 feeder treatment"
                    .to_string(),
            publication_status: "held-pending-policy-acceptance".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t3-lower-tier-feeder-gap-policy-acceptance.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t3_lower_tier_feeder_gap_policy(
    path: &Path,
    rows: &[T3LowerTierFeederGapPolicyRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t3_lower_tier_feeder_gap_policy_summary(
    output: &Path,
    rows: &[T3LowerTierFeederGapPolicyRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T3 lower-tier feeder-gap policy rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t3_lower_tier_feeder_gap_policy_gate_failures` moved to support::tier

pub(crate) fn load_t3_lower_tier_feeder_gap_policy(path: &Path) -> Result<Vec<T3LowerTierFeederGapPolicyRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t3_lower_tier_feeder_gap_policy_acceptance_rows(
    policy_rows: &[T3LowerTierFeederGapPolicyRow],
) -> Vec<T3LowerTierFeederGapPolicyAcceptanceRow> {
    let mut rows = policy_rows
        .iter()
        .filter(|row| {
            row.feeder_policy_decision == "lower-tier-feeder-policy-authored-review"
                && row.publication_status == "held-pending-policy-acceptance"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T3LowerTierFeederGapPolicyAcceptanceRow {
            acceptance_id: format!("T3FEEDERACCEPT-{}", stable_id_fragment(&row.route)),
            policy_id: row.policy_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            score_band: row.score_band.clone(),
            accepted_map_treatment: row.map_treatment.clone(),
            accepted_evidence_treatment: row.evidence_treatment.clone(),
            accepted_upgrade_treatment: row.upgrade_treatment.clone(),
            acceptance_decision: "lower-tier-feeder-policy-accepted".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t3-lower-tier-feeder-gap-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t3_lower_tier_feeder_gap_policy_acceptance(
    path: &Path,
    rows: &[T3LowerTierFeederGapPolicyAcceptanceRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t3_lower_tier_feeder_gap_policy_acceptance_summary(
    output: &Path,
    rows: &[T3LowerTierFeederGapPolicyAcceptanceRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T3 lower-tier feeder-gap policy acceptance rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t3_lower_tier_feeder_gap_policy_acceptance_gate_failures` moved to support::tier

pub(crate) fn load_t3_lower_tier_feeder_gap_policy_acceptance(
    path: &Path,
) -> Result<Vec<T3LowerTierFeederGapPolicyAcceptanceRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t3_lower_tier_feeder_gap_blocker_relief_rows(
    acceptance_rows: &[T3LowerTierFeederGapPolicyAcceptanceRow],
) -> Vec<T3LowerTierFeederGapBlockerReliefRow> {
    let mut rows = acceptance_rows
        .iter()
        .filter(|row| {
            row.acceptance_decision == "lower-tier-feeder-policy-accepted"
                && row.claim_blocker_delta == 0
                && row.blocker_count_after > 0
        })
        .map(|row| T3LowerTierFeederGapBlockerReliefRow {
            relief_id: format!("T3FEEDERRELIEF-{}", stable_id_fragment(&row.route)),
            acceptance_id: row.acceptance_id.clone(),
            policy_id: row.policy_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            score_band: row.score_band.clone(),
            accepted_map_treatment: row.accepted_map_treatment.clone(),
            relief_decision: "relief-ready-for-constraint-ledger-replay".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: String::new(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: 0,
            claim_blocker_delta: -(row.blocker_count_after as isize),
            ledger_replay_status: "pending-optimizer-constraint-ledger-replay".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t3_lower_tier_feeder_gap_blocker_relief(
    path: &Path,
    rows: &[T3LowerTierFeederGapBlockerReliefRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t3_lower_tier_feeder_gap_blocker_relief_summary(
    output: &Path,
    rows: &[T3LowerTierFeederGapBlockerReliefRow],
) {
    let before = rows
        .iter()
        .map(|row| row.blocker_count_before)
        .sum::<usize>();
    let after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T3 lower-tier feeder-gap blocker relief rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers before: {before}");
    println!("  claim blockers after: {after}");
}

// `t3_lower_tier_feeder_gap_blocker_relief_gate_failures` moved to support::tier

#[derive(Default)]
struct T1SharedSegmentPolicyBuilder {
    routes: std::collections::BTreeSet<String>,
    source_review_ids: std::collections::BTreeSet<String>,
    blocker_claims: std::collections::BTreeSet<String>,
    blocker_count: usize,
    policy_basis: std::collections::BTreeSet<String>,
    design_treatments: std::collections::BTreeSet<String>,
}

// `t1_shared_segment_map_policy_rows` moved to support::tier

pub(crate) fn route_display_key(route: &str) -> String {
    route.trim().replace('-', "")
}

pub(crate) fn shared_segment_pair_id(route: &str, overlap_route: &str) -> String {
    let mut routes = [route_display_key(route), route_display_key(overlap_route)];
    routes.sort();
    routes.join("-")
}

pub(crate) fn write_t1_shared_segment_map_policy(
    path: &Path,
    rows: &[T1SharedSegmentMapPolicyRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t1_shared_segment_map_policy_summary(output: &Path, rows: &[T1SharedSegmentMapPolicyRow]) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T1 shared-segment map policy rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t1_shared_segment_map_policy_gate_failures` moved to support::tier

pub(crate) fn load_t1_shared_segment_map_policy(path: &Path) -> Result<Vec<T1SharedSegmentMapPolicyRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t1_shared_segment_policy_acceptance_rows(
    policy_rows: &[T1SharedSegmentMapPolicyRow],
) -> Vec<T1SharedSegmentPolicyAcceptanceRow> {
    let mut rows = policy_rows
        .iter()
        .filter(|row| {
            row.map_policy_decision == "shared-segment-policy-authored-review"
                && row.publication_status == "held-pending-policy-acceptance"
                && row.claim_blocker_delta == 0
        })
        .map(|row| T1SharedSegmentPolicyAcceptanceRow {
            acceptance_id: format!("T1SHAREDACCEPT-{}", stable_id_fragment(&row.policy_id)),
            policy_id: row.policy_id.clone(),
            route_pair: row.route_pair.clone(),
            affected_routes: row.affected_routes.clone(),
            map_policy_decision: row.map_policy_decision.clone(),
            accepted_render_treatment: row.render_treatment.clone(),
            acceptance_status: "accepted-policy-ready-for-relief-replay".to_string(),
            acceptance_basis:
                "policy uses allowed interlined trunk or selected-transfer split treatment"
                    .to_string(),
            publication_status_before: row.publication_status.clone(),
            publication_status_after: "held-pending-blocker-relief-replay".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: row.blocker_count_after,
            claim_blocker_delta: 0,
            next_artifact: "data/t1-schematic-geometry-blocker-relief.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route_pair.cmp(&right.route_pair));
    rows
}

pub(crate) fn write_t1_shared_segment_policy_acceptance(
    path: &Path,
    rows: &[T1SharedSegmentPolicyAcceptanceRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t1_shared_segment_policy_acceptance_summary(
    output: &Path,
    rows: &[T1SharedSegmentPolicyAcceptanceRow],
) {
    let blockers = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T1 shared-segment policy acceptance rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers preserved: {blockers}");
}

// `t1_shared_segment_policy_acceptance_gate_failures` moved to support::tier

pub(crate) fn load_t1_shared_segment_policy_acceptance(
    path: &Path,
) -> Result<Vec<T1SharedSegmentPolicyAcceptanceRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t1_schematic_geometry_blocker_relief_rows(
    acceptance_rows: &[T1SharedSegmentPolicyAcceptanceRow],
) -> Vec<T1SchematicGeometryBlockerReliefRow> {
    let mut rows = acceptance_rows
        .iter()
        .filter(|row| {
            row.acceptance_status == "accepted-policy-ready-for-relief-replay"
                && row.publication_status_after == "held-pending-blocker-relief-replay"
                && row.claim_blocker_delta == 0
        })
        .map(|row| T1SchematicGeometryBlockerReliefRow {
            relief_id: format!(
                "T1SCHEMATICRELIEF-{}",
                stable_id_fragment(&row.acceptance_id)
            ),
            acceptance_id: row.acceptance_id.clone(),
            policy_id: row.policy_id.clone(),
            route_pair: row.route_pair.clone(),
            affected_routes: row.affected_routes.clone(),
            accepted_render_treatment: row.accepted_render_treatment.clone(),
            relief_decision: "relief-ready-for-constraint-ledger-replay".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: String::new(),
            blocker_count_before: row.blocker_count_after,
            blocker_count_after: 0,
            claim_blocker_delta: -(row.blocker_count_after as isize),
            ledger_replay_status: "pending-optimizer-constraint-ledger-replay".to_string(),
            next_artifact: "data/optimizer-constraint-ledger.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route_pair.cmp(&right.route_pair));
    rows
}

pub(crate) fn write_t1_schematic_geometry_blocker_relief(
    path: &Path,
    rows: &[T1SchematicGeometryBlockerReliefRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t1_schematic_geometry_blocker_relief_summary(
    output: &Path,
    rows: &[T1SchematicGeometryBlockerReliefRow],
) {
    let before = rows
        .iter()
        .map(|row| row.blocker_count_before)
        .sum::<usize>();
    let after = rows
        .iter()
        .map(|row| row.blocker_count_after)
        .sum::<usize>();
    println!(
        "  wrote {} T1 schematic-geometry blocker relief rows to {}",
        rows.len(),
        output.display()
    );
    println!("  claim blockers before: {before}");
    println!("  claim blockers after: {after}");
}

// `t1_schematic_geometry_blocker_relief_gate_failures` moved to support::tier

pub(crate) fn pavement_debt_budget_index(rows: &[TierPavementDebtBudgetRow]) -> PavementDebtBudgetIndex {
    let mut index = PavementDebtBudgetIndex::default();
    for row in rows {
        index
            .by_bundle
            .insert(row.segment_bundle_id.clone(), row.clone());
        let route_rollup = index
            .by_route
            .entry(canonical_route_key(&row.route))
            .or_default();
        route_rollup.total_debt_cost_m =
            round_cost_m(route_rollup.total_debt_cost_m + row.total_debt_cost_m);
        route_rollup.debt_classes.insert(row.debt_class.clone());
        route_rollup
            .affected_bundles
            .insert(row.segment_bundle_id.clone());
    }
    index
}

pub(crate) fn pavement_debt_for_candidate(
    route: &str,
    segment_bundle_id: &str,
    index: &PavementDebtBudgetIndex,
) -> (f64, String, String, String) {
    if let Some(row) = index.by_bundle.get(segment_bundle_id) {
        return (
            row.total_debt_cost_m,
            row.debt_class.clone(),
            row.budget_basis.clone(),
            "data/tier-pavement-debt-budget.csv".to_string(),
        );
    }

    if let Some(rollup) = index.by_route.get(&canonical_route_key(route)) {
        return (
            rollup.total_debt_cost_m,
            join_string_set(&rollup.debt_classes),
            format!(
                "route-level pavement debt rollup across {} bundle(s) pending candidate bundle materialization",
                rollup.affected_bundles.len()
            ),
            "data/tier-pavement-debt-budget.csv".to_string(),
        );
    }

    (
        0.0,
        "none".to_string(),
        "no pavement debt row joined".to_string(),
        String::new(),
    )
}

pub(crate) fn print_tier_pavement_debt_budget_summary(
    output: &Path,
    rows: &[TierPavementDebtBudgetRow],
    details: bool,
) {
    let total_cost_m = rows.iter().map(|row| row.total_debt_cost_m).sum::<f64>();
    let mut by_class = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_class.entry(row.debt_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} pavement debt budget rows to {}",
        rows.len(),
        output.display()
    );
    println!("  planning pavement debt: ${total_cost_m:.2}M");
    for (debt_class, count) in by_class {
        println!("  {debt_class}: {count}");
    }

    if details {
        println!();
        println!(
            "{:<4} {:<8} {:<18} {:>7} {:>10} {}",
            "Tier", "Route", "Debt", "Members", "Cost $M", "Bundle"
        );
        println!("{}", "-".repeat(104));
        for row in rows {
            println!(
                "{:<4} {:<8} {:<18} {:>7} {:>10.2} {}",
                row.tier,
                row.route,
                row.debt_class,
                row.blocked_member_count,
                row.total_debt_cost_m,
                row.segment_bundle_id
            );
        }
    }
}

// `tier_pavement_debt_budget_gate_failures` moved to `tier_pavement_debt_budget_gate_failures.rs`

#[derive(Default)]
struct TierPavementAcquisitionBuilder {
    state: String,
    tiers: std::collections::BTreeSet<String>,
    routes: std::collections::BTreeSet<String>,
    bundles: std::collections::BTreeSet<String>,
    blocked_member_count: usize,
}

// `tier_pavement_acquisition_plan_rows` moved to support::pavement

pub(crate) fn pavement_acquisition_action(
    route_count: usize,
    blocked_member_count: usize,
) -> (&'static str, &'static str) {
    if route_count >= 3 || blocked_member_count >= 80 {
        (
            "A",
            "refresh HPMS/state pavement feed for broad multi-route coverage",
        )
    } else if route_count == 2 || blocked_member_count >= 30 {
        (
            "B",
            "refresh HPMS/state pavement feed for targeted corridor coverage",
        )
    } else {
        (
            "C",
            "fill targeted pavement rows from HPMS or state DOT asset feed",
        )
    }
}

pub(crate) fn write_tier_pavement_acquisition_plan(
    path: &Path,
    rows: &[TierPavementAcquisitionPlanRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_acquisition_plan_summary(
    output: &Path,
    rows: &[TierPavementAcquisitionPlanRow],
    details: bool,
) {
    let mut by_priority = std::collections::BTreeMap::<&str, usize>::new();
    let blocked_total: usize = rows.iter().map(|row| row.blocked_member_count).sum();
    for row in rows {
        *by_priority.entry(row.source_priority.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} pavement acquisition rows to {}",
        rows.len(),
        output.display()
    );
    println!("  assigned pavement debt member coverage: {blocked_total}");
    for (priority, count) in by_priority {
        println!("  priority {priority}: {count}");
    }

    if details {
        println!();
        println!(
            "{:<5} {:<3} {:>6} {:>7} {:<28} {}",
            "State", "Pri", "Routes", "Blocked", "Affected routes", "Action"
        );
        println!("{}", "-".repeat(120));
        for row in rows {
            println!(
                "{:<5} {:<3} {:>6} {:>7} {:<28} {}",
                row.state,
                row.source_priority,
                row.route_count,
                row.blocked_member_count,
                truncate_for_table(&row.affected_routes, 28),
                truncate_for_table(&row.acquisition_action, 54)
            );
        }
    }
}

// `tier_pavement_acquisition_plan_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_acquisition_plan(path: &Path) -> Result<Vec<TierPavementAcquisitionPlanRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_acquisition_docket_rows(
    plan_rows: &[TierPavementAcquisitionPlanRow],
) -> Vec<TierPavementAcquisitionDocketRow> {
    let mut rows = plan_rows
        .iter()
        .map(|row| {
            let task_id = format!(
                "PAVEMENT-{}-{}",
                row.source_priority,
                row.state.to_ascii_uppercase()
            );
            TierPavementAcquisitionDocketRow {
                task_id,
                state: row.state.clone(),
                source_priority: row.source_priority.clone(),
                affected_routes: row.affected_routes.clone(),
                affected_bundles: row.affected_bundles.clone(),
                blocked_member_count: row.blocked_member_count,
                fetch_command: format!("route fetch-hpms --states {}", row.state),
                rebuild_command: "route build --all-roads".to_string(),
                verify_command:
                    "route tier-pavement-docket --gate && route tier-pavement-source-gaps --gate"
                        .to_string(),
                source_contract: row.required_fields.clone(),
                next_artifact: row.next_artifact.clone(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| {
        acquisition_priority_rank(&a.source_priority)
            .cmp(&acquisition_priority_rank(&b.source_priority))
            .then_with(|| b.blocked_member_count.cmp(&a.blocked_member_count))
            .then_with(|| a.state.cmp(&b.state))
    });
    rows
}

pub(crate) fn acquisition_priority_rank(priority: &str) -> u8 {
    match priority {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        _ => 3,
    }
}

pub(crate) fn write_tier_pavement_acquisition_docket(
    path: &Path,
    rows: &[TierPavementAcquisitionDocketRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_acquisition_docket_summary(
    output: &Path,
    rows: &[TierPavementAcquisitionDocketRow],
    priority: Option<&str>,
    script: bool,
) {
    let filtered = rows
        .iter()
        .filter(|row| {
            priority
                .map(|priority| row.source_priority.eq_ignore_ascii_case(priority))
                .unwrap_or(true)
        })
        .collect::<Vec<_>>();
    let mut by_priority = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_priority.entry(row.source_priority.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} pavement acquisition docket rows to {}",
        rows.len(),
        output.display()
    );
    println!("  rows shown: {} / {}", filtered.len(), rows.len());
    for (priority, count) in by_priority {
        println!("  priority {priority}: {count}");
    }

    if script {
        println!();
        for row in filtered {
            println!("# {} {} {}", row.task_id, row.state, row.affected_routes);
            println!("{}", row.fetch_command);
            println!("{}", row.rebuild_command);
            println!("{}", row.verify_command);
            println!();
        }
    }
}

// `tier_pavement_acquisition_docket_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_acquisition_docket(
    path: &Path,
) -> Result<Vec<TierPavementAcquisitionDocketRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_source_access_rows(
    docket_rows: &[TierPavementAcquisitionDocketRow],
    priority: &str,
) -> Vec<TierPavementSourceAccessRow> {
    docket_rows
        .iter()
        .filter(|row| row.source_priority.eq_ignore_ascii_case(priority))
        .map(|row| TierPavementSourceAccessRow {
            access_policy_id: format!("PAVEMENTACCESS-{}", stable_id_fragment(&row.task_id)),
            task_id: row.task_id.clone(),
            state: row.state.clone(),
            source_priority: row.source_priority.clone(),
            source_access_mode: "hpms-scoped-fetch".to_string(),
            mutation_mode: "scoped-cache-merge".to_string(),
            cache_targets: format!(
                "data/cache/hpms_2018.csv;data/cache/hpms_{}.csv",
                row.state.to_ascii_lowercase()
            ),
            fetch_command: row.fetch_command.clone(),
            preflight_gate: "route source-fetch-policy --gate".to_string(),
            postfetch_gate: row.verify_command.clone(),
            blocker_claims_before: "publication;sla;transit;upgrade".to_string(),
            blocker_claims_after: "publication;sla;transit;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect()
}

pub(crate) fn write_tier_pavement_source_access(
    path: &Path,
    rows: &[TierPavementSourceAccessRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_source_access_summary(
    output: &Path,
    rows: &[TierPavementSourceAccessRow],
    priority: &str,
) {
    println!(
        "  wrote {} priority-{priority} pavement source-access rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} -> {}",
            row.task_id, row.state, row.source_access_mode, row.mutation_mode
        );
    }
}

// `tier_pavement_source_access_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_source_access(path: &Path) -> Result<Vec<TierPavementSourceAccessRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_source_fetch_attempt_rows(
    source_access_rows: &[TierPavementSourceAccessRow],
) -> Result<Vec<TierPavementSourceFetchAttemptRow>> {
    let mut rows = Vec::new();
    for row in source_access_rows {
        let cache_target = row
            .cache_targets
            .split(';')
            .map(str::trim)
            .find(|target| {
                target.ends_with(&format!("hpms_{}.csv", row.state.to_ascii_lowercase()))
            })
            .unwrap_or("data/cache/hpms_2018.csv");
        let cache_record_count = count_csv_records(Path::new(cache_target))?;
        let fetch_result_status = if cache_record_count == 0 {
            "fetch-failed-or-empty-cache"
        } else {
            "cache-populated-unreviewed"
        };
        rows.push(TierPavementSourceFetchAttemptRow {
            fetch_attempt_id: format!("PAVEMENTFETCH-{}", stable_id_fragment(&row.task_id)),
            access_policy_id: row.access_policy_id.clone(),
            task_id: row.task_id.clone(),
            state: row.state.clone(),
            source_priority: row.source_priority.clone(),
            fetch_command: row.fetch_command.clone(),
            cache_target: cache_target.to_string(),
            cache_record_count,
            fetch_result_status: fetch_result_status.to_string(),
            evidence_acceptance_status: "not-accepted".to_string(),
            blocker_claims_before: row.blocker_claims_before.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/tier-pavement-docket.csv".to_string(),
            validation_status: "review".to_string(),
        });
    }
    Ok(rows)
}

pub(crate) fn count_csv_records(path: &Path) -> Result<usize> {
    if !path.exists() {
        return Ok(0);
    }
    let mut reader = csv::Reader::from_path(path)?;
    Ok(reader.records().count())
}

pub(crate) fn write_tier_pavement_source_fetch_attempt(
    path: &Path,
    rows: &[TierPavementSourceFetchAttemptRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_source_fetch_attempt_summary(
    output: &Path,
    rows: &[TierPavementSourceFetchAttemptRow],
) {
    println!(
        "  wrote {} pavement source-fetch attempt rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} records {}",
            row.task_id, row.state, row.cache_record_count, row.fetch_result_status
        );
    }
}

pub(crate) fn tier_pavement_source_fetch_attempt_gate_failures(
    rows: &[TierPavementSourceFetchAttemptRow],
    source_access_rows: &[TierPavementSourceAccessRow],
) -> Vec<String> {
    let mut failures = Vec::new();
    if !source_access_rows.is_empty() && rows.len() != source_access_rows.len() {
        failures.push(format!(
            "fetch attempt rows {} do not match source-access rows {}",
            rows.len(),
            source_access_rows.len()
        ));
    }
    for row in rows {
        if row.fetch_attempt_id.trim().is_empty()
            || row.access_policy_id.trim().is_empty()
            || row.task_id.trim().is_empty()
            || row.state.trim().is_empty()
            || row.fetch_command.trim().is_empty()
            || row.cache_target.trim().is_empty()
            || row.fetch_result_status.trim().is_empty()
            || row.evidence_acceptance_status.trim().is_empty()
            || row.blocker_claims_before.trim().is_empty()
            || row.blocker_claims_after.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.validation_status.trim().is_empty()
        {
            failures.push(format!("{} has incomplete fetch-attempt row", row.task_id));
        }
        if row.evidence_acceptance_status != "not-accepted" {
            failures.push(format!("{} accepts evidence before review", row.task_id));
        }
        if row.claim_blocker_delta != 0 || row.blocker_claims_after != row.blocker_claims_before {
            failures.push(format!("{} reduces blockers before review", row.task_id));
        }
        if row.cache_record_count == 0 && row.fetch_result_status != "fetch-failed-or-empty-cache" {
            failures.push(format!(
                "{} has empty cache without failed status",
                row.task_id
            ));
        }
    }
    failures
}

pub(crate) fn load_tier_pavement_source_fetch_attempt(
    path: &Path,
) -> Result<Vec<TierPavementSourceFetchAttemptRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `tier_pavement_source_fetch_review_rows` moved to support::pavement

pub(crate) fn pavement_source_gap_still_open_for_task(
    docket_row: &TierPavementAcquisitionDocketRow,
    source_gap_rows: &[TierPavementSourceGapRow],
) -> bool {
    let affected_bundles = docket_row
        .affected_bundles
        .split(';')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .collect::<std::collections::BTreeSet<_>>();
    source_gap_rows.iter().any(|gap_row| {
        affected_bundles.contains(gap_row.segment_bundle_id.as_str())
            && gap_row
                .affected_states
                .split(';')
                .map(str::trim)
                .any(|state| state == docket_row.state)
    })
}

pub(crate) fn write_tier_pavement_source_fetch_review(
    path: &Path,
    rows: &[TierPavementSourceFetchReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_source_fetch_review_summary(
    output: &Path,
    rows: &[TierPavementSourceFetchReviewRow],
) {
    println!(
        "  wrote {} pavement source-fetch review rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} unresolved {}",
            row.task_id, row.state, row.join_review_status, row.postfetch_unresolved_member_count
        );
    }
}

// `tier_pavement_source_fetch_review_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_source_fetch_review(
    path: &Path,
) -> Result<Vec<TierPavementSourceFetchReviewRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `tier_pavement_unmatched_join_review_rows` moved to support::pavement::tier_pavement_unmatched_join_review_rows

pub(crate) fn write_tier_pavement_unmatched_join_review(
    path: &Path,
    rows: &[TierPavementUnmatchedJoinReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_unmatched_join_review_summary(
    output: &Path,
    rows: &[TierPavementUnmatchedJoinReviewRow],
) {
    println!(
        "  wrote {} pavement unmatched join review rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} source-needed {} repair {} hpms-route-records {}",
            row.state,
            row.join_review_status,
            row.source_needed_member_count,
            row.repair_required_member_count,
            row.hpms_records_for_source_needed_routes
        );
    }
}

// `tier_pavement_unmatched_join_review_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_unmatched_join_review(
    path: &Path,
) -> Result<Vec<TierPavementUnmatchedJoinReviewRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `tier_pavement_hpms_scope_broadening_rows` moved to support::pavement

pub(crate) fn write_tier_pavement_hpms_scope_broadening(
    path: &Path,
    rows: &[TierPavementHpmsScopeBroadeningRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_hpms_scope_broadening_summary(
    output: &Path,
    rows: &[TierPavementHpmsScopeBroadeningRow],
) {
    println!(
        "  wrote {} pavement HPMS scope-broadening rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} systems {} routes {} members {}",
            row.state,
            row.broadened_functional_systems,
            row.source_needed_routes,
            row.source_needed_member_count
        );
    }
}

// `tier_pavement_hpms_scope_broadening_gate_failures` moved to support::pavement

// `tier_pavement_repair_debt_review_rows` moved to support::pavement

pub(crate) fn write_tier_pavement_repair_debt_review(
    path: &Path,
    rows: &[TierPavementRepairDebtReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_repair_debt_review_summary(
    output: &Path,
    rows: &[TierPavementRepairDebtReviewRow],
) {
    println!(
        "  wrote {} pavement repair debt review rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} members {} repair ${:.2}M",
            row.state,
            row.route,
            row.repair_debt_status,
            row.blocked_member_count,
            row.estimated_repair_cost_m
        );
    }
}

// `tier_pavement_repair_debt_review_gate_failures` moved to support::pavement

pub(crate) fn pavement_excluded_repair_member_count(
    join_row: &TierPavementUnmatchedJoinReviewRow,
    exclusion_rows: &[TierPavementRouteStateExclusionRow],
) -> usize {
    let repair_routes = semicolon_values(&join_row.repair_required_routes);
    exclusion_rows
        .iter()
        .filter(|row| {
            row.validation_status == "pass"
                && row.exclusion_status == "route-state-not-supported"
                && row.state == join_row.state
                && repair_routes
                    .iter()
                    .any(|route| route_display_key(route) == route_display_key(&row.route))
        })
        .map(|row| row.excluded_member_count)
        .sum()
}

pub(crate) fn pavement_funded_repair_member_count(
    join_row: &TierPavementUnmatchedJoinReviewRow,
    funding_rows: &[TierPavementRepairFundingAcceptanceRow],
) -> usize {
    let repair_routes = semicolon_values(&join_row.repair_required_routes);
    funding_rows
        .iter()
        .filter(|row| {
            row.validation_status == "pass"
                && row.acceptance_status == "accepted-full-cost-repair-funding"
                && row.state == join_row.state
                && repair_routes
                    .iter()
                    .any(|route| route_display_key(route) == route_display_key(&row.route))
                && row.committed_amount_m + 1e-6 >= row.covered_repair_cost_m
                && row.covered_repair_cost_m > 0.0
        })
        .map(|row| (row.covered_repair_cost_m / PAVEMENT_REPAIR_COST_PER_MEMBER_M).round() as usize)
        .sum()
}

pub(crate) fn load_tier_pavement_repair_debt_review(
    path: &Path,
) -> Result<Vec<TierPavementRepairDebtReviewRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_repair_disposition_rows(
    repair_rows: &[TierPavementRepairDebtReviewRow],
) -> Vec<TierPavementRepairDispositionRow> {
    repair_rows
        .iter()
        .filter(|row| {
            row.source_priority == "A"
                && row.repair_debt_status == "confirmed-repair-debt"
                && row.validation_status == "review"
        })
        .map(|row| TierPavementRepairDispositionRow {
            disposition_id: format!(
                "PAVEMENTREPAIRDISPOSITION-{}",
                stable_id_fragment(&row.repair_review_id)
            ),
            repair_review_id: row.repair_review_id.clone(),
            state: row.state.clone(),
            source_priority: row.source_priority.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            blocked_member_count: row.blocked_member_count,
            estimated_repair_cost_m: row.estimated_repair_cost_m,
            disposition: "repair-funding-required".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_action:
                "create repair funding package or downgrade/exclude bundle before relief replay"
                    .to_string(),
            next_artifact: "data/tier-pavement-repair-disposition.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

pub(crate) fn write_tier_pavement_repair_disposition(
    path: &Path,
    rows: &[TierPavementRepairDispositionRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_repair_disposition_summary(
    output: &Path,
    rows: &[TierPavementRepairDispositionRow],
) {
    println!(
        "  wrote {} pavement repair disposition rows to {}",
        rows.len(),
        output.display()
    );
    let total_cost = rows
        .iter()
        .map(|row| row.estimated_repair_cost_m)
        .sum::<f64>();
    println!("  repair funding required: ${total_cost:.2}M");
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.disposition, row.relief_eligibility
        );
    }
}

// `tier_pavement_repair_disposition_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_repair_disposition(
    path: &Path,
) -> Result<Vec<TierPavementRepairDispositionRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_repair_funding_package_rows(
    disposition_rows: &[TierPavementRepairDispositionRow],
) -> Vec<TierPavementRepairFundingPackageRow> {
    disposition_rows
        .iter()
        .filter(|row| {
            row.disposition == "repair-funding-required"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementRepairFundingPackageRow {
            funding_package_id: format!(
                "PAVEMENTREPAIRFUNDING-{}",
                stable_id_fragment(&row.disposition_id)
            ),
            disposition_id: row.disposition_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            blocked_member_count: row.blocked_member_count,
            estimated_repair_cost_m: row.estimated_repair_cost_m,
            funding_package_status: "package-required".to_string(),
            funding_commitment_status: "unfunded".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_action:
                "attach accepted funding commitment or choose downgrade/exclusion before relief replay"
                    .to_string(),
            next_artifact: "data/tier-pavement-repair-funding-package.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

pub(crate) fn write_tier_pavement_repair_funding_package(
    path: &Path,
    rows: &[TierPavementRepairFundingPackageRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_repair_funding_package_summary(
    output: &Path,
    rows: &[TierPavementRepairFundingPackageRow],
) {
    println!(
        "  wrote {} pavement repair funding package rows to {}",
        rows.len(),
        output.display()
    );
    let total_cost = rows
        .iter()
        .map(|row| row.estimated_repair_cost_m)
        .sum::<f64>();
    println!("  unfunded repair package total: ${total_cost:.2}M");
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.funding_package_status, row.funding_commitment_status
        );
    }
}

// `tier_pavement_repair_funding_package_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_repair_funding_package(
    path: &Path,
) -> Result<Vec<TierPavementRepairFundingPackageRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_funding_commitment_review_rows(
    package_rows: &[TierPavementRepairFundingPackageRow],
) -> Vec<TierPavementFundingCommitmentReviewRow> {
    package_rows
        .iter()
        .filter(|row| {
            row.funding_package_status == "package-required"
                && row.funding_commitment_status == "unfunded"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingCommitmentReviewRow {
            commitment_review_id: format!(
                "PAVEMENTFUNDINGCOMMITMENT-{}",
                stable_id_fragment(&row.funding_package_id)
            ),
            funding_package_id: row.funding_package_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            estimated_repair_cost_m: row.estimated_repair_cost_m,
            funding_commitment_status: "no-accepted-commitment-attached".to_string(),
            accepted_commitment_artifact: "none".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_action: "choose downgrade/exclusion or attach accepted funding commitment before relief replay".to_string(),
            next_artifact: "data/tier-pavement-funding-commitment-review.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

pub(crate) fn write_tier_pavement_funding_commitment_review(
    path: &Path,
    rows: &[TierPavementFundingCommitmentReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_commitment_review_summary(
    output: &Path,
    rows: &[TierPavementFundingCommitmentReviewRow],
) {
    println!(
        "  wrote {} pavement funding commitment review rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.funding_commitment_status, row.relief_eligibility
        );
    }
}

// `tier_pavement_funding_commitment_review_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_commitment_review(
    path: &Path,
) -> Result<Vec<TierPavementFundingCommitmentReviewRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_downgrade_exclusion_decision_rows(
    commitment_rows: &[TierPavementFundingCommitmentReviewRow],
) -> Vec<TierPavementDowngradeExclusionDecisionRow> {
    commitment_rows
        .iter()
        .filter(|row| {
            row.funding_commitment_status == "no-accepted-commitment-attached"
                && row.accepted_commitment_artifact == "none"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementDowngradeExclusionDecisionRow {
            downgrade_exclusion_decision_id: format!(
                "PAVEMENTDOWNGRADEEXCLUSION-{}",
                stable_id_fragment(&row.commitment_review_id)
            ),
            commitment_review_id: row.commitment_review_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            estimated_repair_cost_m: row.estimated_repair_cost_m,
            downgrade_decision: "no-downgrade-selected".to_string(),
            exclusion_decision: "no-exclusion-selected".to_string(),
            service_status: "held-at-current-tier".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_action:
                "attach accepted funding evidence before relief replay or open a separate downgrade/exclusion authorization"
                    .to_string(),
            next_artifact: "data/tier-pavement-downgrade-exclusion-decision.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

pub(crate) fn write_tier_pavement_downgrade_exclusion_decision(
    path: &Path,
    rows: &[TierPavementDowngradeExclusionDecisionRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_downgrade_exclusion_decision_summary(
    output: &Path,
    rows: &[TierPavementDowngradeExclusionDecisionRow],
) {
    println!(
        "  wrote {} pavement downgrade/exclusion decision rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.downgrade_decision, row.exclusion_decision
        );
    }
}

// `tier_pavement_downgrade_exclusion_decision_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_downgrade_exclusion_decision(
    path: &Path,
) -> Result<Vec<TierPavementDowngradeExclusionDecisionRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_funding_evidence_contract_rows(
    decision_rows: &[TierPavementDowngradeExclusionDecisionRow],
) -> Vec<TierPavementFundingEvidenceContractRow> {
    decision_rows
        .iter()
        .filter(|row| {
            row.downgrade_decision == "no-downgrade-selected"
                && row.exclusion_decision == "no-exclusion-selected"
                && row.service_status == "held-at-current-tier"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceContractRow {
            evidence_contract_id: format!(
                "PAVEMENTFUNDINGEVIDENCE-{}",
                stable_id_fragment(&row.downgrade_exclusion_decision_id)
            ),
            downgrade_exclusion_decision_id: row.downgrade_exclusion_decision_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            estimated_repair_cost_m: row.estimated_repair_cost_m,
            required_evidence:
                "accepted-programming-document-or-state-dot-commitment-covering-full-repair-cost"
                    .to_string(),
            minimum_commitment_amount_m: row.estimated_repair_cost_m,
            accepted_evidence_status: "source-needed".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_action: "attach accepted funding evidence artifact before relief replay"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-contract.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

pub(crate) fn write_tier_pavement_funding_evidence_contract(
    path: &Path,
    rows: &[TierPavementFundingEvidenceContractRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_contract_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceContractRow],
) {
    println!(
        "  wrote {} pavement funding evidence contract rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} min ${:.2}M",
            row.state, row.route, row.accepted_evidence_status, row.minimum_commitment_amount_m
        );
    }
}

// `tier_pavement_funding_evidence_contract_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_contract(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceContractRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_funding_evidence_source_capture_rows(
    contract_rows: &[TierPavementFundingEvidenceContractRow],
) -> Vec<TierPavementFundingEvidenceSourceCaptureRow> {
    contract_rows
        .iter()
        .filter(|row| {
            row.accepted_evidence_status == "source-needed"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceSourceCaptureRow {
            source_capture_id: format!(
                "PAVEMENTFUNDINGSOURCE-{}",
                stable_id_fragment(&row.evidence_contract_id)
            ),
            evidence_contract_id: row.evidence_contract_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            minimum_commitment_amount_m: row.minimum_commitment_amount_m,
            source_capture_status: "source-needed".to_string(),
            captured_artifact: "none".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_action: "attach accepted funding artifact for review before relief replay"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-source-capture.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

pub(crate) fn write_tier_pavement_funding_evidence_source_capture(
    path: &Path,
    rows: &[TierPavementFundingEvidenceSourceCaptureRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_source_capture_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceSourceCaptureRow],
) {
    println!(
        "  wrote {} pavement funding evidence source-capture rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.source_capture_status, row.captured_artifact
        );
    }
}

// `tier_pavement_funding_evidence_source_capture_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_source_capture(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceSourceCaptureRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_funding_evidence_artifact_attachment_rows(
    capture_rows: &[TierPavementFundingEvidenceSourceCaptureRow],
) -> Vec<TierPavementFundingEvidenceArtifactAttachmentRow> {
    capture_rows
        .iter()
        .filter(|row| {
            row.source_capture_status == "source-needed"
                && row.captured_artifact == "none"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceArtifactAttachmentRow {
            artifact_attachment_id: format!(
                "PAVEMENTFUNDINGATTACH-{}",
                stable_id_fragment(&row.source_capture_id)
            ),
            source_capture_id: row.source_capture_id.clone(),
            evidence_contract_id: row.evidence_contract_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            minimum_commitment_amount_m: row.minimum_commitment_amount_m,
            attachment_status: "source-needed".to_string(),
            attached_artifact: "none".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims_before: row.blocked_claims.clone(),
            blocked_claims_after: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            attachment_blocker:
                "accepted full-cost programming or DOT commitment artifact has not been attached"
                    .to_string(),
            next_action: "attach accepted funding artifact for review before relief replay"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-artifact-attachment.csv"
                .to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

pub(crate) fn write_tier_pavement_funding_evidence_artifact_attachment(
    path: &Path,
    rows: &[TierPavementFundingEvidenceArtifactAttachmentRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_artifact_attachment_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceArtifactAttachmentRow],
) {
    println!(
        "  wrote {} pavement funding evidence artifact-attachment rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.attachment_status, row.attached_artifact
        );
    }
}

// `tier_pavement_funding_evidence_artifact_attachment_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_artifact_attachment(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceArtifactAttachmentRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_funding_evidence_review_docket_rows(
    attachment_rows: &[TierPavementFundingEvidenceArtifactAttachmentRow],
) -> Vec<TierPavementFundingEvidenceReviewDocketRow> {
    attachment_rows
        .iter()
        .filter(|row| {
            row.attachment_status == "source-needed"
                && row.attached_artifact == "none"
                && row.evidence_review_status == "not-reviewed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceReviewDocketRow {
            funding_evidence_review_id: format!(
                "PAVEMENTFUNDINGREVIEW-{}",
                stable_id_fragment(&row.artifact_attachment_id)
            ),
            artifact_attachment_id: row.artifact_attachment_id.clone(),
            evidence_contract_id: row.evidence_contract_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            minimum_commitment_amount_m: row.minimum_commitment_amount_m,
            attached_artifact: row.attached_artifact.clone(),
            review_decision: "held-no-attached-artifact".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            review_reason:
                "artifact attachment remains source-needed; funding evidence cannot be reviewed or accepted"
                    .to_string(),
            blocked_claims_before: row.blocked_claims_after.clone(),
            blocked_claims_after: row.blocked_claims_after.clone(),
            claim_blocker_delta: 0,
            next_action: "attach accepted funding artifact before evidence review or relief replay"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-review-docket.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

pub(crate) fn write_tier_pavement_funding_evidence_review_docket(
    path: &Path,
    rows: &[TierPavementFundingEvidenceReviewDocketRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_review_docket_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceReviewDocketRow],
) {
    println!(
        "  wrote {} pavement funding evidence review rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.review_decision, row.accepted_evidence_status
        );
    }
}

// `tier_pavement_funding_evidence_review_docket_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_review_docket(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceReviewDocketRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_funding_evidence_acquisition_rows(
    review_rows: &[TierPavementFundingEvidenceReviewDocketRow],
) -> Vec<TierPavementFundingEvidenceAcquisitionRow> {
    review_rows
        .iter()
        .filter(|row| {
            row.review_decision == "held-no-attached-artifact"
                && row.attached_artifact == "none"
                && row.evidence_review_status == "not-reviewed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceAcquisitionRow {
            funding_evidence_acquisition_id: format!(
                "PAVEMENTFUNDINGACQUIRE-{}",
                stable_id_fragment(&row.funding_evidence_review_id)
            ),
            funding_evidence_review_id: row.funding_evidence_review_id.clone(),
            evidence_contract_id: row.evidence_contract_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            minimum_commitment_amount_m: row.minimum_commitment_amount_m,
            required_artifact_type: "accepted-full-cost-programming-or-dot-commitment".to_string(),
            acquisition_status: "source-needed".to_string(),
            candidate_source_owner: format!("{} DOT or accepted programming authority", row.state),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: row.blocked_claims_after.clone(),
            claim_blocker_delta: 0,
            acquisition_reason:
                "funding evidence review is held because no accepted artifact is attached"
                    .to_string(),
            next_action: "acquire accepted full-cost funding artifact before attachment and review"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-acquisition.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

pub(crate) fn write_tier_pavement_funding_evidence_acquisition(
    path: &Path,
    rows: &[TierPavementFundingEvidenceAcquisitionRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_acquisition_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcquisitionRow],
) {
    println!(
        "  wrote {} pavement funding evidence acquisition rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.acquisition_status, row.accepted_evidence_status
        );
    }
}

// `tier_pavement_funding_evidence_acquisition_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_acquisition(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceAcquisitionRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `tier_pavement_funding_evidence_source_access_rows` moved to support::pavement

pub(crate) fn write_tier_pavement_funding_evidence_source_access(
    path: &Path,
    rows: &[TierPavementFundingEvidenceSourceAccessRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_source_access_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceSourceAccessRow],
) {
    println!(
        "  wrote {} pavement funding evidence source-access rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.access_mode, row.evidence_artifact
        );
    }
}

// `tier_pavement_funding_evidence_source_access_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_source_access(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceSourceAccessRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_funding_evidence_intake_rows(
    access_rows: &[TierPavementFundingEvidenceSourceAccessRow],
) -> Vec<TierPavementFundingEvidenceIntakeRow> {
    access_rows
        .iter()
        .filter(|row| {
            row.access_mode == "manual-or-cached-source-needed"
                && row.evidence_artifact == "source-needed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceIntakeRow {
            funding_evidence_intake_id: format!(
                "PAVEMENTFUNDINGINTAKE-{}",
                stable_id_fragment(&row.source_access_id)
            ),
            source_access_id: row.source_access_id.clone(),
            evidence_contract_id: row.evidence_contract_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            required_artifact_type: "accepted-full-cost-programming-or-dot-commitment".to_string(),
            required_source_metadata: row.required_source_metadata.clone(),
            intake_status: "artifact-required".to_string(),
            evidence_artifact: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            intake_blocker: "accepted funding artifact metadata has not been captured or cached"
                .to_string(),
            blocked_claims: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_action: "capture accepted funding artifact metadata before attachment and review"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-intake.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

pub(crate) fn write_tier_pavement_funding_evidence_intake(
    path: &Path,
    rows: &[TierPavementFundingEvidenceIntakeRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_intake_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceIntakeRow],
) {
    println!(
        "  wrote {} pavement funding evidence intake rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.intake_status, row.evidence_artifact
        );
    }
}

// `tier_pavement_funding_evidence_intake_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_intake(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceIntakeRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_funding_evidence_metadata_capture_rows(
    intake_rows: &[TierPavementFundingEvidenceIntakeRow],
) -> Vec<TierPavementFundingEvidenceMetadataCaptureRow> {
    intake_rows
        .iter()
        .filter(|row| {
            row.intake_status == "artifact-required"
                && row.evidence_artifact == "source-needed"
                && row.evidence_review_status == "not-reviewed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceMetadataCaptureRow {
            metadata_capture_id: format!(
                "PAVEMENTFUNDINGMETADATA-{}",
                stable_id_fragment(&row.funding_evidence_intake_id)
            ),
            funding_evidence_intake_id: row.funding_evidence_intake_id.clone(),
            evidence_contract_id: row.evidence_contract_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            required_artifact_type: row.required_artifact_type.clone(),
            metadata_capture_status: "source-needed".to_string(),
            captured_artifact: "none".to_string(),
            captured_source_title: "source-needed".to_string(),
            captured_source_url: "source-needed".to_string(),
            captured_commitment_amount_m: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            blocked_claims: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_action: "capture accepted funding artifact metadata before attachment and review"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-metadata-capture.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

pub(crate) fn write_tier_pavement_funding_evidence_metadata_capture(
    path: &Path,
    rows: &[TierPavementFundingEvidenceMetadataCaptureRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_metadata_capture_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceMetadataCaptureRow],
) {
    println!(
        "  wrote {} pavement funding evidence metadata-capture rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.metadata_capture_status, row.captured_artifact
        );
    }
}

// `tier_pavement_funding_evidence_metadata_capture_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_metadata_capture(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceMetadataCaptureRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `tier_pavement_funding_evidence_accepted_artifact_attachment_rows` moved to support::pavement

pub(crate) fn write_tier_pavement_funding_evidence_accepted_artifact_attachment(
    path: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedArtifactAttachmentRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_accepted_artifact_attachment_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedArtifactAttachmentRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted-artifact attachment rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.attachment_status, row.attached_artifact
        );
    }
}

// `tier_pavement_funding_evidence_accepted_artifact_attachment_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_accepted_artifact_attachment(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceAcceptedArtifactAttachmentRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_funding_evidence_accepted_attachment_review_rows(
    attachment_rows: &[TierPavementFundingEvidenceAcceptedArtifactAttachmentRow],
) -> Vec<TierPavementFundingEvidenceAcceptedAttachmentReviewRow> {
    attachment_rows
        .iter()
        .filter(|row| {
            row.attachment_status == "source-needed"
                && row.attached_artifact == "none"
                && row.evidence_review_status == "not-reviewed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceAcceptedAttachmentReviewRow {
            accepted_attachment_review_id: format!(
                "PAVEMENTFUNDINGACCEPTEDREVIEW-{}",
                stable_id_fragment(&row.accepted_artifact_attachment_id)
            ),
            accepted_artifact_attachment_id: row.accepted_artifact_attachment_id.clone(),
            evidence_contract_id: row.evidence_contract_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            required_artifact_type: row.required_artifact_type.clone(),
            attached_artifact: row.attached_artifact.clone(),
            review_decision: "held-no-attached-artifact".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            review_reason:
                "accepted artifact attachment remains source-needed; funding evidence cannot be reviewed or accepted"
                    .to_string(),
            blocked_claims_before: row.blocked_claims_after.clone(),
            blocked_claims_after: row.blocked_claims_after.clone(),
            claim_blocker_delta: 0,
            next_action: "attach accepted funding artifact before evidence review or relief replay"
                .to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-accepted-attachment-review.csv"
                .to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

pub(crate) fn write_tier_pavement_funding_evidence_accepted_attachment_review(
    path: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedAttachmentReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_accepted_attachment_review_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedAttachmentReviewRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted-attachment review rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.review_decision, row.accepted_evidence_status
        );
    }
}

// `tier_pavement_funding_evidence_accepted_attachment_review_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_accepted_attachment_review(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceAcceptedAttachmentReviewRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `tier_pavement_funding_evidence_accepted_artifact_acquisition_rows` moved to support::pavement

pub(crate) fn write_tier_pavement_funding_evidence_accepted_artifact_acquisition(
    path: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedArtifactAcquisitionRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_accepted_artifact_acquisition_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedArtifactAcquisitionRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted-artifact acquisition rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.acquisition_status, row.cache_status
        );
    }
}

// `tier_pavement_funding_evidence_accepted_artifact_acquisition_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_accepted_artifact_acquisition(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceAcceptedArtifactAcquisitionRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `tier_pavement_funding_evidence_accepted_source_access_rows` moved to support::pavement

pub(crate) fn write_tier_pavement_funding_evidence_accepted_source_access(
    path: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedSourceAccessRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_accepted_source_access_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedSourceAccessRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted source-access rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.access_mode, row.evidence_artifact
        );
    }
}

// `tier_pavement_funding_evidence_accepted_source_access_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_accepted_source_access(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceAcceptedSourceAccessRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_funding_evidence_accepted_intake_rows(
    access_rows: &[TierPavementFundingEvidenceAcceptedSourceAccessRow],
) -> Vec<TierPavementFundingEvidenceAcceptedIntakeRow> {
    access_rows
        .iter()
        .filter(|row| {
            row.access_mode == "manual-or-cached-source-needed"
                && row.cache_status == "not-cached"
                && row.evidence_artifact == "source-needed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceAcceptedIntakeRow {
            accepted_intake_id: format!(
                "PAVEMENTFUNDINGACCEPTEDINTAKE-{}",
                stable_id_fragment(&row.accepted_source_access_id)
            ),
            accepted_source_access_id: row.accepted_source_access_id.clone(),
            evidence_contract_id: row.evidence_contract_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            required_artifact_type: "accepted-full-cost-programming-or-dot-commitment".to_string(),
            required_source_metadata: row.required_source_metadata.clone(),
            intake_status: "artifact-required".to_string(),
            cache_status: "not-cached".to_string(),
            evidence_artifact: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            intake_blocker: "accepted funding artifact metadata has not been captured".to_string(),
            blocked_claims: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_action: "capture accepted funding artifact metadata".to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-accepted-intake.csv".to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

pub(crate) fn write_tier_pavement_funding_evidence_accepted_intake(
    path: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedIntakeRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_accepted_intake_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedIntakeRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted intake rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.intake_status, row.evidence_artifact
        );
    }
}

// `tier_pavement_funding_evidence_accepted_intake_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_accepted_intake(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceAcceptedIntakeRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `tier_pavement_funding_evidence_accepted_metadata_capture_rows` moved to support::pavement

pub(crate) fn write_tier_pavement_funding_evidence_accepted_metadata_capture(
    path: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataCaptureRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_accepted_metadata_capture_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataCaptureRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted metadata-capture rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.metadata_capture_status, row.captured_artifact
        );
    }
}

// `tier_pavement_funding_evidence_accepted_metadata_capture_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_accepted_metadata_capture(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceAcceptedMetadataCaptureRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `tier_pavement_funding_evidence_accepted_metadata_artifact_attachment_rows` moved to support::pavement

pub(crate) fn write_tier_pavement_funding_evidence_accepted_metadata_artifact_attachment(
    path: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataArtifactAttachmentRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_accepted_metadata_artifact_attachment_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataArtifactAttachmentRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted metadata artifact-attachment rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.attachment_status, row.attached_artifact
        );
    }
}

// `tier_pavement_funding_evidence_accepted_metadata_artifact_attachment_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_accepted_metadata_artifact_attachment(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceAcceptedMetadataArtifactAttachmentRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `tier_pavement_funding_evidence_accepted_metadata_attachment_review_rows` moved to support::pavement

pub(crate) fn write_tier_pavement_funding_evidence_accepted_metadata_attachment_review(
    path: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataAttachmentReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_accepted_metadata_attachment_review_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataAttachmentReviewRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted metadata attachment-review rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.review_decision, row.accepted_evidence_status
        );
    }
}

// `tier_pavement_funding_evidence_accepted_metadata_attachment_review_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_accepted_metadata_attachment_review(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceAcceptedMetadataAttachmentReviewRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition_rows` moved to support::pavement

pub(crate) fn write_tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition(
    path: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataArtifactAcquisitionRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataArtifactAcquisitionRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted metadata artifact-acquisition rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.acquisition_status, row.cache_status
        );
    }
}

// `tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_accepted_metadata_artifact_acquisition(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceAcceptedMetadataArtifactAcquisitionRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `tier_pavement_funding_evidence_accepted_metadata_source_access_rows` moved to support::pavement

pub(crate) fn write_tier_pavement_funding_evidence_accepted_metadata_source_access(
    path: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataSourceAccessRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_accepted_metadata_source_access_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataSourceAccessRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted metadata source-access rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.access_mode, row.evidence_artifact
        );
    }
}

// `tier_pavement_funding_evidence_accepted_metadata_source_access_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_accepted_metadata_source_access(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceAcceptedMetadataSourceAccessRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn tier_pavement_funding_evidence_accepted_metadata_intake_rows(
    access_rows: &[TierPavementFundingEvidenceAcceptedMetadataSourceAccessRow],
) -> Vec<TierPavementFundingEvidenceAcceptedMetadataIntakeRow> {
    access_rows
        .iter()
        .filter(|row| {
            row.access_mode == "manual-or-cached-source-needed"
                && row.cache_status == "not-cached"
                && row.evidence_artifact == "source-needed"
                && row.accepted_evidence_status == "not-accepted"
                && row.relief_eligibility == "not-eligible-for-relief"
                && row.validation_status == "held"
        })
        .map(|row| TierPavementFundingEvidenceAcceptedMetadataIntakeRow {
            accepted_metadata_intake_id: format!(
                "PAVEMENTFUNDINGACCEPTEDMETAINTAKE-{}",
                stable_id_fragment(&row.accepted_metadata_source_access_id)
            ),
            accepted_metadata_source_access_id: row.accepted_metadata_source_access_id.clone(),
            evidence_contract_id: row.evidence_contract_id.clone(),
            state: row.state.clone(),
            tier: row.tier.clone(),
            route: row.route.clone(),
            segment_bundle_id: row.segment_bundle_id.clone(),
            required_artifact_type: "accepted-full-cost-programming-or-dot-commitment".to_string(),
            required_source_metadata: row.required_source_metadata.clone(),
            intake_status: "artifact-required".to_string(),
            cache_status: "not-cached".to_string(),
            evidence_artifact: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            accepted_evidence_status: "not-accepted".to_string(),
            relief_eligibility: "not-eligible-for-relief".to_string(),
            intake_blocker: "accepted funding artifact metadata has not been captured".to_string(),
            blocked_claims: row.blocked_claims.clone(),
            claim_blocker_delta: 0,
            next_action: "capture accepted funding artifact metadata".to_string(),
            next_artifact: "data/tier-pavement-funding-evidence-accepted-metadata-intake.csv"
                .to_string(),
            validation_status: "held".to_string(),
        })
        .collect()
}

pub(crate) fn write_tier_pavement_funding_evidence_accepted_metadata_intake(
    path: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataIntakeRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_accepted_metadata_intake_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataIntakeRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted metadata intake rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.intake_status, row.evidence_artifact
        );
    }
}

// `tier_pavement_funding_evidence_accepted_metadata_intake_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_accepted_metadata_intake(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceAcceptedMetadataIntakeRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `tier_pavement_funding_evidence_accepted_metadata_source_capture_rows` moved to support::pavement

pub(crate) fn write_tier_pavement_funding_evidence_accepted_metadata_source_capture(
    path: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataSourceCaptureRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_accepted_metadata_source_capture_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataSourceCaptureRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted metadata source-capture rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.source_capture_status, row.captured_artifact
        );
    }
}

// `tier_pavement_funding_evidence_accepted_metadata_source_capture_gate_failures` moved to support::pavement

pub(crate) fn load_tier_pavement_funding_evidence_accepted_metadata_source_capture(
    path: &Path,
) -> Result<Vec<TierPavementFundingEvidenceAcceptedMetadataSourceCaptureRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `tier_pavement_funding_evidence_accepted_metadata_source_capture_artifact_attachment_rows` moved to support::pavement

pub(crate) fn write_tier_pavement_funding_evidence_accepted_metadata_source_capture_artifact_attachment(
    path: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataSourceCaptureArtifactAttachmentRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_pavement_funding_evidence_accepted_metadata_source_capture_artifact_attachment_summary(
    output: &Path,
    rows: &[TierPavementFundingEvidenceAcceptedMetadataSourceCaptureArtifactAttachmentRow],
) {
    println!(
        "  wrote {} pavement funding evidence accepted metadata source-capture artifact-attachment rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} {} {} {}",
            row.state, row.route, row.attachment_status, row.attached_artifact
        );
    }
}

// `tier_pavement_funding_evidence_accepted_metadata_source_capture_artifact_attachment_gate_failures` moved to support::pavement::tier_pavement_funding_evidence_accepted_metadata_source_capture_artifact_attachment_gate_failures

pub(crate) fn load_tier_table_rows(path: &Path) -> Result<Vec<TierTableScoreRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `lower_tier_pressure_witness_rows` moved to support::misc::lower_tier_pressure_witness_rows

pub(crate) fn lower_tier_score_pressure_row(
    row: &TierTableScoreRow,
    pressure_type: &str,
    witness_action: &str,
    target_tier: &str,
    selection_basis: &str,
) -> LowerTierPressureWitnessRow {
    LowerTierPressureWitnessRow {
        route: row.route.clone(),
        current_tier: row.tier.clone(),
        current_score: row.score,
        confidence: row.confidence,
        confidence_label: row.confidence_label.clone(),
        pressure_type: pressure_type.to_string(),
        witness_action: witness_action.to_string(),
        target_tier: target_tier.to_string(),
        selection_basis: selection_basis.to_string(),
        source_artifact: "data/tier-table.csv".to_string(),
        next_artifact: if target_tier == "T2" {
            "data/tier-contact-witnesses.csv".to_string()
        } else {
            "data/tier-region-workloads.csv".to_string()
        },
        validation_status: "review".to_string(),
    }
}

pub(crate) fn write_lower_tier_pressure_witnesses(
    path: &Path,
    rows: &[LowerTierPressureWitnessRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_lower_tier_pressure_witness_summary(output: &Path, rows: &[LowerTierPressureWitnessRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.pressure_type.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} pressure witness rows to {}",
        rows.len(),
        output.display()
    );
    for (pressure_type, count) in counts {
        println!("  {pressure_type}: {count}");
    }
}

pub(crate) fn lower_tier_pressure_witness_gate_failures(rows: &[LowerTierPressureWitnessRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no lower-tier pressure witnesses emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.current_tier.trim().is_empty()
            || row.pressure_type.trim().is_empty()
            || row.witness_action.trim().is_empty()
            || row.target_tier.trim().is_empty()
            || row.next_artifact.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete pressure witness contract",
                row.route
            ));
        }
    }
    failures
}

pub(crate) fn load_lower_tier_pressure_witnesses(path: &Path) -> Result<Vec<LowerTierPressureWitnessRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t3_t4_pressure_intake_rows(
    pressure_rows: &[LowerTierPressureWitnessRow],
) -> Vec<T3T4PressureIntakeRow> {
    let mut rows = pressure_rows
        .iter()
        .map(|row| {
            let (intake_class, intake_action, target_tier, next_artifact, optimizer_effect) =
                t3_t4_pressure_intake_decision(row);
            T3T4PressureIntakeRow {
                route: row.route.clone(),
                source_pressure_type: row.pressure_type.clone(),
                current_tier: row.current_tier.clone(),
                current_score: row.current_score,
                target_tier: target_tier.to_string(),
                intake_class: intake_class.to_string(),
                intake_action: intake_action.to_string(),
                selection_basis: row.selection_basis.clone(),
                source_artifact: row.source_artifact.clone(),
                next_artifact: next_artifact.to_string(),
                optimizer_effect: optimizer_effect.to_string(),
                validation_status: "review".to_string(),
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| {
        a.intake_class
            .cmp(&b.intake_class)
            .then_with(|| b.current_score.total_cmp(&a.current_score))
            .then_with(|| a.route.cmp(&b.route))
    });
    rows
}

// `t3_t4_pressure_intake_decision` moved to support::tier

pub(crate) fn write_t3_t4_pressure_intake(path: &Path, rows: &[T3T4PressureIntakeRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t3_t4_pressure_intake_summary(output: &Path, rows: &[T3T4PressureIntakeRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.intake_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T3/T4 pressure intake rows to {}",
        rows.len(),
        output.display()
    );
    for (intake_class, count) in counts {
        println!("  {intake_class}: {count}");
    }
}

pub(crate) fn t3_t4_pressure_intake_gate_failures(rows: &[T3T4PressureIntakeRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T3/T4 pressure intake rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.intake_class.trim().is_empty()
            || row.intake_action.trim().is_empty()
            || row.target_tier.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!("{} has incomplete pressure intake", row.route));
        }
    }
    failures
}

pub(crate) fn load_t3_t4_pressure_intake(path: &Path) -> Result<Vec<T3T4PressureIntakeRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t3_zone_access_obligation_rows` moved to support::tier

pub(crate) fn t3_zone_map_ids(atlas_rows: &[MapAtlasRow]) -> std::collections::BTreeSet<String> {
    atlas_rows
        .iter()
        .filter(|row| row.map_type == "t3-zone")
        .map(|row| row.map_id.clone())
        .collect()
}

pub(crate) fn t3_zone_for_route(route: &str) -> Option<(&'static str, &'static str)> {
    let key = canonical_route_key(route);
    let zone_id = match key.as_str() {
        "I71" | "I72" | "I74" | "I75" | "I93" | "I96" | "I115" | "I129" | "I176" | "I180"
        | "I190" | "I196" | "I220" | "I235" | "I264" | "I270" | "I271" | "I275" | "I276"
        | "I279" | "I280" | "I294" | "I390" | "I465" | "I471" | "I478" | "I480" | "I496"
        | "I675" | "I691" | "I696" | "I990" | "US7" | "US10" | "US15" | "US22" | "US31"
        | "US35" | "US40" | "US41" | "US42" | "US74" | "US75" | "US223" | "US224" | "US250" => {
            "t3-great-lakes"
        }
        "I16" | "I22" | "I24" | "I37" | "I57" | "I59" | "I65" | "I85" | "I140" | "I175"
        | "I185" | "I464" | "I795" | "US17" | "US45E" | "US45W" | "US74E" | "US80" | "US82"
        | "US84" | "US90Z" | "US119" | "US278" | "US301" => "t3-southeast",
        "I2" | "I10" | "I19" | "I37W" | "I45" | "I69E" | "I110" | "I410" | "I510" | "I610"
        | "US69" | "US77" | "US83" | "US90" | "US96" | "US281" => "t3-texas-border",
        "I8" | "I15" | "I25" | "I70" | "I80" | "I135" | "I205" | "I215" | "I225" | "I335"
        | "I680" | "I705" | "I880" | "US2" | "US6" | "US14" | "US26" | "US76" | "US87" | "US95"
        | "US287" => "t3-mountain-west",
        "I30" | "I40" | "I44" | "I49" | "I55" | "I169" | "I181" | "I240" | "I255" | "I277"
        | "I295" | "I630" | "I635" | "I664" | "I759" | "I840" | "US24" | "US66" | "US69S"
        | "US70" | "US71" | "US167" | "US270" | "US421" => "t3-mid-south",
        _ => return None,
    };
    t3_zone_catalog_entry(zone_id)
}

pub(crate) fn t3_zone_catalog_entry(zone_id: &str) -> Option<(&'static str, &'static str)> {
    match zone_id {
        "t3-great-lakes" => Some(("t3-great-lakes", "Great Lakes / Ohio Valley")),
        "t3-southeast" => Some(("t3-southeast", "Southeast / Appalachia")),
        "t3-texas-border" => Some(("t3-texas-border", "Texas Border / Gulf Access")),
        "t3-mountain-west" => Some(("t3-mountain-west", "Mountain West / Interior Coverage")),
        "t3-mid-south" => Some(("t3-mid-south", "Mid-South / Delta / Ozarks")),
        _ => None,
    }
}

pub(crate) fn t3_obligation_class_for_intake(intake_class: &str) -> &'static str {
    match intake_class {
        "bubble-up-t2-review" => "regional-upgrade-review",
        "t4-local-intake" => "terminal-local-access",
        _ => "regional-feeder-access",
    }
}

pub(crate) fn t3_zone_obligation_contract(
    obligation_class: &str,
) -> (&'static str, u8, &'static str, &'static str) {
    match obligation_class {
        "regional-upgrade-review" => (
            "prove T2 contact and regional service value before upgrade",
            24,
            "data/t2-bubble-up-review.csv",
            "keeps lower-tier upgrade pressure attached to zone maps before any T2 reopening",
        ),
        "terminal-local-access" => (
            "select T4 terminal/local access chain inside the zone",
            1,
            "data/t4-terminal-access-columns.csv",
            "turns local pressure into terminal access columns instead of national promotion",
        ),
        _ => (
            "select T3 feeder/contact chain inside the zone",
            6,
            "data/t3-zone-route-columns.csv",
            "turns lower-tier pressure into regional feeder obligations for zone maps",
        ),
    }
}

pub(crate) fn write_t3_zone_access_obligations(path: &Path, rows: &[T3ZoneAccessObligationRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t3_zone_access_obligation_summary(output: &Path, rows: &[T3ZoneAccessObligationRow]) {
    let mut by_zone = std::collections::BTreeMap::<&str, usize>::new();
    let mut by_class = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_zone.entry(row.zone_id.as_str()).or_default() += 1;
        *by_class.entry(row.obligation_class.as_str()).or_default() += 1;
    }

    println!(
        "  wrote {} T3 zone access obligation rows to {}",
        rows.len(),
        output.display()
    );
    for (zone, count) in by_zone {
        println!("  {zone}: {count}");
    }
    for (class, count) in by_class {
        println!("  {class}: {count}");
    }
}

// `t3_zone_access_obligation_gate_failures` moved to support::tier

pub(crate) fn load_t3_zone_access_obligations(path: &Path) -> Result<Vec<T3ZoneAccessObligationRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t3_zone_route_column_rows` moved to support::tier

pub(crate) fn semicolon_values(value: &str) -> Vec<String> {
    value
        .split(';')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(ToString::to_string)
        .collect()
}

pub(crate) fn constraint_class_values(value: &str) -> Vec<String> {
    value
        .split([';', '|'])
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(ToString::to_string)
        .collect()
}

// `t3_zone_route_column_decision` moved to support::tier

pub(crate) fn write_t3_zone_route_columns(path: &Path, rows: &[T3ZoneRouteColumnRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t3_zone_route_column_summary(output: &Path, rows: &[T3ZoneRouteColumnRow]) {
    let mut by_decision = std::collections::BTreeMap::<&str, usize>::new();
    let mut by_zone = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_decision.entry(row.column_decision.as_str()).or_default() += 1;
        *by_zone.entry(row.zone_id.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T3 zone route column rows to {}",
        rows.len(),
        output.display()
    );
    for (zone, count) in by_zone {
        println!("  {zone}: {count}");
    }
    for (decision, count) in by_decision {
        println!("  {decision}: {count}");
    }
}

// `t3_zone_route_column_gate_failures` moved to support::tier

// `t4_terminal_access_column_rows` moved to support::tier

// `t4_terminal_access_decision` moved to support::tier

pub(crate) fn t4_terminal_source_contract(zone_id: &str) -> (&'static str, &'static str) {
    match zone_id {
        "t3-great-lakes" => (
            "prove one-hour access to a Great Lakes / Ohio Valley terminal district: Chicago Intermodal Complex, Columbus South, Indianapolis Avon, Detroit Livernois, Minneapolis Twin Cities, St. Louis Gateway, Philadelphia Frankford, or New York Fresh Pond",
            "cite a terminal district/contact source from data/intermodal_terminals.csv and attach the route to a selected T3/T2/T1 column",
        ),
        "t3-southeast" => (
            "prove one-hour access to a Southeast / Appalachia terminal district: Atlanta Hulsey, Charlotte Intermodal, Savannah Garden City, Miami Hialeah, or New Orleans Gentilly",
            "cite a terminal district/contact source from data/intermodal_terminals.csv and attach the route to a selected T3/T2/T1 column",
        ),
        "t3-texas-border" => (
            "prove one-hour access to a Texas Border / Gulf terminal district: Dallas Alliance, Houston Englewood, San Antonio Kirby, or New Orleans Gentilly",
            "cite a terminal district/contact source from data/intermodal_terminals.csv and attach the route to a selected T3/T2/T1 column",
        ),
        "t3-mountain-west" => (
            "prove one-hour access to a Mountain West / Interior terminal district: Denver Logistics Hub, Salt Lake City, Phoenix Sky Harbor area, Portland Albina, Seattle BNSF, Los Angeles/Long Beach, or Kansas City Gateway",
            "cite a terminal district/contact source from data/intermodal_terminals.csv and attach the route to a selected T3/T2/T1 column",
        ),
        "t3-mid-south" => (
            "prove one-hour access to a Mid-South / Delta / Ozarks terminal district: Memphis Intermodal, Kansas City Gateway, St. Louis Gateway, New Orleans Gentilly, or Louisville KentuckyOne",
            "cite a terminal district/contact source from data/intermodal_terminals.csv and attach the route to a selected T3/T2/T1 column",
        ),
        _ => (
            "prove one-hour terminal, port, yard, warehouse, or local freight access",
            "named terminal/local district plus contact to selected T3/T2/T1 column",
        ),
    }
}

pub(crate) fn write_t4_terminal_access_columns(path: &Path, rows: &[T4TerminalAccessColumnRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_access_column_summary(output: &Path, rows: &[T4TerminalAccessColumnRow]) {
    let mut by_decision = std::collections::BTreeMap::<&str, usize>::new();
    let mut by_zone = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_decision.entry(row.column_decision.as_str()).or_default() += 1;
        *by_zone.entry(row.zone_id.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access column rows to {}",
        rows.len(),
        output.display()
    );
    for (zone, count) in by_zone {
        println!("  {zone}: {count}");
    }
    for (decision, count) in by_decision {
        println!("  {decision}: {count}");
    }
}

// `t4_terminal_access_column_gate_failures` moved to support::tier

pub(crate) fn load_t3_zone_route_columns(path: &Path) -> Result<Vec<T3ZoneRouteColumnRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t4_terminal_access_columns(path: &Path) -> Result<Vec<T4TerminalAccessColumnRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t4_terminal_contact_evidence_rows(
    terminal_rows: &[T4TerminalAccessColumnRow],
) -> Vec<T4TerminalContactEvidenceRow> {
    let mut rows = terminal_rows
        .iter()
        .filter(|row| row.column_decision == "terminal-review")
        .map(|row| T4TerminalContactEvidenceRow {
            queue_id: format!(
                "T4CONTACT-{}-{}",
                canonical_route_key(&row.zone_id),
                canonical_route_key(&row.route)
            ),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            terminal_district_seed: terminal_district_seed_for_row(row),
            terminal_district_seed_source: "data/intermodal_terminals.csv".to_string(),
            contact_basis: terminal_contact_basis_for_row(row),
            contact_proof_source: String::new(),
            evidence_status: "source-needed".to_string(),
            selected_higher_tier_attachment: "source-needed".to_string(),
            decision: "source-needed".to_string(),
            next_artifact: terminal_contact_next_artifact(&row.zone_id),
            source_column_artifact: "data/t4-terminal-access-columns.csv".to_string(),
            source_column_decision: row.column_decision.clone(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();

    rows.sort_by(|a, b| {
        a.zone_id
            .cmp(&b.zone_id)
            .then_with(|| a.decision.cmp(&b.decision))
            .then_with(|| a.route.cmp(&b.route))
    });
    rows
}

pub(crate) fn terminal_district_seed_for_row(row: &T4TerminalAccessColumnRow) -> String {
    terminal_seed_for_zone_route(&row.zone_id, &row.route)
        .unwrap_or_else(|| terminal_district_seed(&row.terminal_obligation))
}

pub(crate) fn terminal_district_seed(terminal_obligation: &str) -> String {
    terminal_obligation
        .split_once(": ")
        .map(|(_, seed)| seed.to_string())
        .unwrap_or_else(|| terminal_obligation.to_string())
}

pub(crate) fn great_lakes_terminal_seed_for_route(route: &str) -> Option<String> {
    let district = match canonical_route_key(route).as_str() {
        "I115" | "I176" | "I294" | "US41" => "Chicago Intermodal Complex",
        "I129" | "I465" | "US31" => "Indianapolis Avon",
        "I180" | "I72" | "US42" => "St. Louis Gateway",
        "I190" | "I390" | "I478" | "I691" | "I990" | "US7" => "New York Fresh Pond",
        "I196" | "I496" | "I696" | "US10" | "US223" => "Detroit Livernois",
        "I235" => "Minneapolis Twin Cities",
        "I271" | "I471" | "US22" | "US35" | "US224" | "US250" | "US74" => "Columbus South",
        "I276" | "I93" | "US15" => "Philadelphia Frankford",
        "I279" => "Columbus South",
        _ => return None,
    };
    Some(district.to_string())
}

pub(crate) fn terminal_seed_for_zone_route(zone_id: &str, route: &str) -> Option<String> {
    let key = canonical_route_key(route);
    let district = match zone_id {
        "t3-great-lakes" => return great_lakes_terminal_seed_for_route(route),
        "t3-southeast" => match key.as_str() {
            "I140" | "US301" => "Savannah Garden City",
            "I175" => "Miami Hialeah",
            "I185" | "US278" | "US84" => "Atlanta Hulsey",
            "I795" | "US119" => "Charlotte Intermodal",
            "US45E" | "US45W" | "US82" | "US90Z" => "New Orleans Gentilly",
            _ => return None,
        },
        "t3-mid-south" => match key.as_str() {
            "I169" | "US24" | "US66" => "Kansas City Gateway",
            "I181" | "I277" | "US421" => "Louisville KentuckyOne",
            "I255" => "St. Louis Gateway",
            "I759" | "I840" | "US167" | "US270" => "Memphis Intermodal",
            _ => return None,
        },
        "t3-mountain-west" => match key.as_str() {
            "I135" | "I335" | "US76" => "Kansas City Gateway",
            "I705" => "Seattle BNSF",
            "I880" => "Los Angeles/Long Beach",
            "US14" | "US95" => "Salt Lake City",
            "US26" => "Portland Albina",
            "US87" => "Denver Logistics Hub",
            _ => return None,
        },
        "t3-texas-border" => match key.as_str() {
            "I510" => "New Orleans Gentilly",
            "I69E" | "US281" => "San Antonio Kirby",
            "US96" => "Houston Englewood",
            _ => return None,
        },
        _ => return None,
    };
    Some(district.to_string())
}

pub(crate) fn terminal_contact_basis_for_row(row: &T4TerminalAccessColumnRow) -> String {
    if terminal_seed_for_zone_route(&row.zone_id, &row.route).is_some() {
        "candidate-terminal-district-assigned; route-to-terminal contact source still needed"
            .to_string()
    } else {
        "source-needed-route-to-terminal-contact".to_string()
    }
}

pub(crate) fn terminal_contact_next_artifact(zone_id: &str) -> String {
    match zone_id {
        "t3-great-lakes" => {
            "waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-02.md".to_string()
        }
        _ => "waves/2026-05-13-t4-terminal-contact-evidence/plans/pulse-03.md".to_string(),
    }
}

pub(crate) fn write_t4_terminal_contact_evidence(
    path: &Path,
    rows: &[T4TerminalContactEvidenceRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_contact_evidence_summary(
    output: &Path,
    rows: &[T4TerminalContactEvidenceRow],
) {
    let mut by_decision = std::collections::BTreeMap::<&str, usize>::new();
    let mut by_zone = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_decision.entry(row.decision.as_str()).or_default() += 1;
        *by_zone.entry(row.zone_id.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal contact evidence rows to {}",
        rows.len(),
        output.display()
    );
    for (zone, count) in by_zone {
        println!("  {zone}: {count}");
    }
    for (decision, count) in by_decision {
        println!("  {decision}: {count}");
    }
}

// `t4_terminal_contact_evidence_gate_failures` moved to support::tier

pub(crate) fn t4_terminal_access_evidence_review_rows(
    contact_rows: &[T4TerminalContactEvidenceRow],
) -> Vec<T4TerminalAccessEvidenceReviewRow> {
    let mut rows = contact_rows
        .iter()
        .map(|row| T4TerminalAccessEvidenceReviewRow {
            review_id: format!("T4ACCESSREVIEW-{}", stable_id_fragment(&row.queue_id)),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            terminal_district_seed: row.terminal_district_seed.clone(),
            terminal_district_seed_source: row.terminal_district_seed_source.clone(),
            evidence_status_before: row.evidence_status.clone(),
            review_decision: "held-source-needed".to_string(),
            review_reason:
                "terminal district seed assignment is not contact proof; non-seed source artifact still required"
                    .to_string(),
            source_action: "route-to-terminal-access-proof-acquisition".to_string(),
            blocker_claims_before: "map;publication;upgrade".to_string(),
            blocker_claims_after: "map;publication;upgrade".to_string(),
            claim_blocker_delta: 0,
            next_artifact: row.next_artifact.clone(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t4_terminal_access_evidence_review(
    path: &Path,
    rows: &[T4TerminalAccessEvidenceReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_access_evidence_review_summary(
    output: &Path,
    rows: &[T4TerminalAccessEvidenceReviewRow],
) {
    let mut by_zone = std::collections::BTreeMap::<&str, usize>::new();
    let mut by_decision = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_zone.entry(row.zone_id.as_str()).or_default() += 1;
        *by_decision.entry(row.review_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access evidence review rows to {}",
        rows.len(),
        output.display()
    );
    for (zone, count) in by_zone {
        println!("  {zone}: {count}");
    }
    for (decision, count) in by_decision {
        println!("  {decision}: {count}");
    }
}

// `t4_terminal_access_evidence_review_gate_failures` moved to support::tier

pub(crate) fn load_t4_terminal_contact_evidence(path: &Path) -> Result<Vec<T4TerminalContactEvidenceRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t4_terminal_access_evidence_review(
    path: &Path,
) -> Result<Vec<T4TerminalAccessEvidenceReviewRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t4_terminal_access_proof_acquisition_rows(
    review_rows: &[T4TerminalAccessEvidenceReviewRow],
) -> Vec<T4TerminalAccessProofAcquisitionRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| row.review_decision == "held-source-needed")
        .map(|row| T4TerminalAccessProofAcquisitionRow {
            acquisition_id: format!("T4ACCESSACQ-{}", stable_id_fragment(&row.review_id)),
            review_id: row.review_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            terminal_district_seed: row.terminal_district_seed.clone(),
            required_proof:
                "non-seed route-to-terminal contact source with route, terminal, connector, and date"
                    .to_string(),
            prohibited_seed_source: row.terminal_district_seed_source.clone(),
            acquisition_status: "source-needed".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            proof_artifact_status: "not-attached".to_string(),
            next_artifact: "data/t4-terminal-access-proof-artifacts.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t4_terminal_access_proof_acquisition(
    path: &Path,
    rows: &[T4TerminalAccessProofAcquisitionRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_access_proof_acquisition_summary(
    output: &Path,
    rows: &[T4TerminalAccessProofAcquisitionRow],
) {
    let mut by_zone = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_zone.entry(row.zone_id.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access proof acquisition rows to {}",
        rows.len(),
        output.display()
    );
    for (zone, count) in by_zone {
        println!("  {zone}: {count}");
    }
}

// `t4_terminal_access_proof_acquisition_gate_failures` moved to support::tier

pub(crate) fn load_t4_terminal_access_proof_acquisition(
    path: &Path,
) -> Result<Vec<T4TerminalAccessProofAcquisitionRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t4_terminal_access_proof_artifact_rows(
    acquisition_rows: &[T4TerminalAccessProofAcquisitionRow],
) -> Vec<T4TerminalAccessProofArtifactRow> {
    let mut rows = acquisition_rows
        .iter()
        .filter(|row| row.proof_artifact_status == "not-attached")
        .map(|row| T4TerminalAccessProofArtifactRow {
            proof_artifact_id: format!(
                "T4ACCESSARTIFACT-{}",
                stable_id_fragment(&row.acquisition_id)
            ),
            acquisition_id: row.acquisition_id.clone(),
            review_id: row.review_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            terminal_district_seed: row.terminal_district_seed.clone(),
            required_proof: row.required_proof.clone(),
            source_artifact_reference: "source-needed".to_string(),
            attachment_status: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/t4-terminal-access-proof-review.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t4_terminal_access_proof_artifacts(
    path: &Path,
    rows: &[T4TerminalAccessProofArtifactRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_access_proof_artifacts_summary(
    output: &Path,
    rows: &[T4TerminalAccessProofArtifactRow],
) {
    let mut by_zone = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_zone.entry(row.zone_id.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access proof artifact rows to {}",
        rows.len(),
        output.display()
    );
    for (zone, count) in by_zone {
        println!("  {zone}: {count}");
    }
}

// `t4_terminal_access_proof_artifact_gate_failures` moved to support::tier

pub(crate) fn load_t4_terminal_access_proof_artifacts(
    path: &Path,
) -> Result<Vec<T4TerminalAccessProofArtifactRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t4_terminal_access_proof_review_rows(
    artifact_rows: &[T4TerminalAccessProofArtifactRow],
) -> Vec<T4TerminalAccessProofReviewRow> {
    let mut rows = artifact_rows
        .iter()
        .filter(|row| row.source_artifact_reference == "source-needed")
        .map(|row| T4TerminalAccessProofReviewRow {
            proof_review_id: format!(
                "T4ACCESSREVIEWPROOF-{}",
                stable_id_fragment(&row.proof_artifact_id)
            ),
            proof_artifact_id: row.proof_artifact_id.clone(),
            acquisition_id: row.acquisition_id.clone(),
            review_id: row.review_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            source_artifact_reference: row.source_artifact_reference.clone(),
            review_decision: "held-no-source-artifact".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            optimization_return_status: "return-to-optimizer-held-known".to_string(),
            review_reason:
                "proof artifact remains source-needed; terminal-access proof cannot be accepted"
                    .to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/tier-optimizer-runs.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t4_terminal_access_proof_review(
    path: &Path,
    rows: &[T4TerminalAccessProofReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_access_proof_review_summary(
    output: &Path,
    rows: &[T4TerminalAccessProofReviewRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.review_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access proof review rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

// `t4_terminal_access_proof_review_gate_failures` moved to support::tier

pub(crate) fn load_t4_terminal_access_proof_review(
    path: &Path,
) -> Result<Vec<T4TerminalAccessProofReviewRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t4_terminal_access_source_access_rows(
    review_rows: &[T4TerminalAccessProofReviewRow],
) -> Vec<T4TerminalAccessSourceAccessRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| row.review_decision == "held-no-source-artifact")
        .map(|row| T4TerminalAccessSourceAccessRow {
            source_access_id: format!(
                "T4ACCESSSOURCE-{}",
                stable_id_fragment(&row.proof_review_id)
            ),
            proof_review_id: row.proof_review_id.clone(),
            proof_artifact_id: row.proof_artifact_id.clone(),
            acquisition_id: row.acquisition_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            source_owner: "terminal operator, port authority, state DOT, or public terminal map".to_string(),
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
            proof_acceptance_status: row.proof_acceptance_status.clone(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/t4-terminal-access-proof-artifacts.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t4_terminal_access_source_access(
    path: &Path,
    rows: &[T4TerminalAccessSourceAccessRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_access_source_access_summary(
    output: &Path,
    rows: &[T4TerminalAccessSourceAccessRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.access_mode.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access source access rows to {}",
        rows.len(),
        output.display()
    );
    for (mode, count) in counts {
        println!("  {mode}: {count}");
    }
}

// `t4_terminal_access_source_access_gate_failures` moved to support::tier

pub(crate) fn load_t4_terminal_access_source_access(
    path: &Path,
) -> Result<Vec<T4TerminalAccessSourceAccessRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t4_terminal_access_proof_intake_rows(
    access_rows: &[T4TerminalAccessSourceAccessRow],
) -> Vec<T4TerminalAccessProofIntakeRow> {
    let mut rows = access_rows
        .iter()
        .filter(|row| row.evidence_artifact == "source-needed")
        .map(|row| T4TerminalAccessProofIntakeRow {
            proof_intake_id: format!(
                "T4ACCESSINTAKE-{}",
                stable_id_fragment(&row.source_access_id)
            ),
            source_access_id: row.source_access_id.clone(),
            proof_review_id: row.proof_review_id.clone(),
            proof_artifact_id: row.proof_artifact_id.clone(),
            acquisition_id: row.acquisition_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
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
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/t4-terminal-access-proof-artifacts.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t4_terminal_access_proof_intake(
    path: &Path,
    rows: &[T4TerminalAccessProofIntakeRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_access_proof_intake_summary(
    output: &Path,
    rows: &[T4TerminalAccessProofIntakeRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.proof_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access proof intake rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

// `t4_terminal_access_proof_intake_gate_failures` moved to support::tier

pub(crate) fn load_t4_terminal_access_proof_intake(
    path: &Path,
) -> Result<Vec<T4TerminalAccessProofIntakeRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t4_terminal_access_proof_source_capture_rows(
    intake_rows: &[T4TerminalAccessProofIntakeRow],
) -> Vec<T4TerminalAccessProofSourceCaptureRow> {
    let mut rows = intake_rows
        .iter()
        .filter(|row| row.proof_artifact == "source-needed")
        .map(|row| T4TerminalAccessProofSourceCaptureRow {
            source_capture_id: format!(
                "T4ACCESSCAPTURE-{}",
                stable_id_fragment(&row.proof_intake_id)
            ),
            proof_intake_id: row.proof_intake_id.clone(),
            source_access_id: row.source_access_id.clone(),
            proof_artifact_id: row.proof_artifact_id.clone(),
            acquisition_id: row.acquisition_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            source_artifact_reference: "source-needed".to_string(),
            source_artifact_type: "manual-or-cached-terminal-access-proof".to_string(),
            capture_status: "source-needed".to_string(),
            evidence_acceptance_status: "not-reviewed".to_string(),
            capture_blocker:
                "manual or cached non-seed terminal-access source artifact has not been attached"
                    .to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/t4-terminal-access-proof-artifacts.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t4_terminal_access_proof_source_capture(
    path: &Path,
    rows: &[T4TerminalAccessProofSourceCaptureRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_access_proof_source_capture_summary(
    output: &Path,
    rows: &[T4TerminalAccessProofSourceCaptureRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.capture_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access proof source-capture rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

// `t4_terminal_access_proof_source_capture_gate_failures` moved to support::tier

pub(crate) fn load_t4_terminal_access_proof_source_capture(
    path: &Path,
) -> Result<Vec<T4TerminalAccessProofSourceCaptureRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t4_terminal_access_proof_artifact_attachment_rows(
    capture_rows: &[T4TerminalAccessProofSourceCaptureRow],
) -> Vec<T4TerminalAccessProofArtifactAttachmentRow> {
    let mut rows = capture_rows
        .iter()
        .filter(|row| row.source_artifact_reference == "source-needed")
        .map(|row| T4TerminalAccessProofArtifactAttachmentRow {
            artifact_attachment_id: format!(
                "T4ACCESSATTACH-{}",
                stable_id_fragment(&row.source_capture_id)
            ),
            source_capture_id: row.source_capture_id.clone(),
            proof_intake_id: row.proof_intake_id.clone(),
            proof_artifact_id: row.proof_artifact_id.clone(),
            acquisition_id: row.acquisition_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            source_artifact_reference: "source-needed".to_string(),
            attachment_status: "source-needed".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            attachment_blocker:
                "manual or cached non-seed terminal-access proof artifact has not been attached"
                    .to_string(),
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/t4-terminal-access-proof-attachment-review.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t4_terminal_access_proof_artifact_attachment(
    path: &Path,
    rows: &[T4TerminalAccessProofArtifactAttachmentRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_access_proof_artifact_attachment_summary(
    output: &Path,
    rows: &[T4TerminalAccessProofArtifactAttachmentRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.attachment_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access proof artifact-attachment rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

// `t4_terminal_access_proof_artifact_attachment_gate_failures` moved to support::tier

pub(crate) fn load_t4_terminal_access_proof_artifact_attachment(
    path: &Path,
) -> Result<Vec<T4TerminalAccessProofArtifactAttachmentRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t4_terminal_access_proof_attachment_review_rows(
    attachment_rows: &[T4TerminalAccessProofArtifactAttachmentRow],
) -> Vec<T4TerminalAccessProofAttachmentReviewRow> {
    let mut rows = attachment_rows
        .iter()
        .filter(|row| {
            row.source_artifact_reference == "source-needed"
                && row.attachment_status == "source-needed"
                && row.evidence_review_status == "not-reviewed"
                && row.proof_acceptance_status == "not-accepted"
                && row.validation_status == "review"
        })
        .map(|row| T4TerminalAccessProofAttachmentReviewRow {
            attachment_review_id: format!(
                "T4ACCESSATTACHREVIEW-{}",
                stable_id_fragment(&row.artifact_attachment_id)
            ),
            artifact_attachment_id: row.artifact_attachment_id.clone(),
            source_capture_id: row.source_capture_id.clone(),
            proof_intake_id: row.proof_intake_id.clone(),
            proof_artifact_id: row.proof_artifact_id.clone(),
            acquisition_id: row.acquisition_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            source_artifact_reference: row.source_artifact_reference.clone(),
            review_decision: "held-no-source-artifact".to_string(),
            evidence_review_status: "not-reviewed".to_string(),
            proof_acceptance_status: "not-accepted".to_string(),
            optimization_return_status: "return-to-optimizer-held-known".to_string(),
            review_reason:
                "proof artifact attachment remains source-needed; terminal-access proof cannot be accepted"
                    .to_string(),
            blocker_claims_before: row.blocker_claims_before.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_artifact: "data/optimizer-residual-blocker-backlog.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t4_terminal_access_proof_attachment_review(
    path: &Path,
    rows: &[T4TerminalAccessProofAttachmentReviewRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_access_proof_attachment_review_summary(
    output: &Path,
    rows: &[T4TerminalAccessProofAttachmentReviewRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.review_decision.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access proof attachment-review rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in counts {
        println!("  {decision}: {count}");
    }
}

// `t4_terminal_access_proof_attachment_review_gate_failures` moved to support::tier

pub(crate) fn load_t4_terminal_access_proof_attachment_review(
    path: &Path,
) -> Result<Vec<T4TerminalAccessProofAttachmentReviewRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t4_terminal_access_proof_artifact_acquisition_target_rows(
    review_rows: &[T4TerminalAccessProofAttachmentReviewRow],
) -> Vec<T4TerminalAccessProofArtifactAcquisitionTargetRow> {
    let mut rows = review_rows
        .iter()
        .filter(|row| {
            row.source_artifact_reference == "source-needed"
                && row.review_decision == "held-no-source-artifact"
                && row.proof_acceptance_status == "not-accepted"
                && row.validation_status == "review"
        })
        .map(|row| T4TerminalAccessProofArtifactAcquisitionTargetRow {
            acquisition_target_id: format!(
                "T4ACCESSARTIFACTTARGET-{}",
                stable_id_fragment(&row.attachment_review_id)
            ),
            attachment_review_id: row.attachment_review_id.clone(),
            artifact_attachment_id: row.artifact_attachment_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
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
            blocker_claims_before: row.blocker_claims_after.clone(),
            blocker_claims_after: row.blocker_claims_after.clone(),
            claim_blocker_delta: 0,
            next_action: "acquire or cache non-seed route-to-terminal proof artifact".to_string(),
            next_artifact:
                "data/t4-terminal-access-proof-artifact-acquisition-targets.csv".to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t4_terminal_access_proof_artifact_acquisition_targets(
    path: &Path,
    rows: &[T4TerminalAccessProofArtifactAcquisitionTargetRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_access_proof_artifact_acquisition_target_summary(
    output: &Path,
    rows: &[T4TerminalAccessProofArtifactAcquisitionTargetRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.acquisition_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access proof artifact acquisition-target rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

// `t4_terminal_access_proof_artifact_acquisition_target_gate_failures` moved to support::tier

pub(crate) fn load_t4_terminal_access_proof_artifact_acquisition_targets(
    path: &Path,
) -> Result<Vec<T4TerminalAccessProofArtifactAcquisitionTargetRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t4_terminal_access_proof_artifact_source_access_rows` moved to support::tier

pub(crate) fn write_t4_terminal_access_proof_artifact_source_access(
    path: &Path,
    rows: &[T4TerminalAccessProofArtifactSourceAccessRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_access_proof_artifact_source_access_summary(
    output: &Path,
    rows: &[T4TerminalAccessProofArtifactSourceAccessRow],
) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.access_mode.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal access proof artifact source-access rows to {}",
        rows.len(),
        output.display()
    );
    for (mode, count) in counts {
        println!("  {mode}: {count}");
    }
}

// `t4_terminal_access_proof_artifact_source_access_gate_failures` moved to support::tier

pub(crate) fn t4_terminal_contact_source_plan_rows(
    contact_rows: &[T4TerminalContactEvidenceRow],
) -> Vec<T4TerminalContactSourcePlanRow> {
    let mut rows = contact_rows
        .iter()
        .filter(|row| row.decision == "source-needed")
        .map(|row| T4TerminalContactSourcePlanRow {
            plan_id: format!("T4SOURCEPLAN-{}", stable_id_fragment(&row.queue_id)),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            terminal_district: row.terminal_district_seed.clone(),
            terminal_district_seed_source: row.terminal_district_seed_source.clone(),
            contact_proof_source_family: "public-terminal-contact-proof".to_string(),
            contact_proof_source_artifact: "source-needed".to_string(),
            required_proof_fields:
                "route; terminal district; route-to-terminal contact statement; source title; source url or cached artifact; capture date"
                    .to_string(),
            acquisition_status: "source-needed".to_string(),
            proof_blocker:
                "terminal district seed is not route-to-terminal contact proof; acquire separate source"
                    .to_string(),
            next_artifact:
                "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-02.md"
                    .to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();

    rows.sort_by(|a, b| {
        a.terminal_district
            .cmp(&b.terminal_district)
            .then_with(|| a.route.cmp(&b.route))
            .then_with(|| a.queue_id.cmp(&b.queue_id))
    });
    rows
}

pub(crate) fn write_t4_terminal_contact_source_plan(
    path: &Path,
    rows: &[T4TerminalContactSourcePlanRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_contact_source_plan_summary(
    output: &Path,
    rows: &[T4TerminalContactSourcePlanRow],
) {
    let mut by_district = std::collections::BTreeMap::<&str, usize>::new();
    let mut by_status = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_district
            .entry(row.terminal_district.as_str())
            .or_default() += 1;
        *by_status
            .entry(row.acquisition_status.as_str())
            .or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal contact source plan rows to {}",
        rows.len(),
        output.display()
    );
    for (district, count) in by_district {
        println!("  {district}: {count}");
    }
    for (status, count) in by_status {
        println!("  {status}: {count}");
    }
}

pub(crate) fn t4_terminal_contact_source_catalog_rows(
    plan_rows: &[T4TerminalContactSourcePlanRow],
) -> Vec<T4TerminalContactSourceCatalogRow> {
    let mut districts = std::collections::BTreeMap::<String, usize>::new();
    for row in plan_rows {
        *districts.entry(row.terminal_district.clone()).or_default() += 1;
    }

    districts
        .into_iter()
        .map(|(terminal_district, route_task_count)| T4TerminalContactSourceCatalogRow {
            catalog_id: format!(
                "T4SOURCECATALOG-{}",
                canonical_route_key(&terminal_district)
            ),
            terminal_district,
            route_task_count,
            source_family: "public-terminal-contact-proof".to_string(),
            source_access_mode: "manual-or-cached-source-needed".to_string(),
            required_proof_fields:
                "route; terminal district; route-to-terminal contact statement; source title; source url or cached artifact; capture date"
                    .to_string(),
            acquisition_status: "source-needed".to_string(),
            proof_blocker:
                "no safe live fetcher or cached contact proof source is registered for this district"
                    .to_string(),
            cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                .to_string(),
            next_artifact:
                "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-03.md"
                    .to_string(),
            validation_status: "review".to_string(),
        })
        .collect()
}

pub(crate) fn write_t4_terminal_contact_source_catalog(
    path: &Path,
    rows: &[T4TerminalContactSourceCatalogRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_contact_source_catalog_summary(
    output: &Path,
    rows: &[T4TerminalContactSourceCatalogRow],
) {
    println!(
        "  wrote {} T4 terminal contact source catalog rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {}: {} route tasks ({})",
            row.terminal_district, row.route_task_count, row.acquisition_status
        );
    }
}

// `t4_terminal_contact_source_plan_gate_failures` moved to support::tier

// `t4_terminal_contact_source_catalog_gate_failures` moved to support::tier

// `t4_terminal_contact_proof_docket_rows` moved to support::tier

pub(crate) fn write_t4_terminal_contact_proof_docket(
    path: &Path,
    rows: &[T4TerminalContactProofDocketRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_contact_proof_docket_summary(
    output: &Path,
    rows: &[T4TerminalContactProofDocketRow],
) {
    let mut by_status = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_status.entry(row.proof_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal contact proof docket rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in by_status {
        println!("  {status}: {count}");
    }
}

// `t4_terminal_contact_proof_docket_gate_failures` moved to support::tier::t4_terminal_contact_proof_docket_gate_failures

pub(crate) fn t4_terminal_contact_proof_artifact_contract_rows(
) -> Vec<T4TerminalContactProofArtifactContractRow> {
    vec![T4TerminalContactProofArtifactContractRow {
        contract_id: "T4CONTACT-PROOF-CONTRACT-001".to_string(),
        source_family: "public-terminal-contact-proof".to_string(),
        accepted_proof_status: "source-backed".to_string(),
        required_fields:
            "route; terminal district; route-to-terminal contact statement; source title; source url or cached artifact; capture date; selected higher-tier attachment; validation decision"
                .to_string(),
        allowed_artifact_modes: "manual-citation;cached-source-artifact".to_string(),
        prohibited_sources: "data/intermodal_terminals.csv;terminal district seed;route proximity;district membership"
            .to_string(),
        promotion_rule:
            "source-backed requires a non-seed source artifact naming route terminal district contact statement source title url-or-cache capture date selected higher-tier attachment and validation decision"
                .to_string(),
        source_needed_decision:
            "missing proof artifact remains source-needed and review".to_string(),
        blocked_decision:
            "inaccessible or policy-unsupported source remains blocked with blocker text".to_string(),
        rejected_decision:
            "artifact that does not name route-to-terminal contact remains rejected and cannot feed scenario readiness"
                .to_string(),
        next_artifact:
            "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-02.md"
                .to_string(),
        validation_status: "pass".to_string(),
    }]
}

pub(crate) fn write_t4_terminal_contact_proof_artifact_contract(
    path: &Path,
    rows: &[T4TerminalContactProofArtifactContractRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_contact_proof_artifact_contract_summary(
    output: &Path,
    rows: &[T4TerminalContactProofArtifactContractRow],
) {
    println!(
        "  wrote {} terminal contact proof artifact contract rows to {}",
        rows.len(),
        output.display()
    );
}

// `t4_terminal_contact_proof_artifact_contract_gate_failures` moved to support::tier

// `t4_terminal_contact_proof_source_registry_rows` moved to support::tier

pub(crate) fn write_t4_terminal_contact_proof_source_registry(
    path: &Path,
    rows: &[T4TerminalContactProofSourceRegistryRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_contact_proof_source_registry_summary(
    output: &Path,
    rows: &[T4TerminalContactProofSourceRegistryRow],
) {
    let mut by_status = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_status.entry(row.registry_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} terminal contact proof source registry rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in by_status {
        println!("  {status}: {count}");
    }
}

// `t4_terminal_contact_proof_source_registry_gate_failures` moved to support::tier::t4_terminal_contact_proof_source_registry_gate_failures

pub(crate) fn load_t4_terminal_contact_proof_source_registry(
    path: &Path,
) -> Result<Vec<T4TerminalContactProofSourceRegistryRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t4_terminal_contact_accepted_proof_sources(
    path: &Path,
) -> Result<Vec<T4TerminalContactAcceptedProofSourceRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t4_terminal_contact_rejected_proof_sources(
    path: &Path,
) -> Result<Vec<T4TerminalContactRejectedProofSourceRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t4_terminal_contact_district_proof_import_rows` moved to support::tier

pub(crate) fn largest_registry_district(
    registry_rows: &[T4TerminalContactProofSourceRegistryRow],
) -> Option<String> {
    let mut counts = std::collections::BTreeMap::<String, usize>::new();
    for row in registry_rows {
        *counts.entry(row.terminal_district.clone()).or_default() += 1;
    }
    counts
        .into_iter()
        .max_by(|left, right| left.1.cmp(&right.1).then_with(|| right.0.cmp(&left.0)))
        .map(|(district, _)| district)
}

pub(crate) fn write_t4_terminal_contact_district_proof_import(
    path: &Path,
    rows: &[T4TerminalContactDistrictProofImportRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_contact_district_proof_import_summary(
    output: &Path,
    rows: &[T4TerminalContactDistrictProofImportRow],
) {
    let mut by_status = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_status.entry(row.import_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} terminal contact district proof import rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in by_status {
        println!("  {status}: {count}");
    }
}

// `t4_terminal_contact_district_proof_import_gate_failures` moved to support::tier

pub(crate) fn load_t4_terminal_contact_proof_docket(
    path: &Path,
) -> Result<Vec<T4TerminalContactProofDocketRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t4_terminal_contact_district_proof_import(
    path: &Path,
) -> Result<Vec<T4TerminalContactDistrictProofImportRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn accepted_t4_terminal_proof_route_set(
    rows: &[T4TerminalContactDistrictProofImportRow],
) -> std::collections::BTreeSet<String> {
    rows.iter()
        .filter(|row| {
            row.import_status == "accepted"
                && row.proof_decision == "source-backed"
                && row.validation_status == "pass"
        })
        .map(|row| route_display_key(&row.route))
        .collect()
}

pub(crate) fn rejected_t4_terminal_proof_route_set(
    rows: &[T4TerminalContactRejectedProofSourceRow],
) -> std::collections::BTreeSet<String> {
    rows.iter()
        .filter(|row| {
            row.rejection_status == "route-not-supported-by-terminal-access-source"
                && row.validation_status == "pass"
        })
        .map(|row| route_display_key(&row.route))
        .collect()
}

pub(crate) fn t4_terminal_columbus_proof_intake_rows(
    proof_rows: &[T4TerminalContactProofDocketRow],
) -> Vec<T4TerminalColumbusProofIntakeRow> {
    let mut rows = proof_rows
        .iter()
        .filter(|row| {
            row.terminal_district == "Columbus South" && row.proof_status == "source-needed"
        })
        .map(|row| T4TerminalColumbusProofIntakeRow {
            intake_id: format!("T4COLUMBUS-{}", stable_id_fragment(&row.queue_id)),
            task_id: row.task_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            zone_id: row.zone_id.clone(),
            terminal_district: row.terminal_district.clone(),
            source_family: row.source_family.clone(),
            required_proof_field: row.required_proof_field.clone(),
            selected_higher_tier_attachment_requirement: row
                .selected_higher_tier_attachment_requirement
                .clone(),
            contact_proof_source_artifact: row.contact_proof_source_artifact.clone(),
            proof_status: row.proof_status.clone(),
            proof_blocker: row.proof_blocker.clone(),
            next_artifact:
                "waves/2026-05-13-columbus-south-terminal-contact-proof/plans/pulse-02.md"
                    .to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();

    rows.sort_by(|a, b| {
        a.route
            .cmp(&b.route)
            .then_with(|| a.queue_id.cmp(&b.queue_id))
    });
    rows
}

pub(crate) fn write_t4_terminal_columbus_proof_intake(
    path: &Path,
    rows: &[T4TerminalColumbusProofIntakeRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_columbus_proof_intake_summary(
    output: &Path,
    rows: &[T4TerminalColumbusProofIntakeRow],
) {
    let mut by_status = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_status.entry(row.proof_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} Columbus South proof intake rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in by_status {
        println!("  {status}: {count}");
    }
}

// `t4_terminal_columbus_proof_intake_gate_failures` moved to support::tier

pub(crate) fn load_t4_terminal_columbus_proof_intake(
    path: &Path,
) -> Result<Vec<T4TerminalColumbusProofIntakeRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t4_terminal_columbus_source_access_rows(
    intake_rows: &[T4TerminalColumbusProofIntakeRow],
) -> Vec<T4TerminalColumbusSourceAccessRow> {
    let mut rows = intake_rows
        .iter()
        .map(|row| T4TerminalColumbusSourceAccessRow {
            access_id: format!("T4COLUMBUSACCESS-{}", stable_id_fragment(&row.queue_id)),
            intake_id: row.intake_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            terminal_district: row.terminal_district.clone(),
            source_family: row.source_family.clone(),
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
        })
        .collect::<Vec<_>>();

    rows.sort_by(|a, b| {
        a.route
            .cmp(&b.route)
            .then_with(|| a.queue_id.cmp(&b.queue_id))
    });
    rows
}

pub(crate) fn write_t4_terminal_columbus_source_access(
    path: &Path,
    rows: &[T4TerminalColumbusSourceAccessRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_columbus_source_access_summary(
    output: &Path,
    rows: &[T4TerminalColumbusSourceAccessRow],
) {
    let mut by_status = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_status
            .entry(row.acquisition_status.as_str())
            .or_default() += 1;
    }
    println!(
        "  wrote {} Columbus South source access rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in by_status {
        println!("  {status}: {count}");
    }
}

// `t4_terminal_columbus_source_access_gate_failures` moved to support::tier::t4_terminal_columbus_source_access_gate_failures

pub(crate) fn load_t4_terminal_columbus_source_access(
    path: &Path,
) -> Result<Vec<T4TerminalColumbusSourceAccessRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn t4_terminal_columbus_proof_attempt_rows(
    source_access_rows: &[T4TerminalColumbusSourceAccessRow],
) -> Vec<T4TerminalColumbusProofAttemptRow> {
    let mut rows = source_access_rows
        .iter()
        .map(|row| T4TerminalColumbusProofAttemptRow {
            attempt_id: format!("T4COLUMBUSATTEMPT-{}", stable_id_fragment(&row.queue_id)),
            access_id: row.access_id.clone(),
            intake_id: row.intake_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            terminal_district: row.terminal_district.clone(),
            source_family: row.source_family.clone(),
            source_artifact: row.contact_proof_source_artifact.clone(),
            capture_status: "not-attempted-live-fetch-unsupported".to_string(),
            contact_statement_status: "source-needed".to_string(),
            selected_higher_tier_attachment_status: "source-needed".to_string(),
            proof_attempt_status: "blocked".to_string(),
            proof_decision: "source-needed".to_string(),
            proof_blocker: row.source_access_blocker.clone(),
            next_artifact:
                "waves/2026-05-13-columbus-south-terminal-contact-proof/plans/pulse-04.md"
                    .to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.route.cmp(&right.route));
    rows
}

pub(crate) fn write_t4_terminal_columbus_proof_attempts(
    path: &Path,
    rows: &[T4TerminalColumbusProofAttemptRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_columbus_proof_attempt_summary(
    output: &Path,
    rows: &[T4TerminalColumbusProofAttemptRow],
) {
    let mut by_status = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_status
            .entry(row.proof_attempt_status.as_str())
            .or_default() += 1;
    }
    println!(
        "  wrote {} Columbus South proof attempt rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in by_status {
        println!("  {status}: {count}");
    }
}

// `t4_terminal_columbus_proof_attempt_gate_failures` moved to support::tier

// `t4_terminal_scenario_readiness_rows` moved to support::tier

pub(crate) fn terminal_scenario_rationale(row: &T4TerminalContactEvidenceRow) -> String {
    format!(
        "source-backed contact between {} and {}; preserves T4 access while selecting scenario scope",
        row.route, row.terminal_district_seed
    )
}

pub(crate) fn write_t4_terminal_scenario_readiness(
    path: &Path,
    rows: &[T4TerminalScenarioReadinessRow],
) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t4_terminal_scenario_readiness_summary(
    output: &Path,
    rows: &[T4TerminalScenarioReadinessRow],
) {
    let mut by_decision = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_decision
            .entry(row.scenario_decision.as_str())
            .or_default() += 1;
    }
    println!(
        "  wrote {} T4 terminal scenario readiness rows to {}",
        rows.len(),
        output.display()
    );
    for (decision, count) in by_decision {
        println!("  {decision}: {count}");
    }
}

// `t4_terminal_scenario_readiness_gate_failures` moved to support::tier

// `t3_t4_access_gap_rows` moved to support::tier

pub(crate) fn write_t3_t4_access_gaps(path: &Path, rows: &[T3T4AccessGapRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t3_t4_access_gap_summary(output: &Path, rows: &[T3T4AccessGapRow]) {
    let mut by_class = std::collections::BTreeMap::<&str, usize>::new();
    let mut by_surface = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_class.entry(row.gap_class.as_str()).or_default() += 1;
        *by_surface.entry(row.source_surface.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T3/T4 access gap rows to {}",
        rows.len(),
        output.display()
    );
    for (class, count) in by_class {
        println!("  {class}: {count}");
    }
    for (surface, count) in by_surface {
        println!("  {surface}: {count}");
    }
}

// `t3_t4_access_gap_gate_failures` moved to support::tier

pub(crate) fn load_t3_t4_access_gaps(path: &Path) -> Result<Vec<T3T4AccessGapRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t4_terminal_access_map_exclusion(
    path: &Path,
) -> Result<Vec<T4TerminalAccessMapExclusionRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn accepted_t4_terminal_access_map_exclusion(
    rows: &[T4TerminalAccessMapExclusionRow],
) -> Option<&T4TerminalAccessMapExclusionRow> {
    rows.iter().find(|row| {
        row.decision == "exclude-terminal-access-overlay-from-map-publication"
            && row.validation_status == "accepted"
            && row.affected_constraint_class == "terminal_access_evidence_gap"
            && row.affected_gap_class == "terminal-evidence-needed"
            && row.affected_tier == "T4"
            && row.excluded_claims == "map|publication"
            && !row.preserved_claims_after.trim().is_empty()
    })
}

pub(crate) fn load_t2_asset_condition_map_publication_exclusion(
    path: &Path,
) -> Result<Vec<T2AssetConditionMapPublicationExclusionRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn accepted_t2_asset_condition_map_publication_exclusion(
    rows: &[T2AssetConditionMapPublicationExclusionRow],
) -> Option<&T2AssetConditionMapPublicationExclusionRow> {
    rows.iter().find(|row| {
        row.decision == "exclude-asset-condition-debt-from-map-publication"
            && row.validation_status == "accepted"
            && row.affected_constraint_class == "asset_condition_debt"
            && row.affected_tier == "T2"
            && row.excluded_claims == "publication"
            && row.preserved_claims_after == "sla|transit|upgrade"
    })
}

// `t3_zone_map_diagnostic_rows` moved to support::tier

pub(crate) fn t3_zone_map_diagnostic_decision(
    selected_route_count: usize,
    access_gap_count: usize,
    zone_assignment_gap_count: usize,
) -> (&'static str, &'static str, &'static str) {
    if selected_route_count == 0 {
        return (
            "blocked-no-selected-feeders",
            "select at least one T3 feeder before rendering zone map",
            "review",
        );
    }
    if zone_assignment_gap_count > 0 {
        return (
            "review-zone-assignment-gaps",
            "render selected feeders but keep unassigned local access hidden",
            "review",
        );
    }
    if access_gap_count > 0 {
        return (
            "review-terminal-and-feeder-gaps",
            "render selected feeders with held access-gap callouts",
            "review",
        );
    }
    (
        "ready-for-zone-render",
        "render selected T3 feeder columns on zone map",
        "pass",
    )
}

pub(crate) fn write_t3_zone_map_diagnostics(path: &Path, rows: &[T3ZoneMapDiagnosticRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t3_zone_map_diagnostic_summary(output: &Path, rows: &[T3ZoneMapDiagnosticRow]) {
    let mut by_readiness = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_readiness.entry(row.map_readiness.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T3 zone map diagnostic rows to {}",
        rows.len(),
        output.display()
    );
    for (readiness, count) in by_readiness {
        println!("  {readiness}: {count}");
    }
}

// `t3_zone_map_diagnostic_gate_failures` moved to support::tier

pub(crate) fn load_t3_zone_map_diagnostics(path: &Path) -> Result<Vec<T3ZoneMapDiagnosticRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t3_zone_render_board_rows` moved to `t3_zone_render_board_rows.rs`

pub(crate) fn route_layer_rank(column_decision: &str) -> u8 {
    match column_decision {
        "selected" => 0,
        "upward-review" => 1,
        "review" => 2,
        _ => 3,
    }
}

pub(crate) fn board_layer_rank(board_layer: &str) -> u8 {
    match board_layer {
        "zone-summary" => 0,
        "selected-route" => 1,
        "review-connector" => 2,
        "held-gap" => 3,
        "unassigned-gap-backlog" => 4,
        _ => 5,
    }
}

pub(crate) fn t3_national_segment_id(zone_id: &str, route: &str) -> String {
    format!(
        "US.HWYSEG.{:016X}",
        stable_segment_hash(&format!(
            "{}|{}",
            zone_id.trim(),
            normalise_designation(route)
        ))
    )
}

pub(crate) fn t3_stitch_group_id(zone_id: &str, route: &str) -> String {
    format!(
        "US.HWYSTITCH.{:016X}",
        stable_segment_hash(&format!(
            "{}|{}",
            zone_id.trim(),
            normalise_designation(route)
        ))
    )
}

pub(crate) fn t3_segment_bundle_id(zone_id: &str, route: &str) -> String {
    format!(
        "US.HWYBUNDLE.{:016X}",
        stable_segment_hash(&format!(
            "{}|{}",
            zone_id.trim(),
            normalise_designation(route)
        ))
    )
}

pub(crate) fn t3_segment_aliases(zone_id: &str, route: &str, layer: &str) -> String {
    let mut aliases = vec![
        "current-tier:T3".to_string(),
        format!("current-zone:{}", zone_id.trim()),
        format!("layer:{}", layer.trim()),
    ];
    let route = normalise_designation(route);
    if !route.is_empty() {
        aliases.push(format!("route:{route}"));
        aliases.push(format!("route-label:{route}"));
        aliases.push(format!("zone-route:{}:{route}", zone_id.trim()));
    }
    aliases.join(";")
}

pub(crate) fn stable_segment_hash(value: &str) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

pub(crate) fn route_render_action(row: &T3ZoneRouteColumnRow) -> String {
    match row.column_decision.as_str() {
        "selected" => "render selected T3 route column with stop placement constraints".to_string(),
        "upward-review" => "show as review connector without promotion".to_string(),
        "review" => "show as held feeder candidate only through gap callout".to_string(),
        _ => "hold route outside rendered zone board".to_string(),
    }
}

pub(crate) fn write_t3_zone_render_board(path: &Path, rows: &[T3ZoneRenderBoardRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t3_zone_render_board_summary(output: &Path, rows: &[T3ZoneRenderBoardRow]) {
    let mut by_layer = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_layer.entry(row.board_layer.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T3 zone render board rows to {}",
        rows.len(),
        output.display()
    );
    for (layer, count) in by_layer {
        println!("  {layer}: {count}");
    }
}

// `t3_zone_render_board_gate_failures` moved to support::tier

pub(crate) fn load_t3_zone_render_board(path: &Path) -> Result<Vec<T3ZoneRenderBoardRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t3_zone_stop_placement_rows` moved to support::tier

pub(crate) fn t3_transfer_grade_stop(stop: &StopCandidateRow) -> bool {
    matches!(
        stop.requested_class.trim().to_ascii_uppercase().as_str(),
        "S1" | "S2" | "S3"
    )
}

pub(crate) fn t3_stop_state_scope(stops: &[&StopCandidateRow]) -> String {
    stops
        .iter()
        .flat_map(|stop| stop.state.split(['/', ';', ',']))
        .map(|state| state.trim().to_ascii_uppercase())
        .filter(|state| !state.is_empty())
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>()
        .join(";")
}

pub(crate) fn t3_zone_stop_plan_for_route<'a>(
    rows: &'a [StopCandidateRow],
    route: &str,
    zone_id: &str,
) -> Vec<&'a StopCandidateRow> {
    let mut stops = stop_plan_for_route(rows, route)
        .into_iter()
        .filter(|stop| t3_stop_in_zone(stop, zone_id))
        .collect::<Vec<_>>();
    sort_stops_for_route(&mut stops);
    stops
}

pub(crate) fn t3_stop_in_zone(stop: &StopCandidateRow, zone_id: &str) -> bool {
    let Some((min_lat, max_lat, min_lon, max_lon)) = t3_zone_bounds(zone_id) else {
        return true;
    };
    let Some(lat) = parse_coord(&stop.lat) else {
        return false;
    };
    let Some(lon) = parse_coord(&stop.lon) else {
        return false;
    };
    (min_lat..=max_lat).contains(&lat) && (min_lon..=max_lon).contains(&lon)
}

pub(crate) fn t3_zone_bounds(zone_id: &str) -> Option<(f64, f64, f64, f64)> {
    match zone_id {
        "t3-great-lakes" => Some((37.0, 46.5, -92.0, -74.0)),
        "t3-southeast" => Some((25.0, 39.5, -91.5, -75.0)),
        "t3-texas-border" => Some((25.0, 34.5, -107.5, -93.0)),
        "t3-mountain-west" => Some((31.0, 49.5, -125.0, -102.0)),
        "t3-mid-south" => Some((29.0, 40.5, -96.5, -75.0)),
        _ => None,
    }
}

pub(crate) fn t3_zone_stop_placement_decision(
    stop_count: usize,
    transfer_grade_stop_count: usize,
) -> (&'static str, &'static str, &'static str, &'static str) {
    if stop_count >= 2 && transfer_grade_stop_count >= 1 {
        (
            "ready-for-stop-layout",
            "place route on zone schematic using ordered stop chain",
            "maps/t3-zone",
            "pass",
        )
    } else if stop_count == 1 {
        (
            "needs-terminal-stop",
            "author one additional transfer or regional terminal stop before geometry",
            "data/tier-stop-candidates.csv",
            "review",
        )
    } else {
        (
            "needs-stop-chain",
            "author at least two visible T3 zone stops before geometry",
            "data/tier-stop-candidates.csv",
            "review",
        )
    }
}

pub(crate) fn write_t3_zone_stop_placement(path: &Path, rows: &[T3ZoneStopPlacementRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t3_zone_stop_placement_summary(output: &Path, rows: &[T3ZoneStopPlacementRow]) {
    let mut by_status = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_status.entry(row.placement_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T3 zone stop placement rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in by_status {
        println!("  {status}: {count}");
    }
}

// `t3_zone_stop_placement_gate_failures` moved to support::tier::t3_zone_stop_placement_gate_failures

pub(crate) fn load_t3_zone_stop_placement(path: &Path) -> Result<Vec<T3ZoneStopPlacementRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

#[derive(Default)]
struct NationalSegmentRegistryBuilder {
    national_segment_id: String,
    segment_bundle_id: String,
    bundle_role: String,
    stitch_group_id: String,
    zone_id: String,
    current_tier: String,
    route: String,
    evidence_state_scope: std::collections::BTreeSet<String>,
    geometry_state_scope: std::collections::BTreeSet<String>,
    segment_aliases: std::collections::BTreeSet<String>,
    bundle_aliases: std::collections::BTreeSet<String>,
    board_layers: std::collections::BTreeSet<String>,
    source_artifacts: std::collections::BTreeSet<String>,
    stop_placement_status: std::collections::BTreeSet<String>,
    qualification_effects: std::collections::BTreeSet<String>,
    validation_statuses: std::collections::BTreeSet<String>,
}

// `national_segment_registry_rows` moved to `national_segment_registry_rows.rs`

pub(crate) fn merge_segment_identity(
    builder: &mut NationalSegmentRegistryBuilder,
    segment_bundle_id: &str,
    stitch_group_id: &str,
    zone_id: &str,
    current_tier: &str,
    route: &str,
) {
    if builder.segment_bundle_id.is_empty() {
        builder.segment_bundle_id = segment_bundle_id.to_string();
    }
    if builder.stitch_group_id.is_empty() {
        builder.stitch_group_id = stitch_group_id.to_string();
    }
    if builder.zone_id.is_empty() {
        builder.zone_id = zone_id.to_string();
    }
    if builder.current_tier.is_empty() {
        builder.current_tier = current_tier.to_string();
    }
    if builder.route.is_empty() {
        builder.route = route.to_string();
    }
}

pub(crate) fn tier_segment_bundle_role(row: &TierSegmentCandidateRow) -> &'static str {
    if row.member_role == "stitched-member" {
        "stitched-service"
    } else {
        "single-segment"
    }
}

pub(crate) fn national_segment_member_key(segment_bundle_id: &str, national_segment_id: &str) -> String {
    format!(
        "{}|{}",
        segment_bundle_id.trim(),
        national_segment_id.trim()
    )
}

pub(crate) fn insert_semicolon_values(target: &mut std::collections::BTreeSet<String>, value: &str) {
    for item in value
        .split(';')
        .map(str::trim)
        .filter(|item| !item.is_empty())
    {
        target.insert(item.to_string());
    }
}

pub(crate) fn insert_pipe_values(target: &mut std::collections::BTreeSet<String>, value: &str) {
    for item in value
        .split('|')
        .map(str::trim)
        .filter(|item| !item.is_empty())
    {
        target.insert(item.to_string());
    }
}

pub(crate) fn insert_non_empty_string(target: &mut std::collections::BTreeSet<String>, value: &str) {
    if !value.trim().is_empty() {
        target.insert(value.trim().to_string());
    }
}

pub(crate) fn join_string_set(values: &std::collections::BTreeSet<String>) -> String {
    values.iter().cloned().collect::<Vec<_>>().join(";")
}

pub(crate) fn join_pipe_set(values: &std::collections::BTreeSet<String>) -> String {
    values.iter().cloned().collect::<Vec<_>>().join("|")
}

pub(crate) fn national_segment_registry_action(
    board_layers: &std::collections::BTreeSet<String>,
    stop_statuses: &std::collections::BTreeSet<String>,
    evidence_state_scope: &std::collections::BTreeSet<String>,
    geometry_state_scope: &std::collections::BTreeSet<String>,
) -> &'static str {
    if board_layers.contains("zone-summary") || board_layers.contains("unassigned-gap-backlog") {
        return "track-zone-or-backlog-identity";
    }
    if board_layers.contains("tier-segment-candidate") {
        return "eligible-for-service-bundle";
    }
    if stop_statuses.contains("ready-for-stop-layout") {
        return "eligible-for-geometry-layout";
    }
    if evidence_state_scope.is_empty() && geometry_state_scope.is_empty() {
        return "author-zone-bounded-stop-chain";
    }
    "complete-terminal-stop-chain"
}

pub(crate) fn write_national_segment_registry(path: &Path, rows: &[NationalSegmentRegistryRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_national_segment_registry_summary(output: &Path, rows: &[NationalSegmentRegistryRow]) {
    let mut by_action = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_action.entry(row.registry_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} national segment registry rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in by_action {
        println!("  {action}: {count}");
    }
}

// `national_segment_registry_gate_failures` moved to support::gates::national_segment_registry_gate_failures

pub(crate) fn load_national_segment_registry(path: &Path) -> Result<Vec<NationalSegmentRegistryRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `national_segment_bundle_rows` moved to support::network

pub(crate) fn write_national_segment_bundles(path: &Path, rows: &[NationalSegmentBundleRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_national_segment_bundle_summary(output: &Path, rows: &[NationalSegmentBundleRow]) {
    let mut by_status = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_status.entry(row.bundle_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} national segment bundle rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in by_status {
        println!("  {status}: {count}");
    }
}

// `national_segment_bundle_gate_failures` moved to support::gates

pub(crate) fn t2_bubble_up_review_rows(intake_rows: &[T3T4PressureIntakeRow]) -> Vec<T2BubbleUpReviewRow> {
    intake_rows
        .iter()
        .filter(|row| row.intake_class == "bubble-up-t2-review")
        .map(|row| T2BubbleUpReviewRow {
            route: row.route.clone(),
            source_intake_class: row.intake_class.clone(),
            current_score: row.current_score,
            review_action: "require-t2-contact-witness-before-upgrade".to_string(),
            required_evidence: "T2 contact witness plus source-backed regional service value"
                .to_string(),
            next_artifact: "data/tier-contact-witnesses.csv".to_string(),
            optimizer_effect: "may reopen T2 candidate review only after contact validation"
                .to_string(),
            validation_status: "review".to_string(),
        })
        .collect()
}

pub(crate) fn write_t2_bubble_up_review(path: &Path, rows: &[T2BubbleUpReviewRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t2_bubble_up_review_summary(output: &Path, rows: &[T2BubbleUpReviewRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.review_action.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T2 bubble-up review rows to {}",
        rows.len(),
        output.display()
    );
    for (action, count) in counts {
        println!("  {action}: {count}");
    }
}

pub(crate) fn t2_bubble_up_review_gate_failures(rows: &[T2BubbleUpReviewRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T2 bubble-up review rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.review_action.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!("{} has incomplete T2 bubble-up review", row.route));
        }
    }
    failures
}

pub(crate) fn load_t2_bubble_up_review(path: &Path) -> Result<Vec<T2BubbleUpReviewRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_t1_sla_pairs(path: &Path) -> Result<Vec<T1SlaPairRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t1_feedback_docket_rows` moved to support::tier

pub(crate) fn t1_sla_pairs_by_route(
    sla_rows: &[T1SlaPairRow],
) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut pairs_by_route = std::collections::BTreeMap::<String, Vec<String>>::new();
    for pair in sla_rows {
        for route in pair.required_routes.split(';') {
            let route_key = canonical_route_key(route);
            if !route_key.is_empty() {
                pairs_by_route
                    .entry(route_key)
                    .or_default()
                    .push(pair.pair_id.clone());
            }
        }
    }
    for pairs in pairs_by_route.values_mut() {
        pairs.sort();
        pairs.dedup();
    }
    pairs_by_route
}

pub(crate) fn t1_feedback_service_decision(
    row: &T2ServiceSelectionRow,
    has_t1_sla_pair: bool,
) -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
) {
    if has_t1_sla_pair {
        return (
            "t1-sla-candidate",
            "evaluate-t1-sla-route-substitution",
            "named T1 SLA pair plus proof the lower-tier service improves that promise",
            "data/t1-line-selector.csv",
            "may reopen T1 line selection only as an SLA/stop/topology repair",
            "review",
        );
    }
    if row.selection_action == "closure-review-needs-beck-diagnostic" {
        return (
            "beck-diagnostic-needed",
            "add-beck-diagnostic-before-t1-feedback",
            "Beck T2 diagnostic plus named T1 SLA dependency before any T1 review",
            "data/beck-t2-diagnostics.csv",
            "holds below T1 until service geometry and promise dependency are proven",
            "review",
        );
    }
    (
        "no-t1-action",
        "keep-below-t1",
        "named T1 SLA pair, T1 stop obligation, or T1 topology repair witness",
        "data/t2-service-selection.csv",
        "score or regional service value alone cannot promote a route to T1",
        "pass",
    )
}

pub(crate) fn t1_feedback_bubble_decision(
    has_t1_sla_pair: bool,
) -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
) {
    if has_t1_sla_pair {
        return (
            "t1-sla-candidate",
            "evaluate-t1-sla-route-substitution-after-contact",
            "T2 contact witness plus named T1 SLA improvement",
            "data/t1-line-selector.csv",
            "may reopen T1 only after lower-tier contact and SLA dependency are both proven",
            "review",
        );
    }
    (
        "t2-contact-first",
        "require-t2-contact-before-any-t1-review",
        "T2 contact witness plus source-backed regional service value; T1 also requires named SLA dependency",
        "data/tier-contact-witnesses.csv",
        "holds pressure at T2 because no T1 promise depends on this route",
        "review",
    )
}

pub(crate) fn t1_feedback_intake_decision(
    row: &T3T4PressureIntakeRow,
    has_t1_sla_pair: bool,
) -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
) {
    if has_t1_sla_pair {
        return (
            "t1-sla-candidate",
            "evaluate-t1-sla-route-substitution-after-contact",
            "lower-tier contact witness plus named T1 SLA improvement",
            "data/t1-line-selector.csv",
            "may reopen T1 only as a promise-preserving repair",
            "review",
        );
    }
    if row.current_score >= T1_THRESHOLD - 5.0 {
        return (
            "reject-t1-score-only-promotion",
            "reject-score-only-t1-promotion",
            "named T1 SLA pair, T1 stop obligation, or T1 topology repair witness",
            "data/t1-sla-pairs.csv",
            "near-threshold score is visible but cannot override the T1 promise portfolio",
            "pass",
        );
    }
    (
        "t2-contact-first",
        "require-t2-contact-before-any-t1-review",
        "T2 contact witness plus source-backed regional service value; T1 also requires named SLA dependency",
        "data/tier-contact-witnesses.csv",
        "holds pressure below T1 because no T1 promise depends on this route",
        "review",
    )
}

pub(crate) fn write_t1_feedback_docket(path: &Path, rows: &[T1FeedbackDocketRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t1_feedback_docket_summary(output: &Path, rows: &[T1FeedbackDocketRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.t1_feedback_class.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T1 feedback rows to {}",
        rows.len(),
        output.display()
    );
    for (class, count) in counts {
        println!("  {class}: {count}");
    }
}

pub(crate) fn t1_feedback_docket_gate_failures(rows: &[T1FeedbackDocketRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T1 feedback docket rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if row.route.trim().is_empty()
            || row.source_surface.trim().is_empty()
            || row.source_action.trim().is_empty()
            || row.t1_feedback_class.trim().is_empty()
            || row.t1_feedback_action.trim().is_empty()
            || row.required_evidence.trim().is_empty()
            || row.next_artifact.trim().is_empty()
            || row.optimizer_effect.trim().is_empty()
        {
            failures.push(format!("{} has incomplete T1 feedback row", row.route));
        }
        if row.t1_feedback_class == "t1-sla-candidate"
            && (row.t1_sla_pair_count == 0 || row.t1_sla_pairs.trim().is_empty())
        {
            failures.push(format!(
                "{} promoted to T1 candidate without named SLA pair",
                row.route
            ));
        }
        if row.t1_feedback_class != "t1-sla-candidate"
            && row.t1_feedback_action.contains("t1-sla-route-substitution")
        {
            failures.push(format!(
                "{} has T1 substitution action outside t1-sla-candidate class",
                row.route
            ));
        }
    }
    failures
}

// `source_fetch_policy_rows` moved to support::misc

pub(crate) fn load_source_fetch_policy(path: &Path) -> Result<Vec<SourceFetchPolicyRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn load_source_snapshot_publication_exclusion(
    path: &Path,
) -> Result<Vec<SourceSnapshotPublicationExclusionRow>> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn accepted_source_snapshot_publication_exclusion(
    rows: &[SourceSnapshotPublicationExclusionRow],
) -> Option<&SourceSnapshotPublicationExclusionRow> {
    rows.iter().find(|row| {
        row.decision == "exclude-live-snapshot-guard-from-map-publication"
            && row.validation_status == "accepted"
            && row.affected_constraint_class == "source_acquisition_snapshot_guard"
            && row.affected_fetch_family == "t1-live-event-snapshots"
            && row.excluded_claims == "publication"
            && row.preserved_claims_after == "evidence"
    })
}

pub(crate) fn source_fetch_policy_row(
    fetch_family: &str,
    commands: &str,
    cache_targets: &str,
    mutation_mode: &str,
    preservation_contract: &str,
    implementation_guard: &str,
    validation_floor: &str,
) -> SourceFetchPolicyRow {
    SourceFetchPolicyRow {
        fetch_family: fetch_family.to_string(),
        commands: commands.to_string(),
        cache_targets: cache_targets.to_string(),
        mutation_mode: mutation_mode.to_string(),
        preservation_contract: preservation_contract.to_string(),
        implementation_guard: implementation_guard.to_string(),
        validation_floor: validation_floor.to_string(),
        policy_doc: "docs/source-fetch-cache-policy.md".to_string(),
        validation_status: "pass".to_string(),
    }
}

pub(crate) fn write_source_fetch_policy(path: &Path, rows: &[SourceFetchPolicyRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_source_fetch_policy_summary(output: &Path, rows: &[SourceFetchPolicyRow]) {
    let mut modes = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *modes.entry(row.mutation_mode.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} source fetch policy rows to {}",
        rows.len(),
        output.display()
    );
    for (mode, count) in modes {
        println!("  {mode}: {count}");
    }
}

// `print_fletch_source_handoff_summary` moved to support::print

pub(crate) fn fletch_source_handoff_gate_failures(
    report: &route_data::FletchSourceHandoffReport,
) -> Vec<String> {
    let mut failures = Vec::new();
    if !report.registry_valid {
        failures.push(format!(
            "registry {} is not valid ({} findings)",
            report.registry_id, report.validation_finding_count
        ));
    }
    if !report.missing_policy_families.is_empty() {
        failures.push(format!(
            "missing FLETCH coverage for source policy families: {}",
            report.missing_policy_families.join(", ")
        ));
    }
    if report.rows.is_empty() {
        failures.push("FLETCH source handoff emitted no rows".to_string());
    }
    for row in &report.rows {
        if row.validation_status != "pass" {
            failures.push(format!("{} handoff row is not valid", row.fletch_id));
        }
        if row.fetch_family.trim().is_empty() {
            failures.push(format!("{} missing fetch_family metadata", row.fletch_id));
        }
        if row.cache_targets.trim().is_empty() {
            failures.push(format!("{} missing cache targets", row.fletch_id));
        }
        if row.activation_rule.trim().is_empty() {
            failures.push(format!("{} missing activation rule", row.fletch_id));
        }
        if row.route_validation_floor.trim().is_empty() {
            failures.push(format!("{} missing ROUTE validation floor", row.fletch_id));
        }
    }
    failures
}

pub(crate) fn print_fletch_cache_index_summary(
    output: &Path,
    report: &route_data::FletchCacheIndexReport,
    details: bool,
) {
    println!(
        "  wrote {} FLETCH cache-index rows to {}",
        report.rows.len(),
        output.display()
    );
    println!("  registry: {}", report.registry_id);
    println!(
        "  registered matched: {}/{} (missing: {})",
        report.matched_registered_count, report.registered_count, report.missing_registered_count
    );
    println!(
        "  entries: {} verified / {} unverified; unexpected: {}; bytes: {}",
        report.verified_count,
        report.unverified_count,
        report.unexpected_entry_count,
        report.byte_count
    );
    if details {
        println!();
        println!("  {:36}  {:10}  {:10}  path", "FLETCH", "registry", "cache");
        println!("  {}", "-".repeat(92));
        for row in &report.rows {
            println!(
                "  {:36}  {:10}  {:10}  {}",
                row.fletch_id, row.registry_status, row.cache_status, row.relative_path
            );
        }
    }
}

// `source_fetch_policy_gate_failures` moved to support::gates

pub(crate) fn known_source_fetch_commands() -> &'static [&'static str] {
    &[
        "route fetch",
        "route fetch-hpms",
        "route fetch-hpms --states",
        "route fetch-acs",
        "route fetch-acs-income",
        "route fetch-fema-d1",
        "route fetch-fema",
        "route t1-fetch-iowa511",
        "route t1-fetch-tdot-smartway",
        "route t1-fetch-mdot-midrive",
        "route t1-fetch-indot-trafficwise",
    ]
}

pub(crate) fn source_fetch_policy_row_covers_command(row: &SourceFetchPolicyRow, command: &str) -> bool {
    row.commands
        .split(';')
        .map(str::trim)
        .any(|candidate| candidate == command || candidate.starts_with(&format!("{command} ")))
}

// `tier_optimizer_run_rows` moved to `optimizer_run.rs`

pub(crate) fn csv_record_count(path: &Path) -> Result<usize> {
    if !path.exists() {
        return Ok(0);
    }
    let mut reader = csv::Reader::from_path(path)?;
    Ok(reader.records().count())
}

pub(crate) fn write_tier_optimizer_runs(path: &Path, rows: &[TierOptimizerRunRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_tier_optimizer_run_summary(output: &Path, rows: &[TierOptimizerRunRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.gate_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} optimizer run rows to {}",
        rows.len(),
        output.display()
    );
    for (gate_status, count) in counts {
        println!("  {gate_status}: {count}");
    }
}

pub(crate) fn load_tier_optimizer_runs(path: &Path) -> Result<Vec<TierOptimizerRunRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub(crate) fn print_optimizer_manifest_summary(path: &Path, rows: &[TierOptimizerRunRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.gate_status.as_str()).or_default() += 1;
    }
    println!(
        "  read {} optimizer manifest rows from {}",
        rows.len(),
        path.display()
    );
    for (gate_status, count) in counts {
        println!("  {gate_status}: {count}");
    }
}

pub(crate) fn tier_optimizer_run_gate_failures(all_tiers: bool, rows: &[TierOptimizerRunRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if !all_tiers {
        failures.push("tier-optimize bundle gate requires --all-tiers".to_string());
    }
    failures.extend(optimizer_manifest_gate_failures(rows));
    failures
}

// `optimizer_manifest_gate_failures` moved to support::gates

// `optimizer_map_hook_rows` moved to support::optimizer

pub(crate) fn artifact_has_content(path: &str) -> bool {
    std::fs::metadata(repo_relative_artifact_path(path))
        .map(|metadata| metadata.is_file() && metadata.len() > 0)
        .unwrap_or(false)
}

pub(crate) fn write_optimizer_map_hooks(path: &Path, rows: &[OptimizerMapHookRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_optimizer_map_hook_summary(output: &Path, rows: &[OptimizerMapHookRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.consumer_type.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} optimizer map hook rows to {}",
        rows.len(),
        output.display()
    );
    for (consumer_type, count) in counts {
        println!("  {consumer_type}: {count}");
    }
}

// `optimizer_map_hook_gate_failures` moved to support::gates

// `bundle_architecture_rows` moved to support::network

pub(crate) fn missing_source_tokens(source_path: &str, required_tokens: &str) -> Vec<String> {
    let Ok(source) = std::fs::read_to_string(resolve_repo_path(source_path)) else {
        return semicolon_values(required_tokens);
    };
    semicolon_values(required_tokens)
        .into_iter()
        .filter(|token| !source.contains(token))
        .collect()
}

pub(crate) fn resolve_repo_path(path: &str) -> PathBuf {
    let direct = PathBuf::from(path);
    if direct.exists() {
        return direct;
    }
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(|workspace| workspace.join(path))
        .unwrap_or(direct)
}

pub(crate) fn write_bundle_architecture(path: &Path, rows: &[BundleArchitectureRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_bundle_architecture_summary(output: &Path, rows: &[BundleArchitectureRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.validation_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} bundle architecture rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

pub(crate) fn bundle_architecture_gate_failures(rows: &[BundleArchitectureRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no bundle architecture rows emitted".to_string());
        return failures;
    }
    let mut seen = std::collections::BTreeSet::<String>::new();
    for row in rows {
        if row.crate_name.trim().is_empty()
            || row.role.trim().is_empty()
            || row.bundle_entrypoint.trim().is_empty()
            || row.source_path.trim().is_empty()
            || row.required_tokens.trim().is_empty()
            || row.architecture_status.trim().is_empty()
            || row.next_action.trim().is_empty()
        {
            failures.push(format!(
                "{} has incomplete architecture row",
                row.crate_name
            ));
        }
        if !seen.insert(row.crate_name.clone()) {
            failures.push(format!("{} has duplicate architecture row", row.crate_name));
        }
        if row.validation_status != "pass" {
            failures.push(format!(
                "{} bundle architecture check failed: {}",
                row.crate_name, row.next_action
            ));
        }
        if !matches!(
            row.architecture_status.as_str(),
            "bundle-native" | "bundle-upstream"
        ) {
            failures.push(format!(
                "{} has unknown architecture status {}",
                row.crate_name, row.architecture_status
            ));
        }
    }
    failures
}

pub(crate) fn print_tier_region_workload_summary(
    tier: &str,
    requested_regions: usize,
    output: &Path,
    repairs: &Path,
    rows: &[TierRegionWorkloadRow],
) {
    let mut route_counts = vec![0usize; requested_regions];
    let mut weight_counts = vec![0i32; requested_regions];
    for row in rows {
        route_counts[row.region_id] += 1;
        weight_counts[row.region_id] += row.route_weight;
    }
    println!(
        "  wrote {} {tier} route workload rows to {}",
        rows.len(),
        output.display()
    );
    for region in 0..requested_regions {
        println!(
            "  region {region}: {} routes, {} weighted miles",
            route_counts[region], weight_counts[region]
        );
    }
    if let Some(status) = rows.first().map(|row| row.component_status.as_str()) {
        println!("  graph status: {status}");
    }
    println!("  wrote repair docket: {}", repairs.display());
}

// `tier_region_gate_failures` moved to support::tier

#[derive(Debug, Clone, serde::Deserialize)]
struct EndpointExceptionRow {
    route: String,
    requested_tier: String,
    endpoint_name: String,
    endpoint_role: String,
    exception_type: String,
    evidence_level: String,
    artifact: String,
    next_step: String,
}

#[derive(Debug)]
struct TierConnectivityGateFailure<'a> {
    row: &'a route_network::TierConnectivityRow,
    reason: String,
}

pub(crate) fn load_endpoint_exceptions(path: &Path) -> Result<Vec<EndpointExceptionRow>> {
    let file = std::fs::File::open(path)?;
    parse_endpoint_exceptions(file)
}

pub(crate) fn parse_endpoint_exceptions<R: std::io::Read>(reader: R) -> Result<Vec<EndpointExceptionRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

pub(crate) fn endpoint_exceptions_for_route<'a>(
    exceptions: &'a [EndpointExceptionRow],
    route: &str,
    tier: &str,
) -> Vec<&'a EndpointExceptionRow> {
    let route = normalise_designation(route);
    exceptions
        .iter()
        .filter(|row| normalise_designation(&row.route) == route)
        .filter(|row| row.requested_tier.trim().eq_ignore_ascii_case(tier.trim()))
        .collect()
}

pub(crate) fn tier_connectivity_gate_failures_with_exceptions<'a>(
    rows: &'a [route_network::TierConnectivityRow],
    exceptions: &[EndpointExceptionRow],
    tier: &str,
) -> Vec<TierConnectivityGateFailure<'a>> {
    rows.iter()
        .filter_map(|row| {
            if matches!(
                row.classification,
                route_network::TierNodeClass::TrunkConnector
                    | route_network::TierNodeClass::ReliefLoop
            ) {
                return None;
            }

            let route_exceptions = endpoint_exceptions_for_route(exceptions, &row.route, tier);
            if route_exception_allows_connectivity_gate(row, &route_exceptions) {
                return None;
            }

            Some(TierConnectivityGateFailure {
                row,
                reason: endpoint_exception_failure_reason(row, &route_exceptions),
            })
        })
        .collect()
}

pub(crate) fn route_exception_allows_connectivity_gate(
    row: &route_network::TierConnectivityRow,
    exceptions: &[&EndpointExceptionRow],
) -> bool {
    match row.classification {
        route_network::TierNodeClass::OneEndedFeeder => exceptions
            .iter()
            .any(|exception| endpoint_exception_is_terminal_worthy(exception)),
        route_network::TierNodeClass::LocalSpur => exceptions.iter().any(|exception| {
            endpoint_exception_is_terminal_worthy(exception)
                && exception
                    .evidence_level
                    .trim()
                    .eq_ignore_ascii_case("validated")
        }),
        route_network::TierNodeClass::MissingGraphData => false,
        route_network::TierNodeClass::TrunkConnector | route_network::TierNodeClass::ReliefLoop => {
            true
        }
    }
}

pub(crate) fn endpoint_exception_is_terminal_worthy(row: &EndpointExceptionRow) -> bool {
    if !endpoint_exception_has_contract(row) {
        return false;
    }

    let role = row.endpoint_role.trim().to_ascii_lowercase();
    let exception_type = row.exception_type.trim().to_ascii_lowercase();
    let terminal_role = matches!(
        role.as_str(),
        "national_terminal" | "t2_terminal_exception" | "graph_endpoint_gap"
    );
    let terminal_exception = matches!(
        exception_type.as_str(),
        "port_terminal"
            | "border_gateway"
            | "military_logistics"
            | "resilience_relief"
            | "future_tier_continuation"
            | "regional_terminal"
    );
    terminal_role && terminal_exception
}

pub(crate) fn endpoint_exception_has_contract(row: &EndpointExceptionRow) -> bool {
    !row.endpoint_name.trim().is_empty()
        && !row.endpoint_role.trim().is_empty()
        && !row.exception_type.trim().is_empty()
        && !row.artifact.trim().is_empty()
        && !row.next_step.trim().is_empty()
        && valid_endpoint_evidence_level(&row.evidence_level)
}

pub(crate) fn valid_endpoint_evidence_level(level: &str) -> bool {
    matches!(
        level.trim().to_ascii_lowercase().as_str(),
        "validated" | "heuristic" | "planned" | "missing_graph_data" | "demote"
    )
}

pub(crate) fn endpoint_exception_failure_reason(
    row: &route_network::TierConnectivityRow,
    exceptions: &[&EndpointExceptionRow],
) -> String {
    if exceptions.is_empty() {
        return "no endpoint exception record".to_string();
    }

    if matches!(
        row.classification,
        route_network::TierNodeClass::MissingGraphData
    ) {
        return "graph/contact data must be fixed before endpoint exception can promote route"
            .to_string();
    }

    let invalid_contracts = exceptions
        .iter()
        .filter(|exception| !endpoint_exception_has_contract(exception))
        .count();
    if invalid_contracts > 0 {
        return format!(
            "{invalid_contracts} endpoint exception record(s) lack a complete contract"
        );
    }

    "endpoint exception is not terminal-worthy for requested tier".to_string()
}

pub(crate) fn endpoint_exception_summary(
    exceptions: &[EndpointExceptionRow],
    route: &str,
    tier: &str,
) -> String {
    let route_exceptions = endpoint_exceptions_for_route(exceptions, route, tier);
    if route_exceptions.is_empty() {
        return "-".to_string();
    }
    route_exceptions
        .iter()
        .map(|row| {
            format!(
                "{}:{}:{}",
                row.endpoint_role.trim(),
                row.exception_type.trim(),
                row.evidence_level.trim()
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

pub(crate) fn filter_endpoint_exceptions<'a>(
    rows: &'a [EndpointExceptionRow],
    tier: Option<&str>,
    route: Option<&str>,
) -> Vec<&'a EndpointExceptionRow> {
    let route = route.map(normalise_designation);
    rows.iter()
        .filter(|row| {
            tier.map(|tier| row.requested_tier.trim().eq_ignore_ascii_case(tier.trim()))
                .unwrap_or(true)
        })
        .filter(|row| {
            route
                .as_ref()
                .map(|route| normalise_designation(&row.route) == *route)
                .unwrap_or(true)
        })
        .collect()
}

pub(crate) fn endpoint_exception_gate_failures(
    rows: &[&EndpointExceptionRow],
    require_terminal_worthy: bool,
) -> Vec<String> {
    let mut failures = Vec::new();
    for row in rows {
        let route = normalise_designation(&row.route);
        if route.is_empty() {
            failures.push("row missing route".to_string());
        }
        if row.requested_tier.trim().is_empty() {
            failures.push(format!("{route}: missing requested_tier"));
        }
        if !endpoint_exception_has_contract(row) {
            failures.push(format!("{route}: incomplete endpoint exception contract"));
        }
        if !valid_endpoint_evidence_level(&row.evidence_level) {
            failures.push(format!(
                "{route}: unsupported evidence_level {}",
                row.evidence_level
            ));
        }
        if require_terminal_worthy && !endpoint_exception_is_terminal_worthy(row) {
            failures.push(format!(
                "{route}: endpoint exception is not terminal-worthy for requested tier"
            ));
        }
    }
    failures
}

// `print_endpoint_exceptions` moved to support::print

#[derive(Debug, Clone, serde::Deserialize)]
struct StopCandidateRow {
    stop_id: String,
    name: String,
    state: String,
    lat: String,
    lon: String,
    requested_class: String,
    route_refs: String,
    stop_role: String,
    transfer_value: String,
    freight_volume: String,
    spacing_need: String,
    resilience_value: String,
    energy_service: String,
    land_ops_feasibility: String,
    equity_community: String,
    evidence_status: String,
    source_artifact: String,
    next_step: String,
}

#[derive(Debug)]
struct T1LineSelectorRow {
    route: String,
    tier: String,
    score: f64,
    constraint_adjusted_score: f64,
    rank: usize,
    selected: bool,
    selected_stop_count: usize,
    top_city_stop_count: usize,
    sla_pair_count: usize,
    budget_cost: usize,
    hard_blocker_count: usize,
    claim_blocker_count: usize,
    constraint_debt_cost_m: f64,
    lifecycle_debt_cost_m: f64,
    constraint_penalty_score: f64,
    top_constraint_classes: String,
    constraint_ledger_artifact: String,
    decision: &'static str,
    reason: &'static str,
    selected_stops: String,
    top_city_stops: String,
    sla_pairs: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct T1SlaCandidateUniverseRow {
    pair_id: String,
    origin_id: String,
    dest_id: String,
    target_hours: f64,
    market_class: String,
    required_routes: String,
    required_stops: String,
    evidence_basis: String,
    market_score: f64,
    conversion_score: f64,
    coverage_score: f64,
    reuse_score: f64,
    resilience_score: f64,
    evidence_score: f64,
    budget_penalty: f64,
    drop_reason_hint: String,
    covered_by_selected_pair: String,
}

#[derive(Debug, Clone, serde::Serialize)]
struct T1SlaCandidatePairRow {
    rank: usize,
    pair_id: String,
    origin_id: String,
    dest_id: String,
    target_hours: f64,
    market_class: String,
    total_score: f64,
    market_score: f64,
    conversion_score: f64,
    coverage_score: f64,
    reuse_score: f64,
    resilience_score: f64,
    evidence_score: f64,
    budget_penalty: f64,
    portfolio_selected: bool,
    selected_budget: usize,
    cutline_status: String,
    cutline_reason: String,
    covered_by_selected_pair: String,
    required_routes: String,
    required_stops: String,
    evidence_basis: String,
    validation_status: String,
}

#[derive(Debug, serde::Deserialize)]
struct T1LineSelectorInputRow {
    route: String,
    selected: bool,
    selected_stops: String,
}

#[derive(Debug, Clone, serde::Serialize)]
struct T1StopSelectorRow {
    route: String,
    stop_sequence: usize,
    stop_id: String,
    stop_name: String,
    requested_class: String,
    selector_weight: i32,
    split_objective: String,
    target_regions: usize,
    metis_region: usize,
    boundary_after: bool,
    evidence_status: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
struct T1StopSelectorInputRow {
    route: String,
    stop_sequence: usize,
    stop_id: String,
    stop_name: String,
    requested_class: String,
    selector_weight: i32,
    split_objective: String,
    target_regions: usize,
    metis_region: usize,
    boundary_after: bool,
    evidence_status: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Serialize)]
struct T1BeckAlignmentRow {
    route: String,
    selector_stop_count: usize,
    selector_boundary_count: usize,
    selector_regions: usize,
    beck_stop_count: usize,
    beck_drawn_stop_count: usize,
    beck_transfer_stop_count: usize,
    beck_action: String,
    beck_review_flag: String,
    alignment_status: String,
    validation_status: String,
}

#[derive(Debug, serde::Deserialize)]
struct TierTableInputRow {
    tier: String,
    route: String,
    score: f64,
}

#[derive(Debug, serde::Deserialize)]
struct T1SlaPairRow {
    pair_id: String,
    origin_id: String,
    dest_id: String,
    target_hours: f64,
    priority: u8,
    market_class: String,
    required_routes: String,
    required_stops: String,
    evidence_basis: String,
}

pub(crate) fn load_t1_sla_candidate_universe(path: &Path) -> Result<Vec<T1SlaCandidateUniverseRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t1_sla_candidate_pair_rows` moved to support::tier

pub(crate) fn t1_sla_candidate_pair_score(row: &T1SlaCandidateUniverseRow) -> f64 {
    row.market_score
        + row.conversion_score
        + row.coverage_score
        + row.reuse_score
        + row.resilience_score
        + row.evidence_score
        - row.budget_penalty
}

pub(crate) fn write_t1_sla_candidate_pairs(path: &Path, rows: &[T1SlaCandidatePairRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t1_sla_candidate_pair_summary(
    output: &Path,
    rows: &[T1SlaCandidatePairRow],
    selected_budget: usize,
) {
    let selected = rows.iter().filter(|row| row.portfolio_selected).count();
    let dropped = rows.iter().filter(|row| !row.portfolio_selected).count();
    let cutline = rows
        .iter()
        .find(|row| row.rank == selected_budget + 1)
        .map(|row| row.pair_id.as_str())
        .unwrap_or("n/a");
    println!(
        "  wrote {} ranked SLA candidate pairs to {}",
        rows.len(),
        output.display()
    );
    println!("  selected portfolio rows: {selected}/{selected_budget}");
    println!("  dropped candidate rows: {dropped}");
    println!("  first dropped by rank: {cutline}");
}

// `t1_sla_candidate_pair_gate_failures` moved to support::tier

// `t1_line_selector_rows` moved to `t1_line_selector_rows.rs`

pub(crate) fn build_t1_line_selector_csv(rows: &[T1LineSelectorRow]) -> String {
    let mut csv = String::from(
        "route,tier,score,constraint_adjusted_score,rank,selected,selected_stop_count,top_city_stop_count,sla_pair_count,budget_cost,hard_blocker_count,claim_blocker_count,constraint_debt_cost_m,lifecycle_debt_cost_m,constraint_penalty_score,top_constraint_classes,constraint_ledger_artifact,decision,reason,selected_stops,top_city_stops,sla_pairs\n",
    );
    for row in rows {
        push_csv_line(
            &mut csv,
            &[
                &row.route,
                &row.tier,
                &format!("{:.1}", row.score),
                &format!("{:.1}", row.constraint_adjusted_score),
                &row.rank.to_string(),
                if row.selected { "true" } else { "false" },
                &row.selected_stop_count.to_string(),
                &row.top_city_stop_count.to_string(),
                &row.sla_pair_count.to_string(),
                &row.budget_cost.to_string(),
                &row.hard_blocker_count.to_string(),
                &row.claim_blocker_count.to_string(),
                &format!("{:.2}", row.constraint_debt_cost_m),
                &format!("{:.2}", row.lifecycle_debt_cost_m),
                &format!("{:.2}", row.constraint_penalty_score),
                &row.top_constraint_classes,
                &row.constraint_ledger_artifact,
                row.decision,
                row.reason,
                &row.selected_stops,
                &row.top_city_stops,
                &row.sla_pairs,
            ],
        );
    }
    csv
}

pub(crate) fn push_csv_line(csv: &mut String, cells: &[&str]) {
    for (idx, cell) in cells.iter().enumerate() {
        if idx > 0 {
            csv.push(',');
        }
        let needs_quotes = cell.contains(',') || cell.contains('"') || cell.contains('\n');
        if needs_quotes {
            csv.push('"');
            csv.push_str(&cell.replace('"', "\"\""));
            csv.push('"');
        } else {
            csv.push_str(cell);
        }
    }
    csv.push('\n');
}

// `t1_line_selector_gate_failures` moved to support::tier

pub(crate) fn load_t1_line_selector(path: &Path) -> Result<Vec<T1LineSelectorInputRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t1_stop_selector_rows` moved to support::tier

pub(crate) fn write_t1_stop_selector(path: &Path, rows: &[T1StopSelectorRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t1_stop_selector_summary(output: &Path, rows: &[T1StopSelectorRow]) {
    let route_count = rows
        .iter()
        .map(|row| row.route.as_str())
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let boundary_count = rows.iter().filter(|row| row.boundary_after).count();
    println!(
        "  wrote {} stop rows across {} T1 routes to {}",
        rows.len(),
        route_count,
        output.display()
    );
    println!("  METIS split boundaries: {boundary_count}");
}

pub(crate) fn t1_stop_selector_gate_failures(rows: &[T1StopSelectorRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T1 stop selector rows emitted".to_string());
        return failures;
    }
    let mut by_route = std::collections::BTreeMap::<&str, Vec<&T1StopSelectorRow>>::new();
    for row in rows {
        by_route.entry(row.route.as_str()).or_default().push(row);
        if row.selector_weight <= 0 {
            failures.push(format!(
                "{}:{} has non-positive selector weight",
                row.route, row.stop_id
            ));
        }
        if !row.validation_status.eq_ignore_ascii_case("pass") {
            failures.push(format!(
                "{}:{} has validation_status={}",
                row.route, row.stop_id, row.validation_status
            ));
        }
    }
    for (route, route_rows) in by_route {
        if route_rows.len() < 3 {
            failures.push(format!("{route}: fewer than 3 selected stops"));
        }
        let regions = route_rows
            .iter()
            .map(|row| row.metis_region)
            .collect::<std::collections::BTreeSet<_>>();
        if regions.len() != route_rows[0].target_regions {
            failures.push(format!(
                "{route}: expected {} METIS regions, found {}",
                route_rows[0].target_regions,
                regions.len()
            ));
        }
    }
    failures
}

pub(crate) fn load_t1_stop_selector(path: &Path) -> Result<Vec<T1StopSelectorInputRow>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

// `t1_beck_alignment_rows` moved to support::tier

pub(crate) fn write_t1_beck_alignment(path: &Path, rows: &[T1BeckAlignmentRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t1_beck_alignment_summary(output: &Path, rows: &[T1BeckAlignmentRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.alignment_status.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} alignment rows to {}",
        rows.len(),
        output.display()
    );
    for (status, count) in counts {
        println!("  {status}: {count}");
    }
}

pub(crate) fn t1_beck_alignment_gate_failures(rows: &[T1BeckAlignmentRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.is_empty() {
        failures.push("no T1 Beck alignment rows emitted".to_string());
        return failures;
    }
    for row in rows {
        if !row.validation_status.eq_ignore_ascii_case("pass") {
            failures.push(format!(
                "{} alignment_status={} selector_stops={} beck_stops={}",
                row.route, row.alignment_status, row.selector_stop_count, row.beck_stop_count
            ));
        }
    }
    failures
}

#[derive(Debug, Clone)]
struct T1DesignReviewRow {
    route: String,
    selected: bool,
    design_role: &'static str,
    promise_count: usize,
    selected_stop_count: usize,
    top_city_stop_count: usize,
    selector_reason: String,
    beck_action: String,
    beck_review_flag: String,
    overlap_corridors: String,
    design_status: &'static str,
    next_design_action: &'static str,
}

// `t1_design_review_rows` moved to support::tier

pub(crate) fn build_t1_design_review_csv(rows: &[T1DesignReviewRow]) -> String {
    let mut csv = String::from(
        "route,selected,design_role,promise_count,selected_stop_count,top_city_stop_count,selector_reason,beck_action,beck_review_flag,overlap_corridors,design_status,next_design_action\n",
    );
    for row in rows {
        push_csv_line(
            &mut csv,
            &[
                &row.route,
                if row.selected { "true" } else { "false" },
                row.design_role,
                &row.promise_count.to_string(),
                &row.selected_stop_count.to_string(),
                &row.top_city_stop_count.to_string(),
                &row.selector_reason,
                &row.beck_action,
                &row.beck_review_flag,
                &row.overlap_corridors,
                row.design_status,
                row.next_design_action,
            ],
        );
    }
    csv
}

pub(crate) fn t1_design_review_gate_failures(rows: &[T1DesignReviewRow]) -> Vec<String> {
    let mut failures = Vec::new();
    if rows.iter().all(|row| !row.selected) {
        failures.push("no selected T1 design rows".to_string());
    }
    for row in rows {
        if row.selected && row.selected_stop_count == 0 {
            failures.push(format!("{} selected without stop chain", row.route));
        }
        if !row.selected && row.promise_count > 0 {
            failures.push(format!(
                "{} carries {} promise pairs but is not selected",
                row.route, row.promise_count
            ));
        }
    }
    failures
}

#[derive(Debug, Clone, serde::Deserialize)]
#[allow(dead_code)]
struct T1DesignReviewCsvRow {
    route: String,
    selected: bool,
    design_role: String,
    promise_count: usize,
    selected_stop_count: usize,
    top_city_stop_count: usize,
    selector_reason: String,
    beck_action: String,
    beck_review_flag: String,
    overlap_corridors: String,
    design_status: String,
    next_design_action: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T1TopologyRepairRow {
    route: String,
    selected: bool,
    design_role: String,
    design_status: String,
    beck_review_flag: String,
    overlap_corridors: String,
    repair_type: String,
    repair_basis: String,
    next_artifact: String,
    next_action: String,
    validation_status: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct T1DesignPolicyActionRow {
    action: String,
    applies_to_status: String,
    definition: String,
    required_policy: String,
    design_treatment: String,
    gate_policy: String,
    next_selector_use: String,
}

pub(crate) fn load_t1_design_review(path: &Path) -> Result<Vec<T1DesignReviewCsvRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_design_review(file)
}

pub(crate) fn parse_t1_design_review<R: std::io::Read>(reader: R) -> Result<Vec<T1DesignReviewCsvRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

pub(crate) fn t1_topology_repair_rows(rows: &[T1DesignReviewCsvRow]) -> Vec<T1TopologyRepairRow> {
    rows.iter()
        .filter(|row| !row.design_status.eq_ignore_ascii_case("accepted"))
        .map(|row| {
            let (repair_type, repair_basis, next_artifact, validation_status) =
                t1_topology_repair_contract(row);
            T1TopologyRepairRow {
                route: row.route.clone(),
                selected: row.selected,
                design_role: row.design_role.clone(),
                design_status: row.design_status.clone(),
                beck_review_flag: row.beck_review_flag.clone(),
                overlap_corridors: row.overlap_corridors.clone(),
                repair_type: repair_type.to_string(),
                repair_basis: repair_basis.to_string(),
                next_artifact: next_artifact.to_string(),
                next_action: row.next_design_action.clone(),
                validation_status: validation_status.to_string(),
            }
        })
        .collect()
}

pub(crate) fn t1_topology_repair_contract(
    row: &T1DesignReviewCsvRow,
) -> (&'static str, &'static str, &'static str, &'static str) {
    if row.selected
        && row
            .beck_review_flag
            .trim()
            .eq_ignore_ascii_case("overlap-review")
    {
        (
            "shared-backbone-policy",
            "selected-t1-route-shares-beck-segment",
            "data/t1-design-policy-actions.csv",
            "review",
        )
    } else if row.selected && row.design_status.eq_ignore_ascii_case("policy-review") {
        (
            "national-relay-justification",
            "selected-score-exception-needs-national-role-proof",
            "data/t1-score-exceptions.csv",
            "review",
        )
    } else if !row.selected && row.design_status.eq_ignore_ascii_case("held") {
        (
            "held-candidate",
            "outside-current-t1-budget-or-demoted",
            "data/t1-line-selector.csv",
            "pass",
        )
    } else {
        (
            "unclassified-topology-review",
            "non-accepted-design-status",
            "data/t1-topology-repairs.csv",
            "review",
        )
    }
}

pub(crate) fn write_t1_topology_repairs(path: &Path, rows: &[T1TopologyRepairRow]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub(crate) fn print_t1_topology_repair_summary(output: &Path, rows: &[T1TopologyRepairRow]) {
    let mut counts = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *counts.entry(row.repair_type.as_str()).or_default() += 1;
    }
    println!("  wrote {} repair rows to {}", rows.len(), output.display());
    for (repair_type, count) in counts {
        println!("  {repair_type}: {count}");
    }
}

pub(crate) fn t1_topology_repair_gate_failures(rows: &[T1TopologyRepairRow]) -> Vec<String> {
    let mut failures = Vec::new();
    for row in rows {
        if row.next_action.trim().is_empty() {
            failures.push(format!("{} has no next topology repair action", row.route));
        }
        if row.next_artifact.trim().is_empty() {
            failures.push(format!(
                "{} has no next topology repair artifact",
                row.route
            ));
        }
    }
    failures
}

pub(crate) fn load_t1_design_policy_actions(path: &Path) -> Result<Vec<T1DesignPolicyActionRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_design_policy_actions(file)
}

pub(crate) fn parse_t1_design_policy_actions<R: std::io::Read>(
    reader: R,
) -> Result<Vec<T1DesignPolicyActionRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

pub(crate) fn print_t1_design_policy(
    review_rows: &[T1DesignReviewCsvRow],
    policy_rows: &[T1DesignPolicyActionRow],
    details: bool,
) {
    let mut action_counts = std::collections::BTreeMap::<String, usize>::new();
    for row in review_rows {
        *action_counts
            .entry(row.next_design_action.clone())
            .or_insert(0) += 1;
    }

    println!("route t1-design-policy");
    println!("  review rows: {}", review_rows.len());
    println!("  policy actions: {}", policy_rows.len());
    println!("  action use: {}", format_count_map(&action_counts));
    println!();
    println!("{:<34} {:<18} {:>5} Treatment", "Action", "Status", "Uses");
    println!("{}", "-".repeat(110));
    for row in policy_rows {
        let uses = action_counts.get(&row.action).copied().unwrap_or(0);
        println!(
            "{:<34} {:<18} {:>5} {}",
            row.action, row.applies_to_status, uses, row.design_treatment
        );
        if details {
            println!("  definition: {}", row.definition);
            println!("  required_policy: {}", row.required_policy);
            println!("  gate_policy: {}", row.gate_policy);
            println!("  next_selector_use: {}", row.next_selector_use);
        }
    }
}

pub(crate) fn t1_design_policy_gate_failures(
    review_rows: &[T1DesignReviewCsvRow],
    policy_rows: &[T1DesignPolicyActionRow],
) -> Vec<String> {
    let actions = policy_rows
        .iter()
        .map(|row| row.action.trim().to_string())
        .collect::<std::collections::BTreeSet<_>>();
    let mut failures = Vec::new();
    if policy_rows.is_empty() {
        failures.push("no T1 design policy action rows".to_string());
    }
    for row in policy_rows {
        if row.action.trim().is_empty()
            || row.applies_to_status.trim().is_empty()
            || row.definition.trim().is_empty()
            || row.required_policy.trim().is_empty()
            || row.design_treatment.trim().is_empty()
            || row.gate_policy.trim().is_empty()
            || row.next_selector_use.trim().is_empty()
        {
            failures.push(format!("{} has incomplete policy contract", row.action));
        }
    }
    for row in review_rows {
        if !actions.contains(row.next_design_action.trim()) {
            failures.push(format!(
                "{} uses uncovered next_design_action {}",
                row.route, row.next_design_action
            ));
        }
        if row.selected
            && row.design_status == "policy-review"
            && row.next_design_action.trim().is_empty()
        {
            failures.push(format!("{} policy review has no next action", row.route));
        }
    }
    failures
}

#[derive(Debug, Clone, serde::Deserialize)]
struct T1ScoreExceptionRow {
    route: String,
    decision: String,
    exception_type: String,
    rationale: String,
    evidence_status: String,
    artifact: String,
    replacement_candidate: String,
    next_selector_action: String,
}

pub(crate) fn load_t1_score_exceptions(path: &Path) -> Result<Vec<T1ScoreExceptionRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_score_exceptions(file)
}

pub(crate) fn parse_t1_score_exceptions<R: std::io::Read>(reader: R) -> Result<Vec<T1ScoreExceptionRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

pub(crate) fn print_t1_score_exceptions(
    review_rows: &[T1DesignReviewCsvRow],
    exception_rows: &[T1ScoreExceptionRow],
    details: bool,
) {
    let score_only_count = review_rows
        .iter()
        .filter(|row| row.selected && row.design_role == "score-backbone-exception")
        .count();
    let mut by_decision = std::collections::BTreeMap::<String, usize>::new();
    for row in exception_rows {
        *by_decision.entry(row.decision.clone()).or_insert(0) += 1;
    }

    println!("route t1-score-exceptions");
    println!("  score-only selected T1 routes: {score_only_count}");
    println!("  exception rows: {}", exception_rows.len());
    println!("  decisions: {}", format_count_map(&by_decision));
    println!();
    println!(
        "{:<8} {:<24} {:<22} Replacement",
        "Route", "Decision", "Exception"
    );
    println!("{}", "-".repeat(92));
    for row in exception_rows {
        println!(
            "{:<8} {:<24} {:<22} {}",
            row.route, row.decision, row.exception_type, row.replacement_candidate
        );
        if details {
            println!("  rationale: {}", row.rationale);
            println!("  evidence: {}", row.evidence_status);
            println!("  artifact: {}", row.artifact);
            println!("  next: {}", row.next_selector_action);
        }
    }
}

// `t1_score_exception_gate_failures` moved to support::tier

pub(crate) fn stop_candidate_selector_score(row: &StopCandidateRow) -> u16 {
    stop_class_selector_score(&row.requested_class) * 100
        + selector_signal_score(&row.transfer_value) * 12
        + selector_signal_score(&row.freight_volume) * 12
        + selector_signal_score(&row.resilience_value) * 8
        + selector_signal_score(&row.land_ops_feasibility) * 4
        + selector_signal_score(&row.equity_community)
}

pub(crate) fn stop_class_selector_score(value: &str) -> u16 {
    match value.trim().to_ascii_uppercase().as_str() {
        "S1" => 5,
        "S2" => 4,
        "S3" => 3,
        "S4" => 2,
        "S5" => 1,
        _ => 0,
    }
}

pub(crate) fn selector_signal_score(value: &str) -> u16 {
    match value.trim().to_ascii_lowercase().as_str() {
        "high" | "met" | "required" => 3,
        "medium" | "planned" | "review_needed" => 2,
        "low" => 1,
        _ => 0,
    }
}

pub(crate) fn parse_stop_candidates<R: std::io::Read>(reader: R) -> Result<Vec<StopCandidateRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

pub(crate) fn filter_stop_candidates<'a>(
    rows: &'a [StopCandidateRow],
    stop_class: Option<&str>,
    route: Option<&str>,
) -> Vec<&'a StopCandidateRow> {
    let route = route.map(normalise_designation);
    rows.iter()
        .filter(|row| {
            stop_class
                .map(|class| {
                    row.requested_class
                        .trim()
                        .eq_ignore_ascii_case(class.trim())
                })
                .unwrap_or(true)
        })
        .filter(|row| {
            route
                .as_ref()
                .map(|route| {
                    stop_candidate_routes(row)
                        .iter()
                        .any(|candidate| candidate == route)
                })
                .unwrap_or(true)
        })
        .collect()
}

pub(crate) fn stop_candidate_routes(row: &StopCandidateRow) -> Vec<String> {
    row.route_refs
        .split([';', ','])
        .map(|route| normalise_designation(route.trim()))
        .filter(|route| !route.is_empty())
        .collect()
}

pub(crate) fn stop_plan_for_route<'a>(rows: &'a [StopCandidateRow], route: &str) -> Vec<&'a StopCandidateRow> {
    let mut stops = filter_stop_candidates(rows, None, Some(route));
    sort_stops_for_route(&mut stops);
    stops
}

pub(crate) fn sort_stops_for_route(stops: &mut [&StopCandidateRow]) {
    if stops.len() < 2 {
        return;
    }
    let coords = stops
        .iter()
        .filter_map(|row| Some((parse_coord(&row.lat)?, parse_coord(&row.lon)?)))
        .collect::<Vec<_>>();
    if coords.len() < 2 {
        stops.sort_by(|a, b| a.name.cmp(&b.name));
        return;
    }
    let (min_lat, max_lat) = coords
        .iter()
        .map(|(lat, _)| *lat)
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), value| {
            (min.min(value), max.max(value))
        });
    let (min_lon, max_lon) = coords
        .iter()
        .map(|(_, lon)| *lon)
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), value| {
            (min.min(value), max.max(value))
        });
    let lat_span = max_lat - min_lat;
    let lon_span = max_lon - min_lon;
    if lat_span >= lon_span {
        stops.sort_by(|a, b| coord_or_default(&a.lat).total_cmp(&coord_or_default(&b.lat)));
    } else {
        stops.sort_by(|a, b| coord_or_default(&a.lon).total_cmp(&coord_or_default(&b.lon)));
    }
}

pub(crate) fn parse_coord(value: &str) -> Option<f64> {
    value
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
}

pub(crate) fn coord_or_default(value: &str) -> f64 {
    parse_coord(value).unwrap_or(0.0)
}

// `print_stop_plan` moved to support::print

pub(crate) fn stop_plan_gate_failures(route: &str, stops: &[&StopCandidateRow]) -> Vec<String> {
    stop_plan_gate_failures_for_tier(route, stops, "T1")
}

// `stop_plan_gate_failures_for_tier` moved to support::gates

#[derive(Debug)]
struct StopCoverageRow {
    route: String,
    stop_count: usize,
    major_stop_count: usize,
    classes: String,
    failures: Vec<String>,
}

pub(crate) fn stop_coverage_for_routes(
    rows: &[StopCandidateRow],
    routes: &[String],
    tier: &str,
) -> Vec<StopCoverageRow> {
    routes
        .iter()
        .map(|route| {
            let plan = stop_plan_for_route(rows, route);
            let mut by_class = std::collections::BTreeMap::new();
            for stop in &plan {
                *by_class
                    .entry(stop.requested_class.trim().to_ascii_uppercase())
                    .or_insert(0usize) += 1;
            }
            let major_stop_count = plan
                .iter()
                .filter(|stop| {
                    matches!(
                        stop.requested_class.trim().to_ascii_uppercase().as_str(),
                        "S1" | "S2"
                    )
                })
                .count();
            StopCoverageRow {
                route: route.clone(),
                stop_count: plan.len(),
                major_stop_count,
                classes: format_count_map(&by_class),
                failures: stop_plan_gate_failures_for_tier(route, &plan, tier),
            }
        })
        .collect()
}

pub(crate) fn stop_coverage_gate_failures(rows: &[StopCoverageRow]) -> Vec<String> {
    rows.iter()
        .filter(|row| !row.failures.is_empty())
        .map(|row| format!("{}: {}", row.route, row.failures.join("; ")))
        .collect()
}

pub(crate) fn print_stop_coverage(tier: &str, rows: &[StopCoverageRow], blockers: bool) {
    let visible = rows
        .iter()
        .filter(|row| !blockers || !row.failures.is_empty())
        .collect::<Vec<_>>();
    let passing = rows.iter().filter(|row| row.failures.is_empty()).count();
    println!("  tier: {tier}");
    println!("  routes: {}", rows.len());
    println!("  passing stop plans: {passing}");
    println!("  blockers: {}", rows.len().saturating_sub(passing));
    println!();
    println!(
        "{:<8} {:>5} {:>8} {:<22} Status",
        "Route", "Stops", "S1/S2", "Class mix"
    );
    println!("{}", "-".repeat(72));
    for row in visible {
        println!(
            "{:<8} {:>5} {:>8} {:<22} {}",
            row.route,
            row.stop_count,
            row.major_stop_count,
            truncate_for_table(&row.classes, 22),
            if row.failures.is_empty() {
                "pass".to_string()
            } else {
                truncate_for_table(&row.failures.join("; "), 28)
            }
        );
    }
}

// `print_stop_candidates` moved to support::print

// `stop_candidate_gate_failures` moved to support::gates

pub(crate) fn valid_stop_evidence_status(status: &str) -> bool {
    matches!(
        status.trim().to_ascii_lowercase().as_str(),
        "validated"
            | "heuristic"
            | "planned"
            | "partial"
            | "source_needed"
            | "missing_source"
            | "missing_graph_data"
    )
}

pub(crate) fn high_or_medium(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "high" | "medium" | "met"
    )
}

// `print_tier_connectivity` moved to support::print

// `print_bridge_standard_coverage` moved to support::print

pub(crate) fn bridge_standard_missing_routes(
    routes: &[String],
    nbi: &std::collections::HashMap<String, NbiBridgeRecord>,
) -> Vec<String> {
    routes
        .iter()
        .filter(|route| !nbi.contains_key(*route))
        .cloned()
        .collect()
}

#[derive(Debug, Clone, serde::Deserialize)]
struct PressureScenarioRow {
    scenario_id: String,
    scenario_name: String,
    adversity_class: String,
    standards_tested: String,
    current_status: String,
    existing_artifact: String,
    blocking_gap: String,
    next_evidence_step: String,
}

pub(crate) fn load_pressure_scenarios(path: &Path) -> Result<Vec<PressureScenarioRow>> {
    let file = std::fs::File::open(path)?;
    parse_pressure_scenarios(file)
}

pub(crate) fn parse_pressure_scenarios<R: std::io::Read>(reader: R) -> Result<Vec<PressureScenarioRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

pub(crate) fn print_pressure_scenarios(rows: &[PressureScenarioRow], blockers: bool, details: bool) {
    let failures = pressure_scenario_gate_failures(rows);
    let filtered = if blockers {
        failures.clone()
    } else {
        rows.iter().collect::<Vec<_>>()
    };
    let mut by_status: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for row in rows {
        *by_status.entry(row.current_status.clone()).or_insert(0) += 1;
    }

    println!("route pressure-scenarios");
    println!(
        "  scenarios: {} shown / {} total",
        filtered.len(),
        rows.len()
    );
    println!("  status: {}", format_count_map(&by_status));
    println!("  L2 gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<18} {:<24} {:<14} {:<28} {}",
        "Scenario", "Name", "Status", "Adversity", "Gap"
    );
    println!("{}", "-".repeat(132));
    for row in filtered {
        println!(
            "{:<18} {:<24} {:<14} {:<28} {}",
            row.scenario_id,
            truncate_for_table(&row.scenario_name, 24),
            row.current_status,
            truncate_for_table(&row.adversity_class, 28),
            row.blocking_gap
        );
        if details {
            println!("  standards: {}", row.standards_tested);
            println!("  artifact: {}", row.existing_artifact);
            println!("  next: {}", row.next_evidence_step);
        }
    }
}

pub(crate) fn pressure_scenario_gate_failures(rows: &[PressureScenarioRow]) -> Vec<&PressureScenarioRow> {
    rows.iter()
        .filter(|row| !pressure_scenario_has_bounded_contract(row))
        .collect()
}

pub(crate) fn pressure_scenario_readiness_gate_failures(
    rows: &[PressureScenarioRow],
) -> Vec<&PressureScenarioRow> {
    rows.iter()
        .filter(|row| !pressure_scenario_is_executable(row))
        .collect()
}

pub(crate) fn pressure_scenario_missing_required_adversity(rows: &[PressureScenarioRow]) -> Vec<&'static str> {
    const REQUIRED: &[(&str, &[&str])] = &[
        ("T1/T1 closure", &["t1/t1"]),
        ("corridor segment closure", &["corridor segment", "closure"]),
        ("port surge", &["port surge"]),
        ("weather/flood disruption", &["weather", "flood"]),
        ("relay hub outage", &["relay hub outage"]),
        ("EV/rest-area outage", &["ev/rest-area outage"]),
        ("managed-lane sensitivity", &["managed-lane"]),
    ];

    REQUIRED
        .iter()
        .filter_map(|(label, terms)| {
            let covered = rows.iter().any(|row| {
                let class = row.adversity_class.to_ascii_lowercase();
                if *label == "weather/flood disruption" {
                    terms.iter().any(|term| class.contains(term))
                } else {
                    terms.iter().all(|term| class.contains(term))
                }
            });
            (!covered).then_some(*label)
        })
        .collect()
}

// `print_pressure_standard_coverage` moved to support::print

pub(crate) fn pressure_standard_coverage_failures<'a>(
    standards: &'a [StandardsProofRow],
    scenarios: &[PressureScenarioRow],
) -> Vec<&'a StandardsProofRow> {
    let scenario_refs = pressure_standard_scenario_refs(scenarios);
    pressure_standard_coverage_focus(standards)
        .into_iter()
        .filter(|row| !scenario_refs.contains_key(row.standard_id.as_str()))
        .collect()
}

pub(crate) fn pressure_standard_coverage_focus(standards: &[StandardsProofRow]) -> Vec<&StandardsProofRow> {
    standards
        .iter()
        .filter(|row| {
            row.tier == "T1"
                && matches!(row.standard_family.as_str(), "throughput" | "resilience")
                && !row.evidence_level.eq_ignore_ascii_case("deprecated")
        })
        .collect()
}

pub(crate) fn pressure_scenario_unknown_standard_refs(
    standards: &[StandardsProofRow],
    scenarios: &[PressureScenarioRow],
) -> Vec<String> {
    let known = standards
        .iter()
        .map(|row| row.standard_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    let mut unknown = pressure_standard_scenario_refs(scenarios)
        .keys()
        .filter(|standard_id| !known.contains(**standard_id))
        .map(|standard_id| (*standard_id).to_string())
        .collect::<Vec<_>>();
    unknown.sort();
    unknown
}

pub(crate) fn pressure_standard_scenario_refs(
    scenarios: &[PressureScenarioRow],
) -> std::collections::BTreeMap<&str, Vec<String>> {
    let mut refs = std::collections::BTreeMap::new();
    for row in scenarios {
        for standard_id in row
            .standards_tested
            .split(';')
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            refs.entry(standard_id)
                .or_insert_with(Vec::new)
                .push(row.scenario_id.clone());
        }
    }
    refs
}

pub(crate) fn pressure_scenario_is_executable(row: &PressureScenarioRow) -> bool {
    matches!(
        row.current_status.trim().to_ascii_lowercase().as_str(),
        "implemented" | "heuristic"
    )
}

pub(crate) fn pressure_scenario_has_bounded_contract(row: &PressureScenarioRow) -> bool {
    let has_identity = row.scenario_id.starts_with("S-L2-")
        && !row.scenario_name.trim().is_empty()
        && !row.adversity_class.trim().is_empty();
    let has_test_scope = !row.standards_tested.trim().is_empty()
        && row
            .standards_tested
            .split(';')
            .any(|value| value.trim().starts_with('T'));
    let has_artifact = !row.existing_artifact.trim().is_empty();
    let has_next_step = !row.next_evidence_step.trim().is_empty();
    let status = row.current_status.to_ascii_lowercase();
    let status_is_labeled = matches!(
        status.as_str(),
        "implemented" | "heuristic" | "planned" | "stub" | "deprecated"
    );

    has_identity && has_test_scope && has_artifact && has_next_step && status_is_labeled
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ThroughputProofRow {
    proof_id: String,
    proof_name: String,
    binding_type: String,
    stressor: String,
    primary_metric: String,
    existing_artifact: String,
    current_status: String,
    blocking_gap: String,
    next_evidence_step: String,
}

pub(crate) fn load_throughput_proof_matrix(path: &Path) -> Result<Vec<ThroughputProofRow>> {
    let file = std::fs::File::open(path)?;
    parse_throughput_proof_matrix(file)
}

pub(crate) fn parse_throughput_proof_matrix<R: std::io::Read>(reader: R) -> Result<Vec<ThroughputProofRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

pub(crate) fn print_throughput_proof_matrix(rows: &[ThroughputProofRow], blockers: bool, details: bool) {
    let failures = throughput_proof_gate_failures(rows);
    let filtered = if blockers {
        failures.clone()
    } else {
        rows.iter().collect::<Vec<_>>()
    };
    let mut by_binding: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for row in rows {
        *by_binding.entry(row.binding_type.clone()).or_insert(0) += 1;
    }

    println!("route throughput-proof");
    println!("  rows: {} shown / {} total", filtered.len(), rows.len());
    println!("  binding: {}", format_count_map(&by_binding));
    println!("  gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<18} {:<26} {:<20} {:<12} {}",
        "Proof", "Name", "Binding", "Status", "Gap"
    );
    println!("{}", "-".repeat(132));
    for row in filtered {
        println!(
            "{:<18} {:<26} {:<20} {:<12} {}",
            row.proof_id,
            truncate_for_table(&row.proof_name, 26),
            row.binding_type,
            row.current_status,
            row.blocking_gap
        );
        if details {
            println!("  stressor: {}", row.stressor);
            println!("  metric: {}", row.primary_metric);
            println!("  artifact: {}", row.existing_artifact);
            println!("  next: {}", row.next_evidence_step);
        }
    }
}

pub(crate) fn throughput_proof_gate_failures(rows: &[ThroughputProofRow]) -> Vec<&ThroughputProofRow> {
    rows.iter()
        .filter(|row| !throughput_proof_has_bounded_contract(row))
        .collect()
}

pub(crate) fn throughput_proof_has_bounded_contract(row: &ThroughputProofRow) -> bool {
    let binding = row.binding_type.trim().to_ascii_lowercase();
    let binding_is_labeled = matches!(
        binding.as_str(),
        "congestion_binding" | "resilience_binding"
    );
    row.proof_id.starts_with("TP-")
        && !row.proof_name.trim().is_empty()
        && binding_is_labeled
        && !row.stressor.trim().is_empty()
        && !row.primary_metric.trim().is_empty()
        && !row.existing_artifact.trim().is_empty()
        && standards_evidence_level_is_allowed(&row.current_status)
        && !row.next_evidence_step.trim().is_empty()
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T1FailureRow {
    site_id: String,
    intersection: String,
    location: String,
    failure_mode: String,
    annual_probability: Option<f64>,
    duration_p50_hours: Option<f64>,
    duration_p95_hours: Option<f64>,
    throughput_retention_current: Option<f64>,
    throughput_retention_i2: Option<f64>,
    reroute_time_p50_hours: Option<f64>,
    reroute_time_p95_hours: Option<f64>,
    source_status: String,
    confidence: String,
    current_artifact: String,
    blocking_gap: String,
    next_evidence_step: String,
}

pub(crate) fn load_t1_failure_ledger(path: &Path) -> Result<Vec<T1FailureRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_failure_ledger(file)
}

pub(crate) fn parse_t1_failure_ledger<R: std::io::Read>(reader: R) -> Result<Vec<T1FailureRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

pub(crate) fn write_t1_failure_ledger(path: &Path, rows: &[T1FailureRow]) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)?;
    for row in rows {
        wtr.serialize(row)?;
    }
    wtr.flush()?;
    Ok(())
}

// `print_t1_failures` moved to support::print

pub(crate) fn t1_failure_evidence_gate_failures(rows: &[T1FailureRow]) -> Vec<&T1FailureRow> {
    rows.iter()
        .filter(|row| !t1_failure_row_has_evidence_contract(row))
        .collect()
}

pub(crate) fn t1_failure_row_has_evidence_contract(row: &T1FailureRow) -> bool {
    let status = row.source_status.trim().to_ascii_lowercase();
    let status_is_labeled = matches!(status.as_str(), "empirical" | "modeled" | "source_needed");
    let confidence = row.confidence.trim().to_ascii_lowercase();
    let confidence_is_labeled =
        matches!(confidence.as_str(), "high" | "medium" | "low" | "unknown");
    let source_needed_has_gap = status != "source_needed" || !row.blocking_gap.trim().is_empty();

    !row.site_id.trim().is_empty()
        && !row.intersection.trim().is_empty()
        && !row.failure_mode.trim().is_empty()
        && status_is_labeled
        && confidence_is_labeled
        && !row.current_artifact.trim().is_empty()
        && !row.next_evidence_step.trim().is_empty()
        && source_needed_has_gap
}

pub(crate) fn fmt_opt(value: Option<f64>) -> String {
    value
        .map(|v| format!("{v:.3}"))
        .unwrap_or_else(|| "-".to_string())
}

const EXPECTED_T1_DIAMOND_SITES: &[&str] = &[
    "T1X-I80-I90",
    "T1X-I35-I80",
    "T1X-I35-I40",
    "T1X-I40-I75",
    "T1X-I10-I35",
    "T1X-I75-I80",
    "T1X-I90-I95",
    "T1X-I10-I95",
    "T1X-I5-I10",
    "T1X-I5-I80",
    "T1X-I5-I90",
    "T1X-I35-I90",
    "T1X-I40-I95",
    "T1X-I75-I90",
    "T1X-I5-I40",
];

#[derive(Debug, Clone, serde::Deserialize)]
struct T1DiamondValidationRow {
    site_id: String,
    intersection: String,
    location: String,
    priority_band: String,
    anchor_lon: f64,
    anchor_lat: f64,
    analyzer_status: String,
    manual_geometry_status: String,
    alternate_capacity_status: String,
    observed_failure_status: String,
    validation_status: String,
    current_artifact: String,
    blocking_gap: String,
    next_validation_step: String,
}

pub(crate) fn load_t1_diamond_validation(path: &Path) -> Result<Vec<T1DiamondValidationRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_diamond_validation(file)
}

pub(crate) fn parse_t1_diamond_validation<R: std::io::Read>(reader: R) -> Result<Vec<T1DiamondValidationRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

// `print_t1_diamond_validation` moved to support::print

#[derive(Debug, Clone, PartialEq, Eq)]
struct T1DiamondValidationTask {
    priority_band: String,
    category: &'static str,
    site_id: String,
    intersection: String,
    location: String,
    action: String,
    source_action: Option<String>,
}

pub(crate) fn print_t1_diamond_validation_docket(
    rows: &[T1DiamondValidationRow],
    priority: Option<&str>,
    source_rows: Option<&[T1SourceHealthRow]>,
    details: bool,
) {
    let tasks = t1_diamond_validation_tasks(rows, priority, source_rows);
    let mut by_category: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for task in &tasks {
        *by_category.entry(task.category.to_string()).or_insert(0) += 1;
    }

    println!("route t1-diamond-validation --docket");
    println!("  tasks: {} shown", tasks.len());
    println!("  categories: {}", format_count_map(&by_category));
    println!();
    println!(
        "{:<8} {:<20} {:<18} {:<14} {}",
        "Priority", "Category", "Site", "Intersection", "Action"
    );
    println!("{}", "-".repeat(132));
    for task in tasks {
        println!(
            "{:<8} {:<20} {:<18} {:<14} {}",
            task.priority_band, task.category, task.site_id, task.intersection, task.action
        );
        if details {
            println!("  location: {}", task.location);
            if let Some(source_action) = &task.source_action {
                println!("  source: {source_action}");
            }
        }
    }
}

// `t1_diamond_validation_tasks` moved to support::tier

pub(crate) fn t1_diamond_validation_task(
    row: &T1DiamondValidationRow,
    category: &'static str,
    action: &str,
    source_action: Option<String>,
) -> T1DiamondValidationTask {
    T1DiamondValidationTask {
        priority_band: row.priority_band.clone(),
        category,
        site_id: row.site_id.clone(),
        intersection: row.intersection.clone(),
        location: row.location.clone(),
        action: action.to_string(),
        source_action,
    }
}

pub(crate) fn t1_source_health_by_site(
    rows: &[T1SourceHealthRow],
) -> std::collections::HashMap<&str, &T1SourceHealthRow> {
    let mut by_site = std::collections::HashMap::new();
    for row in rows {
        by_site.entry(row.site_id.as_str()).or_insert(row);
    }
    by_site
}

pub(crate) fn t1_diamond_priority_rank(priority: &str) -> usize {
    match priority.trim().to_ascii_uppercase().as_str() {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        _ => 99,
    }
}

pub(crate) fn t1_diamond_validation_gate_failures(
    rows: &[T1DiamondValidationRow],
) -> Vec<&T1DiamondValidationRow> {
    rows.iter()
        .filter(|row| !t1_diamond_validation_row_has_contract(row))
        .collect()
}

pub(crate) fn t1_diamond_validation_missing_sites(rows: &[T1DiamondValidationRow]) -> Vec<String> {
    let present: std::collections::HashSet<_> =
        rows.iter().map(|row| row.site_id.as_str()).collect();
    EXPECTED_T1_DIAMOND_SITES
        .iter()
        .filter(|site_id| !present.contains(**site_id))
        .map(|site_id| (*site_id).to_string())
        .collect()
}

pub(crate) fn t1_diamond_validation_row_has_contract(row: &T1DiamondValidationRow) -> bool {
    let analyzer = row.analyzer_status.trim().to_ascii_lowercase();
    let geometry = row.manual_geometry_status.trim().to_ascii_lowercase();
    let alternate = row.alternate_capacity_status.trim().to_ascii_lowercase();
    let observed = row.observed_failure_status.trim().to_ascii_lowercase();
    let validation = row.validation_status.trim().to_ascii_lowercase();

    !row.site_id.trim().is_empty()
        && EXPECTED_T1_DIAMOND_SITES.contains(&row.site_id.as_str())
        && !row.intersection.trim().is_empty()
        && !row.location.trim().is_empty()
        && !row.priority_band.trim().is_empty()
        && row.anchor_lon.is_finite()
        && row.anchor_lat.is_finite()
        && matches!(analyzer.as_str(), "recognized" | "missing" | "conflict")
        && matches!(
            geometry.as_str(),
            "validated" | "heuristic" | "pending" | "conflict"
        )
        && matches!(alternate.as_str(), "validated" | "heuristic" | "pending")
        && matches!(observed.as_str(), "empirical" | "modeled" | "source_needed")
        && matches!(
            validation.as_str(),
            "validated" | "heuristic" | "pending" | "conflict"
        )
        && !row.current_artifact.trim().is_empty()
        && !row.next_validation_step.trim().is_empty()
        && (validation == "validated" || !row.blocking_gap.trim().is_empty())
}

#[derive(Debug, Clone, serde::Deserialize)]
struct T1FailureSourceRow {
    site_id: String,
    intersection: String,
    location: String,
    primary_state_sources: String,
    national_sources: String,
    fields_to_populate: String,
    access_status: String,
    source_url: String,
    notes: String,
}

pub(crate) fn load_t1_failure_source_plan(path: &Path) -> Result<Vec<T1FailureSourceRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_failure_source_plan(file)
}

pub(crate) fn parse_t1_failure_source_plan<R: std::io::Read>(reader: R) -> Result<Vec<T1FailureSourceRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

pub(crate) fn print_t1_failure_sources(rows: &[T1FailureSourceRow], lookup_needed: bool) {
    let filtered: Vec<&T1FailureSourceRow> = rows
        .iter()
        .filter(|row| !lookup_needed || row.access_status.eq_ignore_ascii_case("lookup_needed"))
        .collect();
    let identified = rows
        .iter()
        .filter(|row| row.access_status.eq_ignore_ascii_case("identified"))
        .count();
    let lookup = rows
        .iter()
        .filter(|row| row.access_status.eq_ignore_ascii_case("lookup_needed"))
        .count();

    println!("route t1-failure-sources");
    println!("  sources: {} shown / {} total", filtered.len(), rows.len());
    println!("  access: identified {identified}, lookup_needed {lookup}");
    println!();
    println!(
        "{:<18} {:<14} {:<18} {:<14} {}",
        "Site", "Intersection", "Location", "Access", "Primary sources"
    );
    println!("{}", "-".repeat(120));
    for row in filtered {
        println!(
            "{:<18} {:<14} {:<18} {:<14} {}",
            row.site_id,
            row.intersection,
            row.location,
            row.access_status,
            row.primary_state_sources
        );
        println!("  fields: {}", row.fields_to_populate);
        println!("  national: {}", row.national_sources);
        if !row.source_url.trim().is_empty() {
            println!("  url: {}", row.source_url);
        }
        println!("  notes: {}", row.notes);
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct T1SourceHealthRow {
    site_id: String,
    source_name: String,
    source_url: String,
    source_kind: String,
    access_health: String,
    ingestion_status: String,
    history_status: String,
    last_checked: String,
    blocking_gap: String,
    next_step: String,
}

pub(crate) fn load_t1_source_health(path: &Path) -> Result<Vec<T1SourceHealthRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_source_health(file)
}

pub(crate) fn parse_t1_source_health<R: std::io::Read>(reader: R) -> Result<Vec<T1SourceHealthRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

// `print_t1_source_health` moved to support::print

pub(crate) fn t1_source_health_blockers(rows: &[T1SourceHealthRow]) -> Vec<&T1SourceHealthRow> {
    rows.iter()
        .filter(|row| t1_source_health_is_blocked(row))
        .collect()
}

pub(crate) fn t1_source_health_is_blocked(row: &T1SourceHealthRow) -> bool {
    !matches!(
        (
            row.access_health.as_str(),
            row.ingestion_status.as_str(),
            row.history_status.as_str()
        ),
        ("live", "implemented", "snapshot_only") | ("live", "documented", "historical_method")
    )
}

// `print_t1_access_docket` moved to support::print

#[derive(Debug, Clone, PartialEq, Eq)]
struct T1AccessDocketItem {
    site_id: String,
    source_name: String,
    source_url: String,
    access_health: String,
    history_status: String,
    blocking_gap: String,
    category: String,
    priority: String,
    action: String,
}

pub(crate) fn t1_access_docket_item(row: &T1SourceHealthRow) -> T1AccessDocketItem {
    let category = t1_access_category(row).to_string();
    let priority = t1_access_priority(row).to_string();
    let action = match category.as_str() {
        "api_key" => format!("Request credentials; then implement {}", row.source_name),
        "account" => format!("Obtain account/export; then map {}", row.source_name),
        "access_request" => format!(
            "Request data access or partner extract for {}",
            row.source_name
        ),
        "endpoint_tuning" => format!("Tune query/export path for {}", row.source_name),
        "records_request" => format!(
            "Request archive/export or identify allowed endpoint for {}",
            row.source_name
        ),
        _ => row.next_step.clone(),
    };
    T1AccessDocketItem {
        site_id: row.site_id.clone(),
        source_name: row.source_name.clone(),
        source_url: row.source_url.clone(),
        access_health: row.access_health.clone(),
        history_status: row.history_status.clone(),
        blocking_gap: row.blocking_gap.clone(),
        category,
        priority,
        action,
    }
}

pub(crate) fn t1_access_category(row: &T1SourceHealthRow) -> &'static str {
    match row.access_health.as_str() {
        "requires_key" => "api_key",
        "requires_account" => "account",
        "requires_access" => "access_request",
        "blocked_query" => "endpoint_tuning",
        "blocked_access" => "records_request",
        _ if row.ingestion_status != "implemented" => "implementation",
        _ if row.history_status == "snapshot_only" => "history_archive",
        _ => "monitoring",
    }
}

pub(crate) fn t1_access_priority(row: &T1SourceHealthRow) -> &'static str {
    if row.source_kind == "travel_time_reliability" {
        "critical"
    } else if row.access_health == "blocked_query" || row.access_health == "blocked_access" {
        "high"
    } else if row.access_health == "requires_access" || row.access_health == "requires_key" {
        "high"
    } else {
        "medium"
    }
}

pub(crate) fn t1_access_priority_rank(priority: &str) -> u8 {
    match priority {
        "critical" => 0,
        "high" => 1,
        "medium" => 2,
        _ => 3,
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct T1SnapshotPlanRow {
    site_id: String,
    intersection: String,
    priority_band: String,
    source_name: String,
    source_health: String,
    cadence: String,
    fetch_command: String,
    import_command: String,
    accumulate_command: String,
    raw_output: String,
    normalized_output: String,
    accumulated_output: String,
    blocking_gap: String,
    next_step: String,
}

pub(crate) fn load_t1_snapshot_plan(path: &Path) -> Result<Vec<T1SnapshotPlanRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_snapshot_plan(file)
}

pub(crate) fn parse_t1_snapshot_plan<R: std::io::Read>(reader: R) -> Result<Vec<T1SnapshotPlanRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

// `print_t1_snapshot_plan` moved to support::print

pub(crate) fn print_t1_snapshot_script(rows: &[T1SnapshotPlanRow], priority: Option<&str>) {
    let filtered = filtered_t1_snapshot_rows(rows, priority);

    println!("route t1-snapshot-plan --script");
    println!("  feeds: {} shown / {} total", filtered.len(), rows.len());
    println!();
    for row in filtered {
        println!(
            "# {} {} ({})",
            row.site_id, row.intersection, row.source_name
        );
        println!("{}", row.fetch_command);
        println!("{}", row.import_command);
        println!("{}", row.accumulate_command);
        println!();
    }
}

pub(crate) fn filtered_t1_snapshot_rows<'a>(
    rows: &'a [T1SnapshotPlanRow],
    priority: Option<&str>,
) -> Vec<&'a T1SnapshotPlanRow> {
    rows.iter()
        .filter(|row| {
            priority
                .map(|priority| row.priority_band.eq_ignore_ascii_case(priority))
                .unwrap_or(true)
        })
        .collect()
}

pub(crate) fn t1_snapshot_plan_gate_failures(rows: &[T1SnapshotPlanRow]) -> Vec<&T1SnapshotPlanRow> {
    rows.iter()
        .filter(|row| !t1_snapshot_plan_row_has_contract(row))
        .collect()
}

pub(crate) fn t1_snapshot_plan_row_has_contract(row: &T1SnapshotPlanRow) -> bool {
    !row.site_id.trim().is_empty()
        && !row.intersection.trim().is_empty()
        && !row.priority_band.trim().is_empty()
        && !row.source_name.trim().is_empty()
        && row.source_health.trim() == "live/implemented/snapshot_only"
        && matches!(
            row.cadence.trim(),
            "daily" | "twice_daily" | "hourly" | "weekly"
        )
        && row.fetch_command.trim().starts_with("route t1-fetch-")
        && row.import_command.trim().starts_with("route t1-import-")
        && row
            .accumulate_command
            .trim()
            .starts_with("route t1-accumulate-events")
        && row.raw_output.trim().ends_with(".json")
        && row.normalized_output.trim().ends_with(".csv")
        && row.accumulated_output.trim().ends_with(".csv")
        && !row.blocking_gap.trim().is_empty()
        && !row.next_step.trim().is_empty()
}

#[derive(Debug, Clone, serde::Deserialize)]
struct T1EvidenceWindowRow {
    window_id: String,
    site_id: String,
    source_name: String,
    evidence_mode: String,
    capture_started_at: String,
    capture_ended_at: String,
    observation_start: String,
    observation_end: String,
    raw_artifact: String,
    normalized_artifact: String,
    event_count: usize,
    freight_relevant_count: usize,
    promotion_eligible: bool,
    blocking_gap: String,
    next_step: String,
    review_artifact: String,
}

pub(crate) fn load_t1_evidence_windows(path: &Path) -> Result<Vec<T1EvidenceWindowRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_evidence_windows(file)
}

pub(crate) fn parse_t1_evidence_windows<R: std::io::Read>(reader: R) -> Result<Vec<T1EvidenceWindowRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

// `print_t1_evidence_windows` moved to support::print

pub(crate) fn t1_evidence_window_gate_failures(rows: &[T1EvidenceWindowRow]) -> Vec<String> {
    if rows.is_empty() {
        return vec!["no evidence-window rows found".to_string()];
    }

    let mut failures = Vec::new();
    for row in rows {
        if !t1_evidence_window_has_contract(row) {
            failures.push(format!(
                "{} lacks required source-window metadata",
                row.window_id
            ));
        }
        if row.promotion_eligible && !t1_evidence_window_can_promote(row) {
            failures.push(format!(
                "{} is promotion eligible without repeated-window or archive evidence",
                row.window_id
            ));
        }
        if row.evidence_mode.trim() == "snapshot_only" && row.promotion_eligible {
            failures.push(format!(
                "{} marks snapshot-only evidence as promotion eligible",
                row.window_id
            ));
        }
    }
    failures
}

pub(crate) fn t1_evidence_window_has_contract(row: &T1EvidenceWindowRow) -> bool {
    !row.window_id.trim().is_empty()
        && !row.site_id.trim().is_empty()
        && !row.source_name.trim().is_empty()
        && matches!(
            row.evidence_mode.trim(),
            "snapshot_only" | "repeated_window" | "historical_archive" | "enrichment_blocker"
        )
        && !row.capture_started_at.trim().is_empty()
        && !row.capture_ended_at.trim().is_empty()
        && !row.raw_artifact.trim().is_empty()
        && !row.normalized_artifact.trim().is_empty()
        && row.freight_relevant_count <= row.event_count
        && !row.blocking_gap.trim().is_empty()
        && !row.next_step.trim().is_empty()
        && !row.review_artifact.trim().is_empty()
}

pub(crate) fn t1_evidence_window_can_promote(row: &T1EvidenceWindowRow) -> bool {
    matches!(
        row.evidence_mode.trim(),
        "repeated_window" | "historical_archive"
    ) && !row.observation_start.trim().is_empty()
        && !row.observation_end.trim().is_empty()
        && row.event_count > 0
}

pub(crate) fn format_count_map(counts: &std::collections::BTreeMap<String, usize>) -> String {
    if counts.is_empty() {
        "none".to_string()
    } else {
        counts
            .iter()
            .map(|(key, count)| format!("{key}: {count}"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

pub(crate) fn truncate_for_table(value: &str, width: usize) -> String {
    if value.chars().count() <= width {
        value.to_string()
    } else {
        value
            .chars()
            .take(width.saturating_sub(1))
            .collect::<String>()
            + "…"
    }
}

pub(crate) fn beck_t2_diagnostics_gate_failure(review_flag: &str) -> bool {
    matches!(
        review_flag,
        "unstopped-t1-contact-review"
            | "parallel-spacing-review"
            | "split-anchor-review"
            | "dense-label-review"
    )
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
struct T1FailureEventRow {
    site_id: String,
    event_id: String,
    source: String,
    source_event_id: String,
    observation_year: u16,
    start_time: String,
    end_time: String,
    duration_hours: Option<f64>,
    event_type: String,
    full_closure: bool,
    lanes_closed: Option<u8>,
    freight_relevant: bool,
    confidence: String,
    notes: String,
}

#[derive(Debug, Clone, PartialEq)]
struct T1FailureEventSummary {
    site_id: String,
    observed_years: usize,
    event_count: usize,
    annual_rate: f64,
    annual_probability: f64,
    duration_p50_hours: Option<f64>,
    duration_p95_hours: Option<f64>,
    confidence: String,
}

pub(crate) fn load_t1_failure_events(path: &Path) -> Result<Vec<T1FailureEventRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_failure_events(file)
}

pub(crate) fn write_t1_failure_events(path: &Path, rows: &[T1FailureEventRow]) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut wtr = csv::Writer::from_path(path)?;
    for row in rows {
        wtr.serialize(row)?;
    }
    wtr.flush()?;
    Ok(())
}

pub(crate) fn parse_t1_failure_events<R: std::io::Read>(reader: R) -> Result<Vec<T1FailureEventRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    let mut rows = Vec::new();
    for result in rdr.deserialize() {
        rows.push(result?);
    }
    Ok(rows)
}

pub(crate) fn merge_t1_failure_events(
    existing: &[T1FailureEventRow],
    incoming: &[T1FailureEventRow],
) -> Vec<T1FailureEventRow> {
    let mut rows = existing.to_vec();
    let mut seen = rows
        .iter()
        .map(t1_failure_event_key)
        .collect::<std::collections::BTreeSet<_>>();

    for row in incoming {
        if seen.insert(t1_failure_event_key(row)) {
            rows.push(row.clone());
        }
    }

    rows.sort_by(|a, b| {
        a.site_id
            .cmp(&b.site_id)
            .then_with(|| a.observation_year.cmp(&b.observation_year))
            .then_with(|| a.event_id.cmp(&b.event_id))
    });
    rows
}

pub(crate) fn t1_failure_event_key(row: &T1FailureEventRow) -> (String, String) {
    (row.site_id.clone(), row.event_id.clone())
}

pub(crate) fn fetch_iowa511_events(output: &Path) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let url = "https://services.arcgis.com/8lRhdTsQyJpO52F1/arcgis/rest/services/CARS511_Iowa_View/FeatureServer/0/query?f=json&where=1%3D1&outFields=ID,Route,StartTime,EndTime,IssueDate,IssueTime,headline,cause,Restrict_,Desc0&returnGeometry=true&outSR=4326";
    let body = reqwest::blocking::get(url)?.error_for_status()?.text()?;
    ensure_no_arcgis_error(&body)?;
    atomic_write_text(output, body)?;
    Ok(())
}

pub(crate) fn fetch_tdot_smartway_events(output: &Path, timeout_seconds: u64) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let url = "https://spatial.tdot.tn.gov/arcgis/rest/services/Smartway/Smartway_Events/FeatureServer/1/query?f=json&where=1%3D1&outFields=ID,START_DATE,END_DATE,CD_ROAD_NAMES,CD_DIRECTION,EVENT_TYPE,EVENT_SUBTYPE,DESCRIPTION,HAS_CLOSURE,MIDPOINT_LATITUDE_DD,MIDPOINT_LONGITUDE_DD,COUNTY_NAME&returnGeometry=false&resultRecordCount=200";
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_seconds.max(1)))
        .build()?;
    let body = client.get(url).send()?.error_for_status()?.text()?;
    ensure_no_arcgis_error(&body)?;
    atomic_write_text(output, body)?;
    Ok(())
}

pub(crate) fn fetch_mdot_midrive_events(output: &Path) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let url = "https://mdotjboss.state.mi.us/MiDrive/incidents/AllForMap/";
    let body = reqwest::blocking::get(url)?.error_for_status()?.text()?;
    atomic_write_text(output, body)?;
    Ok(())
}

// `fetch_indot_trafficwise_events` moved to support::misc

pub(crate) fn ensure_no_arcgis_error(json: &str) -> Result<()> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    if let Some(error) = value.get("error") {
        let message = error
            .get("message")
            .and_then(|value| value.as_str())
            .unwrap_or("ArcGIS query failed");
        let details = error
            .get("details")
            .and_then(|value| value.as_array())
            .map(|values| {
                values
                    .iter()
                    .filter_map(|value| value.as_str())
                    .collect::<Vec<_>>()
                    .join("; ")
            })
            .unwrap_or_default();
        if details.is_empty() {
            anyhow::bail!("{message}");
        } else {
            anyhow::bail!("{message}: {details}");
        }
    }
    Ok(())
}

pub(crate) fn ensure_no_graphql_errors(json: &str) -> Result<()> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    if let Some(errors) = value.get("errors").and_then(|value| value.as_array()) {
        let messages = errors
            .iter()
            .filter_map(|error| error.get("message").and_then(|value| value.as_str()))
            .collect::<Vec<_>>()
            .join("; ");
        if messages.is_empty() {
            anyhow::bail!("GraphQL query failed");
        } else {
            anyhow::bail!("{messages}");
        }
    }
    Ok(())
}

// `parse_iowa511_events` moved to support::misc

// `parse_tdot_smartway_events` moved to support::misc

// `parse_mdot_midrive_events` moved to support::misc

// `parse_indot_trafficwise_events` moved to support::misc

pub(crate) fn indot_trafficwise_event_millis(feature: &serde_json::Value, key: &str) -> Option<i64> {
    feature
        .get("_eventReport")
        .and_then(|value| value.get(key))
        .and_then(|value| value.get("time"))
        .and_then(json_value_i64)
        .or_else(|| {
            feature
                .get(key)
                .and_then(|value| value.get("timestamp"))
                .and_then(json_value_i64)
        })
        .or_else(|| {
            feature
                .get("_eventMapFeature")
                .and_then(|value| {
                    if key == "beginTime" {
                        value.get("startTime")
                    } else {
                        value.get(key)
                    }
                })
                .and_then(|value| value.get("time"))
                .and_then(json_value_i64)
        })
}

pub(crate) fn json_string(attrs: &serde_json::Map<String, serde_json::Value>, key: &str) -> String {
    attrs
        .get(key)
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .trim()
        .to_string()
}

pub(crate) fn json_value_string(value: &serde_json::Value, key: &str) -> String {
    value
        .get(key)
        .and_then(|value| value.as_str())
        .unwrap_or("")
        .trim()
        .to_string()
}

pub(crate) fn json_scalar_to_string(value: &serde_json::Value) -> String {
    value
        .as_str()
        .map(str::to_string)
        .or_else(|| value.as_i64().map(|value| value.to_string()))
        .or_else(|| value.as_u64().map(|value| value.to_string()))
        .unwrap_or_default()
}

pub(crate) fn json_value_i64(value: &serde_json::Value) -> Option<i64> {
    value
        .as_i64()
        .or_else(|| value.as_u64().and_then(|value| i64::try_from(value).ok()))
        .or_else(|| value.as_str().and_then(|value| value.parse().ok()))
}

pub(crate) fn json_f64(attrs: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<f64> {
    attrs.get(key).and_then(|value| {
        value
            .as_f64()
            .or_else(|| value.as_str().and_then(|value| value.parse().ok()))
    })
}

pub(crate) fn json_i64(attrs: &serde_json::Map<String, serde_json::Value>, key: &str) -> Option<i64> {
    attrs.get(key).and_then(|value| {
        value
            .as_i64()
            .or_else(|| value.as_str().and_then(|value| value.parse().ok()))
    })
}

pub(crate) fn iowa511_is_t1_relevant(route: &str, text: &str) -> bool {
    let route_norm = route.to_ascii_uppercase().replace(' ', "");
    let text_norm = text.to_ascii_uppercase();
    (route_norm.contains("I-35")
        || route_norm.contains("I35")
        || route_norm.contains("I-80")
        || route_norm.contains("I80"))
        && ["CLOSED", "CLOSURE", "CONSTRUCTION", "CRASH", "INCIDENT"]
            .iter()
            .any(|needle| text_norm.contains(needle))
}

pub(crate) fn tdot_smartway_is_t1_relevant(road_names: &str, text: &str) -> bool {
    let route_norm = road_names.to_ascii_uppercase().replace(' ', "");
    let text_norm = text.to_ascii_uppercase().replace(' ', "");
    (route_norm.contains("I-40")
        || route_norm.contains("I40")
        || route_norm.contains("I-75")
        || route_norm.contains("I75")
        || text_norm.contains("I-40")
        || text_norm.contains("I40")
        || text_norm.contains("I-75")
        || text_norm.contains("I75"))
        && ["CLOSURE", "CLOSED", "CRASH", "INCIDENT", "CONSTRUCTION"]
            .iter()
            .any(|needle| text.to_ascii_uppercase().contains(needle))
}

pub(crate) fn mdot_midrive_is_t1_relevant(text: &str) -> bool {
    let text_norm = text.to_ascii_uppercase().replace(' ', "");
    (text_norm.contains("I-75")
        || text_norm.contains("I75")
        || text_norm.contains("I-94")
        || text_norm.contains("I94")
        || text_norm.contains("I-96")
        || text_norm.contains("I96")
        || text_norm.contains("I-275")
        || text_norm.contains("I275")
        || text_norm.contains("I-696")
        || text_norm.contains("I696"))
        && ["CLOSURE", "CLOSED", "CRASH", "INCIDENT", "CONSTRUCTION"]
            .iter()
            .any(|needle| text.to_ascii_uppercase().contains(needle))
}

pub(crate) fn indot_trafficwise_is_t1_relevant(text: &str) -> bool {
    let text_norm = text.to_ascii_uppercase().replace(' ', "");
    (text_norm.contains("I-80")
        || text_norm.contains("I80")
        || text_norm.contains("I-90")
        || text_norm.contains("I90")
        || text_norm.contains("I-94")
        || text_norm.contains("I94")
        || text_norm.contains("TOLLROAD"))
        && [
            "CLOSURE",
            "CLOSED",
            "CRASH",
            "INCIDENT",
            "CONSTRUCTION",
            "ROADWORK",
            "LANE CLOSED",
        ]
        .iter()
        .any(|needle| text.to_ascii_uppercase().contains(needle))
}

pub(crate) fn tdot_smartway_event_type(text: &str) -> &'static str {
    let text = text.to_ascii_lowercase();
    if text.contains("construction") || text.contains("maintenance") {
        "work_zone"
    } else if text.contains("crash") {
        "crash"
    } else if text.contains("closure") || text.contains("closed") {
        "closure"
    } else {
        "incident"
    }
}

pub(crate) fn indot_trafficwise_event_type(text: &str) -> &'static str {
    let text = text.to_ascii_lowercase();
    if text.contains("roadwork") || text.contains("construction") || text.contains("maintenance") {
        "work_zone"
    } else if text.contains("crash") {
        "crash"
    } else if text.contains("closure") || text.contains("closed") {
        "closure"
    } else {
        "incident"
    }
}

pub(crate) fn mdot_midrive_event_type(text: &str) -> &'static str {
    let text = text.to_ascii_lowercase();
    if text.contains("construction") || text.contains("maintenance") {
        "work_zone"
    } else if text.contains("crash") {
        "crash"
    } else if text.contains("closure") || text.contains("closed") {
        "closure"
    } else {
        "incident"
    }
}

pub(crate) fn iowa511_event_type(text: &str) -> &'static str {
    let text = text.to_ascii_lowercase();
    if text.contains("construction") {
        "work_zone"
    } else if text.contains("crash") {
        "crash"
    } else if text.contains("closed") || text.contains("closure") {
        "closure"
    } else {
        "incident"
    }
}

pub(crate) fn iowa511_full_closure(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    if text.contains("shoulder") || text.contains("lane closed") || text.contains("lanes closed") {
        return false;
    }
    text.contains("road closed")
        || text.contains("ramp closed")
        || text.contains("entrance ramp closed")
        || text.contains(": closed")
}

pub(crate) fn indot_trafficwise_full_closure(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    text.contains("road closed")
        || text.contains("ramp closed")
        || text.contains("entrance ramp closed")
        || text.contains("exit ramp closed")
        || text.contains("freeway closed")
}

pub(crate) fn mdot_midrive_full_closure(text: &str) -> bool {
    let text = text.to_ascii_lowercase();
    text.contains("all lanes")
        || text.contains("road closed")
        || text.contains("freeway closed")
        || text.contains("ramp closed")
}

pub(crate) fn mdot_midrive_lanes_closed(text: &str) -> Option<u8> {
    let text = text.to_ascii_lowercase();
    if text.contains("center lane") && (text.contains("left lane") || text.contains("right lane")) {
        Some(2)
    } else if text.contains("left lane") && text.contains("right lane") {
        Some(2)
    } else if text.contains("two lanes") || text.contains("2 lanes") {
        Some(2)
    } else if text.contains("three lanes") || text.contains("3 lanes") {
        Some(3)
    } else if text.contains("left lane") || text.contains("right lane") || text.contains("1 lane") {
        Some(1)
    } else if text.contains("left shoulder") || text.contains("right shoulder") {
        Some(0)
    } else {
        None
    }
}

pub(crate) fn epoch_millis_year(millis: i64) -> Option<u16> {
    epoch_millis_ymd(millis).and_then(|(year, _, _)| u16::try_from(year).ok())
}

pub(crate) fn epoch_millis_date(millis: i64) -> Option<String> {
    epoch_millis_ymd(millis).map(|(year, month, day)| format!("{year:04}-{month:02}-{day:02}"))
}

pub(crate) fn epoch_millis_ymd(millis: i64) -> Option<(i32, u32, u32)> {
    if millis < 0 {
        return None;
    }
    let days = millis.div_euclid(86_400_000);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096).div_euclid(365);
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2).div_euclid(153);
    let day = doy - (153 * mp + 2).div_euclid(5) + 1;
    let month = mp + if mp < 10 { 3 } else { -9 };
    let year = y + if month <= 2 { 1 } else { 0 };
    Some((year as i32, month as u32, day as u32))
}

pub(crate) fn current_utc_year() -> u16 {
    let millis = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis().min(i64::MAX as u128) as i64)
        .unwrap_or(0);
    epoch_millis_year(millis).unwrap_or(1970)
}

pub(crate) fn compact_note(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub(crate) fn strip_html_tags(input: &str) -> String {
    let mut output = String::new();
    let mut in_tag = false;
    for ch in input.chars() {
        match ch {
            '<' => {
                in_tag = true;
                output.push(' ');
            }
            '>' => {
                in_tag = false;
                output.push(' ');
            }
            _ if !in_tag => output.push(ch),
            _ => {}
        }
    }
    output
        .replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&#39;", "'")
        .replace("&quot;", "\"")
}

pub(crate) fn extract_after_label(text: &str, label: &str) -> Option<String> {
    let (_, tail) = text.split_once(label)?;
    let value = tail.split('|').next().unwrap_or(tail).trim().to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

pub(crate) fn combine_iowa_date_time(issue_date: &str, time: &str) -> String {
    if issue_date.len() != 8 || time.trim().is_empty() {
        return time.to_string();
    }
    format!(
        "{}-{}-{} {}",
        &issue_date[0..4],
        &issue_date[4..6],
        &issue_date[6..8],
        time.trim()
    )
}

pub(crate) fn same_day_duration_hours(start: &str, end: &str) -> Option<f64> {
    let start = parse_12h_minutes(start)?;
    let end = parse_12h_minutes(end)?;
    if end >= start {
        Some((end - start) as f64 / 60.0)
    } else {
        None
    }
}

pub(crate) fn parse_12h_minutes(input: &str) -> Option<i32> {
    let input = input.trim();
    let (time, suffix) = input.rsplit_once(' ')?;
    let (hour, minute) = time.split_once(':')?;
    let mut hour = hour.parse::<i32>().ok()?;
    let minute = minute.parse::<i32>().ok()?;
    if !(1..=12).contains(&hour) || !(0..=59).contains(&minute) {
        return None;
    }
    let suffix = suffix.to_ascii_uppercase();
    if suffix == "PM" && hour != 12 {
        hour += 12;
    } else if suffix == "AM" && hour == 12 {
        hour = 0;
    } else if suffix != "AM" && suffix != "PM" {
        return None;
    }
    Some(hour * 60 + minute)
}

// `summarize_t1_failure_events` moved to support::misc

pub(crate) fn annual_probability_from_rate(rate: f64) -> f64 {
    if rate <= 0.0 {
        0.0
    } else {
        1.0 - (-rate).exp()
    }
}

pub(crate) fn event_summary_confidence(rows: &[&T1FailureEventRow]) -> String {
    if rows.is_empty() {
        return "unknown".to_string();
    }
    if rows
        .iter()
        .all(|row| row.confidence.eq_ignore_ascii_case("high"))
    {
        "high".to_string()
    } else if rows
        .iter()
        .any(|row| row.confidence.eq_ignore_ascii_case("low"))
    {
        "low".to_string()
    } else {
        "medium".to_string()
    }
}

pub(crate) fn percentile_nearest(sorted_values: &[f64], p: f64) -> Option<f64> {
    if sorted_values.is_empty() {
        return None;
    }
    let p = p.clamp(0.0, 1.0);
    let idx = ((sorted_values.len() - 1) as f64 * p).round() as usize;
    sorted_values.get(idx).copied()
}

pub(crate) fn apply_t1_failure_events_to_ledger(
    ledger_rows: &[T1FailureRow],
    event_rows: &[T1FailureEventRow],
    event_artifact: &Path,
) -> Vec<T1FailureRow> {
    let summaries = summarize_t1_failure_events(event_rows)
        .into_iter()
        .map(|summary| (summary.site_id.clone(), summary))
        .collect::<std::collections::BTreeMap<_, _>>();

    ledger_rows
        .iter()
        .cloned()
        .map(|mut row| {
            if let Some(summary) = summaries.get(&row.site_id) {
                row.annual_probability = Some(summary.annual_probability);
                row.duration_p50_hours = summary.duration_p50_hours;
                row.duration_p95_hours = summary.duration_p95_hours;
                row.source_status = "empirical".to_string();
                row.confidence = summary.confidence.clone();
                row.current_artifact = append_artifact(&row.current_artifact, event_artifact);
                row.blocking_gap = "Snapshot empirical event observations loaded, but annual closure probability is not stable until a polling/archive window is built; reroute time and throughput retention still require source validation".to_string();
                row.next_evidence_step = "Join event windows to NPMRDS/FPM travel-time traces and reroute simulations; continue polling or obtain DOT history before publication".to_string();
            }
            row
        })
        .collect()
}

pub(crate) fn append_artifact(existing: &str, artifact: &Path) -> String {
    let artifact = artifact.to_string_lossy();
    if existing
        .split(';')
        .map(str::trim)
        .any(|value| value == artifact)
    {
        existing.to_string()
    } else if existing.trim().is_empty() {
        artifact.to_string()
    } else {
        format!("{}; {}", existing.trim(), artifact)
    }
}

// `print_t1_failure_event_summary` moved to support::print

pub(crate) fn t1_failure_event_observation_gate_failures(rows: &[T1FailureEventRow]) -> Vec<String> {
    if rows.is_empty() {
        return vec!["event ledger has no observation rows".to_string()];
    }

    let mut failures = Vec::new();
    for row in rows {
        let label = if row.event_id.trim().is_empty() {
            format!("{}:<missing-event-id>", row.site_id)
        } else {
            format!("{}:{}", row.site_id, row.event_id)
        };

        if !t1_failure_event_has_observation_contract(row) {
            failures.push(format!(
                "{label} missing site/event/source/year/type/confidence/timing contract"
            ));
        }
    }

    if !rows.iter().any(|row| row.freight_relevant) {
        failures.push("event ledger has no freight-relevant observations".to_string());
    }

    failures
}

pub(crate) fn t1_failure_event_has_observation_contract(row: &T1FailureEventRow) -> bool {
    let confidence = row.confidence.trim().to_ascii_lowercase();
    let confidence_is_labeled = matches!(confidence.as_str(), "high" | "medium" | "low");
    let has_timing = row.duration_hours.is_some()
        || (!row.start_time.trim().is_empty() && !row.end_time.trim().is_empty());

    !row.site_id.trim().is_empty()
        && !row.event_id.trim().is_empty()
        && !row.source.trim().is_empty()
        && !row.source_event_id.trim().is_empty()
        && row.observation_year >= 2000
        && !row.event_type.trim().is_empty()
        && confidence_is_labeled
        && has_timing
}

pub(crate) fn join_set(values: &std::collections::BTreeSet<&str>) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.iter().copied().collect::<Vec<_>>().join(", ")
    }
}

#[cfg(test)]
mod tests {
    include!("tests_inline.rs");
}

/// Normalise user input to internal route ID: "I-80" → "I80", "i80" → "I80"
pub(crate) fn normalise_designation(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_uppercase()
}

pub(crate) fn census_api_key() -> Result<String> {
    validate_census_api_key(std::env::var("CENSUS_API_KEY").ok())
}

pub(crate) fn validate_census_api_key(value: Option<String>) -> Result<String> {
    let key = value.context("CENSUS_API_KEY is required for Census ACS requests")?;
    let key = key.trim();
    if key.is_empty() {
        anyhow::bail!("CENSUS_API_KEY is empty");
    }
    Ok(key.to_string())
}

/// Ensure the TIGER shapefile is extracted; return path to .shp file.
pub(crate) fn ensure_shapefile(manifest: &route_data::Manifest) -> Result<std::path::PathBuf> {
    let extract_dir = manifest.cache_dir.join("tiger-primary-roads");
    let shp_path = extract_dir.join("tl_2023_us_primaryroads.shp");
    if shp_path.exists() {
        return Ok(shp_path);
    }

    let zip_path = manifest.cache_path("tiger-primary-roads");
    if !zip_path.exists() {
        anyhow::bail!("TIGER primary roads not cached — run `route fetch` first.");
    }
    println!("  extracting shapefile…");
    route_data::fetch::extract_shp(&zip_path, &extract_dir)
}

// `ensure_reviewed_report_sources` moved to support::misc

#[derive(serde::Deserialize)]
struct I80SourcePolicyRow {
    source_id: String,
    acquisition_status: String,
}

pub(crate) fn load_excluded_i80_sources(path: &std::path::Path) -> Result<std::collections::BTreeSet<String>> {
    let mut reader = csv::Reader::from_path(path)
        .with_context(|| format!("reading I-80 source contract {}", path.display()))?;
    let mut excluded = std::collections::BTreeSet::new();
    for row in reader.deserialize::<I80SourcePolicyRow>() {
        let row = row.with_context(|| format!("parsing {}", path.display()))?;
        if row.acquisition_status.ends_with("excluded") {
            excluded.insert(row.source_id);
        }
    }
    Ok(excluded)
}

/// Load the HighwayGraph from cached TIGER + optional HPMS.
pub(crate) fn load_graph(manifest: &route_data::Manifest) -> Result<route_network::HighwayGraph> {
    let shp_path = ensure_shapefile(manifest)?;
    // Always load all road classes — US highways needed for upgrade-candidate scoring
    let segments = route_data::nhs::read_nhs_shapefile(&shp_path, true)
        .map_err(|e| anyhow::anyhow!("shapefile error: {e}"))?;

    // Auto-load HPMS if cached
    let hpms_path = manifest.cache_dir.join("hpms_2018.csv");
    let hpms = if hpms_path.exists() {
        route_data::hpms::read_hpms_csv(&hpms_path).unwrap_or_default()
    } else {
        Vec::new()
    };

    let fpm = load_cached_fpm(manifest);
    let (graph, _) = route_network::build_graph_with_fpm(segments, &hpms, &fpm);
    Ok(graph)
}

pub(crate) fn load_cached_fpm(manifest: &route_data::Manifest) -> Vec<route_data::HpmsFpmRecord> {
    [
        "hpms_fpm.csv",
        "fpm_2023.csv",
        "freight_performance_measures.csv",
    ]
    .iter()
    .map(|name| manifest.cache_dir.join(name))
    .find(|path| path.exists())
    .and_then(|path| route_data::hpms::read_hpms_fpm_csv(&path).ok())
    .unwrap_or_default()
}

/// Load county gazetteer + ACS population from cache (if available).
/// Returns None silently if the files are not cached — scoring degrades gracefully.
pub(crate) fn load_acs_counties_for_scoring(
    manifest: &route_data::Manifest,
) -> Option<Vec<route_data::CountyCentroid>> {
    // Locate gazetteer
    let gaz_path: Option<std::path::PathBuf> = std::fs::read_dir(&manifest.cache_dir)
        .ok()
        .and_then(|entries| {
            entries
                .filter_map(|e| e.ok())
                .find(|e| {
                    e.file_name()
                        .to_string_lossy()
                        .ends_with("counties_national.txt")
                })
                .map(|e| e.path())
        });

    let gaz_path = gaz_path?;
    let mut counties = route_data::read_county_gazetteer(&gaz_path).ok()?;

    // Join ACS population if cached
    let pop_path = manifest.cache_dir.join("acs_county_pop_2022.csv");
    if pop_path.exists() {
        let _ = route_data::join_population(&mut counties, &pop_path);
    }

    // Join ACS median household income if cached (for C3 scoring)
    let inc_path = manifest.cache_dir.join("acs_county_income_2022.csv");
    if inc_path.exists() {
        let _ = route_data::join_income(&mut counties, &inc_path);
    }

    // Join RUCC rural codes if cached (for C2 rural_share scoring)
    let rucc_path = manifest.cache_dir.join("rucc_2023.csv");
    if rucc_path.exists() {
        let _ = route_data::join_rucc(&mut counties, &rucc_path);
    }

    Some(counties)
}

/// Load ports.csv (top 25 ports + major border crossings) for B3 scoring.
pub(crate) fn load_ports() -> Vec<PortLocation> {
    let path = std::path::Path::new("data/ports.csv");
    if !path.exists() {
        return Vec::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return Vec::new();
    };
    rdr.records()
        .filter_map(|r| r.ok())
        .filter_map(|rec| {
            if rec.len() < 5 {
                return None;
            }
            let lat: f64 = rec[1].parse().ok()?;
            let lon: f64 = rec[2].parse().ok()?;
            let rank: u32 = rec[3].parse().ok()?;
            let is_border = rec[4].contains("border");
            Some(PortLocation {
                lat,
                lon,
                _rank: rank,
                is_border,
            })
        })
        .collect()
}

struct PortLocation {
    lat: f64,
    lon: f64,
    _rank: u32,
    is_border: bool,
}

/// Load intermodal terminal locations from data/intermodal_terminals.csv.
pub(crate) fn load_intermodal_terminals() -> Vec<(f64, f64)> {
    let path = std::path::Path::new("data/intermodal_terminals.csv");
    if !path.exists() {
        return Vec::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return Vec::new();
    };
    rdr.records()
        .filter_map(|r| r.ok())
        .filter_map(|rec| {
            if rec.len() < 5 {
                return None;
            }
            let lat: f64 = rec[3].parse().ok()?;
            let lon: f64 = rec[4].parse().ok()?;
            Some((lat, lon))
        })
        .collect()
}

/// Compute intermodal hub count for a corridor (hubs within 30 miles).
// `join_intermodal_to_corridor` moved to support::misc

/// Load DCFC charging station locations from cache.
pub(crate) fn load_dcfc_stations() -> Vec<(f64, f64)> {
    // (lat, lon)
    let path = std::path::Path::new("data/cache/dcfc_stations.csv");
    if !path.exists() {
        return Vec::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return Vec::new();
    };
    rdr.records()
        .filter_map(|r| r.ok())
        .filter_map(|rec| {
            if rec.len() < 7 {
                return None;
            }
            let lat: f64 = rec[4].parse().ok()?;
            let lon: f64 = rec[5].parse().ok()?;
            if lat.abs() < 1.0 || lon.abs() < 1.0 {
                return None;
            }
            Some((lat, lon))
        })
        .collect()
}

/// Compute DCFC per 100 miles for a corridor.
// `join_dcfc_to_corridor` moved to support::misc

/// Compute B3 fields: port terminus flag, border crossing flag, nearest port distance.
// `join_port_access_to_corridor` moved to support::misc

/// A 1°×1° FEMA NFHL tile with an SFHA feature count.
struct FemaTile {
    name: String,
    xmin: f64,
    ymin: f64,
    xmax: f64,
    ymax: f64,
    sfha_count: u32,
    status: String,
}

/// Load FEMA SFHA tile counts from data/cache/fema_sfha_tile_counts.csv.
/// Returns an empty Vec if the file is not present or cannot be parsed.
pub(crate) fn load_fema_tiles() -> Vec<FemaTile> {
    let path = std::path::Path::new("data/cache/fema_sfha_tile_counts.csv");
    if !path.exists() {
        return Vec::new();
    }
    let Ok(file) = std::fs::File::open(path) else {
        return Vec::new();
    };
    parse_fema_tiles(file)
}

pub(crate) fn parse_fema_tiles(reader: impl std::io::Read) -> Vec<FemaTile> {
    let mut rdr = csv::Reader::from_reader(reader);
    rdr.records()
        .filter_map(|r| r.ok())
        .filter_map(|rec| {
            if rec.len() < 7 {
                return None;
            }
            let xmin: f64 = rec[1].trim().parse().ok()?;
            let ymin: f64 = rec[2].trim().parse().ok()?;
            let xmax: f64 = rec[3].trim().parse().ok()?;
            let ymax: f64 = rec[4].trim().parse().ok()?;
            let sfha_count: u32 = rec[5].trim().parse().ok()?;
            let status = rec[6].trim().to_string();
            if status != "ok" {
                return None;
            }
            Some(FemaTile {
                name: rec[0].trim().to_string(),
                xmin,
                ymin,
                xmax,
                ymax,
                sfha_count,
                status,
            })
        })
        .collect()
}

/// Join FEMA D1 SFHA data onto a corridor's CorridorAttributes.
///
/// Algorithm:
/// 1. Collect edge geometry bounding boxes for the corridor.
/// 2. Sum each SFHA tile whose bbox overlaps at least one route edge bbox.
/// 3. Estimate fema_sfha_miles = sum × 0.3 (avg SFHA polygon ~0.3 mi span).
/// 4. Set max_consecutive_sfha_miles as a 70% proxy (coastal/valley assumption).
// `join_fema_d1_to_corridor` moved to support::misc

/// Apply a D3 IRI proxy when NBI bridge data is unavailable.
///
/// Maps mean_iri to an estimated mean_year_built and pct_bridges_poor:
///   IRI < 50  → post-2000 construction/resurfacing  → year 2005
///   IRI 50-80 → 1985–2000 era                       → year 1990
///   IRI 80-120→ 1970–1985 era                       → year 1975
///   IRI > 120 → pre-1970 Eisenhower era              → year 1965
///
/// pct_bridges_poor proxy = (IRI / 170.0).min(0.30)
/// (IRI 170 ≈ "poor" pavement threshold; maps 0–170 IRI → 0–30% poor bridges)

// NBI data record for joining
struct NbiBridgeRecord {
    pct_bridges_poor: f32,
    mean_year_built: f32,
    bridge_count: u32,
}

/// Load NBI per-corridor summary from data/cache/nbi_bridges.csv.
pub(crate) fn load_nbi_bridges() -> std::collections::HashMap<String, NbiBridgeRecord> {
    let path = std::path::Path::new("data/cache/nbi_bridges.csv");
    if !path.exists() {
        return std::collections::HashMap::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return std::collections::HashMap::new();
    };
    let mut totals: std::collections::HashMap<String, (u32, f32, f32)> =
        std::collections::HashMap::new();
    for result in rdr.records().filter_map(|r| r.ok()) {
        if result.len() < 5 {
            continue;
        }
        let route_id = normalise_designation(result[0].trim());
        if route_id.is_empty() {
            continue;
        }
        let bridge_count: u32 = result[1].parse().unwrap_or(0);
        let pct: f32 = result[3].parse().unwrap_or(0.0);
        let year: f32 = result[4].parse().unwrap_or(1970.0);
        let poor_count = pct * bridge_count as f32;
        let year_sum = year * bridge_count as f32;
        let entry = totals.entry(route_id).or_insert((0, 0.0, 0.0));
        entry.0 += bridge_count;
        entry.1 += poor_count;
        entry.2 += year_sum;
    }
    let mut map = std::collections::HashMap::new();
    for (route_id, (bridge_count, poor_count, year_sum)) in totals {
        let denom = bridge_count.max(1) as f32;
        map.insert(
            route_id,
            NbiBridgeRecord {
                pct_bridges_poor: poor_count / denom,
                mean_year_built: year_sum / denom,
                bridge_count,
            },
        );
    }
    map
}

/// Load FARS 2022 fatal crash rates by route from data/cache/fars_2022_routes.csv.
/// Columns: route_id, fatal_count, fatal_rate_per_100mvmt
/// Returns route_id -> crash_rate_per_100M_VMT.
pub(crate) fn load_fars_safety() -> std::collections::HashMap<String, f32> {
    let path = std::path::Path::new("data/cache/fars_2022_routes.csv");
    if !path.exists() {
        return std::collections::HashMap::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return std::collections::HashMap::new();
    };
    let mut map = std::collections::HashMap::new();
    for result in rdr.records().filter_map(|r| r.ok()) {
        if result.len() < 3 {
            continue;
        }
        let route_id = result[0].to_string();
        let rate: f32 = result[2].parse().unwrap_or(0.0);
        map.insert(route_id, rate);
    }
    map
}

/// Load railroad parallel data from data/railroad_parallels.csv.
/// Columns: interstate, railroad, railroad_owner, approx_parallel_miles, within_50mi, notes
/// Returns: route_id (normalized e.g. "I80") -> railroad_name (only within_50mi=true entries).
pub(crate) fn load_railroad_parallels() -> std::collections::HashMap<String, String> {
    let path = std::path::Path::new("data/railroad_parallels.csv");
    if !path.exists() {
        return std::collections::HashMap::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return std::collections::HashMap::new();
    };
    let mut map = std::collections::HashMap::new();
    for result in rdr.records().filter_map(|r| r.ok()) {
        if result.len() < 5 {
            continue;
        }
        // Columns: interstate, railroad, railroad_owner, approx_parallel_miles, within_50mi, notes
        let interstate = result[0].trim().to_string();
        let railroad = result[1].trim().to_string();
        let within_50mi = result[4].trim() == "true";
        if within_50mi {
            // Normalize interstate name: "I-80" -> "I80"
            let id: String = interstate
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_uppercase();
            map.insert(id, railroad);
        }
    }
    map
}

struct HazardZone {
    wildfire: f32,
    tornado: f32,
    seismic: f32,
}

/// Load multi-hazard zone scores from data/hazard_zones.csv.
/// Columns: route, wildfire_risk, tornado_risk, seismic_risk
/// Route names like "I-5 (CA Siskiyou)" are normalized to "I5"; MAX taken for multi-segment corridors.
// `load_hazard_zones` moved to support::misc

/// Join NBI bridge condition data to a corridor.
pub(crate) fn join_nbi_to_corridor(
    route_id: &str,
    attrs: &mut route_network::CorridorAttributes,
    nbi: &std::collections::HashMap<String, NbiBridgeRecord>,
) {
    if let Some(rec) = nbi.get(route_id) {
        attrs.pct_bridges_poor = Some(rec.pct_bridges_poor);
        attrs.mean_year_built = Some(rec.mean_year_built);
        attrs.bridge_count = rec.bridge_count as usize;
    }
}

/// Estimate A2 freight value from representative HPMS daily truck crossings.
/// Uses p90 AADT when available, then mean AADT as the secondary A2 path.
pub(crate) fn join_a2_freight_proxy(attrs: &mut route_network::CorridorAttributes, _corridor_miles: f64) {
    if attrs.annual_freight_value_b.is_some() {
        return;
    }
    let Some(aadt) = attrs.p90_aadt.or(attrs.mean_aadt) else {
        return;
    };
    let truck_pct = attrs.mean_pct_truck.unwrap_or(0.084) as f64;
    let truck_aadt = aadt * truck_pct;
    let freight_b = truck_aadt * 365.0 * 16.0 * 1_000.0 / 1_000_000_000.0;
    attrs.annual_freight_value_b = Some(freight_b);
    attrs.freight_value_is_hpms_proxy = true;
}

///
/// Only fills in fields that are currently None.
pub(crate) fn join_d3_iri_proxy(attrs: &mut route_network::CorridorAttributes) {
    // Only apply when NBI data is absent
    if attrs.pct_bridges_poor.is_some() {
        return;
    }
    let Some(iri) = attrs.mean_iri else {
        return;
    };

    let estimated_year = if iri < 50.0 {
        2005.0_f32
    } else if iri < 80.0 {
        1990.0
    } else if iri < 120.0 {
        1975.0
    } else {
        1965.0
    };

    if attrs.mean_year_built.is_none() {
        attrs.mean_year_built = Some(estimated_year);
    }
    let iri_proxy = (iri / 170.0).min(0.30);
    attrs.pct_bridges_poor = Some(iri_proxy);
}

/// Join ACS population onto a single corridor's CorridorAttributes.
/// No-op if the cached files are not present.
// `join_acs_population_to_corridor` moved to support::misc

/// Print a formatted score table to stdout.
// `print_score_table` moved to support::print

/// Build a simple demand matrix from HPMS AADT data in the graph.
/// Proxy for FAF5-based O-D demand until FAF5 routing is implemented.
pub(crate) fn build_demand_from_graph(g: &route_network::HighwayGraph) -> route_sim::demand::DemandMatrix {
    use route_sim::demand::{demand_from_aadt, DemandParams};
    let params = DemandParams::default();
    let mut demand = Vec::new();

    // Create O-D pairs from terminus nodes of each interstate
    for route_id in g.interstate_ids() {
        let edges = g.route_edges(&route_id);
        if edges.len() < 2 {
            continue;
        }

        // Use first and last edge endpoints as a crude O-D pair
        if let (Some(&first_ei), Some(&last_ei)) = (edges.first(), edges.last()) {
            if let (Some((s, _)), Some((_, t))) = (
                g.graph.edge_endpoints(first_ei),
                g.graph.edge_endpoints(last_ei),
            ) {
                let mean_aadt = edges
                    .iter()
                    .filter_map(|&ei| g.graph[ei].aadt.map(|a| a as f64))
                    .sum::<f64>()
                    / edges.len() as f64;
                let mean_pct = edges
                    .iter()
                    .filter_map(|&ei| g.graph[ei].pct_truck)
                    .sum::<f32>()
                    / edges.len() as f32;

                if mean_aadt > 0.0 {
                    demand.push(demand_from_aadt(mean_aadt, mean_pct, &params, s, t));
                }
            }
        }
    }
    demand
}

pub(crate) fn game_engine_facts(scenario_id: &str, manifest_path: &Path) -> Result<Option<game::EngineFacts>> {
    if scenario_id != game::DES_MOINES_SCENARIO_ID {
        return Ok(None);
    }

    let manifest = route_data::Manifest::load(manifest_path)
        .with_context(|| format!("loading manifest from {}", manifest_path.display()))?;
    let graph = load_graph(&manifest)?;
    let demand = build_demand_from_graph(&graph);

    let toml_str = route_sim::scenarios::load_scenario("des-moines-interchange")
        .ok_or_else(|| anyhow::anyhow!("missing embedded des-moines-interchange scenario"))?;
    let scenario: route_sim::Scenario =
        toml::from_str(toml_str).context("parsing des-moines-interchange scenario")?;
    let result = route_sim::run_scenario(&graph, &demand, &scenario);
    let intersection = route_network::find_intersection(&graph, "I35xI80")
        .ok_or_else(|| anyhow::anyhow!("missing I35xI80 diamond anchor"))?;
    let diamond = route_network::analyze_diamond(&graph, intersection);

    Ok(Some(game::EngineFacts {
        baseline_throughput_vph: result
            .baseline
            .metrics
            .total_throughput_vph
            .round()
            .max(0.0) as u32,
        incident_throughput_vph: result
            .incident
            .metrics
            .total_throughput_vph
            .round()
            .max(0.0) as u32,
        intervention_throughput_vph: result
            .intervention
            .as_ref()
            .map(|run| run.metrics.total_throughput_vph.round().max(0.0) as u32)
            .unwrap_or(0),
        recovery_hours: result.incident.t90_hours.unwrap_or(0.0),
        diamond_k_current: diamond.k_current.min(u8::MAX as usize) as u8,
        connectors_needed: diamond.connectors_needed.min(u8::MAX as usize) as u8,
        evidence_level: "Heuristic live ROUTE summary",
    }))
}

// `print_scenario_result` moved to support::print

pub(crate) fn print_chaos_result(result: &route_sim::ChaosResult) {
    println!("\n=== Chaos Results ({} iterations) ===", result.iterations);
    println!(
        "  Mean freight cost delta: +${:.2}M/peak-hr",
        result.mean_freight_cost_delta_m
    );
    println!(
        "  P95 freight cost delta:  +${:.2}M/peak-hr",
        result.p95_freight_cost_delta_m
    );
    println!(
        "  Max freight cost delta:  +${:.2}M/peak-hr",
        result.max_freight_cost_delta_m
    );
    println!("  Mean network PTI:        {:.2}", result.mean_network_pti);
    println!(
        "  Saturation fraction:     {:.1}%",
        result.saturation_fraction * 100.0
    );
    if !result.worst_case_corridors.is_empty() {
        println!(
            "  Worst-case corridors:    {}",
            result.worst_case_corridors.join(", ")
        );
    }
}
