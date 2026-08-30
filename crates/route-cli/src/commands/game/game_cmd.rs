//! `Game` command handler — exemplar-style extraction from main.
use crate::commands::ctx;
use crate::*;
#[allow(unused_variables)]
pub(crate) fn run(ctx: &ctx::Ctx<'_>, command: GameCommand) -> Result<()> {
    let manifest_path = ctx.manifest_path.to_path_buf();
    let _scoring_cfg = ctx.scoring_cfg;
    let _scoring_config_path = ctx.scoring_config_path;
    match command {
        GameCommand::Scenarios => game::print_scenarios(),
        GameCommand::Campaign {
            ledger,
            map_atlas,
            gate,
        } => game::campaign_cli(&ledger, &map_atlas, gate)?,
        GameCommand::T2Overlays {
            ledger,
            standards,
            map_atlas,
            gate,
        } => game::t2_service_overlays_cli(&ledger, &standards, &map_atlas, gate)?,
        GameCommand::T2Hooks {
            ledger,
            campaign,
            overlays,
            gate,
        } => game::t2_scenario_hooks_cli(&ledger, &campaign, &overlays, gate)?,
        GameCommand::Inspect { scenario } => game::print_inspect(&scenario)?,
        GameCommand::RunSeason {
            scenario,
            season,
            event,
            project,
            state,
            write_state,
            append_log,
        } => game::run_season_cli(
            &scenario,
            season,
            &event,
            &project,
            state.as_deref(),
            write_state.as_deref(),
            append_log.as_deref(),
        )?,
        GameCommand::Score {
            scenario,
            log,
            details,
            gate_promotion,
        } => {
            let engine_facts = game_engine_facts(&scenario, &manifest_path).with_context(|| {
                format!("summarizing ROUTE engine facts for game scenario {scenario}")
            })?;
            game::score_cli(&scenario, &log, details, gate_promotion, engine_facts)?
        }
    }
    Ok(())
}
