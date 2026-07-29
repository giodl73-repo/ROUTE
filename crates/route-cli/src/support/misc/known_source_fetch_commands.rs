//! Helper `known_source_fetch_commands`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn known_source_fetch_commands() -> &'static [&'static str] {
    &[
        "route fetch",
        "route fetch-hpms",
        "route fetch-hpms --states",
        "route fetch-acs",
        "route fetch-acs-income",
        "route fetch-fema-d1",
        "route fetch-fema",
        "route t1-fetch-iowa511",
        "route t1-fetch-tdot-smartway",
        "route t1-fetch-mdot-midrive",
        "route t1-fetch-indot-trafficwise",
    ]
}

