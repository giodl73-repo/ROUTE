//! Helper `tier_candidate_aliases`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn tier_candidate_aliases(tier: &str, region_id: &str, route: &str, bundle_scope: &str) -> String {
    let mut aliases = vec![
        format!("current-tier:{tier}"),
        format!("current-zone:{region_id}"),
        format!("route:{route}"),
        format!("route-label:{route}"),
        "layer:segment-candidate".to_string(),
    ];
    if !bundle_scope.trim().is_empty() {
        aliases.push(format!("route-family-scope:{bundle_scope}"));
    }
    aliases.join(";")
}

