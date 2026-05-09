/// Named scenario files.
///
/// Each scenario is a TOML file that defines a specific "what if" question.
/// Scenarios are loaded from this directory at runtime by name.
///
/// Available scenarios:
///   donner-closure         — I-80 Donner Pass closes 48h (winter storm)
///   atlanta-peak           — I-75/I-285 Atlanta AM peak, managed lanes intervention
///   des-moines-interchange — I-35 × I-80 Des Moines closes, diamond vs no-diamond
///   houston-surge          — I-10/I-45/I-610 Houston hurricane evacuation surge
///   northern-tier-baseline — What does the Northern Tier add to national throughput?
///
/// To run: route sim --scenario donner-closure
/// To test intervention: route sim scenario des-moines-interchange --intervention

pub const DONNER_CLOSURE: &str = include_str!("donner-closure.toml");
pub const ATLANTA_PEAK: &str = include_str!("atlanta-peak.toml");
pub const DES_MOINES_INTERCHANGE: &str = include_str!("des-moines-interchange.toml");
pub const HOUSTON_SURGE: &str = include_str!("houston-surge.toml");

/// Load a scenario by name from the embedded TOML files.
pub fn load_scenario(name: &str) -> Option<&'static str> {
    match name {
        "donner-closure" => Some(DONNER_CLOSURE),
        "atlanta-peak" => Some(ATLANTA_PEAK),
        "des-moines-interchange" => Some(DES_MOINES_INTERCHANGE),
        // Historical alias kept so old notes fail softly into the corrected fixture.
        "omaha-interchange" => Some(DES_MOINES_INTERCHANGE),
        "houston-surge" => Some(HOUSTON_SURGE),
        _ => None,
    }
}

/// List all available scenario names.
pub fn available_scenarios() -> &'static [&'static str] {
    &[
        "donner-closure",
        "atlanta-peak",
        "des-moines-interchange",
        "houston-surge",
    ]
}
