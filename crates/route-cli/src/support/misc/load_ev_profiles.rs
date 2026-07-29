//! Helper `load_ev_profiles`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_ev_profiles(data_dir: &std::path::Path) -> Vec<route_sim::EvProfile> {
    let path = data_dir.join("ev-profiles.toml");
    if let Ok(text) = std::fs::read_to_string(&path) {
        if let Ok(file) = toml::from_str::<EvProfilesFile>(&text) {
            if !file.vehicles.is_empty() {
                return file
                    .vehicles
                    .into_iter()
                    .map(|r| {
                        // Box::leak turns an owned String into a &'static str for the lifetime of the
                        // process. Acceptable in a CLI binary that doesn't free profiles at runtime.
                        let name: &'static str = Box::leak(r.name.into_boxed_str());
                        route_sim::EvProfile {
                            name,
                            highway_range_miles: r.highway_range_miles,
                            charge_rate_kw: r.charge_rate_kw,
                            battery_kwh: r.battery_kwh,
                            kwh_per_mile: r.kwh_per_mile,
                        }
                    })
                    .collect();
            }
        }
    }
    // Fall back to built-in profiles
    vec![
        route_sim::average_ev_2026(),
        route_sim::tesla_model_y(),
        route_sim::tesla_semi(),
    ]
}

