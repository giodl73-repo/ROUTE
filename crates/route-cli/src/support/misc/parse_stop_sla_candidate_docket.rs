//! Helper `parse_stop_sla_candidate_docket`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn parse_stop_sla_candidate_docket<R: std::io::Read>(
    reader: R,
) -> Result<Vec<StopSlaCandidateDocketRow>> {
    let mut rdr = csv::Reader::from_reader(reader);
    rdr.deserialize()
        .collect::<std::result::Result<Vec<_>, _>>()
        .context("parsing stop SLA candidate docket")
}

