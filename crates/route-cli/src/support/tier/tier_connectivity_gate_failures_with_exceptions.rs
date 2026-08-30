//! Helper `tier_connectivity_gate_failures_with_exceptions`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_connectivity_gate_failures_with_exceptions<'a>(
    rows: &'a [route_network::TierConnectivityRow],
    exceptions: &[EndpointExceptionRow],
    tier: &str,
) -> Vec<TierConnectivityGateFailure<'a>> {
    rows.iter()
        .filter_map(|row| {
            if matches!(
                row.classification,
                route_network::TierNodeClass::TrunkConnector
                    | route_network::TierNodeClass::ReliefLoop
            ) {
                return None;
            }

            let route_exceptions = endpoint_exceptions_for_route(exceptions, &row.route, tier);
            if route_exception_allows_connectivity_gate(row, &route_exceptions) {
                return None;
            }

            Some(TierConnectivityGateFailure {
                row,
                reason: endpoint_exception_failure_reason(row, &route_exceptions),
            })
        })
        .collect()
}
