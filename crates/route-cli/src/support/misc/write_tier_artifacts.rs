//! Helper `write_tier_artifacts`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn write_tier_artifacts(score_rows: &[ScoreAllRow]) -> Result<()> {
    write_tier_artifacts_to(score_rows, Path::new("data"))
}

