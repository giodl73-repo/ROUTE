/// O-D Transit Time Monte Carlo Simulation
///
/// Tests the core I2.0 claim: managed freight lanes (PTI ≤ 1.15) enable
/// 48-hour shipper commitment windows on NY→LA, vs 80+ hours on GP lanes.
///
/// Model:
///   - Corridor is divided into segments: congested urban, rural free-flow,
///     mountain passes (Donner, Siskiyou), and interchange nodes (Dallas, St Louis)
///   - Each simulated trip draws random conditions per segment
///   - HOS regulations applied: 11h driving / 10h mandatory rest
///   - PTI computed as p95_transit / free_flow_transit across N trips
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::path::Path;

// ── TOML file loading ─────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct CorridorsFile {
    corridor: std::collections::HashMap<String, CorridorRecord>,
}

#[derive(Debug, Deserialize)]
struct CorridorRecord {
    name: String,
    origin: String,
    destination: String,
    fixed_overhead_hours: f64,
    hos_driving_hours: f64,
    hos_rest_hours: f64,
    segments: Vec<SegmentRecord>,
}

#[derive(Debug, Deserialize)]
struct SegmentRecord {
    name: String,
    miles: f64,
    base_vc: f64,
    free_flow_mph: f64,
    incident_prob: f64,
    incident_delay_mean_h: f64,
    incident_delay_std_h: f64,
    managed_lane_bypasses: bool,
    managed_lane_vc: f64,
}

impl From<CorridorRecord> for OdCorridor {
    fn from(r: CorridorRecord) -> Self {
        OdCorridor {
            name: r.name,
            origin: r.origin,
            destination: r.destination,
            fixed_overhead_hours: r.fixed_overhead_hours,
            hos_driving_hours: r.hos_driving_hours,
            hos_rest_hours: r.hos_rest_hours,
            segments: r
                .segments
                .into_iter()
                .map(|s| CorridorSegment {
                    name: s.name,
                    miles: s.miles,
                    base_vc: s.base_vc,
                    free_flow_mph: s.free_flow_mph,
                    incident_prob: s.incident_prob,
                    incident_delay_mean_hours: s.incident_delay_mean_h,
                    incident_delay_std_hours: s.incident_delay_std_h,
                    managed_lane_bypasses_incident: s.managed_lane_bypasses,
                    managed_lane_vc: s.managed_lane_vc,
                })
                .collect(),
        }
    }
}

/// Load a named corridor from data/od-corridors.toml.
/// Falls back to the built-in function if the file is missing or the corridor is not found.
pub fn load_corridor(data_dir: &Path, slug: &str) -> Option<OdCorridor> {
    let path = data_dir.join("od-corridors.toml");
    if path.exists() {
        if let Ok(text) = std::fs::read_to_string(&path) {
            if let Ok(file) = toml::from_str::<CorridorsFile>(&text) {
                if let Some(record) = file
                    .corridor
                    .into_iter()
                    .find(|(k, _)| k == slug)
                    .map(|(_, v)| v)
                {
                    return Some(record.into());
                }
            }
        }
    }
    // Fall back to built-in
    match slug {
        "ny_la" => Some(ny_la_corridor()),
        "hou_chi_current" => Some(hou_chi_current()),
        "hou_chi_i69" => Some(hou_chi_i69()),
        "chi_la" => Some(chi_la()),
        "mia_nyc" => Some(mia_nyc()),
        "sea_chi" => Some(sea_chi()),
        "dal_nyc" => Some(dal_nyc()),
        "la_sea" => Some(la_sea()),
        "atl_chi" => Some(atl_chi()),
        "ny_chi" => Some(ny_chi()),
        _ => None,
    }
}

/// One segment of a corridor with its own traffic characteristics.
#[derive(Debug, Clone)]
pub struct CorridorSegment {
    pub name: String,
    pub miles: f64,
    /// Baseline V/C ratio under typical demand
    pub base_vc: f64,
    /// Free-flow speed (mph) — design or posted speed
    pub free_flow_mph: f64,
    /// Probability of a major incident during any given trip (0.0–1.0)
    pub incident_prob: f64,
    /// Mean hours of delay if incident occurs
    pub incident_delay_mean_hours: f64,
    /// Std dev of incident delay (lognormal)
    pub incident_delay_std_hours: f64,
    /// If true, managed lanes bypass this segment's incident risk (e.g. Donner tunnel)
    pub managed_lane_bypasses_incident: bool,
    /// V/C on managed lanes (target design value)
    pub managed_lane_vc: f64,
}

/// Full O-D corridor definition.
#[derive(Debug, Clone)]
pub struct OdCorridor {
    pub name: String,
    pub origin: String,
    pub destination: String,
    pub segments: Vec<CorridorSegment>,
    /// Hours of Operations: max driving hours per HOS cycle
    pub hos_driving_hours: f64,
    /// Hours of mandatory rest per HOS cycle
    pub hos_rest_hours: f64,
    /// Extra non-driving time per trip: fueling, inspection, loading/unloading (hours)
    pub fixed_overhead_hours: f64,
}

impl OdCorridor {
    pub fn total_miles(&self) -> f64 {
        self.segments.iter().map(|s| s.miles).sum()
    }

    pub fn free_flow_driving_hours(&self) -> f64 {
        self.segments
            .iter()
            .map(|s| s.miles / s.free_flow_mph)
            .sum()
    }

    pub fn free_flow_elapsed_hours(&self) -> f64 {
        let driving = self.free_flow_driving_hours();
        let rest_stops = (driving / self.hos_driving_hours).floor() as u32;
        driving + (rest_stops as f64 * self.hos_rest_hours) + self.fixed_overhead_hours
    }
}

/// One simulated trip result.
#[derive(Debug, Clone)]
pub struct TripResult {
    pub elapsed_hours: f64,
    pub driving_hours: f64,
    pub delay_hours: f64,
    pub incidents: Vec<String>,
    pub managed_lanes: bool,
}

/// An I2.0 intervention that modifies corridor segment parameters.
/// Each intervention is independent and stackable.
#[derive(Debug, Clone)]
pub enum Intervention {
    /// Dedicated freight-only lanes at V/C 0.70 (no passenger induced demand).
    /// Eliminates congestion surges; improves segment speeds to design speed.
    ManagedFreightLanes,

    /// Driver relay stations at T1 hubs (~500-mile intervals).
    /// Eliminates mandatory HOS rest stops. Truck runs 24/7.
    DriverRelay { stations: usize, swap_minutes: f64 },

    /// Second driver in cab. Co-driver sleeps in berth while truck moves.
    TeamDrivers,

    /// Donner Pass hardening: early warning system + preemptive closure protocols.
    /// Reduces closure probability by 50%; mean duration 18h→12h.
    DonnerHardening,

    /// Donner freight tunnel: bypasses the mountain pass entirely.
    /// Eliminates all weather-closure risk on Donner segment.
    DonnerTunnel,

    /// Diamond interchange upgrades at k=1 nodes (Atlanta, Jacksonville, Chicago).
    /// Reduces incident probability at major interchange bottlenecks by 60%.
    DiamondInterchanges,

    /// Intelligent routing: real-time V2I incident alerts.
    /// Trucks reroute or hold at relay stations before encountering incidents.
    /// Reduces incident delay by 40% (earlier warning = shorter delay absorbed).
    IntelligentRouting,

    /// Platooning: 2-4 truck convoys with V2V communication.
    /// Reduces aerodynamic drag 20-25%, increases effective speed by 2-3 mph on rural segments.
    Platooning,
}

impl Intervention {
    pub fn label(&self) -> &str {
        match self {
            Self::ManagedFreightLanes => "Managed freight lanes",
            Self::DriverRelay { .. } => "Driver relay network",
            Self::TeamDrivers => "Team drivers (2-person)",
            Self::DonnerHardening => "Donner hardening",
            Self::DonnerTunnel => "Donner tunnel",
            Self::DiamondInterchanges => "Diamond interchanges",
            Self::IntelligentRouting => "Intelligent routing (V2I)",
            Self::Platooning => "Truck platooning",
        }
    }

    pub fn capex_label(&self) -> &str {
        match self {
            Self::ManagedFreightLanes => "$121B (7 T1 corridors)",
            Self::DriverRelay { .. } => "$40M (8 relay stations)",
            Self::TeamDrivers => "$0 (operational change)",
            Self::DonnerHardening => "$800M (snowshed + warning)",
            Self::DonnerTunnel => "$4B (12-mile tunnel)",
            Self::DiamondInterchanges => "$930M (Phase 1: ATL/JAX/TOL)",
            Self::IntelligentRouting => "$200M (V2I hardware + software)",
            Self::Platooning => "$50M (V2V + platooning systems)",
        }
    }
}

/// Apply one or more interventions to a corridor, returning the modified corridor
/// and the driver mode implied by the interventions.
pub fn apply_interventions(
    base: &OdCorridor,
    interventions: &[Intervention],
) -> (OdCorridor, DriverMode) {
    let mut corridor = base.clone();
    let mut driver_mode = DriverMode::Solo;
    let mut relay_stations = 0usize;
    let mut relay_swap_min = 20.0f64;

    for intervention in interventions {
        match intervention {
            Intervention::ManagedFreightLanes => {
                for seg in &mut corridor.segments {
                    // Switch all segments to managed lane V/C
                    seg.base_vc = seg.managed_lane_vc;
                    // No congestion surge events on access-controlled lanes
                }
            }
            Intervention::DriverRelay {
                stations,
                swap_minutes,
            } => {
                relay_stations = *stations;
                relay_swap_min = *swap_minutes;
                driver_mode = DriverMode::Relay {
                    stations: relay_stations,
                    swap_minutes: relay_swap_min,
                };
            }
            Intervention::TeamDrivers => {
                if driver_mode == DriverMode::Solo {
                    driver_mode = DriverMode::Team;
                }
            }
            Intervention::DonnerHardening => {
                for seg in &mut corridor.segments {
                    if seg.name.contains("Donner") {
                        seg.incident_prob *= 0.50; // 50% fewer closures
                        seg.incident_delay_mean_hours *= 0.67; // 18h→12h mean
                    }
                }
            }
            Intervention::DonnerTunnel => {
                for seg in &mut corridor.segments {
                    if seg.name.contains("Donner") {
                        seg.incident_prob = 0.0; // tunnel bypasses all weather risk
                        seg.managed_lane_bypasses_incident = true;
                        // Tunnel also eliminates grade: slightly faster
                        seg.free_flow_mph = 65.0;
                    }
                }
            }
            Intervention::DiamondInterchanges => {
                // Diamond interchanges at major bottleneck nodes reduce incident prob 60%
                for seg in &mut corridor.segments {
                    if seg.name.contains("interchange")
                        || seg.name.contains("metro")
                        || seg.name.contains("approach")
                        || seg.name.contains("bypass")
                    {
                        seg.incident_prob *= 0.40; // k≥3 means trucks reroute around incidents
                        seg.incident_delay_mean_hours *= 0.60; // shorter delays with alternates
                    }
                }
            }
            Intervention::IntelligentRouting => {
                // V2I alerts let trucks avoid or prepare for incidents
                // 40% reduction in experienced delay (can hold at relay station or take early alternate)
                for seg in &mut corridor.segments {
                    seg.incident_delay_mean_hours *= 0.60;
                }
            }
            Intervention::Platooning => {
                // 2-3 mph effective speed gain on rural/highway segments from reduced drag
                for seg in &mut corridor.segments {
                    if seg.base_vc < 0.60 {
                        // only effective on free-flowing segments
                        seg.free_flow_mph *= 1.035; // ~3.5% speed increase
                    }
                }
            }
        }
    }

    // If relay was set with specifics, update
    if matches!(driver_mode, DriverMode::Relay { .. }) {
        driver_mode = DriverMode::Relay {
            stations: relay_stations,
            swap_minutes: relay_swap_min,
        };
    }

    (corridor, driver_mode)
}

/// Run a single intervention stack and return the distribution.
pub fn run_intervention_stack(
    base: &OdCorridor,
    interventions: &[Intervention],
    n_trips: usize,
    seed: u64,
) -> TransitDistribution {
    let (corridor, driver) = apply_interventions(base, interventions);
    run_od_simulation_with_driver(&corridor, false, &driver, n_trips, seed)
}

/// Benchmark all interventions individually + in combination.
pub struct InterventionBenchmark {
    pub corridor_name: String,
    pub baseline: TransitDistribution,
    pub results: Vec<InterventionResult>,
}

pub struct InterventionResult {
    pub label: String,
    pub capex: String,
    pub dist: TransitDistribution,
    pub p95_delta_hours: f64, // vs baseline (negative = improvement)
    pub pct_under_48h: f64,
    pub sla_achieved: bool,
}

impl InterventionBenchmark {
    pub fn run(corridor: &OdCorridor, n_trips: usize, seed: u64) -> Self {
        let relay_count = ((corridor.total_miles() / 500.0).ceil() as usize).max(1);

        // Define all single interventions to test
        let singles: Vec<(&str, &str, Vec<Intervention>)> = vec![
            ("Baseline (Solo/GP)", "$0", vec![]),
            (
                "Managed lanes only",
                "$121B",
                vec![Intervention::ManagedFreightLanes],
            ),
            ("Team drivers only", "$0", vec![Intervention::TeamDrivers]),
            (
                "Driver relay only",
                "$40M",
                vec![Intervention::DriverRelay {
                    stations: relay_count,
                    swap_minutes: 20.0,
                }],
            ),
            (
                "Intelligent routing only",
                "$200M",
                vec![Intervention::IntelligentRouting],
            ),
            (
                "Diamond interchanges",
                "$930M",
                vec![Intervention::DiamondInterchanges],
            ),
            (
                "Donner hardening",
                "$800M",
                vec![Intervention::DonnerHardening],
            ),
            ("Donner tunnel", "$4B", vec![Intervention::DonnerTunnel]),
            ("Platooning", "$50M", vec![Intervention::Platooning]),
            // Combinations
            (
                "Relay + Managed lanes",
                "$121B+$40M",
                vec![
                    Intervention::ManagedFreightLanes,
                    Intervention::DriverRelay {
                        stations: relay_count,
                        swap_minutes: 20.0,
                    },
                ],
            ),
            (
                "Relay + Managed + Donner tunnel",
                "$125B",
                vec![
                    Intervention::ManagedFreightLanes,
                    Intervention::DonnerTunnel,
                    Intervention::DriverRelay {
                        stations: relay_count,
                        swap_minutes: 20.0,
                    },
                ],
            ),
            (
                "Relay + Diamonds + Routing",
                "$1.2B",
                vec![
                    Intervention::DriverRelay {
                        stations: relay_count,
                        swap_minutes: 20.0,
                    },
                    Intervention::DiamondInterchanges,
                    Intervention::IntelligentRouting,
                ],
            ),
            (
                "Full I2.0 stack",
                "$126B+$1.2B",
                vec![
                    Intervention::ManagedFreightLanes,
                    Intervention::DonnerTunnel,
                    Intervention::DiamondInterchanges,
                    Intervention::IntelligentRouting,
                    Intervention::Platooning,
                    Intervention::DriverRelay {
                        stations: relay_count,
                        swap_minutes: 15.0,
                    },
                ],
            ),
        ];

        let baseline_dist = run_intervention_stack(corridor, &[], n_trips, seed);
        let baseline_p95 = baseline_dist.p95_hours;

        let results = singles
            .into_iter()
            .enumerate()
            .map(|(i, (label, capex, interventions))| {
                let dist =
                    run_intervention_stack(corridor, &interventions, n_trips, seed + i as u64);
                let delta = dist.p95_hours - baseline_p95;
                let pct = dist.pct_under_48h;
                let sla = dist.p95_hours <= 48.0;
                InterventionResult {
                    label: label.to_string(),
                    capex: capex.to_string(),
                    dist,
                    p95_delta_hours: delta,
                    pct_under_48h: pct,
                    sla_achieved: sla,
                }
            })
            .collect();

        InterventionBenchmark {
            corridor_name: corridor.name.clone(),
            baseline: baseline_dist,
            results,
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum DriverMode {
    /// Single driver: mandatory 10h rest stop after every 11h driving.
    Solo,

    /// Two drivers in-cab, swapping behind the wheel. Co-driver sleeps in
    /// sleeper berth — counts as HOS rest while truck keeps moving.
    /// Net effect: zero mandatory stop time.
    Team,

    /// Driver relay network — the airline crew model applied to trucking.
    ///
    /// T1 diamond hubs and T1/T2 intersections become crew-change stations.
    /// Fresh driver boards, outgoing driver catches a repositioning truck or
    /// transit back to their home base. Truck never stops more than the swap
    /// time. Each driver works one regional leg (~300-600 miles / 5-8 hours)
    /// and returns home the same day — dramatically improving driver quality
    /// of life and retention.
    ///
    /// Operational precedent: UPS Worldport hub relay, BNSF crew changes,
    /// airline gate turns. The I2.0 hub infrastructure (parking, amenities,
    /// scheduling office) is already funded by the freight program.
    Relay {
        /// Number of crew-change stations along the corridor.
        /// For NY→LA: ~5 stations (Chicago, Omaha/Des Moines, SLC, Reno, Sacramento)
        stations: usize,
        /// Minutes per driver swap (park, handoff paperwork, pre-trip inspection).
        /// Target: 15 min. Realistic early-phase: 20-25 min.
        swap_minutes: f64,
    },
}

/// Run one simulated trip through the corridor.
fn simulate_trip(
    corridor: &OdCorridor,
    managed_lanes: bool,
    driver: &DriverMode,
    rng: &mut impl Rng,
) -> TripResult {
    let mut driving_hours = 0.0f64;
    let mut delay_hours = 0.0f64;
    let mut incidents = Vec::new();

    for seg in &corridor.segments {
        let vc = if managed_lanes {
            seg.managed_lane_vc
        } else {
            let variation = rng.gen_range(0.80..=1.20f64);
            let surge = if rng.gen_bool(0.10) { 1.40 } else { 1.0 };
            (seg.base_vc * variation * surge).min(1.5)
        };

        let ff_hours = seg.miles / seg.free_flow_mph;
        let segment_hours = bpr_time(ff_hours, vc);
        driving_hours += segment_hours;

        let skip_incident = managed_lanes && seg.managed_lane_bypasses_incident;
        if !skip_incident && rng.gen_bool(seg.incident_prob as f64) {
            let delay = sample_lognormal(
                seg.incident_delay_mean_hours,
                seg.incident_delay_std_hours,
                rng,
            )
            .max(0.1);
            delay_hours += delay;
            incidents.push(format!("{} (+{:.1}h)", seg.name, delay));
        }
    }

    // HOS model depends on driver configuration
    let hos_overhead = match driver {
        DriverMode::Solo => {
            // Full 10-hour stop after every 11-hour driving window
            let rest_stops = (driving_hours / corridor.hos_driving_hours).floor() as u32;
            rest_stops as f64 * corridor.hos_rest_hours
        }
        DriverMode::Team => {
            // Co-driver sleeps in berth — truck never stops for rest.
            // Small overhead: driver-swap time every 11 hours (2-5 min to swap seats).
            let swaps = (driving_hours / corridor.hos_driving_hours).floor() as u32;
            swaps as f64 * (4.0 / 60.0) // 4-minute swap, truck barely slows
        }
        DriverMode::Relay {
            stations,
            swap_minutes,
        } => {
            // Fresh driver at each relay station; swap takes swap_minutes.
            // Relay is always at a T1 hub so no search time.
            // Small variance: swap time can run +5 min if paperwork delayed.
            let swap_variance = rng.gen_range(0.0..5.0_f64) / 60.0; // 0-5 min extra
            *stations as f64 * (swap_minutes / 60.0 + swap_variance)
        }
    };

    let elapsed = driving_hours + hos_overhead + delay_hours + corridor.fixed_overhead_hours;
    TripResult {
        elapsed_hours: elapsed,
        driving_hours,
        delay_hours,
        incidents,
        managed_lanes,
    }
}

fn bpr_time(free_flow_hours: f64, vc: f64) -> f64 {
    // BPR: t = t0 * (1 + 0.15 * (v/c)^4)
    // Capped at V/C = 1.3 for BPR validity; above that apply linear extrapolation
    let vc_capped = vc.min(1.3);
    free_flow_hours * (1.0 + 0.15 * vc_capped.powi(4))
}

fn sample_lognormal(mean: f64, std: f64, rng: &mut impl Rng) -> f64 {
    // Convert mean/std to lognormal parameters
    let variance = std * std;
    let mu = (mean * mean / (mean * mean + variance).sqrt()).ln();
    let sigma = ((1.0 + variance / (mean * mean)).ln()).sqrt();
    // Box-Muller normal sample → lognormal
    let u1: f64 = rng.gen_range(f64::EPSILON..1.0);
    let u2: f64 = rng.gen_range(f64::EPSILON..1.0);
    let normal = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
    (mu + sigma * normal).exp()
}

/// Distribution statistics over N simulated trips.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransitDistribution {
    pub n_trips: usize,
    pub free_flow_hours: f64,
    pub mean_hours: f64,
    pub p50_hours: f64,
    pub p75_hours: f64,
    pub p90_hours: f64,
    pub p95_hours: f64,
    pub p99_hours: f64,
    pub pti: f64,                     // p95 / free_flow
    pub commitment_window_hours: f64, // = p95
    pub commitment_window_days: f64,
    pub pct_under_48h: f64, // fraction of trips completing under 48 hours elapsed
    pub pct_sla_met: f64,   // fraction meeting the tier SLA (configurable)
    pub managed_lanes: bool,
}

impl TransitDistribution {
    pub fn sla_hours(&self) -> f64 {
        self.commitment_window_hours
    }
}

/// Apply seasonal incident probability modifiers to a corridor.
/// month: 1=January ... 12=December
pub fn apply_seasonal(corridor: &OdCorridor, month: u8) -> OdCorridor {
    if month == 0 || month > 12 {
        return corridor.clone();
    }
    let mut c = corridor.clone();
    let is_winter = matches!(month, 11 | 12 | 1 | 2 | 3 | 4);
    let is_holiday = matches!(month, 10 | 11 | 12);
    let is_harvest = matches!(month, 9 | 10 | 11);
    let is_construction = matches!(month, 6 | 7 | 8);

    for seg in &mut c.segments {
        let name_lower = seg.name.to_lowercase();
        // Mountain pass winter closures
        if is_winter
            && (name_lower.contains("donner")
                || name_lower.contains("pass")
                || name_lower.contains("snoqualmie")
                || name_lower.contains("siskiyou"))
        {
            seg.incident_prob *= 2.4;
            seg.incident_prob = seg.incident_prob.min(0.60);
        } else if !is_winter && (name_lower.contains("donner") || name_lower.contains("pass")) {
            seg.incident_prob *= 0.25;
        }
        // Holiday freight surge at urban bottlenecks
        if is_holiday
            && (name_lower.contains("urban")
                || name_lower.contains("metro")
                || name_lower.contains("approach")
                || name_lower.contains("interchange"))
        {
            seg.base_vc = (seg.base_vc * 1.20).min(1.5);
        }
        // Harvest surge on rural corridors
        if is_harvest
            && (name_lower.contains("rural")
                || name_lower.contains("i-35")
                || name_lower.contains("i-55"))
        {
            seg.base_vc = (seg.base_vc * 1.15).min(1.2);
        }
        // Summer construction
        if is_construction && (name_lower.contains("urban") || name_lower.contains("approach")) {
            seg.incident_prob = (seg.incident_prob * 1.20).min(0.45);
        }
    }
    c
}

/// Run Monte Carlo and return distribution statistics.
pub fn run_od_simulation(
    corridor: &OdCorridor,
    managed_lanes: bool,
    n_trips: usize,
    seed: u64,
) -> TransitDistribution {
    run_od_simulation_with_driver(corridor, managed_lanes, &DriverMode::Solo, n_trips, seed)
}

pub fn run_od_simulation_with_driver(
    corridor: &OdCorridor,
    managed_lanes: bool,
    driver: &DriverMode,
    n_trips: usize,
    seed: u64,
) -> TransitDistribution {
    use rand::SeedableRng;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);

    let mut elapsed_times: Vec<f64> = (0..n_trips)
        .map(|_| simulate_trip(corridor, managed_lanes, driver, &mut rng).elapsed_hours)
        .collect();

    elapsed_times.sort_by(f64::total_cmp);

    let n = elapsed_times.len() as f64;
    let mean = elapsed_times.iter().sum::<f64>() / n;
    let p50 = percentile(&elapsed_times, 0.50);
    let p75 = percentile(&elapsed_times, 0.75);
    let p90 = percentile(&elapsed_times, 0.90);
    let p95 = percentile(&elapsed_times, 0.95);
    let p99 = percentile(&elapsed_times, 0.99);
    let ff = corridor.free_flow_elapsed_hours();
    let pti = p95 / ff;

    let under_48 = elapsed_times.iter().filter(|&&t| t <= 48.0).count() as f64 / n;

    // SLA = p95 commitment window
    TransitDistribution {
        n_trips,
        free_flow_hours: ff,
        mean_hours: mean,
        p50_hours: p50,
        p75_hours: p75,
        p90_hours: p90,
        p95_hours: p95,
        p99_hours: p99,
        pti,
        commitment_window_hours: p95,
        commitment_window_days: p95 / 24.0,
        pct_under_48h: under_48 * 100.0,
        pct_sla_met: under_48 * 100.0,
        managed_lanes,
    }
}

fn percentile(sorted: &[f64], p: f64) -> f64 {
    let idx = (p * (sorted.len() - 1) as f64).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

// ── Pre-defined corridors ─────────────────────────────────────────────────────

/// NY→LA via I-80 (northern transcontinental, 2,800 miles).
pub fn ny_la_corridor() -> OdCorridor {
    OdCorridor {
        name: "NY→LA (I-80 Northern Transcontinental)".into(),
        origin: "New York City, NY".into(),
        destination: "Los Angeles, CA".into(),
        fixed_overhead_hours: 4.0, // fueling (3 stops × 45min), inspection, loading
        hos_driving_hours: 11.0,
        hos_rest_hours: 10.0,
        segments: vec![
            CorridorSegment {
                name: "NJ/PA urban (I-95→I-78→I-80)".into(),
                miles: 120.0,
                base_vc: 0.92,
                free_flow_mph: 65.0,
                incident_prob: 0.18, // high urban incident rate
                incident_delay_mean_hours: 1.2,
                incident_delay_std_hours: 0.8,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
            CorridorSegment {
                name: "PA/OH/IN rural I-80".into(),
                miles: 560.0,
                base_vc: 0.35,
                free_flow_mph: 70.0,
                incident_prob: 0.04,
                incident_delay_mean_hours: 0.8,
                incident_delay_std_hours: 0.5,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.30,
            },
            CorridorSegment {
                name: "Chicago metro bypass (I-80/I-90)".into(),
                miles: 80.0,
                base_vc: 1.05,
                free_flow_mph: 65.0,
                incident_prob: 0.20,
                incident_delay_mean_hours: 1.8,
                incident_delay_std_hours: 1.2,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.68,
            },
            CorridorSegment {
                name: "IL/IA/NE/WY rural I-80".into(),
                miles: 1100.0,
                base_vc: 0.28,
                free_flow_mph: 75.0, // higher rural speed limit (NE/WY 80mph)
                incident_prob: 0.06, // weather (blizzard, ice)
                incident_delay_mean_hours: 3.5,
                incident_delay_std_hours: 2.0,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.25,
            },
            CorridorSegment {
                name: "Donner Pass (Sierra Nevada, I-80)".into(),
                miles: 65.0,
                base_vc: 0.80,
                free_flow_mph: 55.0, // mountain grade; lower speed
                // 50 closures/year × mean 18h / (8760h/yr) = 10.3% closure probability
                // But per trip: closure hits if you arrive during a closure window
                // ~50 events × 18h / 8760h = 10.3% chance on any given arrival
                incident_prob: 0.103,
                incident_delay_mean_hours: 18.0, // mean closure: wait or reroute
                incident_delay_std_hours: 12.0,
                managed_lane_bypasses_incident: true, // managed lane = tunnel
                managed_lane_vc: 0.72,
            },
            CorridorSegment {
                name: "Bay Area approach (I-80→I-580, Emeryville)".into(),
                miles: 90.0,
                base_vc: 1.15, // chronically over capacity
                free_flow_mph: 65.0,
                incident_prob: 0.22,
                incident_delay_mean_hours: 1.5,
                incident_delay_std_hours: 1.0,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
            CorridorSegment {
                name: "Bay Area→I-5 (cross CA via I-580/I-5)".into(),
                miles: 350.0,
                base_vc: 0.65,
                free_flow_mph: 70.0,
                incident_prob: 0.08,
                incident_delay_mean_hours: 1.0,
                incident_delay_std_hours: 0.6,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.60,
            },
            CorridorSegment {
                name: "LA approach (I-5/I-405 metro)".into(),
                miles: 50.0,
                base_vc: 1.30, // highest V/C segment
                free_flow_mph: 65.0,
                incident_prob: 0.25,
                incident_delay_mean_hours: 1.2,
                incident_delay_std_hours: 0.8,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
        ],
    }
}

/// Houston→Chicago via I-45/I-35/I-55 (current 3-corridor hop, 1,090 miles).
pub fn hou_chi_current() -> OdCorridor {
    OdCorridor {
        name: "HOU→CHI (I-45→I-35→I-55, current 3-hop)".into(),
        origin: "Houston, TX".into(),
        destination: "Chicago, IL".into(),
        fixed_overhead_hours: 3.0,
        hos_driving_hours: 11.0,
        hos_rest_hours: 10.0,
        segments: vec![
            CorridorSegment {
                name: "Houston metro (I-45)".into(),
                miles: 60.0,
                base_vc: 1.10,
                free_flow_mph: 65.0,
                incident_prob: 0.18,
                incident_delay_mean_hours: 1.4,
                incident_delay_std_hours: 0.9,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
            CorridorSegment {
                name: "I-45 Houston→Dallas".into(),
                miles: 240.0,
                base_vc: 0.55,
                free_flow_mph: 75.0,
                incident_prob: 0.06,
                incident_delay_mean_hours: 1.0,
                incident_delay_std_hours: 0.6,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.50,
            },
            CorridorSegment {
                // Dallas interchange: I-45/I-35 merge — major bottleneck
                name: "Dallas interchange (I-45/I-35 merge)".into(),
                miles: 30.0,
                base_vc: 1.35, // ATRI top-10 bottleneck
                free_flow_mph: 60.0,
                incident_prob: 0.28,
                incident_delay_mean_hours: 1.8,
                incident_delay_std_hours: 1.2,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
            CorridorSegment {
                name: "I-35 Dallas→Oklahoma City".into(),
                miles: 210.0,
                base_vc: 0.60,
                free_flow_mph: 75.0,
                incident_prob: 0.08, // tornado/severe wx corridor
                incident_delay_mean_hours: 2.5,
                incident_delay_std_hours: 1.5,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.55,
            },
            CorridorSegment {
                name: "I-35 OKC→Kansas City".into(),
                miles: 340.0,
                base_vc: 0.45,
                free_flow_mph: 75.0,
                incident_prob: 0.05,
                incident_delay_mean_hours: 0.8,
                incident_delay_std_hours: 0.5,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.40,
            },
            CorridorSegment {
                // St. Louis interchange: I-35→I-55 transfer — variance node
                name: "St. Louis interchange (I-44/I-55/I-70 complex)".into(),
                miles: 40.0,
                base_vc: 0.95,
                free_flow_mph: 60.0,
                incident_prob: 0.15,
                incident_delay_mean_hours: 1.2,
                incident_delay_std_hours: 0.8,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.68,
            },
            CorridorSegment {
                name: "I-55 St. Louis→Chicago".into(),
                miles: 300.0,
                base_vc: 0.50,
                free_flow_mph: 70.0,
                incident_prob: 0.05,
                incident_delay_mean_hours: 0.8,
                incident_delay_std_hours: 0.5,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.45,
            },
            CorridorSegment {
                name: "Chicago south approach (I-55)".into(),
                miles: 50.0,
                base_vc: 1.05,
                free_flow_mph: 65.0,
                incident_prob: 0.18,
                incident_delay_mean_hours: 1.3,
                incident_delay_std_hours: 0.9,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
        ],
    }
}

/// Houston→Chicago via I-69 (post-completion, direct, 870 miles).
pub fn hou_chi_i69() -> OdCorridor {
    let mut c = hou_chi_current();
    c.name = "HOU→CHI (I-69 direct, post-completion)".into();
    // I-69 route: Texarkana→Little Rock→Indianapolis→Chicago
    // Eliminates Dallas interchange and St. Louis interchange
    // Reduces distance by ~220 miles
    c.segments = vec![
        CorridorSegment {
            name: "Houston metro (I-69 approach)".into(),
            miles: 60.0,
            base_vc: 1.10,
            free_flow_mph: 65.0,
            incident_prob: 0.18,
            incident_delay_mean_hours: 1.4,
            incident_delay_std_hours: 0.9,
            managed_lane_bypasses_incident: false,
            managed_lane_vc: 0.70,
        },
        CorridorSegment {
            name: "I-69 Houston→Texarkana".into(),
            miles: 185.0,
            base_vc: 0.40, // new road, lower initial V/C
            free_flow_mph: 75.0,
            incident_prob: 0.04,
            incident_delay_mean_hours: 0.8,
            incident_delay_std_hours: 0.5,
            managed_lane_bypasses_incident: false,
            managed_lane_vc: 0.38,
        },
        CorridorSegment {
            name: "I-69 Texarkana→Memphis".into(),
            miles: 220.0,
            base_vc: 0.38,
            free_flow_mph: 75.0,
            incident_prob: 0.04,
            incident_delay_mean_hours: 0.8,
            incident_delay_std_hours: 0.5,
            managed_lane_bypasses_incident: false,
            managed_lane_vc: 0.35,
        },
        CorridorSegment {
            name: "I-69 Memphis→Indianapolis".into(),
            miles: 380.0,
            base_vc: 0.42,
            free_flow_mph: 70.0,
            incident_prob: 0.05,
            incident_delay_mean_hours: 0.9,
            incident_delay_std_hours: 0.6,
            managed_lane_bypasses_incident: false,
            managed_lane_vc: 0.38,
        },
        CorridorSegment {
            name: "Indianapolis→Chicago (I-65/I-80)".into(),
            miles: 165.0,
            base_vc: 0.65,
            free_flow_mph: 70.0,
            incident_prob: 0.08,
            incident_delay_mean_hours: 1.1,
            incident_delay_std_hours: 0.7,
            managed_lane_bypasses_incident: false,
            managed_lane_vc: 0.60,
        },
        CorridorSegment {
            name: "Chicago south approach".into(),
            miles: 50.0,
            base_vc: 1.05,
            free_flow_mph: 65.0,
            incident_prob: 0.18,
            incident_delay_mean_hours: 1.3,
            incident_delay_std_hours: 0.9,
            managed_lane_bypasses_incident: false,
            managed_lane_vc: 0.70,
        },
    ];
    c
}

// ── Additional corridors ──────────────────────────────────────────────────────

/// Chicago → Los Angeles via I-80 (I-80/I-88 to I-80, ~2,020 miles)
pub fn chi_la() -> OdCorridor {
    OdCorridor {
        name: "CHI→LA (I-80 midwest transcontinental)".into(),
        origin: "Chicago, IL".into(),
        destination: "Los Angeles, CA".into(),
        fixed_overhead_hours: 3.0,
        hos_driving_hours: 11.0,
        hos_rest_hours: 10.0,
        segments: vec![
            CorridorSegment {
                name: "Chicago south approach".into(),
                miles: 60.0,
                base_vc: 1.05,
                free_flow_mph: 65.0,
                incident_prob: 0.20,
                incident_delay_mean_hours: 1.4,
                incident_delay_std_hours: 0.9,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
            CorridorSegment {
                name: "IL/IA/NE rural I-80".into(),
                miles: 840.0,
                base_vc: 0.28,
                free_flow_mph: 75.0,
                incident_prob: 0.05,
                incident_delay_mean_hours: 2.5,
                incident_delay_std_hours: 1.5,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.25,
            },
            CorridorSegment {
                name: "WY/UT rural I-80".into(),
                miles: 560.0,
                base_vc: 0.22,
                free_flow_mph: 75.0,
                incident_prob: 0.06,
                incident_delay_mean_hours: 3.0,
                incident_delay_std_hours: 2.0,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.20,
            },
            CorridorSegment {
                name: "Donner Pass (Sierra Nevada)".into(),
                miles: 65.0,
                base_vc: 0.80,
                free_flow_mph: 55.0,
                incident_prob: 0.103,
                incident_delay_mean_hours: 18.0,
                incident_delay_std_hours: 12.0,
                managed_lane_bypasses_incident: true,
                managed_lane_vc: 0.72,
            },
            CorridorSegment {
                name: "Bay Area → I-5 south".into(),
                miles: 280.0,
                base_vc: 0.65,
                free_flow_mph: 70.0,
                incident_prob: 0.08,
                incident_delay_mean_hours: 1.0,
                incident_delay_std_hours: 0.6,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.60,
            },
            CorridorSegment {
                name: "LA approach".into(),
                miles: 50.0,
                base_vc: 1.30,
                free_flow_mph: 65.0,
                incident_prob: 0.25,
                incident_delay_mean_hours: 1.2,
                incident_delay_std_hours: 0.8,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
        ],
    }
}

/// Miami → New York via I-95 (1,280 miles — the most important single corridor for perishables)
pub fn mia_nyc() -> OdCorridor {
    OdCorridor {
        name: "MIA→NYC (I-95 Southeast spine)".into(),
        origin: "Miami, FL".into(),
        destination: "New York City, NY".into(),
        fixed_overhead_hours: 2.5,
        hos_driving_hours: 11.0,
        hos_rest_hours: 10.0,
        segments: vec![
            CorridorSegment {
                name: "Miami metro (I-95)".into(),
                miles: 50.0,
                base_vc: 1.25,
                free_flow_mph: 65.0,
                incident_prob: 0.22,
                incident_delay_mean_hours: 1.5,
                incident_delay_std_hours: 1.0,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
            CorridorSegment {
                name: "I-95 FL/GA rural".into(),
                miles: 380.0,
                base_vc: 0.48,
                free_flow_mph: 70.0,
                incident_prob: 0.05,
                incident_delay_mean_hours: 1.0,
                incident_delay_std_hours: 0.6,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.42,
            },
            CorridorSegment {
                name: "I-95 SC/NC/VA".into(),
                miles: 440.0,
                base_vc: 0.55,
                free_flow_mph: 70.0,
                incident_prob: 0.06,
                incident_delay_mean_hours: 1.1,
                incident_delay_std_hours: 0.7,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.48,
            },
            CorridorSegment {
                name: "DC/Baltimore corridor".into(),
                miles: 200.0,
                base_vc: 1.15,
                free_flow_mph: 65.0,
                incident_prob: 0.22,
                incident_delay_mean_hours: 1.6,
                incident_delay_std_hours: 1.1,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
            CorridorSegment {
                name: "NJ/NYC approach".into(),
                miles: 100.0,
                base_vc: 1.30,
                free_flow_mph: 60.0,
                incident_prob: 0.25,
                incident_delay_mean_hours: 1.4,
                incident_delay_std_hours: 0.9,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
        ],
    }
}

/// Seattle → Chicago via I-90 (2,060 miles — Pacific NW to Midwest)
pub fn sea_chi() -> OdCorridor {
    OdCorridor {
        name: "SEA→CHI (I-90 Northern Tier)".into(),
        origin: "Seattle, WA".into(),
        destination: "Chicago, IL".into(),
        fixed_overhead_hours: 3.0,
        hos_driving_hours: 11.0,
        hos_rest_hours: 10.0,
        segments: vec![
            CorridorSegment {
                name: "Seattle metro (I-90)".into(),
                miles: 30.0,
                base_vc: 1.10,
                free_flow_mph: 65.0,
                incident_prob: 0.18,
                incident_delay_mean_hours: 1.3,
                incident_delay_std_hours: 0.8,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
            CorridorSegment {
                name: "Snoqualmie Pass (Cascades)".into(),
                miles: 80.0,
                base_vc: 0.75,
                free_flow_mph: 55.0,
                incident_prob: 0.085,
                incident_delay_mean_hours: 12.0,
                incident_delay_std_hours: 8.0,
                managed_lane_bypasses_incident: true,
                managed_lane_vc: 0.68,
            },
            CorridorSegment {
                name: "Eastern WA/ID/MT I-90".into(),
                miles: 480.0,
                base_vc: 0.30,
                free_flow_mph: 80.0,
                incident_prob: 0.05,
                incident_delay_mean_hours: 2.5,
                incident_delay_std_hours: 1.5,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.25,
            },
            CorridorSegment {
                name: "WY/SD rural I-90".into(),
                miles: 760.0,
                base_vc: 0.22,
                free_flow_mph: 80.0,
                incident_prob: 0.04,
                incident_delay_mean_hours: 2.0,
                incident_delay_std_hours: 1.2,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.18,
            },
            CorridorSegment {
                name: "MN/WI I-90 to Chicago".into(),
                miles: 460.0,
                base_vc: 0.45,
                free_flow_mph: 70.0,
                incident_prob: 0.05,
                incident_delay_mean_hours: 0.9,
                incident_delay_std_hours: 0.6,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.40,
            },
            CorridorSegment {
                name: "Chicago south approach".into(),
                miles: 60.0,
                base_vc: 1.05,
                free_flow_mph: 65.0,
                incident_prob: 0.18,
                incident_delay_mean_hours: 1.3,
                incident_delay_std_hours: 0.8,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
        ],
    }
}

/// Dallas → New York via I-30/I-40/I-81 (1,580 miles)
pub fn dal_nyc() -> OdCorridor {
    OdCorridor {
        name: "DAL→NYC (I-30/I-40/I-81 energy-to-finance corridor)".into(),
        origin: "Dallas, TX".into(),
        destination: "New York City, NY".into(),
        fixed_overhead_hours: 3.0,
        hos_driving_hours: 11.0,
        hos_rest_hours: 10.0,
        segments: vec![
            CorridorSegment {
                name: "Dallas metro".into(),
                miles: 40.0,
                base_vc: 1.20,
                free_flow_mph: 65.0,
                incident_prob: 0.22,
                incident_delay_mean_hours: 1.5,
                incident_delay_std_hours: 1.0,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
            CorridorSegment {
                name: "I-30 TX/AR".into(),
                miles: 340.0,
                base_vc: 0.45,
                free_flow_mph: 75.0,
                incident_prob: 0.05,
                incident_delay_mean_hours: 1.0,
                incident_delay_std_hours: 0.6,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.40,
            },
            CorridorSegment {
                name: "TN/VA I-40/I-81".into(),
                miles: 580.0,
                base_vc: 0.50,
                free_flow_mph: 70.0,
                incident_prob: 0.07,
                incident_delay_mean_hours: 1.2,
                incident_delay_std_hours: 0.8,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.44,
            },
            CorridorSegment {
                name: "I-81/I-78 PA/NJ".into(),
                miles: 400.0,
                base_vc: 0.75,
                free_flow_mph: 65.0,
                incident_prob: 0.12,
                incident_delay_mean_hours: 1.3,
                incident_delay_std_hours: 0.8,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.65,
            },
            CorridorSegment {
                name: "NYC approach (I-95/GWB)".into(),
                miles: 80.0,
                base_vc: 1.35,
                free_flow_mph: 55.0,
                incident_prob: 0.28,
                incident_delay_mean_hours: 1.6,
                incident_delay_std_hours: 1.1,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
        ],
    }
}

/// Los Angeles → Seattle via I-5 (1,140 miles — Pacific Coast spine)
pub fn la_sea() -> OdCorridor {
    OdCorridor {
        name: "LA→SEA (I-5 Pacific Coast)".into(),
        origin: "Los Angeles, CA".into(),
        destination: "Seattle, WA".into(),
        fixed_overhead_hours: 2.5,
        hos_driving_hours: 11.0,
        hos_rest_hours: 10.0,
        segments: vec![
            CorridorSegment {
                name: "LA metro (I-5)".into(),
                miles: 50.0,
                base_vc: 1.30,
                free_flow_mph: 65.0,
                incident_prob: 0.25,
                incident_delay_mean_hours: 1.3,
                incident_delay_std_hours: 0.8,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
            CorridorSegment {
                name: "I-5 Central Valley CA".into(),
                miles: 380.0,
                base_vc: 0.50,
                free_flow_mph: 70.0,
                incident_prob: 0.06,
                incident_delay_mean_hours: 0.9,
                incident_delay_std_hours: 0.6,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.45,
            },
            CorridorSegment {
                name: "Siskiyou Pass (OR border)".into(),
                miles: 80.0,
                base_vc: 0.70,
                free_flow_mph: 55.0,
                incident_prob: 0.065,
                incident_delay_mean_hours: 6.0,
                incident_delay_std_hours: 4.0,
                managed_lane_bypasses_incident: true,
                managed_lane_vc: 0.65,
            },
            CorridorSegment {
                name: "I-5 OR/WA".into(),
                miles: 440.0,
                base_vc: 0.45,
                free_flow_mph: 70.0,
                incident_prob: 0.05,
                incident_delay_mean_hours: 0.9,
                incident_delay_std_hours: 0.6,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.40,
            },
            CorridorSegment {
                name: "Seattle metro (I-5)".into(),
                miles: 30.0,
                base_vc: 1.10,
                free_flow_mph: 65.0,
                incident_prob: 0.18,
                incident_delay_mean_hours: 1.2,
                incident_delay_std_hours: 0.8,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.68,
            },
        ],
    }
}

/// New York → Chicago via I-80/I-90 (790 miles — the overnight AV sweet spot)
pub fn ny_chi() -> OdCorridor {
    OdCorridor {
        name: "NY→CHI (I-80/I-90 overnight corridor)".into(),
        origin: "New York City, NY".into(),
        destination: "Chicago, IL".into(),
        fixed_overhead_hours: 2.0,
        hos_driving_hours: 11.0,
        hos_rest_hours: 10.0,
        segments: vec![
            CorridorSegment {
                name: "NJ/PA urban I-80".into(),
                miles: 120.0,
                base_vc: 0.92,
                free_flow_mph: 65.0,
                incident_prob: 0.18,
                incident_delay_mean_hours: 1.2,
                incident_delay_std_hours: 0.8,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
            CorridorSegment {
                name: "PA/OH/IN rural I-80/I-90".into(),
                miles: 560.0,
                base_vc: 0.35,
                free_flow_mph: 75.0,
                incident_prob: 0.04,
                incident_delay_mean_hours: 0.8,
                incident_delay_std_hours: 0.5,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.30,
            },
            CorridorSegment {
                name: "Chicago south approach".into(),
                miles: 80.0,
                base_vc: 1.05,
                free_flow_mph: 65.0,
                incident_prob: 0.18,
                incident_delay_mean_hours: 1.3,
                incident_delay_std_hours: 0.8,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
        ],
    }
}

/// Atlanta → Chicago via I-65 (730 miles — already fast; shows where relay adds little)
pub fn atl_chi() -> OdCorridor {
    OdCorridor {
        name: "ATL→CHI (I-65 Southeast-Midwest spine)".into(),
        origin: "Atlanta, GA".into(),
        destination: "Chicago, IL".into(),
        fixed_overhead_hours: 2.0,
        hos_driving_hours: 11.0,
        hos_rest_hours: 10.0,
        segments: vec![
            CorridorSegment {
                name: "Atlanta metro (I-75/I-285)".into(),
                miles: 40.0,
                base_vc: 1.30,
                free_flow_mph: 65.0,
                incident_prob: 0.28,
                incident_delay_mean_hours: 1.8,
                incident_delay_std_hours: 1.2,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.70,
            },
            CorridorSegment {
                name: "I-65 AL/TN/KY".into(),
                miles: 540.0,
                base_vc: 0.42,
                free_flow_mph: 70.0,
                incident_prob: 0.06,
                incident_delay_mean_hours: 1.0,
                incident_delay_std_hours: 0.6,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.38,
            },
            CorridorSegment {
                name: "I-65 IN to Chicago".into(),
                miles: 150.0,
                base_vc: 0.65,
                free_flow_mph: 70.0,
                incident_prob: 0.08,
                incident_delay_mean_hours: 1.1,
                incident_delay_std_hours: 0.7,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.58,
            },
        ],
    }
}

// ── Passenger corridors ───────────────────────────────────────────────────────

/// Passenger travel model — different from freight:
/// - Bus speed: 55 mph average (stops, acceleration, boarding)
/// - Personal AV: 75 mph sustained on managed lane
/// - No HOS rest stops for AV (automated driving)
/// - Stops at every T1/T2 hub (bus) vs relay-only (freight)
#[derive(Debug, Clone, Copy)]
pub enum PassengerMode {
    /// Intercity express bus with relay driver at T1 hubs
    /// Competitive with Amtrak on medium corridors
    ExpressBus,
    /// Autonomous personal vehicle on managed lane
    /// Driver sleeps/works; takes over at urban exit
    AutonomousVehicle,
    /// Current Amtrak benchmark (for comparison)
    Amtrak {
        schedule_hours: f64,
        reliability_pti: f64,
    },
}

/// Passenger trip result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassengerTripDistribution {
    pub mode: String,
    pub corridor: String,
    pub distance_miles: f64,
    pub p50_hours: f64,
    pub p95_hours: f64,
    pub pti: f64,
    pub free_flow_hours: f64,
    /// Cost per passenger ($)
    pub cost_per_passenger: f64,
    /// Amtrak schedule for comparison (hours)
    pub amtrak_hours: Option<f64>,
    /// Competitive with air? (air typically 6h total door-to-door for 500-mile corridors)
    pub beats_air_under_miles: bool,
}

/// EV vehicle profile for range/charging calculations.
#[derive(Debug, Clone)]
pub struct EvProfile {
    pub name: &'static str,
    /// Practical range at highway speed (miles) — typically 15-20% below EPA
    pub highway_range_miles: f64,
    /// Charge rate at I2.0 DCFC (kW) — limited by onboard charger
    pub charge_rate_kw: f64,
    /// Battery capacity (kWh)
    pub battery_kwh: f64,
    /// Energy use at highway speed (kWh/mile)
    pub kwh_per_mile: f64,
}

impl EvProfile {
    /// Minutes to add N miles of range at DCFC
    pub fn charge_minutes_for_miles(&self, miles_needed: f64, dcfc_kw: f64) -> f64 {
        let kw = self.charge_rate_kw.min(dcfc_kw);
        let kwh_needed = miles_needed * self.kwh_per_mile;
        (kwh_needed / kw) * 60.0
    }

    /// Number of charging stops needed for a given distance
    pub fn stops_needed(&self, distance: f64) -> usize {
        ((distance / self.highway_range_miles).ceil() as usize).saturating_sub(1)
    }

    /// Total charging time (minutes) for a trip of distance miles
    pub fn total_charge_minutes(&self, _distance: f64, dcfc_kw: f64, stops: usize) -> f64 {
        // Each stop: charge from ~10% to ~80% (practical fast-charge window)
        // 70% of battery capacity per stop
        let kwh_per_stop = self.battery_kwh * 0.70;
        let kw = self.charge_rate_kw.min(dcfc_kw);
        let mins_per_stop = (kwh_per_stop / kw) * 60.0;
        stops as f64 * mins_per_stop
    }
}

/// Common EV profiles for I2.0 corridor analysis.
pub fn tesla_model_y() -> EvProfile {
    EvProfile {
        name: "Tesla Model Y (Long Range)",
        highway_range_miles: 290.0, // EPA 330mi, 88% at 75mph
        charge_rate_kw: 250.0,      // V3 Supercharger rate
        battery_kwh: 82.0,
        kwh_per_mile: 0.283,
    }
}

pub fn tesla_semi() -> EvProfile {
    EvProfile {
        name: "Tesla Semi (500mi range)",
        highway_range_miles: 480.0, // at highway freight speed
        charge_rate_kw: 1000.0,     // Megacharger (1MW)
        battery_kwh: 900.0,
        kwh_per_mile: 1.875,
    }
}

pub fn average_ev_2026() -> EvProfile {
    EvProfile {
        name: "Average EV 2026 (Chevy Equinox / Hyundai Ioniq 5)",
        highway_range_miles: 220.0, // conservative highway range
        charge_rate_kw: 150.0,      // 150kW = I2.0 minimum standard
        battery_kwh: 78.0,
        kwh_per_mile: 0.355,
    }
}

/// EV charging analysis for a corridor.
#[derive(Debug, Serialize, Deserialize)]
pub struct EvChargingAnalysis {
    pub ev_name: String,
    pub corridor_miles: f64,
    /// Stops needed with I2.0 guaranteed DCFC (50-mile spacing)
    pub stops_i20: usize,
    /// Stops needed on current infrastructure (avg 85-mile spacing on rural T1)
    pub stops_current: usize,
    /// Can current-gen EV complete trip without range anxiety on I2.0?
    pub i20_viable: bool,
    /// Total charging time on I2.0 (minutes)
    pub charge_minutes_i20: f64,
    /// Charging overhead added to AV trip (hours)
    pub charge_overhead_hours: f64,
    /// Is the overnight scenario viable? (charge while sleeping at hub)
    pub overnight_scenario: bool,
    pub overnight_note: String,
}

pub fn analyze_ev_charging(
    corridor: &OdCorridor,
    ev: &EvProfile,
    i20_dcfc_kw: f64,
) -> EvChargingAnalysis {
    let miles = corridor.total_miles();
    let stops = ev.stops_needed(miles);
    // Current infrastructure: rural T1 has avg 85-mile DCFC spacing; some 120+ mile gaps
    let current_viable = ev.highway_range_miles >= 120.0; // can get through worst gap
    let charge_mins = ev.total_charge_minutes(miles, i20_dcfc_kw, stops);

    // Overnight scenario: charging happens automatically during relay hub stops
    // Hub stop = 20-25 min (driver swap for freight); AV plugs in automatically
    // Is 20min enough to add the miles needed between stops?
    let miles_per_stop = miles / (stops + 1) as f64;
    let charge_needed_per_stop = miles_per_stop * ev.kwh_per_mile;
    let charge_per_hub_stop = i20_dcfc_kw.min(ev.charge_rate_kw) * (20.0 / 60.0); // 20 min
    let overnight_ok = charge_per_hub_stop >= charge_needed_per_stop * 1.1; // 10% buffer

    let overnight_note = if overnight_ok {
        format!(
            "Auto-charges at hub stops ({} stops × 20min = {:.0}min total — no wake needed)",
            stops, charge_mins
        )
    } else {
        format!(
            "Needs {:.0}min/stop; hub stop provides {:.0}min — may need brief charge stop",
            charge_needed_per_stop / i20_dcfc_kw.min(ev.charge_rate_kw) * 60.0,
            20.0
        )
    };

    EvChargingAnalysis {
        ev_name: ev.name.to_string(),
        corridor_miles: miles,
        stops_i20: stops,
        stops_current: if current_viable { stops + 1 } else { stops + 3 }, // more uncertainty
        i20_viable: true, // 50-mile spacing guarantees viability for any EV with 100+ mile range
        charge_minutes_i20: charge_mins,
        charge_overhead_hours: charge_mins / 60.0,
        overnight_scenario: overnight_ok,
        overnight_note,
    }
}

/// Run passenger SLA simulation on a freight corridor definition (reuses segment geography).
pub fn run_passenger_simulation(
    corridor: &OdCorridor,
    mode: PassengerMode,
    n_trips: usize,
    seed: u64,
    amtrak_hours: Option<f64>,
) -> PassengerTripDistribution {
    use rand::SeedableRng;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    let miles = corridor.total_miles();

    let elapsed_times: Vec<f64> = (0..n_trips)
        .map(|_| {
            match mode {
                PassengerMode::ExpressBus => {
                    // Bus at 55 mph effective, relay driver at T1 hubs
                    // More stops than freight: boarding time adds ~5 min per hub stop
                    let relay_stations = ((miles / 400.0).ceil() as usize).max(1);
                    let mut drive_hours = 0.0f64;
                    let mut delay = 0.0f64;

                    for seg in &corridor.segments {
                        let vc = {
                            let v = rng.gen_range(0.80..=1.15f64);
                            let surge = if rng.gen_bool(0.08) { 1.30 } else { 1.0 };
                            // Bus uses managed lane where available
                            seg.managed_lane_vc * v * surge
                        };
                        let ff = seg.miles / 55.0_f64.min(seg.free_flow_mph);
                        drive_hours += bpr_time(ff, vc.min(1.3));
                        if !seg.managed_lane_bypasses_incident
                            && rng.gen_bool(seg.incident_prob * 0.5)
                        {
                            // Bus can reroute faster than trucks; half the delay
                            delay += sample_lognormal(
                                seg.incident_delay_mean_hours * 0.5,
                                seg.incident_delay_std_hours * 0.5,
                                &mut rng,
                            )
                            .max(0.05);
                        }
                    }
                    // Relay driver swaps + passenger boarding overhead
                    let swap_overhead = relay_stations as f64 * (20.0 + 8.0) / 60.0; // 28 min/stop
                    let terminal_overhead = 1.5; // boarding at origin + alighting at destination
                    drive_hours + delay + swap_overhead + terminal_overhead
                }

                PassengerMode::AutonomousVehicle => {
                    // AV at 75 mph on managed lane — no HOS stops, no driver fatigue
                    // Occasional slowdown: construction, weather, sensor limitation zone
                    let mut drive_hours = 0.0f64;
                    let mut delay = 0.0f64;

                    for seg in &corridor.segments {
                        let base_speed = 75.0_f64.min(seg.free_flow_mph * 1.1);
                        let vc = seg.managed_lane_vc * rng.gen_range(0.90..=1.05f64);
                        let ff = seg.miles / base_speed;
                        drive_hours += bpr_time(ff, vc.min(1.0)); // AV disengages above V/C 1.0

                        // AV encounters: severe weather (must slow to 45mph), sensor degradation
                        if rng.gen_bool(seg.incident_prob * 0.3) {
                            // AV slows but doesn't stop; partial delay
                            delay += rng.gen_range(0.1..0.5_f64);
                        }
                        // Donner/mountain pass: AV may need to slow; no closure because hardened
                        if seg.name.contains("Donner") || seg.name.contains("Pass") {
                            drive_hours *= 1.05; // 5% speed reduction for grade/weather caution
                        }
                    }
                    // AV overhead: managed lane entry/exit (hub junction)
                    let hub_stops = ((miles / 500.0).ceil() as usize).max(0);
                    let hub_overhead = hub_stops as f64 * 0.1; // 6 minutes per hub junction
                    let terminal_overhead = 0.5; // park and walk at destination
                    drive_hours + delay + hub_overhead + terminal_overhead
                }

                PassengerMode::Amtrak {
                    schedule_hours,
                    reliability_pti,
                } => {
                    // Amtrak: scheduled time × PTI variance
                    let variance = rng.gen_range(0.85..=(reliability_pti * 1.1));
                    schedule_hours * variance
                }
            }
        })
        .collect();

    let mut sorted = elapsed_times.clone();
    sorted.sort_by(f64::total_cmp);
    let n = sorted.len() as f64;
    let p50 = sorted[(0.50 * (n - 1.0)) as usize];
    let p95 = sorted[(0.95 * (n - 1.0)) as usize];
    let ff = match mode {
        PassengerMode::ExpressBus => miles / 55.0 + 1.5,
        PassengerMode::AutonomousVehicle => miles / 75.0 + 0.5,
        PassengerMode::Amtrak { schedule_hours, .. } => schedule_hours,
    };
    let cost = match mode {
        PassengerMode::ExpressBus => miles * 0.12, // $0.12/mile
        PassengerMode::AutonomousVehicle => miles * 0.18 + 15.0, // $0.18/mi fuel+wear + managed lane toll
        PassengerMode::Amtrak { .. } => miles * 0.15,            // Amtrak avg
    };
    let mode_label = match mode {
        PassengerMode::ExpressBus => "Express bus (relay)",
        PassengerMode::AutonomousVehicle => "AV personal vehicle",
        PassengerMode::Amtrak { .. } => "Amtrak (current)",
    };

    PassengerTripDistribution {
        mode: mode_label.to_string(),
        corridor: corridor.name.clone(),
        distance_miles: miles,
        p50_hours: p50,
        p95_hours: p95,
        pti: p95 / ff,
        free_flow_hours: ff,
        cost_per_passenger: cost,
        amtrak_hours,
        beats_air_under_miles: p95 <= 6.0 && miles <= 600.0, // air ~6h door-to-door
    }
}

/// Full comparison across all driver modes and lane types.
#[derive(Debug, Serialize)]
pub struct OdComparison {
    pub corridor_name: String,
    /// Solo driver, current GP lanes — the baseline
    pub solo_gp: TransitDistribution,
    /// Solo driver, I2.0 managed lanes
    pub solo_managed: TransitDistribution,
    /// Team drivers (2-person crew), I2.0 managed lanes
    pub team_managed: TransitDistribution,
    /// Relay network (fresh driver at each hub), I2.0 managed lanes
    pub relay_managed: TransitDistribution,
    /// Relay network, GP lanes — what relay buys without managed lanes
    pub relay_gp: TransitDistribution,
}

impl OdComparison {
    pub fn run(corridor: &OdCorridor, n: usize, seed: u64) -> Self {
        // Relay stations: roughly every 500 miles on T1 corridors
        // NY→LA (2800mi): Chicago, Omaha, Salt Lake City, Reno, Sacramento = 5 stations
        // HOU→CHI (1090mi): Dallas, KC = 2 stations
        // HOU→CHI I-69 (870mi): Texarkana, Indianapolis = 2 stations
        let relay_stations = ((corridor.total_miles() / 500.0).ceil() as usize).max(1);
        let relay = DriverMode::Relay {
            stations: relay_stations,
            swap_minutes: 20.0, // 20-minute target swap; conservative
        };

        OdComparison {
            corridor_name: corridor.name.clone(),
            solo_gp: run_od_simulation_with_driver(corridor, false, &DriverMode::Solo, n, seed),
            solo_managed: run_od_simulation_with_driver(
                corridor,
                true,
                &DriverMode::Solo,
                n,
                seed + 1,
            ),
            team_managed: run_od_simulation_with_driver(
                corridor,
                true,
                &DriverMode::Team,
                n,
                seed + 2,
            ),
            relay_gp: run_od_simulation_with_driver(corridor, false, &relay, n, seed + 3),
            relay_managed: run_od_simulation_with_driver(corridor, true, &relay, n, seed + 4),
        }
    }
}

/// Relay network design parameters for a corridor.
pub struct RelayNetwork {
    pub corridor_miles: f64,
    pub stations: usize,
    pub avg_leg_miles: f64,
    pub avg_leg_hours: f64,
    pub swap_time_min: f64,
    /// Estimated station infrastructure cost ($M each)
    pub station_cost_m: f64,
    /// Total network capex ($M)
    pub total_capex_m: f64,
}

impl RelayNetwork {
    /// Build relay network stats from free-flow elapsed hours (used by CLI display).
    pub fn for_corridor_miles(free_flow_hours: f64) -> Self {
        // Approximate miles from hours at 65 mph average across all segment types
        let miles = free_flow_hours * 58.0; // ~58 mph effective speed accounting for rest
        Self::for_miles(miles)
    }

    fn for_miles(miles: f64) -> Self {
        let stations = ((miles / 500.0).ceil() as usize).max(1);
        let avg_leg = miles / (stations + 1) as f64;
        let station_cost = 5.0;
        RelayNetwork {
            corridor_miles: miles,
            stations,
            avg_leg_miles: avg_leg,
            avg_leg_hours: avg_leg / 65.0,
            swap_time_min: 20.0,
            station_cost_m: station_cost,
            total_capex_m: stations as f64 * station_cost,
        }
    }

    pub fn for_corridor(corridor: &OdCorridor) -> Self {
        Self::for_miles(corridor.total_miles())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_corridor(miles: f64) -> OdCorridor {
        OdCorridor {
            name: "Test freight corridor".to_string(),
            origin: "Origin".to_string(),
            destination: "Destination".to_string(),
            segments: vec![CorridorSegment {
                name: "Rural T1 segment".to_string(),
                miles,
                base_vc: 0.55,
                free_flow_mph: 65.0,
                incident_prob: 0.0,
                incident_delay_mean_hours: 0.0,
                incident_delay_std_hours: 0.0,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.45,
            }],
            hos_driving_hours: 11.0,
            hos_rest_hours: 10.0,
            fixed_overhead_hours: 2.0,
        }
    }

    #[test]
    fn relay_network_spacing_uses_500_mile_station_intervals() {
        let corridor = test_corridor(1_000.0);

        let relay = RelayNetwork::for_corridor(&corridor);

        assert_eq!(relay.corridor_miles, 1_000.0);
        assert_eq!(relay.stations, 2);
        assert!((relay.avg_leg_miles - (1_000.0 / 3.0)).abs() < 1e-9);
        assert!((relay.avg_leg_hours - (1_000.0 / 3.0 / 65.0)).abs() < 1e-9);
        assert_eq!(relay.swap_time_min, 20.0);
        assert_eq!(relay.station_cost_m, 5.0);
        assert_eq!(relay.total_capex_m, 10.0);
    }

    #[test]
    fn relay_network_always_allocates_at_least_one_station() {
        let corridor = test_corridor(120.0);

        let relay = RelayNetwork::for_corridor(&corridor);

        assert_eq!(relay.stations, 1);
        assert_eq!(relay.avg_leg_miles, 60.0);
        assert!((relay.avg_leg_hours - (60.0 / 65.0)).abs() < 1e-9);
        assert_eq!(relay.total_capex_m, 5.0);
    }

    #[test]
    fn driver_relay_intervention_preserves_station_count_and_swap_minutes() {
        let corridor = test_corridor(1_000.0);

        let (_corridor, driver_mode) = apply_interventions(
            &corridor,
            &[
                Intervention::DriverRelay {
                    stations: 3,
                    swap_minutes: 15.0,
                },
                Intervention::TeamDrivers,
            ],
        );

        assert_eq!(
            driver_mode,
            DriverMode::Relay {
                stations: 3,
                swap_minutes: 15.0,
            }
        );
    }

    #[test]
    fn team_drivers_do_not_override_existing_relay_mode() {
        let corridor = test_corridor(1_000.0);

        let (_corridor, driver_mode) = apply_interventions(
            &corridor,
            &[
                Intervention::TeamDrivers,
                Intervention::DriverRelay {
                    stations: 4,
                    swap_minutes: 18.0,
                },
            ],
        );

        assert_eq!(
            driver_mode,
            DriverMode::Relay {
                stations: 4,
                swap_minutes: 18.0,
            }
        );
    }
}
