pub mod assignment;
pub mod chaos;
/// ROUTE Simulation Engine
///
/// Validates Interstate 2.0 standards through traffic simulation.
///
/// Three simulation modes:
///   1. Scenario — named incident or intervention, single run, deterministic
///   2. Chaos    — Monte Carlo random failures, distribution of outcomes
///   3. Intervention — before/after comparison for a specific I2.0 feature
///
/// Core algorithm: Wardrop User Equilibrium via Frank-Wolfe.
///   All drivers/trucks minimize their own travel time.
///   Equilibrium: no user can reduce their travel time by switching routes.
///   Implemented as iterative: all-or-nothing → BPR travel times → repeat.
pub mod demand;
pub mod hub;
pub mod incident;
pub mod metrics;
pub mod od;
pub mod scenario;
pub mod scenarios;

pub use assignment::{wardrop_equilibrium, FlowState};
pub use chaos::{run_chaos, ChaosConfig, ChaosResult};
pub use hub::{
    compute_network_summary, load_hubs, proposed_hubs, run_hub_outage_sensitivity, t1_diamond_hubs,
    HubOutageConfig, HubOutageResult, HubOutageSummary, HubStaffing, NetworkSummary, RelayHub,
};
pub use incident::{
    apply_bundle_incident, apply_incident, restore_incident, BundleIncidentSpec, IncidentSpec,
};
pub use metrics::{corridor_pti, freight_cost_delta, network_throughput, SimMetrics};
pub use od::{
    analyze_ev_charging, analyze_ev_rest_outage, apply_interventions, apply_seasonal, atl_chi,
    average_ev_2026, chi_la, dal_nyc, hou_chi_current, hou_chi_i69, la_sea, load_corridor, mia_nyc,
    ny_chi, ny_la_corridor, run_intervention_stack, run_od_simulation,
    run_od_simulation_with_driver, run_passenger_simulation, sea_chi, sla_proof_table,
    tesla_model_y, tesla_semi, DriverMode, EvChargingAnalysis, EvProfile, EvRestOutageConfig,
    EvRestOutageResult, Intervention, InterventionBenchmark, InterventionResult, OdComparison,
    OdCorridor, PassengerMode, PassengerTripDistribution, RelayNetwork, SlaProofRow,
    TransitDistribution,
};
pub use scenario::{run_scenario, scenario_validation_warnings, Scenario, ScenarioResult};
