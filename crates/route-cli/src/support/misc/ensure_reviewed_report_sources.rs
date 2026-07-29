//! Helper `ensure_reviewed_report_sources`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn ensure_reviewed_report_sources(
    output_path: &std::path::Path,
    cache_dir: &std::path::Path,
    repo_root: &std::path::Path,
    allow_partial: bool,
) -> Result<()> {
    if allow_partial || !output_path.exists() {
        return Ok(());
    }

    let existing = std::fs::read_to_string(output_path)
        .with_context(|| format!("reading {}", output_path.display()))?;
    if !existing
        .lines()
        .any(|line| line.trim() == "status: reviewed")
    {
        return Ok(());
    }

    let tiger_ready = cache_dir
        .join("tiger-primary-roads")
        .join("tl_2023_us_primaryroads.shp")
        .exists()
        || cache_dir.join("tl_2023_us_primaryroads.zip").exists();
    let gazetteer_ready = std::fs::read_dir(cache_dir).ok().is_some_and(|entries| {
        entries.filter_map(|entry| entry.ok()).any(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .ends_with("counties_national.txt")
        })
    });

    let mut missing = Vec::new();
    if !tiger_ready {
        missing.push("data/cache TIGER primary roads");
    }
    if !gazetteer_ready {
        missing.push("data/cache/*counties_national.txt");
    }

    let required_cache = [
        "hpms_2018.csv",
        "acs_county_pop_2022.csv",
        "acs_county_income_2022.csv",
        "rucc_2023.csv",
    ];
    for path in required_cache {
        if !cache_dir.join(path).exists() {
            missing.push(path);
        }
    }

    let required_repo = [
        "data/ports.csv",
        "data/intermodal_terminals.csv",
        "data/railroad_parallels.csv",
        "data/hazard_zones.csv",
    ];
    for path in required_repo {
        if !repo_root.join(path).exists() {
            missing.push(path);
        }
    }

    if missing.is_empty() {
        return Ok(());
    }

    anyhow::bail!(
        "refusing to overwrite reviewed corpus entry {} with incomplete sources; missing: {}. \
         Restore the source caches or pass --allow-partial to explicitly generate a degraded report.",
        output_path.display(),
        missing.join(", ")
    )
}

