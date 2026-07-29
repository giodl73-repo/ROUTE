//! Helper `print_score_table`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn print_score_table(
    designation: &str,
    scores: &route_score::DimensionScores,
    all_estimated: bool,
) {
    println!("\n┌─────────────────────────────────────────────────────────────────────┐");
    println!(
        "│  {} — Dimension Scores (rubric {}, conf {:.2}, score-conf {:.2})",
        designation,
        scores.rubric_version,
        scores.mean_confidence(),
        scores.score_weighted_confidence()
    );
    println!("├──────┬──────────────────────────────┬───────┬─────┬────────┬──────┤");
    println!("│ Dim  │ Name                         │ Score │ Est │ Quality│ Conf │");
    println!("├──────┼──────────────────────────────┼───────┼─────┼────────┼──────┤");

    let all = [
        &scores.a1, &scores.a2, &scores.a3, &scores.a4, &scores.a5, &scores.b1, &scores.b2,
        &scores.b3, &scores.b4, &scores.c1, &scores.c2, &scores.c3, &scores.c4, &scores.d1,
        &scores.d2, &scores.d3,
    ];

    for sd in all {
        let est = if sd.estimated || all_estimated {
            "†"
        } else {
            " "
        };
        println!(
            "│ {:4} │ {:<28} │ {:>5.1} │  {}  │ {:<6} │ {:>4.2} │",
            sd.dim.code(),
            sd.dim.name(),
            sd.score,
            est,
            sd.quality_label(),
            sd.confidence
        );
    }

    println!("├──────┴──────────────────────────────┼───────┼─────┴────────┴──────┤");
    println!(
        "│ Band A (Flow)                        │ {:>5.1} │                    │",
        scores.band_a()
    );
    println!(
        "│ Band B (Network)                     │ {:>5.1} │                    │",
        scores.band_b()
    );
    println!(
        "│ Band C (People)                      │ {:>5.1} │                    │",
        scores.band_c()
    );
    println!(
        "│ Band D (Future)                      │ {:>5.1} │                    │",
        scores.band_d()
    );
    println!(
        "│ TOTAL                                │ {:>5.1} │ /160               │",
        scores.total()
    );
    println!("└──────────────────────────────────────┴───────┴─────────────────────┘");
}

