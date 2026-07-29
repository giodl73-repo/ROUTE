//! Helper `extract_after_label`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn extract_after_label(text: &str, label: &str) -> Option<String> {
    let (_, tail) = text.split_once(label)?;
    let value = tail.split('|').next().unwrap_or(tail).trim().to_string();
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

