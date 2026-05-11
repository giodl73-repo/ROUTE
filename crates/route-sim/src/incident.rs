/// Incident specification and application.
///
/// An incident reduces the capacity of one or more edges for a specified duration.
/// After an incident, demand reassigns via Wardrop equilibrium on the degraded network.
///
/// Incident types:
///   - Closure: capacity drops to 0 (full blockage)
///   - Partial: capacity drops to specified fraction (lane closure, construction)
///   - Weather: capacity drops based on weather type (snow → -30%, rain → -10%)
///   - Bridge failure: structural closure of specific bridge segments
use petgraph::graph::EdgeIndex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Specification for a simulated incident.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncidentSpec {
    /// Human-readable name for reporting
    pub name: String,
    /// Edges affected by the incident
    pub affected_edges: Vec<u64>, // edge IDs (not indices — stable across graph rebuilds)
    pub incident_type: IncidentType,
    /// Duration in hours
    pub duration_hours: f64,
    /// Annual frequency of this incident type (for economic cost calculation)
    pub annual_occurrences: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum IncidentType {
    /// Full road closure (capacity = 0)
    Closure,
    /// Lane closure (capacity reduced to fraction of normal)
    LaneClosure { remaining_fraction: f64 },
    /// Weather event (Donner Pass winter closure, Gulf Coast hurricane, etc.)
    Weather { weather_type: WeatherType },
    /// Construction (multi-month reduced capacity)
    Construction {
        capacity_fraction: f64,
        duration_days: f64,
    },
    /// Bridge failure (closure of specific structure)
    BridgeFailure { bridge_id: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeatherType {
    SnowIce,     // -30% capacity
    HeavyRain,   // -10% capacity
    Fog,         // -20% capacity
    ExtremeCold, // -5% capacity (equipment issues)
    Hurricane,   // full closure of affected segments
}

impl WeatherType {
    pub fn capacity_fraction(&self) -> f64 {
        match self {
            WeatherType::SnowIce => 0.70,
            WeatherType::HeavyRain => 0.90,
            WeatherType::Fog => 0.80,
            WeatherType::ExtremeCold => 0.95,
            WeatherType::Hurricane => 0.0,
        }
    }
}

impl IncidentType {
    pub fn capacity_fraction(&self) -> f64 {
        match self {
            IncidentType::Closure => 0.0,
            IncidentType::LaneClosure { remaining_fraction } => *remaining_fraction,
            IncidentType::Weather { weather_type } => weather_type.capacity_fraction(),
            IncidentType::Construction {
                capacity_fraction, ..
            } => *capacity_fraction,
            IncidentType::BridgeFailure { .. } => 0.0,
        }
    }
}

/// Apply an incident to a capacity map.
/// Returns the modified capacity map and a snapshot of original capacities.
pub fn apply_incident(
    base_capacities: &HashMap<EdgeIndex, f64>,
    incident: &IncidentSpec,
    edge_id_to_index: &HashMap<u64, EdgeIndex>,
) -> (HashMap<EdgeIndex, f64>, HashMap<EdgeIndex, f64>) {
    let mut modified = base_capacities.clone();
    let mut snapshot = HashMap::new();

    let fraction = incident.incident_type.capacity_fraction();
    for &edge_id in &incident.affected_edges {
        if let Some(&ei) = edge_id_to_index.get(&edge_id) {
            if let Some(&orig) = base_capacities.get(&ei) {
                snapshot.insert(ei, orig);
                modified.insert(ei, orig * fraction);
            }
        }
    }

    (modified, snapshot)
}

/// Restore capacities after an incident.
pub fn restore_incident(
    modified: &mut HashMap<EdgeIndex, f64>,
    snapshot: &HashMap<EdgeIndex, f64>,
) {
    for (ei, &orig) in snapshot {
        modified.insert(*ei, orig);
    }
}

#[cfg(test)]
mod tests {
    use super::{apply_incident, restore_incident, IncidentSpec, IncidentType, WeatherType};
    use petgraph::graph::EdgeIndex;
    use std::collections::HashMap;

    #[test]
    fn closure_sets_capacity_to_zero_and_restores_original_capacity() {
        let e1 = EdgeIndex::new(0);
        let e2 = EdgeIndex::new(1);
        let capacities = HashMap::from([(e1, 3_800.0), (e2, 1_900.0)]);
        let edge_ids = HashMap::from([(101, e1), (202, e2)]);
        let incident = IncidentSpec {
            name: "full closure".to_string(),
            affected_edges: vec![101],
            incident_type: IncidentType::Closure,
            duration_hours: 4.0,
            annual_occurrences: 1.0,
        };

        let (mut modified, snapshot) = apply_incident(&capacities, &incident, &edge_ids);

        assert_eq!(modified[&e1], 0.0);
        assert_eq!(modified[&e2], 1_900.0);
        assert_eq!(snapshot[&e1], 3_800.0);

        restore_incident(&mut modified, &snapshot);

        assert_eq!(modified, capacities);
    }

    #[test]
    fn lane_and_weather_incidents_apply_expected_capacity_fractions() {
        let edge = EdgeIndex::new(0);
        let capacities = HashMap::from([(edge, 4_000.0)]);
        let edge_ids = HashMap::from([(77, edge)]);
        let lane_incident = IncidentSpec {
            name: "lane closure".to_string(),
            affected_edges: vec![77],
            incident_type: IncidentType::LaneClosure {
                remaining_fraction: 0.25,
            },
            duration_hours: 2.0,
            annual_occurrences: 3.0,
        };
        let snow_incident = IncidentSpec {
            name: "snow".to_string(),
            affected_edges: vec![77],
            incident_type: IncidentType::Weather {
                weather_type: WeatherType::SnowIce,
            },
            duration_hours: 8.0,
            annual_occurrences: 2.0,
        };

        let (lane_caps, _) = apply_incident(&capacities, &lane_incident, &edge_ids);
        let (snow_caps, _) = apply_incident(&capacities, &snow_incident, &edge_ids);

        assert_eq!(lane_caps[&edge], 1_000.0);
        assert_eq!(snow_caps[&edge], 2_800.0);
    }

    #[test]
    fn unknown_edge_ids_are_ignored_without_touching_capacities() {
        let edge = EdgeIndex::new(0);
        let capacities = HashMap::from([(edge, 3_800.0)]);
        let edge_ids = HashMap::from([(11, edge)]);
        let incident = IncidentSpec {
            name: "bad id".to_string(),
            affected_edges: vec![99],
            incident_type: IncidentType::Closure,
            duration_hours: 1.0,
            annual_occurrences: 1.0,
        };

        let (modified, snapshot) = apply_incident(&capacities, &incident, &edge_ids);

        assert_eq!(modified, capacities);
        assert!(snapshot.is_empty());
    }
}
