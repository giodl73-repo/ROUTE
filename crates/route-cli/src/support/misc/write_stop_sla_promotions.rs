//! Helper `write_stop_sla_promotions`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn write_stop_sla_promotions(output: &Path, rows: &[StopCandidateRow]) -> Result<()> {
    if let Some(parent) = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(output)?;
    writer.write_record([
        "stop_id",
        "name",
        "state",
        "lat",
        "lon",
        "requested_class",
        "route_refs",
        "stop_role",
        "transfer_value",
        "freight_volume",
        "spacing_need",
        "resilience_value",
        "energy_service",
        "land_ops_feasibility",
        "equity_community",
        "evidence_status",
        "source_artifact",
        "next_step",
    ])?;
    for row in rows {
        writer.write_record([
            row.stop_id.as_str(),
            row.name.as_str(),
            row.state.as_str(),
            row.lat.as_str(),
            row.lon.as_str(),
            row.requested_class.as_str(),
            row.route_refs.as_str(),
            row.stop_role.as_str(),
            row.transfer_value.as_str(),
            row.freight_volume.as_str(),
            row.spacing_need.as_str(),
            row.resilience_value.as_str(),
            row.energy_service.as_str(),
            row.land_ops_feasibility.as_str(),
            row.equity_community.as_str(),
            row.evidence_status.as_str(),
            row.source_artifact.as_str(),
            row.next_step.as_str(),
        ])?;
    }
    writer.flush()?;
    Ok(())
}

