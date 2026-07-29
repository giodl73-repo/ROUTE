//! Helper `print_tier_segment_candidate_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_segment_candidate_summary(output: &Path, rows: &[TierSegmentCandidateRow]) {
    let mut by_tier = std::collections::BTreeMap::<&str, usize>::new();
    let mut bundles = std::collections::BTreeSet::<&str>::new();
    for row in rows {
        *by_tier.entry(row.tier.as_str()).or_default() += 1;
        bundles.insert(row.segment_bundle_id.as_str());
    }
    println!(
        "  wrote {} segment candidate rows across {} bundle candidates to {}",
        rows.len(),
        bundles.len(),
        output.display()
    );
    for (tier, count) in by_tier {
        println!("  {tier}: {count}");
    }
}

