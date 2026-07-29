//! Helper `t2_route_family_split_decision`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_route_family_split_decision(
    exception: Option<&EndpointExceptionRow>,
) -> (
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
) {
    let Some(exception) = exception else {
        return (
            "add-endpoint-exception-record",
            "blocked",
            "route-family endpoint exception record",
            "data/tier-node-exceptions.csv",
            "blocked from T2 until route family source row exists",
        );
    };

    let endpoint_role = exception.endpoint_role.trim().to_ascii_lowercase();
    let exception_type = exception.exception_type.trim().to_ascii_lowercase();
    if endpoint_role == "graph_endpoint_gap" || exception_type == "missing_graph_geometry" {
        if endpoint_role == "local_access_end" {
            return (
                "split-local-family-or-demote",
                "lower-tier-pressure",
                "metro-specific segment split plus T1/T2 contact proof",
                "data/lower-tier-pressure-witnesses.csv",
                "kept out of T2 until a split segment proves regional service",
            );
        }
        return (
            "split-numbered-family",
            "blocked",
            "represented segment id plus T1/T2 contact proof",
            "data/tier-node-exceptions.csv",
            "blocked from T2 until route family is disambiguated",
        );
    }

    (
        "review-route-family",
        "blocked",
        "route-family split basis",
        "data/tier-node-exceptions.csv",
        "blocked from T2 until route family disposition is explicit",
    )
}

