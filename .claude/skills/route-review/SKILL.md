---
name: route-review
description: "Run a ROUTE role review using .roles personas and write findings to wave panels or docs/reviews."
tags: [route, review, roles, findings, doctrine]
---

# route-review

Use this skill for plan, pulse, spec, artifact, or code reviews.

## Role Sources

Load roles from `.roles/`:

- `.roles/parliament/`
- `.roles/stakeholders/`
- `.roles/editorial/`
- `.roles/panel/` when present

## Review Output

For wave-local review, write:

```text
waves/{active}/panels/{review-name}/R1-{role}.md
waves/{active}/panels/{review-name}/R1-consolidated.md
```

For doctrine/public review, write under `docs/reviews/`.

## Finding Severity

- `BLOCK`: must fix before claim/gate can pass.
- `WARN`: can proceed only if the risk is carried visibly.
- `NOTE`: useful improvement or future pulse.

Every finding must name a file/artifact, consequence, and concrete fix.
