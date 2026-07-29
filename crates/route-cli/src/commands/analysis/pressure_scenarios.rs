//! `PressureScenarios` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    ledger: PathBuf,
    blockers: bool,
    details: bool,
    coverage: bool,
    standards_ledger: PathBuf,
    gate_l2: bool,
    gate_readiness: bool,
    gate_coverage: bool
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let rows = load_pressure_scenarios(&ledger).with_context(|| {
        format!("loading pressure scenario ledger {}", ledger.display())
    })?;
    print_pressure_scenarios(&rows, blockers, details);
    let standards_rows = if coverage || gate_coverage {
        Some(
            load_standards_proof_ledger(&standards_ledger).with_context(|| {
                format!(
                    "loading standards proof ledger {}",
                    standards_ledger.display()
                )
            })?,
        )
    } else {
        None
    };
    if let Some(standards_rows) = standards_rows.as_ref() {
        print_pressure_standard_coverage(standards_rows, &rows);
    }

    if gate_l2 {
        let failures = pressure_scenario_gate_failures(&rows);
        let missing_adversity = pressure_scenario_missing_required_adversity(&rows);
        if !failures.is_empty() || !missing_adversity.is_empty() {
            println!();
            println!("L2 scenario gate: FAIL");
            println!(
                "  {} scenario rows still lack bounded proof contracts.",
                failures.len()
            );
            if !missing_adversity.is_empty() {
                println!(
                    "  missing required adversity classes: {}",
                    missing_adversity.join(", ")
                );
            }
            for row in failures.iter().take(10) {
                println!(
                    "  - {} [{}]: {}",
                    row.scenario_id, row.current_status, row.blocking_gap
                );
            }
            anyhow::bail!("pressure scenario gate failed");
        }
        println!();
        println!("L2 scenario gate: PASS");
    }
    if gate_readiness {
        let failures = pressure_scenario_readiness_gate_failures(&rows);
        if !failures.is_empty() {
            println!();
            println!("L2 scenario readiness gate: FAIL");
            println!(
                "  {} scenario rows are not executable pressure tests yet.",
                failures.len()
            );
            for row in failures.iter().take(10) {
                println!(
                    "  - {} [{}]: {}",
                    row.scenario_id, row.current_status, row.next_evidence_step
                );
            }
            anyhow::bail!("pressure scenario readiness gate failed");
        }
        println!();
        println!("L2 scenario readiness gate: PASS");
    }
    if let Some(standards_rows) = standards_rows.as_ref() {
        if gate_coverage {
            let failures = pressure_standard_coverage_failures(standards_rows, &rows);
            let unknown = pressure_scenario_unknown_standard_refs(standards_rows, &rows);
            if !failures.is_empty() || !unknown.is_empty() {
                println!();
                println!("Pressure standard coverage gate: FAIL");
                if !failures.is_empty() {
                    println!(
                        "  {} high-stakes T1 standards lack scenario hooks.",
                        failures.len()
                    );
                    for row in failures.iter().take(10) {
                        println!(
                            "  - {} [{} {}]: {}",
                            row.standard_id,
                            row.tier,
                            row.standard_family,
                            row.primary_stressor
                        );
                    }
                }
                if !unknown.is_empty() {
                    println!("  unknown scenario standard refs: {}", unknown.join(", "));
                }
                anyhow::bail!("pressure standard coverage gate failed");
            }
            println!();
            println!("Pressure standard coverage gate: PASS");
        }
    }
        
    Ok(())
}

