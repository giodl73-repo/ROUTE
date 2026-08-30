//! Helper `normalise_designation`.
#[allow(unused_imports)]
use crate::*;

/// Normalise user input to internal route ID: "I-80" → "I80", "i80" → "I80"
pub(crate) fn normalise_designation(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_uppercase()
}
