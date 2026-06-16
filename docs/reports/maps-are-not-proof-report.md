---
name: Maps Are Not Proof
slug: maps-are-not-proof-report
type: report
status: draft
rubric_version: v1.0
author: copilot
created: 2026-06-16
updated: 2026-06-16
sources:
  - README.md
  - docs/map-publication-scope.md
  - docs/beck-renderer-contract.md
  - docs/reports/route-evidence-posture.md
  - docs/reports/interstate-2-0-doctrine-report.md
  - docs/vtrace/COMMUNICATIONS_STRATEGY.md
---

# Maps Are Not Proof

## Research Question

How should ROUTE use generated maps as persuasive infrastructure storytelling
without implying SLA readiness, upgrade readiness, terminal-access proof, asset
condition repair, construction readiness, or official adoption?

## Decision This Supports

This report supports the communications decision to use maps as vision and
inspection surfaces while keeping evidence claims attached to their actual
gates.

## Executive Thesis

Maps make a service doctrine legible. They do not prove the doctrine is ready to
build.

ROUTE maps should answer:

```text
what is the service shape?
  -> which stops, routes, tiers, and contacts are visible?
  -> what evidence label travels with the surface?
  -> which claims are excluded, held, or still blocked?
```

## Local Evidence Inventory

| Source | Relevant Evidence |
|---|---|
| `docs/map-publication-scope.md` | Separates render validity from publication validity and states structural maps may be published only as held-claim surfaces. |
| `docs/beck-renderer-contract.md` | Defines renderer contracts, stop/geometry expectations, map consumers, and current optimizer-gated map limits. |
| `docs/reports/route-evidence-posture.md` | Says generated maps are story-ready structural visuals but not proof of SLA, upgrade, transit, or asset-condition readiness. |
| `docs/reports/interstate-2-0-doctrine-report.md` | Names "maps are persuasive, but maps are not proof" as a doctrine finding. |
| `README.md` | Uses maps as generated artifacts tied to service data and states map validity does not prove every downstream claim. |

## Findings

### ROUTE-MAP-01 - Render validity is not claim validity.

**Observed constraint:** Map images can satisfy file and geometry contracts
without proving SLA, transit, upgrade, terminal, evidence, or asset-condition
claims.

**Implication:** Every map caption should state the map's claim posture.

**Confidence:** High.

### ROUTE-MAP-02 - Structural maps are useful precisely because they are bounded.

**Observed constraint:** Current T1-T4 maps can explain service structure when
labeled as structural or held-claim surfaces.

**Implication:** Maps belong in decks, but not as standalone proof slides.

**Confidence:** High.

### ROUTE-MAP-03 - Exclusions must stay visible.

**Observed constraint:** Terminal-access, source-snapshot, and T2
asset-condition exclusions remove specific publication blockers only. They do
not accept terminal, evidence, SLA, transit, upgrade, or repair proof.

**Implication:** Do not let a published map imply the excluded claims have been
solved.

**Confidence:** High.

### ROUTE-MAP-04 - Map truth depends on stops, contacts, and identity.

**Observed constraint:** Schematic maps must avoid false stops, false transfers,
and route-label-only identity claims.

**Implication:** Map stories should point to stop/SLA gates, bundle identity,
and diagnostics where technical credibility matters.

**Confidence:** Medium-high.

## Map Claim Ladder

| Claim Level | Meaning | Current Use |
|---|---|---|
| Render-valid | File exists and satisfies renderer contract checks. | Technical artifact claim only. |
| Structural | Shows service tiers, stops, contacts, or schematic shape. | Safe for decks with label. |
| Held-claim surface | Useful visual with blocked downstream claims. | Safe when blockers are named. |
| Evidence-valid | Source and gate evidence support the specific claim. | Future claim only where evidence closes. |
| Publication-ready proof | Ready for external proof-grade use. | Not implied by current communications package. |

## Safe Language

| Use This | Avoid This |
|---|---|
| "This is a structural service map." | "This map proves the plan is ready." |
| "The map makes the doctrine visible." | "The map proves SLA performance." |
| "Some downstream claims remain held." | "Publication means every claim is cleared." |
| "Render validity is separate from evidence validity." | "A clean image is a validated corridor." |

## Evidence Needed To Promote A Map Claim

| Evidence Need | Why It Matters |
|---|---|
| Map atlas and renderer gate | Confirms the image artifact and geometry contract. |
| Stop/SLA diagnostics | Prevents false service or missing-contact claims. |
| Bundle identity | Keeps route labels from becoming unstable claim keys. |
| Publication scope decision | Names exclusions and residual blockers. |
| Claim-specific evidence | Separates SLA, terminal, upgrade, asset, and evidence claims. |

## Recommendations

### Adopt Now

| Recommendation | Owner | Validation |
|---|---|---|
| Caption every map with its claim posture. | Schematic Cartographer | Deck/report review finds no map-proof wording. |
| Use maps as visual argument surfaces, not proof substitutes. | communications owner | Evidence links or blockers accompany map claims. |
| Preserve exclusions and residual blockers in technical contexts. | Citation Auditor | Map claims cite `docs/map-publication-scope.md`. |

### Reject Or Defer

| Claim / Action | Reason |
|---|---|
| Saying the national map is the final plan. | Official-plan and construction claims are gated. |
| Treating map publication as SLA or upgrade proof. | Publication scope explicitly separates those claims. |
| Hiding terminal or asset-condition exclusions. | Exclusions do not clear downstream evidence obligations. |

## Non-Goals

- This report does not validate any new map artifact.
- This report does not claim SLA, terminal, transit, upgrade, asset-condition, or
  construction readiness.
- This report does not select final routes or official designations.

## Gate

Decision: pass_with_risk

Rationale: Maps are safe and valuable as structural communications surfaces
when labeled. They remain unsafe as proof of service, upgrade, terminal, asset,
official-plan, or construction claims until claim-specific evidence closes.
