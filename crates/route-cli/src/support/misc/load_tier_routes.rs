//! Helper `load_tier_routes`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_tier_routes(path: &Path, tier: &str) -> Result<Vec<String>> {
    let mut rdr = csv::Reader::from_path(path)?;
    let headers = rdr.headers()?.clone();
    let tier_idx = headers
        .iter()
        .position(|value| value == "tier")
        .context("tier table missing tier column")?;
    let route_idx = headers
        .iter()
        .position(|value| value == "route")
        .context("tier table missing route column")?;
    let mut routes = Vec::new();
    for result in rdr.records() {
        let row = result?;
        if row
            .get(tier_idx)
            .unwrap_or("")
            .trim()
            .eq_ignore_ascii_case(tier)
        {
            let route = normalise_designation(row.get(route_idx).unwrap_or("").trim());
            if !route.is_empty() {
                routes.push(route);
            }
        }
    }
    routes.sort();
    routes.dedup();
    Ok(routes)
}
