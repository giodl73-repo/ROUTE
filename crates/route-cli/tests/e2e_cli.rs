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
        "360",
    ]);
    let summary_stdout = String::from_utf8_lossy(&summary.stdout);
    assert!(summary_stdout.contains("route stop-sla-summary"));
    assert!(summary_stdout.contains("Recurring Segment"));
    assert!(summary_stdout.contains("stop SLA max-gap gate: PASS"));
}

#[test]
fn e2e_gates_map_atlas_and_l2_pressure_coverage() {
    let atlas = assert_success(&["map-atlas", "--gate"]);
    let atlas_stdout = String::from_utf8_lossy(&atlas.stdout);
    assert!(atlas_stdout.contains("route map-atlas"));
    assert!(atlas_stdout.contains("gate blockers: 0"));

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
