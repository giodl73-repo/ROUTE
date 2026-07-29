//! `T1AccumulateEvents` command handler extracted from main.
use crate::*;
use crate::commands::ctx;
#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &ctx::Ctx<'_>,
    events: PathBuf,
    input: PathBuf,
    output: PathBuf
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();

    let existing = if events.exists() {
        load_t1_failure_events(&events)
            .with_context(|| format!("loading accumulated events {}", events.display()))?
    } else {
        Vec::new()
    };
    let incoming = load_t1_failure_events(&input)
        .with_context(|| format!("loading incoming events {}", input.display()))?;
    let merged = merge_t1_failure_events(&existing, &incoming);
    let added = merged.len().saturating_sub(existing.len());
    write_t1_failure_events(&output, &merged)
        .with_context(|| format!("writing accumulated events {}", output.display()))?;
    println!("route t1-accumulate-events");
    println!("  existing rows: {}", existing.len());
    println!("  incoming rows: {}", incoming.len());
    println!("  merged rows: {}", merged.len());
    println!("  net new rows: {added}");
    println!("  wrote {}", output.display());
        
    Ok(())
}

