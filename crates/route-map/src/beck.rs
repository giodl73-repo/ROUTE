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
        "I-35" => "#10b981",
        "I-40" => "#eab308",
        "I-69" => "#059669",
        "I-75" => "#06b6d4",
        "I-80" => "#3b82f6",
        "I-90" => "#8b5cf6",
        "I-95" => "#f43f5e",
        _ => "#94a3b8",
    }
}

#[derive(Clone)]
enum LabelDir {
    Right,
    Left,
    Up,
    Down,
}

/// A line segment on the Beck diagram.
/// Beck lines are sequences of waypoints connected at 0°/45°/90°.
struct LineSegment {
    corridor: &'static str,
    // Points defining the schematic path (Beck angles: horizontal/vertical/diagonal only)
    // These are LAYOUT coordinates in a Beck grid, not geographic
    waypoints: Vec<(f64, f64)>,
}

impl LineSegment {
    fn to_svg_path(&self) -> String {
        if self.waypoints.is_empty() {
            return String::new();
        }
        let mut d = format!("M {:.1} {:.1}", self.waypoints[0].0, self.waypoints[0].1);
        for pt in &self.waypoints[1..] {
            d += &format!(" L {:.1} {:.1}", pt.0, pt.1);
        }
        d
    }
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

    // Title
    s += "<text x=\"1200\" y=\"60\" font-family=\"'Helvetica Neue',Arial,sans-serif\" \
          font-size=\"28\" font-weight=\"900\" fill=\"white\" text-anchor=\"middle\" \
          letter-spacing=\"4\">INTERSTATE 2.0 RELAY NETWORK</text>\n";
    s += "<text x=\"1200\" y=\"90\" font-family=\"Arial,sans-serif\" \
          font-size=\"13\" fill=\"#64748b\" text-anchor=\"middle\" \
          letter-spacing=\"2\">SCHEMATIC DIAGRAM · T1 PRIMARY ARTERIES</text>\n";

    // ── Line definitions ──────────────────────────────────────────────────────────
    // Beck layout: 0°/45°/90° only
    // Grid origin approx: Chicago = (1200, 540)
    // West runs left, East runs right
    // North runs up, South runs down

    let lines = vec![
        // I-5: Pacific Coast — vertical on left side
        LineSegment {
            corridor: "I-5",
            waypoints: vec![
                (180.0, 220.0),  // Seattle
                (180.0, 480.0),  // Portland
                (180.0, 680.0),  // Sacramento junction
                (210.0, 800.0),  // Bay Area (diagonal SE)
                (240.0, 900.0),  // Fresno (diagonal)
                (240.0, 1080.0), // LA
            ],
        },
        // I-10: Southern Transcontinental — nearly horizontal across south
        LineSegment {
            corridor: "I-10",
            waypoints: vec![
                (240.0, 1080.0), // LA
                (420.0, 1080.0), // Phoenix
                (560.0, 1080.0), // El Paso
                (780.0, 1080.0), // San Antonio
                (900.0, 1020.0), // Houston (diagonal NE)
                (1020.0, 960.0), // Beaumont
                (1200.0, 960.0), // Baton Rouge / Gulf approach
                (1380.0, 960.0), // New Orleans
                (1560.0, 900.0), // Pensacola (diagonal NE)
                (1680.0, 840.0), // Tallahassee
                (1800.0, 840.0), // Jacksonville
            ],
        },
        // I-35: North-South Central — vertical through middle
        LineSegment {
            corridor: "I-35",
            waypoints: vec![
                (840.0, 220.0),  // Minneapolis
                (840.0, 480.0),  // Des Moines
                (840.0, 620.0),  // Kansas City
                (840.0, 780.0),  // Oklahoma City
                (840.0, 960.0),  // Dallas
                (780.0, 1080.0), // San Antonio (diagonal SW)
            ],
        },
        // I-40: Southern Transcontinental West — horizontal
        LineSegment {
            corridor: "I-40",
            waypoints: vec![
                (240.0, 900.0),  // CA/AZ border
                (420.0, 900.0),  // Albuquerque
                (600.0, 900.0),  // Amarillo
                (780.0, 900.0),  // Oklahoma City junction
                (960.0, 900.0),  // Memphis
                (1080.0, 840.0), // Nashville (diagonal NE)
                (1260.0, 840.0), // Knoxville
                (1440.0, 780.0), // Asheville (diagonal NE)
                (1620.0, 780.0), // Charlotte
                (1800.0, 840.0), // Jacksonville junction
            ],
        },
        // I-69: Gulf-to-Midwest (proposed) — diagonal NE from Houston
        LineSegment {
            corridor: "I-69",
            waypoints: vec![
                (900.0, 1020.0), // Houston
                (960.0, 900.0),  // Memphis junction
                (1020.0, 780.0), // Indianapolis (diagonal NE)
                (1080.0, 660.0), // Fort Wayne
                (1140.0, 540.0), // Chicago junction (diagonal NE)
            ],
        },
        // I-75: Southeast-Midwest — vertical on right-center
        LineSegment {
            corridor: "I-75",
            waypoints: vec![
                (1320.0, 360.0),  // Detroit/Toledo
                (1320.0, 480.0),  // Columbus
                (1320.0, 620.0),  // Cincinnati
                (1320.0, 720.0),  // Knoxville junction
                (1380.0, 840.0),  // Chattanooga (diagonal SE)
                (1440.0, 960.0),  // Atlanta
                (1440.0, 1080.0), // Gainesville
                (1500.0, 1150.0), // Tampa (diagonal SE)
            ],
        },
        // I-80: Northern Transcontinental — nearly horizontal through middle
        LineSegment {
            corridor: "I-80",
            waypoints: vec![
                (180.0, 680.0),  // Sacramento
                (240.0, 620.0),  // Reno (diagonal NE)
                (360.0, 540.0),  // Salt Lake (diagonal NE)
                (540.0, 540.0),  // Cheyenne
                (720.0, 540.0),  // Omaha
                (1020.0, 540.0), // Chicago
                (1200.0, 480.0), // Toledo area
                (1380.0, 420.0), // Cleveland
                (1500.0, 360.0), // Buffalo
                (1680.0, 300.0), // Albany
                (1800.0, 300.0), // Hartford
                (1920.0, 360.0), // Providence/Boston area
            ],
        },
        // I-90: Northern Tier — horizontal at top
        LineSegment {
            corridor: "I-90",
            waypoints: vec![
                (180.0, 220.0),  // Seattle
                (360.0, 220.0),  // Spokane
                (540.0, 220.0),  // Billings
                (720.0, 220.0),  // Rapid City
                (840.0, 220.0),  // Minneapolis junction
                (1020.0, 300.0), // Milwaukee (diagonal SE)
                (1200.0, 360.0), // Chicago (diagonal SE)
                (1320.0, 360.0), // Detroit/Toledo junction
                (1500.0, 300.0), // Cleveland → Buffalo
                (1680.0, 300.0), // Albany junction
                (1800.0, 300.0), // Boston area
            ],
        },
        // I-95: East Coast — vertical on right side
        LineSegment {
            corridor: "I-95",
            waypoints: vec![
                (1920.0, 360.0),  // Boston
                (1920.0, 480.0),  // New Haven
                (1920.0, 600.0),  // New York City
                (1920.0, 720.0),  // Philadelphia
                (1920.0, 840.0),  // Baltimore/DC
                (1860.0, 960.0),  // Richmond (diagonal SW)
                (1800.0, 1020.0), // Rocky Mount
                (1800.0, 1080.0), // Fayetteville
                (1800.0, 1150.0), // Jacksonville junction
            ],
        },
    ];

    // ── Draw lines ────────────────────────────────────────────────────────────────
    // Draw glow first, then main line
    for line in &lines {
        let color = t1_line_color(line.corridor);
        let d = line.to_svg_path();
        if d.is_empty() {
            continue;
        }
        // Glow
        s += &format!(
            "<path d=\"{d}\" stroke=\"{color}\" stroke-width=\"12\" fill=\"none\" \
             opacity=\"0.15\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n"
        );
        // Main line
        s += &format!(
            "<path d=\"{d}\" stroke=\"{color}\" stroke-width=\"7\" fill=\"none\" \
             opacity=\"1.0\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n"
        );
        // White center line for premium look
        s += &format!(
            "<path d=\"{d}\" stroke=\"white\" stroke-width=\"1.5\" fill=\"none\" \
             opacity=\"0.15\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n"
        );
    }

    // ── Draw stations ─────────────────────────────────────────────────────────────
    // Major relay hubs and interchange points
    let stations = vec![
        // Confirmed relay hubs (diamond-shaped, corridor colored)
        (
            "SEA",
            "Seattle",
            180.0,
            220.0,
            true,
            true,
            &["I-5", "I-90"][..],
            LabelDir::Right,
        ),
        (
            "CHI",
            "Chicago",
            1200.0,
            480.0,
            true,
            true,
            &["I-80", "I-90", "I-69"][..],
            LabelDir::Down,
        ),
        (
            "ATL",
            "Atlanta",
            1440.0,
            960.0,
            true,
            true,
            &["I-75", "I-85"][..],
            LabelDir::Right,
        ),
        (
            "BOS",
            "Boston",
            1920.0,
            360.0,
            true,
            true,
            &["I-90", "I-95"][..],
            LabelDir::Right,
        ),
        (
            "SAC",
            "Sacramento",
            180.0,
            680.0,
            true,
            true,
            &["I-5", "I-80"][..],
            LabelDir::Left,
        ),
        (
            "SAT",
            "San Antonio",
            780.0,
            1080.0,
            true,
            true,
            &["I-35", "I-10"][..],
            LabelDir::Down,
        ),
        (
            "JAX",
            "Jacksonville",
            1800.0,
            1150.0,
            true,
            true,
            &["I-10", "I-95"][..],
            LabelDir::Right,
        ),
        (
            "TOL",
            "Toledo",
            1320.0,
            360.0,
            true,
            true,
            &["I-75", "I-90"][..],
            LabelDir::Up,
        ),
        (
            "RIC",
            "Richmond",
            1860.0,
            960.0,
            true,
            false,
            &["I-95", "I-85"][..],
            LabelDir::Left,
        ),
        // Proposed hubs
        (
            "WIC",
            "Wichita*",
            780.0,
            720.0,
            false,
            false,
            &["I-35"][..],
            LabelDir::Right,
        ),
        (
            "HOU",
            "Houston*",
            900.0,
            1020.0,
            false,
            false,
            &["I-10", "I-69"][..],
            LabelDir::Down,
        ),
        (
            "BIL",
            "Billings*",
            540.0,
            220.0,
            false,
            false,
            &["I-90"][..],
            LabelDir::Up,
        ),
        // Key interchanges (not hubs, but important junctions)
        (
            "LA",
            "Los Angeles",
            240.0,
            1080.0,
            false,
            true,
            &["I-5", "I-10"][..],
            LabelDir::Down,
        ),
        (
            "PHX",
            "Phoenix",
            420.0,
            1080.0,
            false,
            false,
            &["I-10"][..],
            LabelDir::Down,
        ),
        (
            "DEN",
            "Denver",
            480.0,
            540.0,
            false,
            false,
            &["I-70", "I-25"][..],
            LabelDir::Up,
        ),
        (
            "DAL",
            "Dallas",
            840.0,
            900.0,
            false,
            true,
            &["I-35", "I-40"][..],
            LabelDir::Right,
        ),
        (
            "MEM",
            "Memphis",
            960.0,
            900.0,
            false,
            true,
            &["I-40", "I-69"][..],
            LabelDir::Down,
        ),
        (
            "SLC",
            "Salt Lake",
            360.0,
            540.0,
            false,
            false,
            &["I-80", "I-15"][..],
            LabelDir::Down,
        ),
        (
            "NYC",
            "New York",
            1920.0,
            600.0,
            false,
            true,
            &["I-95", "I-80"][..],
            LabelDir::Right,
        ),
        (
            "DC",
            "Washington",
            1920.0,
            840.0,
            false,
            false,
            &["I-95"][..],
            LabelDir::Left,
        ),
        (
            "DET",
            "Detroit",
            1260.0,
            360.0,
            false,
            false,
            &["I-75", "I-90"][..],
            LabelDir::Up,
        ),
        (
            "MIN",
            "Minneapolis",
            840.0,
            220.0,
            false,
            true,
            &["I-35", "I-90"][..],
            LabelDir::Left,
        ),
        (
            "KC",
            "Kansas City",
            840.0,
            620.0,
            false,
            false,
            &["I-35", "I-70"][..],
            LabelDir::Right,
        ),
        (
            "POR",
            "Portland",
            180.0,
            480.0,
            false,
            false,
            &["I-5"][..],
            LabelDir::Right,
        ),
        (
            "SPK",
            "Spokane",
            360.0,
            220.0,
            false,
            false,
            &["I-90"][..],
            LabelDir::Down,
        ),
    ];

    for (_, label, x, y, is_hub, is_interchange, lines_at_station, label_dir) in &stations {
        // Pick primary line color
        let color = lines_at_station
            .first()
            .map(|l| t1_line_color(l))
            .unwrap_or("#94a3b8");

        if *is_hub {
            // Relay hub: filled circle with white ring — confirmed
            s += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"14\" fill=\"white\"/>\n");
            s += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"11\" fill=\"{color}\"/>\n");
            s += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"6\" fill=\"white\"/>\n");
        } else if *is_interchange {
            // T1/T1 interchange: double ring
            s += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"10\" fill=\"white\" stroke=\"{color}\" stroke-width=\"3\"/>\n");
            s += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"4\" fill=\"{color}\"/>\n");
        } else {
            // Regular station: single ring
            s += &format!("<circle cx=\"{x}\" cy=\"{y}\" r=\"6\" fill=\"{color}\" stroke=\"white\" stroke-width=\"2\"/>\n");
        }

        // Label
        let (lx, ly, anchor) = match label_dir {
            LabelDir::Right => (x + 18.0, y + 4.0, "start"),
            LabelDir::Left => (x - 18.0, y + 4.0, "end"),
            LabelDir::Up => (*x, y - 18.0, "middle"),
            LabelDir::Down => (*x, y + 24.0, "middle"),
        };
        let font_size = if *is_hub { 13.0_f64 } else { 11.0_f64 };
        let fill = if *is_hub { "white" } else { "#94a3b8" };
        s += &format!(
            "<text x=\"{lx:.1}\" y=\"{ly:.1}\" font-family=\"'Helvetica Neue',Arial,sans-serif\" \
             font-size=\"{font_size}\" font-weight=\"{}\" fill=\"{fill}\" \
             text-anchor=\"{anchor}\">{label}</text>\n",
            if *is_hub { "700" } else { "400" }
        );
    }

    // ── Line labels (corridor shields) ────────────────────────────────────────────
    // Place one shield per corridor at a natural position
    let shields: &[(&str, f64, f64)] = &[
        ("I-5", 160.0, 580.0),
        ("I-10", 660.0, 1080.0),
        ("I-35", 820.0, 480.0),
        ("I-40", 560.0, 920.0),
        ("I-69", 960.0, 840.0),
        ("I-75", 1300.0, 720.0),
        ("I-80", 660.0, 560.0),
        ("I-90", 660.0, 240.0),
        ("I-95", 1900.0, 720.0),
    ];
    for &(corridor, sx, sy) in shields {
        let color = t1_line_color(corridor);
        s += &format!("<circle cx=\"{sx}\" cy=\"{sy}\" r=\"20\" fill=\"{color}\"/>\n");
        s += &format!("<circle cx=\"{sx}\" cy=\"{sy}\" r=\"20\" fill=\"none\" stroke=\"white\" stroke-width=\"1.5\" opacity=\"0.3\"/>\n");
        s += &format!(
            "<text x=\"{sx}\" y=\"{:.1}\" font-family=\"'Helvetica Neue',Arial,sans-serif\" \
             font-size=\"11\" font-weight=\"900\" fill=\"white\" text-anchor=\"middle\">{corridor}</text>\n",
            sy + 4.0
        );
    }

    // ── Legend ────────────────────────────────────────────────────────────────────
    let lx = 60.0_f64;
    let ly = 900.0_f64;
    s += &format!(
        "<rect x=\"{lx}\" y=\"{ly}\" width=\"240\" height=\"320\" rx=\"8\" \
         fill=\"#1e2d3d\" fill-opacity=\"0.95\"/>\n"
    );
    s += &format!(
        "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"'Helvetica Neue',Arial,sans-serif\" \
         font-size=\"12\" font-weight=\"700\" fill=\"white\" letter-spacing=\"1\">LEGEND</text>\n",
        lx + 20.0,
        ly + 28.0
    );

    // Hub types
    let legend_items = vec![
        ("Relay Hub (confirmed)", true, false),
        ("T1/T1 Interchange", false, true),
        ("Station", false, false),
    ];
    for (i, (label, is_hub, is_ix)) in legend_items.iter().enumerate() {
        let iy = ly + 60.0 + i as f64 * 40.0;
        let mx = lx + 24.0;
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

    // Proposed marker
    let py = ly + 200.0;
    s += &format!("<circle cx=\"{:.1}\" cy=\"{py}\" r=\"10\" fill=\"none\" stroke=\"#DAA520\" stroke-width=\"2\" stroke-dasharray=\"4,3\"/>\n", lx+24.0);
    s += &format!(
        "<text x=\"{:.1}\" y=\"{py}\" font-family=\"Arial,sans-serif\" font-size=\"11\" \
         fill=\"#94a3b8\" dominant-baseline=\"middle\">Proposed hub (*)</text>\n",
        lx + 42.0
    );

    s += &format!(
        "<text x=\"{:.1}\" y=\"{:.1}\" font-family=\"Arial,sans-serif\" font-size=\"9\" \
         fill=\"#475569\" text-anchor=\"middle\">NOT GEOGRAPHICALLY ACCURATE</text>\n",
        lx + 120.0,
        ly + 300.0
    );

    // ── Footer ────────────────────────────────────────────────────────────────────
    s += "<text x=\"1200\" y=\"1320\" font-family=\"Arial,sans-serif\" font-size=\"11\" \
          fill=\"#334155\" text-anchor=\"middle\">Relay hubs: ● = confirmed T1/T1 diamond    \
          ○ = T1/T1 interchange    · = station    * = proposed (missing link corridor)    \
          Inspired by H. Beck 1933 London Underground Map</text>\n";

    s += "</svg>\n";
    s
}
