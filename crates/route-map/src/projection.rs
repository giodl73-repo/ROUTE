/// Albers Equal Area Conic projection for continental US maps.
///
/// Standard parameters for CONUS:
///   Standard parallels: 29.5°N and 45.5°N
///   Central meridian:   96°W
///   Origin latitude:    37.5°N
///
/// Output is in abstract projection units; caller scales to pixel coordinates.
use std::f64::consts::PI;

const DEG_TO_RAD: f64 = PI / 180.0;

/// Albers Equal Area Conic — US configuration.
pub struct AlbersUS {
    n: f64,
    c: f64,
    rho0: f64,
    lambda0: f64,
}

impl AlbersUS {
    pub fn new() -> Self {
        let phi1 = 29.5_f64.to_radians();   // first standard parallel
        let phi2 = 45.5_f64.to_radians();   // second standard parallel
        let phi0 = 37.5_f64.to_radians();   // origin latitude
        let lambda0 = (-96.0_f64).to_radians(); // central meridian

        let n = (phi1.sin() + phi2.sin()) / 2.0;
        let c = phi1.cos().powi(2) + 2.0 * n * phi1.sin();
        let rho0 = (c - 2.0 * n * phi0.sin()).sqrt() / n;

        AlbersUS { n, c, rho0, lambda0 }
    }

    /// Project (lon_deg, lat_deg) → (x, y) in Albers units.
    pub fn project(&self, lon_deg: f64, lat_deg: f64) -> (f64, f64) {
        let phi = lat_deg.to_radians();
        let lambda = lon_deg.to_radians();
        let theta = self.n * (lambda - self.lambda0);
        let rho = (self.c - 2.0 * self.n * phi.sin()).max(0.0).sqrt() / self.n;
        let x = rho * theta.sin();
        let y = self.rho0 - rho * theta.cos();
        (x, y)
    }
}

impl Default for AlbersUS {
    fn default() -> Self { Self::new() }
}

/// Bounding box for continental US in Albers units.
/// Used to scale coordinates to pixel space.
pub struct ViewTransform {
    pub x_min: f64,
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub width: f64,
    pub height: f64,
    pub padding: f64,
}

impl ViewTransform {
    /// Standard CONUS viewbox with padding.
    pub fn conus(width: f64, height: f64) -> Self {
        let proj = AlbersUS::new();
        // Corners of CONUS in Albers units
        let corners = [
            proj.project(-124.8, 49.0), // NW
            proj.project(-66.9, 49.0),  // NE
            proj.project(-124.8, 24.5), // SW
            proj.project(-66.9, 24.5),  // SE
        ];
        let x_min = corners.iter().map(|c| c.0).fold(f64::MAX, f64::min);
        let x_max = corners.iter().map(|c| c.0).fold(f64::MIN, f64::max);
        let y_min = corners.iter().map(|c| c.1).fold(f64::MAX, f64::min);
        let y_max = corners.iter().map(|c| c.1).fold(f64::MIN, f64::max);

        ViewTransform { x_min, x_max, y_min, y_max, width, height, padding: 40.0 }
    }

    /// Convert Albers (x, y) to SVG pixel (px, py).
    pub fn to_pixel(&self, x: f64, y: f64) -> (f64, f64) {
        let draw_w = self.width - 2.0 * self.padding;
        let draw_h = self.height - 2.0 * self.padding;
        let scale = (draw_w / (self.x_max - self.x_min))
            .min(draw_h / (self.y_max - self.y_min));

        // Center the map
        let map_w = (self.x_max - self.x_min) * scale;
        let map_h = (self.y_max - self.y_min) * scale;
        let x_off = self.padding + (draw_w - map_w) / 2.0;
        let y_off = self.padding + (draw_h - map_h) / 2.0;

        let px = x_off + (x - self.x_min) * scale;
        // SVG y is inverted (top = 0)
        let py = y_off + (self.y_max - y) * scale;
        (px, py)
    }

    /// Project lon/lat directly to pixel.
    pub fn project_to_pixel(&self, proj: &AlbersUS, lon: f64, lat: f64) -> (f64, f64) {
        let (x, y) = proj.project(lon, lat);
        self.to_pixel(x, y)
    }
}
