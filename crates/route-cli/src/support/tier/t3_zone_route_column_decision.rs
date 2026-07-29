//! Helper `t3_zone_route_column_decision`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_route_column_decision(
    obligation: &T3ZoneAccessObligationRow,
    _route: &str,
    current_score: f64,
    has_intake: bool,
) -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
) {
    if !has_intake {
        return (
            "source-needed",
            "unscored-candidate",
            "intake-score-required",
            "hide-from-map",
            "candidate route is missing from T3/T4 pressure intake",
            "data/t3-t4-pressure-intake.csv",
            "cannot select a T3 zone route without a scored intake row",
            "review",
        );
    }

    match obligation.obligation_class.as_str() {
        "regional-upgrade-review" => (
            "upward-review",
            "possible-t2-upgrade",
            "t2-contact-witness-required",
            "show-as-review-connector",
            "24h obligation is a T2 reopening review, not a T3 feeder selection",
            "data/t2-bubble-up-review.csv",
            "keeps near-threshold routes visible while blocking direct promotion",
            "review",
        ),
        "terminal-local-access" => (
            "t4-access-review",
            "terminal-local-access",
            "terminal-obligation-required",
            "show-as-local-access-candidate",
            "1h obligation belongs to terminal/local access columns",
            "data/t4-terminal-access-columns.csv",
            "passes local access pressure to the T4 selector",
            "review",
        ),
        _ if current_score >= T3_THRESHOLD => (
            "selected",
            "regional-feeder",
            "higher-tier-or-regional-contact-required",
            "render-as-zone-column",
            "score meets T3 threshold and satisfies a 6h feeder obligation",
            "data/t3-zone-map-diagnostics.csv",
            "feeds the T3 zone map and stop-column selector",
            "pass",
        ),
        _ => (
            "review",
            "below-threshold-feeder-candidate",
            "score-or-terminal-evidence-required",
            "show-as-held-zone-candidate",
            "candidate is below T3 threshold for a 6h feeder obligation",
            "data/t3-t4-access-gaps.csv",
            "holds weak feeder pressure for access-gap review instead of selecting it",
            "review",
        ),
    }
}

