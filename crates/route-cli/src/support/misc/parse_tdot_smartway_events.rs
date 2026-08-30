//! Helper `parse_tdot_smartway_events`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_tdot_smartway_events(
    json: &str,
    site_id: &str,
    lat: f64,
    lon: f64,
    radius_miles: f64,
) -> Result<Vec<T1FailureEventRow>> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    ensure_no_arcgis_error(json)?;
    let Some(features) = value.get("features").and_then(|value| value.as_array()) else {
        return Ok(Vec::new());
    };

    let mut rows = Vec::new();
    let mut seen_event_ids = std::collections::BTreeSet::new();
    for feature in features {
        let attrs = feature
            .get("attributes")
            .and_then(|value| value.as_object())
            .cloned()
            .unwrap_or_default();
        let event_lat = json_f64(&attrs, "MIDPOINT_LATITUDE_DD");
        let event_lon = json_f64(&attrs, "MIDPOINT_LONGITUDE_DD");
        if let (Some(event_lat), Some(event_lon)) = (event_lat, event_lon) {
            if haversine_miles(lat, lon, event_lat, event_lon) > radius_miles {
                continue;
            }
        }

        let road_names = json_string(&attrs, "CD_ROAD_NAMES");
        let text = [
            road_names.as_str(),
            json_string(&attrs, "CD_DIRECTION").as_str(),
            json_string(&attrs, "EVENT_TYPE").as_str(),
            json_string(&attrs, "EVENT_SUBTYPE").as_str(),
            json_string(&attrs, "DESCRIPTION").as_str(),
            json_string(&attrs, "COUNTY_NAME").as_str(),
        ]
        .join(" ");
        if !tdot_smartway_is_t1_relevant(&road_names, &text) {
            continue;
        }

        let source_event_id = json_string(&attrs, "ID");
        let event_id = if source_event_id.trim().is_empty() {
            format!("TDOT-SMARTWAY-{}", rows.len() + 1)
        } else {
            format!("TDOT-SMARTWAY-{source_event_id}")
        };
        if !seen_event_ids.insert(event_id.clone()) {
            continue;
        }

        let start_ms = json_i64(&attrs, "START_DATE");
        let end_ms = json_i64(&attrs, "END_DATE");
        let duration_hours = match (start_ms, end_ms) {
            (Some(start), Some(end)) if end >= start => Some((end - start) as f64 / 3_600_000.0),
            _ => None,
        };
        let observation_year = start_ms.and_then(epoch_millis_year).unwrap_or(0);

        rows.push(T1FailureEventRow {
            site_id: site_id.to_string(),
            event_id,
            source: "TDOT SmartWay ArcGIS".to_string(),
            source_event_id,
            observation_year,
            start_time: start_ms
                .and_then(epoch_millis_date)
                .unwrap_or_else(|| json_string(&attrs, "START_DATE")),
            end_time: end_ms
                .and_then(epoch_millis_date)
                .unwrap_or_else(|| json_string(&attrs, "END_DATE")),
            duration_hours,
            event_type: tdot_smartway_event_type(&text).to_string(),
            full_closure: json_i64(&attrs, "HAS_CLOSURE").unwrap_or(0) > 0,
            lanes_closed: None,
            freight_relevant: true,
            confidence: if duration_hours.is_some() {
                "medium".to_string()
            } else {
                "low".to_string()
            },
            notes: compact_note(&text),
        });
    }
    Ok(rows)
}
