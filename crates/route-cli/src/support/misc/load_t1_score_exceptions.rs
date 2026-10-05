//! Helper `load_t1_score_exceptions`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_t1_score_exceptions(path: &Path) -> Result<Vec<T1ScoreExceptionRow>> {
    let file = std::fs::File::open(path)?;
    parse_t1_score_exceptions(file)
}
