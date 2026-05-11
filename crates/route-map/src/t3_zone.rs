use std::collections::HashMap;

use anyhow::{anyhow, Result};
use route_network::{HighwayGraph, RouteTier};

const W: f64 = 1800.0;
const H: f64 = 1000.0;

pub struct T3Stop {
    pub id: String,
    pub name: String,
    pub class_name: String,
    pub lat: f64,
    pub lon: f64,
}

struct T3Zone {
    id: &'static str,
    title: &'static str,
    status: &'static str,
    goal: &'static str,
    next_step: &'static str,
}

struct Stop {
    id: &'static str,
    label: &'static str,
    x: f64,
    y: f64,
    class_name: &'static str,
    transfer: bool,
    label_dir: LabelDir,
}

struct Line {
    route: &'static str,
    tier: RouteTier,
    status: LineStatus,
    stop_ids: &'static [&'static str],
    lane_shift: (f64, f64),
    badge_offset: (f64, f64),
    label_anchor: LabelAnchor,
    special_lane: SpecialLane,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LabelDir {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LabelAnchor {
    Start,
    End,
    Middle,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LineStatus {
    Passing,
    Blocked,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SpecialLane {
    None,
    DedicatedRelay,
    IncidentBypass,
    HardeningPilot,
    RestitchCandidate,
}

impl LineStatus {
    fn as_str(self) -> &'static str {
        match self {
            LineStatus::Passing => "passing",
            LineStatus::Blocked => "blocked",
        }
    }
}

impl SpecialLane {
    fn as_str(self) -> &'static str {
        match self {
            SpecialLane::None => "none",
            SpecialLane::DedicatedRelay => "dedicated-relay",
            SpecialLane::IncidentBypass => "incident-bypass",
            SpecialLane::HardeningPilot => "hardening-pilot",
            SpecialLane::RestitchCandidate => "restitch-candidate",
        }
    }

    fn color(self) -> &'static str {
        match self {
            SpecialLane::None => "#f8fafc",
            SpecialLane::DedicatedRelay => "#facc15",
            SpecialLane::IncidentBypass => "#38bdf8",
            SpecialLane::HardeningPilot => "#22d3ee",
            SpecialLane::RestitchCandidate => "#fb7185",
        }
    }

    fn project_id(self) -> &'static str {
        match self {
            SpecialLane::None => "",
            SpecialLane::DedicatedRelay => "managed-lane-pilot",
            SpecialLane::IncidentBypass => "incident-bypass-lane",
            SpecialLane::HardeningPilot => "resilience-hardening-pilot",
            SpecialLane::RestitchCandidate => "regional-restitch-candidate",
        }
    }

    fn project_name(self) -> &'static str {
        match self {
            SpecialLane::None => "",
            SpecialLane::DedicatedRelay => "Dedicated relay lane",
            SpecialLane::IncidentBypass => "Incident bypass lane",
            SpecialLane::HardeningPilot => "Weather hardening pilot",
            SpecialLane::RestitchCandidate => "Regional restitch candidate",
        }
    }

    fn cost_band(self) -> &'static str {
        match self {
            SpecialLane::None => "",
            SpecialLane::DedicatedRelay => "corridor-specific",
            SpecialLane::IncidentBypass => "moderate-capex",
            SpecialLane::HardeningPilot => "source-gated",
            SpecialLane::RestitchCandidate => "planning-study",
        }
    }

    fn effect(self) -> &'static str {
        match self {
            SpecialLane::None => "",
            SpecialLane::DedicatedRelay => "raises relay throughput and schedule reliability",
            SpecialLane::IncidentBypass => "preserves movement during closures and work zones",
            SpecialLane::HardeningPilot => "reduces weather disruption sensitivity",
            SpecialLane::RestitchCandidate => "tests whether a blocked feeder should connect",
        }
    }

    fn evidence_status(self) -> &'static str {
        match self {
            SpecialLane::None => "",
            SpecialLane::DedicatedRelay => "heuristic-held",
            SpecialLane::IncidentBypass => "scenario-candidate",
            SpecialLane::HardeningPilot => "source-gated",
            SpecialLane::RestitchCandidate => "candidate",
        }
    }
}

fn zones() -> &'static [T3Zone] {
    &[
        T3Zone {
            id: "T3ZGREATLAKES",
            title: "Great Lakes / Ohio Valley",
            status: "ready seed",
            goal: "Prove regional T3 rules on I-71, I-74, and I-83 before adding blocked feeders.",
            next_step: "Use this schematic to author missing regional terminal stops for I-79, I-96, and I-88.",
        },
        T3Zone {
            id: "T3ZSOUTHEAST",
            title: "Southeast / Appalachia",
            status: "blocked: missing stops",
            goal: "Resolve port, Appalachian, and storm-resilience feeders without promoting local spurs to T2.",
            next_step: "Author visible terminals for Asheville, Orlando, Cumberland, and State College.",
        },
        T3Zone {
            id: "T3ZTEXASBORDER",
            title: "Texas Border / Gulf Access",
            status: "blocked: missing stops",
            goal: "Force border, desert, and agricultural feeders to land on real transfer or terminal stops.",
            next_step: "Add regional terminal stops for McAllen, Tucson, Lubbock, Yuma, and Tulsa.",
        },
        T3Zone {
            id: "T3ZMOUNTAINWEST",
            title: "Mountain West / Interior Coverage",
            status: "blocked: missing stops",
            goal: "Represent rural coverage corridors by zone, not on the national Beck map.",
            next_step: "Select one coverage corridor and author an S3/S4/S5 chain with endpoint notes.",
        },
        T3Zone {
            id: "T3ZMIDSOUTH",
            title: "Mid-South / Delta / Ozarks",
            status: "blocked: missing stops",
            goal: "Separate real regional feeders from local-access T4 branches.",
            next_step: "Build candidate stop set from Mississippi Delta and Ozarks coverage roles.",
        },
    ]
}

pub fn build_t3_zone_svg(
    _graph: &HighwayGraph,
    zone_id: &str,
    _stops: &[T3Stop],
    _scores: &HashMap<String, f32>,
) -> Result<String> {
    let norm = zone_id
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_uppercase();
    let zone = zones()
        .iter()
        .find(|zone| zone.id == norm)
        .ok_or_else(|| anyhow!("unknown T3 zone '{zone_id}'"))?;
    let (stops, lines) = zone_schematic(zone.id);
    Ok(render_zone(zone, &stops, &lines))
}

pub fn build_t3_zone_board_csv(zone_id: &str) -> Result<String> {
    let norm = zone_id
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_uppercase();
    let zone = zones()
        .iter()
        .find(|zone| zone.id == norm)
        .ok_or_else(|| anyhow!("unknown T3 zone '{zone_id}'"))?;
    let (stops, lines) = zone_schematic(zone.id);
    Ok(render_board_csv(zone, &stops, &lines))
}

fn zone_schematic(zone_id: &str) -> (Vec<Stop>, Vec<Line>) {
    match zone_id {
        "T3ZGREATLAKES" => (
            vec![
                stop_dir(
                    "CIN",
                    "Cincinnati",
                    420.0,
                    760.0,
                    "S2",
                    true,
                    LabelDir::Down,
                ),
                stop_dir("COL", "Columbus", 640.0, 610.0, "S2", true, LabelDir::Right),
                stop_dir("TOL", "Toledo", 700.0, 360.0, "S2", true, LabelDir::Up),
                stop_dir(
                    "IND",
                    "Indianapolis",
                    300.0,
                    620.0,
                    "S2",
                    true,
                    LabelDir::Left,
                ),
                stop("HAR", "Harrisburg", 1320.0, 560.0, "S2", true),
                stop("PIT", "Pittsburgh", 1030.0, 650.0, "S3", true),
                stop("CLE", "Cleveland", 820.0, 420.0, "S4?", false),
                stop_dir("DET", "Detroit", 720.0, 230.0, "S4?", false, LabelDir::Up),
                stop("GRR", "Grand Rapids", 500.0, 230.0, "S4?", false),
                stop("RFD", "Rockford", 220.0, 360.0, "S4?", false),
                stop("YRK", "York", 1380.0, 670.0, "S4?", false),
                stop_dir(
                    "BAL",
                    "Baltimore",
                    1480.0,
                    760.0,
                    "S2",
                    true,
                    LabelDir::Down,
                ),
            ],
            vec![
                line(
                    "I-75",
                    RouteTier::T1,
                    LineStatus::Passing,
                    &["CIN", "TOL", "DET"],
                ),
                line(
                    "I-80",
                    RouteTier::T1,
                    LineStatus::Passing,
                    &["RFD", "TOL", "CLE"],
                ),
                line("I-76", RouteTier::T1, LineStatus::Passing, &["PIT", "HAR"]),
                line(
                    "I-70",
                    RouteTier::T2,
                    LineStatus::Passing,
                    &["IND", "COL", "PIT", "HAR"],
                ),
                line("I-65", RouteTier::T2, LineStatus::Passing, &["IND", "CIN"]),
                line_project(
                    "I-71",
                    RouteTier::T3,
                    LineStatus::Passing,
                    &["CIN", "COL", "CLE"],
                    (16.0, -14.0),
                    (30.0, -20.0),
                    LabelAnchor::Middle,
                    SpecialLane::DedicatedRelay,
                ),
                line_style(
                    "I-74",
                    RouteTier::T3,
                    LineStatus::Passing,
                    &["CIN", "IND", "RFD"],
                    (-18.0, 18.0),
                    (-28.0, 22.0),
                    LabelAnchor::Middle,
                ),
                line_style(
                    "I-83",
                    RouteTier::T3,
                    LineStatus::Passing,
                    &["BAL", "YRK", "HAR"],
                    (14.0, 8.0),
                    (30.0, 14.0),
                    LabelAnchor::Middle,
                ),
                line_style(
                    "I-79",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["PIT", "CLE"],
                    (18.0, 0.0),
                    (22.0, -18.0),
                    LabelAnchor::Middle,
                ),
                line_style(
                    "I-96",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["DET", "GRR"],
                    (0.0, -16.0),
                    (0.0, -24.0),
                    LabelAnchor::Middle,
                ),
                line_style(
                    "I-88",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["RFD", "CLE"],
                    (0.0, 16.0),
                    (0.0, 24.0),
                    LabelAnchor::End,
                ),
            ],
        ),
        "T3ZSOUTHEAST" => (
            vec![
                stop("ATL", "Atlanta", 560.0, 470.0, "S2", true),
                stop("BHM", "Birmingham", 380.0, 520.0, "S2", true),
                stop("KNX", "Knoxville", 650.0, 300.0, "S3", true),
                stop("JAX", "Jacksonville", 1060.0, 760.0, "S2", true),
                stop("SAV", "Savannah", 940.0, 650.0, "S3", true),
                stop("HAR", "Harrisburg", 1360.0, 200.0, "S2", true),
                stop("AVL", "Asheville", 760.0, 330.0, "S4?", false),
                stop("ORL", "Orlando", 1120.0, 880.0, "S4?", false),
                stop("CUM", "Cumberland", 1180.0, 260.0, "S4?", false),
                stop("SC", "State College", 1300.0, 120.0, "S4?", false),
            ],
            vec![
                line("I-75", RouteTier::T1, LineStatus::Passing, &["KNX", "ATL"]),
                line("I-95", RouteTier::T1, LineStatus::Passing, &["SAV", "JAX"]),
                line("I-81", RouteTier::T1, LineStatus::Passing, &["HAR", "KNX"]),
                line("I-20", RouteTier::T2, LineStatus::Passing, &["BHM", "ATL"]),
                line("I-16", RouteTier::T3, LineStatus::Blocked, &["ATL", "SAV"]),
                line_project(
                    "I-26",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["KNX", "AVL", "SAV"],
                    (18.0, -12.0),
                    (28.0, -20.0),
                    LabelAnchor::Middle,
                    SpecialLane::IncidentBypass,
                ),
                line("I-4", RouteTier::T3, LineStatus::Blocked, &["JAX", "ORL"]),
                line("I-68", RouteTier::T3, LineStatus::Blocked, &["HAR", "CUM"]),
                line("I-99", RouteTier::T3, LineStatus::Blocked, &["HAR", "SC"]),
                line(
                    "US-220",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["CUM", "AVL"],
                ),
            ],
        ),
        "T3ZTEXASBORDER" => (
            vec![
                stop("DFW", "Dallas/Fort Worth", 780.0, 320.0, "S2", true),
                stop("SAT", "San Antonio", 760.0, 650.0, "S2", true),
                stop("LRD", "Laredo", 620.0, 820.0, "S1", true),
                stop("CRP", "Corpus Christi", 900.0, 790.0, "S2", true),
                stop("ELP", "El Paso", 220.0, 650.0, "S2", true),
                stop("PHX", "Phoenix", 120.0, 440.0, "S3", true),
                stop("MCN", "McAllen", 740.0, 910.0, "S4?", false),
                stop("TUC", "Tucson", 260.0, 530.0, "S4?", false),
                stop("LBB", "Lubbock", 620.0, 220.0, "S4?", false),
                stop("YUM", "Yuma", 80.0, 550.0, "S4?", false),
                stop("TUL", "Tulsa", 960.0, 180.0, "S3?", false),
            ],
            vec![
                line(
                    "I-10",
                    RouteTier::T1,
                    LineStatus::Passing,
                    &["PHX", "TUC", "ELP", "SAT"],
                ),
                line(
                    "I-35",
                    RouteTier::T1,
                    LineStatus::Passing,
                    &["DFW", "SAT", "LRD"],
                ),
                line("I-20", RouteTier::T2, LineStatus::Passing, &["ELP", "DFW"]),
                line(
                    "I-2",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["LRD", "MCN", "CRP"],
                ),
                line("I-19", RouteTier::T3, LineStatus::Blocked, &["PHX", "TUC"]),
                line("I-27", RouteTier::T3, LineStatus::Blocked, &["DFW", "LBB"]),
                line("I-17", RouteTier::T3, LineStatus::Blocked, &["PHX", "ELP"]),
                line(
                    "I-8",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["YUM", "TUC", "ELP"],
                ),
                line("US-75", RouteTier::T3, LineStatus::Blocked, &["DFW", "TUL"]),
                line_project(
                    "US-77",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["SAT", "CRP"],
                    (16.0, 10.0),
                    (28.0, 18.0),
                    LabelAnchor::Middle,
                    SpecialLane::DedicatedRelay,
                ),
            ],
        ),
        "T3ZMOUNTAINWEST" => (
            vec![
                stop("SLC", "Salt Lake City", 760.0, 620.0, "S2", true),
                stop("DEN", "Denver", 1160.0, 640.0, "S2", true),
                stop("CHY", "Cheyenne", 1100.0, 500.0, "S3", true),
                stop("BIL", "Billings", 1060.0, 260.0, "S2", true),
                stop("LV", "Las Vegas", 520.0, 820.0, "S3", true),
                stop("SAC", "Sacramento", 260.0, 660.0, "S2", true),
                stop("TWF", "Twin Falls", 700.0, 470.0, "S4?", false),
                stop("YAK", "Yakima", 470.0, 260.0, "S4?", false),
                stop("ABR", "Aberdeen", 1320.0, 300.0, "S4?", false),
                stop("DUR", "Durango", 980.0, 820.0, "S4?", false),
                stop("FLG", "Flagstaff", 690.0, 820.0, "S4?", false),
            ],
            vec![
                line(
                    "I-80",
                    RouteTier::T1,
                    LineStatus::Passing,
                    &["SAC", "SLC", "CHY"],
                ),
                line("I-25", RouteTier::T1, LineStatus::Passing, &["DEN", "CHY"]),
                line("I-15", RouteTier::T1, LineStatus::Passing, &["LV", "SLC"]),
                line("I-70", RouteTier::T2, LineStatus::Passing, &["SLC", "DEN"]),
                line(
                    "I-82",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["SAC", "YAK", "TWF"],
                ),
                line(
                    "I-86",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["SLC", "TWF", "BIL"],
                ),
                line("US-12", RouteTier::T3, LineStatus::Blocked, &["BIL", "ABR"]),
                line(
                    "US-20",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["SAC", "TWF", "CHY"],
                ),
                line(
                    "US-160",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["LV", "FLG", "DUR", "DEN"],
                ),
                line_project(
                    "US-85",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["DEN", "CHY", "BIL"],
                    (20.0, 0.0),
                    (30.0, -18.0),
                    LabelAnchor::Middle,
                    SpecialLane::HardeningPilot,
                ),
            ],
        ),
        _ => (
            vec![
                stop("MEM", "Memphis", 760.0, 260.0, "S2", true),
                stop("LR", "Little Rock", 720.0, 430.0, "S2", true),
                stop("JOP", "Joplin", 560.0, 250.0, "S3", true),
                stop("KC", "Kansas City", 520.0, 120.0, "S2", true),
                stop("STL", "St. Louis", 900.0, 140.0, "S2", true),
                stop("TUL", "Tulsa", 520.0, 390.0, "S3", true),
                stop("GNV", "Greenville MS", 840.0, 560.0, "S4?", false),
                stop("JBR", "Jonesboro", 770.0, 340.0, "S4?", false),
                stop("FSM", "Fort Smith", 510.0, 520.0, "S4?", false),
                stop("SGF", "Springfield", 610.0, 280.0, "S4?", false),
            ],
            vec![
                line("I-55", RouteTier::T1, LineStatus::Passing, &["STL", "MEM"]),
                line(
                    "I-40",
                    RouteTier::T1,
                    LineStatus::Passing,
                    &["MEM", "LR", "FSM"],
                ),
                line(
                    "I-44",
                    RouteTier::T2,
                    LineStatus::Passing,
                    &["STL", "SGF", "TUL"],
                ),
                line(
                    "I-49",
                    RouteTier::T2,
                    LineStatus::Passing,
                    &["KC", "JOP", "FSM"],
                ),
                line(
                    "US-49",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["MEM", "GNV", "LR"],
                ),
                line(
                    "US-51",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["STL", "MEM", "GNV"],
                ),
                line("US-61", RouteTier::T3, LineStatus::Blocked, &["STL", "MEM"]),
                line(
                    "US-63",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["KC", "JBR", "MEM"],
                ),
                line(
                    "US-65",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["KC", "SGF", "LR"],
                ),
                line(
                    "US-69",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["KC", "TUL", "FSM"],
                ),
                line_project(
                    "US-412",
                    RouteTier::T3,
                    LineStatus::Blocked,
                    &["JOP", "TUL", "LR"],
                    (14.0, 14.0),
                    (30.0, 20.0),
                    LabelAnchor::Middle,
                    SpecialLane::RestitchCandidate,
                ),
            ],
        ),
    }
}

fn stop(
    id: &'static str,
    label: &'static str,
    x: f64,
    y: f64,
    class_name: &'static str,
    transfer: bool,
) -> Stop {
    Stop {
        id,
        label,
        x,
        y,
        class_name,
        transfer,
        label_dir: LabelDir::Right,
    }
}

fn stop_dir(
    id: &'static str,
    label: &'static str,
    x: f64,
    y: f64,
    class_name: &'static str,
    transfer: bool,
    label_dir: LabelDir,
) -> Stop {
    Stop {
        id,
        label,
        x,
        y,
        class_name,
        transfer,
        label_dir,
    }
}

fn line(
    route: &'static str,
    tier: RouteTier,
    status: LineStatus,
    stop_ids: &'static [&'static str],
) -> Line {
    Line {
        route,
        tier,
        status,
        stop_ids,
        lane_shift: (0.0, 0.0),
        badge_offset: (0.0, 0.0),
        label_anchor: LabelAnchor::Start,
        special_lane: SpecialLane::None,
    }
}

fn line_style(
    route: &'static str,
    tier: RouteTier,
    status: LineStatus,
    stop_ids: &'static [&'static str],
    lane_shift: (f64, f64),
    badge_offset: (f64, f64),
    label_anchor: LabelAnchor,
) -> Line {
    Line {
        route,
        tier,
        status,
        stop_ids,
        lane_shift,
        badge_offset,
        label_anchor,
        special_lane: SpecialLane::None,
    }
}

fn line_project(
    route: &'static str,
    tier: RouteTier,
    status: LineStatus,
    stop_ids: &'static [&'static str],
    lane_shift: (f64, f64),
    badge_offset: (f64, f64),
    label_anchor: LabelAnchor,
    special_lane: SpecialLane,
) -> Line {
    Line {
        route,
        tier,
        status,
        stop_ids,
        lane_shift,
        badge_offset,
        label_anchor,
        special_lane,
    }
}

fn render_zone(zone: &T3Zone, stops: &[Stop], lines: &[Line]) -> String {
    let mut s = String::new();
    s += &format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {W} {H}\" width=\"{W}\" height=\"{H}\">\n\
         <rect width=\"{W}\" height=\"{H}\" fill=\"#0f1623\"/>\n"
    );

    draw_grid(&mut s);
    draw_lines(&mut s, stops, lines);
    draw_stops(&mut s, stops);
    draw_title(&mut s, zone);
    draw_legend(&mut s);

    s += "</svg>\n";
    s
}

fn render_board_csv(zone: &T3Zone, stops: &[Stop], lines: &[Line]) -> String {
    let mut csv = String::from(
        "row_type,zone_id,zone_title,id,label,class,tier,status,route,stop_ids,special_lane,project_id,project_name,cost_band,effect,evidence_status\n",
    );
    for stop in stops {
        push_csv_row(
            &mut csv,
            &[
                "stop",
                zone.id,
                zone.title,
                stop.id,
                stop.label,
                stop.class_name,
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
                "",
            ],
        );
    }
    for line in lines {
        let stop_ids = line.stop_ids.join(";");
        push_csv_row(
            &mut csv,
            &[
                "line",
                zone.id,
                zone.title,
                line.route,
                line.route,
                "",
                line.tier.as_str(),
                line.status.as_str(),
                line.route,
                &stop_ids,
                line.special_lane.as_str(),
                line.special_lane.project_id(),
                line.special_lane.project_name(),
                line.special_lane.cost_band(),
                line.special_lane.effect(),
                line.special_lane.evidence_status(),
            ],
        );
    }
    csv
}

fn push_csv_row(csv: &mut String, fields: &[&str]) {
    let row = fields
        .iter()
        .map(|field| csv_escape(field))
        .collect::<Vec<_>>()
        .join(",");
    csv.push_str(&row);
    csv.push('\n');
}

fn csv_escape(field: &str) -> String {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        format!("\"{}\"", field.replace('"', "\"\""))
    } else {
        field.to_string()
    }
}

fn point(stops: &[Stop], id: &str) -> (f64, f64) {
    let stop = stops
        .iter()
        .find(|stop| stop.id == id)
        .unwrap_or_else(|| panic!("missing T3 schematic stop {id}"));
    (stop.x, stop.y)
}

fn same_point(a: (f64, f64), b: (f64, f64)) -> bool {
    (a.0 - b.0).abs() < 0.001 && (a.1 - b.1).abs() < 0.001
}

fn octilinear_path(points: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut routed = Vec::new();
    for window in points.windows(2) {
        let segment = octilinear_segment(window[0], window[1]);
        if routed.is_empty() {
            routed.extend(segment);
        } else {
            routed.extend(segment.into_iter().skip(1));
        }
    }
    if routed.is_empty() && !points.is_empty() {
        routed.push(points[0]);
    }
    routed
}

fn octilinear_segment(a: (f64, f64), b: (f64, f64)) -> Vec<(f64, f64)> {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    if dx.abs() < 0.001 || dy.abs() < 0.001 || (dx.abs() - dy.abs()).abs() < 0.001 {
        return vec![a, b];
    }

    let sx = dx.signum();
    let sy = dy.signum();
    let ax = dx.abs();
    let ay = dy.abs();
    let bend = if ax > ay {
        (b.0 - sx * ay, a.1)
    } else {
        (a.0, b.1 - sy * ax)
    };
    if same_point(a, bend) || same_point(bend, b) {
        vec![a, b]
    } else {
        vec![a, bend, b]
    }
}

fn apply_lane_shift(points: Vec<(f64, f64)>, shift: (f64, f64)) -> Vec<(f64, f64)> {
    if points.len() <= 2 || same_point(shift, (0.0, 0.0)) {
        return points;
    }
    let last = points.len() - 1;
    let shifted = points
        .into_iter()
        .enumerate()
        .map(|(idx, point)| {
            if idx == 0 || idx == last {
                point
            } else {
                (point.0 + shift.0, point.1 + shift.1)
            }
        })
        .collect::<Vec<_>>();
    octilinear_path(&shifted)
}

fn svg_path(points: &[(f64, f64)]) -> String {
    points
        .iter()
        .enumerate()
        .map(|(idx, (x, y))| {
            if idx == 0 {
                format!("M {x:.1} {y:.1}")
            } else {
                format!("L {x:.1} {y:.1}")
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn draw_grid(s: &mut String) {
    for x in (120..1681).step_by(120) {
        s.push_str(&format!(
            "<path d=\"M {x} 150 L {x} 760\" stroke=\"#1e293b\" stroke-width=\"1\" opacity=\"0.34\"/>\n"
        ));
    }
    for y in (180..761).step_by(100) {
        s.push_str(&format!(
            "<path d=\"M 90 {y} L 1700 {y}\" stroke=\"#1e293b\" stroke-width=\"1\" opacity=\"0.34\"/>\n"
        ));
    }
}

fn draw_lines(s: &mut String, stops: &[Stop], lines: &[Line]) {
    for tier in [RouteTier::T1, RouteTier::T2, RouteTier::T3] {
        draw_lines_for_tier(s, stops, lines, tier);
    }
}

fn draw_lines_for_tier(s: &mut String, stops: &[Stop], lines: &[Line], tier: RouteTier) {
    for line in lines {
        if line.tier != tier {
            continue;
        }
        let raw_pts = line
            .stop_ids
            .iter()
            .map(|id| point(stops, id))
            .collect::<Vec<_>>();
        if raw_pts.len() < 2 {
            continue;
        }
        let pts = apply_lane_shift(octilinear_path(&raw_pts), line.lane_shift);
        let d = svg_path(&pts);
        let color = match line.tier {
            RouteTier::T1 => "#38bdf8",
            RouteTier::T2 => "#22c55e",
            RouteTier::T3 => match line.status {
                LineStatus::Passing => "#eab308",
                LineStatus::Blocked => "#94a3b8",
            },
            RouteTier::T4 => "#64748b",
        };
        let (stroke_width, opacity, label_fill) = match line.tier {
            RouteTier::T1 => (9.0, 0.42, "#bfdbfe"),
            RouteTier::T2 => (7.0, 0.42, "#bbf7d0"),
            RouteTier::T3 => {
                if line.status == LineStatus::Passing {
                    (6.0, 0.96, "#f8fafc")
                } else {
                    (6.0, 0.70, "#f8fafc")
                }
            }
            RouteTier::T4 => (4.0, 0.52, "#e2e8f0"),
        };
        s.push_str(&format!(
            "<path data-route=\"{}\" data-tier=\"{}\" data-status=\"{}\" data-special-lane=\"{}\" data-role=\"halo\" d=\"{d}\" stroke=\"#020617\" stroke-width=\"{:.1}\" fill=\"none\" opacity=\"0.72\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n\
             <path data-route=\"{}\" data-tier=\"{}\" data-status=\"{}\" data-special-lane=\"{}\" data-role=\"line\" d=\"{d}\" stroke=\"{color}\" stroke-width=\"{stroke_width:.1}\" fill=\"none\" opacity=\"{opacity}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n",
            line.route,
            line.tier.as_str(),
            line.status.as_str(),
            line.special_lane.as_str(),
            stroke_width + 5.0,
            line.route,
            line.tier.as_str(),
            line.status.as_str(),
            line.special_lane.as_str(),
        ));
        if line.special_lane != SpecialLane::None {
            s.push_str(&format!(
                "<path data-route=\"{}\" data-tier=\"{}\" data-status=\"{}\" data-special-lane=\"{}\" data-project-id=\"{}\" data-project-name=\"{}\" data-cost-band=\"{}\" data-effect=\"{}\" data-evidence-status=\"{}\" data-role=\"special-lane\" d=\"{d}\" stroke=\"{}\" stroke-width=\"2.6\" fill=\"none\" opacity=\"0.96\" stroke-linecap=\"round\" stroke-linejoin=\"round\" stroke-dasharray=\"10 8\"/>\n",
                line.route,
                line.tier.as_str(),
                line.status.as_str(),
                line.special_lane.as_str(),
                line.special_lane.project_id(),
                line.special_lane.project_name(),
                line.special_lane.cost_band(),
                line.special_lane.effect(),
                line.special_lane.evidence_status(),
                line.special_lane.color()
            ));
        }
        let badge_index = match line.label_anchor {
            LabelAnchor::Start => pts.len().min(2) - 1,
            LabelAnchor::End => pts.len().saturating_sub(2),
            LabelAnchor::Middle => pts.len() / 2,
        };
        let (badge_x, badge_y) = pts[badge_index];
        let lx = badge_x + line.badge_offset.0;
        let ly = badge_y + line.badge_offset.1;
        let label_opacity = if line.tier == RouteTier::T3 {
            1.0
        } else {
            0.72
        };
        s.push_str(&format!(
            "<rect x=\"{:.1}\" y=\"{:.1}\" width=\"70\" height=\"28\" rx=\"5\" fill=\"#0f1623\" fill-opacity=\"0.88\" stroke=\"{color}\" stroke-width=\"1.5\" opacity=\"{label_opacity}\"/>\n\
             <text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"900\" fill=\"{label_fill}\" opacity=\"{label_opacity}\" text-anchor=\"middle\">{}</text>\n",
            lx - 35.0,
            ly - 14.0,
            lx,
            ly + 4.0,
            line.route
        ));
    }
}

fn draw_stops(s: &mut String, stops: &[Stop]) {
    for stop in stops {
        if stop.transfer {
            s.push_str(&format!(
                "<circle data-stop=\"{}\" data-class=\"{}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"14\" fill=\"#f8fafc\"/>\n\
                 <circle data-stop=\"{}\" data-class=\"{}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"9\" fill=\"#0f1623\" stroke=\"#38bdf8\" stroke-width=\"3\"/>\n",
                stop.id, stop.class_name, stop.x, stop.y, stop.id, stop.class_name, stop.x, stop.y
            ));
        } else {
            s.push_str(&format!(
                "<circle data-stop=\"{}\" data-class=\"{}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"8\" fill=\"#0f1623\" stroke=\"#c084fc\" stroke-width=\"2.5\"/>\n",
                stop.id, stop.class_name, stop.x, stop.y
            ));
        }
        let (lx, ly, anchor) = match stop.label_dir {
            LabelDir::Right => (stop.x + 16.0, stop.y + 4.0, "start"),
            LabelDir::Left => (stop.x - 16.0, stop.y + 4.0, "end"),
            LabelDir::Up => (stop.x, stop.y - 18.0, "middle"),
            LabelDir::Down => (stop.x, stop.y + 24.0, "middle"),
        };
        s.push_str(&format!(
            "<text x=\"{lx:.1}\" y=\"{ly:.1}\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"#dbeafe\" text-anchor=\"{anchor}\">{} ({})</text>\n",
            stop.label,
            stop.class_name
        ));
    }
}

fn draw_title(s: &mut String, zone: &T3Zone) {
    let status_color = if zone.status == "ready seed" {
        "#22c55e"
    } else {
        "#f59e0b"
    };
    s.push_str(&format!(
        "<rect x=\"32\" y=\"28\" width=\"820\" height=\"116\" rx=\"8\" fill=\"#0f1623\" fill-opacity=\"0.95\" stroke=\"#334155\"/>\n\
         <text x=\"56\" y=\"66\" font-family=\"'Helvetica Neue',Arial,sans-serif\" font-size=\"27\" font-weight=\"900\" fill=\"#f8fafc\">T3 Beck Zone · {}</text>\n\
         <text x=\"56\" y=\"94\" font-family=\"Arial,sans-serif\" font-size=\"13\" fill=\"#94a3b8\">Schematic regional feeder map · stops define bends, endpoints, and transfers</text>\n\
         <text x=\"56\" y=\"120\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"900\" fill=\"{status_color}\">{}</text>\n",
        zone.title,
        zone.status.to_uppercase()
    ));
    s.push_str(&format!(
        "<rect x=\"32\" y=\"810\" width=\"1188\" height=\"126\" rx=\"8\" fill=\"#0f1623\" fill-opacity=\"0.93\" stroke=\"#334155\"/>\n\
         <text x=\"58\" y=\"844\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"900\" fill=\"#f8fafc\">SCHEMATIC GOAL</text>\n\
         <text x=\"58\" y=\"870\" font-family=\"Arial,sans-serif\" font-size=\"13\" fill=\"#cbd5e1\">{}</text>\n\
         <text x=\"58\" y=\"906\" font-family=\"Arial,sans-serif\" font-size=\"12\" font-weight=\"900\" fill=\"#f8fafc\">NEXT STOP-SELECTION STEP</text>\n\
         <text x=\"58\" y=\"930\" font-family=\"Arial,sans-serif\" font-size=\"13\" fill=\"#cbd5e1\">{}</text>\n",
        zone.goal,
        zone.next_step
    ));
}

fn draw_legend(s: &mut String) {
    s.push_str(
        "<rect x=\"1260\" y=\"790\" width=\"500\" height=\"150\" rx=\"8\" fill=\"#0f1623\" fill-opacity=\"0.93\" stroke=\"#334155\"/>\n\
         <path d=\"M 1290 806 L 1350 806\" stroke=\"#facc15\" stroke-width=\"2.6\" stroke-dasharray=\"10 8\" stroke-linecap=\"round\"/>\n\
         <text x=\"1370\" y=\"811\" font-family=\"Arial,sans-serif\" font-size=\"12\" fill=\"#e2e8f0\">Special lane / project overlay</text>\n\
         <path d=\"M 1290 834 L 1350 834\" stroke=\"#38bdf8\" stroke-width=\"9\" opacity=\"0.42\" stroke-linecap=\"round\"/>\n\
         <text x=\"1370\" y=\"839\" font-family=\"Arial,sans-serif\" font-size=\"12\" fill=\"#e2e8f0\">Local T1 trunk context</text>\n\
         <path d=\"M 1290 862 L 1350 862\" stroke=\"#22c55e\" stroke-width=\"7\" opacity=\"0.42\" stroke-linecap=\"round\"/>\n\
         <text x=\"1370\" y=\"867\" font-family=\"Arial,sans-serif\" font-size=\"12\" fill=\"#e2e8f0\">Local T2 connector context</text>\n\
         <path d=\"M 1290 890 L 1350 890\" stroke=\"#eab308\" stroke-width=\"6\" stroke-linecap=\"round\"/>\n\
         <text x=\"1370\" y=\"895\" font-family=\"Arial,sans-serif\" font-size=\"12\" fill=\"#e2e8f0\">Passing T3 seed chain</text>\n\
         <path d=\"M 1290 918 L 1350 918\" stroke=\"#94a3b8\" stroke-width=\"6\" opacity=\"0.70\" stroke-linecap=\"round\"/>\n\
         <text x=\"1370\" y=\"923\" font-family=\"Arial,sans-serif\" font-size=\"12\" fill=\"#e2e8f0\">Blocked / candidate T3 feeder</text>\n",
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use route_network::HighwayGraph;
    use std::collections::HashMap;

    fn segment_is_octilinear(a: (f64, f64), b: (f64, f64)) -> bool {
        let dx = (b.0 - a.0).abs();
        let dy = (b.1 - a.1).abs();
        dx < 0.001 || dy < 0.001 || (dx - dy).abs() < 0.001
    }

    #[test]
    fn t3_zone_svg_exposes_game_binding_metadata() {
        let graph = HighwayGraph::new();
        let svg =
            build_t3_zone_svg(&graph, "T3Z-GREAT-LAKES", &[], &HashMap::new()).expect("zone svg");

        assert!(svg.contains("data-route=\"I-71\""));
        assert!(svg.contains("data-tier=\"T3\""));
        assert!(svg.contains("data-status=\"passing\""));
        assert!(svg.contains("data-status=\"blocked\""));
        assert!(svg.contains("data-special-lane=\"dedicated-relay\""));
        assert!(svg.contains("data-project-id=\"managed-lane-pilot\""));
        assert!(svg.contains("data-cost-band=\"corridor-specific\""));
        assert!(svg.contains("data-effect=\"raises relay throughput and schedule reliability\""));
        assert!(svg.contains("data-evidence-status=\"heuristic-held\""));
        assert!(svg.contains("data-role=\"special-lane\""));
        assert!(svg.contains("data-stop=\"CIN\""));
        assert!(svg.contains("data-class=\"S2\""));
    }

    #[test]
    fn each_t3_zone_has_a_game_upgrade_project_overlay() {
        for zone in zones() {
            let (_stops, lines) = zone_schematic(zone.id);
            assert!(
                lines
                    .iter()
                    .any(|line| line.special_lane != SpecialLane::None),
                "{} should expose at least one game upgrade overlay",
                zone.id
            );
        }
    }

    #[test]
    fn t3_zone_board_csv_exports_stops_lines_and_project_metadata() {
        let csv = build_t3_zone_board_csv("T3Z-GREAT-LAKES").expect("board csv");

        assert!(csv.starts_with("row_type,zone_id,zone_title"));
        assert!(csv.contains("stop,T3ZGREATLAKES,Great Lakes / Ohio Valley,CIN,Cincinnati,S2"));
        assert!(csv.contains("line,T3ZGREATLAKES,Great Lakes / Ohio Valley,I-71,I-71,,T3,passing,I-71,CIN;COL;CLE,dedicated-relay,managed-lane-pilot,Dedicated relay lane,corridor-specific"));
        assert!(csv.contains("raises relay throughput and schedule reliability"));
        assert!(csv.contains("heuristic-held"));
    }

    #[test]
    fn t3_zone_lines_route_octilinearly() {
        let (stops, lines) = zone_schematic("T3ZGREATLAKES");
        for line in lines {
            let raw_pts = line
                .stop_ids
                .iter()
                .map(|id| point(&stops, id))
                .collect::<Vec<_>>();
            let pts = apply_lane_shift(octilinear_path(&raw_pts), line.lane_shift);
            for segment in pts.windows(2) {
                assert!(
                    segment_is_octilinear(segment[0], segment[1]),
                    "{} has non-octilinear segment {:?}",
                    line.route,
                    segment
                );
            }
        }
    }
}
