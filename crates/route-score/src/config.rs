use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

/// Three-point linear interpolation anchor map.
/// anchor_0 → score 0.0, anchor_5 → score 5.0, anchor_10 → score 10.0.
/// Values beyond anchor_10 are clamped to 10.0.
#[derive(Debug, Clone, Deserialize)]
pub struct AnchorMap {
    pub anchor_0: f64,
    pub anchor_5: f64,
    pub anchor_10: f64,
}

impl AnchorMap {
    /// Interpolate a score for the given value.
    /// Handles both increasing (low=0, high=10) and decreasing (high=0, low=10) anchors.
    pub fn score(&self, value: f64) -> f64 {
        let increasing = self.anchor_10 > self.anchor_0;
        if increasing {
            if value <= self.anchor_0 { return 0.0; }
            if value >= self.anchor_10 { return 10.0; }
            if value <= self.anchor_5 {
                5.0 * (value - self.anchor_0) / (self.anchor_5 - self.anchor_0)
            } else {
                5.0 + 5.0 * (value - self.anchor_5) / (self.anchor_10 - self.anchor_5)
            }
        } else {
            // Decreasing: high raw value → low score (e.g. GDP per capita relative for C3)
            if value >= self.anchor_0 { return 0.0; }
            if value <= self.anchor_10 { return 10.0; }
            if value >= self.anchor_5 {
                5.0 * (self.anchor_0 - value) / (self.anchor_0 - self.anchor_5)
            } else {
                5.0 + 5.0 * (self.anchor_5 - value) / (self.anchor_5 - self.anchor_10)
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct C2Anchors {
    pub rural_share: AnchorMap,
    pub interchange_gap: AnchorMap,
}

#[derive(Debug, Clone, Deserialize)]
pub struct D1Anchors {
    pub consecutive_sfha: AnchorMap,
    pub total_sfha: AnchorMap,
}

#[derive(Debug, Clone, Deserialize)]
pub struct D2Anchors {
    pub intermodal_hubs: AnchorMap,
    pub dcfc_per_100mi: AnchorMap,
}

#[derive(Debug, Clone, Deserialize)]
pub struct D3Anchors {
    pub bridges_poor: AnchorMap,
    pub mean_year_built: AnchorMap,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ScoringConfig {
    pub meta: ConfigMeta,
    pub a1: AnchorMap,
    pub a2: AnchorMap,
    pub a3: AnchorMap,
    pub b1: AnchorMap,
    pub b2: AnchorMap,
    pub b3: AnchorMap,  // used only for distance fallback; stepped logic in score_b3
    pub c1: AnchorMap,
    pub c2: C2Anchors,
    pub c3: AnchorMap,
    pub d1: D1Anchors,
    pub d2: D2Anchors,
    pub d3: D3Anchors,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfigMeta {
    pub version: String,
    pub rubric_version: String,
}

impl ScoringConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading scoring config {}", path.display()))?;
        toml::from_str(&text)
            .with_context(|| format!("parsing scoring config {}", path.display()))
    }

    /// Built-in defaults — matches config/scoring.toml in the repo.
    /// Used when no config file is found.
    pub fn default_config() -> Self {
        toml::from_str(include_str!("../../../config/scoring.toml"))
            .expect("built-in scoring config must parse")
    }
}
