---
name: route-plan
description: "Create ROUTE wave or pulse plans with mission, artifacts, governing roles, gates, and non-goals."
tags: [route, plan, pulse, gates]
---

# route-plan

Use this skill when drafting a wave, milestone, pulse, checklist, or execution
plan.

## Pulse Plan Format

Write pulse plans to `waves/{active}/plans/pulse-NN.md` with:

- frontmatter: `wave`, `pulse`, `date`, `status`, `depends_on`,
  `governing_roles`
- mission
- scope inventory
- deliverables checklist
- expected gates
- non-goals
- evidence/commits when backfilling completed work

## Planning Rules

- Prefer one committable outcome per pulse.
- Name source artifacts and generated artifacts explicitly.
- Include review roles when the pulse changes doctrine or claim status.
- Put gates in the plan before implementation.
- Backfilled pulses must cite commits or artifacts instead of pretending they
  were planned before the work happened.
