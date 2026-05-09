# ROUTE Spec Index

Start here when deciding which document owns a claim.

| Document | Owns | Use when |
|---|---|---|
| `docs/SYSTEM_PLAN.md` | Living roadmap, Milepost phases, roles, truth labels, forward plan | You need the current operating plan |
| `specs/2026-05-06-route-design.md` | Core ROUTE method: corpus, dimensions, gap map, parliament, design proposals | You need the conceptual process |
| `specs/2026-05-06-route-rust-architecture.md` | Rust workspace architecture, CLI contracts, data model, output formats | You need implementation boundaries |
| `specs/2026-05-06-interstate-2-design.md` | Interstate 2.0 feature set, investment thesis, simulation toolkit, transit integration | You need the national design framework |
| `specs/2026-05-06-tier-standards.md` | T1/T2/T3/T4 tier standards and service expectations | You need tier definitions |
| `research/MODULE.md` | Tracks A-F, paper chain, quantification contracts, review history | You need the research program |
| `.roles/ROLE.md` | Parliament, stakeholder, editorial, and panel-review role index | You need review gates or role selection |
| `TRACKER.md` | Current status board | You need the live project state |

---

## Ownership Rules

1. If a claim describes what ROUTE is trying to do, update `specs/2026-05-06-route-design.md`.
2. If a claim describes how the Rust system works, update `specs/2026-05-06-route-rust-architecture.md`.
3. If a claim describes Interstate 2.0 as a build program, update `specs/2026-05-06-interstate-2-design.md`.
4. If a claim changes the roadmap, phase theme, or done criteria, update `docs/SYSTEM_PLAN.md` and `TRACKER.md`.
5. If a claim appears in a paper, make sure `research/MODULE.md` still names the dependency and quantification contract.
6. If a claim requires judgment, attach the relevant `.roles` pass before calling it validated.

---

## Claim Status Vocabulary

Use the same status words across specs, README, CLI help, and papers:

| Status | Meaning |
|---|---|
| Implemented | Code runs end-to-end and is testable |
| Heuristic | Code runs but uses a proxy, partial data, or simplified model |
| Stub | Interface exists but the analysis is not real yet |
| Planned | Specified but not implemented |
| Deprecated | Historical and no longer a current claim |

When in doubt, mark a claim lower. ROUTE is stronger when it is honest about what is measured versus what is proposed.

