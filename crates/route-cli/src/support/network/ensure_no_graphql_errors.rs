//! Helper `ensure_no_graphql_errors`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn ensure_no_graphql_errors(json: &str) -> Result<()> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    if let Some(errors) = value.get("errors").and_then(|value| value.as_array()) {
        let messages = errors
            .iter()
            .filter_map(|error| error.get("message").and_then(|value| value.as_str()))
            .collect::<Vec<_>>()
            .join("; ");
        if messages.is_empty() {
            anyhow::bail!("GraphQL query failed");
        } else {
            anyhow::bail!("{messages}");
        }
    }
    Ok(())
}
