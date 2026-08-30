//! Helper `t3_zone_catalog_entry`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t3_zone_catalog_entry(zone_id: &str) -> Option<(&'static str, &'static str)> {
    match zone_id {
        "t3-great-lakes" => Some(("t3-great-lakes", "Great Lakes / Ohio Valley")),
        "t3-southeast" => Some(("t3-southeast", "Southeast / Appalachia")),
        "t3-texas-border" => Some(("t3-texas-border", "Texas Border / Gulf Access")),
        "t3-mountain-west" => Some(("t3-mountain-west", "Mountain West / Interior Coverage")),
        "t3-mid-south" => Some(("t3-mid-south", "Mid-South / Delta / Ozarks")),
        _ => None,
    }
}
