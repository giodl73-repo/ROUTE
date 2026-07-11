---
name: I-80 Flagship Baseline
slug: i80-flagship-baseline
type: report
status: draft
rubric_version: v1.4
author: copilot
created: 2026-07-11
updated: 2026-07-11
sources:
  - corpus/existing/i80.md
  - data/scores-all.csv
  - gaps/missing-link.md
  - docs/STANDARDS_EVALUATION.md
  - docs/reports/route-evidence-posture.md
  - waves/PHASES.md
---

# I-80 Flagship Baseline

## Decision

I-80 is the ROUTE anchor. Expansion is paused until this corridor has a complete
source-to-decision record or an explicit review decision that the available
evidence cannot support one.

No investment treatment is selected in this baseline. Treatment selection must
follow the corridor-specific gap diagnosis.

## Current Assets

| Surface | Current state | Evidence |
|---|---|---|
| Corridor record | Draft generated record | `corpus/existing/i80.md` |
| Rubric result | 89.8 / 160, T1, v1.4 | `data/scores-all.csv` |
| Confidence | 0.76 overall, 0.80 score confidence, both Medium | `data/scores-all.csv` |
| Structural map | Generated regional corridor map | `maps/i80.png` |
| Resilience scenario | Donner closure concept and executable scenario exist | `docs/game/donner-weather-closure-g0.md` |
| T1/T1 operating evidence | I-35/I-80 Iowa evidence path exists but remains snapshot-limited | `docs/STANDARDS_EVALUATION.md` |
| Claim controls | Maps, SLA, ROI, construction, and official-plan claims are bounded | `docs/reports/route-evidence-posture.md` |

## Material Gaps

| Gap | Why it blocks the flagship | Required closeout |
|---|---|---|
| Human narrative is incomplete | Overview, notable segments, fit, and open questions remain placeholders | Replace placeholders with sourced, bounded analysis |
| Several scores are proxy or hard-coded | A2, A3, A4, B4, C4, and hazard reasoning need source-level audit | Confirm, relabel, or hold each material input |
| No I-80-specific gap artifact exists | National gap outputs do not establish the corridor's decisive failure | Produce a corridor-specific measured/source/model/geometry gap record |
| Treatment is not evidence-selected | Existing Donner, interchange, managed-lane, and redundancy ideas are separate threads | Compare bounded treatments after gap diagnosis |
| No I-80 Parliament record exists | Architecture reviews do not review the corridor decision | Run all seven voices on the selected treatment |
| No corridor design proposal exists | The repository has no `design/` flagship output for I-80 | Create one reviewed, evidence-labeled treatment proposal |
| No compact decision packet exists | A user must navigate hundreds of artifacts | Generate one short report with maps and evidence appendix |
| No external review record exists | Internal gates do not establish practitioner credibility | Prepare DOT/MPO, freight, and research review lanes |

## Engineering Risks On The Flagship Path

| Risk | Effect on the flagship |
|---|---|
| `route-cli/src/main.rs` owns too much domain logic | The command path is difficult to review and maintain |
| `Cargo.lock` is ignored | Clean-clone dependency resolution is not fully reproducible |
| Git dependencies follow branch heads | FLETCH or METIS-CORE changes can alter later builds |
| CI is Windows-only and artifact-heavy | Portability and fast software-quality feedback are weak |

These risks belong in the final hardening pulse unless an earlier pulse is
blocked by them.

## Flagship Exit Gates

1. The I-80 corpus record has no placeholders.
2. Every material number is cited, estimated, or held.
3. A corridor-specific gap artifact names the decisive problem and falsifier.
4. One bounded treatment is selected by comparison, not advocacy.
5. Seven Parliament voices and three editorial gates produce recorded decisions.
6. A compact packet regenerates from named commands.
7. Three external reviewer lanes are ready.
8. The flagship command bundle is reproducible from a clean clone.
