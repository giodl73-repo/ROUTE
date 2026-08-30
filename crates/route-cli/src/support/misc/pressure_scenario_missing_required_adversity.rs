//! Helper `pressure_scenario_missing_required_adversity`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pressure_scenario_missing_required_adversity(
    rows: &[PressureScenarioRow],
) -> Vec<&'static str> {
    const REQUIRED: &[(&str, &[&str])] = &[
        ("T1/T1 closure", &["t1/t1"]),
        ("corridor segment closure", &["corridor segment", "closure"]),
        ("port surge", &["port surge"]),
        ("weather/flood disruption", &["weather", "flood"]),
        ("relay hub outage", &["relay hub outage"]),
        ("EV/rest-area outage", &["ev/rest-area outage"]),
        ("managed-lane sensitivity", &["managed-lane"]),
    ];

    REQUIRED
        .iter()
        .filter_map(|(label, terms)| {
            let covered = rows.iter().any(|row| {
                let class = row.adversity_class.to_ascii_lowercase();
                if *label == "weather/flood disruption" {
                    terms.iter().any(|term| class.contains(term))
                } else {
                    terms.iter().all(|term| class.contains(term))
                }
            });
            (!covered).then_some(*label)
        })
        .collect()
}
