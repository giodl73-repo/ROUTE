//! Helper `load_pressure_scenarios`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_pressure_scenarios(path: &Path) -> Result<Vec<PressureScenarioRow>> {
    let file = std::fs::File::open(path)?;
    parse_pressure_scenarios(file)
}
