# ROUTE Module — Panel Revision Plan (Round 1)

Generated: 2026-05-07 | Module score: 7.4/10 (B+) | Target: 8.0+ (A-)

---

## PP1 — Blocking Items (must address before recheck)

### Critical path first: D.1 → B.3 → E.2

- [ ] **PP1.3** — D.1 composite metric resolution *(McKinnon 2/4)*
  - [ ] Normalize both SFHA and winter-closure components to expected annual lane-closure-hours per 100 miles (Option B)
  - [ ] Update config/scoring.toml with new D1 formula — tag as v1.3 amendment
  - [ ] Document under forward-only protocol in A.2 Section 6 (calibration history)
  - [ ] Re-run D1 scores for all 227 corridors under new formula
  - [ ] B.3: re-run compound exposure corpus with corrected D1 scores; verify 11 corridors still qualify

- [ ] **PP2.4** — B.3 Donner NPV waiting-cost correction *(Elefteriadou)*
  - [ ] Correct waiting cost rate from $225/hr → ~$91/hr (driver $89 + idle fuel $2)
  - [ ] Recalculate Donner tunnel 30-year NPV (expected: ~$11.6B from ~$15.8B)
  - [ ] Recalculate Gulf Coast I-10 NPV with corrected rate
  - [ ] Verify investment priority ordering is unchanged; note in B.3 conclusion
  - [ ] Update portfolio summary table in B.3 Section 5

- [ ] **PP1.1** — E.2 NPV arithmetic reconciliation *(Neumark, Puentes)*
  - [ ] Add benefit reconciliation table to E.2 Section 3
  - [ ] Show: each component's annual benefit × sum = $31.2B/yr → discounts to $298B at 7%/30yr
  - [ ] Ensure corrected B.3 Donner NPV and D.2 sensitivity midpoint are used as inputs
  - [ ] Verify managed lane ($12.7B) + diamond ($3.2B) + hardening ($3.9B) + missing links ($8.4B) + intermodal ($2.1B) + EV ($0.9B) = $31.2B/yr

### Parallel (independent of D.1 path):

- [ ] **PP1.2** — F.1/F.2 proximity-access reframing *(Walker, Schmitt)*
  - [ ] F.1: Add to abstract and Section 3: "geographic proximity (within 30 miles) — operational access requires local feeder transit not included in hub investment"
  - [ ] F.1: Add Limitations paragraph: hub increment cost does not include local feeder service ($500-800M estimated)
  - [ ] F.1: Revise 12.4M figure to "12.4M transit-dependent Americans within 30 miles of a potential hub" (not "served by")
  - [ ] F.2: Apply same reframing to 24M annual passenger estimate — note local feeder assumption
  - [ ] F.2: Add to Section 3: "demand model assumes local connecting service is available at T1 hubs"

- [ ] **PP1.4** — C.1 BPR extrapolation acknowledgment *(Elefteriadou 2/4)*
  - [ ] Add to C.1 Section 3 (Methods): note BPR calibrated for V/C ≤ 1.3; Bay Area at V/C 1.86 is extrapolated
  - [ ] State that BPR likely underestimates PTI at very high V/C (conservative estimate)
  - [ ] Add: "NPMRDS probe data for I-580/I-80 corridor confirms PTI ≥ 2.0 at peak, validating the order of magnitude"
  - [ ] C.2: Add note in Section 3 that C.1's Bay Area PTI is a lower-bound estimate

---

## PP2 — Important Items (address in parallel with PP1)

- [ ] **PP2.1** — F.1/F.2 EIT designation reframe *(Puentes)*
  - [ ] F.1 Section 7 and F.2 Section 7: replace "Essential Intercity Transportation (EIT)" with "a designation analogous to EAS (49 U.S.C. § 41731) requiring new authorization"
  - [ ] Add: "The closest existing authority is 49 U.S.C. § 5311(f) (FTA rural intercity bus)"
  - [ ] Frame as legislative recommendation, not existing mechanism

- [ ] **PP2.2** — F.2 stop-penalty correction *(Walker)*
  - [ ] Revise F.2 Section 3: add stop_penalty = N_stops × 8 min to travel time model
  - [ ] N_intermediate = ⌊distance / 150⌋ (one T1/T2 hub stop per 150 miles)
  - [ ] Recalculate all 12 corridor travel times (expected: effective avg drops from 62 to ~54-58 mph)
  - [ ] Revise Table in Section 4 with corrected times
  - [ ] Verify T1 bus still faster than Greyhound on all 12 corridors (it will be; Greyhound averages ~44 mph)
  - [ ] Update competitive advantage percentages

- [ ] **PP2.3** — Decarbonization thread in E.1/E.2 *(McKinnon)*
  - [ ] E.2 Section 2: add paragraph on EV-compatible design (DCFC ≤ 50 mi, Component 6 funding, 2030-35 EV curve)
  - [ ] E.1 Section 6 (Implications): add sentence connecting managed freight lanes to platooning/EV ecosystem
  - [ ] B.2 Section 8: add note on bottleneck cost growth as EV trucking scales (charging gaps at bottleneck locations)

- [ ] **PP2.5** — Cross-track dependency documentation *(Adamic, Hanson)*
  - [ ] B.3 Section 2: add \citep{ROUTE_D1} where D1 scores are first referenced
  - [ ] MODULE.md Track B: add note "B.3 requires D.1's D1 scores (cross-track dependency)"
  - [ ] E.2 Section 5: add explicit citations to D.2 closure cost model and B.3 compound exposure corpus

- [ ] **PP2.6** — Equity thread consistency *(Schmitt)*
  - [ ] F.1 Section 6: verify ACS B08201 (2022 vintage) cited
  - [ ] F.2 Section 6: same ACS citation
  - [ ] E.2 Section 6: add sentence: "The C3 alignment finding (r=0.68 between C3 score and transit-dependent HH density at hub locations; see F.1 Section 6) suggests the I2.0 equity benefit is concentrated in corridors already identified as economic opportunity priorities."
  - [ ] Verify r=0.68 coefficient is consistent across F.1 and MODULE.md

---

## Recheck Panel (after PP1 items addressed)

| Paper | Reviewers | Focus |
|---|---|---|
| E.2 | Neumark + Puentes | NPV arithmetic + legislative pathway |
| F.1 | Walker + Schmitt | Proximity reframe + EIT fix |
| F.2 | Walker | Stop-penalty correction + passenger estimate |
| C.1 | Elefteriadou | BPR acknowledgment adequate? |
| D.1 | McKinnon | Composite metric resolution satisfactory? |
| B.3 | Elefteriadou + McKinnon | Donner NPV corrected? |

---

## Estimated Score After Revisions

| Property | Current | After PP1+PP2 |
|---|---|---|
| Causal Chain | 7.5 | 8.0 (B.3 dependency documented; E.2 corrected) |
| No Weak Links | 7.0 | 8.0 (F track reframed; BPR acknowledged; D1 composite fixed) |
| Actionable Numbers | 8.0 | 8.5 (E.2 NPV reconciled; F.2 corrected) |
| **Module Score** | **7.4** | **8.2 (A-)** |
