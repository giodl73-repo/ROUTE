use std::collections::{BTreeMap, BTreeSet};

/// Segment member input used to build the service/corridor bundle registry.
///
/// Bundles are the route system's service abstraction. Segment ids remain the
/// auditable physical members inside each bundle.
#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SegmentBundleMember {
    pub national_segment_id: String,
    pub segment_bundle_id: String,
    pub bundle_role: String,
    pub stitch_group_id: String,
    pub current_tier: String,
    pub current_zone_id: String,
    pub route_label: String,
    pub state_scope: String,
    pub evidence_state_scope: String,
    pub geometry_state_scope: String,
    pub bundle_aliases: String,
    pub source_artifacts: String,
    pub registry_action: String,
    pub validation_status: String,
    pub member_segment_ids: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct SegmentBundle {
    pub segment_bundle_id: String,
    pub bundle_role: String,
    pub member_segment_ids: Vec<String>,
    pub stitch_group_ids: Vec<String>,
    pub current_tiers: Vec<String>,
    pub current_zone_ids: Vec<String>,
    pub route_labels: Vec<String>,
    pub state_scope: Vec<String>,
    pub evidence_state_scope: Vec<String>,
    pub geometry_state_scope: Vec<String>,
    pub bundle_aliases: Vec<String>,
    pub source_artifacts: Vec<String>,
    pub registry_actions: Vec<String>,
    pub validation_statuses: Vec<String>,
    pub bundle_status: BundleStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum BundleStatus {
    BundleReady,
    BundleReview,
    InvalidSingleSegmentBundle,
    MissingMembers,
    NeedsStitchedMembers,
    NeedsStopChain,
    NeedsTerminalStop,
}

impl BundleStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BundleReady => "bundle-ready",
            Self::BundleReview => "bundle-review",
            Self::InvalidSingleSegmentBundle => "invalid-single-segment-bundle",
            Self::MissingMembers => "missing-members",
            Self::NeedsStitchedMembers => "needs-stitched-members",
            Self::NeedsStopChain => "needs-stop-chain",
            Self::NeedsTerminalStop => "needs-terminal-stop",
        }
    }

    pub fn validation_status(self) -> &'static str {
        if self == Self::BundleReady {
            "pass"
        } else {
            "review"
        }
    }
}

#[derive(Default)]
struct SegmentBundleBuilder {
    segment_bundle_id: String,
    bundle_role: String,
    member_segment_ids: BTreeSet<String>,
    stitch_group_ids: BTreeSet<String>,
    current_tiers: BTreeSet<String>,
    current_zone_ids: BTreeSet<String>,
    route_labels: BTreeSet<String>,
    state_scope: BTreeSet<String>,
    evidence_state_scope: BTreeSet<String>,
    geometry_state_scope: BTreeSet<String>,
    bundle_aliases: BTreeSet<String>,
    source_artifacts: BTreeSet<String>,
    registry_actions: BTreeSet<String>,
    validation_statuses: BTreeSet<String>,
}

pub fn build_segment_bundles(members: &[SegmentBundleMember]) -> Vec<SegmentBundle> {
    let mut builders = BTreeMap::<String, SegmentBundleBuilder>::new();

    for member in members {
        let builder = builders
            .entry(member.segment_bundle_id.clone())
            .or_insert_with(|| SegmentBundleBuilder {
                segment_bundle_id: member.segment_bundle_id.clone(),
                bundle_role: member.bundle_role.clone(),
                ..Default::default()
            });
        if builder.bundle_role.is_empty() {
            builder.bundle_role = member.bundle_role.clone();
        }
        insert_semicolon_values(&mut builder.member_segment_ids, &member.member_segment_ids);
        if builder.member_segment_ids.is_empty() && !member.national_segment_id.trim().is_empty() {
            builder
                .member_segment_ids
                .insert(member.national_segment_id.clone());
        }
        insert_non_empty(&mut builder.stitch_group_ids, &member.stitch_group_id);
        insert_non_empty(&mut builder.current_tiers, &member.current_tier);
        insert_non_empty(&mut builder.current_zone_ids, &member.current_zone_id);
        insert_non_empty(&mut builder.route_labels, &member.route_label);
        insert_semicolon_values(&mut builder.state_scope, &member.state_scope);
        insert_semicolon_values(
            &mut builder.evidence_state_scope,
            &member.evidence_state_scope,
        );
        insert_semicolon_values(
            &mut builder.geometry_state_scope,
            &member.geometry_state_scope,
        );
        insert_semicolon_values(&mut builder.bundle_aliases, &member.bundle_aliases);
        insert_semicolon_values(&mut builder.source_artifacts, &member.source_artifacts);
        insert_non_empty(&mut builder.registry_actions, &member.registry_action);
        insert_non_empty(&mut builder.validation_statuses, &member.validation_status);
    }

    builders
        .into_values()
        .map(|builder| {
            let status = bundle_status(
                &builder.bundle_role,
                builder.member_segment_ids.len(),
                &builder.registry_actions,
                &builder.validation_statuses,
            );
            SegmentBundle {
                segment_bundle_id: builder.segment_bundle_id,
                bundle_role: builder.bundle_role,
                member_segment_ids: builder.member_segment_ids.into_iter().collect(),
                stitch_group_ids: builder.stitch_group_ids.into_iter().collect(),
                current_tiers: builder.current_tiers.into_iter().collect(),
                current_zone_ids: builder.current_zone_ids.into_iter().collect(),
                route_labels: builder.route_labels.into_iter().collect(),
                state_scope: builder.state_scope.into_iter().collect(),
                evidence_state_scope: builder.evidence_state_scope.into_iter().collect(),
                geometry_state_scope: builder.geometry_state_scope.into_iter().collect(),
                bundle_aliases: builder.bundle_aliases.into_iter().collect(),
                source_artifacts: builder.source_artifacts.into_iter().collect(),
                registry_actions: builder.registry_actions.into_iter().collect(),
                validation_statuses: builder.validation_statuses.into_iter().collect(),
                bundle_status: status,
            }
        })
        .collect()
}

pub fn bundle_action(
    status: BundleStatus,
    registry_actions: &[String],
) -> (&'static str, &'static str) {
    match status {
        BundleStatus::BundleReady => ("use bundle as service join surface", "maps/t3-zone"),
        BundleStatus::NeedsTerminalStop => (
            "complete terminal stop chain before service geometry",
            "data/t3-zone-stop-placement.csv",
        ),
        BundleStatus::NeedsStopChain => (
            "author zone-bounded stops before bundle geometry",
            "data/tier-stop-candidates.csv",
        ),
        BundleStatus::NeedsStitchedMembers => (
            "add ordered member segments before promotion or stitched service",
            "data/national-segment-registry.csv",
        ),
        _ if registry_actions
            .iter()
            .any(|action| action == "track-zone-or-backlog-identity") =>
        {
            (
                "keep zone or backlog bundle visible but out of service geometry",
                "data/national-segment-registry.csv",
            )
        }
        _ => (
            "review bundle membership and identity fields",
            "data/national-segment-registry.csv",
        ),
    }
}

fn bundle_status(
    bundle_role: &str,
    member_count: usize,
    registry_actions: &BTreeSet<String>,
    validation_statuses: &BTreeSet<String>,
) -> BundleStatus {
    if member_count == 0 {
        return BundleStatus::MissingMembers;
    }
    if bundle_role == "single-segment" && member_count != 1 {
        return BundleStatus::InvalidSingleSegmentBundle;
    }
    if bundle_role != "single-segment" && member_count < 2 {
        return BundleStatus::NeedsStitchedMembers;
    }
    if registry_actions
        .iter()
        .any(|action| action == "author-zone-bounded-stop-chain")
    {
        return BundleStatus::NeedsStopChain;
    }
    if registry_actions
        .iter()
        .any(|action| action == "complete-terminal-stop-chain")
    {
        return BundleStatus::NeedsTerminalStop;
    }
    if validation_statuses.iter().all(|status| status == "pass") {
        BundleStatus::BundleReady
    } else {
        BundleStatus::BundleReview
    }
}

fn insert_non_empty(target: &mut BTreeSet<String>, value: &str) {
    if !value.trim().is_empty() {
        target.insert(value.to_string());
    }
}

fn insert_semicolon_values(target: &mut BTreeSet<String>, value: &str) {
    for item in value
        .split(';')
        .map(str::trim)
        .filter(|item| !item.is_empty())
    {
        target.insert(item.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::{build_segment_bundles, BundleStatus, SegmentBundleMember};

    #[test]
    fn bundles_are_built_as_service_objects_from_segment_members() {
        let rows = vec![
            SegmentBundleMember {
                national_segment_id: "US.HWYSEG.A".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.SERVICE".to_string(),
                bundle_role: "stitched-corridor".to_string(),
                stitch_group_id: "US.HWYSTITCH.SERVICE".to_string(),
                current_tier: "T2".to_string(),
                current_zone_id: "midwest".to_string(),
                route_label: "I80".to_string(),
                state_scope: "IA".to_string(),
                evidence_state_scope: "IA".to_string(),
                geometry_state_scope: String::new(),
                bundle_aliases: "route:I80".to_string(),
                source_artifacts: "fixture-a".to_string(),
                registry_action: "eligible-for-geometry-layout".to_string(),
                validation_status: "pass".to_string(),
                member_segment_ids: "US.HWYSEG.A".to_string(),
            },
            SegmentBundleMember {
                national_segment_id: "US.HWYSEG.B".to_string(),
                segment_bundle_id: "US.HWYBUNDLE.SERVICE".to_string(),
                bundle_role: "stitched-corridor".to_string(),
                stitch_group_id: "US.HWYSTITCH.SERVICE".to_string(),
                current_tier: "T2".to_string(),
                current_zone_id: "midwest".to_string(),
                route_label: "I80".to_string(),
                state_scope: "IL".to_string(),
                evidence_state_scope: "IL".to_string(),
                geometry_state_scope: String::new(),
                bundle_aliases: "route:I80".to_string(),
                source_artifacts: "fixture-b".to_string(),
                registry_action: "eligible-for-geometry-layout".to_string(),
                validation_status: "pass".to_string(),
                member_segment_ids: "US.HWYSEG.B".to_string(),
            },
        ];

        let bundles = build_segment_bundles(&rows);

        assert_eq!(bundles.len(), 1);
        assert_eq!(bundles[0].segment_bundle_id, "US.HWYBUNDLE.SERVICE");
        assert_eq!(
            bundles[0].member_segment_ids,
            ["US.HWYSEG.A", "US.HWYSEG.B"]
        );
        assert_eq!(bundles[0].state_scope, ["IA", "IL"]);
        assert_eq!(bundles[0].bundle_status, BundleStatus::BundleReady);
    }
}
