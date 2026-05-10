# Des Moines Diamond G0 Panel

Scenario: Des Moines Diamond  
Scenario version: G0 v0.2  
Evidence reviewed:

- `docs/game/des-moines-diamond-g0.md`
- `docs/game/des-moines-diamond-playtest.md`
- `docs/game/des-moines-diamond-blind-playtest-001.md`
- `docs/game/des-moines-diamond-blind-playtest-002.md`
- `docs/game/des-moines-diamond-blind-playtest-003.md`
- `docs/game/des-moines-diamond-playtest-synthesis.md`
- `docs/game/des-moines-diamond-amendments.md`

Decision: G0-B pass; G1-A implementation may begin. Hold G0-C until a human blind playtest or explicit acceptance of simulated evidence.

## Summary

The scenario is paper-playable and the intended topology aha landed in all three simulated passes. The v0.2 amendments fixed the biggest playability problems from the first pass: unclear budget failure, premature `k-connectivity` language, evidence acquisition feeling arbitrary, and tutorial drag after learning.

The panel does not grant publication proof. It grants game-prototype readiness.

## TIGRIS Review

Focus: board-game clarity, tradeoffs, promotion readiness.

Scores:

| Dimension | Status | Note |
|---|---|---|
| Tension | Pass | Closure, budget, publication lock, and political pressure create real stakes |
| Interaction | Pass | Events push back even in solo play |
| Game-spectrum | Pass | Rules are learnable but nontrivial |
| Experiential | Pass | Player feels fragility rather than reading about it |
| Range | Hold | Replayability is narrow until more event combinations are tested |
| Accessibility | Pass | v0.2 language ladder helps |
| Texture | Pass | Budget, crews, patience, and evidence pull in different directions |

Finding: begin CLI implementation, but do not broaden the campaign until Des Moines has a deterministic score engine.

## HUNT Review

Focus: reveal, fairness, aha.

Findings:

- The forced closure is fair in retrospect because the evidence drawer and warning copy point at the transfer.
- The one-aha contract is clean: capacity is not topology.
- The hint ladder is now better after replacing early `k-connectivity` language.
- The early tutorial end condition is a good HUNT-style escape hatch: do not keep teaching after the puzzle is solved.

Risk: optimization-minded players may identify the connector before the closure. That is acceptable if the after-action still requires them to explain why it works.

## ASPECT Review

Focus: screen grammar and visual truth.

Findings:

- The scenario board has clear visual jobs: choose, stress, replay, explain.
- Evidence labels must be first-class in CLI output, not only browser UI.
- The pressure playback needs exact before/after values once implemented.
- Color cannot carry evidence status alone in G2.

CLI implication: `route game inspect` should print evidence status beside each project and gate.

## PROSE Review

Focus: player-facing language.

Findings:

- "Independent transfer paths" is the right first phrase.
- `k-connectivity` should be treated as the engineering name after the player has the concept.
- Publication lock copy improved: it now names missing observed closure evidence rather than a stale analyzer mismatch.
- "Validated evidence rejected" is accurate but harsh; CLI should phrase it as "validated evidence unavailable: no observed artifact."

Copy requirement for G1: every rejected action gets a reason sentence.

## SCORE Review

Focus: rhythm, pacing, audio/tempo.

Findings:

- v0.2 tutorial ends at season 4 or 5, which gives the scenario a better arc.
- Silence/decision space remains important; do not over-animate the first CLI/browser version.
- Distinct state changes are now clear enough for future cues: closure, connector complete, source request, validation unavailable, publication locked.

Implementation implication: event-log output should have stable event names, which future audio can mirror.

## QUEST Review

Focus: session logs, consequence, campaign continuity.

Findings:

- The session log is doing real work. Keep it append-only.
- The budget crisis amendment is necessary for campaign continuity; debt cannot be vague.
- Evidence-first play is a valid player style and should be preserved as a partial route.
- The campaign can remember whether the player solved operational resilience before evidence, or evidence before resilience.

Implementation implication: CLI state should include flags for `connector_package_complete`, `source_requested`, `validated_evidence_available`, and `fiscal_crisis`.

## Required G1-A Changes

| Requirement | Source |
|---|---|
| Implement `route game scenarios` | CLI design |
| Implement `route game inspect des-moines-diamond` | CLI design |
| Include v0.2 cards, not v0.1 cards | Amendment log |
| Print publication gate separately from operational status | Playtest synthesis |
| Use independent-transfer-path language before `k-connectivity` | PROSE/HUNT |
| Include action rejection reasons in future `run-season` | PROSE/QUEST |

## Held Issues

| Issue | Reason |
|---|---|
| G0-C promotion | Needs human blind playtest or owner acceptance of simulated evidence |
| Campaign persistence rules | Wait until `run-season` exists |
| Audio design | Wait until browser prototype |
| Full T1/T1 anchor set | Separate Milepost 4 data task |

## Decision

Proceed to G1-A implementation for:

- `route game scenarios`
- `route game inspect des-moines-diamond`

Do not implement `run-season` until the inspect output proves the G0 v0.2 scenario is represented faithfully.

