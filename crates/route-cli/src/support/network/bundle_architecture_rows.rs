//! Helper `bundle_architecture_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn bundle_architecture_rows() -> Vec<BundleArchitectureRow> {
    [
        (
            "route-data",
            "raw-source-ingestion",
            "source rows remain upstream; downstream converts to SegmentBundleMember",
            "crates/route-data/src/lib.rs",
            "SegmentBundleMember;bundles the system abstraction",
            "bundle-upstream",
            "keep raw ingestion free of optimizer identity policy",
        ),
        (
            "route-network",
            "bundle-identity-owner",
            "route_network::build_segment_bundles",
            "crates/route-network/src/bundle.rs",
            "pub struct SegmentBundle;pub struct SegmentBundleMember;pub struct BundleRegistry;pub enum BundleStatus;pub fn build_segment_bundles;pub fn bundle_action",
            "bundle-native",
            "use as canonical bundle rollup and status API",
        ),
        (
            "route-score",
            "bundle-scoring",
            "route_score::score_bundle",
            "crates/route-score/src/score.rs",
            "pub struct BundleScores;pub fn score_bundle",
            "bundle-native",
            "score service/corridor bundles before falling back to corridor compatibility",
        ),
        (
            "route-map",
            "bundle-rendering",
            "route_map::build_bundle_svg",
            "crates/route-map/src/renderer.rs",
            "pub struct BundleRenderIdentity;pub fn build_bundle_svg;segment_bundle_id",
            "bundle-native",
            "render maps with bundle identity carried through output metadata",
        ),
        (
            "route-sim",
            "bundle-simulation",
            "route_sim::BundleIncidentSpec",
            "crates/route-sim/src/incident.rs",
            "pub struct BundleIncidentSpec;pub fn apply_bundle_incident",
            "bundle-native",
            "attach incidents and SLA outcomes to bundles by default",
        ),
        (
            "route-report",
            "bundle-publication",
            "route_report::write_bundle_corpus_entry",
            "crates/route-report/src/lib.rs",
            "pub fn write_bundle_corpus_entry;bundle:",
            "bundle-native",
            "publish corpus/report artifacts with bundle frontmatter",
        ),
        (
            "route-cli",
            "bundle-orchestration",
            "route national-segment-bundles --gate",
            "crates/route-cli/src/support/network/national_segment_bundle_rows.rs",
            "route_network::build_segment_bundles",
            "bundle-native",
            "orchestrate gates while moving identity policy into library crates",
        ),
    ]
    .into_iter()
    .map(
        |(
            crate_name,
            role,
            bundle_entrypoint,
            source_path,
            required_tokens,
            architecture_status,
            next_action,
        )| {
            let missing_tokens = missing_source_tokens(source_path, required_tokens);
            let validation_status = if missing_tokens.is_empty() {
                "pass"
            } else {
                "missing-token"
            };
            BundleArchitectureRow {
                crate_name: crate_name.to_string(),
                role: role.to_string(),
                bundle_entrypoint: bundle_entrypoint.to_string(),
                source_path: source_path.to_string(),
                required_tokens: required_tokens.to_string(),
                architecture_status: architecture_status.to_string(),
                next_action: if missing_tokens.is_empty() {
                    next_action.to_string()
                } else {
                    format!("restore required tokens: {}", missing_tokens.join(";"))
                },
                validation_status: validation_status.to_string(),
            }
        },
    )
    .collect()
}
