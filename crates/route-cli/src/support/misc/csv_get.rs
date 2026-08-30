//! Helper `csv_get`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn csv_get(record: &csv::StringRecord, idx: usize) -> &str {
    record.get(idx).unwrap_or("")
}
