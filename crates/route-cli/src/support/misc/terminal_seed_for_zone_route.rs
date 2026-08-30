//! Helper `terminal_seed_for_zone_route`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn terminal_seed_for_zone_route(zone_id: &str, route: &str) -> Option<String> {
    let key = canonical_route_key(route);
    let district = match zone_id {
        "t3-great-lakes" => return great_lakes_terminal_seed_for_route(route),
        "t3-southeast" => match key.as_str() {
            "I140" | "US301" => "Savannah Garden City",
            "I175" => "Miami Hialeah",
            "I185" | "US278" | "US84" => "Atlanta Hulsey",
            "I795" | "US119" => "Charlotte Intermodal",
            "US45E" | "US45W" | "US82" | "US90Z" => "New Orleans Gentilly",
            _ => return None,
        },
        "t3-mid-south" => match key.as_str() {
            "I169" | "US24" | "US66" => "Kansas City Gateway",
            "I181" | "I277" | "US421" => "Louisville KentuckyOne",
            "I255" => "St. Louis Gateway",
            "I759" | "I840" | "US167" | "US270" => "Memphis Intermodal",
            _ => return None,
        },
        "t3-mountain-west" => match key.as_str() {
            "I135" | "I335" | "US76" => "Kansas City Gateway",
            "I705" => "Seattle BNSF",
            "I880" => "Los Angeles/Long Beach",
            "US14" | "US95" => "Salt Lake City",
            "US26" => "Portland Albina",
            "US87" => "Denver Logistics Hub",
            _ => return None,
        },
        "t3-texas-border" => match key.as_str() {
            "I510" => "New Orleans Gentilly",
            "I69E" | "US281" => "San Antonio Kirby",
            "US96" => "Houston Englewood",
            _ => return None,
        },
        _ => return None,
    };
    Some(district.to_string())
}
