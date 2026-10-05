//! Helper `fetch_indot_trafficwise_events`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn fetch_indot_trafficwise_events(
    output: &Path,
    north: f64,
    south: f64,
    east: f64,
    west: f64,
    zoom: u8,
) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let query = r#"
query MapFeatures($input: MapFeaturesArgs!) {
  mapFeaturesQuery(input: $input) {
    mapFeatures {
      bbox
      title
      tooltip
      uri
      __typename
      features {
        id
        geometry
        properties
        type
      }
    }
    error {
      message
      type
    }
  }
}
"#;
    let body = serde_json::json!({
        "query": query,
        "variables": {
            "input": {
                "north": north,
                "south": south,
                "east": east,
                "west": west,
                "zoom": zoom,
                "layerSlugs": ["incidents", "construction"]
            }
        }
    });
    let client = reqwest::blocking::Client::new();
    let request_body = serde_json::to_string(&body)?;
    let text = client
        .post("https://511in.org/api/graphql")
        .header("content-type", "application/json")
        .body(request_body)
        .send()?
        .error_for_status()?
        .text()?;
    ensure_no_graphql_errors(&text)?;
    atomic_write_text(output, text)?;
    Ok(())
}
