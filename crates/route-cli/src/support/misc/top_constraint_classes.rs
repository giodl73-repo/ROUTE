//! Helper `top_constraint_classes`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn top_constraint_classes(class_counts: &std::collections::BTreeMap<String, usize>) -> String {
    let mut classes = class_counts.iter().collect::<Vec<_>>();
    classes.sort_by(|left, right| right.1.cmp(left.1).then_with(|| left.0.cmp(right.0)));
    classes
        .into_iter()
        .take(3)
        .map(|(class, _)| class.as_str())
        .collect::<Vec<_>>()
        .join("|")
}

