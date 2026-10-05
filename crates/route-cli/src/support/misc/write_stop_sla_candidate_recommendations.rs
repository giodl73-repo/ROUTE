//! Helper `write_stop_sla_candidate_recommendations`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn write_stop_sla_candidate_recommendations(
    output: &Path,
    recommendations: &[StopSlaCandidateRecommendation],
) -> Result<()> {
    if let Some(parent) = output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(output)?;
    writer.write_record([
        "gap_segment",
        "gap_labels",
        "gap_miles",
        "gap_row_count",
        "gap_routes",
        "candidate_rank",
        "candidate_id",
        "candidate_name",
        "candidate_class",
        "candidate_lat",
        "candidate_lon",
        "candidate_source_type",
        "candidate_evidence_status",
        "candidate_route_refs",
        "candidate_basis",
        "largest_resulting_gap_miles",
        "spacing_gain_miles",
        "offset_miles",
        "intersection_route_count",
        "score",
    ])?;
    for rec in recommendations {
        for (idx, candidate) in rec.candidates.iter().enumerate() {
            writer.write_record([
                rec.gap.segment_id.as_str(),
                rec.gap.labels.as_str(),
                &format!("{:.0}", rec.gap.miles),
                &rec.gap.row_count.to_string(),
                rec.gap.route_path.as_str(),
                &(idx + 1).to_string(),
                candidate.stop_id.as_str(),
                candidate.name.as_str(),
                candidate.requested_class.as_str(),
                &format!("{:.4}", candidate.lat),
                &format!("{:.4}", candidate.lon),
                candidate.source_type.as_str(),
                candidate.evidence_status.as_str(),
                candidate.route_refs.as_str(),
                candidate.basis.as_str(),
                &format!("{:.0}", candidate.largest_resulting_gap_miles),
                &format!("{:.0}", candidate.spacing_gain_miles),
                &format!("{:.0}", candidate.distance_from_segment_miles),
                &candidate.intersection_route_count.to_string(),
                &format!("{:.1}", candidate.score),
            ])?;
        }
    }
    writer.flush()?;
    Ok(())
}
