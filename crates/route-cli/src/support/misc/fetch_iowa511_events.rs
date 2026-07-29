//! Helper `fetch_iowa511_events`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn fetch_iowa511_events(output: &Path) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let url = "https://services.arcgis.com/8lRhdTsQyJpO52F1/arcgis/rest/services/CARS511_Iowa_View/FeatureServer/0/query?f=json&where=1%3D1&outFields=ID,Route,StartTime,EndTime,IssueDate,IssueTime,headline,cause,Restrict_,Desc0&returnGeometry=true&outSR=4326";
    let body = reqwest::blocking::get(url)?.error_for_status()?.text()?;
    ensure_no_arcgis_error(&body)?;
    atomic_write_text(output, body)?;
    Ok(())
}

