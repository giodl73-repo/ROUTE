//! Helper `census_api_key`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn census_api_key() -> Result<String> {
    validate_census_api_key(std::env::var("CENSUS_API_KEY").ok())
}
