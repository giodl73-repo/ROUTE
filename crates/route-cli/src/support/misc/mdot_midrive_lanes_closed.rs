//! Helper `mdot_midrive_lanes_closed`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn mdot_midrive_lanes_closed(text: &str) -> Option<u8> {
    let text = text.to_ascii_lowercase();
    if text.contains("center lane") && (text.contains("left lane") || text.contains("right lane")) {
        Some(2)
    } else if text.contains("left lane") && text.contains("right lane") {
        Some(2)
    } else if text.contains("two lanes") || text.contains("2 lanes") {
        Some(2)
    } else if text.contains("three lanes") || text.contains("3 lanes") {
        Some(3)
    } else if text.contains("left lane") || text.contains("right lane") || text.contains("1 lane") {
        Some(1)
    } else if text.contains("left shoulder") || text.contains("right shoulder") {
        Some(0)
    } else {
        None
    }
}

