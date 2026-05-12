use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("route-cli lives under crates/route-cli")
        .to_path_buf()
}

fn route_cmd(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_route"))
        .args(args)
        .current_dir(repo_root())
        .output()
        .unwrap_or_else(|err| panic!("failed to run route {args:?}: {err}"))
}

fn assert_success(args: &[&str]) -> Output {
    let output = route_cmd(args);
    assert!(
        output.status.success(),
        "route {args:?} failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output
}

fn target_artifact(name: &str) -> PathBuf {
    let dir = repo_root().join("target").join("e2e");
    std::fs::create_dir_all(&dir).expect("create target/e2e");
    dir.join(name)
}

#[test]
fn e2e_generates_beck_t2_map_and_stop_sla_surface() {
    let map_out = target_artifact("beck-schematic-t2-e2e.png");
    let t2_only_map_out = target_artifact("beck-schematic-t2-only-e2e.png");
    let t2_diagnostics_out = target_artifact("beck-t2-diagnostics-e2e.csv");
    let t1_diagnostics_out = target_artifact("beck-t1-diagnostics-e2e.csv");
    let t1_selector_out = target_artifact("t1-line-selector-e2e.csv");
    let t2_standards_out = target_artifact("beck-t2-service-standards-e2e.csv");
    let t2_actions_out = target_artifact("beck-t2-qualification-actions-e2e.csv");
    let csv_out = target_artifact("beck-stop-sla-e2e.csv");

    assert_success(&[
        "map",
        "BECKT2",
        "--output",
        map_out.to_str().expect("utf-8 output path"),
    ]);
    assert!(
        map_out.metadata().expect("map output metadata").len() > 100_000,
        "Beck T2 PNG should be a rendered artifact"
    );
    assert_success(&[
        "map",
        "BECKT2ONLY",
        "--output",
        t2_only_map_out.to_str().expect("utf-8 output path"),
    ]);
    assert!(
        t2_only_map_out
            .metadata()
            .expect("T2-only map output metadata")
            .len()
            > 100_000,
        "Beck T2-only PNG should be a rendered artifact"
    );
    assert_success(&[
        "beck-t2-diagnostics",
        "--output",
        t2_diagnostics_out.to_str().expect("utf-8 output path"),
        "--gate",
    ]);
    let t2_diagnostics =
        std::fs::read_to_string(&t2_diagnostics_out).expect("read T2 diagnostics CSV");
    assert!(
        t2_diagnostics.starts_with("corridor,trunk,start_trunk,end_trunk,color_mode,service_class")
    );
    assert!(t2_diagnostics.contains("split-parent"));
    assert!(t2_diagnostics.contains("compact-service"));
    assert!(t2_diagnostics.contains("dense-transfer-review"));

    assert_success(&[
        "beck-t1-diagnostics",
        "--output",
        t1_diagnostics_out.to_str().expect("utf-8 output path"),
    ]);
    let t1_diagnostics =
        std::fs::read_to_string(&t1_diagnostics_out).expect("read T1 diagnostics CSV");
    assert!(t1_diagnostics.starts_with("corridor,endpoint_start,endpoint_end"));
    assert!(t1_diagnostics.contains("overlap-review"));

    assert_success(&[
        "t1-line-selector",
        "--output",
        t1_selector_out.to_str().expect("utf-8 output path"),
        "--gate",
    ]);
    let t1_selector = std::fs::read_to_string(&t1_selector_out).expect("read T1 selector CSV");
    assert!(t1_selector.starts_with("route,tier,score,rank,selected"));
    assert!(t1_selector.contains("sla-required-budget-fit"));
    assert!(t1_selector.contains("NYC-LA-48"));

    assert_success(&[
        "beck-t2-service-standards",
        "--output",
        t2_standards_out.to_str().expect("utf-8 output path"),
        "--gate",
    ]);
    let t2_standards = std::fs::read_to_string(&t2_standards_out).expect("read T2 standards CSV");
    assert!(t2_standards.starts_with("service_class,definition,min_schematic_px"));
    assert!(t2_standards.contains("transfer-spine"));
    assert!(t2_standards.contains("long-connector"));

    assert_success(&[
        "beck-t2-qualification-actions",
        "--output",
        t2_actions_out.to_str().expect("utf-8 output path"),
        "--gate",
    ]);
    let t2_actions = std::fs::read_to_string(&t2_actions_out).expect("read T2 actions CSV");
    assert!(t2_actions.starts_with("service_action,definition,required_evidence"));
    assert!(t2_actions.contains("keep-primary-review"));
    assert!(t2_actions.contains("demote-review"));

    assert_success(&[
        "stop-sla-surface",
        "--output",
        csv_out.to_str().expect("utf-8 output path"),
    ]);
    let csv = std::fs::read_to_string(&csv_out).expect("read SLA CSV");
    assert!(csv.starts_with("origin_id,origin_label,origin_tier"));
    assert!(csv.contains("freight_sla_window"));
    assert!(csv.contains("heuristic-planning"));

    let summary = assert_success(&[
        "stop-sla-summary",
        "--input",
        csv_out.to_str().expect("utf-8 output path"),
        "--top",
        "4",
        "--gate-max-gap",
        "250",
    ]);
    let summary_stdout = String::from_utf8_lossy(&summary.stdout);
    assert!(summary_stdout.contains("route stop-sla-summary"));
    assert!(summary_stdout.contains("Recurring Segment"));
    assert!(summary_stdout.contains("stop SLA max-gap gate: PASS"));

    let candidates = assert_success(&[
        "stop-sla-candidates",
        "--input",
        csv_out.to_str().expect("utf-8 output path"),
        "--output",
        target_artifact("beck-stop-sla-candidates-e2e.csv")
            .to_str()
            .expect("utf-8 candidate output path"),
        "--target-gap",
        "250",
        "--top",
        "3",
        "--gate",
        "--gate-no-algorithmic",
    ]);
    let candidates_stdout = String::from_utf8_lossy(&candidates.stdout);
    assert!(candidates_stdout.contains("route stop-sla-candidates"));
    assert!(candidates_stdout.contains("target gap"));
    assert!(candidates_stdout.contains("stop SLA candidate gate: PASS"));
    assert!(candidates_stdout.contains("stop SLA named-candidate gate: PASS"));

    let promotions = assert_success(&[
        "stop-sla-promotions",
        "--input",
        target_artifact("beck-stop-sla-candidates-e2e.csv")
            .to_str()
            .expect("utf-8 candidate input path"),
        "--output",
        target_artifact("beck-stop-sla-promotions-e2e.csv")
            .to_str()
            .expect("utf-8 promotion output path"),
        "--gate",
    ]);
    let promotions_stdout = String::from_utf8_lossy(&promotions.stdout);
    assert!(promotions_stdout.contains("route stop-sla-promotions"));
    assert!(promotions_stdout.contains("stop SLA promotion gate: PASS"));
}

#[test]
fn e2e_gates_map_atlas_and_l2_pressure_coverage() {
    let atlas = assert_success(&["map-atlas", "--gate"]);
    let atlas_stdout = String::from_utf8_lossy(&atlas.stdout);
    assert!(atlas_stdout.contains("route map-atlas"));
    assert!(atlas_stdout.contains("gate blockers: 0"));

    let t1_design = assert_success(&["t1-design-review", "--gate"]);
    let t1_design_stdout = String::from_utf8_lossy(&t1_design.stdout);
    assert!(t1_design_stdout.contains("route t1-design-review"));
    assert!(t1_design_stdout.contains("T1 design review gate: PASS"));

    let t2_overlays = assert_success(&["game", "t2-overlays", "--gate"]);
    let t2_overlays_stdout = String::from_utf8_lossy(&t2_overlays.stdout);
    assert!(t2_overlays_stdout.contains("route game t2-overlays"));
    assert!(t2_overlays_stdout.contains("T2 service overlay gate: PASS"));

    let t2_hooks = assert_success(&["game", "t2-hooks", "--gate"]);
    let t2_hooks_stdout = String::from_utf8_lossy(&t2_hooks.stdout);
    assert!(t2_hooks_stdout.contains("route game t2-hooks"));
    assert!(t2_hooks_stdout.contains("T2 scenario hook gate: PASS"));

    let moments = assert_success(&["significant-moments", "--gate"]);
    let moments_stdout = String::from_utf8_lossy(&moments.stdout);
    assert!(moments_stdout.contains("route significant-moments"));
    assert!(moments_stdout.contains("Significant moments gate: PASS"));

    let pressure = assert_success(&[
        "pressure-scenarios",
        "--coverage",
        "--gate-l2",
        "--gate-coverage",
    ]);
    let pressure_stdout = String::from_utf8_lossy(&pressure.stdout);
    assert!(pressure_stdout.contains("L2 scenario gate: PASS"));
    assert!(pressure_stdout.contains("Pressure standard coverage gate: PASS"));
}

#[test]
fn e2e_reports_current_t1_stop_coverage_blockers() {
    let output = assert_success(&["stop-coverage", "--tier", "T1"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(stdout.contains("route stop-coverage --tier T1"));
    assert!(stdout.contains("passing stop plans"));
    assert!(stdout.contains("blockers"));
    assert!(
        stdout.contains("I395"),
        "known T1 stop-coverage blocker should stay visible until resolved"
    );
}
