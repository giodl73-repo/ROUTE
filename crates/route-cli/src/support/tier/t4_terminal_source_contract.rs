//! Helper `t4_terminal_source_contract`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_source_contract(zone_id: &str) -> (&'static str, &'static str) {
    match zone_id {
        "t3-great-lakes" => (
            "prove one-hour access to a Great Lakes / Ohio Valley terminal district: Chicago Intermodal Complex, Columbus South, Indianapolis Avon, Detroit Livernois, Minneapolis Twin Cities, St. Louis Gateway, Philadelphia Frankford, or New York Fresh Pond",
            "cite a terminal district/contact source from data/intermodal_terminals.csv and attach the route to a selected T3/T2/T1 column",
        ),
        "t3-southeast" => (
            "prove one-hour access to a Southeast / Appalachia terminal district: Atlanta Hulsey, Charlotte Intermodal, Savannah Garden City, Miami Hialeah, or New Orleans Gentilly",
            "cite a terminal district/contact source from data/intermodal_terminals.csv and attach the route to a selected T3/T2/T1 column",
        ),
        "t3-texas-border" => (
            "prove one-hour access to a Texas Border / Gulf terminal district: Dallas Alliance, Houston Englewood, San Antonio Kirby, or New Orleans Gentilly",
            "cite a terminal district/contact source from data/intermodal_terminals.csv and attach the route to a selected T3/T2/T1 column",
        ),
        "t3-mountain-west" => (
            "prove one-hour access to a Mountain West / Interior terminal district: Denver Logistics Hub, Salt Lake City, Phoenix Sky Harbor area, Portland Albina, Seattle BNSF, Los Angeles/Long Beach, or Kansas City Gateway",
            "cite a terminal district/contact source from data/intermodal_terminals.csv and attach the route to a selected T3/T2/T1 column",
        ),
        "t3-mid-south" => (
            "prove one-hour access to a Mid-South / Delta / Ozarks terminal district: Memphis Intermodal, Kansas City Gateway, St. Louis Gateway, New Orleans Gentilly, or Louisville KentuckyOne",
            "cite a terminal district/contact source from data/intermodal_terminals.csv and attach the route to a selected T3/T2/T1 column",
        ),
        _ => (
            "prove one-hour terminal, port, yard, warehouse, or local freight access",
            "named terminal/local district plus contact to selected T3/T2/T1 column",
        ),
    }
}

