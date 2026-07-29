//! Shared CLI context for extracted command handlers.
use super::super::*;

pub(crate) struct Ctx<'a> {
    pub manifest_path: &'a Path,
    pub scoring_cfg: &'a route_score::ScoringConfig,
    pub scoring_config_path: &'a Path,
}
