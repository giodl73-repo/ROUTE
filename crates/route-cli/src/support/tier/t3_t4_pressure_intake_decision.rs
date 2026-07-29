//! Helper `t3_t4_pressure_intake_decision`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_t4_pressure_intake_decision(
    row: &LowerTierPressureWitnessRow,
) -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
) {
    match row.pressure_type.as_str() {
        "regional-upgrade-pressure" => (
            "bubble-up-t2-review",
            "send-to-t2-contact-review",
            "T2",
            "data/tier-contact-witnesses.csv",
            "lower-tier score pressure can reopen T2 only through contact gates",
        ),
        "local-upgrade-pressure" => (
            "t3-regional-intake",
            "evaluate-for-t3-zone-treatment",
            "T3",
            "data/t3-t4-pressure-intake.csv",
            "hold for T3 zone treatment; no national map promotion",
        ),
        "closure-demotion-pressure" | "demotion-pressure" => {
            if row.current_score >= T3_THRESHOLD {
                (
                    "t3-regional-intake",
                    "accept-as-t3-regional-review",
                    "T3",
                    "data/t3-t4-pressure-intake.csv",
                    "consume T2 demotion as regional feeder review",
                )
            } else {
                (
                    "t4-local-intake",
                    "accept-as-t4-local-access-review",
                    "T4",
                    "data/t3-t4-pressure-intake.csv",
                    "consume demotion as local access review",
                )
            }
        }
        _ => (
            "evidence-needed",
            "review-pressure-source",
            "T3/T4",
            "data/lower-tier-pressure-witnesses.csv",
            "pressure row needs explicit intake rule",
        ),
    }
}

