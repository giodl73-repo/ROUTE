---
status: draft
kind: how-to
owner: route-cli
truth_label: implemented / heuristic
---

# Run the ROUTE Requirement-to-Refinement Demo

## Purpose

This demo shows the current ROUTE loop that matters for funders, analysts, and
maintainers:

```text
requirement -> service promise -> generated artifact -> evidence label -> next refinement
```

The current demo does not prove an official construction program, operating
guarantee, or final corridor ranking. It proves that ROUTE can turn a stated
requirement into inspectable artifacts, gates, and evidence asks that either
pass, produce a candidate refinement, or stay held.

## Demo story

Use this narrative when presenting the commands:

> A state, carrier, or community asks: "Can this network support a reliable
> regional freight promise without hiding delivery, access, or evidence risk?"
>
> ROUTE converts that question into stop spacing, service-class, diagnostic,
> candidate, and promotion artifacts. If the current network can support the
> service promise, the gate passes. If not, ROUTE produces the next thing to
> inspect rather than pretending the map is done.

## Before running

Run commands from the ROUTE repo root:

```powershell
Set-Location C:\src\TRACKER\repos\applied-systems\route
```

Create a local artifact folder. The folder is intentionally under `target\` so
demo outputs do not become source artifacts by accident.

```powershell
New-Item -ItemType Directory -Force target\demo | Out-Null
```

## 1. Show the current service promise surface

Generate the stop/SLA surface:

```powershell
cargo run -q -p route -- stop-sla-surface --output target\demo\beck-stop-sla-demo.csv
```

Explain it this way:

- the output is a machine-readable service surface;
- the promise windows are planning targets, not verified guarantees;
- rows carry heuristic/evidence posture instead of pretending the surface is
  proof-grade.

Then summarize and gate the largest stop gaps:

```powershell
cargo run -q -p route -- stop-sla-summary --input target\demo\beck-stop-sla-demo.csv --top 8 --gate-max-gap 250
```

Presenter line:

> The requirement is no longer a comment. It becomes a threshold, a recurring
> gap list, and a pass/fail gate.

## 2. Show how a requirement becomes a refinement docket

Ask: "What would need inspection if the service promise is too stretched?"

Generate the candidate docket:

```powershell
cargo run -q -p route -- stop-sla-candidates --input target\demo\beck-stop-sla-demo.csv --output target\demo\beck-stop-sla-candidates-demo.csv --target-gap 250 --top 8 --gate --gate-no-algorithmic
```

Expected interpretation:

- if the gate passes with candidate rows, current inspected recurring gaps have
  named review targets instead of anonymous midpoint guesses;
- if the gate passes with zero rows, the current threshold has no open candidate
  docket and the next demo should lower the threshold or use a scenario fixture;
- if the gate fails, the failure is the next evidence/refinement task;
- candidate rows are not construction recommendations until source, role, and
  delivery review close.

Convert the candidate docket into an append-ready promotion scaffold:

```powershell
cargo run -q -p route -- stop-sla-promotions --input target\demo\beck-stop-sla-candidates-demo.csv --output target\demo\beck-stop-sla-promotions-demo.csv --gate
```

Presenter line:

> ROUTE does not jump from "problem" to "build this." It creates a promotion
> scaffold that still needs source-backed review. When the scaffold is empty, it
> is evidence that the current threshold has no open promotion rows.

Optional stress test for demos:

```powershell
cargo run -q -p route -- stop-sla-candidates --input target\demo\beck-stop-sla-demo.csv --output target\demo\beck-stop-sla-candidates-225-demo.csv --target-gap 225 --top 5 --gate
```

This deliberately tightens the rhythm below the current passing threshold. In
the current artifact set, it surfaces named review candidates such as South
Bend/Elkhart and Montgomery while also exposing algorithmic midpoint fallbacks.
That is useful in a live story: ROUTE shows which refinements have named
evidence paths and which ones must stay held.

## 3. Show how T2 operating requirements become diagnostics

Generate a T2-only service map:

```powershell
cargo run -q -p route -- map BECKT2ONLY --output target\demo\beck-schematic-t2-only-demo.png
```

Generate the corresponding diagnostics:

```powershell
cargo run -q -p route -- beck-t2-diagnostics --output target\demo\beck-t2-diagnostics-demo.csv --gate
```

Explain it this way:

- the map is a schematic service artifact, not proof of upgrade readiness;
- the diagnostics separate compact services, transfer spines, long connectors,
  dense-transfer review, and other service-class issues;
- operators can now say which classes need more evidence or different treatment.

Generate the service-class and qualification-action contracts:

```powershell
cargo run -q -p route -- beck-t2-service-standards --output target\demo\beck-t2-service-standards-demo.csv --gate
cargo run -q -p route -- beck-t2-qualification-actions --output target\demo\beck-t2-qualification-actions-demo.csv --gate
```

Presenter line:

> The requirement changes the artifact contract: what counts as a connector,
> transfer spine, duplicate-service keep, merge review, or demotion review is
> visible in data.

## 4. Show the review/evidence gates

Run the current fast confidence gate:

```powershell
npm run check:l0
```

For the command bundle represented in the e2e tests:

```powershell
cargo test -q -p route --test e2e_cli
```

If a public or release claim is being prepared, run the documented L1/L2 profile
from `docs/vtrace/VERIFICATION.md`. L2 currently includes browser/game tooling
that may be environment-sensitive; do not promote browser or public-release
claims unless that gate is closed or explicitly scoped out.

## What the demo proves

| Claim | Current posture |
|---|---|
| ROUTE can generate stop/SLA artifacts from current repo data. | implemented |
| ROUTE can summarize and gate stop spacing against a planning threshold. | implemented / heuristic |
| ROUTE can produce candidate and promotion dockets for recurring gaps. | implemented / source-needed |
| ROUTE can generate T2 schematic maps and diagnostics. | implemented / schematic |
| ROUTE can expose service-class and qualification-action contracts. | implemented |
| ROUTE can turn requirements into final investment decisions. | held |
| ROUTE can prove SLA operating guarantees or construction readiness. | held |

## What to show funders

Do not lead with command names. Lead with the loop:

1. A leader names a service requirement.
2. ROUTE turns it into thresholds, diagnostics, and generated artifacts.
3. The system shows what passes, what fails, and what needs source review.
4. State, industry, community, and environmental inputs can change the next
   artifact instead of becoming static meeting notes.

That is the hidden gem: ROUTE makes the Interstate 2.0 vision refinable.

## Next demo gap

The next stronger demo should add a small scenario fixture where a state,
industry, or community requirement changes one selected service option and
records the before/after evidence label. Until that exists, present this demo as
the current artifact/gate path, not a full recursive optimizer demonstration.
