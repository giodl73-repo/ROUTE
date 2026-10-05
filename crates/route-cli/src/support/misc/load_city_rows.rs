//! Helper `load_city_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_city_rows(path: &Path) -> Result<Vec<CitySeedRow>> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let file: CitySeedFile =
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))?;
    Ok(file.cities)
}
