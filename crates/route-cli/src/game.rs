use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub const DES_MOINES_SCENARIO_ID: &str = "des-moines-diamond";
pub const DONNER_SCENARIO_ID: &str = "donner-weather-closure";

#[derive(Clone, Copy, Debug)]
pub struct Track {
    pub name: &'static str,
    pub start: i32,
    pub failure_condition: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct ProjectCard {
    pub slug: &'static str,
    pub name: &'static str,
    pub cost: i32,
    pub crew: i32,
    pub time: i32,
    pub effect: &'static str,
    pub evidence: &'static str,
    pub protects: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct EventCard {
    pub slug: &'static str,
    pub name: &'static str,
    pub trigger: &'static str,
    pub effect: &'static str,
    pub warning: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct EvidenceCard {
    pub name: &'static str,
    pub known: &'static str,
    pub missing: &'static str,
    pub effect: &'static str,
}

#[derive(Clone, Copy, Debug)]
pub struct Scenario {
    pub id: &'static str,
    pub name: &'static str,
    pub phase: &'static str,
    pub version: &'static str,
    pub evidence_level: &'static str,
    pub standards: &'static str,
    pub hook: &'static str,
    pub one_aha: &'static str,
    pub time_limit: &'static str,
    pub publication_gate: &'static str,
    pub engine_hook: &'static str,
    pub panel_status: &'static str,
    pub tracks: &'static [Track],
    pub projects: &'static [ProjectCard],
    pub events: &'static [EventCard],
    pub evidence: &'static [EvidenceCard],
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActiveProject {
    pub slug: String,
    pub remaining_seasons: u8,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GameState {
    pub scenario_id: String,
    pub season: u8,
    pub budget: i32,
    pub construction_crews: i32,
    pub political_capital: i32,
    pub public_patience: i32,
    pub operations_capacity: i32,
    pub evidence_confidence: i32,
    #[serde(default)]
    pub weather_readiness: i32,
    pub active_projects: Vec<ActiveProject>,
    pub completed_projects: Vec<String>,
    pub first_closure_seen: bool,
    pub connector_package_complete: bool,
    pub source_requested: bool,
    pub validated_evidence_available: bool,
    pub fiscal_crisis: bool,
    #[serde(default)]
    pub trapped_queue: bool,
    pub publication_gate: String,
}

#[derive(Clone, Debug)]
pub struct SeasonResult {
    pub state: GameState,
    pub accepted_projects: Vec<String>,
    pub rejected_actions: Vec<String>,
    pub event_result: String,
    pub throughput_retention: f64,
    pub recovery_hours: f64,
    pub sla_status: String,
}

#[derive(Debug, Deserialize)]
struct SessionLogRow {
    season: u8,
    #[serde(rename = "accepted_projects")]
    _accepted_projects: String,
    #[serde(rename = "rejected_count")]
    _rejected_count: usize,
    budget_remaining: i32,
    political_capital: i32,
    public_patience: i32,
    #[serde(rename = "operations_capacity")]
    _operations_capacity: i32,
    #[serde(rename = "evidence_confidence")]
    _evidence_confidence: i32,
    throughput_retention: f64,
    recovery_hours: f64,
    sla_status: String,
    publication_gate: String,
}

#[derive(Debug, Deserialize)]
struct CampaignRow {
    order: u8,
    phase_name: String,
    scenario_id: String,
    scenario_name: String,
    map_id: String,
    tier_focus: String,
    standard_lesson: String,
    evidence_gate: String,
    playable_status: String,
    publication_gate: String,
    next_artifact: String,
}

#[derive(Debug, Deserialize)]
struct T2ServiceOverlayRow {
    service_class: String,
    map_id: String,
    scenario_hook: String,
    incident_lever: String,
    upgrade_lever: String,
    restitch_lever: String,
    release_gate: String,
}

#[derive(Clone, Debug)]
pub struct ScoreResult {
    pub scenario_id: String,
    pub seasons: usize,
    pub final_season: u8,
    pub throughput_points: u8,
    pub recovery_points: u8,
    pub sla_points: u8,
    pub budget_points: u8,
    pub support_points: u8,
    pub evidence_points: u8,
    pub total: u8,
    pub win_band: &'static str,
    pub publication_gate: String,
    pub promotion_readiness: String,
    pub engine_facts: Option<EngineFacts>,
}

#[derive(Clone, Copy, Debug)]
pub struct EngineFacts {
    pub baseline_throughput_vph: u32,
    pub incident_throughput_vph: u32,
    pub intervention_throughput_vph: u32,
    pub recovery_hours: f64,
    pub diamond_k_current: u8,
    pub connectors_needed: u8,
    pub evidence_level: &'static str,
}

const TRACKS: &[Track] = &[
    Track {
        name: "Budget",
        start: 12,
        failure_condition: "Below 0; fiscal crisis caps result at Partial Win",
    },
    Track {
        name: "Construction crews",
        start: 3,
        failure_condition: "No available crew for a required project",
    },
    Track {
        name: "Political capital",
        start: 5,
        failure_condition: "Below 0",
    },
    Track {
        name: "Public patience",
        start: 6,
        failure_condition: "Below 0",
    },
    Track {
        name: "Operations capacity",
        start: 4,
        failure_condition: "Below 0 during an outage",
    },
    Track {
        name: "Evidence confidence",
        start: 2,
        failure_condition: "Publication gate locked below 4",
    },
];

const PROJECTS: &[ProjectCard] = &[
    ProjectCard {
        slug: "diamond-connector-package",
        name: "Diamond connector package",
        cost: 5,
        crew: 2,
        time: 3,
        effect: "Unlocks redundant transfer paths; enables recovery gate attempt",
        evidence: "Heuristic",
        protects: "Single interchange closure",
    },
    ProjectCard {
        slug: "express-freight-flyovers",
        name: "Express freight flyovers",
        cost: 4,
        crew: 2,
        time: 2,
        effect: "Protects freight transfer from local capture",
        evidence: "Planned",
        protects: "Local congestion and connector closure",
    },
    ProjectCard {
        slug: "work-zone-sequencing",
        name: "Work-zone sequencing",
        cost: 1,
        crew: 1,
        time: 1,
        effect: "Reduces patience loss during closures",
        evidence: "Heuristic",
        protects: "Construction backlash",
    },
    ProjectCard {
        slug: "intelligent-routing",
        name: "Intelligent routing",
        cost: 2,
        crew: 1,
        time: 1,
        effect: "Reduces incident delay penalty",
        evidence: "Heuristic",
        protects: "Poor reroute timing",
    },
    ProjectCard {
        slug: "relay-hub-reserve-staffing",
        name: "Relay hub reserve staffing",
        cost: 2,
        crew: 1,
        time: 1,
        effect: "Protects driver swaps during outage",
        evidence: "Heuristic",
        protects: "Missed relay windows",
    },
    ProjectCard {
        slug: "ev-rest-hardening",
        name: "EV/rest hardening",
        cost: 2,
        crew: 1,
        time: 1,
        effect: "Protects queue and dwell-time buffers",
        evidence: "Heuristic",
        protects: "Rest/charging outage",
    },
    ProjectCard {
        slug: "general-purpose-widening",
        name: "General-purpose widening",
        cost: 4,
        crew: 2,
        time: 3,
        effect: "Improves local capacity but does not add independent transfer paths",
        evidence: "Heuristic",
        protects: "Congestion-binding stress",
    },
    ProjectCard {
        slug: "source-request",
        name: "Source request",
        cost: 1,
        crew: 0,
        time: 1,
        effect: "Raises evidence confidence by 1 and names the missing source",
        evidence: "Implemented as artifact plan",
        protects: "Unknown evidence blocker",
    },
    ProjectCard {
        slug: "validated-evidence",
        name: "Validated evidence",
        cost: 2,
        crew: 0,
        time: 1,
        effect: "Requires source request; raises evidence confidence by 2 if a matching observed artifact exists",
        evidence: "Planned",
        protects: "Publication-grade proof",
    },
];

const EVENTS: &[EventCard] = &[
    EventCard {
        slug: "full-interchange-zone-closure",
        name: "Full interchange-zone closure",
        trigger: "Forced tutorial, then rare",
        effect: "Apply closure stress; test transfer retention",
        warning: "Transfer capacity is collapsing at the interchange zone.",
    },
    EventCard {
        slug: "night-work-zone-closure",
        name: "Night work-zone closure",
        trigger: "Common",
        effect: "Lose 1 patience unless sequencing is active",
        warning: "The closure is short, but the warning signs are visible.",
    },
    EventCard {
        slug: "relay-hub-surge",
        name: "Relay hub surge",
        trigger: "Medium",
        effect: "Lose 1 operations unless reserve staffing exists",
        warning: "Pavement is open; operations are binding.",
    },
    EventCard {
        slug: "ev-rest-queue",
        name: "EV/rest queue",
        trigger: "Medium",
        effect: "Lose SLA margin unless EV/rest hardening exists",
        warning: "Rest and charging queues are now part of freight reliability.",
    },
    EventCard {
        slug: "political-lane-mile-pressure",
        name: "Political lane-mile pressure",
        trigger: "Common",
        effect: "Widening costs 1 less; connector package costs 1 political capital more",
        warning: "The easy ribbon-cutting project is not the proof project.",
    },
    EventCard {
        slug: "source-challenge",
        name: "Source challenge",
        trigger: "Medium",
        effect: "Publication gate checks evidence confidence",
        warning: "A reviewer asks what was observed versus modeled.",
    },
];

const EVIDENCE: &[EvidenceCard] = &[
    EvidenceCard {
        name: "Scenario run",
        known: "The closure scenario runs and gives bounded throughput outputs",
        missing: "Geometry validation for the intervention",
        effect: "Unlocks heuristic win scoring",
    },
    EvidenceCard {
        name: "Des Moines diamond analyzer",
        known: "route diamond I35xI80 recognizes the curated Des Moines anchor",
        missing: "Full empirical k-class and alternate-capacity validation",
        effect: "Unlocks analyzer consistency; publication still needs observed evidence",
    },
    EvidenceCard {
        name: "Iowa 511 sample",
        known: "Normalized observations exist for I-35/I-80 work-zone rows",
        missing: "Annual depth and closure-rate confidence",
        effect: "Unlocks low-confidence failure probability",
    },
    EvidenceCard {
        name: "NPMRDS/PTI",
        known: "Source target is identified",
        missing: "Direct extract and validation",
        effect: "Locks publication-grade SLA proof",
    },
    EvidenceCard {
        name: "Standards proof ledger",
        known: "T1/T1 standards have acceptance gates",
        missing: "Empirical top-site validation",
        effect: "Shows why the project matters",
    },
];

const DONNER_TRACKS: &[Track] = &[
    Track {
        name: "Budget",
        start: 13,
        failure_condition: "Below 0",
    },
    Track {
        name: "Construction crews",
        start: 3,
        failure_condition: "No available crew for a required project",
    },
    Track {
        name: "Weather readiness",
        start: 3,
        failure_condition: "Below 0 during a storm",
    },
    Track {
        name: "Public patience",
        start: 5,
        failure_condition: "Below 0",
    },
    Track {
        name: "Operations capacity",
        start: 4,
        failure_condition: "Below 0 during reroute or reopening",
    },
    Track {
        name: "Evidence confidence",
        start: 1,
        failure_condition: "Publication gate locked below 4",
    },
];

const DONNER_PROJECTS: &[ProjectCard] = &[
    ProjectCard {
        slug: "early-egress-spurs",
        name: "Early egress spurs",
        cost: 3,
        crew: 1,
        time: 2,
        effect: "Lets trucks leave before the closure zone instead of queueing at the pass",
        evidence: "Planned",
        protects: "Trapped freight and late reroute",
    },
    ProjectCard {
        slug: "winter-operations-package",
        name: "Winter operations package",
        cost: 2,
        crew: 1,
        time: 1,
        effect: "Raises weather readiness by 2 and reduces reopening delay",
        evidence: "Heuristic",
        protects: "Slow clearance and chain-control shock",
    },
    ProjectCard {
        slug: "lower-elevation-freight-bypass",
        name: "Lower-elevation freight bypass",
        cost: 6,
        crew: 2,
        time: 4,
        effect: "Adds independent road capacity below the storm zone",
        evidence: "Planned",
        protects: "Pass closure with no road alternate",
    },
    ProjectCard {
        slug: "managed-freight-tunnel",
        name: "Managed freight tunnel",
        cost: 5,
        crew: 2,
        time: 3,
        effect: "Protects eligible priority freight after it opens; not an instant first-storm fix",
        evidence: "Planned",
        protects: "High-value SLA misses",
    },
    ProjectCard {
        slug: "rail-intermodal-surge-slots",
        name: "Rail/intermodal surge slots",
        cost: 3,
        crew: 1,
        time: 2,
        effect: "Moves eligible freight before the road queue grows",
        evidence: "Heuristic",
        protects: "Overloaded road detour",
    },
    ProjectCard {
        slug: "dynamic-closure-routing",
        name: "Dynamic closure routing",
        cost: 2,
        crew: 0,
        time: 1,
        effect: "Reduces operations loss when storms are forecast",
        evidence: "Heuristic",
        protects: "Late route decisions",
    },
    ProjectCard {
        slug: "general-snow-storage-shoulders",
        name: "General snow storage / shoulders",
        cost: 2,
        crew: 1,
        time: 1,
        effect: "Helps reopening but does not create alternate capacity",
        evidence: "Heuristic",
        protects: "Recovery delay",
    },
    ProjectCard {
        slug: "source-request",
        name: "Source request",
        cost: 1,
        crew: 0,
        time: 1,
        effect: "Raises evidence confidence by 1 and names the missing weather/closure sources",
        evidence: "Implemented as artifact plan",
        protects: "Unknown evidence blocker",
    },
    ProjectCard {
        slug: "validated-weather-evidence",
        name: "Validated weather evidence",
        cost: 2,
        crew: 0,
        time: 1,
        effect: "Requires source request; raises evidence confidence only when observed closure history exists",
        evidence: "Planned",
        protects: "Publication-grade proof",
    },
];

const DONNER_EVENTS: &[EventCard] = &[
    EventCard {
        slug: "whiteout-closure",
        name: "Whiteout closure",
        trigger: "Forced tutorial, then rare",
        effect: "Close the pass for 48 hours; add a trapped-queue marker unless egress or routing is ready",
        warning: "The pass is closed; the queue is forming before the alternate decision.",
    },
    EventCard {
        slug: "chain-control-slowdown",
        name: "Chain-control slowdown",
        trigger: "Common",
        effect: "Lose 1 weather readiness unless winter operations are active",
        warning: "The route is open, but speed and compliance are binding.",
    },
    EventCard {
        slug: "detour-capacity-pinch",
        name: "Detour capacity pinch",
        trigger: "Common",
        effect: "Lose 1 operations unless a bypass or rail slots are active",
        warning: "The alternate exists on the map, but it is not absorbing T1 freight.",
    },
    EventCard {
        slug: "reopening-surge",
        name: "Reopening surge",
        trigger: "Medium",
        effect: "Lose 1 public patience unless snow storage or routing is active",
        warning: "The closure ended; the queue did not.",
    },
    EventCard {
        slug: "high-value-sla-wave",
        name: "High-value SLA wave",
        trigger: "Medium",
        effect: "Lose SLA margin unless tunnel or intermodal slots are active",
        warning: "Not all freight can wait for the road to reopen.",
    },
    EventCard {
        slug: "evidence-challenge",
        name: "Evidence challenge",
        trigger: "Medium",
        effect: "Publication gate checks evidence confidence and observed-source status",
        warning: "A reviewer asks how many closures, how long, and what the alternate carried.",
    },
];

const DONNER_EVIDENCE: &[EvidenceCard] = &[
    EvidenceCard {
        name: "Donner scenario fixture",
        known: "Bound weather closure scenario exists",
        missing: "Focused I-80 mountain demand and intervention acceptance gate",
        effect: "Unlocks heuristic storm scoring",
    },
    EvidenceCard {
        name: "Weather closure history",
        known: "Source need is named",
        missing: "Observed annual frequency and duration distribution",
        effect: "Locks publication-grade closure probability",
    },
    EvidenceCard {
        name: "Alternate road capacity",
        known: "Standards need is named",
        missing: "Truck-capable capacity, winter reliability, and detour time",
        effect: "Locks reroute throughput claim",
    },
    EvidenceCard {
        name: "NPMRDS/PTI",
        known: "Source target is known",
        missing: "Direct pass and alternate travel-time validation",
        effect: "Locks SLA proof",
    },
    EvidenceCard {
        name: "Rail/intermodal relief",
        known: "Design concept exists",
        missing: "Eligible freight share and surge slot capacity",
        effect: "Keeps intermodal project heuristic",
    },
];

pub const SCENARIOS: &[Scenario] = &[
    Scenario {
        id: DES_MOINES_SCENARIO_ID,
        name: "Des Moines Diamond",
        phase: "G0-B pass; G1-A start",
        version: "G0 v0.2",
        evidence_level: "Heuristic",
        standards: "T1-DIAMOND-K; T1-FLYOVER; T1-RECOVERY",
        hook: "The national freight grid looks healthy until one interchange closure breaks the transfer.",
        one_aha: "Capacity is not topology: widening can help congestion without creating independent transfer paths.",
        time_limit: "10 seasons; tutorial can end early after connector selection/completion, scoring, explanation, and publication status.",
        publication_gate: "locked: empirical closure evidence and direct PTI/NPMRDS validation missing",
        engine_hook: "route sim scenario des-moines-interchange --intervention",
        panel_status: "G0-C held for human blind playtest or owner acceptance of simulated evidence",
        tracks: TRACKS,
        projects: PROJECTS,
        events: EVENTS,
        evidence: EVIDENCE,
    },
    Scenario {
        id: DONNER_SCENARIO_ID,
        name: "Donner Weather Closure",
        phase: "G0-B paper prototype; G1-A seed",
        version: "G0 v0.2",
        evidence_level: "Heuristic",
        standards: "T1-SPURS; T1-CLIMATE; T1-RECOVERY; T1-INTERMODAL",
        hook: "I-80 looks like a line across the mountains until winter turns it into a timed gate.",
        one_aha: "The route around the storm is not free capacity.",
        time_limit: "8 seasons; tutorial can end early after one whiteout, a response, scoring, and publication status.",
        publication_gate: "locked: weather closure and alternate-capacity evidence missing",
        engine_hook: "route sim scenario donner-closure",
        panel_status: "G0-C held for human blind playtest or owner acceptance of simulated evidence",
        tracks: DONNER_TRACKS,
        projects: DONNER_PROJECTS,
        events: DONNER_EVENTS,
        evidence: DONNER_EVIDENCE,
    },
];

pub fn render_scenarios() -> String {
    let mut out = String::from(
        "scenario_id\tphase\tevidence_level\tstandards\tengine_hook\tpublication_gate\n",
    );
    for scenario in SCENARIOS {
        out.push_str(&format!(
            "{}\t{}\t{}\t{}\t{}\t{}\n",
            scenario.id,
            scenario.phase,
            scenario.evidence_level,
            scenario.standards,
            scenario.engine_hook,
            scenario.publication_gate
        ));
    }
    out
}

pub fn render_inspect(scenario_id: &str) -> Result<String> {
    let scenario = scenario_by_id(scenario_id)?;
    let mut out = String::new();

    out.push_str(&format!("Scenario: {}\n", scenario.name));
    out.push_str(&format!("scenario_id: {}\n", scenario.id));
    out.push_str(&format!("version: {}\n", scenario.version));
    out.push_str(&format!("phase: {}\n", scenario.phase));
    out.push_str(&format!("evidence_level: {}\n", scenario.evidence_level));
    out.push_str(&format!("standards: {}\n", scenario.standards));
    out.push_str(&format!("hook: {}\n", scenario.hook));
    out.push_str(&format!("one_aha: {}\n", scenario.one_aha));
    out.push_str(&format!("time_limit: {}\n", scenario.time_limit));
    out.push_str(&format!("panel_status: {}\n\n", scenario.panel_status));

    out.push_str("Tracks\n");
    for track in scenario.tracks {
        out.push_str(&format!(
            "- {}: start {}; failure: {}\n",
            track.name, track.start, track.failure_condition
        ));
    }

    out.push_str("\nProject Cards\n");
    for project in scenario.projects {
        out.push_str(&format!(
            "- {} ({}) cost={} crew={} time={} evidence={}; protects={}; effect={}\n",
            project.name,
            project.slug,
            project.cost,
            project.crew,
            project.time,
            project.evidence,
            project.protects,
            project.effect
        ));
    }

    out.push_str("\nEvent Cards\n");
    for event in scenario.events {
        out.push_str(&format!(
            "- {} ({}) trigger={}; effect={}; warning=\"{}\"\n",
            event.name, event.slug, event.trigger, event.effect, event.warning
        ));
    }

    out.push_str("\nEvidence Cards\n");
    for evidence in scenario.evidence {
        out.push_str(&format!(
            "- {}: known={}; missing={}; effect={}\n",
            evidence.name, evidence.known, evidence.missing, evidence.effect
        ));
    }

    out.push_str("\nWin Bands\n");
    out.push_str("- 80-100: Operational win; publication may still be locked\n");
    out.push_str(
        "- 60-79: Partial win; resilience works but support, budget, or evidence is weak\n",
    );
    out.push_str("- 0-59: Failure; the player did not protect the network under adversity\n");

    out.push_str("\nPublication Gate\n");
    out.push_str(&format!("- {}\n", scenario.publication_gate));
    if scenario.id == DES_MOINES_SCENARIO_ID {
        out.push_str("- analyzer recognition: pass via curated I-35/I-80 anchor\n");
        out.push_str("- observed versus modeled failure data: required\n");
    } else {
        out.push_str("- observed weather closure frequency/duration: required\n");
        out.push_str(
            "- truck-capable alternate capacity and direct PTI/SLA validation: required\n",
        );
        out.push_str("- source requested is not source observed\n");
    }

    out.push_str("\nROUTE Engine Hooks\n");
    if scenario.id == DES_MOINES_SCENARIO_ID {
        out.push_str("- route sim scenario des-moines-interchange\n");
        out.push_str("- route sim scenario des-moines-interchange --intervention\n");
        out.push_str("- route diamond I35xI80\n");
    } else {
        out.push_str("- route sim scenario donner-closure\n");
        out.push_str("- route pressure-scenarios --gate-l2\n");
        out.push_str("- route game campaign --gate\n");
    }

    Ok(out)
}

pub fn print_scenarios() {
    print!("{}", render_scenarios());
}

pub fn print_inspect(scenario_id: &str) -> Result<()> {
    print!("{}", render_inspect(scenario_id)?);
    Ok(())
}

pub fn run_season_cli(
    scenario_id: &str,
    season: u8,
    event_slug: &str,
    project_slugs: &[String],
    state_path: Option<&Path>,
    write_state_path: Option<&Path>,
    append_log_path: Option<&Path>,
) -> Result<()> {
    let state = if let Some(path) = state_path {
        let body = std::fs::read_to_string(path)?;
        serde_json::from_str(&body)?
    } else {
        default_state(scenario_id)?
    };
    let result = run_season(state, season, event_slug, project_slugs)?;

    if let Some(path) = write_state_path {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent)?;
            }
        }
        std::fs::write(path, serde_json::to_string_pretty(&result.state)?)?;
    }

    if let Some(path) = append_log_path {
        append_session_log(path, &result)?;
    }

    print!("{}", render_season_result(&result));
    Ok(())
}

pub fn score_cli(
    scenario_id: &str,
    log_path: &Path,
    details: bool,
    gate_promotion: bool,
    engine_facts_override: Option<EngineFacts>,
) -> Result<()> {
    scenario_by_id(scenario_id)?;
    let body = std::fs::read_to_string(log_path)?;
    let mut result = score_session_log(scenario_id, body.as_bytes())?;
    if let Some(engine_facts) = engine_facts_override {
        result.engine_facts = Some(engine_facts);
    }
    if gate_promotion && !result.promotion_readiness.starts_with("ready") {
        anyhow::bail!("{}", result.promotion_readiness);
    }
    print!("{}", render_score_result(&result, details));
    Ok(())
}

pub fn campaign_cli(ledger_path: &Path, map_atlas_path: &Path, gate: bool) -> Result<()> {
    let rows = load_campaign_spine(ledger_path)?;
    let atlas_ids = load_map_atlas_ids(map_atlas_path)?;
    let blockers = campaign_gate_blockers(&rows, &atlas_ids);

    print!("{}", render_campaign_spine(&rows, blockers.len()));
    if gate && !blockers.is_empty() {
        anyhow::bail!("campaign spine gate failed: {}", blockers.join("; "));
    }
    if gate {
        println!("Campaign spine gate: PASS");
    }
    Ok(())
}

pub fn t2_service_overlays_cli(
    overlay_path: &Path,
    standards_path: &Path,
    map_atlas_path: &Path,
    gate: bool,
) -> Result<()> {
    let rows = load_t2_service_overlays(overlay_path)?;
    let standard_classes = load_t2_service_standard_classes(standards_path)?;
    let atlas_ids = load_map_atlas_ids(map_atlas_path)?;
    let blockers = t2_service_overlay_gate_blockers(&rows, &standard_classes, &atlas_ids);

    print!("{}", render_t2_service_overlays(&rows, blockers.len()));
    if gate && !blockers.is_empty() {
        anyhow::bail!("T2 service overlay gate failed: {}", blockers.join("; "));
    }
    if gate {
        println!("T2 service overlay gate: PASS");
    }
    Ok(())
}

pub fn default_state(scenario_id: &str) -> Result<GameState> {
    scenario_by_id(scenario_id)?;
    if scenario_id == DONNER_SCENARIO_ID {
        return Ok(GameState {
            scenario_id: scenario_id.to_string(),
            season: 0,
            budget: 13,
            construction_crews: 3,
            political_capital: 0,
            public_patience: 5,
            operations_capacity: 4,
            evidence_confidence: 1,
            weather_readiness: 3,
            active_projects: Vec::new(),
            completed_projects: Vec::new(),
            first_closure_seen: false,
            connector_package_complete: false,
            source_requested: false,
            validated_evidence_available: false,
            fiscal_crisis: false,
            trapped_queue: false,
            publication_gate: "locked: weather closure and alternate-capacity evidence missing"
                .to_string(),
        });
    }
    Ok(GameState {
        scenario_id: scenario_id.to_string(),
        season: 0,
        budget: 12,
        construction_crews: 3,
        political_capital: 5,
        public_patience: 6,
        operations_capacity: 4,
        evidence_confidence: 2,
        weather_readiness: 0,
        active_projects: Vec::new(),
        completed_projects: Vec::new(),
        first_closure_seen: false,
        connector_package_complete: false,
        source_requested: false,
        validated_evidence_available: false,
        fiscal_crisis: false,
        trapped_queue: false,
        publication_gate:
            "locked: empirical closure evidence and direct PTI/NPMRDS validation missing"
                .to_string(),
    })
}

fn load_campaign_spine(path: &Path) -> Result<Vec<CampaignRow>> {
    let file = std::fs::File::open(path)?;
    parse_campaign_spine(file)
}

fn parse_campaign_spine<R: std::io::Read>(reader: R) -> Result<Vec<CampaignRow>> {
    let mut reader = csv::Reader::from_reader(reader);
    let rows = reader
        .deserialize()
        .collect::<std::result::Result<Vec<CampaignRow>, csv::Error>>()?;
    Ok(rows)
}

fn load_t2_service_overlays(path: &Path) -> Result<Vec<T2ServiceOverlayRow>> {
    let file = std::fs::File::open(path)?;
    parse_t2_service_overlays(file)
}

fn parse_t2_service_overlays<R: std::io::Read>(reader: R) -> Result<Vec<T2ServiceOverlayRow>> {
    let mut reader = csv::Reader::from_reader(reader);
    let rows = reader
        .deserialize()
        .collect::<std::result::Result<Vec<T2ServiceOverlayRow>, csv::Error>>()?;
    Ok(rows)
}

fn load_t2_service_standard_classes(path: &Path) -> Result<HashSet<String>> {
    let mut reader = csv::Reader::from_path(path)?;
    t2_service_standard_classes_from_reader(&mut reader)
}

fn t2_service_standard_classes_from_reader<R: std::io::Read>(
    reader: &mut csv::Reader<R>,
) -> Result<HashSet<String>> {
    let headers = reader.headers()?.clone();
    let service_class_idx = headers
        .iter()
        .position(|header| header == "service_class")
        .ok_or_else(|| anyhow::anyhow!("T2 service standards missing service_class column"))?;
    let mut classes = HashSet::new();
    for record in reader.records() {
        let record = record?;
        let service_class = record.get(service_class_idx).unwrap_or_default().trim();
        if !service_class.is_empty() {
            classes.insert(service_class.to_string());
        }
    }
    Ok(classes)
}

fn load_map_atlas_ids(path: &Path) -> Result<HashSet<String>> {
    let mut reader = csv::Reader::from_path(path)?;
    map_atlas_ids_from_reader(&mut reader)
}

fn map_atlas_ids_from_reader<R: std::io::Read>(
    reader: &mut csv::Reader<R>,
) -> Result<HashSet<String>> {
    let headers = reader.headers()?.clone();
    let map_id_idx = headers
        .iter()
        .position(|header| header == "map_id")
        .ok_or_else(|| anyhow::anyhow!("map atlas is missing map_id column"))?;
    let mut ids = HashSet::new();
    for record in reader.records() {
        let record = record?;
        let map_id = record.get(map_id_idx).unwrap_or_default().trim();
        if !map_id.is_empty() {
            ids.insert(map_id.to_string());
        }
    }
    Ok(ids)
}

fn t2_service_overlay_gate_blockers(
    rows: &[T2ServiceOverlayRow],
    standard_classes: &HashSet<String>,
    atlas_ids: &HashSet<String>,
) -> Vec<String> {
    let mut blockers = Vec::new();
    let mut seen_classes = HashSet::new();
    for row in rows {
        let service_class = row.service_class.trim();
        if service_class.is_empty()
            || row.map_id.trim().is_empty()
            || row.scenario_hook.trim().is_empty()
            || row.incident_lever.trim().is_empty()
            || row.upgrade_lever.trim().is_empty()
            || row.restitch_lever.trim().is_empty()
            || row.release_gate.trim().is_empty()
        {
            blockers.push(format!("{service_class} has a blank required field"));
        }
        if !seen_classes.insert(service_class.to_string()) {
            blockers.push(format!("duplicate T2 service overlay for {service_class}"));
        }
        if !standard_classes.contains(service_class) {
            blockers.push(format!(
                "{service_class} is not defined in T2 service standards"
            ));
        }
        if !atlas_ids.contains(row.map_id.trim()) {
            blockers.push(format!(
                "{service_class} references unknown map_id {}",
                row.map_id
            ));
        }
        if !row.release_gate.contains("beck-t2-service-standards") {
            blockers.push(format!(
                "{service_class} release gate must reference beck-t2-service-standards"
            ));
        }
    }
    for service_class in standard_classes {
        if !seen_classes.contains(service_class) {
            blockers.push(format!("missing T2 service overlay for {service_class}"));
        }
    }
    blockers
}

fn campaign_gate_blockers(rows: &[CampaignRow], atlas_ids: &HashSet<String>) -> Vec<String> {
    let mut blockers = Vec::new();
    if rows.len() != 8 {
        blockers.push(format!("expected 8 campaign rows, found {}", rows.len()));
    }
    for (idx, row) in rows.iter().enumerate() {
        let expected_order = (idx + 1) as u8;
        if row.order != expected_order {
            blockers.push(format!(
                "{} order is {}, expected {}",
                row.scenario_id, row.order, expected_order
            ));
        }
        if row.phase_name.trim().is_empty()
            || row.scenario_id.trim().is_empty()
            || row.scenario_name.trim().is_empty()
            || row.tier_focus.trim().is_empty()
            || row.standard_lesson.trim().is_empty()
            || row.evidence_gate.trim().is_empty()
            || row.playable_status.trim().is_empty()
            || row.publication_gate.trim().is_empty()
            || row.next_artifact.trim().is_empty()
        {
            blockers.push(format!("{} has a blank required field", row.scenario_id));
        }
        if !atlas_ids.contains(&row.map_id) {
            blockers.push(format!(
                "{} references unknown map_id {}",
                row.scenario_id, row.map_id
            ));
        }
        if !row.publication_gate.starts_with("locked:") {
            blockers.push(format!(
                "{} publication gate must stay explicit and locked",
                row.scenario_id
            ));
        }
    }
    if rows.first().map(|row| row.scenario_id.as_str()) != Some(DES_MOINES_SCENARIO_ID) {
        blockers.push("campaign must start with des-moines-diamond".to_string());
    }
    blockers
}

fn render_t2_service_overlays(rows: &[T2ServiceOverlayRow], blocker_count: usize) -> String {
    let mut out = String::from("route game t2-overlays\n");
    out.push_str(&format!("  overlays: {}\n", rows.len()));
    out.push_str(&format!("  gate blockers: {blocker_count}\n\n"));
    out.push_str("Service Class     Map                    Scenario Hook\n");
    out.push_str("--------------------------------------------------------------------------\n");
    for row in rows {
        out.push_str(&format!(
            "{:<17} {:<22} {}\n",
            row.service_class, row.map_id, row.scenario_hook
        ));
    }
    out
}

fn render_campaign_spine(rows: &[CampaignRow], blocker_count: usize) -> String {
    let mut out = String::from("route game campaign\n");
    out.push_str(&format!("  stops: {}\n", rows.len()));
    out.push_str(&format!("  gate blockers: {blocker_count}\n\n"));
    out.push_str(
        "Order  Phase            Scenario                       Map              Status\n",
    );
    out.push_str(
        "--------------------------------------------------------------------------------\n",
    );
    for row in rows {
        out.push_str(&format!(
            "{:<6} {:<16} {:<30} {:<16} {}\n",
            row.order, row.phase_name, row.scenario_name, row.map_id, row.playable_status
        ));
    }
    out
}

pub fn run_season(
    mut state: GameState,
    season: u8,
    event_slug: &str,
    project_slugs: &[String],
) -> Result<SeasonResult> {
    scenario_by_id(&state.scenario_id)?;
    let scenario_id = state.scenario_id.clone();
    let event = event_by_slug(&scenario_id, event_slug)?;
    state.season = season;
    state.construction_crews = 3;

    let mut accepted_projects = Vec::new();
    let mut rejected_actions = Vec::new();
    let political_lane_pressure = event.slug == "political-lane-mile-pressure";

    for slug in project_slugs {
        let project = project_by_slug(&scenario_id, slug)?;
        let mut cost = project.cost;
        let mut political_cost = 0;
        if political_lane_pressure && project.slug == "general-purpose-widening" {
            cost -= 1;
        }
        if political_lane_pressure && project.slug == "diamond-connector-package" {
            political_cost = 1;
        }

        if matches!(
            project.slug,
            "validated-evidence" | "validated-weather-evidence"
        ) && !state.source_requested
        {
            rejected_actions.push(format!(
                "{} rejected: source request must be completed first.",
                project.name
            ));
            continue;
        }
        if matches!(
            project.slug,
            "validated-evidence" | "validated-weather-evidence"
        ) && !state.validated_evidence_available
        {
            rejected_actions.push(format!(
                "{} rejected: source requested is not source observed; no observed artifact exists yet.",
                project.name
            ));
            continue;
        }
        if state.budget < cost {
            rejected_actions.push(format!(
                "{} rejected: budget {} is below required cost {}.",
                project.name, state.budget, cost
            ));
            continue;
        }
        if state.construction_crews < project.crew {
            rejected_actions.push(format!(
                "{} rejected: construction crews {} are below required crew {}.",
                project.name, state.construction_crews, project.crew
            ));
            continue;
        }
        if state.political_capital < political_cost {
            rejected_actions.push(format!(
                "{} rejected: political capital {} is below required cost {}.",
                project.name, state.political_capital, political_cost
            ));
            continue;
        }
        if state
            .active_projects
            .iter()
            .any(|active| active.slug == project.slug)
        {
            rejected_actions.push(format!(
                "{} rejected: project is already active.",
                project.name
            ));
            continue;
        }
        if state
            .completed_projects
            .iter()
            .any(|completed| completed == project.slug)
        {
            rejected_actions.push(format!(
                "{} rejected: project is already complete.",
                project.name
            ));
            continue;
        }

        state.budget -= cost;
        state.construction_crews -= project.crew;
        state.political_capital -= political_cost;
        accepted_projects.push(project.slug.to_string());

        if project.time <= 1 {
            complete_project(&mut state, project);
        } else {
            state.active_projects.push(ActiveProject {
                slug: project.slug.to_string(),
                remaining_seasons: project.time as u8,
            });
        }
    }

    tick_active_projects(&mut state)?;

    let event_result = apply_event(&mut state, event);
    if state.budget < 0 {
        state.fiscal_crisis = true;
    }
    state.publication_gate = scenario_by_id(&scenario_id)?.publication_gate.to_string();

    let throughput_retention = if scenario_id == DONNER_SCENARIO_ID {
        donner_throughput_retention(&state)
    } else if state.connector_package_complete {
        1.0
    } else {
        0.962
    };
    let recovery_hours = if scenario_id == DONNER_SCENARIO_ID {
        donner_recovery_hours(&state)
    } else {
        0.9
    };
    let sla_status = if scenario_id == DONNER_SCENARIO_ID {
        donner_sla_status(&state)
    } else if state.operations_capacity >= 0 {
        "bounded heuristic".to_string()
    } else {
        "missed: operations capacity below zero".to_string()
    };

    Ok(SeasonResult {
        state,
        accepted_projects,
        rejected_actions,
        event_result,
        throughput_retention,
        recovery_hours,
        sla_status,
    })
}

pub fn render_season_result(result: &SeasonResult) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "route game run-season {}\n",
        result.state.scenario_id
    ));
    out.push_str(&format!("  season: {}\n", result.state.season));
    out.push_str(&format!(
        "  accepted_projects: {}\n",
        if result.accepted_projects.is_empty() {
            "none".to_string()
        } else {
            result.accepted_projects.join("; ")
        }
    ));
    out.push_str("  rejected_actions:\n");
    if result.rejected_actions.is_empty() {
        out.push_str("    none\n");
    } else {
        for action in &result.rejected_actions {
            out.push_str(&format!("    - {action}\n"));
        }
    }
    out.push_str(&format!("  event_result: {}\n", result.event_result));
    out.push_str(&format!(
        "  tracks: budget={} crews={} political_capital={} public_patience={} operations_capacity={} evidence_confidence={}\n",
        result.state.budget,
        result.state.construction_crews,
        result.state.political_capital,
        result.state.public_patience,
        result.state.operations_capacity,
        result.state.evidence_confidence
    ));
    if result.state.scenario_id == DONNER_SCENARIO_ID {
        out.push_str(&format!(
            "  mountain_tracks: weather_readiness={} trapped_queue={}\n",
            result.state.weather_readiness, result.state.trapped_queue
        ));
    }
    out.push_str(&format!(
        "  active_projects: {}\n",
        if result.state.active_projects.is_empty() {
            "none".to_string()
        } else {
            result
                .state
                .active_projects
                .iter()
                .map(|active| format!("{}:{} seasons", active.slug, active.remaining_seasons))
                .collect::<Vec<_>>()
                .join("; ")
        }
    ));
    out.push_str(&format!(
        "  completed_projects: {}\n",
        if result.state.completed_projects.is_empty() {
            "none".to_string()
        } else {
            result.state.completed_projects.join("; ")
        }
    ));
    out.push_str(&format!(
        "  throughput_retention: {:.3} heuristic\n",
        result.throughput_retention
    ));
    out.push_str(&format!(
        "  recovery_hours: {:.1} heuristic\n",
        result.recovery_hours
    ));
    out.push_str(&format!("  sla_status: {}\n", result.sla_status));
    out.push_str(&format!(
        "  publication_gate: {}\n",
        result.state.publication_gate
    ));
    out
}

pub fn score_session_log<R: std::io::Read>(scenario_id: &str, reader: R) -> Result<ScoreResult> {
    scenario_by_id(scenario_id)?;
    let mut csv = csv::Reader::from_reader(reader);
    let rows = csv
        .deserialize::<SessionLogRow>()
        .collect::<std::result::Result<Vec<_>, _>>()?;
    let last = rows
        .last()
        .ok_or_else(|| anyhow::anyhow!("session log is empty"))?;

    let throughput_points = if rows.iter().any(|row| row.throughput_retention >= 0.8) {
        25
    } else {
        0
    };
    let recovery_gate_hours = if scenario_id == DONNER_SCENARIO_ID {
        8.0
    } else {
        4.0
    };
    let recovery_points = if rows
        .iter()
        .any(|row| row.recovery_hours <= recovery_gate_hours)
    {
        20
    } else {
        0
    };
    let sla_points = if rows
        .iter()
        .all(|row| row.sla_status.contains("bounded") || row.sla_status.contains("pass"))
    {
        15
    } else {
        0
    };
    let budget_points = if last.budget_remaining >= 0 { 10 } else { 0 };
    let support_points = if last.political_capital >= 0 && last.public_patience >= 0 {
        10
    } else {
        0
    };
    let evidence_points = if rows.iter().all(|row| {
        !row.publication_gate.trim().is_empty()
            && (row.publication_gate.contains("locked")
                || row.publication_gate.contains("unlocked"))
    }) {
        20
    } else {
        0
    };
    let total = throughput_points
        + recovery_points
        + sla_points
        + budget_points
        + support_points
        + evidence_points;
    let win_band = if total >= 80 {
        "Operational win"
    } else if total >= 60 {
        "Partial win"
    } else {
        "Failure"
    };
    let publication_gate = last.publication_gate.clone();
    let promotion_readiness = if publication_gate.contains("unlocked") {
        "ready: publication gate unlocked".to_string()
    } else {
        "hold: publication gate locked; needs human blind playtest or owner acceptance plus observed evidence".to_string()
    };

    Ok(ScoreResult {
        scenario_id: scenario_id.to_string(),
        seasons: rows.len(),
        final_season: last.season,
        throughput_points,
        recovery_points,
        sla_points,
        budget_points,
        support_points,
        evidence_points,
        total,
        win_band,
        publication_gate,
        promotion_readiness,
        engine_facts: engine_facts_for(scenario_id),
    })
}

pub fn render_score_result(result: &ScoreResult, details: bool) -> String {
    let mut out = String::new();
    out.push_str(&format!("route game score {}\n", result.scenario_id));
    out.push_str(&format!("  seasons: {}\n", result.seasons));
    out.push_str(&format!("  final_season: {}\n", result.final_season));
    out.push_str(&format!("  operational_score: {}/100\n", result.total));
    out.push_str(&format!("  win_band: {}\n", result.win_band));
    out.push_str(&format!(
        "  publication_gate: {}\n",
        result.publication_gate
    ));
    out.push_str(&format!(
        "  promotion_readiness: {}\n",
        result.promotion_readiness
    ));
    if let Some(engine) = result.engine_facts {
        out.push_str("  engine_facts:\n");
        if result.scenario_id == DONNER_SCENARIO_ID {
            out.push_str(&format!(
                "    baseline_throughput_vph: {}\n",
                engine.baseline_throughput_vph
            ));
            out.push_str(&format!(
                "    incident_throughput_vph: {}\n",
                engine.incident_throughput_vph
            ));
            out.push_str("    synthetic_fixture_note: current sim shows no throughput delta; game scoring remains heuristic\n");
            out.push_str(&format!(
                "    tutorial_recovery_window_hours: {:.1}\n",
                engine.recovery_hours
            ));
        } else {
            out.push_str(&format!(
                "    baseline_throughput_vph: {}\n",
                engine.baseline_throughput_vph
            ));
            out.push_str(&format!(
                "    incident_throughput_vph: {}\n",
                engine.incident_throughput_vph
            ));
            out.push_str(&format!(
                "    intervention_throughput_vph: {}\n",
                engine.intervention_throughput_vph
            ));
            out.push_str(&format!(
                "    recovery_hours: {:.1}\n",
                engine.recovery_hours
            ));
            out.push_str(&format!(
                "    diamond_k_current: {}\n",
                engine.diamond_k_current
            ));
            out.push_str(&format!(
                "    connectors_needed_for_k3: {}\n",
                engine.connectors_needed
            ));
        }
        out.push_str(&format!("    evidence_level: {}\n", engine.evidence_level));
    }
    if details {
        out.push_str("  dimensions:\n");
        out.push_str(&format!(
            "    throughput_retention: {}/25\n",
            result.throughput_points
        ));
        out.push_str(&format!("    recovery: {}/20\n", result.recovery_points));
        out.push_str(&format!("    sla: {}/15\n", result.sla_points));
        out.push_str(&format!(
            "    budget_discipline: {}/10\n",
            result.budget_points
        ));
        out.push_str(&format!(
            "    public_support: {}/10\n",
            result.support_points
        ));
        out.push_str(&format!(
            "    evidence_honesty: {}/20\n",
            result.evidence_points
        ));
    }
    out
}

fn engine_facts_for(scenario_id: &str) -> Option<EngineFacts> {
    match scenario_id {
        DES_MOINES_SCENARIO_ID => Some(EngineFacts {
            baseline_throughput_vph: 86_671,
            incident_throughput_vph: 83_423,
            intervention_throughput_vph: 86_671,
            recovery_hours: 0.9,
            diamond_k_current: 0,
            connectors_needed: 3,
            evidence_level: "Heuristic",
        }),
        DONNER_SCENARIO_ID => Some(EngineFacts {
            baseline_throughput_vph: 86_671,
            incident_throughput_vph: 86_671,
            intervention_throughput_vph: 86_671,
            recovery_hours: 8.0,
            diamond_k_current: 0,
            connectors_needed: 0,
            evidence_level: "Heuristic",
        }),
        _ => None,
    }
}

fn append_session_log(path: &Path, result: &SeasonResult) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let needs_header = !path.exists() || std::fs::metadata(path)?.len() == 0;
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    if needs_header {
        writeln!(
            file,
            "season,accepted_projects,rejected_count,budget_remaining,political_capital,public_patience,operations_capacity,evidence_confidence,throughput_retention,recovery_hours,sla_status,publication_gate"
        )?;
    }
    writeln!(
        file,
        "{},\"{}\",{},{},{},{},{},{},{:.3},{:.1},\"{}\",\"{}\"",
        result.state.season,
        result.accepted_projects.join("; "),
        result.rejected_actions.len(),
        result.state.budget,
        result.state.political_capital,
        result.state.public_patience,
        result.state.operations_capacity,
        result.state.evidence_confidence,
        result.throughput_retention,
        result.recovery_hours,
        result.sla_status,
        result.state.publication_gate
    )?;
    Ok(())
}

fn complete_project(state: &mut GameState, project: &ProjectCard) {
    if !state
        .completed_projects
        .iter()
        .any(|completed| completed == project.slug)
    {
        state.completed_projects.push(project.slug.to_string());
    }
    match project.slug {
        "diamond-connector-package" => state.connector_package_complete = true,
        "winter-operations-package" => state.weather_readiness += 2,
        "early-egress-spurs" | "dynamic-closure-routing" => {
            if state.trapped_queue {
                state.trapped_queue = false;
            }
        }
        "source-request" => {
            state.source_requested = true;
            state.evidence_confidence += 1;
        }
        _ => {}
    }
}

fn tick_active_projects(state: &mut GameState) -> Result<()> {
    let mut still_active = Vec::new();
    let active_projects = std::mem::take(&mut state.active_projects);
    for mut active in active_projects {
        active.remaining_seasons = active.remaining_seasons.saturating_sub(1);
        if active.remaining_seasons == 0 {
            let project = project_by_slug(&state.scenario_id, &active.slug)?;
            complete_project(state, project);
        } else {
            still_active.push(active);
        }
    }
    state.active_projects = still_active;
    Ok(())
}

fn apply_event(state: &mut GameState, event: &EventCard) -> String {
    match event.slug {
        "full-interchange-zone-closure" => {
            state.first_closure_seen = true;
            if state.connector_package_complete {
                "Closure stress applied; redundant transfer paths are available.".to_string()
            } else {
                "Closure stress applied; transfer remains fragile because no independent transfer path is complete.".to_string()
            }
        }
        "night-work-zone-closure" => {
            if state
                .completed_projects
                .iter()
                .any(|project| project == "work-zone-sequencing")
            {
                "Night closure sequenced; public patience protected.".to_string()
            } else {
                state.public_patience -= 1;
                "Night closure unsequenced; public patience loses 1.".to_string()
            }
        }
        "relay-hub-surge" => {
            if state
                .completed_projects
                .iter()
                .any(|project| project == "relay-hub-reserve-staffing")
            {
                "Relay surge absorbed by reserve staffing.".to_string()
            } else {
                state.operations_capacity -= 1;
                "Relay surge strains operations; operations capacity loses 1.".to_string()
            }
        }
        "ev-rest-queue" => {
            if state
                .completed_projects
                .iter()
                .any(|project| project == "ev-rest-hardening")
            {
                "EV/rest queue absorbed by hardening.".to_string()
            } else {
                state.operations_capacity -= 1;
                "EV/rest queue adds dwell risk; operations capacity loses 1.".to_string()
            }
        }
        "political-lane-mile-pressure" => {
            "Lane-mile pressure applied; widening is cheaper and connector package costs political capital.".to_string()
        }
        "source-challenge" => {
            if state.evidence_confidence >= 4 {
                "Source challenge answered; evidence confidence clears the game threshold.".to_string()
            } else {
                "Source challenge holds publication locked; evidence confidence is below 4.".to_string()
            }
        }
        "whiteout-closure" => {
            state.first_closure_seen = true;
            let egress_ready = state
                .completed_projects
                .iter()
                .any(|project| project == "early-egress-spurs" || project == "dynamic-closure-routing");
            if egress_ready {
                "Whiteout closure applied; trucks can leave before the queue hardens.".to_string()
            } else {
                state.trapped_queue = true;
                "Whiteout closure applied; trapped-queue marker added before alternate capacity can absorb freight.".to_string()
            }
        }
        "chain-control-slowdown" => {
            if state
                .completed_projects
                .iter()
                .any(|project| project == "winter-operations-package")
            {
                "Chain control absorbed by winter operations.".to_string()
            } else {
                state.weather_readiness -= 1;
                "Chain control strains readiness; weather readiness loses 1.".to_string()
            }
        }
        "detour-capacity-pinch" => {
            if state.completed_projects.iter().any(|project| {
                project == "lower-elevation-freight-bypass"
                    || project == "rail-intermodal-surge-slots"
            }) {
                "Detour pinch absorbed by usable alternate capacity.".to_string()
            } else {
                state.operations_capacity -= 1;
                "Detour exists on the map but lacks capacity; operations capacity loses 1.".to_string()
            }
        }
        "reopening-surge" => {
            if state.completed_projects.iter().any(|project| {
                project == "general-snow-storage-shoulders"
                    || project == "dynamic-closure-routing"
            }) {
                "Reopening surge managed; queue drain stays inside the tutorial window.".to_string()
            } else {
                state.public_patience -= 1;
                "The pass reopened but the queue did not; public patience loses 1.".to_string()
            }
        }
        "high-value-sla-wave" => {
            if state.completed_projects.iter().any(|project| {
                project == "managed-freight-tunnel" || project == "rail-intermodal-surge-slots"
            }) {
                "Priority freight protected by tunnel or intermodal capacity.".to_string()
            } else {
                "Priority freight misses the bounded SLA because no protected capacity is ready.".to_string()
            }
        }
        "evidence-challenge" => {
            if state.evidence_confidence >= 4 && state.validated_evidence_available {
                "Evidence challenge answered with observed source artifacts.".to_string()
            } else {
                "Evidence challenge holds publication locked; source requested is not source observed.".to_string()
            }
        }
        _ => unreachable!("event slug was validated before apply_event"),
    }
}

fn donner_throughput_retention(state: &GameState) -> f64 {
    if state.completed_projects.iter().any(|project| {
        project == "lower-elevation-freight-bypass"
            || project == "managed-freight-tunnel"
            || project == "rail-intermodal-surge-slots"
    }) {
        0.82
    } else if state.completed_projects.iter().any(|project| {
        project == "early-egress-spurs"
            || project == "dynamic-closure-routing"
            || project == "winter-operations-package"
    }) {
        0.70
    } else if state.trapped_queue {
        0.50
    } else {
        0.62
    }
}

fn donner_recovery_hours(state: &GameState) -> f64 {
    if state.completed_projects.iter().any(|project| {
        project == "winter-operations-package"
            || project == "general-snow-storage-shoulders"
            || project == "dynamic-closure-routing"
    }) {
        8.0
    } else {
        14.0
    }
}

fn donner_sla_status(state: &GameState) -> String {
    if state.completed_projects.iter().any(|project| {
        project == "managed-freight-tunnel" || project == "rail-intermodal-surge-slots"
    }) {
        "bounded heuristic".to_string()
    } else {
        "missed: no protected priority-freight capacity".to_string()
    }
}

fn project_by_slug(scenario_id: &str, slug: &str) -> Result<&'static ProjectCard> {
    scenario_by_id(scenario_id)?
        .projects
        .iter()
        .find(|project| project.slug == slug)
        .ok_or_else(|| anyhow::anyhow!("unknown project card slug '{slug}'"))
}

fn event_by_slug(scenario_id: &str, slug: &str) -> Result<&'static EventCard> {
    scenario_by_id(scenario_id)?
        .events
        .iter()
        .find(|event| event.slug == slug)
        .ok_or_else(|| anyhow::anyhow!("unknown event card slug '{slug}'"))
}

fn scenario_by_id(scenario_id: &str) -> Result<&'static Scenario> {
    SCENARIOS
        .iter()
        .find(|scenario| scenario.id == scenario_id)
        .ok_or_else(|| {
            let ids = SCENARIOS
                .iter()
                .map(|scenario| scenario.id)
                .collect::<Vec<_>>()
                .join(", ");
            anyhow::anyhow!("unknown game scenario '{scenario_id}'. Available scenarios: {ids}")
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scenario_list_includes_des_moines_with_locked_publication_gate() {
        let rendered = render_scenarios();

        assert!(rendered.contains(DES_MOINES_SCENARIO_ID));
        assert!(rendered.contains("G0-B pass; G1-A start"));
        assert!(rendered.contains("locked: empirical closure evidence"));
    }

    #[test]
    fn campaign_spine_links_all_stops_to_map_atlas() {
        let campaign = include_str!("../../../data/game/campaign-spine.csv");
        let atlas = include_str!("../../../data/map-atlas.csv");
        let rows = parse_campaign_spine(campaign.as_bytes()).expect("campaign spine");
        let mut atlas_reader = csv::Reader::from_reader(atlas.as_bytes());
        let atlas_ids = map_atlas_ids_from_reader(&mut atlas_reader).expect("map atlas ids");
        let blockers = campaign_gate_blockers(&rows, &atlas_ids);
        let rendered = render_campaign_spine(&rows, blockers.len());

        assert!(blockers.is_empty(), "{blockers:?}");
        assert_eq!(rows.len(), 8);
        assert_eq!(rows[0].scenario_id, DES_MOINES_SCENARIO_ID);
        assert!(rendered.contains("Ignition"));
        assert!(rendered.contains("i35-region"));
        assert!(rendered.contains("Blueprint Hearing"));
    }

    #[test]
    fn campaign_spine_gate_rejects_unknown_map_ids() {
        let campaign = "\
order,phase_name,scenario_id,scenario_name,map_id,tier_focus,standard_lesson,evidence_gate,playable_status,publication_gate,next_artifact
1,Ignition,des-moines-diamond,Des Moines Diamond,missing-map,T1/T1,lesson,evidence,G0-A seed,locked: missing evidence,next
";
        let rows = parse_campaign_spine(campaign.as_bytes()).expect("campaign spine");
        let atlas_ids = HashSet::new();
        let blockers = campaign_gate_blockers(&rows, &atlas_ids);

        assert!(blockers
            .iter()
            .any(|blocker| blocker.contains("references unknown map_id missing-map")));
    }

    #[test]
    fn t2_service_overlays_cover_standards_and_map_atlas() {
        let overlays = include_str!("../../../data/game/t2-service-overlays.csv");
        let standards = include_str!("../../../data/beck-t2-service-standards.csv");
        let atlas = include_str!("../../../data/map-atlas.csv");
        let rows = parse_t2_service_overlays(overlays.as_bytes()).expect("T2 overlays");
        let mut standards_reader = csv::Reader::from_reader(standards.as_bytes());
        let standard_classes =
            t2_service_standard_classes_from_reader(&mut standards_reader).expect("standards");
        let mut atlas_reader = csv::Reader::from_reader(atlas.as_bytes());
        let atlas_ids = map_atlas_ids_from_reader(&mut atlas_reader).expect("map atlas ids");
        let blockers = t2_service_overlay_gate_blockers(&rows, &standard_classes, &atlas_ids);
        let rendered = render_t2_service_overlays(&rows, blockers.len());

        assert!(blockers.is_empty(), "{blockers:?}");
        assert_eq!(rows.len(), 4);
        assert!(rendered.contains("route game t2-overlays"));
        assert!(rendered.contains("transfer-spine"));
        assert!(rendered.contains("beck-schematic-t2-only"));
    }

    #[test]
    fn t2_service_overlay_gate_rejects_unknown_service_classes() {
        let overlays = "\
service_class,map_id,scenario_hook,incident_lever,upgrade_lever,restitch_lever,release_gate
mystery,beck-schematic-t2-only,hook,incident,upgrade,restitch,beck-t2-service-standards gate
";
        let rows = parse_t2_service_overlays(overlays.as_bytes()).expect("T2 overlays");
        let standard_classes = HashSet::from(["connector".to_string()]);
        let atlas_ids = HashSet::from(["beck-schematic-t2-only".to_string()]);
        let blockers = t2_service_overlay_gate_blockers(&rows, &standard_classes, &atlas_ids);

        assert!(blockers
            .iter()
            .any(|blocker| blocker.contains("mystery is not defined")));
        assert!(blockers
            .iter()
            .any(|blocker| blocker.contains("missing T2 service overlay for connector")));
    }

    #[test]
    fn inspect_includes_v0_2_cards_and_separate_publication_gate() {
        let rendered = render_inspect(DES_MOINES_SCENARIO_ID).expect("render inspect");

        assert!(rendered.contains("version: G0 v0.2"));
        assert!(rendered.contains("Diamond connector package (diamond-connector-package)"));
        assert!(rendered.contains("Source request (source-request)"));
        assert!(rendered.contains("Validated evidence (validated-evidence)"));
        assert!(rendered.contains("General-purpose widening"));
        assert!(rendered.contains("does not add independent transfer paths"));
        assert!(rendered.contains("Publication Gate"));
        assert!(rendered.contains("observed versus modeled failure data: required"));
    }

    #[test]
    fn unknown_scenario_is_rejected_with_available_ids() {
        let error = render_inspect("missing-scenario").expect_err("unknown scenario");

        assert!(error.to_string().contains("unknown game scenario"));
        assert!(error.to_string().contains(DES_MOINES_SCENARIO_ID));
    }

    #[test]
    fn project_and_event_slugs_validate_against_paper_cards() {
        assert_eq!(
            project_by_slug(DES_MOINES_SCENARIO_ID, "source-request")
                .expect("source card")
                .name,
            "Source request"
        );
        assert_eq!(
            event_by_slug(DES_MOINES_SCENARIO_ID, "full-interchange-zone-closure")
                .expect("closure event")
                .name,
            "Full interchange-zone closure"
        );
        assert!(project_by_slug(DES_MOINES_SCENARIO_ID, "evidence-acquisition").is_err());
        assert!(event_by_slug(DES_MOINES_SCENARIO_ID, "unknown-event").is_err());
    }

    #[test]
    fn default_state_matches_g0_tracks() {
        let state = default_state(DES_MOINES_SCENARIO_ID).expect("default state");

        assert_eq!(state.budget, 12);
        assert_eq!(state.construction_crews, 3);
        assert_eq!(state.political_capital, 5);
        assert_eq!(state.public_patience, 6);
        assert_eq!(state.operations_capacity, 4);
        assert_eq!(state.evidence_confidence, 2);
        assert!(state.publication_gate.contains("locked"));
    }

    #[test]
    fn run_season_rejects_unaffordable_projects_with_reasons() {
        let mut state = default_state(DES_MOINES_SCENARIO_ID).expect("default state");
        state.budget = 1;

        let projects = vec!["diamond-connector-package".to_string()];
        let result =
            run_season(state, 1, "full-interchange-zone-closure", &projects).expect("run season");

        assert!(result.accepted_projects.is_empty());
        assert_eq!(result.state.budget, 1);
        assert!(result.rejected_actions[0].contains("budget 1 is below required cost 5"));
        assert!(result.event_result.contains("no independent transfer path"));
    }

    #[test]
    fn run_season_splits_source_request_from_validated_evidence() {
        let state = default_state(DES_MOINES_SCENARIO_ID).expect("default state");
        let projects = vec![
            "source-request".to_string(),
            "validated-evidence".to_string(),
        ];
        let result = run_season(state, 2, "source-challenge", &projects).expect("run season");

        assert_eq!(result.accepted_projects, vec!["source-request"]);
        assert_eq!(result.state.evidence_confidence, 3);
        assert!(result.state.source_requested);
        assert!(result.rejected_actions[0].contains("source requested is not source observed"));
        assert!(result.state.publication_gate.contains("locked"));
    }

    #[test]
    fn scenario_list_includes_donner_with_locked_publication_gate() {
        let rendered = render_scenarios();

        assert!(rendered.contains(DONNER_SCENARIO_ID));
        assert!(rendered.contains("T1-SPURS; T1-CLIMATE; T1-RECOVERY; T1-INTERMODAL"));
        assert!(
            rendered.contains("locked: weather closure and alternate-capacity evidence missing")
        );
    }

    #[test]
    fn inspect_includes_donner_v0_2_trapped_queue_and_source_observed_copy() {
        let rendered = render_inspect(DONNER_SCENARIO_ID).expect("render inspect");

        assert!(rendered.contains("Scenario: Donner Weather Closure"));
        assert!(rendered.contains("version: G0 v0.2"));
        assert!(rendered.contains("Early egress spurs (early-egress-spurs)"));
        assert!(rendered.contains("Managed freight tunnel (managed-freight-tunnel)"));
        assert!(rendered.contains("trapped-queue marker"));
        assert!(rendered.contains("source requested is not source observed"));
        assert!(rendered.contains("route sim scenario donner-closure"));
    }

    #[test]
    fn donner_whiteout_adds_trapped_queue_until_egress_or_routing_is_ready() {
        let state = default_state(DONNER_SCENARIO_ID).expect("default state");
        let result =
            run_season(state, 1, "whiteout-closure", &Vec::<String>::new()).expect("whiteout");

        assert!(result.state.trapped_queue);
        assert_eq!(result.throughput_retention, 0.50);
        assert!(result.event_result.contains("trapped-queue marker"));

        let projects = vec!["dynamic-closure-routing".to_string()];
        let result =
            run_season(result.state, 2, "whiteout-closure", &projects).expect("routing season");

        assert!(!result.state.trapped_queue);
        assert_eq!(result.throughput_retention, 0.70);
        assert!(result
            .event_result
            .contains("leave before the queue hardens"));
    }

    #[test]
    fn donner_source_request_does_not_unlock_validated_weather_evidence() {
        let state = default_state(DONNER_SCENARIO_ID).expect("default state");
        let projects = vec![
            "source-request".to_string(),
            "validated-weather-evidence".to_string(),
        ];
        let result = run_season(state, 1, "evidence-challenge", &projects).expect("evidence");

        assert_eq!(result.accepted_projects, vec!["source-request"]);
        assert_eq!(result.state.evidence_confidence, 2);
        assert!(result.rejected_actions[0].contains("source requested is not source observed"));
        assert!(result.state.publication_gate.contains("alternate-capacity"));
        assert!(result
            .event_result
            .contains("source requested is not source observed"));
    }

    #[test]
    fn render_season_result_keeps_publication_separate_from_operations() {
        let state = default_state(DES_MOINES_SCENARIO_ID).expect("default state");
        let projects = vec!["work-zone-sequencing".to_string()];
        let result =
            run_season(state, 1, "night-work-zone-closure", &projects).expect("run season");
        let rendered = render_season_result(&result);

        assert!(rendered.contains("accepted_projects: work-zone-sequencing"));
        assert!(rendered.contains("sla_status: bounded heuristic"));
        assert!(rendered.contains("publication_gate: locked"));
    }

    #[test]
    fn active_projects_count_down_and_complete_before_event_resolution() {
        let state = default_state(DES_MOINES_SCENARIO_ID).expect("default state");
        let projects = vec!["diamond-connector-package".to_string()];
        let result = run_season(state, 1, "night-work-zone-closure", &projects).expect("season 1");

        assert_eq!(
            result.state.active_projects[0].slug,
            "diamond-connector-package"
        );
        assert_eq!(result.state.active_projects[0].remaining_seasons, 2);
        assert!(!result.state.connector_package_complete);

        let result = run_season(
            result.state,
            2,
            "night-work-zone-closure",
            &Vec::<String>::new(),
        )
        .expect("season 2");
        assert_eq!(result.state.active_projects[0].remaining_seasons, 1);

        let result = run_season(
            result.state,
            3,
            "full-interchange-zone-closure",
            &Vec::<String>::new(),
        )
        .expect("season 3");
        assert!(result.state.active_projects.is_empty());
        assert!(result.state.connector_package_complete);
        assert!(result.event_result.contains("redundant transfer paths"));
        assert_eq!(result.throughput_retention, 1.0);
    }

    #[test]
    fn active_project_cannot_be_started_twice() {
        let state = default_state(DES_MOINES_SCENARIO_ID).expect("default state");
        let projects = vec!["diamond-connector-package".to_string()];
        let result = run_season(state, 1, "night-work-zone-closure", &projects).expect("season 1");
        let result =
            run_season(result.state, 2, "night-work-zone-closure", &projects).expect("season 2");

        assert!(result.rejected_actions[0].contains("already active"));
    }

    #[test]
    fn score_session_log_reports_operational_win_but_locked_publication() {
        let csv = "\
season,accepted_projects,rejected_count,budget_remaining,political_capital,public_patience,operations_capacity,evidence_confidence,throughput_retention,recovery_hours,sla_status,publication_gate
1,\"work-zone-sequencing\",0,11,5,6,4,2,0.962,0.9,\"bounded heuristic\",\"locked: empirical closure evidence missing\"
2,\"relay-hub-reserve-staffing\",0,9,5,6,4,2,0.962,0.9,\"bounded heuristic\",\"locked: empirical closure evidence missing\"
";

        let score = score_session_log(DES_MOINES_SCENARIO_ID, csv.as_bytes()).expect("score");
        let rendered = render_score_result(&score, true);

        assert_eq!(score.total, 100);
        assert_eq!(score.win_band, "Operational win");
        assert!(score.publication_gate.contains("locked"));
        assert!(score.promotion_readiness.starts_with("hold"));
        assert!(rendered.contains("evidence_honesty: 20/20"));
    }

    #[test]
    fn checked_in_score_fixture_preserves_g1_a_output_contract() {
        let fixture = include_str!("../../../data/game/des-moines-diamond-session-fixture.csv");

        let score = score_session_log(DES_MOINES_SCENARIO_ID, fixture.as_bytes()).expect("score");
        let rendered = render_score_result(&score, true);

        assert_eq!(score.seasons, 2);
        assert_eq!(score.final_season, 2);
        assert_eq!(score.total, 100);
        assert_eq!(score.win_band, "Operational win");
        assert!(score.publication_gate.contains("locked"));
        assert!(score.promotion_readiness.starts_with("hold"));
        assert!(rendered.contains("route game score des-moines-diamond"));
        assert!(rendered.contains("operational_score: 100/100"));
        assert!(rendered.contains("publication_gate: locked"));
        assert!(rendered.contains("promotion_readiness: hold"));
        assert!(rendered.contains("engine_facts:"));
        assert!(rendered.contains("baseline_throughput_vph: 86671"));
        assert!(rendered.contains("intervention_throughput_vph: 86671"));
        assert!(rendered.contains("diamond_k_current: 0"));
        assert!(rendered.contains("connectors_needed_for_k3: 3"));
        assert!(rendered.contains("throughput_retention: 25/25"));
        assert!(rendered.contains("evidence_honesty: 20/20"));
    }

    #[test]
    fn checked_in_donner_fixture_scores_operational_win_with_locked_publication() {
        let fixture = include_str!("../../../data/game/donner-weather-closure-session-fixture.csv");

        let score = score_session_log(DONNER_SCENARIO_ID, fixture.as_bytes()).expect("score");
        let rendered = render_score_result(&score, true);

        assert_eq!(score.seasons, 2);
        assert_eq!(score.final_season, 2);
        assert_eq!(score.total, 85);
        assert_eq!(score.win_band, "Operational win");
        assert!(score.publication_gate.contains("locked"));
        assert!(score.promotion_readiness.starts_with("hold"));
        assert!(rendered.contains("route game score donner-weather-closure"));
        assert!(rendered.contains("synthetic_fixture_note"));
        assert!(rendered.contains("tutorial_recovery_window_hours: 8.0"));
        assert!(rendered.contains("sla: 0/15"));
        assert!(rendered.contains("evidence_honesty: 20/20"));
    }

    #[test]
    fn score_session_log_rejects_empty_logs() {
        let csv = "season,accepted_projects,rejected_count,budget_remaining,political_capital,public_patience,operations_capacity,evidence_confidence,throughput_retention,recovery_hours,sla_status,publication_gate\n";

        let error =
            score_session_log(DES_MOINES_SCENARIO_ID, csv.as_bytes()).expect_err("empty log");

        assert!(error.to_string().contains("session log is empty"));
    }
}
