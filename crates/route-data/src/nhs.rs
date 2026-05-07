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

/// Read all NHS segments from a .shp file extracted from the FHWA NHS shapefile zip.
/// Returns segments sorted by route_id + state for deterministic ordering.
/// Filters to nhs_type == 2 (Interstate) by default — pass `all_nhs = true` to include all.
pub fn read_nhs_shapefile(
    shp_path: &std::path::Path,
    all_nhs: bool,
) -> Result<Vec<NhsSegment>, NhsError> {
    let mut reader = shapefile::Reader::from_path(shp_path)
        .map_err(|e| NhsError::Shapefile(e.to_string()))?;

    let mut segments = Vec::new();

    for (idx, result) in reader.iter_shapes_and_records().enumerate() {
        let (shape, record) = result.map_err(|e| NhsError::Shapefile(e.to_string()))?;

        let route_id = get_string_field(&record, "ROUTE_ID")
            .ok_or(NhsError::MissingField("ROUTE_ID"))?;
        let state = get_string_field(&record, "STATE_CODE")
            .ok_or(NhsError::MissingField("STATE_CODE"))?;
        let nhs_type = get_numeric_field(&record, "NHS_TYPE").unwrap_or(1.0) as u8;
        let length_miles = get_numeric_field(&record, "MILES").unwrap_or(0.0);

        if !all_nhs && nhs_type != 2 {
            continue;
        }

        let geometry = shape_to_linestring(shape, idx)?;

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
