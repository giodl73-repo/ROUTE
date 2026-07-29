//! Helper `t4_terminal_contact_source_catalog_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_contact_source_catalog_rows(
    plan_rows: &[T4TerminalContactSourcePlanRow],
) -> Vec<T4TerminalContactSourceCatalogRow> {
    let mut districts = std::collections::BTreeMap::<String, usize>::new();
    for row in plan_rows {
        *districts.entry(row.terminal_district.clone()).or_default() += 1;
    }

    districts
        .into_iter()
        .map(|(terminal_district, route_task_count)| T4TerminalContactSourceCatalogRow {
            catalog_id: format!(
                "T4SOURCECATALOG-{}",
                canonical_route_key(&terminal_district)
            ),
            terminal_district,
            route_task_count,
            source_family: "public-terminal-contact-proof".to_string(),
            source_access_mode: "manual-or-cached-source-needed".to_string(),
            required_proof_fields:
                "route; terminal district; route-to-terminal contact statement; source title; source url or cached artifact; capture date"
                    .to_string(),
            acquisition_status: "source-needed".to_string(),
            proof_blocker:
                "no safe live fetcher or cached contact proof source is registered for this district"
                    .to_string(),
            cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                .to_string(),
            next_artifact:
                "waves/2026-05-13-terminal-contact-source-acquisition-spine/plans/pulse-03.md"
                    .to_string(),
            validation_status: "review".to_string(),
        })
        .collect()
}

