//! Helper `great_lakes_terminal_seed_for_route`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn great_lakes_terminal_seed_for_route(route: &str) -> Option<String> {
    let district = match canonical_route_key(route).as_str() {
        "I115" | "I176" | "I294" | "US41" => "Chicago Intermodal Complex",
        "I129" | "I465" | "US31" => "Indianapolis Avon",
        "I180" | "I72" | "US42" => "St. Louis Gateway",
        "I190" | "I390" | "I478" | "I691" | "I990" | "US7" => "New York Fresh Pond",
        "I196" | "I496" | "I696" | "US10" | "US223" => "Detroit Livernois",
        "I235" => "Minneapolis Twin Cities",
        "I271" | "I471" | "US22" | "US35" | "US224" | "US250" | "US74" => "Columbus South",
        "I276" | "I93" | "US15" => "Philadelphia Frankford",
        "I279" => "Columbus South",
        _ => return None,
    };
    Some(district.to_string())
}

