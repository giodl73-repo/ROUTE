//! Helper `print_t3_zone_render_board_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t3_zone_render_board_summary(output: &Path, rows: &[T3ZoneRenderBoardRow]) {
    let mut by_layer = std::collections::BTreeMap::<&str, usize>::new();
    for row in rows {
        *by_layer.entry(row.board_layer.as_str()).or_default() += 1;
    }
    println!(
        "  wrote {} T3 zone render board rows to {}",
        rows.len(),
        output.display()
    );
    for (layer, count) in by_layer {
        println!("  {layer}: {count}");
    }
}

