//! Helper `t2_closure_bundle_posture`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t2_closure_bundle_posture(
    bundle_by_route: &std::collections::HashMap<String, (String, String, String, String)>,
    route: &str,
) -> (String, String, String, String) {
    bundle_by_route
        .get(&canonical_route_key(route))
        .cloned()
        .unwrap_or_else(|| {
            (
                String::new(),
                "bundle-unchecked".to_string(),
                "join t2-blocker-closure to bundle registry".to_string(),
                String::new(),
            )
        })
}
