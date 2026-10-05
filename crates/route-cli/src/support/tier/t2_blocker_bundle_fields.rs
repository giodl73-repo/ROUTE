//! Helper `t2_blocker_bundle_fields`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_blocker_bundle_fields(
    registry: &route_network::BundleRegistry,
    route: &str,
) -> (String, String, String) {
    registry
        .by_route_label(route)
        .first()
        .map(|bundle| {
            let (bundle_action, _) =
                route_network::bundle_action(bundle.bundle_status, &bundle.registry_actions);
            (
                bundle.segment_bundle_id.clone(),
                bundle.bundle_status.as_str().to_string(),
                bundle_action.to_string(),
            )
        })
        .unwrap_or_else(|| {
            (
                String::new(),
                "bundle-missing".to_string(),
                "resolve route family or add segment bundle".to_string(),
            )
        })
}
