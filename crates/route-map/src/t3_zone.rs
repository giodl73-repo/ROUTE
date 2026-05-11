use std::collections::HashMap;

use anyhow::{anyhow, Result};
use route_network::HighwayGraph;

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
}

struct Line {
    route: &'static str,
    tier: LineTier,
    status: LineStatus,
    stop_ids: &'static [&'static str],
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LineTier {
    T1,
    T2,
    T3,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum LineStatus {
    Passing,
    Blocked,
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

fn zone_schematic(zone_id: &str) -> (Vec<Stop>, Vec<Line>) {
    match zone_id {
        "T3ZGREATLAKES" => (
            vec![
                stop("CIN", "Cincinnati", 420.0, 760.0, "S2", true),
                stop("COL", "Columbus", 640.0, 610.0, "S2", true),
                stop("TOL", "Toledo", 700.0, 360.0, "S2", true),
                stop("IND", "Indianapolis", 300.0, 620.0, "S2", true),
                stop("HAR", "Harrisburg", 1320.0, 560.0, "S2", true),
                stop("PIT", "Pittsburgh", 1030.0, 650.0, "S3", true),
                stop("CLE", "Cleveland", 820.0, 420.0, "S4?", false),
                stop("DET", "Detroit", 720.0, 230.0, "S4?", false),
                stop("GRR", "Grand Rapids", 500.0, 230.0, "S4?", false),
                stop("RFD", "Rockford", 220.0, 360.0, "S4?", false),
                stop("YRK", "York", 1380.0, 670.0, "S4?", false),
                stop("BAL", "Baltimore", 1480.0, 760.0, "S2", true),
            ],
            vec![
                line(
                    "I-75",
                    LineTier::T1,
                    LineStatus::Passing,
                    &["CIN", "TOL", "DET"],
                ),
                line(
                    "I-80",
                    LineTier::T1,
                    LineStatus::Passing,
                    &["RFD", "TOL", "CLE"],
                ),
                line("I-76", LineTier::T1, LineStatus::Passing, &["PIT", "HAR"]),
                line(
                    "I-70",
                    LineTier::T2,
                    LineStatus::Passing,
                    &["IND", "COL", "PIT", "HAR"],
                ),
                line("I-65", LineTier::T2, LineStatus::Passing, &["IND", "CIN"]),
                line(
                    "I-71",
                    LineTier::T3,
                    LineStatus::Passing,
                    &["CIN", "COL", "CLE"],
                ),
                line(
                    "I-74",
                    LineTier::T3,
                    LineStatus::Passing,
                    &["CIN", "IND", "RFD"],
                ),
                line(
                    "I-83",
                    LineTier::T3,
                    LineStatus::Passing,
                    &["BAL", "YRK", "HAR"],
                ),
                line("I-79", LineTier::T3, LineStatus::Blocked, &["PIT", "CLE"]),
                line("I-96", LineTier::T3, LineStatus::Blocked, &["DET", "GRR"]),
                line("I-88", LineTier::T3, LineStatus::Blocked, &["RFD", "CLE"]),
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
                line("I-75", LineTier::T1, LineStatus::Passing, &["KNX", "ATL"]),
                line("I-95", LineTier::T1, LineStatus::Passing, &["SAV", "JAX"]),
                line("I-81", LineTier::T1, LineStatus::Passing, &["HAR", "KNX"]),
                line("I-20", LineTier::T2, LineStatus::Passing, &["BHM", "ATL"]),
                line("I-16", LineTier::T3, LineStatus::Blocked, &["ATL", "SAV"]),
                line(
                    "I-26",
                    LineTier::T3,
                    LineStatus::Blocked,
                    &["KNX", "AVL", "SAV"],
                ),
                line("I-4", LineTier::T3, LineStatus::Blocked, &["JAX", "ORL"]),
                line("I-68", LineTier::T3, LineStatus::Blocked, &["HAR", "CUM"]),
                line("I-99", LineTier::T3, LineStatus::Blocked, &["HAR", "SC"]),
                line("US-220", LineTier::T3, LineStatus::Blocked, &["CUM", "AVL"]),
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
                    LineTier::T1,
                    LineStatus::Passing,
                    &["PHX", "TUC", "ELP", "SAT"],
                ),
                line(
                    "I-35",
                    LineTier::T1,
                    LineStatus::Passing,
                    &["DFW", "SAT", "LRD"],
                ),
                line("I-20", LineTier::T2, LineStatus::Passing, &["ELP", "DFW"]),
                line(
                    "I-2",
                    LineTier::T3,
                    LineStatus::Blocked,
                    &["LRD", "MCN", "CRP"],
                ),
                line("I-19", LineTier::T3, LineStatus::Blocked, &["PHX", "TUC"]),
                line("I-27", LineTier::T3, LineStatus::Blocked, &["DFW", "LBB"]),
                line("I-17", LineTier::T3, LineStatus::Blocked, &["PHX", "ELP"]),
                line(
                    "I-8",
                    LineTier::T3,
                    LineStatus::Blocked,
                    &["YUM", "TUC", "ELP"],
                ),
                line("US-75", LineTier::T3, LineStatus::Blocked, &["DFW", "TUL"]),
                line("US-77", LineTier::T3, LineStatus::Blocked, &["SAT", "CRP"]),
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
                    LineTier::T1,
                    LineStatus::Passing,
                    &["SAC", "SLC", "CHY"],
                ),
                line("I-25", LineTier::T1, LineStatus::Passing, &["DEN", "CHY"]),
                line("I-15", LineTier::T1, LineStatus::Passing, &["LV", "SLC"]),
                line("I-70", LineTier::T2, LineStatus::Passing, &["SLC", "DEN"]),
                line(
                    "I-82",
                    LineTier::T3,
                    LineStatus::Blocked,
                    &["SAC", "YAK", "TWF"],
                ),
                line(
                    "I-86",
                    LineTier::T3,
                    LineStatus::Blocked,
                    &["SLC", "TWF", "BIL"],
                ),
                line("US-12", LineTier::T3, LineStatus::Blocked, &["BIL", "ABR"]),
                line(
                    "US-20",
                    LineTier::T3,
                    LineStatus::Blocked,
                    &["SAC", "TWF", "CHY"],
                ),
                line(
                    "US-160",
                    LineTier::T3,
                    LineStatus::Blocked,
                    &["LV", "FLG", "DUR", "DEN"],
                ),
                line(
                    "US-85",
                    LineTier::T3,
                    LineStatus::Blocked,
                    &["DEN", "CHY", "BIL"],
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
                line("I-55", LineTier::T1, LineStatus::Passing, &["STL", "MEM"]),
                line(
                    "I-40",
                    LineTier::T1,
                    LineStatus::Passing,
                    &["MEM", "LR", "FSM"],
                ),
                line(
                    "I-44",
                    LineTier::T2,
                    LineStatus::Passing,
                    &["STL", "SGF", "TUL"],
                ),
                line(
                    "I-49",
                    LineTier::T2,
                    LineStatus::Passing,
                    &["KC", "JOP", "FSM"],
                ),
                line(
                    "US-49",
                    LineTier::T3,
                    LineStatus::Blocked,
                    &["MEM", "GNV", "LR"],
                ),
                line(
                    "US-51",
                    LineTier::T3,
                    LineStatus::Blocked,
                    &["STL", "MEM", "GNV"],
                ),
                line("US-61", LineTier::T3, LineStatus::Blocked, &["STL", "MEM"]),
                line(
                    "US-63",
                    LineTier::T3,
                    LineStatus::Blocked,
                    &["KC", "JBR", "MEM"],
                ),
                line(
                    "US-65",
                    LineTier::T3,
                    LineStatus::Blocked,
                    &["KC", "SGF", "LR"],
                ),
                line(
                    "US-69",
                    LineTier::T3,
                    LineStatus::Blocked,
                    &["KC", "TUL", "FSM"],
                ),
                line(
                    "US-412",
                    LineTier::T3,
                    LineStatus::Blocked,
                    &["JOP", "TUL", "LR"],
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
    }
}

fn line(
    route: &'static str,
    tier: LineTier,
    status: LineStatus,
    stop_ids: &'static [&'static str],
) -> Line {
    Line {
        route,
        tier,
        status,
        stop_ids,
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

fn point(stops: &[Stop], id: &str) -> (f64, f64) {
    let stop = stops
        .iter()
        .find(|stop| stop.id == id)
        .unwrap_or_else(|| panic!("missing T3 schematic stop {id}"));
    (stop.x, stop.y)
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
    for tier in [LineTier::T1, LineTier::T2, LineTier::T3] {
        draw_lines_for_tier(s, stops, lines, tier);
    }
}

fn draw_lines_for_tier(s: &mut String, stops: &[Stop], lines: &[Line], tier: LineTier) {
    for line in lines {
        if line.tier != tier {
            continue;
        }
        let pts = line
            .stop_ids
            .iter()
            .map(|id| point(stops, id))
            .collect::<Vec<_>>();
        if pts.len() < 2 {
            continue;
        }
        let d = pts
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
            .join(" ");
        let color = match line.tier {
            LineTier::T1 => "#38bdf8",
            LineTier::T2 => "#22c55e",
            LineTier::T3 => match line.status {
                LineStatus::Passing => "#eab308",
                LineStatus::Blocked => "#94a3b8",
            },
        };
        let (stroke_width, opacity, label_fill) = match line.tier {
            LineTier::T1 => (9.0, 0.42, "#bfdbfe"),
            LineTier::T2 => (7.0, 0.42, "#bbf7d0"),
            LineTier::T3 => {
                if line.status == LineStatus::Passing {
                    (6.0, 0.96, "#f8fafc")
                } else {
                    (6.0, 0.70, "#f8fafc")
                }
            }
        };
        s.push_str(&format!(
            "<path d=\"{d}\" stroke=\"#020617\" stroke-width=\"{:.1}\" fill=\"none\" opacity=\"0.72\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n\
             <path d=\"{d}\" stroke=\"{color}\" stroke-width=\"{stroke_width:.1}\" fill=\"none\" opacity=\"{opacity}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n",
            stroke_width + 5.0
        ));
        let (lx, ly) = pts[pts.len() / 2];
        let label_opacity = if line.tier == LineTier::T3 { 1.0 } else { 0.72 };
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
                "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"14\" fill=\"#f8fafc\"/>\n\
                 <circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"9\" fill=\"#0f1623\" stroke=\"#38bdf8\" stroke-width=\"3\"/>\n",
                stop.x, stop.y, stop.x, stop.y
            ));
        } else {
            s.push_str(&format!(
                "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"8\" fill=\"#0f1623\" stroke=\"#c084fc\" stroke-width=\"2.5\"/>\n",
                stop.x, stop.y
            ));
        }
        s.push_str(&format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" font-size=\"11\" font-weight=\"700\" fill=\"#dbeafe\">{} ({})</text>\n",
            stop.x + 16.0,
            stop.y + 4.0,
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
        "<rect x=\"1260\" y=\"810\" width=\"500\" height=\"126\" rx=\"8\" fill=\"#0f1623\" fill-opacity=\"0.93\" stroke=\"#334155\"/>\n\
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
