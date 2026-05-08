/// Coverage analysis — how close is any population point to the highway network?
///
/// Two modes:
///   1. County centroid (preferred): uses Census Gazetteer county centroids,
///      optionally population-weighted. No ocean problem. Actionable county-level gaps.
///   2. Geographic grid (fast proxy): 10-mile grid over bounding box.
///      Includes ocean cells; use only as rough estimate.
use crate::graph::HighwayGraph;
use petgraph::graph::NodeIndex;
use std::collections::HashSet;

/// Population-weighted coverage result (county centroid mode).
#[derive(Debug)]
pub struct PopCoverageResult {
    pub total_counties: usize,
    pub total_population: u64,
    pub total_land_sqmi: f64,
    // By distance threshold
    pub counties_within_20mi: usize,
    pub counties_within_30mi: usize,
    pub counties_within_50mi: usize,
    pub pop_within_20mi: u64,
    pub pop_within_30mi: u64,
    pub pop_within_50mi: u64,
    pub land_within_20mi: f64,
    pub land_within_30mi: f64,
    pub land_within_50mi: f64,
    // Gap counties (exceed threshold)
    pub gap_counties: Vec<CountyGap>,
    pub max_gap_miles: f64,
}

#[derive(Debug, Clone)]
pub struct CountyGap {
    pub geoid: String,
    pub name: String,
    pub state: String,
    pub lat: f64,
    pub lon: f64,
    pub nearest_miles: f64,
    pub population: u64,
    pub aland_sqmi: f64,
}

/// Compute population-weighted coverage using county centroids.
pub fn compute_pop_coverage(
    g: &HighwayGraph,
    counties: &[route_data::CountyCentroid],
    tier_filter: Option<&[&str]>,
    threshold_miles: f64,
) -> PopCoverageResult {
    let interchange_coords: Vec<(f64, f64)> = g.graph.node_indices()
        .filter(|&ni| {
            let node = &g.graph[ni];
            if !node.is_interchange { return false; }
            if let Some(filter) = tier_filter {
                g.graph.edges(ni).any(|er| filter.iter().any(|&f| er.weight().route_id == f))
            } else { true }
        })
        .map(|ni| { let c = g.graph[ni].coord; (c.x, c.y) })
        .collect();

    let mut result = PopCoverageResult {
        total_counties: counties.len(),
        total_population: counties.iter().map(|c| c.population).sum(),
        total_land_sqmi: counties.iter().map(|c| c.aland_sqmi).sum(),
        counties_within_20mi: 0, counties_within_30mi: 0, counties_within_50mi: 0,
        pop_within_20mi: 0, pop_within_30mi: 0, pop_within_50mi: 0,
        land_within_20mi: 0.0, land_within_30mi: 0.0, land_within_50mi: 0.0,
        gap_counties: Vec::new(),
        max_gap_miles: 0.0,
    };

    for county in counties {
        let nearest = find_nearest_miles(&interchange_coords, county.lat, county.lon);
        if nearest > result.max_gap_miles { result.max_gap_miles = nearest; }

        if nearest <= 20.0 {
            result.counties_within_20mi += 1;
            result.pop_within_20mi += county.population;
            result.land_within_20mi += county.aland_sqmi;
        }
        if nearest <= 30.0 {
            result.counties_within_30mi += 1;
            result.pop_within_30mi += county.population;
            result.land_within_30mi += county.aland_sqmi;
        }
        if nearest <= 50.0 {
            result.counties_within_50mi += 1;
            result.pop_within_50mi += county.population;
            result.land_within_50mi += county.aland_sqmi;
        }
        if nearest > threshold_miles {
            result.gap_counties.push(CountyGap {
                geoid: county.geoid.clone(),
                name: county.name.clone(),
                state: county.state.clone(),
                lat: county.lat,
                lon: county.lon,
                nearest_miles: nearest,
                population: county.population,
                aland_sqmi: county.aland_sqmi,
            });
        }
    }

    result.gap_counties.sort_by(|a, b| b.nearest_miles.partial_cmp(&a.nearest_miles).unwrap());
    result
}

/// Result of a geometric grid coverage analysis run.
#[derive(Debug)]
pub struct CoverageResult {
    pub total_cells: usize,
    pub cells_within_20mi: usize,
    pub cells_within_30mi: usize,
    pub cells_within_50mi: usize,
    /// Cells that exceed 30-mile threshold — these identify coverage gaps
    pub gap_cells: Vec<GapCell>,
    pub pct_within_20mi: f64,
    pub pct_within_30mi: f64,
    pub pct_within_50mi: f64,
    /// Max distance to nearest on-ramp in the continental US (miles)
    pub max_gap_miles: f64,
}

/// A grid cell that exceeds the coverage threshold.
#[derive(Debug, Clone)]
pub struct GapCell {
    /// Cell center latitude
    pub lat: f64,
    /// Cell center longitude
    pub lon: f64,
    /// Distance to nearest interchange (miles)
    pub nearest_miles: f64,
    /// State (approximate, from latitude/longitude)
    pub approx_state: String,
}

/// Compute highway network coverage over the continental US.
///
/// `tier_filter`: if provided, only count interchanges belonging to corridors
/// in the given set (e.g., ["I80","I90"] for T1 coverage only).
/// Pass None to count all interchanges.
///
/// `grid_miles`: grid resolution in miles (10.0 is fast, 5.0 is more precise).
/// `threshold_miles`: the coverage threshold to report gap cells for (default 30.0).
pub fn compute_coverage(
    g: &HighwayGraph,
    tier_filter: Option<&[&str]>,
    grid_miles: f64,
    threshold_miles: f64,
) -> CoverageResult {
    // Continental US bounding box (land only — excludes Alaska/Hawaii)
    // NOTE: This bounding box includes ocean cells along all coasts and
    // Canadian/Mexican territory in the bounding box. Results should be
    // interpreted as a geometric proxy only. For population-weighted analysis,
    // use Census county centroid mode (planned for B.1 paper computation).
    const LAT_MIN: f64 = 25.0;  // South Florida (excludes most ocean south of FL)
    const LAT_MAX: f64 = 49.0;  // Northern border
    const LON_MIN: f64 = -124.8; // Pacific coast
    const LON_MAX: f64 = -67.5;  // Eastern Maine (excludes most Atlantic Ocean)

    // Degree equivalents of grid_miles
    const MILES_PER_DEG_LAT: f64 = 69.0;
    let grid_deg_lat = grid_miles / MILES_PER_DEG_LAT;

    // Collect interchange node positions, filtered by tier if requested
    let interchange_coords: Vec<(f64, f64)> = g.graph.node_indices()
        .filter(|&ni| {
            let node = &g.graph[ni];
            if !node.is_interchange {
                return false;
            }
            // Apply tier filter if provided
            if let Some(filter) = tier_filter {
                // Check if any edge at this node belongs to a filtered route
                g.graph.edges(ni).any(|er| {
                    filter.iter().any(|&f| er.weight().route_id == f)
                })
            } else {
                true
            }
        })
        .map(|ni| {
            let c = g.graph[ni].coord;
            (c.x, c.y) // lon, lat
        })
        .collect();

    if interchange_coords.is_empty() {
        return CoverageResult {
            total_cells: 0,
            cells_within_20mi: 0,
            cells_within_30mi: 0,
            cells_within_50mi: 0,
            gap_cells: vec![],
            pct_within_20mi: 0.0,
            pct_within_30mi: 0.0,
            pct_within_50mi: 0.0,
            max_gap_miles: 0.0,
        };
    }

    let mut total = 0usize;
    let mut within_20 = 0usize;
    let mut within_30 = 0usize;
    let mut within_50 = 0usize;
    let mut gap_cells = Vec::new();
    let mut max_gap = 0.0f64;

    let mut lat = LAT_MIN;
    while lat <= LAT_MAX {
        let grid_deg_lon = grid_miles / (MILES_PER_DEG_LAT * lat.to_radians().cos());
        let mut lon = LON_MIN;
        while lon <= LON_MAX {
            // Skip cells that are clearly ocean or non-US land
            // (nearest interstate >400 miles = definitely ocean or outside US)
            let nearest = find_nearest_miles(&interchange_coords, lat, lon);
            if nearest > 400.0 {
                lon += grid_deg_lon;
                continue;
            }

            total += 1;

            if nearest <= 20.0 { within_20 += 1; }
            if nearest <= 30.0 { within_30 += 1; }
            if nearest <= 50.0 { within_50 += 1; }
            if nearest > max_gap { max_gap = nearest; }

            if nearest > threshold_miles {
                gap_cells.push(GapCell {
                    lat,
                    lon,
                    nearest_miles: nearest,
                    approx_state: approx_state(lat, lon),
                });
            }

            lon += grid_deg_lon;
        }
        lat += grid_deg_lat;
    }

    let t = total as f64;
    CoverageResult {
        total_cells: total,
        cells_within_20mi: within_20,
        cells_within_30mi: within_30,
        cells_within_50mi: within_50,
        gap_cells,
        pct_within_20mi: within_20 as f64 / t * 100.0,
        pct_within_30mi: within_30 as f64 / t * 100.0,
        pct_within_50mi: within_50 as f64 / t * 100.0,
        max_gap_miles: max_gap,
    }
}

/// Haversine distance in miles between two lat/lon points.
fn haversine_miles(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 3958.8; // Earth radius miles
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();
    let a = (dlat / 2.0).sin().powi(2)
        + lat1.to_radians().cos() * lat2.to_radians().cos()
        * (dlon / 2.0).sin().powi(2);
    2.0 * R * a.sqrt().asin()
}

/// Find distance (miles) to nearest interchange from a grid point.
fn find_nearest_miles(coords: &[(f64, f64)], lat: f64, lon: f64) -> f64 {
    coords.iter()
        .map(|&(ilon, ilat)| haversine_miles(lat, lon, ilat, ilon))
        .fold(f64::MAX, f64::min)
}

/// Compute population within 50 miles of a specific corridor's interchange nodes.
///
/// Uses the same haversine logic as `compute_pop_coverage` but restricted to
/// interchange nodes belonging to `route_id`. Counties are deduplicated by GEOID
/// so each county is counted once regardless of how many nodes are nearby.
///
/// Returns `(pop_within_50mi, rural_pop_within_50mi)`.
/// Rural = counties whose RUCC code ≥ 4; when RUCC is 0 (not assigned), treated as urban.
pub fn corridor_pop_within_50mi(
    g: &HighwayGraph,
    route_id: &str,
    counties: &[route_data::CountyCentroid],
) -> (u64, u64) {
    // Collect interchange node coordinates for this corridor only
    let interchange_coords: Vec<(f64, f64)> = g.graph.node_indices()
        .filter(|&ni| {
            let node = &g.graph[ni];
            if !node.is_interchange { return false; }
            // Keep node if any adjacent edge belongs to this route
            g.graph.edges(ni).any(|er| er.weight().route_id == route_id)
        })
        .map(|ni| { let c = g.graph[ni].coord; (c.x, c.y) })
        .collect();

    if interchange_coords.is_empty() {
        return (0, 0);
    }

    let mut seen: HashSet<&str> = HashSet::new();
    let mut total_pop: u64 = 0;
    let mut rural_pop: u64 = 0;

    for county in counties {
        if seen.contains(county.geoid.as_str()) {
            continue;
        }
        let nearest = find_nearest_miles(&interchange_coords, county.lat, county.lon);
        if nearest <= 50.0 {
            seen.insert(&county.geoid);
            total_pop += county.population;
            if county.rucc >= 4 {
                rural_pop += county.population;
            }
        }
    }

    (total_pop, rural_pop)
}

/// Return references to all counties within 50 miles of a corridor's interchange nodes.
/// Used for C3 income-weighted scoring. Deduplicates by GEOID.
pub fn counties_within_50mi<'a>(
    g: &HighwayGraph,
    route_id: &str,
    counties: &'a [route_data::CountyCentroid],
) -> Vec<&'a route_data::CountyCentroid> {
    let interchange_coords: Vec<(f64, f64)> = g.graph.node_indices()
        .filter(|&ni| {
            let node = &g.graph[ni];
            if !node.is_interchange { return false; }
            g.graph.edges(ni).any(|er| er.weight().route_id == route_id)
        })
        .map(|ni| { let c = g.graph[ni].coord; (c.x, c.y) })
        .collect();

    if interchange_coords.is_empty() {
        return Vec::new();
    }

    let mut seen: HashSet<&str> = HashSet::new();
    let mut result = Vec::new();
    for county in counties {
        if seen.contains(county.geoid.as_str()) { continue; }
        if find_nearest_miles(&interchange_coords, county.lat, county.lon) <= 50.0 {
            seen.insert(&county.geoid);
            result.push(county);
        }
    }
    result
}

/// Very rough state assignment from lat/lon (bounding box approximation).
fn approx_state(lat: f64, lon: f64) -> String {
    // Simplified bounding boxes for the most gap-prone states
    match (lat, lon) {
        (l, lo) if l > 46.0 && lo < -104.0 => "MT".into(),
        (l, lo) if l > 45.0 && lo < -100.0 && lo > -104.0 => "ND".into(),
        (l, lo) if l > 43.0 && lo < -100.0 && lo > -104.0 => "SD".into(),
        (l, lo) if l > 41.0 && l < 43.0 && lo < -98.0 => "NE".into(),
        (l, lo) if l > 36.0 && l < 42.0 && lo > -109.0 && lo < -102.0 => "CO/WY".into(),
        (l, lo) if l > 36.0 && lo < -114.0 => "NV/ID".into(),
        (l, lo) if l < 32.0 && lo > -93.0 && lo < -88.0 => "MS".into(),
        (l, lo) if l > 37.0 && lo > -83.0 && lo < -77.0 => "WV/VA".into(),
        _ => "—".into(),
    }
}
