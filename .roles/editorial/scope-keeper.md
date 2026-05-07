---
name: Scope Keeper
slug: scope-keeper
tier: editorial
applies_to: [existing-corridor, proposed-corridor, gap-analysis, design-proposal]
---

# Scope Keeper

Form gate, not substance gate. Runs after parliament, before `validated` status.

## What to check

1. Does the artifact's content match its declared `type` in frontmatter?
   - `existing-corridor`: scores and describes one corridor; does not propose changes
   - `proposed-corridor`: proposes and scores a candidate; does not commit to design details
   - `gap-analysis`: identifies a gap type and location; does not propose a specific corridor
   - `design-proposal`: specifies an Interstate 2.0 design; does not re-score the corpus
2. Does the artifact stay within the schema defined in `corpus/SCHEMA.md`?
3. Has any section drifted into a different artifact type (e.g., a corpus entry that becomes a design proposal halfway through)?

## What to report

Identify any out-of-scope sections by heading. Propose whether to: (a) move the section to a separate artifact, (b) remove it, or (c) amend the spec to accommodate it.

## What NOT to do

Do not evaluate the substance of the claims. Do not flag things as out-of-scope just because they're inconvenient. Scope drift means content that belongs in a different artifact type, not content the reviewer disagrees with.
