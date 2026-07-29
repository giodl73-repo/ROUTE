//! Helper `load_endpoint_exceptions`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_endpoint_exceptions(path: &Path) -> Result<Vec<EndpointExceptionRow>> {
    let file = std::fs::File::open(path)?;
    parse_endpoint_exceptions(file)
}

