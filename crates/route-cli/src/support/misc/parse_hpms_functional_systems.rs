//! Helper `parse_hpms_functional_systems`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_hpms_functional_systems(value: &str) -> Result<Vec<u8>> {
    let systems = value
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            part.parse::<u8>()
                .with_context(|| format!("invalid HPMS functional system {part}"))
        })
        .collect::<Result<Vec<_>>>()?;
    if systems.is_empty() {
        anyhow::bail!("at least one HPMS functional system is required");
    }
    let systems = systems
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    if systems.iter().any(|system| !(1..=7).contains(system)) {
        anyhow::bail!("HPMS functional systems must be in 1..=7");
    }
    Ok(systems)
}

