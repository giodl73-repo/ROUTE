//! Helper `missing_source_tokens`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn missing_source_tokens(source_path: &str, required_tokens: &str) -> Vec<String> {
    let Ok(source) = std::fs::read_to_string(resolve_repo_path(source_path)) else {
        return semicolon_values(required_tokens);
    };
    semicolon_values(required_tokens)
        .into_iter()
        .filter(|token| !source.contains(token))
        .collect()
}

