---
name: ROUTE Public Proof
slug: route-public-proof
type: how-to
status: reviewed
rubric_version: v1.0
author: copilot
created: 2026-07-29
updated: 2026-07-29
sources:
  - README.md
  - GOAL.md
  - docs/packets/i80-flagship-review-packet.md
  - docs/map-publication-scope.md
  - docs/adoption/README.md
---

# Public proof (15 minutes)

This is the shortest path to inspect ROUTE’s **external-facing honesty edge**
without treating maps or mileposts as an approved Interstate 2.0 plan.

## What this proves

| Check | Pass means |
|---|---|
| I-80 flagship packet | The hold-and-narrow review packet still builds from corpus, gap, parliament, and docket sources |
| No-credential source gate | Public/no-key I-80 sources are ready or explicitly excluded; credential blockers are named |
| Claim boundary | Packet posture remains **hold and narrow** — no design/ROI/SLA promotion |

## What this does **not** prove

- Official plan, agency endorsement, or construction readiness
- Full clean-clone report regeneration (ACS still needs `CENSUS_API_KEY` when you choose to run it)
- Publication-grade T1–T4 map claims
- Any promoted entry under `design/`

## Command

From the repo root:

```powershell
npm run proof:public
```

Equivalent steps:

```powershell
npm run check:i80:packet
npm run gate:i80:sources:no-credential
```

Optional (only if you intentionally supply a Census key):

```powershell
$env:CENSUS_API_KEY = "<your key>"  # never commit
npm run reproduce:i80:report
```

ROUTE does not store or log the key.

## Read these artifacts

| Order | Artifact | Why |
|---:|---|---|
| 1 | [`docs/packets/i80-flagship-review-packet.md`](../packets/i80-flagship-review-packet.md) | Ten-minute hold-and-narrow packet |
| 2 | [`gaps/i80-flagship.md`](../../gaps/i80-flagship.md) | Observed vs unloaded vs source gaps |
| 3 | [`docs/map-publication-scope.md`](../map-publication-scope.md) | Render-valid ≠ publication-valid |
| 4 | [`docs/adoption/README.md`](../adoption/README.md) | Safe reuse language |

## Exit criteria for “proof green”

1. `npm run proof:public` exits 0  
2. Packet still says **Hold and narrow**  
3. Source gate reports ready/excluded/blocked counts without hiding blockers  
4. No new design or ROI claim is introduced by the proof path  

If you need a deeper lab verification matrix, use
[`docs/vtrace/VERIFICATION.md`](../vtrace/VERIFICATION.md) — that is operator-grade,
not the public 15-minute path.
