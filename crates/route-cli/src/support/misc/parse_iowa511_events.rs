//! Helper `parse_iowa511_events`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_iowa511_events(
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
        let geometry = feature.get("geometry");
        let event_lon = geometry
            .and_then(|value| value.get("x"))
            .and_then(|value| value.as_f64());
        let event_lat = geometry
            .and_then(|value| value.get("y"))
            .and_then(|value| value.as_f64());
        if let (Some(event_lat), Some(event_lon)) = (event_lat, event_lon) {
            if haversine_miles(lat, lon, event_lat, event_lon) > radius_miles {
                continue;
            }
        }

        let route = json_string(&attrs, "Route");
        let text = [
            route.as_str(),
            json_string(&attrs, "headline").as_str(),
            json_string(&attrs, "cause").as_str(),
            json_string(&attrs, "Restrict_").as_str(),
            json_string(&attrs, "Desc0").as_str(),
        ]
        .join(" ");
        if !iowa511_is_t1_relevant(&route, &text) {
            continue;
        }

        let issue_date = json_string(&attrs, "IssueDate");
        let observation_year = issue_date
            .get(0..4)
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(0);
        let start_time = json_string(&attrs, "StartTime");
        let end_time = json_string(&attrs, "EndTime");
        let duration_hours = same_day_duration_hours(&start_time, &end_time);
        let source_event_id = json_string(&attrs, "ID");
        let event_id = if source_event_id.trim().is_empty() {
            format!("IOWA511-{}", rows.len() + 1)
        } else {
            format!("IOWA511-{source_event_id}")
        };
        if !seen_event_ids.insert(event_id.clone()) {
            continue;
        }

        rows.push(T1FailureEventRow {
            site_id: site_id.to_string(),
            event_id,
            source: "Iowa DOT 511 ArcGIS".to_string(),
            source_event_id,
            observation_year,
            start_time: combine_iowa_date_time(&issue_date, &start_time),
            end_time: combine_iowa_date_time(&issue_date, &end_time),
            duration_hours,
            event_type: iowa511_event_type(&text).to_string(),
            full_closure: iowa511_full_closure(&text),
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
