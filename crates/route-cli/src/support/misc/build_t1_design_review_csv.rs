//! Helper `build_t1_design_review_csv`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn build_t1_design_review_csv(rows: &[T1DesignReviewRow]) -> String {
    let mut csv = String::from(
        "route,selected,design_role,promise_count,selected_stop_count,top_city_stop_count,selector_reason,beck_action,beck_review_flag,overlap_corridors,design_status,next_design_action\n",
    );
    for row in rows {
        push_csv_line(
            &mut csv,
            &[
                &row.route,
                if row.selected { "true" } else { "false" },
                row.design_role,
                &row.promise_count.to_string(),
                &row.selected_stop_count.to_string(),
                &row.top_city_stop_count.to_string(),
                &row.selector_reason,
                &row.beck_action,
                &row.beck_review_flag,
                &row.overlap_corridors,
                row.design_status,
                row.next_design_action,
            ],
        );
    }
    csv
}

