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
    service_label: &'static str,
    badge: (f64, f64),
    label_anchor: &'static str,
    stop_ids: Vec<&'static str>,
    waypoints: Vec<(f64, f64)>,
    lane_shift: (f64, f64),
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
    fn to_svg_path(&self) -> String {
        svg_path(&self.routed_waypoints())
    }

    fn routed_waypoints(&self) -> Vec<(f64, f64)> {
        apply_lane_shift(octilinear_path(&self.waypoints), self.lane_shift)
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

fn anchor(id: &'static str, x: f64, y: f64) -> BeckStop {
    BeckStop {
        id,
        label: "",
        x,
        y,
        is_hub: false,
        is_interchange: false,
        draw: false,
        lines: &[],
        label_dir: LabelDir::Right,
    }
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
        anchor("BEAUMONT", 1080.0, 1020.0),
        minor_stop(
            "NOLA",
            "New Orleans",
            1260.0,
            1020.0,
            &["I-10"],
            LabelDir::Down,
        ),
        anchor("GULF_COAST", 1440.0, 1020.0),
        minor_stop(
            "TALLAHASSEE",
            "Tallahassee",
            1620.0,
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
        anchor("OKC_APPROACH", 720.0, 840.0),
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
        anchor("ASHEVILLE", 1440.0, 840.0),
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
        anchor("LEWISTON", 330.0, 320.0),
        minor_stop(
            "CHEYENNE",
            "Cheyenne",
            540.0,
            540.0,
            &["I-80", "I-25"],
            LabelDir::Up,
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
        minor_stop(
            "CLEVELAND",
            "Cleveland",
            1500.0,
            420.0,
            &["I-80", "I-77"],
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
            false,
            &["I-90"],
            LabelDir::Down,
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
            "MILWAUKEE",
            "Milwaukee",
            1020.0,
            300.0,
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
        anchor("NEW_HAVEN", 1920.0, 480.0),
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
            false,
            &["I-95"],
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
        anchor("GREENVILLE", 1500.0, 900.0),
        anchor("GREENSBORO", 1740.0, 900.0),
        minor_stop(
            "MOBILE",
            "Mobile",
            1260.0,
            1020.0,
            &["I-10", "I-65"],
            LabelDir::Down,
        ),
        anchor("BIRMINGHAM", 1200.0, 960.0),
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
        anchor("TEXARKANA", 930.0, 900.0),
        minor_stop(
            "LAS_VEGAS",
            "Las Vegas",
            300.0,
            960.0,
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
        anchor("COLORADO_SPRINGS", 540.0, 720.0),
        anchor("US287_PLAINS", 660.0, 780.0),
        minor_stop(
            "CAPITAL_BELTWAY_N",
            "Capital North",
            1920.0,
            780.0,
            &["I-95", "I-495"],
            LabelDir::Right,
        ),
        anchor("CAPITAL_BELTWAY_E", 1980.0, 840.0),
        minor_stop(
            "CAPITAL_BELTWAY_S",
            "Capital South",
            1890.0,
            900.0,
            &["I-95", "I-495"],
            LabelDir::Right,
        ),
        anchor("I59_TIE", 1380.0, 840.0),
        anchor("OZARKS", 900.0, 780.0),
        anchor("SHENANDOAH_W", 1440.0, 720.0),
        anchor("SHENANDOAH_E", 1620.0, 720.0),
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
        anchor("APPALACHIAN_NORTH", 1500.0, 660.0),
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
        anchor("US30_W", 360.0, 600.0),
        anchor("US30_C", 720.0, 600.0),
        anchor("US6_W", 360.0, 660.0),
        anchor("US6_C", 720.0, 660.0),
        anchor("US6_E", 1200.0, 600.0),
        anchor("US70_W", 420.0, 960.0),
        anchor("US70_C", 840.0, 840.0),
        anchor("US70_E", 1260.0, 900.0),
        anchor("US90_W", 780.0, 1140.0),
        anchor("US90_C", 1260.0, 1080.0),
        anchor("US90_E", 1800.0, 1140.0),
        anchor("I22_C", 1200.0, 960.0),
        anchor("I29_W", 720.0, 540.0),
        anchor("I29_C", 780.0, 480.0),
        anchor("US80_W", 840.0, 1020.0),
        anchor("US80_C", 1200.0, 1020.0),
        anchor("US83_N", 720.0, 220.0),
        anchor("US83_C", 720.0, 620.0),
        anchor("I37_S", 780.0, 1140.0),
        anchor("I37_E", 840.0, 1140.0),
    ];
    generate_geo_aware_beck_grid(&mut stops);
    promote_t1_bend_anchors(&mut stops);
    stops
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
        "SAC" => (38.58, -121.49),
        "CENTRAL_VALLEY" => (36.74, -119.79),
        "LA" => (34.05, -118.24),
        "SDTJ" => (32.55, -117.05),
        "PHX" => (33.45, -112.07),
        "ELP" => (31.76, -106.49),
        "SAT" => (29.42, -98.49),
        "LRD" => (27.51, -99.51),
        "HOU" => (29.76, -95.37),
        "BEAUMONT" => (30.08, -94.13),
        "NOLA" => (29.95, -90.07),
        "GULF_COAST" => (30.42, -87.22),
        "TALLAHASSEE" => (30.44, -84.28),
        "JAX" => (30.33, -81.66),
        "MIA" => (25.76, -80.19),
        "DAL" => (32.78, -96.80),
        "SHV" => (32.53, -93.75),
        "BHM_APPROACH" | "BIRMINGHAM" | "I22_C" => (33.52, -86.80),
        "ATL" => (33.75, -84.39),
        "COLUMBIA" => (34.00, -81.03),
        "FLO" => (34.20, -79.76),
        "MIN" => (44.98, -93.27),
        "DSM" => (41.59, -93.62),
        "KC" => (39.10, -94.58),
        "OKC" | "OKC_APPROACH" => (35.47, -97.52),
        "BARSTOW" => (34.90, -117.02),
        "ABQ" => (35.08, -106.65),
        "AMA" => (35.22, -101.83),
        "MEM" => (35.15, -90.05),
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
        "LOUISVILLE" => (38.22, -85.75),
        "CHATTANOOGA" | "I59_TIE" => (35.05, -85.31),
        "GAINESVILLE" => (29.65, -82.32),
        "TAMPA" => (27.95, -82.46),
        "RENO" => (39.53, -119.81),
        "SLC" => (40.76, -111.89),
        "BOI" => (43.62, -116.20),
        "LEWISTON" => (46.42, -117.02),
        "CHEYENNE" => (41.14, -104.82),
        "OMAHA" | "I29_W" => (41.26, -95.93),
        "CLEVELAND" => (41.50, -81.69),
        "PA_APPROACH" => (41.20, -77.19),
        "PENNSYLVANIA" => (40.27, -76.88),
        "NYC" => (40.71, -74.01),
        "SPK" => (47.66, -117.43),
        "BIL" => (45.78, -108.50),
        "RAPID_CITY" => (44.08, -103.23),
        "MILWAUKEE" => (43.04, -87.91),
        "CHI_NORTH" => (42.03, -87.75),
        "DET" => (42.33, -83.05),
        "BUFFALO" => (42.89, -78.87),
        "ALBANY" => (42.65, -73.76),
        "BOS" => (42.36, -71.06),
        "NEW_HAVEN" => (41.31, -72.93),
        "PHILADELPHIA" => (39.95, -75.17),
        "DC" => (38.90, -77.04),
        "RIC" => (37.54, -77.43),
        "BEN" => (35.38, -78.55),
        "WIC" => (37.69, -97.34),
        "GREENVILLE" => (34.85, -82.40),
        "GREENSBORO" => (36.07, -79.79),
        "MOBILE" => (30.69, -88.04),
        "TEXARKANA" => (33.43, -94.05),
        "LAS_VEGAS" => (36.17, -115.14),
        "DEN" => (39.74, -104.99),
        "COLORADO_SPRINGS" => (38.83, -104.82),
        "US287_PLAINS" => (37.00, -102.00),
        "CAPITAL_BELTWAY_N" => (39.03, -77.08),
        "CAPITAL_BELTWAY_E" => (38.90, -76.90),
        "CAPITAL_BELTWAY_S" => (38.79, -77.04),
        "OZARKS" => (36.19, -94.13),
        "SHENANDOAH_W" => (38.07, -79.10),
        "SHENANDOAH_E" => (38.03, -78.48),
        "STL" => (38.63, -90.20),
        "APPALACHIAN_NORTH" => (39.63, -79.96),
        "US30_W" => (41.14, -100.76),
        "US30_C" => (41.14, -95.93),
        "US6_W" => (39.60, -110.81),
        "US6_C" => (40.86, -96.68),
        "US6_E" => (41.70, -86.25),
        "US70_W" => (35.20, -111.65),
        "US70_C" => (35.47, -97.52),
        "US70_E" => (35.05, -85.31),
        "US90_W" => (29.43, -98.49),
        "US90_C" => (30.45, -91.15),
        "US90_E" => (30.33, -81.66),
        "I29_C" => (41.59, -93.62),
        "US80_W" => (32.35, -95.30),
        "US80_C" => (32.50, -89.50),
        "US83_N" => (44.08, -103.23),
        "US83_C" => (39.00, -100.00),
        "I37_S" => (27.80, -97.40),
        "I37_E" => (28.00, -96.80),
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
    trunk: &'static str,
    ids: &'static [&'static str],
    service_label: &'static str,
    badge_stop: &'static str,
    badge_offset: (f64, f64),
    label_anchor: &'static str,
) -> T2LineSegment {
    let badge_base = point(stops, badge_stop);
    T2LineSegment {
        corridor,
        trunk,
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
                "PHX",
                "ELP",
                "SAT",
                "HOU",
                "BEAUMONT",
                "NOLA",
                "MOBILE",
                "GULF_COAST",
                "TALLAHASSEE",
                "JAX",
            ],
        ),
        (
            "I-20",
            vec!["DAL", "SHV", "BHM_APPROACH", "ATL", "COLUMBIA", "FLO"],
        ),
        ("I-35", vec!["MIN", "DSM", "KC", "OKC", "DAL", "SAT", "LRD"]),
        (
            "I-40",
            vec![
                "BARSTOW",
                "ABQ",
                "AMA",
                "OKC_APPROACH",
                "OKC",
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
                "DEN",
                "KC",
                "STL",
                "INDY_APPROACH",
                "COLUMBUS",
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
                "KNX",
                "CHATTANOOGA",
                "ATL",
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
                "CHEYENNE",
                "OMAHA",
                "DSM",
                "CHI",
                "TOL",
                "CLEVELAND",
                "PA_APPROACH",
                "NYC",
            ],
        ),
        (
            "I-90",
            vec![
                "SEA",
                "SPK",
                "BIL",
                "RAPID_CITY",
                "MIN",
                "MILWAUKEE",
                "CHI_NORTH",
                "CHI",
                "TOL",
                "BUFFALO",
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
                "FLO",
                "BEN",
                "JAX",
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
            "I-75",
            &["MOBILE", "BHM_APPROACH", "NASHVILLE", "LOUISVILLE", "CINCINNATI"],
            "Birmingham",
            "BHM_APPROACH",
            (-15.0, -30.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "I-24",
            "I-40",
            &["MEM", "NASHVILLE", "KNX"],
            "Nashville",
            "NASHVILLE",
            (30.0, -20.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-30",
            "I-20",
            &["DAL", "TEXARKANA", "MEM"],
            "Arkansas Link",
            "TEXARKANA",
            (-10.0, 30.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "I-15",
            "I-5",
            &["LA", "BARSTOW", "LAS_VEGAS", "SLC"],
            "Inland West",
            "LAS_VEGAS",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-84",
            "I-80",
            &["POR", "BOI", "SLC"],
            "Snake River",
            "BOI",
            (10.0, -26.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "US95",
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
            "I-35",
            &["BIL", "CHEYENNE", "DEN", "COLORADO_SPRINGS", "ABQ"],
            "High Plains",
            "DEN",
            (-20.0, 15.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "US287",
            "I-35",
            &["DAL", "US287_PLAINS", "DEN"],
            "Front Range",
            "US287_PLAINS",
            (5.0, 5.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "I-495",
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
            "I-75",
            &["NOLA", "BHM_APPROACH", "CHATTANOOGA"],
            "Birmingham Spur",
            "BHM_APPROACH",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-49",
            "I-35",
            &["SHV", "OZARKS", "KC"],
            "Ozarks",
            "OZARKS",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-81",
            "I-95",
            &[
                "KNX",
                "SHENANDOAH_W",
                "SHENANDOAH_E",
                "APPALACHIAN_NORTH",
                "PENNSYLVANIA",
                "NYC",
            ],
            "Knoxville-New York",
            "SHENANDOAH_E",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-44",
            "I-40",
            &["OKC", "STL", "FORT_WAYNE", "CHI_APPROACH"],
            "St. Louis Link",
            "FORT_WAYNE",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-77",
            "I-75",
            &["CLT", "APPALACHIAN_NORTH", "CLEVELAND"],
            "Appalachian North",
            "APPALACHIAN_NORTH",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-76",
            "I-80",
            &["CLEVELAND", "PENNSYLVANIA", "PHILADELPHIA"],
            "Pennsylvania",
            "PENNSYLVANIA",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "US30",
            "I-80",
            &["SLC", "US30_C", "CHI_APPROACH", "PA_APPROACH"],
            "Lincoln Highway",
            "CHI_APPROACH",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "US6",
            "I-80",
            &["SLC", "US6_C", "CHI_APPROACH"],
            "Central Plains",
            "US6_C",
            (0.0, 0.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "US70",
            "I-40",
            &["PHX", "US70_C", "KNX"],
            "Mid-South",
            "US70_C",
            (0.0, 0.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "US90",
            "I-10",
            &["SAT", "US90_C", "JAX"],
            "Gulf Local",
            "US90_C",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-22",
            "I-40",
            &["MEM", "I22_C", "BHM_APPROACH"],
            "Memphis-Birmingham",
            "I22_C",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "I-29",
            "I-35",
            &["OMAHA", "I29_C", "DSM"],
            "Upper Missouri",
            "I29_C",
            (0.0, 0.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "US80",
            "I-20",
            &["DAL", "US80_C", "ATL"],
            "Old South",
            "US80_C",
            (0.0, 0.0),
            "start",
        ),
        t2_line_segment(
            stops,
            "US83",
            "I-35",
            &["MIN", "US83_C", "SAT"],
            "Dakota Spine",
            "US83_C",
            (0.0, 0.0),
            "end",
        ),
        t2_line_segment(
            stops,
            "I-37",
            "I-10",
            &["SAT", "I37_S", "I37_E", "SAT"],
            "San Antonio Spur",
            "I37_E",
            (0.0, 0.0),
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
        badge_at(stops, "I-20", "BHM_APPROACH", (-20.0, 20.0)),
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
    let title = if expanded {
        "THE CONTINENTAL METRO : T2 LOCAL SERVICES"
    } else {
        "THE CONTINENTAL METRO"
    };
    let subtitle = if expanded {
        "FULL SERVICE MAP · 25 THIN T2 CONNECTORS COLOR-CODED TO TRUNKS"
    } else {
        "A SCHEMATIC OF AMERICA'S PRIMARY FREIGHT LINES"
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

    if expanded {
        for line in &t2_lines {
            let color = t1_line_color(line.trunk);
            let d = line.to_svg_path();
            if d.is_empty() {
                continue;
            }
            let corridor = line.corridor;
            s += &format!(
                "<path data-corridor=\"{corridor}\" d=\"{d}\" stroke=\"#020617\" stroke-width=\"9\" fill=\"none\" \
                 opacity=\"0.78\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n"
            );
            s += &format!(
                "<path data-corridor=\"{corridor}\" d=\"{d}\" stroke=\"{color}\" stroke-width=\"5.5\" fill=\"none\" \
                 opacity=\"0.88\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n"
            );
            s += &format!(
                "<path data-corridor=\"{corridor}\" d=\"{d}\" stroke=\"#f8fafc\" stroke-width=\"0.8\" fill=\"none\" \
                opacity=\"0.22\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n"
            );
        }
    }

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
    if expanded {
        for line in &t2_lines {
            draw_generated_bends(
                &mut s,
                &line.waypoints,
                &line.routed_waypoints(),
                t1_line_color(line.trunk),
                3.2,
                0.72,
            );
        }
    }

    if expanded {
        for line in &t2_lines {
            let color = t1_line_color(line.trunk);
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
    for station in stations.iter().filter(|station| station.draw) {
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
        let font_size = if is_primary_terminal(station.id) {
            15.0_f64
        } else if station.is_hub {
            13.0_f64
        } else {
            11.0_f64
        };
        let fill = if station.is_hub || is_primary_terminal(station.id) {
            "white"
        } else {
            "#94a3b8"
        };
        s += &format!(
            "<text x=\"{lx:.1}\" y=\"{ly:.1}\" font-family=\"'Helvetica Neue',Arial,sans-serif\" \
             font-size=\"{font_size}\" font-weight=\"{}\" fill=\"{fill}\" \
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

    if expanded {
        for line in &t2_lines {
            let color = t1_line_color(line.trunk);
            let label = line.corridor.trim_start_matches("I-");
            let (sx, sy) = line.badge;
            s += &format!(
                "<circle cx=\"{sx}\" cy=\"{sy}\" r=\"13\" fill=\"#0f1623\" stroke=\"{color}\" stroke-width=\"3\" opacity=\"0.94\"/>\n"
            );
            s += &format!(
                "<text x=\"{sx}\" y=\"{:.1}\" font-family=\"'Helvetica Neue',Arial,sans-serif\" \
                 font-size=\"10\" font-weight=\"900\" fill=\"#f8fafc\" text-anchor=\"middle\">{label}</text>\n",
                sy + 3.5
            );

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

    if expanded {
        let x1 = lx + 915.0;
        let x2 = lx + 960.0;
        let y = ly + 52.0;
        s += &format!(
            "<path d=\"M {x1:.1} {y:.1} L {x2:.1} {y:.1}\" stroke=\"#f43f5e\" \
             stroke-width=\"5\" opacity=\"0.58\" stroke-linecap=\"round\"/>\n"
        );
        s += &format!(
            "<text x=\"{:.1}\" y=\"{y:.1}\" font-family=\"Arial,sans-serif\" font-size=\"11\" \
             fill=\"#94a3b8\" dominant-baseline=\"middle\">T2 connector</text>\n",
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
    s += "<text x=\"1200\" y=\"1320\" font-family=\"Arial,sans-serif\" font-size=\"11\" \
          fill=\"#475569\" text-anchor=\"middle\">Metro first, highway second: line numbers follow priority Interstate corridors; transfer hubs are national freight relay nodes. Inspired by H. Beck 1933.</text>\n";

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
        beck_stops, build_beck_svg, build_beck_t2_svg, is_primary_terminal, stop_by_id, t1_badges,
        t1_line_segments, t1_route_stop_ids, t2_line_segments, LineSegment,
    };

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
            vec!["MOBILE", "BHM_APPROACH", "NASHVILLE", "LOUISVILLE", "CINCINNATI"]
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
        assert_eq!(svg.matches("stroke-width=\"5.5\"").count(), t2_routes.len());
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
        assert!(t2_lines.iter().all(|line| line.waypoints.len() >= 3));
        assert!(t2_lines.iter().all(|line| !line.service_label.is_empty()));
        assert!(t2_lines.iter().all(|line| !line.label_anchor.is_empty()));
        assert_eq!(t2_lines.len(), 25);
        assert_eq!(t1_badges(&stops).len(), lines.len());
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
}
