//! Helper `write_hpms_records`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn write_hpms_records(path: &Path, records: &[route_data::HpmsRecord]) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let tmp = temp_path_for_atomic_write(path);
    let mut writer = csv::Writer::from_path(&tmp)?;
    writer.write_record([
        "STATE",
        "ROUTE_ID",
        "AADT",
        "PCT_TRUCK",
        "LANE_COUNT",
        "IRI",
        "SPEED_LIMIT",
    ])?;
    for record in records {
        writer.write_record(&[
            record.state.clone(),
            record.route_id.clone(),
            record
                .aadt
                .map(|value| value.to_string())
                .unwrap_or_default(),
            record
                .pct_truck
                .map(|value| format!("{value:.4}"))
                .unwrap_or_default(),
            record
                .lane_count
                .map(|value| value.to_string())
                .unwrap_or_default(),
            record
                .iri
                .map(|value| format!("{value:.1}"))
                .unwrap_or_default(),
            record
                .speed_limit
                .map(|value| value.to_string())
                .unwrap_or_default(),
        ])?;
    }
    writer.flush()?;
    drop(writer);
    replace_with_atomic_write(&tmp, path)?;
    Ok(())
}

