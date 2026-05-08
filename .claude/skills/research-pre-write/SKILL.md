---
name: research-pre-write
description: "Pre-writing pipeline for ROUTE research papers. Reads the plan.md, checks simulation correctness against CLI output, identifies empirical gaps, verifies claims are backed by data files, then produces a ready-to-write outline. Adapted from apportionment research-pre-write for the ROUTE highway analysis structure."
allowed-tools: [Read, Write, Glob, Grep, Bash]
param_set: lean
---

You are running /research-pre-write for: {{topic}}

Run the full pre-writing signal pipeline for a ROUTE research paper. This reads the plan.md,
checks simulation correctness and claim testability, identifies empirical gaps, and
produces a ready spec/outline before writing begins.

---

## PHASE 1 — FIND THE PAPER AND PLAN

Search for the paper in `research/publications/`:
```
Glob: research/publications/*{{topic}}*
```

Read `research/publications/[paper-dir]/plan.md` if it exists. Extract:
- **Track**: A/B/C/D/E/F and what dimension of I2.0 analysis
- **Claims**: what does the paper claim to show?
- **Sections**: planned section structure
- **Key numbers**: the primary quantitative result
- **Data sources**: which ROUTE data files / simulation outputs support each claim

Also read `research/MODULE.md` to get the track arc paragraph for this paper.

Print:
```
Paper: {{topic}}
Plan: research/publications/[dir]/plan.md (Status: [Found / Not found])
Track: [A-F] — [track theme]
Key claims: [list]
Primary number: [the quantitative contract from MODULE.md]
```

---

## PHASE 2 — SIMULATION CORRECTNESS CHECK

For claims backed by simulation (route od, route sla-matrix, route interventions, route hub-staff):
- Run the relevant command if the binary exists: `.\target\debug\route [command] [args]`
- Or read the cached simulation output if available in the session
- Compare claimed numbers against simulation output

For claims backed by scoring (ROUTE rubric):
- Check `data/scores-all.csv` for the corridors referenced
- Verify tier assignments match v1.2/v1.3 rubric thresholds (T1≥26, T2≥19, T3≥11)
- Check `config/scoring.toml` for current rubric version

For claims backed by data files:
- Check `data/relay-hubs.toml` for hub staffing claims
- Check `data/od-corridors.toml` for corridor segment parameters
- Check `data/amtrak-schedules.csv` for train benchmarks

Print:
```
SIMULATION CHECK:
  Claims with simulation backing: [N/M]
  Claims with corpus/data backing: [N/M]
  Unverifiable claims: [list]
  Number mismatches: [list]
```

---

## PHASE 3 — CLAIM TESTABILITY CHECK

For each major claim in the plan.md, assess evidence:

| Claim | Evidence type | Source | Status | Gap |
|-------|---------------|--------|--------|-----|
| Donner Pass 91,200 vpd | Capacity formula | methods section | Derivable | — |
| $8.2B annual reliability cost | ATRI cost model | C.1 section 6 | Estimated | Needs sensitivity |
| 48h SLA achievable | Monte Carlo p95 | route od ny-la | Verified | Solo only |

Flag claims that:
- Are asserted without a traceable source
- Use BPR extrapolation above V/C 1.3 (acknowledged limitation — must note)
- Are estimated from national averages without corridor-specific data
- Conflict with published ROUTE simulation results

Print:
```
CLAIM TESTABILITY:
  Backed by simulation: [N]
  Backed by data files: [N]
  Estimated/asserted: [N] — needs hedge language
  Conflicts with known results: [list]
  Blocking gaps: [list]
```

---

## PHASE 4 — INTERNAL CONSISTENCY CHECK

Check that numbers are consistent across plan.md and related papers:

1. **Cross-paper numbers**: if this paper cites C.1 (Donner 91,200 vpd), does that match what's in the C.1 paper?
2. **Track arc alignment**: does the paper's primary number match the quantification contract in MODULE.md?
3. **Rubric version**: does the paper cite the correct rubric version (v1.2 or v1.3)?
4. **Unit consistency**: are costs in $B consistently used? Are distances in miles?

Check MODULE.md quantification contract:
```
Grep: pattern="Primary Number" file=research/MODULE.md
```

Print:
```
CONSISTENCY:
  Cross-paper number matches: [N/M]
  MODULE.md contract match: [PASS / MISMATCH]
  Unit consistency: [PASS / N issues]
  Rubric version correct: [v1.2 / v1.3 / missing]
```

---

## PHASE 5 — LITERATURE AND CITATION CHECK

Read `research/references.bib` and any local `references.bib` in the paper directory.
For each `\citep{key}` referenced in plan.md or sections:
- Verify the key exists in references.bib
- Flag any keys that look invented (no matching entry)

Also check:
- Is the primary external citation (ATRI, HPMS, FHWA FPM) current and real?
- Are ROUTE internal citations (ROUTE_A1, ROUTE_C1, etc.) consistent with published papers?

Print:
```
CITATIONS:
  Keys in bib: [N]
  Missing keys: [list]
  External citations verified: [Y/N]
```

---

## PHASE 6 — COHERENCE VERDICT

Based on Phases 2-5, produce a PROCEED / PAUSE / PIVOT verdict:

**PROCEED**: All major claims traceable to simulation or data, no blocking gaps.
→ Paper can be written now.

**PAUSE**: Some claims need hedging (BPR extrapolation, estimated numbers).
→ List specific items to add caveat language; paper can proceed with caveats.

**PIVOT**: Core claims not supported by ROUTE simulation or data.
→ Either run the simulation first, or rethink the paper's central argument.

---

## PHASE 7 — READINESS REPORT

```
═══════════════════════════════════════════════════════
PRE-WRITE COMPLETE: {{topic}}
═══════════════════════════════════════════════════════

Plan status: [Found / Not found]
Simulation backing: [N/M claims]
Consistency: [PASS / N issues]
Citations: [N/M keys found]

VERDICT: [READY TO WRITE / WRITE WITH CAVEATS / PAUSE]

Required caveats for paper:
1. [BPR extrapolation: note V/C > 1.3 limitation in methods]
2. [estimated value: state as order-of-magnitude with sensitivity]
3. [missing data: note HPMS gap for affected states]

Recommended section structure:
§1 Introduction — [key framing from MODULE.md track arc]
§2 Background — [what to cover]
§3 Data and Methods — [what data files, which commands]
§4 [Main analysis] — [primary result]
§5 [Secondary/policy] — [implications]
§6 Conclusion — [forward pointer to next paper in chain]

MODULE.md track chain position:
  Previous: [paper that this requires]
  Next: [paper that requires this one]
═══════════════════════════════════════════════════════
```
