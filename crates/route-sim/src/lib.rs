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
pub mod assignment;
pub mod incident;
pub mod chaos;
pub mod metrics;
pub mod scenario;
pub mod scenarios;

pub use scenario::{Scenario, ScenarioResult, run_scenario};
pub use chaos::{ChaosConfig, ChaosResult, run_chaos};
pub use metrics::{SimMetrics, corridor_pti, network_throughput, freight_cost_delta};
pub use assignment::{wardrop_equilibrium, FlowState};
pub use incident::{IncidentSpec, apply_incident, restore_incident};
