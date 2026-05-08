/// Hub staffing model — how many drivers at each T1/T1 relay hub?
///
/// The hub is the airport gate. Drivers are the flight crew.
/// Trucks arrive from N directions, get fresh driver, depart.
/// Buses (intercity express) do the same. AV vehicles eventually
/// get a "remote operator" handoff rather than a physical driver.
///
/// Staffing drivers = f(truck_volume, leg_hours, shift_hours, utilization)
///
/// Data source: data/relay-hubs.toml (loaded at runtime)
use serde::{Serialize, Deserialize};
use std::path::Path;

/// TOML deserialization structures for relay-hubs.toml
#[derive(Debug, Deserialize)]
struct HubsFile {
    hubs: Vec<HubRecord>,
}

#[derive(Debug, Deserialize)]
struct HubRecord {
    name: String,
    corridors: Vec<String>,
    status: String,
    primary_aadt: u32,
    secondary_aadt: Option<u32>,
    primary_truck_pct: f64,
    secondary_truck_pct: Option<f64>,
    transfer_pct: f64,
    daily_bus_services: f64,
    avg_leg_miles: f64,
    #[allow(dead_code)]
    notes: Option<String>,
}

impl HubRecord {
    fn daily_truck_volume(&self) -> f64 {
        let primary = self.primary_aadt as f64 * (self.primary_truck_pct / 100.0)
            * (self.transfer_pct / 100.0);
        let secondary = self.secondary_aadt.unwrap_or(0) as f64
            * (self.secondary_truck_pct.unwrap_or(0.0) / 100.0)
            * (self.transfer_pct / 100.0);
        primary + secondary
    }
}

/// Load relay hubs from data/relay-hubs.toml, falling back to built-in defaults.
pub fn load_hubs(data_dir: &Path, confirmed_only: bool) -> Vec<RelayHub> {
    let path = data_dir.join("relay-hubs.toml");
    if path.exists() {
        match std::fs::read_to_string(&path)
            .and_then(|s| toml::from_str::<HubsFile>(&s).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e)))
        {
            Ok(file) => {
                return file.hubs.into_iter()
                    .filter(|h| !confirmed_only || h.status == "confirmed")
                    .map(|h| {
                        let daily_truck_volume = h.daily_truck_volume();
                        RelayHub {
                            name: h.name,
                            corridors: h.corridors,
                            status: h.status,
                            daily_truck_volume,
                            daily_bus_services: h.daily_bus_services,
                            avg_leg_miles: h.avg_leg_miles,
                            avg_leg_hours: h.avg_leg_miles / 65.0,
                        }
                    })
                    .collect();
            }
            Err(e) => {
                eprintln!("warning: could not parse {}: {e} — using built-in defaults", path.display());
            }
        }
    }
    // Fall back to built-in defaults
    if confirmed_only { t1_diamond_hubs() } else { let mut h = t1_diamond_hubs(); h.extend(proposed_hubs()); h }
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

/// The nine confirmed T1/T1 diamond hubs with estimated truck volumes.
///
/// Truck volumes estimated from:
/// - HPMS AADT on T1 corridors at each hub location
/// - 8% truck fraction on T1 corridors (FHWA VTRIS)
/// - Both directions count (2× directional flow)
pub fn t1_diamond_hubs() -> Vec<RelayHub> {
    vec![
        RelayHub::new(
            "Gary/Chicago, IL (I-80/I-90)",
            vec!["I-80", "I-90"],
            "confirmed",
            daily_trucks(85_000, 12.0) + daily_trucks(65_000, 12.0),
            48.0,
            440.0,
        ),
        // Atlanta: I-75/I-85 — highest truck volume T1/T1 in Southeast
        // I-75 Atlanta AADT: ~280,000 (with 22% trucks at peak); I-85 ~190,000
        // But hub captures trucks that TRANSFER between corridors, not all through trucks
        // Transfer fraction: ~30% of trucks at any T1/T1 junction transfer
        RelayHub::new(
            "Atlanta, GA (I-75/I-85)",
            vec!["I-75", "I-85"],
            "confirmed",
            daily_trucks(280_000, 22.0) * 0.30 + daily_trucks(190_000, 18.0) * 0.30,
            36.0,
            440.0,
        ),
        // Boston: I-95/I-90 — NE regional hub; lower through-truck volume
        RelayHub::new(
            "Boston, MA (I-95/I-90)",
            vec!["I-95", "I-90"],
            "confirmed",
            daily_trucks(85_000, 8.0),
            24.0,
            400.0,
        ),
        // Seattle: I-5/I-90 — Pacific NW hub; USMCA Canada border
        RelayHub::new(
            "Seattle, WA (I-5/I-90)",
            vec!["I-5", "I-90"],
            "confirmed",
            daily_trucks(75_000, 9.0),
            24.0,
            460.0,
        ),
        // Sacramento: I-5/I-80 — the Donner transition point; East-West to North-South
        RelayHub::new(
            "Sacramento, CA (I-5/I-80)",
            vec!["I-5", "I-80"],
            "confirmed",
            daily_trucks(65_000, 9.0),
            20.0,
            430.0,
        ),
        // San Antonio: I-10/I-35 — USMCA gateway; highest A4 score (Laredo feeder)
        RelayHub::new(
            "San Antonio, TX (I-10/I-35)",
            vec!["I-10", "I-35"],
            "confirmed",
            daily_trucks(80_000, 14.0),  // USMCA truck fraction higher
            20.0,
            480.0,
        ),
        // Jacksonville: I-10/I-95 — Florida gateway; Southeast distribution
        RelayHub::new(
            "Jacksonville, FL (I-10/I-95)",
            vec!["I-10", "I-95"],
            "confirmed",
            daily_trucks(60_000, 9.0),
            16.0,
            420.0,
        ),
        // Toledo: I-75/I-90 — Great Lakes manufacturing hub
        RelayHub::new(
            "Toledo, OH (I-75/I-90)",
            vec!["I-75", "I-90"],
            "confirmed",
            daily_trucks(55_000, 12.0),  // high truck fraction: auto parts
            12.0,
            420.0,
        ),
        // Richmond: I-95/I-85 — mid-Atlantic connector
        RelayHub::new(
            "Richmond, VA (I-95/I-85)",
            vec!["I-95", "I-85"],
            "confirmed",
            daily_trucks(65_000, 10.0),
            16.0,
            440.0,
        ),
    ]
}

/// Proposed hubs when missing links are built.
pub fn proposed_hubs() -> Vec<RelayHub> {
    vec![
        // Wichita: proposed I-31/I-29S intersection — Great Plains agricultural hub
        RelayHub::new(
            "Wichita, KS (proposed I-31/I-29S)",
            vec!["I-31 (proposed)", "I-29S (proposed)"],
            "proposed",
            daily_trucks(28_000, 10.0),  // lower initial volume; new corridor
            8.0,
            480.0,
        ),
        // Houston: proposed I-69/I-10 intersection — Gulf Coast gateway
        RelayHub::new(
            "Houston, TX (I-10/I-69 on completion)",
            vec!["I-10", "I-69 (proposed)"],
            "proposed",
            daily_trucks(70_000, 12.0),
            20.0,
            460.0,
        ),
        // Billings MT: proposed I-90/I-92 — Mountain West / Northern Tier
        RelayHub::new(
            "Billings, MT (I-90/I-92 proposed)",
            vec!["I-90", "I-92 (proposed)"],
            "proposed",
            daily_trucks(20_000, 8.0),
            8.0,
            520.0,
        ),
    ]
}

fn daily_trucks(aadt: u32, truck_pct: f64) -> f64 {
    aadt as f64 * (truck_pct / 100.0)
}

/// National network summary across all hubs.
pub struct NetworkSummary {
    pub total_hubs: usize,
    pub total_daily_swaps: u32,
    pub total_freight_drivers: u32,
    pub total_bus_drivers: u32,
    pub total_direct_jobs: u32,
    pub total_hub_employment: u32,
    pub hub_staffings: Vec<HubStaffing>,
}

pub fn compute_network_summary(hubs: &[RelayHub]) -> NetworkSummary {
    let staffings: Vec<HubStaffing> = hubs.iter().map(|h| h.freight_drivers_needed()).collect();
    NetworkSummary {
        total_hubs: staffings.len(),
        total_daily_swaps: staffings.iter().map(|s| s.daily_total_swaps).sum(),
        total_freight_drivers: staffings.iter().map(|s| s.freight_relay_drivers).sum(),
        total_bus_drivers: staffings.iter().map(|s| s.bus_relay_drivers).sum(),
        total_direct_jobs: staffings.iter().map(|s| s.total_direct_jobs).sum(),
        total_hub_employment: staffings.iter().map(|s| s.total_hub_employment).sum(),
        hub_staffings: staffings,
    }
}
