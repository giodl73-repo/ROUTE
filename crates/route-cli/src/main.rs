use anyhow::{Context, Result};
use clap::Parser;
use std::path::{Path, PathBuf};

mod cli;
mod commands;
mod game;
mod support;
mod types;
pub(crate) use support::*;
pub(crate) use types::*;

use cli::{
    Cli, Commands, GameCommand, GapType, InterventionCorridorArg, OdCorridorCmd, SimMode,
    TierRegionGraphArg,
};

const T1_THRESHOLD: f64 = route_network::T1_SCORE_THRESHOLD;
const T2_THRESHOLD: f64 = route_network::T2_SCORE_THRESHOLD;
const T3_THRESHOLD: f64 = route_network::T3_SCORE_THRESHOLD;
const DIMENSION_CODES: [&str; 16] = [
    "A1", "A2", "A3", "A4", "A5", "B1", "B2", "B3", "B4", "C1", "C2", "C3", "C4", "D1", "D2", "D3",
];

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

const PAVEMENT_EVIDENCE_COST_PER_MEMBER_M: f64 = 0.05;
const PAVEMENT_REPAIR_COST_PER_MEMBER_M: f64 = 2.50;

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

#[cfg(test)]
mod tests {
    include!("tests_inline.rs");
}
