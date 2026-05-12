# ROUTE — Role Index

Four tiers of review roles. Read this before opening any role file.

---

## Parliament roles (9 voices)

Expert voices for corridor review. Adversarial by design — they plant incompatible stakes. The argument record is the output; consensus is not the goal.

| File | Voice | Primary tension |
|---|---|---|
| `parliament/eisenhower.md` | General Eisenhower | National defense + public unity vs. local/regional interest |
| `parliament/moses.md` | Robert Moses | Throughput + construction at scale vs. community impact |
| `parliament/foxx.md` | Anthony Foxx | Equity + access vs. efficiency framing |
| `parliament/freight-economist.md` | Freight Economist | NPV + commodity flow vs. social/equity costs |
| `parliament/traffic-engineer.md` | Traffic Engineer | Capacity + safety vs. cost and right-of-way |
| `parliament/climate-engineer.md` | Climate Resilience Engineer | Long-horizon risk vs. near-term cost |
| `parliament/rural-advocate.md` | Rural Advocate | Agricultural + rural access vs. metro-centric framing |
| `parliament/optimization-methodologist.md` | Optimization Methodologist | Reproducible objective/constraint formulation vs. hand-shaped choices |
| `parliament/schematic-cartographer.md` | Schematic Cartographer | Truthful schematic abstraction vs. visually convenient map cheats |

---

## Editorial roles (3 voices)

Quality gate before `validated` status. Run after parliament, not instead of it. Form gate only — not a substance gate.

| File | Role | Checks |
|---|---|---|
| `editorial/citation-auditor.md` | Citation Auditor | Every number has a traceable source |
| `editorial/scope-keeper.md` | Scope Keeper | Entry stays within its declared artifact type |
| `editorial/numeracy-checker.md` | Numeracy Checker | Units consistent; order-of-magnitude sanity; no arithmetic errors |

---

## Stakeholder roles (cross-cutting views)

These are not reviewers — they are lenses for understanding who the highway system serves and how. Used during corpus scoring and gap analysis to ensure the dimension scores reflect real user experience, not just engineering abstractions.

| File | Stakeholder | Primary concern |
|---|---|---|
| `stakeholders/long-haul-trucker.md` | Long-Haul Trucker | Hours of service, rest area quality, weigh stations, grades, weather, fuel |
| `stakeholders/regional-shipper.md` | Regional Shipper | Reliability, drayage costs, port access, last-mile connectivity |
| `stakeholders/rural-farmer.md` | Rural Farmer | Harvest transport windows, weight limits, grain elevator access, distance to corridor |
| `stakeholders/rural-resident.md` | Rural Resident | Access to healthcare, jobs, education; evacuation routes; distance to on-ramp |
| `stakeholders/local-official.md` | Local Official | Economic development, traffic through community, noise, right-of-way, tax base |
| `stakeholders/state-dot.md` | State DOT Planner | Federal funding match, maintenance burden, political feasibility, right-of-way |
| `stakeholders/intercity-traveler.md` | Intercity Traveler | Rest areas, fuel, services, signage, safety, predictability |
| `stakeholders/transit-dependent.md` | Transit-Dependent Traveler | Bus corridor access, park-and-ride, intercity coach stops, first/last mile |
| `stakeholders/freight-industry.md` | Freight Industry (ATA) | Regulatory environment, bridge weight limits, dimensional clearances, hours of service |
| `stakeholders/environmental-community.md` | Environmental Community | Noise, runoff, habitat fragmentation, air quality near corridors |

Stakeholder lenses are used primarily during:
- Corpus scoring (C2 Rural Connectivity, C3 Equity Access, D2 Multimodal Integration scores)
- Gap analysis (which gap type affects which stakeholders most severely)
- Interstate 2.0 feature selection (which features address which stakeholder needs)

---

## Panel reviewer roles (10 domain experts)

Academic peer review panel for ROUTE research papers. These are transportation domain specialists — NOT the plugin's ML/AI reviewer database. Stored in `.roles/panel-reviewer/` for local override. Used by `panel:publication review` for domain-appropriate peer review.

| File | Reviewer | Expertise |
|---|---|---|
| `panel-reviewer/R-T1.md` | Susan Hanson | Transport geography, spatial access, coverage methodology |
| `panel-reviewer/R-T2.md` | David Neumark | Rural/labor economics, causal inference, benefit-cost |
| `panel-reviewer/R-T3.md` | Robert Puentes | Federal highway policy, IIJA, implementation feasibility |
| `panel-reviewer/R-T4.md` | Lada Adamic | Network science, graph algorithms, spatial networks |
| `panel-reviewer/R-T5.md` | Angie Schmitt | Transportation equity, community impact, highway history |
| `panel-reviewer/R-T6.md` | Lily Elefteriadou | Traffic engineering, HCM, highway capacity |
| `panel-reviewer/R-T7.md` | Alan McKinnon | Freight economics, logistics, decarbonization |
| `panel-reviewer/R-T8.md` | Mikhail Chester | Infrastructure resilience, climate adaptation |
| `panel-reviewer/R-T9.md` | Ron Eberts | Rural access, agricultural logistics, USDA/RUCC |
| `panel-reviewer/R-T10.md` | Jarrett Walker | Transit planning, coverage-vs-ridership, multimodal |

**Paper-to-reviewer assignment guide:**
- Gap analysis (B-track): R-T1 (geographer), R-T2 (rural economist), R-T3 (policy), R-T4 (network), R-T5 (equity)
- Freight/throughput (C-track): R-T7 (freight), R-T6 (traffic eng), R-T2 (economist), R-T3 (policy), R-T8 (resilience)
- Resilience (D-track): R-T8 (resilience), R-T4 (network), R-T1 (geographer), R-T6 (traffic eng), R-T3 (policy)
- Design/investment (E-track): R-T3 (policy), R-T2 (economist), R-T7 (freight), R-T5 (equity), R-T9 (rural)
- Transit (F-track): R-T10 (transit), R-T5 (equity), R-T1 (geographer), R-T9 (rural), R-T3 (policy)
