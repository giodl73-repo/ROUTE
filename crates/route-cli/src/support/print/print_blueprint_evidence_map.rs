//! Helper `print_blueprint_evidence_map`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_blueprint_evidence_map(rows: &[BlueprintEvidenceRow], blockers: bool, details: bool) {
    let filtered = if blockers {
        rows.iter()
            .filter(|row| {
                matches!(
                    row.blueprint_claim_status
                        .trim()
                        .to_ascii_lowercase()
                        .as_str(),
                    "held" | "downgraded" | "planned"
                )
            })
            .collect::<Vec<_>>()
    } else {
        rows.iter().collect::<Vec<_>>()
    };

    let mut by_status: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    for row in rows {
        *by_status
            .entry(row.blueprint_claim_status.clone())
            .or_insert(0) += 1;
    }

    println!("route blueprint-evidence");
    println!("  rows: {} shown / {} total", filtered.len(), rows.len());
    println!("  claim status: {}", format_count_map(&by_status));
    println!();
    println!(
        "{:<24} {:<22} {:<10} {:<12} {}",
        "Package", "Standard", "Proof", "Claim", "Promotion rule"
    );
    println!("{}", "-".repeat(126));
    for row in filtered {
        println!(
            "{:<24} {:<22} {:<10} {:<12} {}",
            row.package_id,
            row.standard_id,
            row.proof_evidence_level,
            row.blueprint_claim_status,
            row.promotion_rule
        );
        if details {
            println!("  artifact: {}", row.proof_artifact);
            println!("  forum_hold: {}", row.forum_hold);
            println!("  gap: {}", row.blocking_gap);
            println!("  next: {}", row.required_next_evidence);
        }
    }
}

