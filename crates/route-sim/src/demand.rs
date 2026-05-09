use petgraph::graph::NodeIndex;
/// Traffic demand model.
///
/// Converts FAF5 O-D commodity flows into vehicle demand on the highway network.
/// Each O-D pair (origin FAF zone, destination FAF zone) produces a demand
/// expressed as vehicles per hour in the peak period.
///
/// Truck demand = FAF5 tons/year ÷ average_load_tons ÷ days_per_year ÷ peak_hour_factor
/// Passenger demand: use HPMS AADT × (1 - pct_truck) ÷ daily_to_peak_factor
/// Demand between two graph nodes (vehicles per hour, peak period).
#[derive(Debug, Clone)]
pub struct OdDemand {
    pub origin: NodeIndex,
    pub destination: NodeIndex,
    /// Trucks per hour (peak)
    pub truck_vph: f64,
    /// Passenger cars per hour (peak)
    pub car_vph: f64,
    /// Annual freight value ($B) — for freight cost calculations
    pub freight_value_b: f64,
}

/// Conversion parameters for FAF5 tons → vehicles per hour.
#[derive(Debug, Clone)]
pub struct DemandParams {
    /// Average truck payload in tons
    pub avg_payload_tons: f64,
    /// Days per year of freight movement
    pub days_per_year: f64,
    /// Peak hour as fraction of daily volume (K factor)
    pub peak_hour_factor: f64,
    /// Peak direction factor (D factor)
    pub directional_factor: f64,
}

impl Default for DemandParams {
    fn default() -> Self {
        DemandParams {
            avg_payload_tons: 18.0,   // ~18 tons average truck payload
            days_per_year: 260.0,     // ~260 shipping days/year
            peak_hour_factor: 0.09,   // 9% of daily in peak hour (standard K factor)
            directional_factor: 0.60, // 60% in peak direction (standard D factor)
        }
    }
}

/// Convert annual FAF5 freight tons to peak-hour truck demand.
pub fn tons_to_peak_trucks(annual_tons: f64, params: &DemandParams) -> f64 {
    let daily_tons = annual_tons / params.days_per_year;
    let daily_trucks = daily_tons / params.avg_payload_tons;
    daily_trucks * params.peak_hour_factor * params.directional_factor
}

/// Build O-D demand from HPMS corridor AADT data (no FAF5 required).
/// Uses corridor mean AADT × peak factor to derive O-D demand.
/// Less accurate than FAF5-based but works with available data.
pub fn demand_from_aadt(
    aadt: f64,
    pct_truck: f32,
    params: &DemandParams,
    origin: NodeIndex,
    destination: NodeIndex,
) -> OdDemand {
    let daily_trucks = aadt * pct_truck as f64;
    let daily_cars = aadt * (1.0 - pct_truck as f64);

    OdDemand {
        origin,
        destination,
        truck_vph: daily_trucks * params.peak_hour_factor * params.directional_factor,
        car_vph: daily_cars * params.peak_hour_factor * params.directional_factor,
        freight_value_b: 0.0, // populated from FAF5 when available
    }
}

/// Demand matrix: all O-D pairs and their peak-hour demands.
pub type DemandMatrix = Vec<OdDemand>;
