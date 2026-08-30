//! Helper `stop_candidate_gate_failures`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn stop_candidate_gate_failures(rows: &[&StopCandidateRow]) -> Vec<String> {
    let mut failures = Vec::new();
    for row in rows {
        let id = row.stop_id.trim();
        let class = row.requested_class.trim().to_ascii_uppercase();
        let routes = stop_candidate_routes(row);
        if id.is_empty() {
            failures.push("row missing stop_id".to_string());
        }
        if row.name.trim().is_empty() {
            failures.push(format!("{id}: missing name"));
        }
        if !matches!(class.as_str(), "S1" | "S2" | "S3" | "S4" | "S5") {
            failures.push(format!(
                "{id}: unsupported requested_class {}",
                row.requested_class
            ));
        }
        if routes.is_empty() {
            failures.push(format!("{id}: missing route_refs"));
        }
        if row.stop_role.trim().is_empty() {
            failures.push(format!("{id}: missing stop_role"));
        }
        if row.source_artifact.trim().is_empty() {
            failures.push(format!("{id}: missing source_artifact"));
        }
        if row.next_step.trim().is_empty() {
            failures.push(format!("{id}: missing next_step"));
        }
        if !valid_stop_evidence_status(&row.evidence_status) {
            failures.push(format!(
                "{id}: unsupported evidence_status {}",
                row.evidence_status
            ));
        }

        if matches!(class.as_str(), "S1" | "S2") {
            let role = row.stop_role.to_ascii_lowercase();
            let has_terminal_role = role.contains("national_terminal")
                || role.contains("major_interchange_hub")
                || role.contains("port_gateway")
                || role.contains("intermodal_gateway");
            let one_route_terminal = class == "S1"
                && role.contains("national_terminal")
                && (role.contains("border_gateway") || role.contains("port_gateway"));
            if !has_terminal_role {
                failures.push(format!(
                    "{id}: {class} needs national_terminal, major_interchange_hub, port_gateway, or intermodal_gateway role"
                ));
            }
            if routes.len() < 2 && !one_route_terminal {
                failures.push(format!("{id}: {class} needs at least two route_refs"));
            }
            if !high_or_medium(&row.transfer_value)
                && !high_or_medium(&row.freight_volume)
                && !high_or_medium(&row.resilience_value)
            {
                failures.push(format!(
                    "{id}: {class} needs medium/high transfer, freight, or resilience value"
                ));
            }
        }

        if class == "S3"
            && routes.len() < 2
            && !row.stop_role.to_ascii_lowercase().contains("transfer")
        {
            failures.push(format!(
                "{id}: S3 needs a transfer role or at least two route_refs"
            ));
        }

        if class == "S4" && row.spacing_need.trim().is_empty() {
            failures.push(format!("{id}: S4 needs spacing_need evidence"));
        }
    }
    failures
}
