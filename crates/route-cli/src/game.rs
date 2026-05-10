use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub const DES_MOINES_SCENARIO_ID: &str = "des-moines-diamond";

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
pub struct GameState {
    pub scenario_id: String,
    pub season: u8,
    pub budget: i32,
    pub construction_crews: i32,
    pub political_capital: i32,
    pub public_patience: i32,
    pub operations_capacity: i32,
    pub evidence_confidence: i32,
    pub active_projects: Vec<String>,
    pub completed_projects: Vec<String>,
    pub first_closure_seen: bool,
    pub connector_package_complete: bool,
    pub source_requested: bool,
    pub validated_evidence_available: bool,
    pub fiscal_crisis: bool,
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

pub const SCENARIOS: &[Scenario] = &[Scenario {
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
}];

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
    out.push_str("- analyzer recognition: pass via curated I-35/I-80 anchor\n");
    out.push_str("- observed versus modeled failure data: required\n");

    out.push_str("\nROUTE Engine Hooks\n");
    out.push_str("- route sim scenario des-moines-interchange\n");
    out.push_str("- route sim scenario des-moines-interchange --intervention\n");
    out.push_str("- route diamond I35xI80\n");

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

pub fn default_state(scenario_id: &str) -> Result<GameState> {
    scenario_by_id(scenario_id)?;
    Ok(GameState {
        scenario_id: scenario_id.to_string(),
        season: 0,
        budget: 12,
        construction_crews: 3,
        political_capital: 5,
        public_patience: 6,
        operations_capacity: 4,
        evidence_confidence: 2,
        active_projects: Vec::new(),
        completed_projects: Vec::new(),
        first_closure_seen: false,
        connector_package_complete: false,
        source_requested: false,
        validated_evidence_available: false,
        fiscal_crisis: false,
        publication_gate:
            "locked: empirical closure evidence and direct PTI/NPMRDS validation missing"
                .to_string(),
    })
}

pub fn run_season(
    mut state: GameState,
    season: u8,
    event_slug: &str,
    project_slugs: &[String],
) -> Result<SeasonResult> {
    scenario_by_id(&state.scenario_id)?;
    let event = event_by_slug(event_slug)?;
    state.season = season;
    state.construction_crews = 3;

    let mut accepted_projects = Vec::new();
    let mut rejected_actions = Vec::new();
    let political_lane_pressure = event.slug == "political-lane-mile-pressure";

    for slug in project_slugs {
        let project = project_by_slug(slug)?;
        let mut cost = project.cost;
        let mut political_cost = 0;
        if political_lane_pressure && project.slug == "general-purpose-widening" {
            cost -= 1;
        }
        if political_lane_pressure && project.slug == "diamond-connector-package" {
            political_cost = 1;
        }

        if project.slug == "validated-evidence" && !state.source_requested {
            rejected_actions.push(format!(
                "{} rejected: source request must be completed first.",
                project.name
            ));
            continue;
        }
        if project.slug == "validated-evidence" && !state.validated_evidence_available {
            rejected_actions.push(format!(
                "{} rejected: validated evidence unavailable; no observed artifact exists yet.",
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

        state.budget -= cost;
        state.construction_crews -= project.crew;
        state.political_capital -= political_cost;
        accepted_projects.push(project.slug.to_string());

        if project.time <= 1 {
            complete_project(&mut state, project);
        } else if !state
            .active_projects
            .iter()
            .any(|active| active == project.slug)
        {
            state.active_projects.push(project.slug.to_string());
        }
    }

    let event_result = apply_event(&mut state, event);
    if state.budget < 0 {
        state.fiscal_crisis = true;
    }
    state.publication_gate =
        "locked: empirical closure evidence and direct PTI/NPMRDS validation missing".to_string();

    let throughput_retention = if state.connector_package_complete {
        1.0
    } else {
        0.962
    };
    let recovery_hours = 0.9;
    let sla_status = if state.operations_capacity >= 0 {
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
        "source-request" => {
            state.source_requested = true;
            state.evidence_confidence += 1;
        }
        _ => {}
    }
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
        _ => unreachable!("event slug was validated before apply_event"),
    }
}

fn project_by_slug(slug: &str) -> Result<&'static ProjectCard> {
    PROJECTS
        .iter()
        .find(|project| project.slug == slug)
        .ok_or_else(|| anyhow::anyhow!("unknown project card slug '{slug}'"))
}

fn event_by_slug(slug: &str) -> Result<&'static EventCard> {
    EVENTS
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
            project_by_slug("source-request").expect("source card").name,
            "Source request"
        );
        assert_eq!(
            event_by_slug("full-interchange-zone-closure")
                .expect("closure event")
                .name,
            "Full interchange-zone closure"
        );
        assert!(project_by_slug("evidence-acquisition").is_err());
        assert!(event_by_slug("unknown-event").is_err());
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
        assert!(result.rejected_actions[0].contains("validated evidence unavailable"));
        assert!(result.state.publication_gate.contains("locked"));
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
}
