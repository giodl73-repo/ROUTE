//! ROUTE-owned portable calculations shared by native simulation and Pages.
use serde::{Deserialize, Serialize};
/// BPR (Bureau of Public Roads) travel time function parameters.
#[derive(Debug, Clone)]
pub struct BprParams {
    /// Typically 0.15
    pub alpha: f64,
    /// Typically 4.0
    pub beta: f64,
}

impl Default for BprParams {
    fn default() -> Self {
        BprParams {
            alpha: 0.15,
            beta: 4.0,
        }
    }
}

/// BPR travel time in hours for an edge given current flow.
pub fn bpr_travel_time(
    free_flow_hours: f64,
    flow_vph: f64,
    capacity_vph: f64,
    params: &BprParams,
) -> f64 {
    if capacity_vph <= 0.0 {
        return free_flow_hours * 10.0;
    } // blocked
    free_flow_hours * (1.0 + params.alpha * (flow_vph / capacity_vph).powf(params.beta))
}

/// A relay hub at a T1/T1 or major T1/T2 intersection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelayHub {
    pub name: String,
    pub corridors: Vec<String>,
    /// "confirmed" or "proposed" (proposed = corridor not yet built)
    pub status: String,
    /// Trucks per day in each direction (each direction needs a fresh driver)
    pub daily_truck_volume: f64,
    /// Bus services per day (intercity express on managed lanes)
    pub daily_bus_services: f64,
    /// Average leg length from this hub (miles)
    pub avg_leg_miles: f64,
    /// Average leg duration (hours) = avg_leg_miles / 65 mph
    pub avg_leg_hours: f64,
}

impl RelayHub {
    pub fn new(
        name: &str,
        corridors: Vec<&str>,
        status: &str,
        daily_truck_volume: f64,
        daily_bus_services: f64,
        avg_leg_miles: f64,
    ) -> Self {
        RelayHub {
            name: name.to_string(),
            corridors: corridors.into_iter().map(String::from).collect(),
            status: status.to_string(),
            daily_truck_volume,
            daily_bus_services,
            avg_leg_miles,
            avg_leg_hours: avg_leg_miles / 65.0,
        }
    }

    /// Relay drivers needed for freight (trucks).
    /// Each truck needs 1 driver per leg.
    /// Each driver works 1 leg per shift (7h driving + commute).
    /// Hub operates 24/7 so 3 shifts.
    /// Add 20% buffer for repositioning, sick leave, scheduling slack.
    pub fn freight_drivers_needed(&self) -> HubStaffing {
        // Swaps per day = trucks arriving × (all need fresh driver at this hub)
        let swaps_per_day = self.daily_truck_volume;

        // Each driver does 1 swap per shift
        // Shifts per day: 24h / 8h shift = 3 shifts
        // So each driver-slot covers 3 swaps/day if fully utilized
        // But drivers work 5 days/week → 7/5 = 1.4 coverage factor
        let drivers_per_slot = 3.0 * (5.0 / 7.0); // 3 shifts × 5/7 days
        let base_drivers = swaps_per_day / drivers_per_slot;

        // 20% buffer for repositioning (drivers need to get back to hub or home)
        // 15% buffer for sick/vacation/training
        let freight_drivers = (base_drivers * 1.35).ceil() as u32;

        // Hub support staff: dispatcher (1 per 20 drivers), maintenance (1 per 15 trucks/day),
        // admin/scheduling (1 per 30 drivers)
        let dispatchers = (freight_drivers as f64 / 20.0).ceil() as u32;
        let maintenance = (self.daily_truck_volume / 15.0).ceil() as u32;
        let admin = (freight_drivers as f64 / 30.0).ceil() as u32;

        // Bus drivers: each bus service needs 1 driver per leg
        // Bus drivers work full 8h shifts (longer legs than freight relay)
        let bus_drivers = (self.daily_bus_services / 2.0 * 1.2).ceil() as u32; // /2 = 2 runs per driver/day

        // Total hub employment
        let total_direct = freight_drivers + bus_drivers + dispatchers + maintenance + admin;

        // Induced employment: food, fuel, lodging, security at hub
        // Hub operates 24/7; drivers need food/shower/rest between legs
        // Industry benchmark: 1.8 indirect jobs per direct job at truck stops
        let indirect = (total_direct as f64 * 0.8).ceil() as u32;

        // Daily throughput
        let daily_swaps = self.daily_truck_volume + self.daily_bus_services;

        HubStaffing {
            hub_name: self.name.clone(),
            daily_truck_swaps: self.daily_truck_volume as u32,
            daily_bus_swaps: self.daily_bus_services as u32,
            freight_relay_drivers: freight_drivers,
            bus_relay_drivers: bus_drivers,
            dispatchers,
            maintenance_staff: maintenance,
            admin_scheduling: admin,
            total_direct_jobs: total_direct,
            total_indirect_jobs: indirect,
            total_hub_employment: total_direct + indirect,
            avg_leg_miles: self.avg_leg_miles,
            avg_leg_hours: self.avg_leg_hours,
            daily_total_swaps: daily_swaps as u32,
        }
    }
}

/// Full staffing breakdown for one hub.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HubStaffing {
    pub hub_name: String,
    pub daily_truck_swaps: u32,
    pub daily_bus_swaps: u32,
    pub freight_relay_drivers: u32,
    pub bus_relay_drivers: u32,
    pub dispatchers: u32,
    pub maintenance_staff: u32,
    pub admin_scheduling: u32,
    pub total_direct_jobs: u32,
    pub total_indirect_jobs: u32,
    pub total_hub_employment: u32,
    pub avg_leg_miles: f64,
    pub avg_leg_hours: f64,
    pub daily_total_swaps: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HubOutageConfig {
    pub outage_hours: f64,
    pub reserve_driver_fraction: f64,
    pub adjacent_absorption_fraction: f64,
}

impl Default for HubOutageConfig {
    fn default() -> Self {
        Self {
            outage_hours: 8.0,
            reserve_driver_fraction: 0.15,
            adjacent_absorption_fraction: 0.35,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HubOutageResult {
    pub hub_name: String,
    pub outage_hours: f64,
    pub affected_swaps: f64,
    pub reserve_absorbed_swaps: f64,
    pub adjacent_absorbed_swaps: f64,
    pub missed_swaps: f64,
    pub throughput_retention: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HubOutageSummary {
    pub results: Vec<HubOutageResult>,
    pub total_affected_swaps: f64,
    pub total_missed_swaps: f64,
    pub network_throughput_retention: f64,
    pub worst_hub_throughput_retention: f64,
}

pub fn run_hub_outage_sensitivity(
    staffings: &[HubStaffing],
    config: HubOutageConfig,
) -> HubOutageSummary {
    let outage_share = (config.outage_hours / 24.0).clamp(0.0, 1.0);
    let reserve_fraction = config.reserve_driver_fraction.clamp(0.0, 1.0);
    let adjacent_fraction = config.adjacent_absorption_fraction.clamp(0.0, 1.0);

    let results = staffings
        .iter()
        .map(|staffing| {
            let affected_swaps = staffing.daily_total_swaps as f64 * outage_share;
            let reserve_absorbed_swaps =
                (staffing.freight_relay_drivers as f64 * reserve_fraction).min(affected_swaps);
            let remaining_after_reserve = (affected_swaps - reserve_absorbed_swaps).max(0.0);
            let adjacent_absorbed_swaps = remaining_after_reserve * adjacent_fraction;
            let missed_swaps = (remaining_after_reserve - adjacent_absorbed_swaps).max(0.0);
            let throughput_retention = if affected_swaps > 0.0 {
                1.0 - (missed_swaps / affected_swaps)
            } else {
                1.0
            };

            HubOutageResult {
                hub_name: staffing.hub_name.clone(),
                outage_hours: config.outage_hours,
                affected_swaps,
                reserve_absorbed_swaps,
                adjacent_absorbed_swaps,
                missed_swaps,
                throughput_retention,
            }
        })
        .collect::<Vec<_>>();

    let total_affected_swaps = results.iter().map(|result| result.affected_swaps).sum();
    let total_missed_swaps = results.iter().map(|result| result.missed_swaps).sum();
    let network_throughput_retention = if total_affected_swaps > 0.0 {
        1.0 - (total_missed_swaps / total_affected_swaps)
    } else {
        1.0
    };
    let worst_hub_throughput_retention = results
        .iter()
        .map(|result| result.throughput_retention)
        .reduce(f64::min)
        .unwrap_or(1.0);

    HubOutageSummary {
        results,
        total_affected_swaps,
        total_missed_swaps,
        network_throughput_retention,
        worst_hub_throughput_retention,
    }
}
