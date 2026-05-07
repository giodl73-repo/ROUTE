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

/// Road classification — drives corpus entry type and scoring interpretation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoadClass {
    /// Existing interstate — score as-is
    Interstate,
    /// US numbered highway — score as upgrade candidate
    UsHighway,
    /// State highway — score as upgrade candidate (lower priority)
    StateHighway,
    /// Other
    Other,
}

impl RoadClass {
    pub fn from_rttyp(rttyp: &str) -> Self {
        match rttyp {
            "I" => RoadClass::Interstate,
            "U" => RoadClass::UsHighway,
            "S" => RoadClass::StateHighway,
            _ => RoadClass::Other,
        }
    }

    pub fn corpus_type(&self) -> &'static str {
        match self {
            RoadClass::Interstate => "existing-corridor",
            RoadClass::UsHighway | RoadClass::StateHighway => "upgrade-candidate",
            RoadClass::Other => "upgrade-candidate",
        }
    }

    pub fn prefix(&self) -> &'static str {
        match self {
            RoadClass::Interstate => "I",
            RoadClass::UsHighway => "US",
            RoadClass::StateHighway => "SR",
            RoadClass::Other => "",
        }
    }
}

/// A single segment record from a road shapefile (FHWA NHS or TIGER Primary Roads).
#[derive(Debug, Clone)]
pub struct NhsSegment {
    /// Normalised route identifier, e.g. "I80", "US30", "SR97"
    pub route_id: String,
    /// Two-letter state code (empty for TIGER national file)
    pub state: String,
    /// Road classification
    pub road_class: RoadClass,
    /// Segment length in miles
    pub length_miles: f64,
    /// Geometry in EPSG:4269 (NAD83 geographic)
    pub geometry: LineString<f64>,
    // nhs_type kept for FHWA NHS format compatibility
    pub nhs_type: u8,
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
        let (route_id, state, nhs_type, road_class) = if record.get("ROUTE_ID").is_some() {
            // FHWA NHS format
            let rid = get_string_field(&record, "ROUTE_ID")
                .ok_or(NhsError::MissingField("ROUTE_ID"))?;
            let st = get_string_field(&record, "STATE_CODE").unwrap_or_default();
            let nt = get_numeric_field(&record, "NHS_TYPE").unwrap_or(1.0) as u8;
            let rc = if nt == 2 { RoadClass::Interstate } else { RoadClass::UsHighway };
            (rid, st, nt, rc)
        } else {
            // TIGER Primary Roads format — includes interstates AND US routes
            let rttyp = get_string_field(&record, "RTTYP").unwrap_or_default();
            let fullname = get_string_field(&record, "FULLNAME").unwrap_or_default();
            let rc = RoadClass::from_rttyp(&rttyp);
            let rid = tiger_name_to_route_id(&fullname, &rttyp);
            let nt: u8 = match rc { RoadClass::Interstate => 2, _ => 1 };
            (rid, String::new(), nt, rc)
        };

        // Filter by road class
        let include = match &road_class {
            RoadClass::Interstate => true,
            RoadClass::UsHighway => all_nhs,   // included when all_nhs=true
            RoadClass::StateHighway => all_nhs,
            RoadClass::Other => false,
        };
        if !include { continue; }

        // Skip records with empty/unknown route IDs
        if route_id.is_empty() || route_id == "UNKNOWN" {
            continue;
        }

        let geometry = shape_to_linestring(shape, idx)?;
        let length_miles = get_numeric_field(&record, "MILES")
            .unwrap_or_else(|| approx_length_miles(&geometry));

        segments.push(NhsSegment {
            route_id,
            state,
            road_class,
            nhs_type,
            length_miles,
            geometry,
        });
    }

    segments.sort_by(|a, b| a.route_id.cmp(&b.route_id).then(a.state.cmp(&b.state)));
    Ok(segments)
}

/// Convert TIGER FULLNAME + RTTYP to a normalised route ID.
/// Interstate: "I- 80" → "I80", "Interstate 80" → "I80"
/// US Route:   "US Hwy 30" → "US30", "U.S. 101" → "US101"
/// State:      "State Hwy 97" → "SR97"
fn tiger_name_to_route_id(fullname: &str, rttyp: &str) -> String {
    // Strip common prefixes and extract the number portion
    let stripped = fullname
        .replace("Interstate", "")
        .replace("U.S.", "")
        .replace("US Hwy", "")
        .replace("US Highway", "")
        .replace("US Rte", "")
        .replace("State Hwy", "")
        .replace("State Highway", "")
        .replace("State Rte", "")
        .replace("I-", "")
        .replace("I ", "")
        .trim()
        .to_string();

    // Extract alphanumeric route number (e.g. "101", "30", "280A")
    let num: String = stripped
        .chars()
        .skip_while(|c| !c.is_ascii_digit()) // skip any remaining prefix chars
        .take_while(|c| c.is_ascii_alphanumeric())
        .collect();

    if num.is_empty() {
        return "UNKNOWN".into();
    }

    match rttyp {
        "I" => format!("I{num}"),
        "U" => format!("US{num}"),
        "S" => format!("SR{num}"),
        _ => "UNKNOWN".into(),
    }
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
