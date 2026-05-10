use anyhow::Result;

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
fn validate_project_slug(slug: &str) -> Result<&'static ProjectCard> {
    PROJECTS
        .iter()
        .find(|project| project.slug == slug)
        .ok_or_else(|| anyhow::anyhow!("unknown project card slug '{slug}'"))
}

#[cfg(test)]
fn validate_event_slug(slug: &str) -> Result<&'static EventCard> {
    EVENTS
        .iter()
        .find(|event| event.slug == slug)
        .ok_or_else(|| anyhow::anyhow!("unknown event card slug '{slug}'"))
}

#[cfg(test)]
fn validate_default_state() -> Result<()> {
    if !TRACKS
        .iter()
        .any(|track| track.name == "Budget" && track.start == 12)
    {
        anyhow::bail!("Des Moines default budget drifted from paper scenario");
    }
    if !TRACKS
        .iter()
        .any(|track| track.name == "Evidence confidence" && track.start == 2)
    {
        anyhow::bail!("Des Moines default evidence confidence drifted from paper scenario");
    }
    Ok(())
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
            validate_project_slug("source-request")
                .expect("source card")
                .name,
            "Source request"
        );
        assert_eq!(
            validate_event_slug("full-interchange-zone-closure")
                .expect("closure event")
                .name,
            "Full interchange-zone closure"
        );
        assert!(validate_project_slug("evidence-acquisition").is_err());
        assert!(validate_event_slug("unknown-event").is_err());
    }

    #[test]
    fn default_state_matches_g0_tracks() {
        validate_default_state().expect("default state matches paper scenario");
    }
}
