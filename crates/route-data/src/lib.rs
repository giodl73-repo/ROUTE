pub mod census;
pub mod faf5;
pub mod fema;
pub mod fetch;
pub mod fletch;
pub mod hpms;
pub mod hpms_fetch;
pub mod manifest;
pub mod nbi;
pub mod nhs;

/// Raw data stays upstream of bundle identity.
///
/// `route-data` readers expose source records. Downstream crates translate
/// those records into `route_network::SegmentBundleMember` rows once physical
/// segment identity, state scope, and service/corridor membership are known.
/// This keeps ingestion free of optimizer identity policy while still making
/// bundles the system abstraction everywhere after raw-source parsing.
pub use census::{
    fetch_acs_income, fetch_acs_population, join_income, join_population, join_rucc,
    read_county_gazetteer, CountyCentroid, NATIONAL_MEDIAN_HHI_2022,
};
pub use faf5::Faf5Record;
pub use fema::{fetch_all_sfha_counts, fetch_fema_count, CorridorBbox, FemaSfhaResult, T1_BBOXES};
pub use fletch::{
    fetch_all_manifest_sources_with_fletch, fletch_cache_index_gate_failures,
    fletch_cache_index_report, fletch_cache_manifest_path, fletch_source_handoff_report,
    load_fletch_source_registry, load_route_source_fetch_policy, read_fletch_cache_manifest,
    write_fletch_cache_index, write_fletch_source_handoff, FletchCacheIndexReport,
    FletchCacheIndexRow, FletchSourceHandoffReport, FletchSourceHandoffRow,
    RouteSourceFetchPolicyRow,
};
pub use hpms::{HpmsFpmRecord, HpmsRecord};
pub use hpms_fetch::{fetch_all_hpms, STATE_CODES};
pub use manifest::Manifest;
pub use nbi::NbiRecord;
pub use nhs::{NhsSegment, RoadClass};
