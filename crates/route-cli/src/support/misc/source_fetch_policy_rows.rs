//! Helper `source_fetch_policy_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn source_fetch_policy_rows() -> Vec<SourceFetchPolicyRow> {
    vec![
        source_fetch_policy_row(
            "manifest-downloads",
            "route fetch [--force]",
            "data/cache/<manifest filename>",
            "full-replace-after-validation",
            "skip existing files unless --force; write new payload only after HTTP success",
            "route_data::fetch_all_manifest_sources_with_fletch plus atomic legacy-path write",
            "HTTP success and complete byte write before replace",
        ),
        source_fetch_policy_row(
            "hpms-national",
            "route fetch-hpms",
            "data/cache/hpms_2018.csv",
            "full-replace-after-validation",
            "preserve prior national cache if all-state fetch returns zero rows",
            "temp CSV plus replace_with_temp",
            "non-empty HPMS record set",
        ),
        source_fetch_policy_row(
            "hpms-state-scope",
            "route fetch-hpms --states <STATE[,STATE]>;route fetch-hpms --states <STATE[,STATE]> --functional-systems <SYSTEM[,SYSTEM]>",
            "data/cache/hpms_2018.csv;data/cache/hpms_<state>.csv",
            "scoped-merge",
            "replace fetched state rows for declared functional-system scope and preserve all non-requested state rows",
            "merge_hpms_state_records plus temp CSV replace",
            "at least one fetched HPMS row",
        ),
        source_fetch_policy_row(
            "acs-county",
            "route fetch-acs;route fetch-acs-income",
            "data/cache/acs_county_pop_2022.csv;data/cache/acs_county_income_2022.csv",
            "full-replace-after-validation",
            "preserve prior ACS cache on HTTP, parse, or write failure",
            "temp CSV plus replace_with_temp",
            "Census JSON parses before CSV replace",
        ),
        source_fetch_policy_row(
            "fema-corridor",
            "route fetch-fema;route fetch-fema-d1",
            "data/cache/fema_sfha_counts.csv;data/cache/fema_sfha_tile_counts.csv",
            "full-replace-after-validation",
            "preserve prior FEMA cache until query loop and CSV write finish",
            "temp CSV plus replace_with_temp/replace_with_atomic_write",
            "CSV rows flushed after query loop",
        ),
        source_fetch_policy_row(
            "t1-live-event-snapshots",
            "route t1-fetch-iowa511;route t1-fetch-tdot-smartway;route t1-fetch-mdot-midrive;route t1-fetch-indot-trafficwise",
            "data/cache/*events.json;data/cache/*incidents.json",
            "live-snapshot-preserve",
            "latest snapshot path may advance, but failed fetches preserve the last usable snapshot",
            "atomic_write_text after ArcGIS/GraphQL validation where available",
            "HTTP success and source error envelope check where available",
        ),
    ]
}
