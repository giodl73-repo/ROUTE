//! `T1StopSelector` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    selector: PathBuf,
    stop_candidates: PathBuf,
    output: PathBuf,
    target_regions: usize,
    gate: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route t1-stop-selector");
            let selector_rows = load_t1_line_selector(&selector)
                .with_context(|| format!("loading {}", selector.display()))?;
            let stop_file = std::fs::File::open(&stop_candidates)
                .with_context(|| format!("opening {}", stop_candidates.display()))?;
            let stop_rows = parse_stop_candidates(stop_file)
                .with_context(|| format!("parsing {}", stop_candidates.display()))?;
            let rows = t1_stop_selector_rows(&selector_rows, &stop_rows, target_regions)?;
            write_t1_stop_selector(&output, &rows)
                .with_context(|| format!("writing {}", output.display()))?;
            print_t1_stop_selector_summary(&output, &rows);

            if gate {
                let failures = t1_stop_selector_gate_failures(&rows);
                if !failures.is_empty() {
                    println!();
                    println!("T1 stop selector gate: FAIL");
                    for failure in failures.iter().take(20) {
                        println!("  - {failure}");
                    }
                    anyhow::bail!("T1 stop selector gate failed");
                }
                println!();
                println!("T1 stop selector gate: PASS");
            }
        
    Ok(())
}

