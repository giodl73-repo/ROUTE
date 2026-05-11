/// Beck-style schematic diagram of the I2.0 T1 relay network.
///
/// Inspired by Harry Beck's 1933 London Underground map:
/// - All lines run at 0°, 45°, or 90° only
/// - Geography is distorted for legibility (central area expanded)
/// - Color = line identity (primary navigation variable)
/// - Station markers at relay hubs and major intersections
/// - Minimal labels, all horizontal where possible
///
/// This is NOT a geographic map — it is a topological representation
/// of the relay network. The traveler's question is:
/// "Which corridor? Where do I change? How many hubs?"
use std::collections::{HashMap, HashSet};

use route_network::{RouteTier, StopNodeClass};

const W: f64 = 2400.0;
const H: f64 = 1350.0;

/// T1 corridor colors (same registry as megamap for consistency)
fn t1_line_color(corridor: &str) -> &'static str {
    match corridor {
        "I-5" => "#ef4444",
        "I-10" => "#f97316",
        "I-20" => "#fb7185",
        "I-35" => "#10b981",
        "I-40" => "#eab308",
        "I-69" => "#059669",
        "I-70" => "#f59e0b",
        "I-75" => "#06b6d4",
        "I-80" => "#3b82f6",
        "I-85" => "#22c55e",
        "I-90" => "#8b5cf6",
        "I-95" => "#f43f5e",
        _ => "#94a3b8",
    }
}

#[derive(Clone, Copy)]
enum LabelDir {
    Right,
    Left,
    Up,
    Down,
}

struct BeckStop {
    id: &'static str,
    label: &'static str,
    x: f64,
    y: f64,
    is_hub: bool,
    is_interchange: bool,
    draw: bool,
    lines: &'static [&'static str],
    label_dir: LabelDir,
}

impl BeckStop {
    fn point(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn is_transfer_point(&self) -> bool {
        self.is_hub || self.is_interchange || self.lines.len() >= 2
    }
}

#[derive(Debug, Clone)]
pub struct BeckStopCatalogRow {
    pub id: &'static str,
    pub label: &'static str,
    pub lat: f64,
    pub lon: f64,
    pub lines: Vec<&'static str>,
    pub stop_class: &'static str,
}

pub struct BeckT2DiagnosticRow {
    pub corridor: &'static str,
    pub trunk: &'static str,
    pub start_trunk: &'static str,
    pub end_trunk: &'static str,
    pub color_mode: &'static str,
    pub split_anchor: &'static str,
    pub split_anchor_offset_pct: f64,
    pub unstopped_t1_contact_count: usize,
    pub unstopped_t1_contacts: String,
    pub close_parallel_count: usize,
    pub close_parallel_corridors: String,
    pub service_label: &'static str,
    pub stop_count: usize,
    pub drawn_stop_count: usize,
    pub transfer_stop_count: usize,
    pub schematic_length_px: f64,
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub label_density_per_100px: f64,
    pub review_flag: &'static str,
}

/// A line segment on the Beck diagram.
/// Beck lines are sequences of waypoints connected at 0°/45°/90°.
struct LineSegment {
    corridor: &'static str,
    // Points defining the schematic path (Beck angles: horizontal/vertical/diagonal only)
    // These are LAYOUT coordinates in a Beck grid, not geographic
    waypoints: Vec<(f64, f64)>,
}

struct T2LineSegment {
    corridor: &'static str,
    trunk: &'static str,
    start_trunk: &'static str,
    end_trunk: &'static str,
    service_label: &'static str,
    badge: (f64, f64),
    label_anchor: &'static str,
    stop_ids: Vec<&'static str>,
    waypoints: Vec<(f64, f64)>,
    lane_shift: (f64, f64),
}

#[derive(Clone)]
struct SlaEdge {
    to: &'static str,
    route: &'static str,
    tier: RouteTier,
    miles: f64,
}

#[derive(Clone)]
struct SlaStep {
    from: &'static str,
    to: &'static str,
    route: &'static str,
    tier: RouteTier,
    miles: f64,
}

struct T2SplitAnchor {
    stop_id: &'static str,
    fraction: f64,
}

impl LineSegment {
    fn to_svg_path(&self) -> String {
        svg_path(&self.routed_waypoints())
    }

    fn routed_waypoints(&self) -> Vec<(f64, f64)> {
        octilinear_path(&self.waypoints)
    }
}

impl T2LineSegment {
    fn routed_waypoints(&self) -> Vec<(f64, f64)> {
        apply_lane_shift(octilinear_path(&self.waypoints), self.lane_shift)
    }

    fn is_split_color(&self) -> bool {
        self.start_trunk != self.end_trunk
    }
}

fn svg_path(points: &[(f64, f64)]) -> String {
    if points.is_empty() {
        return String::new();
    }
    let mut d = format!("M {:.1} {:.1}", points[0].0, points[0].1);
    for pt in &points[1..] {
        d += &format!(" L {:.1} {:.1}", pt.0, pt.1);
    }
    d
}

fn split_polyline_at_half(points: &[(f64, f64)]) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    split_polyline_at_distance(points, polyline_length(points) / 2.0)
}

fn split_polyline_at_distance(
    points: &[(f64, f64)],
    distance: f64,
) -> (Vec<(f64, f64)>, Vec<(f64, f64)>) {
    if points.len() <= 1 {
        return (points.to_vec(), Vec::new());
    }
    let target = distance.clamp(0.0, polyline_length(points));
    let mut walked = 0.0;
    let mut left = vec![points[0]];
    for index in 0..points.len() - 1 {
        let a = points[index];
        let b = points[index + 1];
        let segment_length = ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
        if walked + segment_length >= target && segment_length > 0.0 {
            let t = ((target - walked) / segment_length).clamp(0.0, 1.0);
            let midpoint = (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t);
            if !same_point(*left.last().unwrap_or(&a), midpoint) {
                left.push(midpoint);
            }
            let mut right = vec![midpoint];
            if !same_point(midpoint, b) {
                right.push(b);
            }
            right.extend(points.iter().skip(index + 2).copied());
            return (left, right);
        }
        if !same_point(*left.last().unwrap_or(&a), b) {
            left.push(b);
        }
        walked += segment_length;
    }
    (left, Vec::new())
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

fn same_point(a: (f64, f64), b: (f64, f64)) -> bool {
    (a.0 - b.0).abs() < 0.001 && (a.1 - b.1).abs() < 0.001
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

fn stop(
    id: &'static str,
    label: &'static str,
    x: f64,
    y: f64,
    is_hub: bool,
    is_interchange: bool,
    lines: &'static [&'static str],
    label_dir: LabelDir,
) -> BeckStop {
    BeckStop {
        id,
        label,
        x,
        y,
        is_hub,
        is_interchange,
        draw: true,
        lines,
        label_dir,
    }
}

fn minor_stop(
    id: &'static str,
    label: &'static str,
    x: f64,
    y: f64,
    lines: &'static [&'static str],
    label_dir: LabelDir,
) -> BeckStop {
    stop(id, label, x, y, false, false, lines, label_dir)
}

fn beck_stops() -> Vec<BeckStop> {
    let mut stops = vec![
        stop(
            "BLAINE",
            "Blaine/Vancouver",
            180.0,
            140.0,
            true,
            false,
            &["I-5"],
            LabelDir::Right,
        ),
        stop(
            "SEA",
            "Seattle",
            180.0,
            220.0,
            true,
            true,
            &["I-5", "I-90"],
            LabelDir::Right,
        ),
        stop(
            "POR",
            "Portland",
            180.0,
            480.0,
            false,
            false,
            &["I-5"],
            LabelDir::Right,
        ),
        stop(
            "SAC",
            "Sacramento",
            180.0,
            680.0,
            true,
            true,
            &["I-5", "I-80"],
            LabelDir::Left,
        ),
        minor_stop(
            "MEDFORD",
            "Medford",
            180.0,
            560.0,
            &["I-5"],
            LabelDir::Right,
        ),
        minor_stop("EUGENE", "Eugene", 180.0, 515.0, &["I-5"], LabelDir::Right),
        minor_stop(
            "REDDING",
            "Redding",
            180.0,
            620.0,
            &["I-5"],
            LabelDir::Right,
        ),
        minor_stop(
            "CENTRAL_VALLEY",
            "Central Valley",
            240.0,
            840.0,
            &["I-5"],
            LabelDir::Right,
        ),
        stop(
            "LA",
            "Los Angeles",
            240.0,
            1080.0,
            false,
            true,
            &["I-5", "I-10"],
            LabelDir::Down,
        ),
        stop(
            "SDTJ",
            "San Diego/Tijuana",
            240.0,
            1200.0,
            true,
            false,
            &["I-5"],
            LabelDir::Down,
        ),
        stop(
            "PHX",
            "Phoenix",
            420.0,
            1080.0,
            false,
            false,
            &["I-10"],
            LabelDir::Down,
        ),
        minor_stop(
            "PALM_SPRINGS",
            "Palm Springs",
            330.0,
            1080.0,
            &["I-10"],
            LabelDir::Up,
        ),
        minor_stop(
            "QUARTZSITE",
            "Quartzsite",
            380.0,
            1080.0,
            &["I-10"],
            LabelDir::Up,
        ),
        minor_stop("TUCSON", "Tucson", 490.0, 1080.0, &["I-10"], LabelDir::Up),
        minor_stop(
            "LORDSBURG",
            "Lordsburg",
            525.0,
            1080.0,
            &["I-10"],
            LabelDir::Up,
        ),
        minor_stop(
            "SHOW_LOW",
            "Show Low",
            455.0,
            990.0,
            &["US70"],
            LabelDir::Right,
        ),
        stop(
            "ELP",
            "El Paso",
            560.0,
            1080.0,
            true,
            true,
            &["I-10"],
            LabelDir::Down,
        ),
        stop(
            "SAT",
            "San Antonio",
            780.0,
            1080.0,
            true,
            true,
            &["I-35", "I-10"],
            LabelDir::Down,
        ),
        minor_stop("WACO", "Waco", 810.0, 1020.0, &["I-35"], LabelDir::Left),
        stop(
            "LRD",
            "Laredo",
            780.0,
            1200.0,
            true,
            false,
            &["I-35"],
            LabelDir::Down,
        ),
        stop(
            "HOU",
            "Houston*",
            900.0,
            1020.0,
            true,
            true,
            &["I-10", "I-69"],
            LabelDir::Down,
        ),
        minor_stop(
            "BEAUMONT",
            "Beaumont",
            1080.0,
            1020.0,
            &["I-10"],
            LabelDir::Down,
        ),
        minor_stop(
            "LAFAYETTE",
            "Lafayette",
            1140.0,
            1020.0,
            &["I-10", "US90"],
            LabelDir::Down,
        ),
        minor_stop(
            "NOLA",
            "New Orleans",
            1260.0,
            1020.0,
            &["I-10"],
            LabelDir::Down,
        ),
        minor_stop(
            "HATTIESBURG",
            "Hattiesburg",
            1260.0,
            960.0,
            &["I-59"],
            LabelDir::Left,
        ),
        minor_stop(
            "MERIDIAN",
            "Meridian",
            1320.0,
            960.0,
            &["I-59"],
            LabelDir::Up,
        ),
        minor_stop(
            "TALLAHASSEE",
            "Tallahassee",
            1620.0,
            1020.0,
            &["I-10"],
            LabelDir::Up,
        ),
        minor_stop(
            "PANAMA_CITY",
            "Panama City",
            1500.0,
            1020.0,
            &["I-10"],
            LabelDir::Up,
        ),
        stop(
            "JAX",
            "Jacksonville",
            1800.0,
            1080.0,
            true,
            true,
            &["I-10", "I-95"],
            LabelDir::Right,
        ),
        stop(
            "MIA",
            "Miami",
            1860.0,
            1200.0,
            true,
            false,
            &["I-95"],
            LabelDir::Right,
        ),
        minor_stop(
            "DAYTONA",
            "Daytona Beach",
            1830.0,
            1140.0,
            &["I-95"],
            LabelDir::Right,
        ),
        minor_stop(
            "FORT_PIERCE",
            "Fort Pierce",
            1845.0,
            1170.0,
            &["I-95"],
            LabelDir::Right,
        ),
        stop(
            "DAL",
            "Dallas",
            840.0,
            960.0,
            true,
            true,
            &["I-35", "I-20"],
            LabelDir::Left,
        ),
        minor_stop(
            "ABILENE",
            "Abilene",
            720.0,
            960.0,
            &["I-20", "US80", "US83"],
            LabelDir::Down,
        ),
        minor_stop(
            "SHV",
            "Shreveport",
            1020.0,
            960.0,
            &["I-20", "I-49"],
            LabelDir::Down,
        ),
        stop(
            "BHM_APPROACH",
            "Birmingham",
            1200.0,
            960.0,
            true,
            true,
            &["I-20", "I-22"],
            LabelDir::Up,
        ),
        stop(
            "ATL",
            "Atlanta",
            1440.0,
            960.0,
            true,
            true,
            &["I-20", "I-75", "I-85"],
            LabelDir::Right,
        ),
        minor_stop(
            "COLUMBIA",
            "Columbia",
            1620.0,
            960.0,
            &["I-20", "I-26", "I-77"],
            LabelDir::Up,
        ),
        stop(
            "FLO",
            "Florence",
            1800.0,
            960.0,
            false,
            true,
            &["I-20", "I-95"],
            LabelDir::Right,
        ),
        stop(
            "MIN",
            "Minneapolis",
            840.0,
            220.0,
            true,
            true,
            &["I-35", "I-90"],
            LabelDir::Down,
        ),
        minor_stop(
            "ST_CLOUD",
            "St. Cloud",
            810.0,
            290.0,
            &["I-29", "I-94"],
            LabelDir::Left,
        ),
        stop(
            "DSM",
            "Des Moines",
            840.0,
            480.0,
            false,
            true,
            &["I-35", "I-80"],
            LabelDir::Left,
        ),
        minor_stop(
            "MASON_CITY",
            "Mason City",
            840.0,
            350.0,
            &["I-35"],
            LabelDir::Right,
        ),
        minor_stop(
            "IOWA_CITY",
            "Iowa City",
            960.0,
            500.0,
            &["I-80"],
            LabelDir::Down,
        ),
        stop(
            "KC",
            "Kansas City",
            840.0,
            620.0,
            true,
            true,
            &["I-35", "I-70"],
            LabelDir::Right,
        ),
        minor_stop(
            "COLUMBIA_MO",
            "Columbia MO",
            900.0,
            650.0,
            &["I-70"],
            LabelDir::Down,
        ),
        stop(
            "OKC",
            "Oklahoma City",
            840.0,
            780.0,
            true,
            true,
            &["I-35", "I-40"],
            LabelDir::Right,
        ),
        stop(
            "BARSTOW",
            "Barstow",
            300.0,
            900.0,
            true,
            true,
            &["I-40", "I-15"],
            LabelDir::Right,
        ),
        minor_stop(
            "ABQ",
            "Albuquerque",
            420.0,
            900.0,
            &["I-40", "I-25"],
            LabelDir::Down,
        ),
        minor_stop("AMA", "Amarillo", 600.0, 900.0, &["I-40"], LabelDir::Down),
        minor_stop(
            "TUCUMCARI",
            "Tucumcari",
            520.0,
            900.0,
            &["I-40"],
            LabelDir::Down,
        ),
        minor_stop(
            "ELK_CITY",
            "Elk City",
            720.0,
            900.0,
            &["I-40"],
            LabelDir::Down,
        ),
        minor_stop(
            "WICHITA_FALLS",
            "Wichita Falls",
            720.0,
            840.0,
            &["US287"],
            LabelDir::Down,
        ),
        stop(
            "MEM",
            "Memphis",
            960.0,
            900.0,
            true,
            true,
            &["I-40", "I-69"],
            LabelDir::Down,
        ),
        minor_stop("TUPELO", "Tupelo", 1080.0, 930.0, &["I-22"], LabelDir::Down),
        stop(
            "NASHVILLE",
            "Nashville",
            1080.0,
            840.0,
            true,
            true,
            &["I-40", "I-24"],
            LabelDir::Up,
        ),
        stop(
            "KNX",
            "Knoxville",
            1260.0,
            840.0,
            false,
            true,
            &["I-40", "I-75"],
            LabelDir::Up,
        ),
        minor_stop(
            "ASHEVILLE",
            "Asheville",
            1440.0,
            840.0,
            &["I-40"],
            LabelDir::Up,
        ),
        stop(
            "CLT",
            "Charlotte",
            1620.0,
            840.0,
            true,
            true,
            &["I-85", "I-40"],
            LabelDir::Up,
        ),
        stop(
            "RALEIGH_APPROACH",
            "Raleigh",
            1740.0,
            960.0,
            true,
            true,
            &["I-40", "I-85", "I-95"],
            LabelDir::Up,
        ),
        stop(
            "INDY_APPROACH",
            "Indianapolis",
            1020.0,
            780.0,
            true,
            true,
            &["I-69", "I-70"],
            LabelDir::Right,
        ),
        minor_stop(
            "FORT_WAYNE",
            "Fort Wayne",
            1080.0,
            660.0,
            &["I-69"],
            LabelDir::Right,
        ),
        minor_stop(
            "CHI_APPROACH",
            "South Bend",
            1200.0,
            540.0,
            &["I-69"],
            LabelDir::Right,
        ),
        stop(
            "CHI",
            "Chicago",
            1200.0,
            480.0,
            true,
            true,
            &["I-80", "I-90", "I-69"],
            LabelDir::Down,
        ),
        stop(
            "TOL",
            "Toledo",
            1320.0,
            420.0,
            true,
            true,
            &["I-75", "I-80", "I-90"],
            LabelDir::Up,
        ),
        stop(
            "COLUMBUS",
            "Columbus",
            1320.0,
            540.0,
            true,
            true,
            &["I-70", "I-75"],
            LabelDir::Right,
        ),
        stop(
            "CINCINNATI",
            "Cincinnati",
            1320.0,
            620.0,
            true,
            true,
            &["I-75"],
            LabelDir::Right,
        ),
        minor_stop(
            "CHATTANOOGA",
            "Chattanooga",
            1380.0,
            840.0,
            &["I-75"],
            LabelDir::Right,
        ),
        minor_stop(
            "LEXINGTON",
            "Lexington",
            1320.0,
            730.0,
            &["I-75", "I-64"],
            LabelDir::Right,
        ),
        minor_stop(
            "MONTGOMERY",
            "Montgomery",
            1320.0,
            1020.0,
            &["I-65", "I-85", "US80"],
            LabelDir::Down,
        ),
        minor_stop(
            "GAINESVILLE",
            "Gainesville",
            1440.0,
            1080.0,
            &["I-75"],
            LabelDir::Left,
        ),
        stop(
            "TAMPA",
            "Tampa",
            1500.0,
            1150.0,
            false,
            true,
            &["I-75"],
            LabelDir::Left,
        ),
        minor_stop("RENO", "Reno", 240.0, 620.0, &["I-80"], LabelDir::Down),
        stop(
            "SLC",
            "Salt Lake",
            360.0,
            540.0,
            true,
            true,
            &["I-80", "I-70", "I-15"],
            LabelDir::Down,
        ),
        stop(
            "BOI",
            "Boise",
            300.0,
            430.0,
            true,
            true,
            &["I-84", "US95"],
            LabelDir::Up,
        ),
        minor_stop(
            "PENDLETON",
            "Pendleton",
            240.0,
            455.0,
            &["I-84"],
            LabelDir::Down,
        ),
        minor_stop(
            "TWIN_FALLS",
            "Twin Falls",
            330.0,
            485.0,
            &["I-84"],
            LabelDir::Right,
        ),
        minor_stop(
            "LEWISTON",
            "Lewiston",
            330.0,
            320.0,
            &["US95"],
            LabelDir::Left,
        ),
        minor_stop(
            "CHEYENNE",
            "Cheyenne",
            540.0,
            540.0,
            &["I-80", "I-25"],
            LabelDir::Up,
        ),
        minor_stop("CASPER", "Casper", 540.0, 380.0, &["I-25"], LabelDir::Right),
        minor_stop(
            "ROCK_SPRINGS",
            "Rock Springs",
            450.0,
            540.0,
            &["I-80"],
            LabelDir::Down,
        ),
        minor_stop(
            "RAWLINS",
            "Rawlins",
            495.0,
            540.0,
            &["I-80"],
            LabelDir::Down,
        ),
        stop(
            "OMAHA",
            "Omaha",
            720.0,
            540.0,
            true,
            true,
            &["I-80", "I-29"],
            LabelDir::Up,
        ),
        minor_stop("FARGO", "Fargo", 780.0, 360.0, &["I-29"], LabelDir::Left),
        minor_stop(
            "CLEVELAND",
            "Cleveland",
            1500.0,
            420.0,
            &["I-80", "I-77"],
            LabelDir::Down,
        ),
        minor_stop(
            "CLARION",
            "Clarion",
            1590.0,
            450.0,
            &["I-80"],
            LabelDir::Down,
        ),
        minor_stop(
            "PA_APPROACH",
            "Pennsylvania",
            1680.0,
            480.0,
            &["I-80"],
            LabelDir::Up,
        ),
        stop(
            "NYC",
            "New York",
            1920.0,
            600.0,
            false,
            true,
            &["I-95", "I-80"],
            LabelDir::Right,
        ),
        stop(
            "SPK",
            "Spokane",
            360.0,
            220.0,
            false,
            true,
            &["I-90", "US95"],
            LabelDir::Down,
        ),
        minor_stop(
            "MOSES_LAKE",
            "Moses Lake",
            285.0,
            220.0,
            &["I-90"],
            LabelDir::Up,
        ),
        stop(
            "BIL",
            "Billings*",
            540.0,
            220.0,
            true,
            true,
            &["I-90"],
            LabelDir::Up,
        ),
        minor_stop(
            "SHERIDAN",
            "Sheridan",
            540.0,
            300.0,
            &["I-25", "I-90"],
            LabelDir::Right,
        ),
        minor_stop(
            "GILLETTE",
            "Gillette",
            630.0,
            220.0,
            &["I-90"],
            LabelDir::Up,
        ),
        stop(
            "RAPID_CITY",
            "Rapid City",
            720.0,
            220.0,
            true,
            true,
            &["I-90", "US83"],
            LabelDir::Up,
        ),
        minor_stop(
            "CHAMBERLAIN",
            "Chamberlain",
            765.0,
            230.0,
            &["I-90"],
            LabelDir::Up,
        ),
        minor_stop(
            "SIOUX_FALLS",
            "Sioux Falls",
            810.0,
            240.0,
            &["I-90", "I-29"],
            LabelDir::Up,
        ),
        minor_stop(
            "WATERTOWN_SD",
            "Watertown SD",
            810.0,
            255.0,
            &["I-29"],
            LabelDir::Right,
        ),
        minor_stop(
            "MILWAUKEE",
            "Milwaukee",
            1020.0,
            300.0,
            &["I-90"],
            LabelDir::Up,
        ),
        minor_stop(
            "MADISON",
            "Madison",
            990.0,
            330.0,
            &["I-90"],
            LabelDir::Down,
        ),
        minor_stop(
            "EAU_CLAIRE",
            "Eau Claire",
            930.0,
            260.0,
            &["I-90"],
            LabelDir::Up,
        ),
        minor_stop(
            "CHI_NORTH",
            "North Chicago",
            1200.0,
            420.0,
            &["I-90"],
            LabelDir::Left,
        ),
        stop(
            "DET",
            "Detroit",
            1260.0,
            450.0,
            false,
            false,
            &["I-90"],
            LabelDir::Up,
        ),
        minor_stop("BUFFALO", "Buffalo", 1500.0, 300.0, &["I-90"], LabelDir::Up),
        minor_stop("ERIE", "Erie", 1440.0, 360.0, &["I-90"], LabelDir::Up),
        minor_stop(
            "SYRACUSE",
            "Syracuse",
            1590.0,
            300.0,
            &["I-90"],
            LabelDir::Up,
        ),
        minor_stop("ALBANY", "Albany", 1680.0, 300.0, &["I-90"], LabelDir::Up),
        stop(
            "BOS",
            "Boston",
            1920.0,
            360.0,
            true,
            true,
            &["I-90", "I-95"],
            LabelDir::Right,
        ),
        minor_stop(
            "NEW_HAVEN",
            "New Haven",
            1920.0,
            480.0,
            &["I-95"],
            LabelDir::Right,
        ),
        minor_stop(
            "PHILADELPHIA",
            "Philadelphia",
            1920.0,
            720.0,
            &["I-95", "I-76"],
            LabelDir::Right,
        ),
        stop(
            "DC",
            "Washington",
            1920.0,
            840.0,
            false,
            true,
            &["I-95", "I-70"],
            LabelDir::Left,
        ),
        stop(
            "RIC",
            "Richmond",
            1860.0,
            960.0,
            true,
            true,
            &["I-95", "I-85"],
            LabelDir::Left,
        ),
        stop(
            "BEN",
            "Benson",
            1800.0,
            1020.0,
            false,
            true,
            &["I-40", "I-95"],
            LabelDir::Left,
        ),
        stop(
            "WIC",
            "Wichita*",
            780.0,
            720.0,
            false,
            false,
            &["I-35"],
            LabelDir::Right,
        ),
        minor_stop(
            "FORT_SMITH",
            "Fort Smith",
            900.0,
            840.0,
            &["I-40", "I-49", "US70"],
            LabelDir::Right,
        ),
        minor_stop(
            "GREENVILLE",
            "Greenville",
            1500.0,
            900.0,
            &["I-85"],
            LabelDir::Up,
        ),
        minor_stop("MACON", "Macon", 1440.0, 1020.0, &["I-75"], LabelDir::Right),
        minor_stop(
            "TIFTON",
            "Tifton",
            1440.0,
            1050.0,
            &["I-75"],
            LabelDir::Left,
        ),
        minor_stop(
            "GREENSBORO",
            "Greensboro",
            1740.0,
            900.0,
            &["I-85"],
            LabelDir::Down,
        ),
        minor_stop(
            "MOBILE",
            "Mobile",
            1260.0,
            1020.0,
            &["I-10", "I-65"],
            LabelDir::Down,
        ),
        stop(
            "LOUISVILLE",
            "Louisville",
            1260.0,
            660.0,
            true,
            true,
            &["I-65", "I-70"],
            LabelDir::Left,
        ),
        stop(
            "LITTLE_ROCK",
            "Little Rock",
            930.0,
            900.0,
            true,
            true,
            &["I-30", "I-40", "US70"],
            LabelDir::Up,
        ),
        minor_stop(
            "LAS_VEGAS",
            "Las Vegas",
            300.0,
            960.0,
            &["I-15"],
            LabelDir::Right,
        ),
        minor_stop(
            "ST_GEORGE",
            "St. George",
            330.0,
            720.0,
            &["I-15"],
            LabelDir::Right,
        ),
        minor_stop(
            "CEDAR_CITY",
            "Cedar City",
            345.0,
            660.0,
            &["I-15"],
            LabelDir::Right,
        ),
        minor_stop(
            "FILLMORE",
            "Fillmore",
            355.0,
            600.0,
            &["I-15"],
            LabelDir::Right,
        ),
        stop(
            "DEN",
            "Denver",
            480.0,
            540.0,
            true,
            true,
            &["I-70", "I-25", "US287"],
            LabelDir::Up,
        ),
        minor_stop(
            "GRAND_JUNCTION",
            "Grand Junction",
            420.0,
            600.0,
            &["I-70"],
            LabelDir::Down,
        ),
        minor_stop(
            "GREEN_RIVER_UT",
            "Green River UT",
            390.0,
            570.0,
            &["I-70"],
            LabelDir::Down,
        ),
        minor_stop(
            "COLORADO_SPRINGS",
            "Colorado Springs",
            540.0,
            720.0,
            &["I-25"],
            LabelDir::Right,
        ),
        minor_stop(
            "CAPITAL_BELTWAY_N",
            "Capital North",
            1920.0,
            780.0,
            &["I-95", "I-495"],
            LabelDir::Right,
        ),
        minor_stop(
            "CAPITAL_BELTWAY_E",
            "Capital East",
            1980.0,
            840.0,
            &["I-95", "I-495"],
            LabelDir::Right,
        ),
        minor_stop(
            "CAPITAL_BELTWAY_S",
            "Capital South",
            1890.0,
            900.0,
            &["I-95", "I-495"],
            LabelDir::Right,
        ),
        minor_stop(
            "JOPLIN",
            "Joplin",
            900.0,
            780.0,
            &["I-44", "I-49"],
            LabelDir::Down,
        ),
        minor_stop(
            "TEXARKANA",
            "Texarkana",
            960.0,
            900.0,
            &["I-30", "I-49"],
            LabelDir::Down,
        ),
        minor_stop(
            "EFFINGHAM",
            "Effingham",
            1140.0,
            720.0,
            &["I-70"],
            LabelDir::Down,
        ),
        minor_stop(
            "DALHART",
            "Dalhart",
            570.0,
            840.0,
            &["US287"],
            LabelDir::Left,
        ),
        minor_stop(
            "ROANOKE",
            "Roanoke",
            1440.0,
            720.0,
            &["I-81"],
            LabelDir::Down,
        ),
        minor_stop(
            "WYTHEVILLE",
            "Wytheville",
            1500.0,
            780.0,
            &["I-77", "I-81"],
            LabelDir::Right,
        ),
        minor_stop(
            "HAGERSTOWN",
            "Hagerstown",
            1620.0,
            720.0,
            &["I-70", "I-81"],
            LabelDir::Down,
        ),
        stop(
            "STL",
            "St. Louis",
            960.0,
            720.0,
            true,
            true,
            &["I-70", "I-44"],
            LabelDir::Right,
        ),
        minor_stop(
            "PITTSBURGH",
            "Pittsburgh",
            1500.0,
            660.0,
            &["I-70", "I-76", "I-77"],
            LabelDir::Up,
        ),
        minor_stop(
            "CHARLESTON_WV",
            "Charleston WV",
            1560.0,
            740.0,
            &["I-77"],
            LabelDir::Right,
        ),
        stop(
            "PENNSYLVANIA",
            "Harrisburg",
            1680.0,
            540.0,
            true,
            true,
            &["I-70", "I-76", "I-81"],
            LabelDir::Up,
        ),
        minor_stop(
            "WHEELING",
            "Wheeling",
            1500.0,
            560.0,
            &["I-70"],
            LabelDir::Down,
        ),
        minor_stop(
            "PUEBLO",
            "Pueblo",
            540.0,
            780.0,
            &["I-25", "US287"],
            LabelDir::Right,
        ),
        minor_stop("RATON", "Raton", 510.0, 840.0, &["I-25"], LabelDir::Left),
        minor_stop(
            "JUNCTION",
            "Junction",
            750.0,
            1020.0,
            &["US83"],
            LabelDir::Left,
        ),
        stop(
            "GULFPORT",
            "Gulfport",
            1260.0,
            1080.0,
            true,
            true,
            &["I-10", "US90"],
            LabelDir::Down,
        ),
        stop(
            "CORPUS",
            "Corpus Christi",
            780.0,
            1140.0,
            true,
            true,
            &["I-37"],
            LabelDir::Up,
        ),
    ];
    generate_geo_aware_beck_grid(&mut stops);
    apply_schematic_adjustments(&mut stops);
    promote_t1_bend_anchors(&mut stops);
    stops
}

fn apply_schematic_adjustments(stops: &mut [BeckStop]) {
    for stop in stops {
        if stop.id == "CORPUS" {
            stop.x += 130.0;
            stop.y += 45.0;
        }
    }
}

fn promote_t1_bend_anchors(stops: &mut [BeckStop]) {
    for (_corridor, ids) in t1_route_stop_ids() {
        for window in ids.windows(3) {
            let Some(a_idx) = stops.iter().position(|stop| stop.id == window[0]) else {
                continue;
            };
            let Some(b_idx) = stops.iter().position(|stop| stop.id == window[1]) else {
                continue;
            };
            let Some(c_idx) = stops.iter().position(|stop| stop.id == window[2]) else {
                continue;
            };
            let a = stops[a_idx].point();
            let b = stops[b_idx].point();
            let c = stops[c_idx].point();
            let ab = (b.0 - a.0, b.1 - a.1);
            let bc = (c.0 - b.0, c.1 - b.1);
            let cross = ab.0 * bc.1 - ab.1 * bc.0;
            if cross.abs() > 0.001 {
                stops[b_idx].draw = true;
            }
        }
    }
}

fn generate_geo_aware_beck_grid(stops: &mut [BeckStop]) {
    let geos = stops
        .iter()
        .enumerate()
        .map(|(idx, stop)| {
            let (lat, lon) = geo_proxy(stop);
            (idx, lat, lon)
        })
        .collect::<Vec<_>>();
    let x_positions = axis_positions(
        &geos
            .iter()
            .map(|(idx, _lat, lon)| (*idx, *lon))
            .collect::<Vec<_>>(),
        160.0,
        2280.0,
    );
    let y_positions = axis_positions(
        &geos
            .iter()
            .map(|(idx, lat, _lon)| (*idx, -*lat))
            .collect::<Vec<_>>(),
        180.0,
        1260.0,
    );
    for (idx, stop) in stops.iter_mut().enumerate() {
        stop.x = x_positions[idx];
        stop.y = y_positions[idx];
    }
}

fn axis_positions(values: &[(usize, f64)], min_out: f64, max_out: f64) -> Vec<f64> {
    let mut ordered = values.to_vec();
    ordered.sort_by(|a, b| a.1.total_cmp(&b.1).then(a.0.cmp(&b.0)));
    let mut cumulative = vec![0.0; ordered.len()];
    for i in 1..ordered.len() {
        let geo_gap = (ordered[i].1 - ordered[i - 1].1).abs();
        cumulative[i] = cumulative[i - 1] + 1.0 + geo_gap.sqrt().min(2.2);
    }
    let max_cumulative = cumulative.last().copied().unwrap_or(1.0).max(1.0);
    let mut positions = vec![0.0; values.len()];
    for (rank, (idx, _)) in ordered.iter().enumerate() {
        positions[*idx] = min_out + (cumulative[rank] / max_cumulative) * (max_out - min_out);
    }
    positions
}

fn geo_proxy(stop: &BeckStop) -> (f64, f64) {
    match stop.id {
        "BLAINE" => (48.99, -122.75),
        "SEA" => (47.61, -122.33),
        "POR" => (45.52, -122.68),
        "EUGENE" => (44.05, -123.09),
        "MEDFORD" => (42.33, -122.87),
        "REDDING" => (40.59, -122.39),
        "SAC" => (38.58, -121.49),
        "CENTRAL_VALLEY" => (36.74, -119.79),
        "LA" => (34.05, -118.24),
        "SDTJ" => (32.55, -117.05),
        "PALM_SPRINGS" => (33.83, -116.55),
        "QUARTZSITE" => (33.66, -114.23),
        "PHX" => (33.45, -112.07),
        "TUCSON" => (32.22, -110.97),
        "LORDSBURG" => (32.35, -108.71),
        "SHOW_LOW" => (34.25, -110.03),
        "ELP" => (31.76, -106.49),
        "SAT" => (29.42, -98.49),
        "WACO" => (31.55, -97.15),
        "JUNCTION" => (30.49, -99.77),
        "LRD" => (27.51, -99.51),
        "HOU" => (29.76, -95.37),
        "BEAUMONT" => (30.08, -94.13),
        "LAFAYETTE" => (30.22, -92.02),
        "NOLA" => (29.95, -90.07),
        "HATTIESBURG" => (31.33, -89.29),
        "MERIDIAN" => (32.36, -88.70),
        "TALLAHASSEE" => (30.44, -84.28),
        "PANAMA_CITY" => (30.16, -85.66),
        "JAX" => (30.33, -81.66),
        "MIA" => (25.76, -80.19),
        "DAYTONA" => (29.21, -81.02),
        "FORT_PIERCE" => (27.45, -80.33),
        "DAL" => (32.78, -96.80),
        "ABILENE" => (32.45, -99.73),
        "SHV" => (32.53, -93.75),
        "BHM_APPROACH" => (33.52, -86.80),
        "ATL" => (33.75, -84.39),
        "COLUMBIA" => (34.00, -81.03),
        "FLO" => (34.20, -79.76),
        "MIN" => (44.98, -93.27),
        "ST_CLOUD" => (45.56, -94.16),
        "MASON_CITY" => (43.15, -93.20),
        "DSM" => (41.59, -93.62),
        "IOWA_CITY" => (41.66, -91.53),
        "KC" => (39.10, -94.58),
        "COLUMBIA_MO" => (38.95, -92.33),
        "OKC" => (35.47, -97.52),
        "BARSTOW" => (34.90, -117.02),
        "ABQ" => (35.08, -106.65),
        "TUCUMCARI" => (35.17, -103.73),
        "AMA" => (35.22, -101.83),
        "ELK_CITY" => (35.41, -99.41),
        "WICHITA_FALLS" => (33.91, -98.49),
        "MEM" => (35.15, -90.05),
        "TUPELO" => (34.26, -88.70),
        "NASHVILLE" => (36.16, -86.78),
        "KNX" => (35.96, -83.92),
        "ASHEVILLE" => (35.60, -82.55),
        "CLT" => (35.23, -80.84),
        "RALEIGH_APPROACH" => (35.78, -78.64),
        "INDY_APPROACH" => (39.77, -86.16),
        "FORT_WAYNE" => (41.08, -85.14),
        "CHI_APPROACH" => (41.68, -86.25),
        "CHI" => (41.88, -87.63),
        "TOL" => (41.66, -83.56),
        "COLUMBUS" => (39.96, -83.00),
        "CINCINNATI" => (39.10, -84.51),
        "LEXINGTON" => (38.04, -84.50),
        "LOUISVILLE" => (38.22, -85.75),
        "CHATTANOOGA" => (35.05, -85.31),
        "MONTGOMERY" => (32.38, -86.30),
        "GAINESVILLE" => (29.65, -82.32),
        "TIFTON" => (31.45, -83.51),
        "TAMPA" => (27.95, -82.46),
        "RENO" => (39.53, -119.81),
        "SLC" => (40.76, -111.89),
        "BOI" => (43.62, -116.20),
        "PENDLETON" => (45.67, -118.79),
        "TWIN_FALLS" => (42.56, -114.46),
        "LEWISTON" => (46.42, -117.02),
        "CASPER" => (42.85, -106.31),
        "ROCK_SPRINGS" => (41.59, -109.20),
        "RAWLINS" => (41.79, -107.24),
        "CHEYENNE" => (41.14, -104.82),
        "OMAHA" => (41.26, -95.93),
        "FARGO" => (46.88, -96.79),
        "CLEVELAND" => (41.50, -81.69),
        "CLARION" => (41.21, -79.39),
        "PA_APPROACH" => (41.20, -77.19),
        "PENNSYLVANIA" => (40.27, -76.88),
        "NYC" => (40.71, -74.01),
        "SPK" => (47.66, -117.43),
        "MOSES_LAKE" => (47.13, -119.28),
        "BIL" => (45.78, -108.50),
        "SHERIDAN" => (44.80, -106.96),
        "GILLETTE" => (44.29, -105.50),
        "RAPID_CITY" => (44.08, -103.23),
        "CHAMBERLAIN" => (43.81, -99.33),
        "SIOUX_FALLS" => (43.55, -96.73),
        "WATERTOWN_SD" => (44.90, -97.11),
        "EAU_CLAIRE" => (44.81, -91.50),
        "MADISON" => (43.07, -89.40),
        "MILWAUKEE" => (43.04, -87.91),
        "CHI_NORTH" => (42.03, -87.75),
        "DET" => (42.33, -83.05),
        "ERIE" => (42.13, -80.09),
        "BUFFALO" => (42.89, -78.87),
        "SYRACUSE" => (43.05, -76.15),
        "ALBANY" => (42.65, -73.76),
        "BOS" => (42.36, -71.06),
        "NEW_HAVEN" => (41.31, -72.93),
        "PHILADELPHIA" => (39.95, -75.17),
        "DC" => (38.90, -77.04),
        "RIC" => (37.54, -77.43),
        "BEN" => (35.38, -78.55),
        "WIC" => (37.69, -97.34),
        "FORT_SMITH" => (35.39, -94.42),
        "GREENVILLE" => (34.85, -82.40),
        "GREENSBORO" => (36.07, -79.79),
        "MACON" => (32.84, -83.63),
        "MOBILE" => (30.69, -88.04),
        "LITTLE_ROCK" => (34.75, -92.29),
        "LAS_VEGAS" => (36.17, -115.14),
        "ST_GEORGE" => (37.10, -113.58),
        "CEDAR_CITY" => (37.68, -113.06),
        "FILLMORE" => (38.97, -112.32),
        "DEN" => (39.74, -104.99),
        "GRAND_JUNCTION" => (39.06, -108.55),
        "GREEN_RIVER_UT" => (38.99, -110.16),
        "COLORADO_SPRINGS" => (38.83, -104.82),
        "CAPITAL_BELTWAY_N" => (39.03, -77.08),
        "CAPITAL_BELTWAY_E" => (38.90, -76.90),
        "CAPITAL_BELTWAY_S" => (38.79, -77.04),
        "JOPLIN" => (37.08, -94.51),
        "TEXARKANA" => (33.43, -94.04),
        "EFFINGHAM" => (39.12, -88.54),
        "DALHART" => (36.06, -102.51),
        "ROANOKE" => (37.27, -79.94),
        "WYTHEVILLE" => (36.95, -81.08),
        "HAGERSTOWN" => (39.64, -77.72),
        "STL" => (38.63, -90.20),
        "PITTSBURGH" => (40.44, -80.00),
        "CHARLESTON_WV" => (38.35, -81.63),
        "WHEELING" => (40.06, -80.72),
        "PUEBLO" => (38.25, -104.61),
        "RATON" => (36.90, -104.44),
        "GULFPORT" => (30.37, -89.09),
        "CORPUS" => (27.80, -97.40),
        _ => fallback_geo_from_seed_grid(stop),
    }
}

fn fallback_geo_from_seed_grid(stop: &BeckStop) -> (f64, f64) {
    let lon = -124.0 + ((stop.x - 180.0) / 1800.0) * 53.0;
    let lat = 49.0 - ((stop.y - 220.0) / 930.0) * 22.0;
    (lat, lon)
}

fn point(stops: &[BeckStop], id: &str) -> (f64, f64) {
    stops
        .iter()
        .find(|stop| stop.id == id)
        .unwrap_or_else(|| panic!("missing Beck stop/anchor: {id}"))
        .point()
}

fn path(stops: &[BeckStop], ids: &[&str]) -> Vec<(f64, f64)> {
    ids.iter().map(|id| point(stops, id)).collect()
}

fn stop_by_id<'a>(stops: &'a [BeckStop], id: &str) -> &'a BeckStop {
    stops
        .iter()
        .find(|stop| stop.id == id)
        .unwrap_or_else(|| panic!("missing Beck stop/anchor: {id}"))
}

fn line_segment(stops: &[BeckStop], corridor: &'static str, ids: &[&str]) -> LineSegment {
    LineSegment {
        corridor,
        waypoints: path(stops, ids),
    }
}

fn t2_line_segment(
    stops: &[BeckStop],
    corridor: &'static str,
    start_trunk: &'static str,
    end_trunk: &'static str,
    ids: &'static [&'static str],
    service_label: &'static str,
    badge_stop: &'static str,
    badge_offset: (f64, f64),
    label_anchor: &'static str,
) -> T2LineSegment {
    let badge_base = point(stops, badge_stop);
    T2LineSegment {
        corridor,
        trunk: start_trunk,
        start_trunk,
        end_trunk,
        service_label,
        badge: (badge_base.0 + badge_offset.0, badge_base.1 + badge_offset.1),
        label_anchor,
        stop_ids: ids.to_vec(),
        waypoints: path(stops, ids),
        lane_shift: t2_lane_shift(corridor),
    }
}

fn t2_lane_shift(corridor: &str) -> (f64, f64) {
    match corridor {
        "I-15" => (28.0, 0.0),
        "I-22" => (0.0, 44.0),
        "I-24" => (0.0, -34.0),
        "I-25" => (-32.0, 0.0),
        "I-84" => (0.0, -42.0),
        "I-29" => (34.0, 0.0),
        "I-30" => (0.0, 34.0),
        "I-37" => (0.0, 30.0),
        "I-44" => (0.0, -46.0),
        "I-49" => (-28.0, 0.0),
        "I-59" => (0.0, 32.0),
        "I-65" => (0.0, -32.0),
        "I-76" => (0.0, -30.0),
        "I-77" => (36.0, 0.0),
        "I-81" => (-32.0, 0.0),
        "I-85" => (-36.0, 0.0),
        "I-495" => (30.0, 0.0),
        "US30" => (0.0, -40.0),
        "US6" => (0.0, 36.0),
        "US70" => (0.0, 38.0),
        "US80" => (0.0, -38.0),
        "US83" => (-36.0, 0.0),
        "US95" => (-34.0, 0.0),
        "US90" => (0.0, 44.0),
        "US287" => (32.0, 0.0),
        _ => (0.0, 0.0),
    }
}

fn t1_line_segments(stops: &[BeckStop]) -> Vec<LineSegment> {
    t1_route_stop_ids()
        .iter()
        .map(|(corridor, ids)| line_segment(stops, corridor, ids))
        .collect()
}

fn t1_route_stop_ids() -> Vec<(&'static str, Vec<&'static str>)> {
    vec![
        (
            "I-5",
            vec![
                "BLAINE",
                "SEA",
                "POR",
                "EUGENE",
                "MEDFORD",
                "REDDING",
                "SAC",
                "CENTRAL_VALLEY",
                "LA",
                "SDTJ",
            ],
        ),
        (
            "I-10",
            vec![
                "LA",
                "PALM_SPRINGS",
                "QUARTZSITE",
                "PHX",
                "TUCSON",
                "LORDSBURG",
                "ELP",
                "SAT",
                "HOU",
                "BEAUMONT",
                "LAFAYETTE",
                "NOLA",
                "GULFPORT",
                "MOBILE",
                "PANAMA_CITY",
                "TALLAHASSEE",
                "JAX",
            ],
        ),
        (
            "I-20",
            vec!["DAL", "SHV", "BHM_APPROACH", "ATL", "COLUMBIA", "FLO"],
        ),
        (
            "I-35",
            vec![
                "MIN",
                "MASON_CITY",
                "DSM",
                "KC",
                "WIC",
                "OKC",
                "DAL",
                "WACO",
                "SAT",
                "LRD",
            ],
        ),
        (
            "I-40",
            vec![
                "BARSTOW",
                "ABQ",
                "TUCUMCARI",
                "AMA",
                "ELK_CITY",
                "OKC",
                "FORT_SMITH",
                "LITTLE_ROCK",
                "MEM",
                "NASHVILLE",
                "KNX",
                "ASHEVILLE",
                "CLT",
                "RALEIGH_APPROACH",
                "FLO",
                "BEN",
            ],
        ),
        (
            "I-69",
            vec![
                "HOU",
                "MEM",
                "INDY_APPROACH",
                "FORT_WAYNE",
                "CHI_APPROACH",
                "CHI",
            ],
        ),
        (
            "I-70",
            vec![
                "SLC",
                "GREEN_RIVER_UT",
                "GRAND_JUNCTION",
                "DEN",
                "KC",
                "COLUMBIA_MO",
                "STL",
                "EFFINGHAM",
                "INDY_APPROACH",
                "COLUMBUS",
                "WHEELING",
                "PENNSYLVANIA",
                "DC",
            ],
        ),
        (
            "I-75",
            vec![
                "TOL",
                "COLUMBUS",
                "CINCINNATI",
                "LEXINGTON",
                "KNX",
                "CHATTANOOGA",
                "ATL",
                "MACON",
                "TIFTON",
                "GAINESVILLE",
                "TAMPA",
                "MIA",
            ],
        ),
        (
            "I-80",
            vec![
                "SAC",
                "RENO",
                "SLC",
                "ROCK_SPRINGS",
                "RAWLINS",
                "CHEYENNE",
                "OMAHA",
                "DSM",
                "IOWA_CITY",
                "CHI",
                "TOL",
                "CLEVELAND",
                "CLARION",
                "PA_APPROACH",
                "NYC",
            ],
        ),
        (
            "I-90",
            vec![
                "SEA",
                "MOSES_LAKE",
                "SPK",
                "BIL",
                "GILLETTE",
                "RAPID_CITY",
                "CHAMBERLAIN",
                "SIOUX_FALLS",
                "MIN",
                "EAU_CLAIRE",
                "MADISON",
                "MILWAUKEE",
                "CHI_NORTH",
                "CHI",
                "TOL",
                "DET",
                "ERIE",
                "BUFFALO",
                "SYRACUSE",
                "ALBANY",
                "BOS",
            ],
        ),
        (
            "I-95",
            vec![
                "BOS",
                "NEW_HAVEN",
                "NYC",
                "PHILADELPHIA",
                "DC",
                "RIC",
                "RALEIGH_APPROACH",
                "FLO",
                "BEN",
                "JAX",
                "DAYTONA",
                "FORT_PIERCE",
                "MIA",
            ],
        ),
    ]
}

fn t2_line_segments(stops: &[BeckStop]) -> Vec<T2LineSegment> {
    vec![
        t2_line_segment(
            stops,
            "I-85",
            "I-20",
            "I-95",
            &["ATL", "GREENVILLE", "CLT", "GREENSBORO", "RIC"],
            "Charlotte Crescent",
            "CLT",
            (-20.0, 40.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "I-65",
            "I-10",
            "I-75",
            &[
                "MOBILE",
                "BHM_APPROACH",
                "NASHVILLE",
                "LOUISVILLE",
                "CINCINNATI",
            ],
            "Birmingham",
            "BHM_APPROACH",
            (-15.0, -30.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "I-24",
            "I-40",
            "I-75",
            &["MEM", "NASHVILLE", "KNX"],
            "Nashville",
            "NASHVILLE",
            (30.0, -20.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-30",
            "I-35",
            "I-40",
            &["DAL", "TEXARKANA", "LITTLE_ROCK", "MEM"],
            "Arkansas Link",
            "LITTLE_ROCK",
            (-10.0, 30.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "I-15",
            "I-5",
            "I-70",
            &[
                "LA",
                "BARSTOW",
                "LAS_VEGAS",
                "ST_GEORGE",
                "CEDAR_CITY",
                "FILLMORE",
                "SLC",
            ],
            "Inland West",
            "LAS_VEGAS",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-84",
            "I-5",
            "I-70",
            &["POR", "PENDLETON", "BOI", "TWIN_FALLS", "SLC"],
            "Snake River",
            "BOI",
            (10.0, -26.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "US95",
            "I-84",
            "I-90",
            &["BOI", "LEWISTON", "SPK"],
            "Inland Northwest",
            "BOI",
            (-18.0, 30.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "I-25",
            "I-90",
            "I-40",
            &[
                "BIL",
                "SHERIDAN",
                "CASPER",
                "CHEYENNE",
                "DEN",
                "COLORADO_SPRINGS",
                "PUEBLO",
                "RATON",
                "ABQ",
            ],
            "High Plains",
            "DEN",
            (-20.0, 15.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "US287",
            "I-35",
            "I-70",
            &[
                "DAL",
                "WICHITA_FALLS",
                "AMA",
                "DALHART",
                "PUEBLO",
                "COLORADO_SPRINGS",
                "DEN",
            ],
            "Front Range",
            "AMA",
            (5.0, 5.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "I-495",
            "I-95",
            "I-95",
            &[
                "DC",
                "CAPITAL_BELTWAY_N",
                "CAPITAL_BELTWAY_E",
                "CAPITAL_BELTWAY_S",
                "DC",
            ],
            "Capital Beltway",
            "CAPITAL_BELTWAY_E",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-59",
            "I-10",
            "I-75",
            &[
                "NOLA",
                "HATTIESBURG",
                "MERIDIAN",
                "BHM_APPROACH",
                "CHATTANOOGA",
            ],
            "Birmingham Spur",
            "BHM_APPROACH",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-49",
            "I-20",
            "I-35",
            &["SHV", "TEXARKANA", "FORT_SMITH", "JOPLIN", "KC"],
            "Ozarks",
            "JOPLIN",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-81",
            "I-40",
            "I-95",
            &[
                "KNX",
                "WYTHEVILLE",
                "ROANOKE",
                "HAGERSTOWN",
                "PENNSYLVANIA",
                "NYC",
            ],
            "Knoxville-New York",
            "HAGERSTOWN",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-44",
            "I-40",
            "I-69",
            &["OKC", "STL", "INDY_APPROACH", "FORT_WAYNE", "CHI_APPROACH"],
            "St. Louis Link",
            "FORT_WAYNE",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-77",
            "I-40",
            "I-80",
            &[
                "CLT",
                "WYTHEVILLE",
                "CHARLESTON_WV",
                "PITTSBURGH",
                "CLEVELAND",
            ],
            "Appalachian North",
            "PITTSBURGH",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-76",
            "I-80",
            "I-95",
            &["CLEVELAND", "PITTSBURGH", "PENNSYLVANIA", "PHILADELPHIA"],
            "Pennsylvania",
            "PENNSYLVANIA",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "US30",
            "I-70",
            "I-80",
            &[
                "SLC",
                "ROCK_SPRINGS",
                "RAWLINS",
                "CHEYENNE",
                "OMAHA",
                "IOWA_CITY",
                "CHI_APPROACH",
                "PA_APPROACH",
            ],
            "Lincoln Highway",
            "CHI_APPROACH",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "US6",
            "I-70",
            "I-80",
            &["SLC", "ROCK_SPRINGS", "CHEYENNE", "OMAHA", "CHI_APPROACH"],
            "Central Plains",
            "OMAHA",
            (0.0, 0.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "US70",
            "I-10",
            "I-40",
            &[
                "PHX",
                "SHOW_LOW",
                "ABQ",
                "AMA",
                "OKC",
                "FORT_SMITH",
                "LITTLE_ROCK",
                "MEM",
                "KNX",
            ],
            "Mid-South",
            "LITTLE_ROCK",
            (-36.0, 38.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "US90",
            "I-10",
            "I-95",
            &["SAT", "LAFAYETTE", "GULFPORT", "JAX"],
            "Gulf Local",
            "GULFPORT",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-22",
            "I-40",
            "I-20",
            &["MEM", "TUPELO", "BHM_APPROACH"],
            "Memphis-Birmingham",
            "BHM_APPROACH",
            (-48.0, 50.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "I-29",
            "I-80",
            "I-35",
            &[
                "OMAHA",
                "SIOUX_FALLS",
                "WATERTOWN_SD",
                "FARGO",
                "ST_CLOUD",
                "MIN",
            ],
            "Upper Missouri",
            "FARGO",
            (0.0, 0.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "US80",
            "I-35",
            "I-20",
            &[
                "DAL",
                "ABILENE",
                "LITTLE_ROCK",
                "MEM",
                "TUPELO",
                "BHM_APPROACH",
                "MONTGOMERY",
                "ATL",
            ],
            "Old South",
            "MONTGOMERY",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "US83",
            "I-90",
            "I-10",
            &["MIN", "RAPID_CITY", "ABILENE", "JUNCTION", "SAT"],
            "Dakota Spine",
            "ABILENE",
            (0.0, 0.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "I-37",
            "I-10",
            "I-10",
            &["SAT", "CORPUS"],
            "Corpus Christi",
            "CORPUS",
            (36.0, 26.0),
            "start",
        ),
    ]
}

fn badge_at(
    stops: &[BeckStop],
    corridor: &'static str,
    stop_id: &str,
    offset: (f64, f64),
) -> (&'static str, f64, f64) {
    let (x, y) = point(stops, stop_id);
    (corridor, x + offset.0, y + offset.1)
}

fn t1_badges(stops: &[BeckStop]) -> Vec<(&'static str, f64, f64)> {
    vec![
        badge_at(stops, "I-5", "POR", (-20.0, 100.0)),
        badge_at(stops, "I-10", "ELP", (100.0, 0.0)),
        badge_at(stops, "I-20", "SHV", (-55.0, 58.0)),
        badge_at(stops, "I-35", "KC", (-35.0, 15.0)),
        badge_at(stops, "I-40", "AMA", (-40.0, 0.0)),
        badge_at(stops, "I-69", "MEM", (0.0, -55.0)),
        badge_at(stops, "I-70", "DEN", (34.0, -28.0)),
        badge_at(stops, "I-75", "CINCINNATI", (-24.0, 100.0)),
        badge_at(stops, "I-80", "OMAHA", (-60.0, 20.0)),
        badge_at(stops, "I-90", "RAPID_CITY", (-60.0, 20.0)),
        badge_at(stops, "I-95", "NYC", (-20.0, 40.0)),
    ]
}

fn is_primary_terminal(id: &str) -> bool {
    matches!(id, "LA" | "NYC" | "BLAINE" | "SDTJ" | "LRD" | "MIA")
}

#[derive(Clone, Copy)]
enum BeckVariant {
    T1,
    T1WithT2,
    T2Only,
}

/// Generate the Beck schematic SVG.
///
/// Layout rationale:
/// - Canvas: 2400×1350 (same as megamap for consistency)
/// - Grid unit: ~60px; stations spaced ~1-3 grid units apart
/// - Central area (Chicago/Midwest) expanded; coastal areas compressed
/// - West Coast runs vertically on left; East Coast on right
/// - Transcontinentals run horizontally across middle
pub fn build_beck_svg() -> String {
    build_beck_svg_variant(BeckVariant::T1)
}

/// Generate the expanded Beck schematic with T2 connectors as thin trunk-tinted lines.
pub fn build_beck_t2_svg() -> String {
    build_beck_svg_variant(BeckVariant::T1WithT2)
}

/// Generate a T2-focused Beck schematic without the bold T1 trunk layer.
pub fn build_beck_t2_only_svg() -> String {
    build_beck_svg_variant(BeckVariant::T2Only)
}

pub fn beck_stop_catalog() -> Vec<BeckStopCatalogRow> {
    beck_stops()
        .into_iter()
        .filter(|stop| stop.draw)
        .map(|stop| {
            let (lat, lon) = geo_proxy(&stop);
            BeckStopCatalogRow {
                id: stop.id,
                label: stop.label,
                lat,
                lon,
                lines: stop.lines.to_vec(),
                stop_class: stop_class(&stop),
            }
        })
        .collect()
}

pub fn build_beck_t2_diagnostics_csv() -> String {
    let mut csv = String::from(
        "corridor,trunk,start_trunk,end_trunk,color_mode,split_anchor,split_anchor_offset_pct,unstopped_t1_contact_count,unstopped_t1_contacts,close_parallel_count,close_parallel_corridors,service_label,stop_count,drawn_stop_count,transfer_stop_count,schematic_length_px,min_x,min_y,max_x,max_y,label_density_per_100px,review_flag\n",
    );
    for row in beck_t2_diagnostics() {
        push_csv_row(
            &mut csv,
            &[
                row.corridor,
                row.trunk,
                row.start_trunk,
                row.end_trunk,
                row.color_mode,
                row.split_anchor,
                &format!("{:.0}", row.split_anchor_offset_pct),
                &row.unstopped_t1_contact_count.to_string(),
                &row.unstopped_t1_contacts,
                &row.close_parallel_count.to_string(),
                &row.close_parallel_corridors,
                row.service_label,
                &row.stop_count.to_string(),
                &row.drawn_stop_count.to_string(),
                &row.transfer_stop_count.to_string(),
                &format!("{:.0}", row.schematic_length_px),
                &format!("{:.0}", row.min_x),
                &format!("{:.0}", row.min_y),
                &format!("{:.0}", row.max_x),
                &format!("{:.0}", row.max_y),
                &format!("{:.2}", row.label_density_per_100px),
                row.review_flag,
            ],
        );
    }
    csv
}

pub fn beck_t2_diagnostics() -> Vec<BeckT2DiagnosticRow> {
    let stops = beck_stops();
    let t2_lines = t2_line_segments(&stops);
    let mut rows = t2_lines
        .iter()
        .map(|line| {
            let points = line.routed_waypoints();
            let (min_x, min_y, max_x, max_y) = bounds(&points);
            let schematic_length_px = polyline_length(&points);
            let split_anchor = t2_split_anchor_distance(line, &stops);
            let unstopped_t1_contacts = unstopped_t1_contacts(line, &stops);
            let close_parallel_corridors = close_parallel_corridors(line, &t2_lines);
            let unstopped_t1_contact_count = unstopped_t1_contacts.len();
            let close_parallel_count = close_parallel_corridors.len();
            let drawn_stop_count = line
                .stop_ids
                .iter()
                .filter(|id| stop_by_id(&stops, id).draw)
                .count();
            let transfer_stop_count = line
                .stop_ids
                .iter()
                .filter(|id| {
                    let stop = stop_by_id(&stops, id);
                    stop.is_transfer_point()
                })
                .count();
            let label_density_per_100px = if schematic_length_px > 0.0 {
                drawn_stop_count as f64 / schematic_length_px * 100.0
            } else {
                0.0
            };
            let review_flag = if unstopped_t1_contact_count > 0 {
                "unstopped-t1-contact-review"
            } else if close_parallel_count >= 2 {
                "parallel-spacing-review"
            } else if line.is_split_color() && split_anchor.is_none() {
                "split-anchor-review"
            } else if label_density_per_100px >= 0.95 {
                "dense-label-review"
            } else if transfer_stop_count >= 5 {
                "transfer-complexity-review"
            } else if schematic_length_px >= 900.0 {
                "long-connector-review"
            } else {
                "ok"
            };
            BeckT2DiagnosticRow {
                corridor: line.corridor,
                trunk: line.trunk,
                start_trunk: line.start_trunk,
                end_trunk: line.end_trunk,
                color_mode: if line.is_split_color() {
                    "split-parent"
                } else {
                    "single-parent"
                },
                split_anchor: split_anchor
                    .as_ref()
                    .map(|anchor| anchor.stop_id)
                    .unwrap_or(""),
                split_anchor_offset_pct: split_anchor
                    .as_ref()
                    .map(|anchor| (anchor.fraction - 0.5).abs() * 100.0)
                    .unwrap_or(0.0),
                unstopped_t1_contact_count,
                unstopped_t1_contacts: unstopped_t1_contacts.join(";"),
                close_parallel_count,
                close_parallel_corridors: close_parallel_corridors.join(";"),
                service_label: line.service_label,
                stop_count: line.stop_ids.len(),
                drawn_stop_count,
                transfer_stop_count,
                schematic_length_px,
                min_x,
                min_y,
                max_x,
                max_y,
                label_density_per_100px,
                review_flag,
            }
        })
        .collect::<Vec<_>>();
    rows.sort_by(|a, b| {
        a.review_flag
            .cmp(b.review_flag)
            .then_with(|| {
                b.label_density_per_100px
                    .total_cmp(&a.label_density_per_100px)
            })
            .then_with(|| a.corridor.cmp(b.corridor))
    });
    rows
}

/// Generate the stop-to-stop SLA surface implied by the Beck T1/T2 topology.
///
/// These are heuristic planning commitments, not publication-grade performance
/// proof. They deliberately expose the stop path and evidence status so game,
/// map, and standards workflows can tell "modeled promise" from observed SLA.
pub fn build_beck_stop_sla_csv() -> String {
    let stops = beck_stops();
    let graph = sla_graph(&stops);
    let mut csv = String::from(
        "origin_id,origin_label,origin_tier,origin_class,dest_id,dest_label,dest_tier,dest_class,network_miles,max_stop_gap_miles,stop_gap_status,stop_count,line_changes,route_path,stop_path,dominant_tier,freight_relay_p95_h,freight_full_i20_p95_h,freight_sla_window,passenger_bus_p95_h,passenger_av_p95_h,air_door_to_door_h,passenger_competitive_with_air,rail_competition_note,evidence_status\n",
    );

    for (idx, origin) in stops.iter().filter(|stop| stop.draw).enumerate() {
        for dest in stops.iter().filter(|stop| stop.draw).skip(idx + 1) {
            let Some(steps) = shortest_sla_path(&graph, origin.id, dest.id) else {
                continue;
            };
            let miles = steps.iter().map(|step| step.miles).sum::<f64>();
            let max_gap = steps.iter().map(|step| step.miles).fold(0.0, f64::max);
            let line_changes = line_changes(&steps);
            let stop_path = stop_path(origin.id, &steps).join(";");
            let route_path = route_path(&steps).join(";");
            let stop_count = steps.len() + 1;
            let dominant_tier = dominant_tier(&steps);

            let freight_relay = freight_relay_p95_hours(miles, line_changes, stop_count);
            let freight_full = freight_full_i20_p95_hours(miles, line_changes, stop_count);
            let passenger_bus = passenger_bus_p95_hours(miles, line_changes);
            let passenger_av = passenger_av_p95_hours(miles, line_changes);
            let air = air_door_to_door_hours(miles);
            let air_comp = passenger_air_competitiveness(passenger_av, air, miles);
            let rail_note = rail_competition_note(origin, dest, miles, dominant_tier);

            push_csv_row(
                &mut csv,
                &[
                    origin.id,
                    origin.label,
                    stop_tier(origin, &steps),
                    stop_class(origin),
                    dest.id,
                    dest.label,
                    stop_tier(dest, &steps),
                    stop_class(dest),
                    &format!("{miles:.0}"),
                    &format!("{max_gap:.0}"),
                    stop_gap_status(max_gap),
                    &stop_count.to_string(),
                    &line_changes.to_string(),
                    &route_path,
                    &stop_path,
                    dominant_tier.as_str(),
                    &format!("{freight_relay:.1}"),
                    &format!("{freight_full:.1}"),
                    sla_window(freight_full),
                    &format!("{passenger_bus:.1}"),
                    &format!("{passenger_av:.1}"),
                    &format!("{air:.1}"),
                    air_comp,
                    rail_note,
                    "heuristic-planning",
                ],
            );
        }
    }

    csv
}

fn sla_graph(stops: &[BeckStop]) -> HashMap<&'static str, Vec<SlaEdge>> {
    let mut graph: HashMap<&'static str, Vec<SlaEdge>> = HashMap::new();

    for (route, ids) in t1_route_stop_ids() {
        add_sla_chain(stops, &mut graph, route, RouteTier::T1, &ids);
    }
    for line in t2_line_segments(stops) {
        add_sla_chain(
            stops,
            &mut graph,
            line.corridor,
            RouteTier::T2,
            &line.stop_ids,
        );
    }

    graph
}

fn add_sla_chain(
    stops: &[BeckStop],
    graph: &mut HashMap<&'static str, Vec<SlaEdge>>,
    route: &'static str,
    tier: RouteTier,
    ids: &[&'static str],
) {
    for pair in ids.windows(2) {
        let from = pair[0];
        let to = pair[1];
        let miles = geo_miles(stop_by_id(stops, from), stop_by_id(stops, to));
        add_sla_edge(graph, from, to, route, tier, miles);
        add_sla_edge(graph, to, from, route, tier, miles);
    }
}

fn add_sla_edge(
    graph: &mut HashMap<&'static str, Vec<SlaEdge>>,
    from: &'static str,
    to: &'static str,
    route: &'static str,
    tier: RouteTier,
    miles: f64,
) {
    graph.entry(from).or_default().push(SlaEdge {
        to,
        route,
        tier,
        miles,
    });
}

fn bounds(points: &[(f64, f64)]) -> (f64, f64, f64, f64) {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for (x, y) in points {
        min_x = min_x.min(*x);
        min_y = min_y.min(*y);
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }
    if points.is_empty() {
        (0.0, 0.0, 0.0, 0.0)
    } else {
        (min_x, min_y, max_x, max_y)
    }
}

fn polyline_length(points: &[(f64, f64)]) -> f64 {
    points
        .windows(2)
        .map(|pair| {
            let dx = pair[1].0 - pair[0].0;
            let dy = pair[1].1 - pair[0].1;
            (dx * dx + dy * dy).sqrt()
        })
        .sum()
}

fn distance_to_segment(point: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
    let (px, py) = point;
    let (ax, ay) = a;
    let (bx, by) = b;
    let dx = bx - ax;
    let dy = by - ay;
    if dx.abs() < 0.001 && dy.abs() < 0.001 {
        return ((px - ax).powi(2) + (py - ay).powi(2)).sqrt();
    }
    let t = (((px - ax) * dx + (py - ay) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
    let cx = ax + t * dx;
    let cy = ay + t * dy;
    ((px - cx).powi(2) + (py - cy).powi(2)).sqrt()
}

fn distance_to_polyline(point: (f64, f64), points: &[(f64, f64)]) -> f64 {
    points
        .windows(2)
        .map(|segment| distance_to_segment(point, segment[0], segment[1]))
        .fold(f64::INFINITY, f64::min)
}

fn point_on_polyline(point: (f64, f64), points: &[(f64, f64)], tolerance: f64) -> bool {
    distance_to_polyline(point, points) <= tolerance
}

fn t2_split_anchor_distance(line: &T2LineSegment, stops: &[BeckStop]) -> Option<T2SplitAnchor> {
    if !line.is_split_color() || line.stop_ids.len() < 3 {
        return None;
    }
    let raw_length = polyline_length(&line.waypoints);
    if raw_length <= 0.0 {
        return None;
    }

    let mut walked = 0.0;
    let mut best: Option<T2SplitAnchor> = None;
    let mut fallback: Option<T2SplitAnchor> = None;
    for index in 1..line.stop_ids.len() - 1 {
        walked += ((line.waypoints[index].0 - line.waypoints[index - 1].0).powi(2)
            + (line.waypoints[index].1 - line.waypoints[index - 1].1).powi(2))
        .sqrt();
        let stop = stop_by_id(stops, line.stop_ids[index]);
        let fraction = walked / raw_length;
        let offset = (fraction - 0.5).abs();
        if offset <= 0.32
            && fallback
                .as_ref()
                .map(|anchor| offset < (anchor.fraction - 0.5).abs())
                .unwrap_or(true)
        {
            fallback = Some(T2SplitAnchor {
                stop_id: stop.id,
                fraction,
            });
        }
        if !stop.is_transfer_point() || offset > 0.22 {
            continue;
        }
        let replace = best
            .as_ref()
            .map(|anchor| offset < (anchor.fraction - 0.5).abs())
            .unwrap_or(true);
        if replace {
            best = Some(T2SplitAnchor {
                stop_id: stop.id,
                fraction,
            });
        }
    }
    best.or(fallback)
}

fn unstopped_t1_contacts(line: &T2LineSegment, stops: &[BeckStop]) -> Vec<&'static str> {
    let points = line.routed_waypoints();
    let line_stop_ids = line.stop_ids.iter().copied().collect::<HashSet<_>>();
    let t1_stop_ids = t1_route_stop_ids()
        .into_iter()
        .flat_map(|(_corridor, ids)| ids)
        .collect::<HashSet<_>>();
    stops
        .iter()
        .filter(|stop| {
            stop.draw
                && !line_stop_ids.contains(stop.id)
                && t1_stop_ids.contains(stop.id)
                && (stop.lines.contains(&line.start_trunk) || stop.lines.contains(&line.end_trunk))
                && point_on_polyline(stop.point(), &points, 10.0)
        })
        .map(|stop| stop.id)
        .collect()
}

fn close_parallel_corridors(
    line: &T2LineSegment,
    all_lines: &[T2LineSegment],
) -> Vec<&'static str> {
    let points = line.routed_waypoints();
    all_lines
        .iter()
        .filter(|other| other.corridor != line.corridor)
        .filter(|other| {
            let other_points = other.routed_waypoints();
            points.windows(2).any(|a| {
                other_points
                    .windows(2)
                    .any(|b| close_parallel_segments(a[0], a[1], b[0], b[1]))
            })
        })
        .map(|other| other.corridor)
        .collect()
}

fn close_parallel_segments(a1: (f64, f64), a2: (f64, f64), b1: (f64, f64), b2: (f64, f64)) -> bool {
    let av = (a2.0 - a1.0, a2.1 - a1.1);
    let bv = (b2.0 - b1.0, b2.1 - b1.1);
    let a_len = (av.0 * av.0 + av.1 * av.1).sqrt();
    let b_len = (bv.0 * bv.0 + bv.1 * bv.1).sqrt();
    if a_len < 80.0 || b_len < 80.0 {
        return false;
    }
    let cross = (av.0 * bv.1 - av.1 * bv.0).abs() / (a_len * b_len);
    if cross > 0.08 {
        return false;
    }
    let distance = [
        distance_to_segment(a1, b1, b2),
        distance_to_segment(a2, b1, b2),
        distance_to_segment(b1, a1, a2),
        distance_to_segment(b2, a1, a2),
    ]
    .into_iter()
    .fold(f64::INFINITY, f64::min);
    distance > 0.001 && distance <= 18.0
}

fn shortest_sla_path(
    graph: &HashMap<&'static str, Vec<SlaEdge>>,
    origin: &'static str,
    dest: &'static str,
) -> Option<Vec<SlaStep>> {
    let mut dist: HashMap<&'static str, f64> = HashMap::from([(origin, 0.0)]);
    let mut prev: HashMap<&'static str, (&'static str, SlaEdge)> = HashMap::new();
    let mut visited = HashSet::new();

    loop {
        let Some((node, cost)) = dist
            .iter()
            .filter(|(node, _)| !visited.contains(**node))
            .min_by(|a, b| a.1.total_cmp(b.1))
            .map(|(node, cost)| (*node, *cost))
        else {
            break;
        };
        if node == dest {
            break;
        }
        visited.insert(node);
        for edge in graph.get(node).into_iter().flatten() {
            let next_cost = cost + sla_routing_weight(edge);
            if next_cost + 0.001 < *dist.get(edge.to).unwrap_or(&f64::INFINITY) {
                dist.insert(edge.to, next_cost);
                prev.insert(edge.to, (node, edge.clone()));
            }
        }
    }

    if !dist.contains_key(dest) {
        return None;
    }

    let mut reversed = Vec::new();
    let mut cursor = dest;
    while cursor != origin {
        let (from, edge) = prev.get(cursor)?;
        reversed.push(SlaStep {
            from: *from,
            to: edge.to,
            route: edge.route,
            tier: edge.tier,
            miles: edge.miles,
        });
        cursor = *from;
    }
    reversed.reverse();
    Some(reversed)
}

fn sla_routing_weight(edge: &SlaEdge) -> f64 {
    let coarse_gap_penalty = if edge.miles > 450.0 { 2_000.0 } else { 0.0 };
    let tier_penalty = if edge.tier == RouteTier::T2 {
        12.0
    } else {
        0.0
    };
    edge.miles + coarse_gap_penalty + tier_penalty
}

fn geo_miles(a: &BeckStop, b: &BeckStop) -> f64 {
    let (lat1, lon1) = geo_proxy(a);
    let (lat2, lon2) = geo_proxy(b);
    let earth_radius_miles = 3958.8_f64;
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let h = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos() * (dlon / 2.0).sin().powi(2);
    let crow = 2.0 * earth_radius_miles * h.sqrt().asin();
    crow * 1.18
}

fn line_changes(steps: &[SlaStep]) -> usize {
    steps
        .windows(2)
        .filter(|pair| pair[0].route != pair[1].route)
        .count()
}

fn stop_path(origin: &'static str, steps: &[SlaStep]) -> Vec<&'static str> {
    let mut ids = vec![origin];
    ids.extend(steps.iter().map(|step| step.to));
    ids
}

fn route_path(steps: &[SlaStep]) -> Vec<&'static str> {
    let mut routes = Vec::new();
    for step in steps {
        if routes.last().copied() != Some(step.route) {
            routes.push(step.route);
        }
    }
    routes
}

fn dominant_tier(steps: &[SlaStep]) -> RouteTier {
    if steps.iter().any(|step| step.tier == RouteTier::T1) {
        RouteTier::T1
    } else {
        RouteTier::T2
    }
}

fn stop_tier(stop: &BeckStop, steps: &[SlaStep]) -> &'static str {
    if steps
        .iter()
        .any(|step| step.tier == RouteTier::T1 && (step.from == stop.id || step.to == stop.id))
    {
        RouteTier::T1.as_str()
    } else if !stop.lines.is_empty() {
        RouteTier::T2.as_str()
    } else {
        RouteTier::T3.as_str()
    }
}

fn stop_class(stop: &BeckStop) -> &'static str {
    beck_stop_class(stop).as_str()
}

fn beck_stop_class(stop: &BeckStop) -> StopNodeClass {
    match (stop.is_hub, stop.is_interchange) {
        (true, true) => StopNodeClass::NationalTransferHub,
        (true, false) => StopNodeClass::NationalTerminal,
        (false, true) => StopNodeClass::TransferHub,
        (false, false) => StopNodeClass::ServiceStop,
    }
}

fn freight_relay_p95_hours(miles: f64, line_changes: usize, stop_count: usize) -> f64 {
    miles / 47.0 + line_changes as f64 * 0.35 + stop_count.saturating_sub(2) as f64 * 0.08
}

fn freight_full_i20_p95_hours(miles: f64, line_changes: usize, stop_count: usize) -> f64 {
    miles / 55.0 + line_changes as f64 * 0.25 + stop_count.saturating_sub(2) as f64 * 0.06
}

fn passenger_bus_p95_hours(miles: f64, line_changes: usize) -> f64 {
    miles / 55.0 + 1.5 + line_changes as f64 * 0.35
}

fn passenger_av_p95_hours(miles: f64, line_changes: usize) -> f64 {
    miles / 75.0 + 0.5 + line_changes as f64 * 0.20
}

fn air_door_to_door_hours(miles: f64) -> f64 {
    2.5 + miles / 500.0
}

fn sla_window(hours: f64) -> &'static str {
    if hours <= 6.0 {
        "6h local"
    } else if hours <= 12.0 {
        "12h half-day"
    } else if hours <= 24.0 {
        "24h overnight"
    } else if hours <= 36.0 {
        "36h next-day"
    } else if hours <= 48.0 {
        "48h two-day"
    } else if hours <= 72.0 {
        "72h three-day"
    } else {
        ">72h source-gated"
    }
}

fn stop_gap_status(max_gap: f64) -> &'static str {
    if max_gap <= 250.0 {
        "sla-stop-rhythm-ok"
    } else if max_gap <= 400.0 {
        "long-gap-review"
    } else {
        "needs-intermediate-stops"
    }
}

fn passenger_air_competitiveness(av_hours: f64, air_hours: f64, miles: f64) -> &'static str {
    if av_hours <= air_hours {
        "time-competitive"
    } else if miles <= 350.0 && av_hours <= air_hours + 1.5 {
        "near-competitive-short-haul"
    } else {
        "not-time-competitive"
    }
}

fn rail_competition_note(
    origin: &BeckStop,
    dest: &BeckStop,
    miles: f64,
    dominant_tier: RouteTier,
) -> &'static str {
    let corridor = [origin.id, dest.id];
    if corridor.contains(&"NYC")
        && (corridor.contains(&"PHILADELPHIA")
            || corridor.contains(&"DC")
            || corridor.contains(&"BOS")
            || corridor.contains(&"NEW_HAVEN"))
    {
        "passenger-rail-wins-where-high-frequency"
    } else if miles >= 700.0 {
        "freight-rail-wins-bulk-road-wins-express"
    } else if dominant_tier == RouteTier::T2 {
        "rail-often-absent-road-flexibility-case"
    } else {
        "corridor-specific-rail-check-needed"
    }
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

fn build_beck_svg_variant(variant: BeckVariant) -> String {
    let mut s = String::new();

    // Background
    s += &format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {W} {H}\" \
         width=\"{W}\" height=\"{H}\">\n\
         <defs>\n\
           <filter id=\"glow\">\n\
             <feGaussianBlur stdDeviation=\"3\" result=\"blur\"/>\n\
             <feComposite in=\"SourceGraphic\" in2=\"blur\" operator=\"over\"/>\n\
           </filter>\n\
         </defs>\n\
         <rect width=\"{W}\" height=\"{H}\" fill=\"#0f1623\"/>\n"
    );

    let expanded = matches!(variant, BeckVariant::T1WithT2);
    let t2_only = matches!(variant, BeckVariant::T2Only);
    let title = match variant {
        BeckVariant::T1 => "THE CONTINENTAL METRO",
        BeckVariant::T1WithT2 => "THE CONTINENTAL METRO : T2 LOCAL SERVICES",
        BeckVariant::T2Only => "THE CONTINENTAL METRO : T2 SERVICE MAP",
    };
    let subtitle = match variant {
        BeckVariant::T1 => "A SCHEMATIC OF AMERICA'S PRIMARY FREIGHT LINES",
        BeckVariant::T1WithT2 => "FULL SERVICE MAP · 25 THIN T2 CONNECTORS COLOR-CODED TO TRUNKS",
        BeckVariant::T2Only => "T2 CONNECTORS ONLY · TRANSFER STOPS AND SERVICE RHYTHM",
    };

    // Title
    s += "<text x=\"1200\" y=\"60\" font-family=\"'Helvetica Neue',Arial,sans-serif\" \
          font-size=\"28\" font-weight=\"900\" fill=\"white\" text-anchor=\"middle\" \
          letter-spacing=\"4\">";
    s += title;
    s += "</text>\n";
    s += "<text x=\"1200\" y=\"90\" font-family=\"Arial,sans-serif\" \
          font-size=\"13\" fill=\"#64748b\" text-anchor=\"middle\" \
          letter-spacing=\"2\">";
    s += subtitle;
    s += "</text>\n";

    // ── Line definitions ──────────────────────────────────────────────────────────
    // Beck layout: 0°/45°/90° only
    // Grid origin approx: Chicago = (1200, 540)
    // West runs left, East runs right
    // North runs up, South runs down

    // Stop-first layout: the ordered stops/anchors define schematic spacing and bends.
    // Long geography is condensed into fixed Beck units; dense interchange space is
    // expanded so transfers and T2 connectors remain legible.
    let stops = beck_stops();
    let lines = t1_line_segments(&stops);
    let t2_lines = t2_line_segments(&stops);

    // ── Draw lines ────────────────────────────────────────────────────────────────
    // Draw halos first, then main lines.
    if !t2_only {
        for line in &lines {
            let color = t1_line_color(line.corridor);
            let d = line.to_svg_path();
            if d.is_empty() {
                continue;
            }
            // Soft halo
            s += &format!(
                "<path d=\"{d}\" stroke=\"{color}\" stroke-width=\"18\" fill=\"none\" \
                 opacity=\"0.12\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n"
            );
            // Main line
            s += &format!(
                "<path d=\"{d}\" stroke=\"{color}\" stroke-width=\"11\" fill=\"none\" \
                 opacity=\"1.0\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n"
            );
            // Subtle center highlight keeps overlapping colors legible without looking glossy.
            s += &format!(
                "<path d=\"{d}\" stroke=\"#f8fafc\" stroke-width=\"1.25\" fill=\"none\" \
                 opacity=\"0.16\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n"
            );
        }
    }

    if expanded || t2_only {
        for line in &t2_lines {
            let points = line.routed_waypoints();
            let d = svg_path(&points);
            if points.len() < 2 || d.is_empty() {
                continue;
            }
            let corridor = line.corridor;
            let halo_width = if t2_only { 13.0 } else { 9.0 };
            let main_width = if t2_only { 7.5 } else { 5.5 };
            let center_width = if t2_only { 1.1 } else { 0.8 };
            let main_opacity = if t2_only { 0.96 } else { 0.88 };
            let start_color = t1_line_color(line.start_trunk);
            let end_color = t1_line_color(line.end_trunk);
            s += &format!(
                "<path data-corridor=\"{corridor}\" d=\"{d}\" stroke=\"#020617\" stroke-width=\"{halo_width}\" fill=\"none\" \
                 opacity=\"0.78\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n"
            );
            if line.is_split_color() {
                let (left, right) = if let Some(split) = t2_split_anchor_distance(line, &stops) {
                    split_polyline_at_distance(&points, polyline_length(&points) * split.fraction)
                } else {
                    split_polyline_at_half(&points)
                };
                let left_d = svg_path(&left);
                let right_d = svg_path(&right);
                s += &format!(
                    "<path data-corridor=\"{corridor}\" data-parent=\"{}\" d=\"{left_d}\" stroke=\"{start_color}\" stroke-width=\"{main_width}\" fill=\"none\" \
                     opacity=\"{main_opacity}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n",
                    line.start_trunk
                );
                s += &format!(
                    "<path data-corridor=\"{corridor}\" data-parent=\"{}\" d=\"{right_d}\" stroke=\"{end_color}\" stroke-width=\"{main_width}\" fill=\"none\" \
                     opacity=\"{main_opacity}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n",
                    line.end_trunk
                );
            } else {
                s += &format!(
                    "<path data-corridor=\"{corridor}\" data-parent=\"{}\" d=\"{d}\" stroke=\"{start_color}\" stroke-width=\"{main_width}\" fill=\"none\" \
                     opacity=\"{main_opacity}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n",
                    line.start_trunk
                );
            }
            s += &format!(
                "<path data-corridor=\"{corridor}\" d=\"{d}\" stroke=\"#f8fafc\" stroke-width=\"{center_width}\" fill=\"none\" \
                opacity=\"0.22\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n"
            );
        }
    }

    if !t2_only {
        for line in &lines {
            draw_generated_bends(
                &mut s,
                &line.waypoints,
                &line.routed_waypoints(),
                t1_line_color(line.corridor),
                4.0,
                0.90,
            );
        }
    }
    if expanded || t2_only {
        for line in &t2_lines {
            draw_generated_bends(
                &mut s,
                &line.waypoints,
                &line.routed_waypoints(),
                if line.is_split_color() {
                    "#f8fafc"
                } else {
                    t1_line_color(line.start_trunk)
                },
                if t2_only { 4.2 } else { 3.2 },
                if t2_only { 0.86 } else { 0.72 },
            );
        }
    }

    if expanded || t2_only {
        for line in &t2_lines {
            let color = t1_line_color(line.start_trunk);
            for stop_id in &line.stop_ids {
                let stop = stop_by_id(&stops, stop_id);
                if stop.draw {
                    continue;
                }
                let (x, y) = stop.point();
                s += &format!(
                    "<circle cx=\"{x}\" cy=\"{y}\" r=\"4.5\" fill=\"#0f1623\" stroke=\"{color}\" stroke-width=\"2\" opacity=\"0.92\"/>\n"
                );
            }
        }
    }

    // ── Draw stations ─────────────────────────────────────────────────────────────
    let stations = beck_stops();
    let t2_stop_ids = t2_lines
        .iter()
        .flat_map(|line| line.stop_ids.iter().copied())
        .collect::<HashSet<_>>();
    for station in stations.iter().filter(|station| station.draw) {
        if t2_only && !t2_stop_ids.contains(station.id) {
            continue;
        }
        let label = station.label;
        let x = station.x;
        let y = station.y;
        if !expanded && label == "Charlotte" {
            continue;
        }

        // Pick primary line color
        let color = station
            .lines
            .first()
            .map(|l| t1_line_color(l))
            .unwrap_or("#94a3b8");

        if is_primary_terminal(station.id) {
            s += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"21\" fill=\"#f8fafc\"/>\n");
            s += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"16\" fill=\"{color}\"/>\n");
            s += &format!(
                "<circle cx=\"{x}\" cy=\"{y}\" r=\"9\" fill=\"#0f1623\" stroke=\"#f8fafc\" stroke-width=\"3\"/>\n"
            );
        } else if station.is_hub {
            // Relay hub: filled circle with white ring — confirmed
            s += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"16\" fill=\"#f8fafc\"/>\n");
            s += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"12\" fill=\"{color}\"/>\n");
            s += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"5\" fill=\"#f8fafc\"/>\n");
        } else if station.is_interchange {
            // T1/T1 interchange: double ring
            s += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"11\" fill=\"#f8fafc\"/>\n");
            s += &format!(
                "<circle cx=\"{x}\" cy=\"{y}\" r=\"7\" fill=\"#0f1623\" stroke=\"{color}\" stroke-width=\"3\"/>\n"
            );
        } else {
            // Regular station: single ring
            s += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"5\" fill=\"#f8fafc\"/>\n");
            s += &format!(
                "<circle cx=\"{x}\" cy=\"{y}\" r=\"3\" fill=\"#0f1623\" stroke=\"{color}\" stroke-width=\"1.5\"/>\n"
            );
        }

        // Label
        let (lx, ly, anchor) = match station.label_dir {
            LabelDir::Right => (x + 18.0, y + 4.0, "start"),
            LabelDir::Left => (x - 18.0, y + 4.0, "end"),
            LabelDir::Up => (x, y - 18.0, "middle"),
            LabelDir::Down => (x, y + 24.0, "middle"),
        };
        let font_size = if t2_only {
            if is_primary_terminal(station.id) || station.is_hub {
                12.5_f64
            } else if station.is_interchange {
                10.5_f64
            } else {
                9.0_f64
            }
        } else if is_primary_terminal(station.id) {
            15.0_f64
        } else if station.is_hub {
            13.0_f64
        } else {
            11.0_f64
        };
        let fill = if station.is_hub || is_primary_terminal(station.id) {
            "white"
        } else if t2_only {
            "#b6c2d1"
        } else {
            "#94a3b8"
        };
        let label_opacity = if t2_only && !station.is_hub && !station.is_interchange {
            0.82
        } else {
            1.0
        };
        s += &format!(
            "<text x=\"{lx:.1}\" y=\"{ly:.1}\" font-family=\"'Helvetica Neue',Arial,sans-serif\" \
             font-size=\"{font_size}\" font-weight=\"{}\" fill=\"{fill}\" opacity=\"{label_opacity}\" \
             text-anchor=\"{anchor}\">{label}</text>\n",
            if station.is_hub || is_primary_terminal(station.id) {
                "800"
            } else {
                "400"
            }
        );
    }

    // ── Line badges ───────────────────────────────────────────────────────────────
    // Metro-style badges reveal the underlying Interstate corridors without making
    // the first read feel like a highway map.
    if !t2_only {
        for (corridor, sx, sy) in t1_badges(&stops) {
            if corridor == "I-85" {
                continue;
            }

            let color = t1_line_color(corridor);
            let label = corridor.trim_start_matches("I-");
            s += &format!(
                "<circle cx=\"{sx}\" cy=\"{sy}\" r=\"18\" fill=\"{color}\" opacity=\"0.95\"/>\n"
            );
            s += &format!("<circle cx=\"{sx}\" cy=\"{sy}\" r=\"18\" fill=\"none\" stroke=\"#f8fafc\" stroke-width=\"1.2\" opacity=\"0.38\"/>\n");
            s += &format!(
                "<text x=\"{sx}\" y=\"{:.1}\" font-family=\"'Helvetica Neue',Arial,sans-serif\" \
                 font-size=\"13\" font-weight=\"900\" fill=\"white\" text-anchor=\"middle\">{label}</text>\n",
                sy + 4.0
            );
        }
    }

    if expanded || t2_only {
        for line in &t2_lines {
            let color = if line.is_split_color() {
                "#f8fafc"
            } else {
                t1_line_color(line.start_trunk)
            };
            let label = line.corridor.trim_start_matches("I-");
            let (sx, sy) = line.badge;
            let r = if t2_only { 15.0 } else { 13.0 };
            let font_size = if t2_only { 11.0 } else { 10.0 };
            s += &format!(
                "<circle cx=\"{sx}\" cy=\"{sy}\" r=\"{r}\" fill=\"#0f1623\" stroke=\"{color}\" stroke-width=\"3\" opacity=\"0.94\"/>\n"
            );
            s += &format!(
                "<text x=\"{sx}\" y=\"{:.1}\" font-family=\"'Helvetica Neue',Arial,sans-serif\" \
                 font-size=\"{font_size}\" font-weight=\"900\" fill=\"#f8fafc\" text-anchor=\"middle\">{label}</text>\n",
                sy + 3.5
            );

            if !t2_only {
                let (x, y) = line.badge;
                let lx = match line.label_anchor {
                    "end" => x - 17.0,
                    "middle" => x,
                    _ => x + 17.0,
                };
                let ly = if line.label_anchor == "middle" {
                    y - 18.0
                } else {
                    y + 3.5
                };
                s += &format!(
                    "<text x=\"{lx:.1}\" y=\"{ly:.1}\" font-family=\"'Helvetica Neue',Arial,sans-serif\" \
                     font-size=\"8.5\" font-weight=\"600\" fill=\"#cbd5e1\" opacity=\"0.78\" \
                     text-anchor=\"{}\">{}</text>\n",
                    line.label_anchor,
                    line.service_label
                );
            }
        }
    }

    // ── Legend ────────────────────────────────────────────────────────────────────
    let lx = 520.0_f64;
    let ly = 118.0_f64;
    s += &format!(
        "<rect x=\"{lx}\" y=\"{ly}\" width=\"1360\" height=\"76\" rx=\"8\" \
         fill=\"#1e2d3d\" fill-opacity=\"0.78\"/>\n"
    );
    s += &format!(
        "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"'Helvetica Neue',Arial,sans-serif\" \
         font-size=\"12\" font-weight=\"700\" fill=\"white\" letter-spacing=\"1\">SYSTEM KEY · NODE CLASSES</text>\n",
        lx + 20.0,
        ly + 28.0
    );

    // Hub types
    let legend_items = vec![
        ("Transfer hub", true, false),
        ("Interchange station", false, true),
        ("Stop", false, false),
    ];
    for (i, (label, is_hub, is_ix)) in legend_items.iter().enumerate() {
        let iy = ly + 52.0;
        let mx = lx + 250.0 + i as f64 * 270.0;
        if *is_hub {
            s += &format!("<circle cx=\"{mx}\" cy=\"{iy}\" r=\"10\" fill=\"white\"/>\n");
            s += &format!("<circle cx=\"{mx}\" cy=\"{iy}\" r=\"8\" fill=\"#3b82f6\"/>\n");
            s += &format!("<circle cx=\"{mx}\" cy=\"{iy}\" r=\"4\" fill=\"white\"/>\n");
        } else if *is_ix {
            s += &format!("<circle cx=\"{mx}\" cy=\"{iy}\" r=\"8\" fill=\"white\" stroke=\"#3b82f6\" stroke-width=\"3\"/>\n");
            s += &format!("<circle cx=\"{mx}\" cy=\"{iy}\" r=\"3\" fill=\"#3b82f6\"/>\n");
        } else {
            s += &format!("<circle cx=\"{mx}\" cy=\"{iy}\" r=\"5\" fill=\"#3b82f6\" stroke=\"white\" stroke-width=\"2\"/>\n");
        }
        s += &format!(
            "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" \
             font-size=\"11\" fill=\"#94a3b8\" dominant-baseline=\"middle\">{label}</text>\n",
            mx + 18.0,
            iy
        );
    }

    if expanded || t2_only {
        let x1 = lx + 915.0;
        let x2 = lx + 960.0;
        let y = ly + 52.0;
        let label = if t2_only {
            "T2 service line"
        } else {
            "T2 connector"
        };
        s += &format!(
            "<path d=\"M {x1:.1} {y:.1} L {x2:.1} {y:.1}\" stroke=\"#f43f5e\" \
             stroke-width=\"{}\" opacity=\"0.72\" stroke-linecap=\"round\"/>\n",
            if t2_only { 7 } else { 5 }
        );
        s += &format!(
            "<text x=\"{:.1}\" y=\"{y:.1}\" font-family=\"Arial,sans-serif\" font-size=\"11\" \
             fill=\"#94a3b8\" dominant-baseline=\"middle\">{label}</text>\n",
            x2 + 16.0
        );
    }

    // Proposed marker
    let py = ly + 52.0;
    s += &format!("<circle cx=\"{:.1}\" cy=\"{py}\" r=\"10\" fill=\"none\" stroke=\"#DAA520\" stroke-width=\"2\" stroke-dasharray=\"4,3\"/>\n", lx+1060.0);
    s += &format!(
        "<text x=\"{:.1}\" y=\"{py}\" font-family=\"Arial,sans-serif\" font-size=\"11\" \
         fill=\"#94a3b8\" dominant-baseline=\"middle\">Future stop (*)</text>\n",
        lx + 1078.0
    );

    s += &format!(
        "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" font-size=\"9\" \
         fill=\"#64748b\" text-anchor=\"end\">NOT GEOGRAPHICALLY ACCURATE</text>\n",
        lx + 1338.0,
        ly + 28.0
    );

    // ── Footer ────────────────────────────────────────────────────────────────────
    let footer = if t2_only {
        "T2-only service view: connectors are shown without the bold T1 trunk layer so local transfer geometry is easier to inspect."
    } else {
        "Metro first, highway second: line numbers follow priority Interstate corridors; transfer hubs are national freight relay nodes. Inspired by H. Beck 1933."
    };
    s += &format!(
        "<text x=\"1200\" y=\"1320\" font-family=\"Arial,sans-serif\" font-size=\"11\" \
          fill=\"#475569\" text-anchor=\"middle\">{footer}</text>\n"
    );

    s += "</svg>\n";
    s
}

fn draw_generated_bends(
    s: &mut String,
    original: &[(f64, f64)],
    routed: &[(f64, f64)],
    color: &str,
    radius: f64,
    opacity: f64,
) {
    for point in routed {
        if original
            .iter()
            .any(|original| same_point(*original, *point))
        {
            continue;
        }
        let (x, y) = *point;
        s.push_str(&format!(
            "<circle cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"{radius}\" fill=\"#0f1623\" stroke=\"{color}\" stroke-width=\"2\" opacity=\"{opacity}\"/>\n"
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::{
        beck_stop_class, beck_stops, beck_t2_diagnostics, build_beck_stop_sla_csv, build_beck_svg,
        build_beck_t2_diagnostics_csv, build_beck_t2_only_svg, build_beck_t2_svg,
        is_primary_terminal, stop_by_id, stop_class, t1_badges, t1_line_segments,
        t1_route_stop_ids, t2_line_segments, LineSegment,
    };
    use route_network::{minimum_system_contacts_for_tier, RouteTier};

    fn path_contains_point(line: &LineSegment, point: (f64, f64)) -> bool {
        line.waypoints.windows(2).any(|segment| {
            let (ax, ay) = segment[0];
            let (bx, by) = segment[1];
            let (px, py) = point;
            let cross = (px - ax) * (by - ay) - (py - ay) * (bx - ax);
            let within_x = px >= ax.min(bx) && px <= ax.max(bx);
            let within_y = py >= ay.min(by) && py <= ay.max(by);
            cross.abs() < 0.001 && within_x && within_y
        })
    }

    fn distance_to_segment(point: (f64, f64), a: (f64, f64), b: (f64, f64)) -> f64 {
        let (px, py) = point;
        let (ax, ay) = a;
        let (bx, by) = b;
        let dx = bx - ax;
        let dy = by - ay;
        let len2 = dx * dx + dy * dy;
        if len2 == 0.0 {
            return ((px - ax).powi(2) + (py - ay).powi(2)).sqrt();
        }
        let t = (((px - ax) * dx + (py - ay) * dy) / len2).clamp(0.0, 1.0);
        let qx = ax + t * dx;
        let qy = ay + t * dy;
        ((px - qx).powi(2) + (py - qy).powi(2)).sqrt()
    }

    fn segment_is_octilinear(a: (f64, f64), b: (f64, f64)) -> bool {
        let dx = (b.0 - a.0).abs();
        let dy = (b.1 - a.1).abs();
        dx < 0.001 || dy < 0.001 || (dx - dy).abs() < 0.001
    }

    #[test]
    fn beck_svg_keeps_jacksonville_as_i10_i95_transfer_node() {
        let stops = beck_stops();
        let jax = stop_by_id(&stops, "JAX").point();
        let lines = t1_line_segments(&stops);
        let i10 = lines.iter().find(|line| line.corridor == "I-10").unwrap();
        let i95 = lines.iter().find(|line| line.corridor == "I-95").unwrap();

        assert!(path_contains_point(i10, jax));
        assert!(path_contains_point(i95, jax));

        let svg = build_beck_svg();
        assert!(svg.contains("Jacksonville"));
        assert!(svg.contains(&format!("{:.1} {:.1}", jax.0, jax.1)));
    }

    #[test]
    fn beck_svg_keeps_i40_connected_to_i95_in_north_carolina() {
        let stops = beck_stops();
        let benson = stop_by_id(&stops, "BEN").point();
        let lines = t1_line_segments(&stops);
        let i40 = lines.iter().find(|line| line.corridor == "I-40").unwrap();
        let i95 = lines.iter().find(|line| line.corridor == "I-95").unwrap();

        assert!(path_contains_point(i40, benson));
        assert!(path_contains_point(i95, benson));

        let svg = build_beck_svg();
        assert!(svg.contains("Benson"));
        assert!(svg.contains(&format!("{:.1} {:.1}", benson.0, benson.1)));
    }

    #[test]
    fn beck_svg_keeps_chicago_as_i69_i80_i90_transfer_node() {
        let stops = beck_stops();
        let chicago = stop_by_id(&stops, "CHI").point();
        let lines = t1_line_segments(&stops);
        let i69 = lines.iter().find(|line| line.corridor == "I-69").unwrap();
        let i80 = lines.iter().find(|line| line.corridor == "I-80").unwrap();
        let i90 = lines.iter().find(|line| line.corridor == "I-90").unwrap();

        assert!(path_contains_point(i69, chicago));
        assert!(path_contains_point(i80, chicago));
        assert!(path_contains_point(i90, chicago));

        let svg = build_beck_svg();
        assert!(svg.contains("Chicago"));
        assert!(svg.contains(&format!("{:.1} {:.1}", chicago.0, chicago.1)));
    }

    #[test]
    fn beck_svg_keeps_des_moines_and_okc_as_central_transfer_nodes() {
        let des_moines = (840.0, 480.0);
        let okc = (840.0, 780.0);
        let i35 = LineSegment {
            corridor: "I-35",
            waypoints: vec![
                (840.0, 220.0),
                des_moines,
                (840.0, 620.0),
                okc,
                (840.0, 960.0),
                (780.0, 1080.0),
            ],
        };
        let i80 = LineSegment {
            corridor: "I-80",
            waypoints: vec![
                (180.0, 680.0),
                (240.0, 620.0),
                (360.0, 540.0),
                (540.0, 540.0),
                (720.0, 540.0),
                des_moines,
                (1200.0, 480.0),
                (1320.0, 420.0),
                (1500.0, 420.0),
                (1680.0, 480.0),
                (1920.0, 600.0),
            ],
        };
        let i40 = LineSegment {
            corridor: "I-40",
            waypoints: vec![
                (240.0, 900.0),
                (420.0, 900.0),
                (600.0, 900.0),
                (720.0, 840.0),
                okc,
                (960.0, 900.0),
                (1080.0, 840.0),
                (1260.0, 840.0),
                (1440.0, 840.0),
                (1620.0, 840.0),
                (1740.0, 960.0),
                (1800.0, 960.0),
                (1800.0, 1020.0),
            ],
        };

        assert!(path_contains_point(&i35, des_moines));
        assert!(path_contains_point(&i80, des_moines));
        assert!(path_contains_point(&i35, okc));
        assert!(path_contains_point(&i40, okc));

        let svg = build_beck_svg();
        assert!(svg.contains("Des Moines"));
        assert!(svg.contains("Oklahoma City"));
    }

    #[test]
    fn beck_svg_connects_i40_to_la_gateway_via_barstow() {
        let stops = beck_stops();
        let barstow = stop_by_id(&stops, "BARSTOW");
        let lines = t1_line_segments(&stops);
        let t2_lines = t2_line_segments(&stops);
        let i40 = lines.iter().find(|line| line.corridor == "I-40").unwrap();
        let i15 = t2_lines
            .iter()
            .find(|line| line.corridor == "I-15")
            .unwrap();

        assert!(barstow.is_hub);
        assert!(barstow.is_interchange);
        assert!(path_contains_point(i40, barstow.point()));
        assert!(i15.stop_ids.contains(&"LA"));
        assert!(i15.stop_ids.contains(&"BARSTOW"));

        let svg = build_beck_t2_svg();
        assert!(svg.contains("Barstow"));
        assert!(svg.contains(">40</text>"));
        assert!(svg.contains("data-corridor=\"I-15\""));
    }

    #[test]
    fn beck_svg_keeps_t1_national_terminals_visible() {
        let stops = beck_stops();
        let lines = t1_line_segments(&stops);
        let terminal_routes = [
            ("I-5", "BLAINE"),
            ("I-5", "SDTJ"),
            ("I-35", "LRD"),
            ("I-75", "MIA"),
        ];
        for (corridor, stop_id) in terminal_routes {
            let stop = stop_by_id(&stops, stop_id);
            let line = lines.iter().find(|line| line.corridor == corridor).unwrap();
            assert!(stop.draw, "{stop_id} should be visible");
            assert!(
                is_primary_terminal(stop.id),
                "{stop_id} should be a terminal"
            );
            assert!(path_contains_point(line, stop.point()));
        }
    }

    #[test]
    fn beck_svg_promotes_raleigh_as_carolina_transfer_hub() {
        let stops = beck_stops();
        let raleigh = stop_by_id(&stops, "RALEIGH_APPROACH");
        let lines = t1_line_segments(&stops);
        let i40 = lines.iter().find(|line| line.corridor == "I-40").unwrap();

        assert!(raleigh.is_hub);
        assert!(raleigh.is_interchange);
        assert!(path_contains_point(i40, raleigh.point()));

        let svg = build_beck_svg();
        assert!(svg.contains("Raleigh"));
    }

    #[test]
    fn beck_svg_keeps_denver_on_i70_primary_trunk() {
        let stops = beck_stops();
        let denver = stop_by_id(&stops, "DEN");
        let salt_lake = stop_by_id(&stops, "SLC");
        let lines = t1_line_segments(&stops);
        let i70 = lines.iter().find(|line| line.corridor == "I-70").unwrap();
        let i80 = lines.iter().find(|line| line.corridor == "I-80").unwrap();

        assert!(denver.is_hub);
        assert!(denver.is_interchange);
        assert!(salt_lake.is_hub);
        assert!(salt_lake.is_interchange);
        assert!(path_contains_point(i70, denver.point()));
        assert!(path_contains_point(i70, salt_lake.point()));
        assert!(path_contains_point(i80, salt_lake.point()));

        let svg = build_beck_svg();
        assert!(svg.contains("Denver"));
        assert!(svg.contains("Salt Lake"));
        assert!(svg.contains(">70</text>"));
    }

    #[test]
    fn beck_svg_promotes_interior_s2_transfer_hubs() {
        let stops = beck_stops();
        let expected_hubs = [
            "SLC",
            "DEN",
            "KC",
            "OKC",
            "DAL",
            "SAT",
            "MEM",
            "STL",
            "NASHVILLE",
            "BHM_APPROACH",
            "INDY_APPROACH",
            "COLUMBUS",
            "CINCINNATI",
            "OMAHA",
            "MIN",
            "BIL",
            "BOI",
            "HOU",
            "ELP",
            "CLT",
            "BARSTOW",
            "LOUISVILLE",
            "RAPID_CITY",
            "PENNSYLVANIA",
            "LITTLE_ROCK",
            "GULFPORT",
            "CORPUS",
        ];
        for id in expected_hubs {
            let stop = stop_by_id(&stops, id);
            assert!(stop.draw, "{id} should be visible");
            assert!(stop.is_hub, "{id} should render as a transfer hub");
            assert!(stop.is_interchange, "{id} should render as an interchange");
            assert!(!stop.label.is_empty(), "{id} should have a label");
        }
    }

    #[test]
    fn beck_svg_draws_t1_southeast_mesh_without_routing_everything_through_new_york() {
        let dallas = (840.0, 960.0);
        let atlanta = (1440.0, 960.0);
        let richmond = (1860.0, 960.0);
        let florence = (1800.0, 960.0);

        let i20 = LineSegment {
            corridor: "I-20",
            waypoints: vec![
                dallas,
                (1020.0, 960.0),
                (1200.0, 960.0),
                atlanta,
                (1620.0, 960.0),
                florence,
            ],
        };
        let i95 = LineSegment {
            corridor: "I-95",
            waypoints: vec![
                (1920.0, 360.0),
                (1920.0, 480.0),
                (1920.0, 600.0),
                (1920.0, 720.0),
                (1920.0, 840.0),
                richmond,
                florence,
                (1800.0, 1020.0),
                (1800.0, 1080.0),
            ],
        };

        assert!(path_contains_point(&i20, dallas));
        assert!(path_contains_point(&i20, atlanta));
        assert!(path_contains_point(&i20, florence));
        assert!(path_contains_point(&i95, florence));

        let svg = build_beck_svg();
        for label in ["Dallas", "Atlanta", "Florence", "Richmond"] {
            assert!(svg.contains(label));
        }
        assert!(svg.contains(">20</text>"));
        assert!(!svg.contains(">85</text>"));
    }

    #[test]
    fn beck_svg_promotes_s3_transfer_stops_to_named_visible_nodes() {
        let stops = beck_stops();
        let expected_visible = [
            "ABQ",
            "COLUMBIA",
            "MOBILE",
            "LAS_VEGAS",
            "CHEYENNE",
            "TAMPA",
            "GREENVILLE",
            "GREENSBORO",
            "JOPLIN",
            "ROANOKE",
            "HAGERSTOWN",
            "PITTSBURGH",
            "BEAUMONT",
            "ASHEVILLE",
            "LEWISTON",
            "NEW_HAVEN",
            "COLORADO_SPRINGS",
            "LAFAYETTE",
            "ABILENE",
            "MONTGOMERY",
            "FARGO",
            "CAPITAL_BELTWAY_E",
        ];
        for id in expected_visible {
            let stop = stop_by_id(&stops, id);
            assert!(stop.draw, "{id} should be visible");
            assert!(!stop.label.is_empty(), "{id} should have a label");
        }

        let i65 = t2_line_segments(&stops)
            .into_iter()
            .find(|line| line.corridor == "I-65")
            .unwrap();
        assert_eq!(
            i65.stop_ids,
            vec![
                "MOBILE",
                "BHM_APPROACH",
                "NASHVILLE",
                "LOUISVILLE",
                "CINCINNATI"
            ]
        );
    }

    #[test]
    fn beck_t2_appalachian_and_pennsylvania_services_use_named_corridors() {
        let stops = beck_stops();
        let routes = t2_line_segments(&stops);
        let i81 = routes.iter().find(|line| line.corridor == "I-81").unwrap();
        let i76 = routes.iter().find(|line| line.corridor == "I-76").unwrap();

        assert_eq!(
            i81.stop_ids,
            vec![
                "KNX",
                "WYTHEVILLE",
                "ROANOKE",
                "HAGERSTOWN",
                "PENNSYLVANIA",
                "NYC"
            ]
        );
        assert_eq!(
            i76.stop_ids,
            vec!["CLEVELAND", "PITTSBURGH", "PENNSYLVANIA", "PHILADELPHIA"]
        );
    }

    #[test]
    fn beck_t2_svg_draws_thin_connector_overlay_tinted_to_trunks() {
        let svg = build_beck_t2_svg();
        assert!(svg.contains("LOCAL SERVICES"));
        assert!(svg.contains("25 THIN T2 CONNECTORS"));
        assert!(svg.contains("T2 connector"));
        assert!(svg.contains("stroke-width=\"5.5\""));
        assert!(svg.contains("Charlotte"));
        assert!(svg.contains("NODE CLASSES"));
        let t2_routes = [
            "US287", "I-85", "I-25", "I-15", "I-84", "US95", "I-65", "I-495", "I-59", "I-24",
            "I-30", "I-49", "I-81", "I-44", "I-77", "I-76", "US30", "US6", "US70", "US90", "I-22",
            "I-29", "US80", "US83", "I-37",
        ];
        for route in t2_routes {
            assert!(svg.contains(&format!("data-corridor=\"{route}\"")));
            let label = route.trim_start_matches("I-");
            assert!(svg.contains(&format!(">{label}</text>")));
        }
        for held_route in ["I-405", "I-610", "US2", "I-285"] {
            assert!(!svg.contains(&format!("data-corridor=\"{held_route}\"")));
        }
        assert!(svg.matches("stroke-width=\"5.5\"").count() > t2_routes.len());
        assert!(svg.contains("data-parent=\"I-20\""));
        assert!(svg.contains("data-parent=\"I-95\""));
    }

    #[test]
    fn beck_t2_only_svg_draws_service_connectors_as_primary_map() {
        let svg = build_beck_t2_only_svg();
        assert!(svg.contains("T2 SERVICE MAP"));
        assert!(svg.contains("T2 service line"));
        assert!(svg.contains("stroke-width=\"7.5\""));
        assert!(svg.contains("data-corridor=\"I-15\""));
        assert!(!svg.contains("Inland West"));
        assert!(!svg.contains("PRIMARY FREIGHT LINES"));
    }

    #[test]
    fn beck_t2_diagnostics_exports_review_flags() {
        let csv = build_beck_t2_diagnostics_csv();
        assert!(csv.starts_with("corridor,trunk,start_trunk,end_trunk,color_mode,split_anchor"));
        assert!(csv.contains("I-495,I-95,I-95,I-95,single-parent,"));
        assert!(csv.contains("split-parent"));
        assert!(csv.contains("long-connector-review"));
        assert!(csv.contains("dense-label-review"));

        let rows = beck_t2_diagnostics();
        assert_eq!(rows.len(), t2_line_segments(&beck_stops()).len());
        assert!(rows.iter().any(|row| row.corridor == "US70"));
        assert!(rows
            .iter()
            .any(|row| row.color_mode == "split-parent" && !row.split_anchor.is_empty()));
    }

    #[test]
    fn beck_paths_resolve_from_stop_catalog() {
        let stops = beck_stops();
        let lines = t1_line_segments(&stops);
        let t2_lines = t2_line_segments(&stops);

        let chicago = stops.iter().find(|stop| stop.id == "CHI").unwrap().point();
        let i80 = lines.iter().find(|line| line.corridor == "I-80").unwrap();
        let i90 = lines.iter().find(|line| line.corridor == "I-90").unwrap();

        assert!(path_contains_point(i80, chicago));
        assert!(path_contains_point(i90, chicago));
        assert!(t2_lines.iter().all(|line| line.waypoints.len() >= 2));
        assert!(t2_lines.iter().all(|line| !line.service_label.is_empty()));
        assert!(t2_lines.iter().all(|line| !line.label_anchor.is_empty()));
        assert_eq!(t2_lines.len(), 25);
        assert_eq!(t1_badges(&stops).len(), lines.len());
    }

    #[test]
    fn beck_stop_catalog_has_no_hidden_routing_nodes() {
        let stops = beck_stops();
        let hidden = stops
            .iter()
            .filter(|stop| !stop.draw || stop.label.is_empty())
            .map(|stop| stop.id)
            .collect::<Vec<_>>();
        assert!(
            hidden.is_empty(),
            "hidden or unlabeled Beck nodes: {hidden:?}"
        );

        for line in t2_line_segments(&stops) {
            for stop_id in &line.stop_ids {
                let stop = stop_by_id(&stops, stop_id);
                assert!(stop.draw, "{} uses hidden stop {stop_id}", line.corridor);
                assert!(
                    !stop.label.is_empty(),
                    "{} uses unlabeled stop {stop_id}",
                    line.corridor
                );
            }
        }
    }

    #[test]
    fn t2_services_keep_at_least_two_system_contact_stops() {
        let stops = beck_stops();
        let t1_ids = t1_route_stop_ids()
            .into_iter()
            .flat_map(|(_corridor, ids)| ids)
            .collect::<std::collections::HashSet<_>>();
        for line in t2_line_segments(&stops) {
            let contacts = line
                .stop_ids
                .iter()
                .filter(|stop_id| {
                    let stop = stop_by_id(&stops, stop_id);
                    let contact_tier = if t1_ids.contains(**stop_id) {
                        RouteTier::T1
                    } else {
                        RouteTier::T2
                    };
                    beck_stop_class(stop).qualifies_for_route_contact(contact_tier)
                })
                .count();
            let minimum_contacts = minimum_system_contacts_for_tier(RouteTier::T2);
            assert!(
                contacts >= minimum_contacts,
                "{} has only {contacts} system contact stop(s)",
                line.corridor
            );
        }
    }

    #[test]
    fn t1_routes_start_and_end_at_qualified_endpoint_stops() {
        let stops = beck_stops();
        for (corridor, ids) in t1_route_stop_ids() {
            let endpoint_ids = [ids[0], ids[ids.len() - 1]];
            for endpoint_id in endpoint_ids {
                let endpoint = stop_by_id(&stops, endpoint_id);
                assert!(
                    beck_stop_class(endpoint).qualifies_for_route_endpoint(RouteTier::T1),
                    "{corridor} endpoint {endpoint_id} is classified as {}",
                    stop_class(endpoint)
                );
            }
        }
    }

    #[test]
    fn generated_beck_paths_are_octilinear() {
        let stops = beck_stops();
        for line in t1_line_segments(&stops) {
            for segment in line.routed_waypoints().windows(2) {
                assert!(
                    segment_is_octilinear(segment[0], segment[1]),
                    "{} has non-Beck segment {:?}",
                    line.corridor,
                    segment
                );
            }
        }
        for line in t2_line_segments(&stops) {
            for segment in line.routed_waypoints().windows(2) {
                assert!(
                    segment_is_octilinear(segment[0], segment[1]),
                    "{} has non-Beck segment {:?}",
                    line.corridor,
                    segment
                );
            }
        }
    }

    #[test]
    fn t1_bends_happen_at_drawn_stops() {
        let stops = beck_stops();
        let mut hidden_bends = Vec::new();
        for (corridor, ids) in t1_route_stop_ids() {
            for window in ids.windows(3) {
                let a = stop_by_id(&stops, window[0]).point();
                let b_stop = stop_by_id(&stops, window[1]);
                let b = b_stop.point();
                let c = stop_by_id(&stops, window[2]).point();
                let ab = (b.0 - a.0, b.1 - a.1);
                let bc = (c.0 - b.0, c.1 - b.1);
                let cross = ab.0 * bc.1 - ab.1 * bc.0;
                if cross.abs() > 0.001 {
                    if !b_stop.draw {
                        hidden_bends.push(format!("{corridor}:{}", b_stop.id));
                    }
                }
            }
        }
        assert!(hidden_bends.is_empty(), "hidden T1 bends: {hidden_bends:?}");
    }

    #[test]
    fn t2_bends_make_visible_expanded_stops() {
        let stops = beck_stops();
        let svg = build_beck_t2_svg();
        for line in t2_line_segments(&stops) {
            for window in line.stop_ids.windows(3) {
                let a = stop_by_id(&stops, window[0]).point();
                let b_stop = stop_by_id(&stops, window[1]);
                let b = b_stop.point();
                let c = stop_by_id(&stops, window[2]).point();
                let ab = (b.0 - a.0, b.1 - a.1);
                let bc = (c.0 - b.0, c.1 - b.1);
                let cross = ab.0 * bc.1 - ab.1 * bc.0;
                if cross.abs() > 0.001 && !b_stop.draw {
                    assert!(
                        svg.contains(&format!("cx=\"{}\" cy=\"{}\"", b.0, b.1)),
                        "{} bends at hidden T2 stop {} that is not rendered",
                        line.corridor,
                        b_stop.id
                    );
                }
            }
        }
    }

    #[test]
    fn t2_endpoints_do_not_close_miss_t1_trunks() {
        let stops = beck_stops();
        let t1_lines = t1_line_segments(&stops);
        for line in t2_line_segments(&stops) {
            let endpoint_ids = [line.stop_ids[0], line.stop_ids[line.stop_ids.len() - 1]];
            for endpoint_id in endpoint_ids {
                let endpoint = stop_by_id(&stops, endpoint_id).point();
                let min_distance = t1_lines
                    .iter()
                    .flat_map(|t1| t1.waypoints.windows(2))
                    .map(|segment| distance_to_segment(endpoint, segment[0], segment[1]))
                    .fold(f64::INFINITY, f64::min);
                assert!(
                    min_distance < 0.001 || min_distance >= 70.0,
                    "{} endpoint {endpoint_id} is {min_distance:.1}px from a T1 trunk without touching it",
                    line.corridor
                );
            }
        }
    }

    #[test]
    fn beck_stop_sla_surface_exports_stop_pair_contracts() {
        let csv = build_beck_stop_sla_csv();

        assert!(csv.starts_with("origin_id,origin_label,origin_tier"));
        assert!(csv.contains("MIA,Miami"));
        assert!(csv.contains("NYC,New York"));
        assert!(csv.contains("LA,Los Angeles"));
        assert!(csv.contains("DEN,Denver"));
        assert!(csv.contains("SLC,Salt Lake"));
        assert!(csv.contains("heuristic-planning"));
        assert!(csv.contains("freight_full_i20_p95_h"));
        assert!(csv.contains("passenger_competitive_with_air"));
    }

    #[test]
    fn beck_stop_sla_surface_keeps_long_haul_paths_auditable() {
        let csv = build_beck_stop_sla_csv();
        let la_den = csv
            .lines()
            .find(|line| {
                (line.starts_with("LA,Los Angeles") && line.contains(",DEN,Denver"))
                    || (line.starts_with("DEN,Denver") && line.contains(",LA,Los Angeles"))
            })
            .expect("LA to Denver SLA row");

        assert!(la_den.contains("sla-stop-rhythm-ok"));
        assert!(la_den.contains("not-time-competitive"));
        assert!(la_den.contains("freight-rail-wins-bulk-road-wins-express"));
    }

    #[test]
    fn beck_stop_sla_surface_has_no_oversized_stop_gaps() {
        let csv = build_beck_stop_sla_csv();
        assert!(!csv.contains("needs-intermediate-stops"));
        assert!(!csv.contains("long-gap-review"));
        assert!(csv.contains("sla-stop-rhythm-ok"));
    }
}
