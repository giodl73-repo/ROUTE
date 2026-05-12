# Milepost 10 Optimizer Output Review

Date: 2026-05-12

## Verdict

Milepost 10 now has a reproducible optimizer surface for T1/T2/T3/T4 selection:
line and stop decisions are represented as artifacts, optimizer gates are
manifested, and map/game consumers have explicit hook rows.

This review does not close the two known T2 holds. It records them as real
blockers rather than treating the current graph as complete.

## Verified Commands

- `cargo test -p route`
- `cargo test -p route-network`
- `cargo test -p route-map`
- `route tier-optimize --all-tiers --gate`
- `route optimizer-map-hooks --gate`

## Passing Artifacts

- `data/t1-stop-selector.csv`
- `data/t1-topology-repairs.csv`
- `data/t1-beck-alignment.csv`
- `data/tier-candidate-columns.csv`
- `data/t2-regionalizer.csv`
- `data/t2-service-selection.csv`
- `data/lower-tier-pressure-witnesses.csv`
- `data/tier-optimizer-runs.csv`
- `data/optimizer-map-hooks.csv`
- `data/t2-contact-resolutions.csv`
- `data/t2-held-contact-actions.csv`
- `data/t2-graph-contact-repairs.csv`
- `data/t2-parent-contact-validation.csv`
- `data/t2-relief-evidence-docket.csv`
- `data/t2-terminal-contact-validation.csv`

## Held Findings

1. `tier-regions --tier T2 --gate` remains held by the bridged T2 component.
   The manifest records this as `held-known`, not pass.

2. `tier-contact-witnesses --gate` remains held by unresolved graph-contact,
   parent-contact, relief-evidence, and terminal exception/contact rows.
   Demotion and candidate-review rows now move downstream through
   `data/t2-contact-resolutions.csv`; lower-tier pressure consumes the
   demotion rows. `data/t2-held-contact-actions.csv` splits the remaining held
   rows into graph repair, parent-contact, relief-evidence, and terminal
   validation surfaces. `data/t2-graph-contact-repairs.csv` further splits
   graph repair into route-family split versus graph-contact-or-demotion work.
   `data/t2-parent-contact-validation.csv` isolates parent-contact proof for
   I24 and I495. `data/t2-relief-evidence-docket.csv` confirms I285 and I405
   have direct ATRI bottleneck evidence, but keeps them in review until graph
   contact validation proves the relief service can attach to the T2 system.
   `data/t2-terminal-contact-validation.csv` separates I25/I65 terminal-worthy
   endpoint exceptions with missing graph contact from I270, which has contact
   observations but lacks a terminal-worthy endpoint exception.

3. Beck T1 is alignment-gated against optimizer-selected stops, but the map
   renderer still does not directly consume `data/t1-stop-selector.csv`.

4. Lower-tier pressure witnesses are a first score/demotion pressure surface.
   They are not yet county-access failure optimization.

## Conclusion

The milestone now has the audit trail we wanted:

```text
SLA / tier score -> graph or split artifact -> selected route/stop column -> gate -> map/game hook
```

Remaining work should focus on resolving T2 contact repairs and replacing the
last alignment-only map checks with direct optimizer-fed rendering.
