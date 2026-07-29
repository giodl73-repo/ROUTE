//! `FetchFemaD1` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

            println!("route fetch-fema-d1 — FEMA NFHL D1 data via per-state small bboxes");
            println!("  Querying Layer 28 (Flood Hazard Zones) in 1°×1° tiles to avoid 504...");

            // State bounding boxes (1°×1° tiles covering major flood-exposed corridors)
            // Focus on Gulf Coast (I-10 LA/TX), Atlantic Coast (I-95), Mississippi Valley
            let state_tiles: Vec<(String, f64, f64, f64, f64)> = vec![
                ("LA-Gulf".to_string(), -93.5, 29.0, -92.5, 30.0),
                ("LA-Gulf2".to_string(), -92.5, 29.0, -91.5, 30.0),
                ("LA-Gulf3".to_string(), -91.5, 29.0, -90.5, 30.0),
                ("LA-Gulf4".to_string(), -90.5, 29.0, -89.5, 30.0),
                ("TX-Gulf".to_string(), -95.5, 29.0, -94.5, 30.0),
                ("TX-Gulf2".to_string(), -94.5, 29.0, -93.5, 30.0),
                ("FL-Gulf".to_string(), -87.5, 30.0, -86.5, 31.0),
                ("FL-SE".to_string(), -81.0, 25.5, -80.0, 26.5),
                ("FL-Atlantic".to_string(), -80.5, 26.5, -79.5, 27.5),
                ("NC-coast".to_string(), -77.5, 34.5, -76.5, 35.5),
                ("VA-coast".to_string(), -76.5, 36.5, -75.5, 37.5),
                ("NJ-coast".to_string(), -74.5, 39.5, -73.5, 40.5),
                ("MS-valley".to_string(), -91.0, 32.0, -90.0, 33.0),
                ("AR-flood".to_string(), -91.5, 33.5, -90.5, 34.5),
            ];
            let fema_url =
                "https://hazards.fema.gov/arcgis/rest/services/public/NFHL/MapServer/28/query";

            let mut results: Vec<(String, u32, String)> = Vec::new();
            for (name, xmin, ymin, xmax, ymax) in &state_tiles {
                let qs = format!(
                    "where=FLD_ZONE+LIKE+%27A%25%27&geometry={},{},{},{}&geometryType=esriGeometryEnvelope&spatialRel=esriSpatialRelIntersects&returnCountOnly=true&f=json",
                    xmin, ymin, xmax, ymax
                );
                let url = format!("{fema_url}?{qs}");
                // Use route-data's reqwest client pattern
                match route_data::fetch_fema_count(&url) {
                    Ok(count) => {
                        println!("  {name}: {count} SFHA features");
                        results.push((name.to_string(), count, "ok".to_string()));
                    }
                    Err(e) => {
                        println!("  {name}: FAILED — {e}");
                        results.push((name.to_string(), 0, format!("error: {e}")));
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(800));
            }

            // Write results only after the fetch loop completes.
            let out = PathBuf::from("data/cache/fema_sfha_tile_counts.csv");
            let tmp = temp_path_for_atomic_write(&out);
            let mut wtr = csv::Writer::from_path(&tmp)?;
            wtr.write_record([
                "tile",
                "xmin",
                "ymin",
                "xmax",
                "ymax",
                "sfha_count",
                "status",
            ])?;
            for (i, (name, count, status)) in results.iter().enumerate() {
                let t = &state_tiles[i];
                wtr.write_record(&[
                    name,
                    &t.1.to_string(),
                    &t.2.to_string(),
                    &t.3.to_string(),
                    &t.4.to_string(),
                    &count.to_string(),
                    status,
                ])?;
            }
            wtr.flush()?;
            drop(wtr);
            replace_with_atomic_write(&tmp, &out)?;
            println!("\n  Saved → {}", out.display());
            let total: u32 = results.iter().map(|(_, count, _)| count).sum();
            println!("  Total SFHA features across flood-exposed tiles: {total}");
            println!("  Next: wire tile counts into corridor D1 scoring via bbox intersection");
        
    Ok(())
}

