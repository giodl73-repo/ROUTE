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
    let mut sorted: Vec<f64> = scores.iter().copied().filter(|v| v.is_finite()).collect();
    if sorted.is_empty() {
        return DimensionStats {
            dim: Some(dim),
            ..Default::default()
        };
    }
    sorted.sort_by(f64::total_cmp);

    let n = sorted.len() as f64;
    let mean = sorted.iter().sum::<f64>() / n;
    let min = sorted[0];
    let max = sorted[sorted.len() - 1];
    let q1 = sorted[(n * 0.25) as usize];
    let q3 = sorted[(n * 0.75) as usize];
    let iqr = q3 - q1;

    DimensionStats {
        dim: Some(dim),
        count: sorted.len(),
        mean,
        iqr,
        min,
        max,
    }
}

/// Compute stats for every dimension currently emitted by [`DimensionScores`].
pub fn compute_all_stats(all_scores: &[DimensionScores]) -> Vec<DimensionStats> {
    let extract =
        |f: fn(&DimensionScores) -> f64| -> Vec<f64> { all_scores.iter().map(f).collect() };

    vec![
        compute_stats(&extract(|s| s.a1.score), Dimension::A1ThroughputGap),
        compute_stats(&extract(|s| s.a2.score), Dimension::A2FreightIntensity),
        compute_stats(&extract(|s| s.a3.score), Dimension::A3SpeedReliability),
        compute_stats(&extract(|s| s.a4.score), Dimension::A4InternationalTrade),
        compute_stats(&extract(|s| s.a5.score), Dimension::A5SafetyRecord),
        compute_stats(&extract(|s| s.b1.score), Dimension::B1Redundancy),
        compute_stats(&extract(|s| s.b2.score), Dimension::B2NetworkCentrality),
        compute_stats(&extract(|s| s.b3.score), Dimension::B3PortBorderAccess),
        compute_stats(&extract(|s| s.b4.score), Dimension::B4MilitaryStrategic),
        compute_stats(&extract(|s| s.c1.score), Dimension::C1PopulationReach),
        compute_stats(&extract(|s| s.c2.score), Dimension::C2RuralConnectivity),
        compute_stats(&extract(|s| s.c3.score), Dimension::C3EconomicOpportunity),
        compute_stats(&extract(|s| s.c4.score), Dimension::C4AgriculturalExport),
        compute_stats(&extract(|s| s.d1.score), Dimension::D1ClimateResilience),
        compute_stats(&extract(|s| s.d2.score), Dimension::D2MultimodalIntegration),
        compute_stats(&extract(|s| s.d3.score), Dimension::D3InfrastructureVintage),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::score::{score_corridor, DimensionScores};
    use route_network::CorridorAttributes;

    #[test]
    fn compute_stats_ignores_non_finite_values() {
        let stats = compute_stats(
            &[1.0, f64::NAN, 3.0, f64::INFINITY],
            Dimension::A1ThroughputGap,
        );
        assert_eq!(stats.count, 2);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 3.0);
        assert_eq!(stats.mean, 2.0);
    }

    #[test]
    fn compute_all_stats_covers_all_emitted_dimensions() {
        let cfg = crate::ScoringConfig::default_config();
        let scores: DimensionScores = score_corridor(&CorridorAttributes::default(), &cfg);
        let stats = compute_all_stats(&[scores]);
        assert_eq!(stats.len(), 16);
        assert_eq!(stats[0].dim, Some(Dimension::A1ThroughputGap));
        assert_eq!(stats[15].dim, Some(Dimension::D3InfrastructureVintage));
    }
}
