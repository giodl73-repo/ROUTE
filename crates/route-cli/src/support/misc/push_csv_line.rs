//! Helper `push_csv_line`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn push_csv_line(csv: &mut String, cells: &[&str]) {
    for (idx, cell) in cells.iter().enumerate() {
        if idx > 0 {
            csv.push(',');
        }
        let needs_quotes = cell.contains(',') || cell.contains('"') || cell.contains('\n');
        if needs_quotes {
            csv.push('"');
            csv.push_str(&cell.replace('"', "\"\""));
            csv.push('"');
        } else {
            csv.push_str(cell);
        }
    }
    csv.push('\n');
}
