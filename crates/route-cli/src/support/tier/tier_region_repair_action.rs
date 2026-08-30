//! Helper `tier_region_repair_action`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_region_repair_action(
    node_class: &route_network::TierNodeClass,
    contact_route_count: usize,
    component_route_count: usize,
) -> (&'static str, &'static str) {
    match node_class {
        route_network::TierNodeClass::TrunkConnector if component_route_count >= 2 => {
            ("keep-for-regionalizer", "touches-multiple-t1-trunks")
        }
        route_network::TierNodeClass::TrunkConnector => (
            "add-dual-contact-witness",
            "qualified-route-is-alone-in-dual-component",
        ),
        route_network::TierNodeClass::ReliefLoop if contact_route_count > 0 => (
            "keep-with-parent-region-review",
            "relief-loop-shares-parent-service-context",
        ),
        route_network::TierNodeClass::ReliefLoop => (
            "add-parent-contact-or-demote",
            "relief-loop-has-no-dual-route-contact",
        ),
        route_network::TierNodeClass::OneEndedFeeder => (
            "terminal-exception-or-demote",
            "one-ended-feeder-needs-terminal-worthy-endpoint",
        ),
        route_network::TierNodeClass::LocalSpur => ("demote-to-t3-t4", "local-spur"),
        route_network::TierNodeClass::MissingGraphData => {
            ("fix-graph-contact-or-demote", "missing-t1-contact-evidence")
        }
    }
}
