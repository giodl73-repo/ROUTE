//! Helper `print_stop_sla_candidate_recommendations`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_stop_sla_candidate_recommendations(
    recommendations: &[StopSlaCandidateRecommendation],
    target_gap: f64,
    candidates_per_gap: usize,
) {
    println!("  target gap: >{target_gap:.0} mi");
    println!("  inspected gaps: {}", recommendations.len());
    println!();
    for rec in recommendations {
        println!(
            "{}  {:.0} mi  rows={}  routes={}",
            rec.gap.segment_id, rec.gap.miles, rec.gap.row_count, rec.gap.route_path
        );
        println!("  {}", rec.gap.labels);
        if rec.candidates.is_empty() {
            println!("  no ledger candidates near this segment");
            println!();
            continue;
        }
        println!(
            "  {:<16} {:<24} {:<5} {:>7} {:>7} {:>7} {:>5} {:<12} Routes",
            "Stop", "Name", "Class", "NewMax", "Gain", "Offset", "Xfer", "Source"
        );
        for candidate in rec.candidates.iter().take(candidates_per_gap.max(1)) {
            println!(
                "  {:<16} {:<24} {:<5} {:>7.0} {:>7.0} {:>7.0} {:>5} {:<12} {}",
                candidate.stop_id,
                truncate_for_table(&candidate.name, 24),
                candidate.requested_class,
                candidate.largest_resulting_gap_miles,
                candidate.spacing_gain_miles,
                candidate.distance_from_segment_miles,
                candidate.intersection_route_count,
                truncate_for_table(&candidate.source_type, 12),
                truncate_for_table(&candidate.route_refs, 28)
            );
            println!(
                "    score={:.1} evidence={}",
                candidate.score, candidate.evidence_status
            );
        }
        println!();
    }
}

