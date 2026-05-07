# ROUTE — House Rules

## 1. Project Identity

ROUTE is a **research and design project for Interstate 2.0** — a data-driven upgrade plan for the US interstate highway system. The mission: score the existing system against a calibrated dimension pool, find the gaps (under-served corridors, missing links, bottlenecks), and design into them.

**The architectural bet** — borrowed from TIGRIS: score enough existing interstates on enough dimensions and the design space tells you its own structure. The gaps aren't invented; they're found. A corridor designed into a real gap is better evidence than one invented from first principles.

**The testable hypothesis**: There is a set of ≤20 missing or severely-underbuilt corridors that, if built or upgraded to Interstate 2.0 standards (managed freight lanes, shared transit facilities, intermodal integration, resilience hardening), would produce disproportionate gains in throughput, redundancy, and rural connectivity — and many align with what Eisenhower-era planners considered but didn't build.

**A rigorous null result is as valid as a positive one.** If the corpus shows the existing system is already near-optimal, that's the finding. Silent scope expansion to rescue a failing hypothesis is not acceptable.

Sibling projects: **REDIST** (`apportionment` — algorithmic redistricting), **CERES** (`ceres` — production economics), **TIGRIS** (`tigris` — board game factory), **LUCIA** (`lucia` — human chronicle). Borrows structural patterns from siblings; own rules apply here.

---

## 2. The E2E Pipeline

```
CORPUS (score existing interstates) → RUBRIC CALIBRATES → GAP MAP → CONCEPT → SCORE → PARLIAMENT → DESIGN → HANDOFF
```

1. **CORPUS** — Score existing interstate corridors against the 12-dimension candidate pool (`personas/axis-pool.md`). One file per corridor in `corpus/existing/`. This is the calibration phase — the rubric evolves from what actually differentiates real corridors.
2. **RUBRIC CALIBRATES** — After ~20 corridors scored, run an amendment pass: which dimensions show high variance (informative), which are correlated (redundant), which are all-high or all-low (not differentiating). Retire weak axes. Promote informative ones. Bump rubric version.
3. **GAP MAP** — Plot scored corridors in the dimension space. Find empty regions: high-freight corridors with no redundancy, high-population corridors with no multimodal integration, etc. These are the design targets. Output in `gaps/`.
4. **CONCEPT** — Propose a new or upgraded corridor targeting a specific gap. Brief document in `corpus/proposed/`.
5. **SCORE** — Score the proposed corridor against the calibrated rubric. Compare against existing corpus distribution.
6. **PARLIAMENT** — 7-voice adversarial expert review. Experts plant incompatible stakes. Argument is the output; consensus is not the goal.
7. **DESIGN** — Interstate 2.0 specification: corridor geometry, Interstate 2.0 features, economic case, phasing. In `design/`.
8. **HANDOFF** — Session snapshot to `docs/handoff/YYYY-MM-DD-<slug>.md`.

**Anchor rule**: One existing corridor must go through the full pipeline (corpus entry → calibration pass → gap map entry) before any proposed corridor is analyzed. One proposed corridor must go through parliament manually before any skill is built. YAGNI is the law.

---

## 3. Directory Conventions

| Directory / File | Purpose |
|---|---|
| `specs/` | Design specifications. Current: `specs/2026-05-06-route-design.md` (the source of truth). |
| `corpus/` | **THE STAR.** One markdown file per corridor. Schema in `corpus/SCHEMA.md`. Subdirs: `corpus/existing/` (scored existing interstates), `corpus/proposed/` (candidate new corridors). |
| `personas/` | Parliament voices and dimension pool. `personas/axis-pool.md` is the 12-dimension candidate pool + scoring ledger. `personas/parliament/` holds expert reviewer persona files. `personas/editorial/` holds the three editorial gate roles. |
| `gaps/` | Gap analysis outputs — regional under-service findings, bottleneck studies, missing-link candidates. |
| `design/` | Interstate 2.0 design proposals — corridor-level specs for new or upgraded routes. |
| `reviews/` | Parliament review outputs. Naming: `R{round}-{voice}-{corridor-slug}.md`. |
| `scoring/` | Scoring rubric (`scoring/RUBRIC.md`) and scoring ledger. |
| `data/` | `data/sources.md` — data source catalog. No raw data committed. |
| `research/` | Research papers. `research/papers/` (markdown), `research/publications/` (LaTeX, future). |
| `docs/` | Pipeline docs, style guide, session handoffs (`docs/handoff/`). |
| `TRACKER.md` | Project-level progress tracking. |
| `.claude/skills/` | Claude Code skills. All skills live here for discovery. |

---

## 4. Frontmatter Contract

Every generated file:

```yaml
---
name:
slug:
type: existing-corridor | proposed-corridor | gap-analysis | design-proposal | review | spec | plan
status: draft | reviewed | validated | deprecated
rubric_version: v1.0
author: <skill-slug or human>
created: YYYY-MM-DD
updated: YYYY-MM-DD
sources: []
# For corridor files only:
corridor:
  termini: ["", ""]
  states: []
  approx_miles: 0
  designation: ""        # e.g. "I-95" or "proposed-I-14"
  classification: trunk | connector | spur | proposed
---
```

Status lifecycle: `draft` → `reviewed` → `validated` → (`deprecated` | `superseded`).

---

## 5. Quality Bar

- Research-paper-level estimates. Order-of-magnitude traffic and economic figures with citations.
- Every number cited. An uncited number blocks promotion to `validated`.
- No procurement plans or vendor specs — conceptual design only.
- No hand-waving on economics. Marginal or negative NPV corridors are reported as such.
- Data sources declared. Every corridor entry names which source its numbers come from (`data/sources.md`).

---

## 6. Parliament Voices

Seven expert voices. Productive tensions are designed, not accidental. Read each persona file before writing any review. Persona files in `personas/parliament/`.

| Voice | Slug | Key question |
|---|---|---|
| General Eisenhower | `eisenhower` | "Does this serve national defense, economic unity, and the public interest at scale?" |
| Robert Moses | `moses` | "Can this actually be built — and will it handle the volume?" |
| Anthony Foxx | `foxx` | "Who gets left out, displaced, or underserved — and is that acceptable?" |
| Freight Economist | `freight-economist` | "What's the NPV, the commodity flow, the return on public capital?" |
| Traffic Engineer | `traffic-engineer` | "Does the geometry, capacity, and safety profile hold under real demand?" |
| Climate Resilience Engineer | `climate-engineer` | "What happens to this corridor in 2050 — flood, heat, fire, sea level?" |
| Rural Advocate | `rural-advocate` | "Does this connect the agricultural and rural economy, or serve only metros?" |

No voice is skipped. A good corridor survives all seven. A weak one collapses under one or two; the collapse is the finding.

---

## 7. Editorial Gate

Three editorial roles gate the `validated` status. Run after parliament, before promotion.

| Role | Slug | Checks |
|---|---|---|
| Citation Auditor | `citation-auditor` | Every number has a source; sources are traceable |
| Scope Keeper | `scope-keeper` | Entry stays within its declared type; no scope drift |
| Numeracy Checker | `numeracy-checker` | Unit consistency, order-of-magnitude sanity, no arithmetic errors |

---

## 8. Session Resume

When the user says **"continue"**, **"resume"**, or equivalent:

1. Read the latest handoff in `docs/handoff/` (sorted by filename `YYYY-MM-DD-<slug>.md`).
2. Confirm TRACKER and axis-pool state match the handoff's claim.
3. Report in one sentence: current rubric version + most recent corridor scored + top 1-3 next priorities.
4. Ask which priority to start on (or `go` / `1` for the top one).

End every substantive session: run `/route-handoff <slug>` to write a fresh resume point before context is cleared.

---

## 9. Skills (Planned)

Build when friction from doing the job manually is clear. YAGNI is the law.

| Skill | Purpose | Status |
|---|---|---|
| `route-score` | Score any corridor against the dimension pool; update scoring ledger | planned |
| `route-panel` | Run 7-voice Parliament review; produce stakes, argument, summary | planned |
| `route-gap-find` | Analyze scored corpus; identify empty regions in dimension space | planned |
| `route-design` | Draft a corridor design proposal targeting a named gap | planned |
| `route-handoff` | Snapshot session state to `docs/handoff/` | planned |

---

## 10. Forbidden Vocabulary

In corpus entries and design proposals: no "obviously needed," "critical gap," "long overdue," or any pre-judged framing before the score supports it. Claims must cite (a) dimension, (b) score, (c) corpus comparison. "This corridor scores 8.4 on Freight Intensity vs. a corpus mean of 5.1" beats "this is a critical freight corridor."

---

## proof — documentation linting

proof is the markdown QA tool for this repo. Binary lives at
`C:/src/target/debug/proof` (workspace build — run `cd C:/src && cargo build` once).

```bash
C:/src/target/debug/proof check .
```
