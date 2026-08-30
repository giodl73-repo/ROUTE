//! Helper `t3_segment_aliases`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_segment_aliases(zone_id: &str, route: &str, layer: &str) -> String {
    let mut aliases = vec![
        "current-tier:T3".to_string(),
        format!("current-zone:{}", zone_id.trim()),
        format!("layer:{}", layer.trim()),
    ];
    let route = normalise_designation(route);
    if !route.is_empty() {
        aliases.push(format!("route:{route}"));
        aliases.push(format!("route-label:{route}"));
        aliases.push(format!("zone-route:{}:{route}", zone_id.trim()));
    }
    aliases.join(";")
}
