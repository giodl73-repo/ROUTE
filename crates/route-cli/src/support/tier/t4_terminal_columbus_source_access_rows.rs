//! Helper `t4_terminal_columbus_source_access_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn t4_terminal_columbus_source_access_rows(
    intake_rows: &[T4TerminalColumbusProofIntakeRow],
) -> Vec<T4TerminalColumbusSourceAccessRow> {
    let mut rows = intake_rows
        .iter()
        .map(|row| T4TerminalColumbusSourceAccessRow {
            access_id: format!("T4COLUMBUSACCESS-{}", stable_id_fragment(&row.queue_id)),
            intake_id: row.intake_id.clone(),
            queue_id: row.queue_id.clone(),
            route: row.route.clone(),
            terminal_district: row.terminal_district.clone(),
            source_family: row.source_family.clone(),
            access_mode: "manual-or-cached-source-needed".to_string(),
            live_fetch_status: "unsupported-no-safe-terminal-fetcher".to_string(),
            required_source_metadata:
                "source title; source url or cached artifact; capture date; route; terminal district; route-to-terminal contact statement"
                    .to_string(),
            contact_proof_source_artifact: "source-needed".to_string(),
            acquisition_status: "source-needed".to_string(),
            source_access_blocker:
                "no safe live terminal-contact fetch command exists; use manual/cached proof artifact or add policy-compliant fetcher"
                    .to_string(),
            cache_policy_artifact: "docs/source-fetch-cache-policy.md;data/source-fetch-policy.csv"
                .to_string(),
            next_artifact:
                "waves/2026-05-13-columbus-south-terminal-contact-proof/plans/pulse-03.md"
                    .to_string(),
            validation_status: "review".to_string(),
        })
        .collect::<Vec<_>>();

    rows.sort_by(|a, b| {
        a.route
            .cmp(&b.route)
            .then_with(|| a.queue_id.cmp(&b.queue_id))
    });
    rows
}
