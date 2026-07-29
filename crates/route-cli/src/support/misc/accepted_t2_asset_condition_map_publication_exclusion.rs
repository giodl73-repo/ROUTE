//! Helper `accepted_t2_asset_condition_map_publication_exclusion`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn accepted_t2_asset_condition_map_publication_exclusion(
    rows: &[T2AssetConditionMapPublicationExclusionRow],
) -> Option<&T2AssetConditionMapPublicationExclusionRow> {
    rows.iter().find(|row| {
        row.decision == "exclude-asset-condition-debt-from-map-publication"
            && row.validation_status == "accepted"
            && row.affected_constraint_class == "asset_condition_debt"
            && row.affected_tier == "T2"
            && row.excluded_claims == "publication"
            && row.preserved_claims_after == "sla|transit|upgrade"
    })
}

