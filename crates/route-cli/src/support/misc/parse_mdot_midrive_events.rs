//! Helper `parse_mdot_midrive_events`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_mdot_midrive_events(
    json: &str,
    site_id: &str,
    lat: f64,
    lon: f64,
    radius_miles: f64,
    observation_year: u16,
) -> Result<Vec<T1FailureEventRow>> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    let Some(events) = value.as_array() else {
        return Ok(Vec::new());
    };

    let mut rows = Vec::new();
    let mut seen_event_ids = std::collections::BTreeSet::new();
    for event in events {
        let event_lat = event.get("latitude").and_then(|value| value.as_f64());
        let event_lon = event.get("longitude").and_then(|value| value.as_f64());
        if let (Some(event_lat), Some(event_lon)) = (event_lat, event_lon) {
            if haversine_miles(lat, lon, event_lat, event_lon) > radius_miles {
                continue;
            }
        }

        let title = json_value_string(event, "title");
        let message = strip_html_tags(&json_value_string(event, "message"));
        let text = compact_note(&format!("{title} {message}"));
        if !mdot_midrive_is_t1_relevant(&text) {
            continue;
        }

        let source_event_id = event
            .get("id")
            .map(json_scalar_to_string)
            .unwrap_or_default();
        let event_id = if source_event_id.trim().is_empty() {
            format!("MDOT-MIDRIVE-{}", rows.len() + 1)
        } else {
            format!("MDOT-MIDRIVE-{source_event_id}")
        };
        if !seen_event_ids.insert(event_id.clone()) {
            continue;
        }

        let reported_time = extract_after_label(&message, "Reported:");
        rows.push(T1FailureEventRow {
            site_id: site_id.to_string(),
            event_id,
            source: "MDOT Mi Drive".to_string(),
            source_event_id,
            observation_year,
            start_time: reported_time.unwrap_or_default(),
            end_time: String::new(),
            duration_hours: None,
            event_type: mdot_midrive_event_type(&text).to_string(),
            full_closure: mdot_midrive_full_closure(&text),
            lanes_closed: mdot_midrive_lanes_closed(&text),
            freight_relevant: true,
            confidence: "low".to_string(),
            notes: text,
        });
    }
    Ok(rows)
}
