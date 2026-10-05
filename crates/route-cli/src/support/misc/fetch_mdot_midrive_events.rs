//! Helper `fetch_mdot_midrive_events`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn fetch_mdot_midrive_events(output: &Path) -> Result<()> {
    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let url = "https://mdotjboss.state.mi.us/MiDrive/incidents/AllForMap/";
    let body = reqwest::blocking::get(url)?.error_for_status()?.text()?;
    atomic_write_text(output, body)?;
    Ok(())
}
