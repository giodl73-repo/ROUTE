//! Frozen local-cache observation; no source acquisition or traffic inference.
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeSet;
const SNAPSHOT: &str = include_str!("../../../data/browser-i80-source-checklist-2026-10-29.json");
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum Status {
    Ready,
    Blocked,
    Excluded,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Query {
    status: Option<Status>,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Contract {
    source_id: String,
    artifact: String,
    publisher: String,
    current_source_year: String,
    candidate_source_year: String,
    acquisition_status: String,
    credential_mode: String,
    current_command: String,
    authoritative_url: String,
    local_parser_status: String,
    blocking_gap: String,
    next_action: String,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Observation {
    source_id: String,
    artifact: String,
    current_source_year: String,
    acquisition_status: String,
    attempt_status: String,
    readiness_status: Status,
    evidence_count: String,
    evidence_detail: String,
    attempt_detail: String,
    blocking_gap: String,
    next_action: String,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Row {
    contract: Contract,
    observation: Observation,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Discrepancy {
    packet_hpms_label: String,
    contract_hpms_year: String,
    status: String,
    packet_values_relinked: bool,
}
#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Snapshot {
    schema: String,
    recorded_at_utc: String,
    origin_commit: String,
    capture_mode: String,
    acquisition_performed: bool,
    primary_hashes_reverified: bool,
    raw_primary_bytes_in_bundle: bool,
    contract_sha256: String,
    preparation_script_sha256: String,
    reviewed_packet_sha256: String,
    vintage_discrepancy: Discrepancy,
    rows: Vec<Row>,
}
fn parse<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, String> {
    serde_json::from_str(s).map_err(|e| e.to_string())
}
impl Snapshot {
    fn counts(&self) -> [usize; 3] {
        [Status::Ready, Status::Blocked, Status::Excluded].map(|s| {
            self.rows
                .iter()
                .filter(|r| r.observation.readiness_status == s)
                .count()
        })
    }
    fn validate(&self) -> Result<(), String> {
        if self.schema != "route.i80-recorded-cache-checklist.v1"
            || self.rows.len() != 12
            || self.counts() != [4, 2, 6]
            || self.acquisition_performed
            || self.primary_hashes_reverified
            || self.raw_primary_bytes_in_bundle
        {
            return Err("Frozen source partition or capture boundaries changed".into());
        }
        let mut ids = BTreeSet::new();
        for r in &self.rows {
            let c = &r.contract;
            let o = &r.observation;
            if !ids.insert(&c.source_id)
                || c.source_id != o.source_id
                || c.artifact != o.artifact
                || c.current_source_year != o.current_source_year
                || c.acquisition_status != o.acquisition_status
                || c.next_action != o.next_action
                || o.attempt_status != "not-attempted"
                || !o.attempt_detail.is_empty()
            {
                return Err(
                    "Frozen source identity or acquisition-attempt metadata differs".into(),
                );
            }
            let n = o
                .evidence_count
                .parse::<u64>()
                .map_err(|_| "Recorded artifact count must be an unsigned integer")?;
            if o.readiness_status == Status::Ready && n == 0 {
                return Err("Recorded-ready entries need recorded artifact evidence".into());
            }
            if !c.authoritative_url.starts_with("https://") {
                return Err("Publisher reference must retain HTTPS".into());
            }
        }
        let d = &self.vintage_discrepancy;
        if d.packet_hpms_label != "HPMS 2023"
            || d.contract_hpms_year != "2018"
            || d.status != "unresolved"
            || d.packet_values_relinked
            || !self.rows.iter().any(|r| {
                r.contract.source_id == "SRC-I80-HPMS" && r.contract.current_source_year == "2018"
            })
        {
            return Err(
                "Unresolved packet/acquisition vintage conflict must remain explicit".into(),
            );
        }
        if self.origin_commit.len() != 40
            || !self.recorded_at_utc.starts_with("2026-10-29")
            || [
                &self.contract_sha256,
                &self.preparation_script_sha256,
                &self.reviewed_packet_sha256,
            ]
            .iter()
            .any(|s| s.len() != 64 || !s.bytes().all(|b| b.is_ascii_hexdigit()))
        {
            return Err("Recorded metadata provenance is incomplete".into());
        }
        Ok(())
    }
}
pub fn evaluate(input: &str) -> Result<String, String> {
    if input.len() > 65536 {
        return Err("Use at most 64 KiB of source-checklist JSON".into());
    }
    let q: Query = parse(input)?;
    let s: Snapshot = parse(SNAPSHOT)?;
    s.validate()?;
    let counts = s.counts();
    let selected: Vec<_> = s
        .rows
        .iter()
        .filter(|r| {
            q.status
                .is_none_or(|status| r.observation.readiness_status == status)
        })
        .collect();
    serde_json::to_string(&json!({"schema":"route.i80-source-checklist.v1","query":{"status":q.status},"selected_source_entries":selected.len(),"recorded_summary":{"source_entries":s.rows.len(),"ready":counts[0],"blocked":counts[1],"excluded":counts[2]},"selected_rows":selected,
 "recording":{"recorded_at_utc":s.recorded_at_utc,"origin_commit":s.origin_commit,"capture_mode":s.capture_mode,"acquisition_performed":false,"raw_primary_bytes_in_bundle":false,"primary_hashes_reverified":false,"contract_sha256":s.contract_sha256,"preparation_script_sha256":s.preparation_script_sha256,"reviewed_packet_sha256":s.reviewed_packet_sha256},
 "vintage_discrepancy":s.vintage_discrepancy,
 "interpretation":{"grain":"source entries with dated local artifact observations; heterogeneous evidence counts must not be added","ready_meaning":"recorded local artifact checks only, not current browser/CI cache readiness, source-byte verification or packet regeneration proof","years":"current_source_year and candidate_source_year are frozen contract labels, not today's latest source or endpoint health","packet_posture":"Hold and narrow; original packet quantities/corpus are unchanged and not relinked to acquired traffic"},
 "authority":{"approved_design":false,"guaranteed_sla":false,"positive_roi":false,"fiscal_savings":false,"current_source_refresh":false,"national_traffic_provenance":false,"agency_endorsement":false},"unobserved":{"current_aadt":null,"intervention_effect":null,"matched_packet_traffic_vintage":null}
 })).map_err(|e|e.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    #[test]
    fn recorded_filters_preserve_source_grain_and_conflict() {
        for (status, count) in [
            (None, 12),
            (Some("ready"), 4),
            (Some("blocked"), 2),
            (Some("excluded"), 6),
        ] {
            let d: Value =
                parse(&evaluate(&json!({"status":status}).to_string()).unwrap()).unwrap();
            assert_eq!(d["selected_source_entries"], count);
            assert_eq!(
                d["recorded_summary"],
                json!({"source_entries":12,"ready":4,"blocked":2,"excluded":6})
            );
            assert_eq!(d["vintage_discrepancy"]["status"], "unresolved");
            assert!(d["authority"]
                .as_object()
                .unwrap()
                .values()
                .all(|v| v == false));
            assert!(d["unobserved"]
                .as_object()
                .unwrap()
                .values()
                .all(Value::is_null));
        }
    }
    #[test]
    fn rejects_unsupported_filters_and_oversized_queries() {
        for s in [
            r#"{"status":"current"}"#,
            r#"{"status":"ready","source_id":"SRC-I80-HPMS"}"#,
            r#"{"date":"2026-10-29"}"#,
            r#"{"corridor":0}"#,
        ] {
            assert!(evaluate(s).is_err());
        }
        assert!(evaluate(&"x".repeat(65537)).is_err());
    }
    #[test]
    fn invalid_recorded_identity_and_promoted_flags_fail() {
        let mut s: Snapshot = parse(SNAPSHOT).unwrap();
        s.rows[0].observation.source_id = "other".into();
        assert!(s.validate().is_err());
        let mut s: Snapshot = parse(SNAPSHOT).unwrap();
        s.vintage_discrepancy.packet_values_relinked = true;
        assert!(s.validate().is_err());
        let mut s: Snapshot = parse(SNAPSHOT).unwrap();
        s.primary_hashes_reverified = true;
        assert!(s.validate().is_err());
        let mut s: Snapshot = parse(SNAPSHOT).unwrap();
        s.rows[0].observation.evidence_count = "0".into();
        assert!(s.validate().is_err());
    }
}
