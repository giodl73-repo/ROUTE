//! Helper `print_t2_game_ops_binding_intake_summary`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_t2_game_ops_binding_intake_summary(output: &Path, rows: &[T2GameOpsBindingIntakeRow]) {
    println!(
        "  wrote {} T2 game/ops binding intake rows to {}",
        rows.len(),
        output.display()
    );
}

