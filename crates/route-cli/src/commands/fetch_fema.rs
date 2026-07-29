//! `FetchFema` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    output: Option<PathBuf>
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let out = output.unwrap_or_else(|| PathBuf::from("data/cache/fema_sfha_counts.csv"));
    println!("route fetch-fema → {}", out.display());
    println!(
        "  source: FEMA NFHL ArcGIS REST — Layer 28 (Flood Hazard Zones / SFHA A-zones)"
    );
    println!(
        "  querying {} T1 corridor bounding boxes…",
        route_data::T1_BBOXES.len()
    );

    std::fs::create_dir_all(out.parent().unwrap_or(std::path::Path::new(".")))?;

    let results = route_data::fetch_all_sfha_counts(&out)?;

    let ok_count = results.iter().filter(|r| r.status == "ok").count();
    println!("\n  Results:");
    println!("  {:10}  {:>14}  {}", "Corridor", "SFHA Features", "Status");
    println!("  {}", "─".repeat(40));
    for r in &results {
        println!("  {:10}  {:>14}  {}", r.corridor, r.sfha_count, r.status);
    }
    println!(
        "\n  {}/{} corridors queried successfully",
        ok_count,
        results.len()
    );
    println!("  saved → {}", out.display());
    println!("  Use counts as D1 proxy: higher = more flood-exposed corridor.");
    println!("  Note: counts reflect SFHA polygons in the bounding box, not miles.");
    println!(
        "  Run `route score <corridor>` after this to see D1 update (manual join needed)."
    );
        
    Ok(())
}

