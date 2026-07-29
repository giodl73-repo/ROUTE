//! Helper `print_t1_sla_candidate_pair_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t1_sla_candidate_pair_summary(
    output: &Path,
    rows: &[T1SlaCandidatePairRow],
    selected_budget: usize,
) {
    let selected = rows.iter().filter(|row| row.portfolio_selected).count();
    let dropped = rows.iter().filter(|row| !row.portfolio_selected).count();
    let cutline = rows
        .iter()
        .find(|row| row.rank == selected_budget + 1)
        .map(|row| row.pair_id.as_str())
        .unwrap_or("n/a");
    println!(
        "  wrote {} ranked SLA candidate pairs to {}",
        rows.len(),
        output.display()
    );
    println!("  selected portfolio rows: {selected}/{selected_budget}");
    println!("  dropped candidate rows: {dropped}");
    println!("  first dropped by rank: {cutline}");
}

