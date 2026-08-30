//! Helper `print_tier_pavement_hpms_scope_broadening_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_tier_pavement_hpms_scope_broadening_summary(
    output: &Path,
    rows: &[TierPavementHpmsScopeBroadeningRow],
) {
    println!(
        "  wrote {} pavement HPMS scope-broadening rows to {}",
        rows.len(),
        output.display()
    );
    for row in rows {
        println!(
            "  {} systems {} routes {} members {}",
            row.state,
            row.broadened_functional_systems,
            row.source_needed_routes,
            row.source_needed_member_count
        );
    }
}
