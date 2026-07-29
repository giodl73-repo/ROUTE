//! Helper `t2_service_diagnostic_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_service_diagnostic_contract(
    row: &T2ServiceSelectionRow,
    bundle: Option<&NationalSegmentBundleRow>,
) -> (&'static str, &'static str, &'static str, &'static str) {
    if row.beck_corridor.trim().is_empty()
        && bundle
            .map(|bundle| {
                semicolon_values(&bundle.state_scope).len() > 1
                    && canonical_route_key(&row.route)
                        .strip_prefix('I')
                        .and_then(|number| number.parse::<u16>().ok())
                        .map(|number| number >= 100)
                        .unwrap_or_default()
            })
            .unwrap_or_default()
    {
        return (
            "route-family-diagnostic-split-needed",
            "split-numbered-route-family-before-beck-diagnostic",
            "data/national-segment-bundles.csv",
            "holds multi-state three-digit route label below national Beck map until represented segment family is split",
        );
    }

    if row.beck_corridor.trim().is_empty() && row.treatment_status == "review-treatment" {
        return (
            "local-relief-map-review",
            "hold-local-relief-below-national-beck-map",
            "data/t3-t4-pressure-intake.csv",
            "keeps review-treatment relief service below map/game overlay until local or zone role is explicit",
        );
    }

    if row.beck_corridor.trim().is_empty() {
        return (
            "beck-diagnostic-missing",
            "author-beck-t2-diagnostic-before-map-overlay",
            "data/beck-t2-diagnostics.csv",
            "holds bundle-ready T2 route below map/game service until Beck service class exists",
        );
    }

    (
        "beck-diagnostic-review",
        "review-existing-beck-t2-diagnostic-before-overlay",
        "data/beck-t2-diagnostics.csv",
        "holds bundle-ready T2 route until Beck diagnostic review is resolved",
    )
}

