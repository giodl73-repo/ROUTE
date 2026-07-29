//! Helper `bundle_qualification_effects_by_route`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn bundle_qualification_effects_by_route(
    bundle_rows: &[NationalSegmentBundleRow],
) -> std::collections::BTreeMap<String, String> {
    let mut effects_by_route =
        std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();
    for row in bundle_rows {
        if row.qualification_effects.trim().is_empty() {
            continue;
        }
        for route in semicolon_values(&row.route_labels) {
            insert_pipe_values(
                effects_by_route
                    .entry(canonical_route_key(&route))
                    .or_default(),
                &row.qualification_effects,
            );
        }
    }
    effects_by_route
        .into_iter()
        .map(|(route, effects)| (route, join_pipe_set(&effects)))
        .collect()
}

