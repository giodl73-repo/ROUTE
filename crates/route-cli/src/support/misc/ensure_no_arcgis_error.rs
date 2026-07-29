//! Helper `ensure_no_arcgis_error`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn ensure_no_arcgis_error(json: &str) -> Result<()> {
    let value: serde_json::Value = serde_json::from_str(json)?;
    if let Some(error) = value.get("error") {
        let message = error
            .get("message")
            .and_then(|value| value.as_str())
            .unwrap_or("ArcGIS query failed");
        let details = error
            .get("details")
            .and_then(|value| value.as_array())
            .map(|values| {
                values
                    .iter()
                    .filter_map(|value| value.as_str())
                    .collect::<Vec<_>>()
                    .join("; ")
            })
            .unwrap_or_default();
        if details.is_empty() {
            anyhow::bail!("{message}");
        } else {
            anyhow::bail!("{message}: {details}");
        }
    }
    Ok(())
}

