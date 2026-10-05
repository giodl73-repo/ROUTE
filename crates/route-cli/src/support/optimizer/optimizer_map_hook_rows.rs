//! Helper `optimizer_map_hook_rows`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn optimizer_map_hook_rows() -> Vec<OptimizerMapHookRow> {
    [
        (
            "t1-selector-beck-schematic",
            "data/t1-stop-selector.csv",
            "maps/beck-schematic.png",
            "map",
            "route t1-beck-alignment --gate",
            "Beck T1 alignment proves optimizer-selected T1 stop chains are covered",
        ),
        (
            "t2-selection-t2-schematic",
            "data/t2-service-selection.csv",
            "maps/beck-schematic-t2-only.png",
            "map",
            "route t2-service-selection --gate",
            "T2 schematic classes are backed by service-selection rows",
        ),
        (
            "lower-tier-pressure-t3-zones",
            "data/lower-tier-pressure-witnesses.csv",
            "data/t3-regional-zone-plan.csv",
            "map-plan",
            "route lower-tier-pressure-witnesses --gate",
            "T3/T4 pressure rows feed regional zone planning and map backlogs",
        ),
        (
            "map-atlas-game-campaign",
            "data/map-atlas.csv",
            "data/game/campaign-spine.csv",
            "game-ledger",
            "route game campaign --gate",
            "Campaign stops reference gated atlas ids",
        ),
        (
            "t2-selection-game-overlays",
            "data/t2-service-selection.csv",
            "data/game/t2-bundle-overlays.csv",
            "game-ledger",
            "route t2-bundle-overlays --gate",
            "Game overlays target bundle-bound or explicitly pending T2 service columns",
        ),
        (
            "t2-service-class-game-overlays",
            "data/game/t2-service-overlays.csv",
            "data/game/t2-bundle-overlays.csv",
            "game-ledger",
            "route t2-bundle-overlays --gate",
            "Service-class levers are joined to T2 service columns through bundle overlay rows",
        ),
        (
            "t2-overlays-scenario-hooks",
            "data/game/t2-service-overlays.csv",
            "data/game/t2-scenario-hooks.csv",
            "game-ledger",
            "route game t2-hooks --gate",
            "Scenario hooks consume service-class overlay contracts",
        ),
    ]
    .into_iter()
    .map(
        |(
            hook_id,
            optimizer_artifact,
            consumer_artifact,
            consumer_type,
            gate_command,
            link_basis,
        )| {
            let validation_status = if artifact_has_content(optimizer_artifact)
                && artifact_has_content(consumer_artifact)
            {
                "pass"
            } else {
                "missing-artifact"
            };
            OptimizerMapHookRow {
                hook_id: hook_id.to_string(),
                optimizer_artifact: optimizer_artifact.to_string(),
                consumer_artifact: consumer_artifact.to_string(),
                consumer_type: consumer_type.to_string(),
                gate_command: gate_command.to_string(),
                link_basis: link_basis.to_string(),
                validation_status: validation_status.to_string(),
            }
        },
    )
    .collect()
}
