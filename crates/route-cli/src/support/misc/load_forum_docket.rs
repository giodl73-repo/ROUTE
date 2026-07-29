//! Helper `load_forum_docket`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_forum_docket(path: &Path) -> Result<Vec<ForumDocketRow>> {
    let file = std::fs::File::open(path)?;
    parse_forum_docket(file)
}

