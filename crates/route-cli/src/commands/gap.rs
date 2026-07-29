//! `Gap` command handler extracted from main.
use super::super::*;

#[allow(unused_variables)]
pub(crate) fn run(
    ctx: &super::ctx::Ctx<'_>,
    type_: GapType,
    slug: Option<String>
) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let scoring_cfg = ctx.scoring_cfg;
    let scoring_config_path = ctx.scoring_config_path.to_path_buf();
    let r#type = type_;

            println!("route gap --type {:?}", r#type);
            let out_slug = slug.unwrap_or_else(|| gap_type_slug(&r#type).to_string());
            let out = PathBuf::from(format!("gaps/{out_slug}.md"));
            write_gap_report(&r#type, &out)?;
            println!("  wrote gap report → {}", out.display());
        
    Ok(())
}

