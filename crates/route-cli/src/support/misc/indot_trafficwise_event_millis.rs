//! Helper `indot_trafficwise_event_millis`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn indot_trafficwise_event_millis(
    feature: &serde_json::Value,
    key: &str,
) -> Option<i64> {
    feature
        .get("_eventReport")
        .and_then(|value| value.get(key))
        .and_then(|value| value.get("time"))
        .and_then(json_value_i64)
        .or_else(|| {
            feature
                .get(key)
                .and_then(|value| value.get("timestamp"))
                .and_then(json_value_i64)
        })
        .or_else(|| {
            feature
                .get("_eventMapFeature")
                .and_then(|value| {
                    if key == "beginTime" {
                        value.get("startTime")
                    } else {
                        value.get(key)
                    }
                })
                .and_then(|value| value.get("time"))
                .and_then(json_value_i64)
        })
}
