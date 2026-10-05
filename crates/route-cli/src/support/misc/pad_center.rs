//! Helper `pad_center`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn pad_center(s: &str, width: usize) -> String {
    if s.len() >= width {
        return s[..width].to_string();
    }
    let pad = width - s.len();
    let left = pad / 2;
    let right = pad - left;
    format!("{}{}{}", " ".repeat(left), s, " ".repeat(right))
}
