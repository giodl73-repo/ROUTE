# Des Moines Diamond G2 Browser Plan

Scenario: `des-moines-diamond`  
Source phase: G1-B CLI loop  
Target phase: G2-A browser playable  
Primary fixtures:

- `data/game/des-moines-diamond-session-fixture.csv`
- `data/game/des-moines-diamond-state-fixture.json`

## Purpose

Build a map-first browser prototype that lets a player feel the T1/T1 topology lesson without hiding the ROUTE evidence labels.

The browser is not a landing page. The first viewport is the playable scenario board.

## First Screen

The first screen has four persistent regions:

| Region | Job | Must show |
|---|---|---|
| Map board | Make transfer topology visible | I-35 north/south, I-80 east/west, interchange core, I-235 local bypass, relay/rest node |
| Project rail | Choose investments under scarcity | Cost, crew, time, evidence label, protected failure mode |
| Track strip | Show pressure and resources | Budget, crews, political capital, public patience, operations capacity, evidence confidence |
| Evidence drawer | Keep proof honest | Scenario run, diamond analyzer, Iowa 511 sample, NPMRDS/PTI, standards proof ledger |
| Campaign atlas | Tie the tutorial board to the national map set | `maps/all-tiers.png`, `maps/i35.png`, `maps/beck-schematic.png` links gated by `data/map-atlas.csv` |

The map board owns the center of the viewport. Project and evidence controls are panels around it, not cards nested inside cards.

## Visual Grammar

| Element | Encoding |
|---|---|
| T1 approaches | Thick route strokes with route labels |
| Interchange core | Transfer node with fragility marker |
| Independent transfer paths | Separate visible connector lines, not wider versions of the same line |
| Closure stress | Warning overlay on the affected zone plus event-log row |
| Throughput | Route stroke width during pressure playback |
| Recovery | Time marker in after-action strip |
| Evidence level | Text badge plus icon/shape; never color alone |
| Publication status | Separate lock/unlock badge outside the operational score |

## Required Interactions

| Interaction | Control |
|---|---|
| Pick a project | Button on project row |
| Inspect evidence | Drawer toggle |
| Inspect atlas context | Campaign map links |
| Advance season | Primary action button |
| Replay pressure | Playback button with step markers |
| View after-action | Tab or report panel after score |

Buttons should use icons where obvious: play/step, lock, info, evidence/document, warning, check.

## State Contract

The browser should consume the same state shape written by `route game run-season`.

Initial fixture: `data/game/des-moines-diamond-state-fixture.json`

Session fixture: `data/game/des-moines-diamond-session-fixture.csv`

The browser may start with fixture playback before it writes state itself. Once interactive, browser actions should map back to CLI concepts:

| Browser action | CLI equivalent |
|---|---|
| Start scenario | `route game inspect des-moines-diamond` |
| Advance season | `route game run-season des-moines-diamond --event ... --project ...` |
| Score run | `route game score des-moines-diamond --log ... --details` |

## Playable Slice

G2-A first slice:

1. Load fixture state and fixture score.
2. Render Des Moines map board with completed connector.
3. Render track strip and evidence drawer.
4. Render publication hold separately from operational win.
5. Provide pressure playback for the closure result.
6. Render after-action score while keeping publication status separate. Done in the static prototype.

G2-A second slice:

1. Let the player choose projects for a season. Done in the static prototype.
2. Resolve season in browser using the same deterministic rules as G1. Partial for immediate project effects and source challenge.
3. Export or display a session log compatible with `route game score`. Done as the `CLI-compatible session log` panel and CSV download.
4. Link the playable board to the gated campaign map atlas. Done for national tiers, I-35 regional context, and relay schematic.

## Accessibility Gates

- No state depends on color alone.
- Every route and evidence badge has text.
- Keyboard can move through project buttons, drawer toggles, playback, and after-action tabs.
- Mobile layout keeps map visible first and moves project/evidence panels below it.
- Audio cues are optional and mirrored by visible event-log rows.

## Playwright Gates

Before G2-A promotion:

| Check | Viewport |
|---|---|
| Static fixture contract passes `docs/game/browser/check-des-moines-browser.ps1` | local |
| Playwright spec passes `npm run check:game-browser` | desktop and mobile |
| Scenario board renders nonblank | desktop and mobile |
| Map routes and connector are visible | desktop and mobile |
| Campaign atlas links resolve to gated map artifacts | desktop |
| Track strip text does not overlap | desktop and mobile |
| Evidence drawer opens and labels remain visible | desktop and mobile |
| Publication lock is visible apart from score | desktop and mobile |
| Pressure playback changes visible state | desktop |

## Open Design Questions

| Question | Current leaning |
|---|---|
| Framework | Use the repo's existing frontend stack if one appears; otherwise keep first prototype minimal |
| Map source | Keep the simplified topology board for play, then attach gated atlas links for national and regional context |
| State mutation | Browser-local for prototype; CLI-compatible session log for reproducibility |
| Audio | Silent-capable first; cue contract only |

## Done Definition

G2-A is done when a player can complete or replay one Des Moines run in the browser, see why topology matters, and see that publication remains locked even after an operational win.
