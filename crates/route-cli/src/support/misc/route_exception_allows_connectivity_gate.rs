//! Helper `route_exception_allows_connectivity_gate`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn route_exception_allows_connectivity_gate(
    row: &route_network::TierConnectivityRow,
    exceptions: &[&EndpointExceptionRow],
) -> bool {
    match row.classification {
        route_network::TierNodeClass::OneEndedFeeder => exceptions
            .iter()
            .any(|exception| endpoint_exception_is_terminal_worthy(exception)),
        route_network::TierNodeClass::LocalSpur => exceptions.iter().any(|exception| {
            endpoint_exception_is_terminal_worthy(exception)
                && exception
                    .evidence_level
                    .trim()
                    .eq_ignore_ascii_case("validated")
        }),
        route_network::TierNodeClass::MissingGraphData => false,
        route_network::TierNodeClass::TrunkConnector | route_network::TierNodeClass::ReliefLoop => {
            true
        }
    }
}
