//! Helper `fmt_opt`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn fmt_opt(value: Option<f64>) -> String {
    value
        .map(|v| format!("{v:.3}"))
        .unwrap_or_else(|| "-".to_string())
}

