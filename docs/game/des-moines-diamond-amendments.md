# Des Moines Diamond Amendment Log

This log is forward-only. Old playtests keep the rules they used; new playtests use the latest accepted amendment version.

Current scenario version: G0 v0.2  
Prior version: G0 v0.1  
Source evidence: `docs/game/des-moines-diamond-blind-playtest-001.md`

## Amendment Protocol

| Status | Meaning |
|---|---|
| Candidate | A playtest finding worth considering |
| Accepted | Rule/copy/card change applies to future playtests |
| Held | Finding needs another occurrence before changing rules |
| Declined | Finding is understood but not adopted |

Two repeated findings create an amendment candidate. A high-severity single finding can be accepted when it blocks playability or evidence honesty.

## v0.2 Accepted Amendments

| ID | Source finding | Decision | Rule change |
|---|---|---|---|
| DM-A001 | Budget below zero was unclear | Accepted | Budget below zero triggers fiscal crisis and operational failure unless a scenario explicitly enables debt |
| DM-A002 | `k-connectivity` was opaque before the aha | Accepted | Player-facing copy says "independent transfer paths" first; `k-connectivity` appears after the player has seen the failure |
| DM-A003 | Evidence acquisition felt like a tax | Accepted | Split evidence work into `Source request` and `Validated evidence`; source request raises confidence but never unlocks publication by itself |
| DM-A004 | Tutorial naturally ended after the aha | Accepted | G0 tutorial may end early after the aha, after-action score, and publication gate are resolved |

## v0.2 Rule Changes

### Fiscal Crisis

Budget may not go below zero in G0 v0.2.

If a player cannot pay a project cost:

- The project cannot start.
- If an event forces a cost the player cannot pay, mark `fiscal_crisis=true`.
- Fiscal crisis sets Budget Discipline to 0 and caps the operational result at Partial Win.

### Language Ladder

Before the first closure:

- Use "transfer path" and "backup path."
- Avoid `k-connectivity`.

After the first closure:

- Use "independent transfer path."
- Explain that `k` is the count of independent paths.

After connector completion:

- It is safe to say "`k-connectivity` is the engineering version of what the player just built."

### Evidence Card Split

Replace `Evidence acquisition` with two distinct cards:

| Card | Cost | Effect | Publication impact |
|---|---:|---|---|
| Source request | 1 | Raises evidence confidence by 1 and names the missing source | Does not unlock publication |
| Validated evidence | 2 | Requires source request; raises evidence confidence by 2 | Can unlock publication only if the scenario has a matching observed-data artifact |

For Des Moines G0 v0.2, `Validated evidence` cannot unlock publication yet because empirical closure history is still missing. It should make the blocker clearer, not disappear.

### Tutorial End Condition

The tutorial can end before season 10 if all are true:

- The player completes or selects the diamond connector package.
- The after-action report has been scored.
- The player can explain the difference between capacity and independent transfer paths.
- Publication status is explicitly locked or unlocked.

The campaign version can still require the full 10-season horizon.

## v0.2 Expected Effects

| Risk | Expected effect |
|---|---|
| Players overspend | Reduced; fiscal crisis is explicit |
| Players miss topology lesson | Reduced; language ladder bridges the concept |
| Evidence feels arbitrary | Reduced; source request and validation are separated |
| Tutorial drags after learning | Reduced; early end condition allows clean stop |

## Held Findings

None yet.

## Declined Findings

None yet.

