//! Helper `print_release_manifest`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_release_manifest(path: &Path, rows: &[ReleaseManifestRow], blockers: bool, details: bool) {
    let failures = release_manifest_gate_failures(rows);
    let failure_paths = failures
        .iter()
        .filter_map(|failure| failure.split_whitespace().next())
        .collect::<std::collections::HashSet<_>>();
    let filtered = if blockers {
        rows.iter()
            .filter(|row| failure_paths.contains(row.artifact_path.as_str()))
            .collect::<Vec<_>>()
    } else {
        rows.iter().collect::<Vec<_>>()
    };

    let mut by_public_status = std::collections::BTreeMap::<String, usize>::new();
    let mut by_release_status = std::collections::BTreeMap::<String, usize>::new();
    for row in rows {
        *by_public_status
            .entry(row.public_status.clone())
            .or_default() += 1;
        *by_release_status
            .entry(row.release_status.clone())
            .or_default() += 1;
    }

    println!("route release-manifest");
    println!("  manifest: {}", path.display());
    println!(
        "  artifacts: {} shown / {} total",
        filtered.len(),
        rows.len()
    );
    println!("  release: {}", format_count_map(&by_release_status));
    println!("  public: {}", format_count_map(&by_public_status));
    println!("  gate blockers: {}", failures.len());
    println!();
    println!(
        "{:<48} {:<18} {:<8} {:<18} {}",
        "Artifact", "Class", "Owner", "Public", "Verification"
    );
    println!("{}", "-".repeat(128));
    for row in filtered {
        println!(
            "{:<48} {:<18} {:<8} {:<18} {}",
            truncate_for_table(&row.artifact_path, 48),
            truncate_for_table(&row.artifact_class, 18),
            row.owner_milepost,
            row.public_status,
            truncate_for_table(&row.verification_command, 36),
        );
        if details {
            println!("  release: {}", row.release_status);
            println!("  notes: {}", row.notes);
        }
    }
}

