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
use rand::distributions::Distribution;
use serde::{Serialize, Deserialize};

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
        self.segments.iter().map(|s| s.miles / s.free_flow_mph).sum()
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
            Self::ManagedFreightLanes  => "Managed freight lanes",
            Self::DriverRelay { .. }   => "Driver relay network",
            Self::TeamDrivers          => "Team drivers (2-person)",
            Self::DonnerHardening      => "Donner hardening",
            Self::DonnerTunnel         => "Donner tunnel",
            Self::DiamondInterchanges  => "Diamond interchanges",
            Self::IntelligentRouting   => "Intelligent routing (V2I)",
            Self::Platooning           => "Truck platooning",
        }
    }

    pub fn capex_label(&self) -> &str {
        match self {
            Self::ManagedFreightLanes  => "$121B (7 T1 corridors)",
            Self::DriverRelay { .. }   => "$40M (8 relay stations)",
            Self::TeamDrivers          => "$0 (operational change)",
            Self::DonnerHardening      => "$800M (snowshed + warning)",
            Self::DonnerTunnel         => "$4B (12-mile tunnel)",
            Self::DiamondInterchanges  => "$930M (Phase 1: ATL/JAX/TOL)",
            Self::IntelligentRouting   => "$200M (V2I hardware + software)",
            Self::Platooning           => "$50M (V2V + platooning systems)",
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
            Intervention::DriverRelay { stations, swap_minutes } => {
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
                        seg.incident_prob *= 0.50;          // 50% fewer closures
                        seg.incident_delay_mean_hours *= 0.67; // 18h→12h mean
                    }
                }
            }
            Intervention::DonnerTunnel => {
                for seg in &mut corridor.segments {
                    if seg.name.contains("Donner") {
                        seg.incident_prob = 0.0;  // tunnel bypasses all weather risk
                        seg.managed_lane_bypasses_incident = true;
                        // Tunnel also eliminates grade: slightly faster
                        seg.free_flow_mph = 65.0;
                    }
                }
            }
            Intervention::DiamondInterchanges => {
                // Diamond interchanges at major bottleneck nodes reduce incident prob 60%
                for seg in &mut corridor.segments {
                    if seg.name.contains("interchange") || seg.name.contains("metro")
                       || seg.name.contains("approach") || seg.name.contains("bypass") {
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
                    if seg.base_vc < 0.60 {  // only effective on free-flowing segments
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
    pub p95_delta_hours: f64,  // vs baseline (negative = improvement)
    pub pct_under_48h: f64,
    pub sla_achieved: bool,
}

impl InterventionBenchmark {
    pub fn run(corridor: &OdCorridor, n_trips: usize, seed: u64) -> Self {
        let relay_count = ((corridor.total_miles() / 500.0).ceil() as usize).max(1);

        // Define all single interventions to test
        let singles: Vec<(&str, &str, Vec<Intervention>)> = vec![
            ("Baseline (Solo/GP)", "$0", vec![]),
            ("Managed lanes only", "$121B", vec![Intervention::ManagedFreightLanes]),
            ("Team drivers only", "$0", vec![Intervention::TeamDrivers]),
            ("Driver relay only", "$40M", vec![
                Intervention::DriverRelay { stations: relay_count, swap_minutes: 20.0 },
            ]),
            ("Intelligent routing only", "$200M", vec![Intervention::IntelligentRouting]),
            ("Diamond interchanges", "$930M", vec![Intervention::DiamondInterchanges]),
            ("Donner hardening", "$800M", vec![Intervention::DonnerHardening]),
            ("Donner tunnel", "$4B", vec![Intervention::DonnerTunnel]),
            ("Platooning", "$50M", vec![Intervention::Platooning]),
            // Combinations
            ("Relay + Managed lanes", "$121B+$40M", vec![
                Intervention::ManagedFreightLanes,
                Intervention::DriverRelay { stations: relay_count, swap_minutes: 20.0 },
            ]),
            ("Relay + Managed + Donner tunnel", "$125B", vec![
                Intervention::ManagedFreightLanes,
                Intervention::DonnerTunnel,
                Intervention::DriverRelay { stations: relay_count, swap_minutes: 20.0 },
            ]),
            ("Relay + Diamonds + Routing", "$1.2B", vec![
                Intervention::DriverRelay { stations: relay_count, swap_minutes: 20.0 },
                Intervention::DiamondInterchanges,
                Intervention::IntelligentRouting,
            ]),
            ("Full I2.0 stack", "$126B+$1.2B", vec![
                Intervention::ManagedFreightLanes,
                Intervention::DonnerTunnel,
                Intervention::DiamondInterchanges,
                Intervention::IntelligentRouting,
                Intervention::Platooning,
                Intervention::DriverRelay { stations: relay_count, swap_minutes: 15.0 },
            ]),
        ];

        let baseline_dist = run_intervention_stack(corridor, &[], n_trips, seed);
        let baseline_p95 = baseline_dist.p95_hours;

        let results = singles.into_iter().enumerate().map(|(i, (label, capex, interventions))| {
            let dist = run_intervention_stack(corridor, &interventions, n_trips, seed + i as u64);
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
        }).collect();

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
            ).max(0.1);
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
            swaps as f64 * (4.0 / 60.0)  // 4-minute swap, truck barely slows
        }
        DriverMode::Relay { stations, swap_minutes } => {
            // Fresh driver at each relay station; swap takes swap_minutes.
            // Relay is always at a T1 hub so no search time.
            // Small variance: swap time can run +5 min if paperwork delayed.
            let swap_variance = rng.gen_range(0.0..5.0_f64) / 60.0; // 0-5 min extra
            *stations as f64 * (swap_minutes / 60.0 + swap_variance)
        }
    };

    let elapsed = driving_hours + hos_overhead + delay_hours + corridor.fixed_overhead_hours;
    TripResult { elapsed_hours: elapsed, driving_hours, delay_hours, incidents, managed_lanes }
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
    pub pti: f64,   // p95 / free_flow
    pub commitment_window_hours: f64,  // = p95
    pub commitment_window_days: f64,
    pub pct_under_48h: f64,   // fraction of trips completing under 48 hours elapsed
    pub pct_sla_met: f64,     // fraction meeting the tier SLA (configurable)
    pub managed_lanes: bool,
}

impl TransitDistribution {
    pub fn sla_hours(&self) -> f64 { self.commitment_window_hours }
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

    elapsed_times.sort_by(|a, b| a.partial_cmp(b).unwrap());

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
                incident_prob: 0.18,  // high urban incident rate
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
                free_flow_mph: 75.0,  // higher rural speed limit (NE/WY 80mph)
                incident_prob: 0.06,  // weather (blizzard, ice)
                incident_delay_mean_hours: 3.5,
                incident_delay_std_hours: 2.0,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: 0.25,
            },
            CorridorSegment {
                name: "Donner Pass (Sierra Nevada, I-80)".into(),
                miles: 65.0,
                base_vc: 0.80,
                free_flow_mph: 55.0,  // mountain grade; lower speed
                // 50 closures/year × mean 18h / (8760h/yr) = 10.3% closure probability
                // But per trip: closure hits if you arrive during a closure window
                // ~50 events × 18h / 8760h = 10.3% chance on any given arrival
                incident_prob: 0.103,
                incident_delay_mean_hours: 18.0,  // mean closure: wait or reroute
                incident_delay_std_hours: 12.0,
                managed_lane_bypasses_incident: true,  // managed lane = tunnel
                managed_lane_vc: 0.72,
            },
            CorridorSegment {
                name: "Bay Area approach (I-80→I-580, Emeryville)".into(),
                miles: 90.0,
                base_vc: 1.15,   // chronically over capacity
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
                base_vc: 1.30,   // highest V/C segment
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
                base_vc: 1.35,   // ATRI top-10 bottleneck
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
                incident_prob: 0.08,  // tornado/severe wx corridor
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
            base_vc: 0.40,  // new road, lower initial V/C
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
            swap_minutes: 20.0,  // 20-minute target swap; conservative
        };

        OdComparison {
            corridor_name: corridor.name.clone(),
            solo_gp:       run_od_simulation_with_driver(corridor, false, &DriverMode::Solo,  n, seed),
            solo_managed:  run_od_simulation_with_driver(corridor, true,  &DriverMode::Solo,  n, seed+1),
            team_managed:  run_od_simulation_with_driver(corridor, true,  &DriverMode::Team,  n, seed+2),
            relay_gp:      run_od_simulation_with_driver(corridor, false, &relay,              n, seed+3),
            relay_managed: run_od_simulation_with_driver(corridor, true,  &relay,              n, seed+4),
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
