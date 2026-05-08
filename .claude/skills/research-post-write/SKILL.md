---
name: research-post-write
description: "Post-writing validation pipeline for ROUTE research papers. Reads written LaTeX sections, runs consistency check (numbers match across abstract/sections/tables), contract check (paper delivers what plan.md promised), and referee simulation (3 hostile reviewers from transport/economics/policy). Produces a pre-panel-review checklist. Adapted from apportionment research-post-write for ROUTE structure."
allowed-tools: [Read, Write, Glob, Grep]
param_set: lean
---

You are running /research-post-write for: {{topic}}

Run the full post-writing validation pipeline for a ROUTE research paper. Reads the written
LaTeX sections, checks consistency and contract, simulates hostile peer review, and produces
a pre-panel-review checklist.

---

## PHASE 1 — READ THE PAPER

Find and read all section files:
```
Glob: research/publications/*{{topic}}*/sections/*.tex
```
Also read:
- `research/publications/[dir]/main.tex` — abstract
- `research/publications/[dir]/plan.md` — the contract
- `research/MODULE.md` — the quantification contract

Extract:
- **Track**: A-F and theme
- **Key claims**: top 3 quantitative claims in the paper
- **Primary number**: the quantification contract number from MODULE.md
- **Venue**: target journal

Print:
```
Paper: {{topic}}
Sections found: [list section files]
Plan found: [yes/no]
Track: [letter — theme]
Venue: [journal]
Key claims:
  1. [claim + section where it appears]
  2. [claim + section]
  3. [claim + section]
Primary number (from MODULE.md contract): [X]
Paper's stated primary number: [Y]
Match: [YES / NO — resolve before panel]
```

---

## PHASE 2 — QUANTITATIVE CONSISTENCY CHECK

Extract every quantitative value from the paper into a registry.
Check that the same value appears consistently across: Abstract, Introduction, Results, Conclusion.

| Q-ID | Quantity | Abstract | §Intro | Table | §Conclusion | Consistent? |
|------|----------|---------|--------|-------|-------------|-------------|
| Q-01 | Donner capacity (vpd) | 91,200 | 91,200 | 91,200 | 91,200 | PASS |
| Q-02 | PTI on I-80 | 1.86 | 1.86 | 1.86 | 1.86 | PASS |
| Q-03 | Annual reliability cost | $8.2B | $5.7B+$2.5B | $8.2B | $8.2B | WARN (decomposition) |

Also check:
- BPR limitation caveat — acknowledged where V/C > 1.3?
- Rubric version tags — are scores labeled with v1.2 or v1.3?
- Estimated values — marked with appropriate hedge language?
- Citation keys — do all `\citep{}` keys exist in the bib file?

Print:
```
CONSISTENCY: [PASS / N warnings / N failures]
P1 (must fix): [list]
P2 (should fix): [list]
P3 (minor): [list]
```

---

## PHASE 3 — CONTRACT CHECK

The plan.md and MODULE.md quantification contract are the contract.
For each promise in plan.md, check whether the paper delivers it.

| Promise (from plan.md) | Paper section | Delivered? | Gap |
|------------------------|---------------|-----------|-----|
| Donner Pass binding at 91,200 vpd | §4 NY-LA | Yes, §4.2 | ✓ |
| PTI model with BPR formula | §3 Methods | Yes, Eq. 1-3 | ✓ |
| SLA window comparison table | §6 I2.0 | Yes, Table 3 | ✓ |
| Incident simulation for Dallas | §6 | Mentioned but not quantified | ✗ |

Print:
```
CONTRACT: [PASS / PARTIAL / FAIL]
Promises kept: [N/M]
Gaps: [list items plan.md promised that paper doesn't deliver]
MODULE.md primary number delivered: [YES / NO]
```

---

## PHASE 4 — REFEREE SIMULATION

Select 3 referees appropriate for ROUTE's research. Choose from:

**R-Traffic** (Transportation Research Part A/B archetype — Lily Elefteriadou style):
Focus: HCM consistency, capacity formula correctness, BPR calibration range, V/C computation.
Hostile to: BPR extrapolation above V/C 1.3, capacity claimed without lane count data, PTI estimates not validated against probe data.

**R-Economics** (Journal of Economic Perspectives / Transportation Research Part E archetype — David Neumark style):
Focus: causal identification, net vs gross cost calculations, counterfactual validity, sensitivity analysis.
Hostile to: point estimates without bounds, causality claims from correlational data, job creation without displacement analysis.

**R-Policy** (Transport Policy / Transportation Research Record archetype — Robert Puentes style):
Focus: implementation feasibility, legislative authority, budget horizon, IIJA alignment.
Hostile to: recommendations requiring new statutory authority not flagged, cost estimates without comparable project benchmarks, timeline claims ignoring NEPA/EIS.

**R-Equity** (Transportation Research Part A / Journal of Transport Geography — Angie Schmitt / Susan Hanson style):
Focus: distributional impacts, who bears costs, community displacement, transit-dependent populations.
Hostile to: freight benefits without mentioning community impact, equity claims without ACS data, no-vehicle household access unaddressed.

**R-Network** (Transportation Science / PNAS — Lada Adamic style):
Focus: graph algorithm correctness, scalability, reproducibility, dataset completeness.
Hostile to: algorithms without complexity analysis, partial graph results generalized to national scale, TIGER data limitations not acknowledged.

Select the 3 most appropriate for this paper. Produce for each:
```
REFEREE [N] — [archetype]
Recommendation: [Accept / Major Revision / Reject]

SUMMARY: [2-3 sentence reaction]

MAJOR CONCERNS:
[I-NN] [specific issue with section reference]

MINOR CONCERNS:
[list]
```

Issue IDs continuous (I-01, I-02, ...).

---

## PHASE 5 — ABSTRACT CHECK

Read main.tex abstract. Evaluate:
- Does it state the primary quantitative result (the MODULE.md contract number)?
- Is the method described in one sentence?
- Is the policy/investment implication stated?
- Is the paper's position in the track chain clear?
- Word count (target: 150-200 words for ROUTE papers)

Print:
```
ABSTRACT: [word count] words
Primary result stated: [YES/NO — quote it]
Method named: [YES/NO]
Policy implication: [YES/NO]
Track chain position: [YES/NO]
```

---

## PHASE 6 — INTERNAL CROSS-PAPER CONSISTENCY

Check that numbers cited from other ROUTE papers match what those papers actually say:
- If citing C.1 Donner PTI: does C.1's section 4 actually say 1.86?
- If citing B.3 Donner NPV: does B.3's section 3 say $12.1B (corrected) or $15.8B (old)?
- If citing E.1 managed lane NPV: does E.1 say $101B or $115B (pre-correction)?

Common traps after the B.3 Donner NPV correction (waiting cost rate $225→$91/hr):
- B.3 NPV = $12.1B (CORRECT); any paper citing B.3 should use $12.1B
- E.2 NPV reconciliation: $246B-$298B range (post-correction)

Print:
```
CROSS-PAPER CONSISTENCY:
  Papers cited: [list]
  Values cross-checked: [N/M]
  Stale citations (pre-correction): [list]
```

---

## PHASE 7 — PRE-PANEL CHECKLIST

```
═══════════════════════════════════════════════════════
POST-WRITE COMPLETE: {{topic}}
═══════════════════════════════════════════════════════

Validation results:
  Consistency:       [PASS / N issues]
  Contract:          [PASS / N gaps]
  Referee sim:       [likely decision]
  Abstract:          [word count] words, [primary number stated / missing]
  Cross-paper:       [PASS / N stale citations]

P1 blockers (fix before panel review):
[I-NN] [description] → [specific fix]

P2 items (should fix):
[I-NN] [description] → [fix]

P3 items (optional polish):
[list]

PRE-PANEL CHECKLIST:
□ All P1 consistency failures resolved
□ MODULE.md primary quantitative contract delivered in paper
□ BPR extrapolation acknowledged where V/C > 1.3
□ Net vs gross cost clearly stated (not conflated)
□ All \citep{} keys exist in references.bib
□ Cross-paper citations use corrected values (B.3 NPV = $12.1B, E.1 = $101B)
□ Rubric version tagged (v1.2 or v1.3) on all score citations
□ Abstract states primary quantitative result
□ Referee P1 blockers addressed

VERDICT: [READY FOR PANEL / FIXES REQUIRED]
Fixes required: [N]
Next: run /panel:publication review [paper-slug]
═══════════════════════════════════════════════════════
```

Write checklist artifact to: `research/publications/[paper-dir]/POST-WRITE-CHECK.md`
