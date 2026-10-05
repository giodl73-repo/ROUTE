//! Helper `load_blueprint_packages`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_blueprint_packages(path: &Path) -> Result<Vec<BlueprintPackageRow>> {
    let file = std::fs::File::open(path)?;
    parse_blueprint_packages(file)
}
