# Language Profiles

## Scope

Repo: ROUTE

VTRACE adoption scope: define validation profiles for ROUTE's Markdown,
Rust, generated artifacts, JavaScript browser tests, and git/submodule
workflow.

## Active Profiles

| Profile ID | Applicability | L0 | L1 | L2 |
|---|---|---|---|---|
| PROFILE-DOCS-001 | VTRACE docs, review records, release notes, public claim text. | `git diff --check -- docs\vtrace` or scoped docs diff check | VTRACE artifact inspection plus role-review matrix | release/review gate before public claim or validated status |
| PROFILE-RUST-001 | Rust crates, binaries, and workspace code. | targeted crate tests or `cargo test -q --workspace --lib --bins` | `cargo test -q` plus code-rigor review | `npm run check:l2` when integration, release, or downstream generated artifact use is affected |
| PROFILE-GEN-001 | Generated score, map, SLA, diagnostic, report, fixture, and evidence artifacts. | generation command named and expected outputs listed | reproducible regeneration and artifact diff inspection | downstream map/SLA/release gate or role review |
| PROFILE-JS-001 | Browser/game checks and Playwright surfaces. | syntax or scoped browser test when touched | `npm run check:game-browser` when game/browser surface changes | `npm run check:l2` before release or public claim |
| PROFILE-GIT-001 | ROUTE child repo commits and TRACKER submodule pointer workflow. | `git status --short` in ROUTE | child commit scope inspection and, when requested, TRACKER submodule diff | portfolio pointer update and TRACKER snapshot review |

## Tailoring Notes

| Profile ID | Local Override | Rationale | Reviewer |
|---|---|---|---|
| PROFILE-DOCS-001 | L2 is not required for docs-only VTRACE readiness unless public or downstream claims are promoted. | Avoid mixing process docs with unrelated integration state. | ROUTE maintainer |
| PROFILE-RUST-001 | `cargo clippy` is recommended for high-risk implementation but not yet a package-level script in ROUTE. | ROUTE currently exposes `check:l0`, `check:l1`, and `check:l2` through `package.json`. | software assurance reviewer |
| PROFILE-GEN-001 | Manual artifact edits require a hold label and replacement command plan. | Generated evidence must remain reproducible. | V&V reviewer |
| PROFILE-GIT-001 | TRACKER pointer updates are a separate package/commit after child repo work. | TRACKER is the portfolio snapshot repo. | portfolio maintainer |

## Gate

Decision: pass_with_risk

Rationale: language/tool profiles are defined, but actual command results remain
deferred to selected work-package execution.
