//! Helper `validate_region_assignment`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn validate_region_assignment(assignment: &[usize], requested_regions: usize) -> Result<()> {
    let mut counts = vec![0usize; requested_regions];
    for &region in assignment {
        if region >= requested_regions {
            anyhow::bail!("METIS assigned route to out-of-range region {region}");
        }
        counts[region] += 1;
    }
    for (region, count) in counts.into_iter().enumerate() {
        if count == 0 {
            anyhow::bail!("METIS produced empty region {region}");
        }
    }
    Ok(())
}

