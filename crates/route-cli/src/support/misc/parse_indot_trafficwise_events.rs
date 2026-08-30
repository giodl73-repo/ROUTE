//! Helper `parse_indot_trafficwise_events`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_indot_trafficwise_events(
    json: &str,
    site_id: &str,
    observation_year: u16,
) -> Result<Vec<T1FailureEventRow>> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    ensure_no_graphql_errors(json)?;
    let Some(features) = value
        .get("data")
        .and_then(|value| value.get("mapFeaturesQuery"))
        .and_then(|value| value.get("mapFeatures"))
        .and_then(|value| value.as_array())
    else {
        return Ok(Vec::new());
    };

    let mut rows = Vec::new();
    let mut seen_event_ids = std::collections::BTreeSet::new();
    for feature in features {
        if json_value_string(feature, "__typename") != "Event" {
            continue;
        }
        let title = json_value_string(feature, "title");
        let tooltip = strip_html_tags(&json_value_string(feature, "tooltip"));
        let text = compact_note(&format!("{title} {tooltip}"));
        if !indot_trafficwise_is_t1_relevant(&text) {
            continue;
        }

        let uri = json_value_string(feature, "uri");
        let source_event_id = uri
            .strip_prefix("event/")
            .unwrap_or(uri.as_str())
            .to_string();
        let event_id = if source_event_id.trim().is_empty() {
            format!("INDOT-TRAFFICWISE-{}", rows.len() + 1)
        } else {
            format!("INDOT-TRAFFICWISE-{source_event_id}")
        };
        if !seen_event_ids.insert(event_id.clone()) {
            continue;
        }

        let start_ms = indot_trafficwise_event_millis(feature, "beginTime");
        let end_ms = indot_trafficwise_event_millis(feature, "endTime");
        let duration_hours = match (start_ms, end_ms) {
            (Some(start), Some(end)) if end >= start => Some((end - start) as f64 / 3_600_000.0),
            _ => None,
        };
        let observation_year = start_ms
            .and_then(epoch_millis_year)
            .unwrap_or(observation_year);

        let row = T1FailureEventRow {
            site_id: site_id.to_string(),
            event_id,
            source: "INDOT TrafficWise GraphQL".to_string(),
            source_event_id,
            observation_year,
            start_time: start_ms.and_then(epoch_millis_date).unwrap_or_default(),
            end_time: end_ms.and_then(epoch_millis_date).unwrap_or_default(),
            duration_hours,
            event_type: indot_trafficwise_event_type(&text).to_string(),
            full_closure: indot_trafficwise_full_closure(&text),
            lanes_closed: mdot_midrive_lanes_closed(&text),
            freight_relevant: true,
            confidence: if duration_hours.is_some() {
                "medium".to_string()
            } else {
                "low".to_string()
            },
            notes: text,
        };
        if t1_failure_event_has_observation_contract(&row) {
            rows.push(row);
        }
    }
    Ok(rows)
}
