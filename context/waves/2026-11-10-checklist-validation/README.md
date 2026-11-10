---
name: Independent workbench readiness in source-checklist validation
slug: checklist-independent-readiness
type: plan
status: draft
rubric_version: v1.4
author: Codex
created: 2026-11-10
updated: 2026-11-10
sources:
  - tests/browser/source-checklist.spec.js
  - web/app.js
  - docs/vtrace/WP_WEB_SOURCE_001.md
---
# Checklist independence validation

Owner merge-all request: fix the fresh PR25 build failure without changing
production behavior, original assertions, deadlines, source data or held claims.
Hosted build42888658510 captured the initial freight placeholder while only the
independent checklist had reached Ready; the final independence assertion then
compared that placeholder with the actual7.2→7.2h freight result.

Wait for both original workers' successful result states before capturing the
freight reference. Checklist invalid/edit/reset and all unchanged assertions
still exercise actual Rust. This fixes test synchronization, not a business
algorithm or UI behavior. No HPMS vintage, packet value, cache, source readiness,
research status or authority changes. WP-WEB-SOURCE-001 remains the boundary.

The provisional local build passed9 of10 cases, including the corrected checklist
case. The remaining original freight case correctly rejected dirty provenance
because the test correction was not committed yet. Preserve those raw logs at
C:/src/route-checklist-readiness-qa-20261110. A clean candidate commit, fresh
build and full10-case suite are required before push; hosted PR gates remain
required before default merge. No assertion or timeout is weakened.
