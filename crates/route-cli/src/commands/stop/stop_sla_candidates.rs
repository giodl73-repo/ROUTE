//! `StopSlaCandidates` command handler extracted from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    input: PathBuf,
    ledger: PathBuf,
    cities: PathBuf,
    target_gap: f64,
    top: usize,
    candidates_per_gap: usize,
    output: Option<PathBuf>,
    gate: bool,
    gate_no_algorithmic: bool,
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    println!("route stop-sla-candidates");
    let sla_file = std::fs::File::open(&input)
        .with_context(|| format!("reading stop SLA surface {}", input.display()))?;
    let rows = parse_stop_sla_rows(sla_file)?;
    let ledger_file = std::fs::File::open(&ledger)
        .with_context(|| format!("opening stop candidate ledger {}", ledger.display()))?;
    let stop_rows = parse_stop_candidates(ledger_file)
        .with_context(|| format!("parsing stop candidate ledger {}", ledger.display()))?;
    let city_rows = load_city_rows(&cities).unwrap_or_else(|err| {
        eprintln!(
            "warning: could not load city seed list {}: {err}",
            cities.display()
        );
        Vec::new()
    });
    let recommendations =
        stop_sla_candidate_recommendations(&rows, &stop_rows, &city_rows, target_gap, top);
    print_stop_sla_candidate_recommendations(&recommendations, target_gap, candidates_per_gap);
    if let Some(output) = output {
        write_stop_sla_candidate_recommendations(&output, &recommendations)
            .with_context(|| format!("writing {}", output.display()))?;
        println!("  wrote candidate docket: {}", output.display());
    }
    if gate {
        let empty = recommendations
            .iter()
            .filter(|recommendation| recommendation.candidates.is_empty())
            .collect::<Vec<_>>();
        if empty.is_empty() {
            println!("stop SLA candidate gate: PASS");
        } else {
            println!("stop SLA candidate gate: FAIL");
            for rec in empty.iter().take(10) {
                println!("  - {} has no candidate", rec.gap.segment_id);
            }
            anyhow::bail!("stop SLA candidate gate failed");
        }
    }
    if gate_no_algorithmic {
        let algorithmic = recommendations
            .iter()
            .filter_map(|recommendation| {
                recommendation
                    .candidates
                    .first()
                    .filter(|candidate| candidate.source_type == "algorithmic-midpoint")
                    .map(|candidate| (&recommendation.gap.segment_id, candidate))
            })
            .collect::<Vec<_>>();
        if algorithmic.is_empty() {
            println!("stop SLA named-candidate gate: PASS");
        } else {
            println!("stop SLA named-candidate gate: FAIL");
            for (segment_id, candidate) in algorithmic.iter().take(10) {
                println!("  - {segment_id} falls back to {}", candidate.name);
            }
            anyhow::bail!("stop SLA named-candidate gate failed");
        }
    }

    Ok(())
}
