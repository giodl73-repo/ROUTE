//! Helper `fetch_tdot_smartway_events`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn fetch_tdot_smartway_events(output: &Path, timeout_seconds: u64) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let url = "https://spatial.tdot.tn.gov/arcgis/rest/services/Smartway/Smartway_Events/FeatureServer/1/query?f=json&where=1%3D1&outFields=ID,START_DATE,END_DATE,CD_ROAD_NAMES,CD_DIRECTION,EVENT_TYPE,EVENT_SUBTYPE,DESCRIPTION,HAS_CLOSURE,MIDPOINT_LATITUDE_DD,MIDPOINT_LONGITUDE_DD,COUNTY_NAME&returnGeometry=false&resultRecordCount=200";
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(timeout_seconds.max(1)))
        .build()?;
    let body = client.get(url).send()?.error_for_status()?.text()?;
    ensure_no_arcgis_error(&body)?;
    atomic_write_text(output, body)?;
    Ok(())
}

