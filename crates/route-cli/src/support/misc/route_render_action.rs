//! Helper `route_render_action`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn route_render_action(row: &T3ZoneRouteColumnRow) -> String {
    match row.column_decision.as_str() {
        "selected" => "render selected T3 route column with stop placement constraints".to_string(),
        "upward-review" => "show as review connector without promotion".to_string(),
        "review" => "show as held feeder candidate only through gap callout".to_string(),
        _ => "hold route outside rendered zone board".to_string(),
    }
}
