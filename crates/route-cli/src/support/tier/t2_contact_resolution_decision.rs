//! Helper `t2_contact_resolution_decision`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_contact_resolution_decision(
    row: &TierContactWitnessInputRow,
    exceptions: &[&EndpointExceptionRow],
) -> (&'static str, &'static str, &'static str, &'static str) {
    if row.witness_type == "graph-contact-needed" {
        if exceptions.iter().any(|exception| {
            exception
                .exception_type
                .trim()
                .eq_ignore_ascii_case("demote")
                || exception
                    .evidence_level
                    .trim()
                    .eq_ignore_ascii_case("demote")
        }) {
            return (
                "move-to-lower-tier-pressure",
                "endpoint-exception-demotion",
                "data/lower-tier-pressure-witnesses.csv",
                "pass",
            );
        }
        if exceptions
            .iter()
            .any(|exception| endpoint_exception_is_terminal_worthy(exception))
        {
            return (
                "hold-for-terminal-contact-validation",
                "terminal-worthy-exception-needs-graph-contact",
                "data/tier-contact-witnesses.csv",
                "review",
            );
        }
        if exceptions.iter().any(|exception| {
            exception
                .exception_type
                .trim()
                .eq_ignore_ascii_case("metro_beltway_relief")
        }) {
            return (
                "hold-for-relief-evidence-or-demotion",
                "metro-beltway-relief-needs-source-backed-contact",
                "data/atri-bottlenecks.csv",
                "review",
            );
        }
    }

    match row.witness_type.as_str() {
        "regionalizer-ready" => (
            "accept-contact-witness",
            "selected-regionalizer-contact",
            "data/tier-candidate-columns.csv",
            "pass",
        ),
        "parent-region-review" => (
            "move-to-candidate-review",
            "relief-loop-shares-parent-service-context",
            "data/tier-candidate-columns.csv",
            "pass",
        ),
        "tier-demotion-needed" => (
            "move-to-lower-tier-pressure",
            "local-spur-policy-demotion",
            "data/lower-tier-pressure-witnesses.csv",
            "pass",
        ),
        "terminal-exception-needed" => {
            if exceptions.iter().any(|exception| {
                exception
                    .exception_type
                    .trim()
                    .eq_ignore_ascii_case("metro_beltway_relief")
            }) {
                (
                    "hold-for-relief-evidence-or-demotion",
                    "metro-beltway-relief-needs-source-backed-contact",
                    "data/atri-bottlenecks.csv",
                    "review",
                )
            } else {
                (
                    "hold-for-terminal-exception",
                    "one-ended-feeder-needs-terminal-worthy-endpoint",
                    "data/tier-node-exceptions.csv",
                    "review",
                )
            }
        }
        "graph-contact-needed" => (
            "hold-for-graph-contact-repair",
            "missing-t1-contact-evidence",
            "data/tier-contact-witnesses.csv",
            "review",
        ),
        "parent-contact-needed" => (
            "hold-for-parent-contact-or-demotion",
            "relief-loop-has-no-dual-route-contact",
            "data/tier-contact-witnesses.csv",
            "review",
        ),
        _ => (
            "hold-for-unknown-repair",
            "unknown-witness-type",
            "data/tier-contact-witnesses.csv",
            "review",
        ),
    }
}

