use crate::score::{Dimension, DimensionScores};

/// Per-dimension statistics across all scored corridors.
/// Written to personas/axis-pool.md after route score-all.
#[derive(Debug, Default)]
pub struct DimensionStats {
    pub dim: Option<Dimension>,
    pub count: usize,
    pub mean: f64,
    pub iqr: f64,
    pub min: f64,
    pub max: f64,
}

/// Compute variance stats across a set of scores for one dimension.
pub fn compute_stats(scores: &[f64], dim: Dimension) -> DimensionStats {
    if scores.is_empty() {
        return DimensionStats { dim: Some(dim), ..Default::default() };
    }
    let mut sorted = scores.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = sorted.len() as f64;
    let mean = sorted.iter().sum::<f64>() / n;
    let min = sorted[0];
    let max = *sorted.last().unwrap();
    let q1 = sorted[(n * 0.25) as usize];
    let q3 = sorted[(n * 0.75) as usize];
    let iqr = q3 - q1;

    DimensionStats { dim: Some(dim), count: sorted.len(), mean, iqr, min, max }
}

/// Compute all 12 dimension stats from a batch of DimensionScores.
pub fn compute_all_stats(all_scores: &[DimensionScores]) -> [DimensionStats; 12] {
    let extract = |f: fn(&DimensionScores) -> f64| -> Vec<f64> {
        all_scores.iter().map(f).collect()
    };

    [
        compute_stats(&extract(|s| s.a1.score), Dimension::A1ThroughputGap),
        compute_stats(&extract(|s| s.a2.score), Dimension::A2FreightIntensity),
        compute_stats(&extract(|s| s.a3.score), Dimension::A3SpeedReliability),
        compute_stats(&extract(|s| s.b1.score), Dimension::B1Redundancy),
        compute_stats(&extract(|s| s.b2.score), Dimension::B2NetworkCentrality),
        compute_stats(&extract(|s| s.b3.score), Dimension::B3PortBorderAccess),
        compute_stats(&extract(|s| s.c1.score), Dimension::C1PopulationReach),
        compute_stats(&extract(|s| s.c2.score), Dimension::C2RuralConnectivity),
        compute_stats(&extract(|s| s.c3.score), Dimension::C3EconomicOpportunity),
        compute_stats(&extract(|s| s.d1.score), Dimension::D1ClimateResilience),
        compute_stats(&extract(|s| s.d2.score), Dimension::D2MultimodalIntegration),
        compute_stats(&extract(|s| s.d3.score), Dimension::D3InfrastructureVintage),
    ]
}
