use geo_types::{Coord, LineString};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NhsError {
    #[error("shapefile read error: {0}")]
    Shapefile(String),
    #[error("missing field '{0}' in NHS shapefile")]
    MissingField(&'static str),
    #[error("unsupported geometry at record {0}: expected Polyline")]
    UnsupportedGeometry(usize),
}

/// A single segment record from the FHWA National Highway System shapefile.
/// One NHS shapefile record = one homogeneous segment (same route, state, NHS type).
#[derive(Debug, Clone)]
pub struct NhsSegment {
    /// Route identifier as in NHS shapefile, e.g. "I80", "I95"
    pub route_id: String,
    /// Two-letter state code
    pub state: String,
    /// NHS type code (1=NHS, 2=Interstate, 3=Intermodal connector, etc.)
    pub nhs_type: u8,
    /// Segment length in miles (from MILES field)
    pub length_miles: f64,
    /// Geometry in EPSG:4269 (NAD83 geographic)
    pub geometry: LineString<f64>,
}

/// Read road segments from a .shp file.
/// Handles two formats:
///   - FHWA NHS shapefile: fields ROUTE_ID, STATE_CODE, NHS_TYPE, MILES
///   - TIGER Primary Roads: fields FULLNAME, RTTYP, LINEARID (no state, no miles)
///
/// TIGER RTTYP codes: 'I' = Interstate, 'U' = US Route, 'S' = State, 'C' = County, etc.
/// When `all_nhs = false`, only RTTYP='I' (interstates) are returned.
pub fn read_nhs_shapefile(
    shp_path: &std::path::Path,
    all_nhs: bool,
) -> Result<Vec<NhsSegment>, NhsError> {
    let mut reader = shapefile::Reader::from_path(shp_path)
        .map_err(|e| NhsError::Shapefile(e.to_string()))?;

    let mut segments = Vec::new();

    for (idx, result) in reader.iter_shapes_and_records().enumerate() {
        let (shape, record) = result.map_err(|e| NhsError::Shapefile(e.to_string()))?;

        // Detect format by presence of ROUTE_ID (FHWA) vs FULLNAME (TIGER)
        let (route_id, state, nhs_type) = if record.get("ROUTE_ID").is_some() {
            // FHWA NHS format
            let rid = get_string_field(&record, "ROUTE_ID")
                .ok_or(NhsError::MissingField("ROUTE_ID"))?;
            let st = get_string_field(&record, "STATE_CODE").unwrap_or_default();
            let nt = get_numeric_field(&record, "NHS_TYPE").unwrap_or(1.0) as u8;
            (rid, st, nt)
        } else {
            // TIGER Primary Roads format
            let rttyp = get_string_field(&record, "RTTYP").unwrap_or_default();
            let fullname = get_string_field(&record, "FULLNAME").unwrap_or_default();
            // Convert TIGER fullname to route_id: "I- 80" → "I80", "I-80" → "I80"
            let rid = tiger_name_to_route_id(&fullname, &rttyp);
            // TIGER has no state field in the national file
            let nt: u8 = if rttyp == "I" { 2 } else { 1 };
            (rid, String::new(), nt)
        };

        // Filter: nhs_type 2 = Interstate; TIGER rttyp 'I' also maps to nhs_type 2
        if !all_nhs && nhs_type != 2 {
            continue;
        }

        // Skip records with empty/unknown route IDs
        if route_id.is_empty() || route_id == "UNKNOWN" {
            continue;
        }

        let geometry = shape_to_linestring(shape, idx)?;
        // Compute length from geometry (degrees → approx miles at mid-latitude)
        let length_miles = get_numeric_field(&record, "MILES")
            .unwrap_or_else(|| approx_length_miles(&geometry));

        segments.push(NhsSegment {
            route_id,
            state,
            nhs_type,
            length_miles,
            geometry,
        });
    }

    segments.sort_by(|a, b| a.route_id.cmp(&b.route_id).then(a.state.cmp(&b.state)));
    Ok(segments)
}

/// Convert TIGER FULLNAME to a normalised route ID.
/// "I- 80" → "I80", "Interstate 80" → "I80", "I-405" → "I405"
fn tiger_name_to_route_id(fullname: &str, rttyp: &str) -> String {
    if rttyp != "I" {
        return "UNKNOWN".into();
    }
    // Strip "I-", "I- ", "Interstate " and extract the number
    let trimmed = fullname
        .replace("Interstate", "")
        .replace("I-", "")
        .replace("I ", "")
        .trim()
        .to_string();
    // Extract leading digits (route number may have letter suffix like "I-80E")
    let num: String = trimmed.chars().take_while(|c| c.is_ascii_alphanumeric()).collect();
    if num.is_empty() { "UNKNOWN".into() } else { format!("I{num}") }
}

/// Approximate segment length in miles from geographic coordinates.
/// Uses flat-earth approximation: good enough for short segments.
fn approx_length_miles(line: &LineString<f64>) -> f64 {
    const DEG_LAT_MILES: f64 = 69.0;
    let coords = &line.0;
    if coords.len() < 2 { return 0.0; }
    let mut total = 0.0;
    for w in coords.windows(2) {
        let dlat = (w[1].y - w[0].y) * DEG_LAT_MILES;
        let mid_lat = (w[0].y + w[1].y) / 2.0;
        let deg_lon_miles = DEG_LAT_MILES * mid_lat.to_radians().cos();
        let dlon = (w[1].x - w[0].x) * deg_lon_miles;
        total += (dlat * dlat + dlon * dlon).sqrt();
    }
    total
}

fn shape_to_linestring(
    shape: shapefile::Shape,
    idx: usize,
) -> Result<LineString<f64>, NhsError> {
    match shape {
        shapefile::Shape::Polyline(poly) => {
            // Take the first part (NHS segments are typically single-part)
            let part = poly.parts().first().ok_or(NhsError::UnsupportedGeometry(idx))?;
            let coords: Vec<Coord<f64>> = part
                .iter()
                .map(|pt| Coord { x: pt.x, y: pt.y })
                .collect();
            Ok(LineString(coords))
        }
        _ => Err(NhsError::UnsupportedGeometry(idx)),
    }
}

fn get_string_field(record: &shapefile::dbase::Record, name: &str) -> Option<String> {
    use shapefile::dbase::FieldValue;
    match record.get(name)? {
        FieldValue::Character(Some(s)) => Some(s.trim().to_string()),
        FieldValue::Character(None) => None,
        _ => None,
    }
}

fn get_numeric_field(record: &shapefile::dbase::Record, name: &str) -> Option<f64> {
    use shapefile::dbase::FieldValue;
    match record.get(name)? {
        FieldValue::Numeric(Some(n)) => Some(*n),
        FieldValue::Float(Some(f)) => Some(*f as f64),
        FieldValue::Integer(n) => Some(*n as f64),
        _ => None,
    }
}
