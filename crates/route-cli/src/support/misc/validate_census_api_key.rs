//! Helper `validate_census_api_key`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn validate_census_api_key(value: Option<String>) -> Result<String> {
    let key = value.context("CENSUS_API_KEY is required for Census ACS requests")?;
    let key = key.trim();
    if key.is_empty() {
        anyhow::bail!("CENSUS_API_KEY is empty");
    }
    Ok(key.to_string())
}

