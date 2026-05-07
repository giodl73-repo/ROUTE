use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};

/// Data source manifest — maps source names to download URLs and local cache paths.
/// Loaded from ~/.route/manifest.json or the path in ROUTE_MANIFEST env var.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub version: String,
    pub cache_dir: PathBuf,
    pub sources: HashMap<String, ManifestSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestSource {
    pub url: String,
    pub filename: String,
    pub format: SourceFormat,
    pub year: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceFormat {
    ShpZip,   // .zip containing .shp + .dbf + .shx
    Csv,
    CsvZip,
}

impl Manifest {
    pub fn load(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading manifest {}", path.display()))?;
        serde_json::from_str(&text)
            .with_context(|| format!("parsing manifest {}", path.display()))
    }

    pub fn default_path() -> PathBuf {
        // Check env var first, then fall back to ~/.route/manifest.json
        if let Ok(p) = std::env::var("ROUTE_MANIFEST") {
            return PathBuf::from(p);
        }
        let home = std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        home.join(".route").join("manifest.json")
    }

    pub fn cache_path(&self, source: &str) -> PathBuf {
        let s = self.sources.get(source).expect("unknown source");
        self.cache_dir.join(&s.filename)
    }
}

