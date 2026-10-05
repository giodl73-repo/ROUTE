//! Bounded, synthetic fixed-route examples; no national routing or SLA certification.
use route_kernel::{
    bpr_travel_time, run_hub_outage_sensitivity, BprParams, HubOutageConfig, RelayHub,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub corridor: u8,
    pub demand_pct: f64,
    pub capacity_pct: f64,
    pub outage_hours: f64,
    pub reserve_pct: f64,
    pub adjacent_pct: f64,
    pub target_hours: f64,
}
impl Default for Input {
    fn default() -> Self {
        Self {
            corridor: 0,
            demand_pct: 100.,
            capacity_pct: 100.,
            outage_hours: 0.,
            reserve_pct: 15.,
            adjacent_pct: 35.,
            target_hours: 8.,
        }
    }
}
#[derive(Debug, Serialize)]
pub struct Leg {
    pub name: &'static str,
    pub miles: f64,
    pub flow_vph: f64,
    pub capacity_vph: f64,
    pub vc_ratio: f64,
    pub hours: f64,
}
#[derive(Debug, Serialize)]
pub struct TripComparison {
    pub mode: &'static str,
    pub typical: route_kernel::od::TripResult,
    pub delayed: route_kernel::od::TripResult,
}
#[derive(Debug, Serialize)]
pub struct Run {
    pub trips: Vec<TripComparison>,
    pub hours: f64,
    pub delay_hours: f64,
    pub target_gap_hours: f64,
    pub daily_swaps: u32,
    pub freight_drivers: u32,
    pub affected_swaps: f64,
    pub missed_swaps: f64,
    pub affected_retention: f64,
    pub legs: Vec<Leg>,
}
#[derive(Debug, Serialize)]
pub struct Output {
    pub model: &'static str,
    pub corridor_name: &'static str,
    pub input: Input,
    pub baseline: Run,
    pub scenario: Run,
}
fn validate(i: &Input) -> Result<(), String> {
    if i.corridor > 2 {
        return Err("Choose a known synthetic corridor".into());
    }
    for (name, value, min, max) in [
        ("Demand", i.demand_pct, 25., 200.),
        ("Capacity", i.capacity_pct, 25., 200.),
        ("Outage", i.outage_hours, 0., 24.),
        ("Reserve", i.reserve_pct, 0., 100.),
        ("Adjacent absorption", i.adjacent_pct, 0., 100.),
        ("Travel target", i.target_hours, 1., 48.),
    ] {
        if !value.is_finite() || value < min || value > max {
            return Err(format!("{name} must be between {min} and {max}"));
        }
    }
    Ok(())
}
fn run(i: &Input) -> Run {
    // Deliberately invented teaching inputs: miles, directional vehicles/hour, capacity/hour.
    let fixtures = [
        [
            (120., 2600., 3800.),
            (180., 3500., 3800.),
            (140., 2800., 3800.),
        ],
        [
            (60., 3200., 3800.),
            (90., 3600., 3800.),
            (70., 3400., 3800.),
        ],
        [
            (200., 1600., 3800.),
            (240., 2100., 3800.),
            (180., 1900., 3800.),
        ],
    ];
    let mut free_hours = 0.;
    let legs: Vec<_> = fixtures[i.corridor as usize]
        .iter()
        .enumerate()
        .map(|(index, &(miles, flow, capacity))| {
            free_hours += miles / 65.;
            let flow_vph = flow * i.demand_pct / 100.;
            let capacity_vph = capacity * i.capacity_pct / 100.;
            Leg {
                name: ["West → Relay A", "Relay A → Relay B", "Relay B → East"][index],
                miles,
                flow_vph,
                capacity_vph,
                vc_ratio: flow_vph / capacity_vph,
                hours: bpr_travel_time(miles / 65., flow_vph, capacity_vph, &BprParams::default()),
            }
        })
        .collect();
    let hours = legs.iter().map(|l| l.hours).sum::<f64>();
    let staffing = RelayHub::new(
        "Relay A (synthetic)",
        vec!["Example"],
        "synthetic",
        1200. * i.demand_pct / 100.,
        0.,
        180.,
    )
    .freight_drivers_needed();
    let outage = run_hub_outage_sensitivity(
        std::slice::from_ref(&staffing),
        HubOutageConfig {
            outage_hours: i.outage_hours,
            reserve_driver_fraction: i.reserve_pct / 100.,
            adjacent_absorption_fraction: i.adjacent_pct / 100.,
        },
    );
    use route_kernel::od::{sample_trip_comparison, CorridorSegment, DriverMode, OdCorridor};
    let corridor = OdCorridor {
        name: "Synthetic fixed corridor".into(),
        origin: "West".into(),
        destination: "East".into(),
        hos_driving_hours: 11.,
        hos_rest_hours: 10.,
        fixed_overhead_hours: 2.,
        segments: legs
            .iter()
            .map(|leg| CorridorSegment {
                name: leg.name.into(),
                miles: leg.miles,
                base_vc: leg.vc_ratio,
                free_flow_mph: 65.,
                incident_prob: 0.1,
                incident_delay_mean_hours: 1.,
                incident_delay_std_hours: 0.5,
                managed_lane_bypasses_incident: false,
                managed_lane_vc: leg.vc_ratio,
            })
            .collect(),
    };
    let trips = [
        ("Solo", DriverMode::Solo),
        ("Team", DriverMode::Team),
        (
            "Relay",
            DriverMode::Relay {
                stations: 2,
                swap_minutes: 20.,
            },
        ),
    ]
    .iter()
    .map(|(mode, driver)| {
        let (typical, delayed) = sample_trip_comparison(&corridor, driver);
        TripComparison {
            mode,
            typical,
            delayed,
        }
    })
    .collect();
    Run {
        trips,
        hours,
        delay_hours: hours - free_hours,
        target_gap_hours: (hours - i.target_hours).max(0.),
        daily_swaps: staffing.daily_total_swaps,
        freight_drivers: staffing.freight_relay_drivers,
        affected_swaps: outage.total_affected_swaps,
        missed_swaps: outage.total_missed_swaps,
        affected_retention: outage.network_throughput_retention,
        legs,
    }
}
pub fn evaluate(i: Input) -> Result<Output, String> {
    validate(&i)?;
    let baseline_input = Input {
        corridor: i.corridor,
        target_hours: i.target_hours,
        ..Input::default()
    };
    Ok(Output {
        model: "route-fixed-corridor-v1",
        corridor_name: [
            "Long-haul example",
            "Urban bottleneck example",
            "Rural distance example",
        ][i.corridor as usize],
        baseline: run(&baseline_input),
        scenario: run(&i),
        input: i,
    })
}
#[cfg_attr(feature = "wasm", wasm_bindgen::prelude::wasm_bindgen)]
pub fn evaluate_json(json: &str) -> Result<String, String> {
    if json.len() > 4096 {
        return Err("Scenario exceeds 4 KB".into());
    }
    let i = serde_json::from_str(json).map_err(|e| e.to_string())?;
    serde_json::to_string(&evaluate(i)?).map_err(|e| e.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn trip_comparison_is_reproducible_and_accounts_for_all_time() {
        let input = Input {
            corridor: 2,
            demand_pct: 200.,
            capacity_pct: 25.,
            ..Input::default()
        };
        let a = evaluate(input.clone()).unwrap();
        let b = evaluate(input).unwrap();
        for (left, right) in a.scenario.trips.iter().zip(&b.scenario.trips) {
            assert_eq!(left.typical.elapsed_hours, right.typical.elapsed_hours);
            assert!(left.delayed.elapsed_hours >= left.typical.elapsed_hours);
            for trip in [&left.typical, &left.delayed] {
                assert!(
                    (trip.elapsed_hours
                        - trip.driving_hours
                        - trip.rest_swap_hours
                        - trip.delay_hours
                        - trip.fixed_overhead_hours)
                        .abs()
                        < 1e-9
                );
            }
        }
        assert!(a.scenario.trips[0].typical.rest_swap_hours >= 10.);
        assert!(
            a.scenario.trips[1].typical.elapsed_hours < a.scenario.trips[0].typical.elapsed_hours
        );
        assert!(
            a.scenario.trips[2].typical.elapsed_hours < a.scenario.trips[0].typical.elapsed_hours
        );
    }
    #[test]
    fn baseline_and_sensitivity() {
        let default = evaluate(Input::default()).unwrap();
        assert_eq!(default.baseline.hours, default.scenario.hours);
        assert_eq!(default.scenario.missed_swaps, 0.);
        let stressed = evaluate(Input {
            demand_pct: 150.,
            capacity_pct: 75.,
            outage_hours: 8.,
            ..Input::default()
        })
        .unwrap();
        assert!(stressed.scenario.hours > default.scenario.hours);
        assert!(stressed.scenario.missed_swaps > 0.);
        let rescued = evaluate(Input {
            adjacent_pct: 100.,
            ..stressed.input
        })
        .unwrap();
        assert_eq!(rescued.scenario.missed_swaps, 0.);
        assert_eq!(rescued.scenario.affected_retention, 1.);
    }
    #[test]
    fn malformed_and_unbounded_inputs_fail() {
        assert!(evaluate(Input {
            corridor: 3,
            ..Input::default()
        })
        .is_err());
        assert!(evaluate(Input {
            capacity_pct: 0.,
            ..Input::default()
        })
        .is_err());
        assert!(evaluate(Input {
            target_hours: f64::NAN,
            ..Input::default()
        })
        .is_err());
        assert!(evaluate_json("{}").is_err());
        assert!(evaluate_json(&"x".repeat(4097)).is_err());
    }
    #[test]
    fn travel_and_outage_are_separate_models() {
        let a = evaluate(Input::default()).unwrap();
        let b = evaluate(Input {
            outage_hours: 24.,
            reserve_pct: 0.,
            adjacent_pct: 0.,
            ..Input::default()
        })
        .unwrap();
        assert_eq!(a.scenario.hours, b.scenario.hours);
        assert_eq!(b.scenario.missed_swaps, b.scenario.daily_swaps as f64);
        assert_eq!(b.scenario.affected_retention, 0.);
    }
}
