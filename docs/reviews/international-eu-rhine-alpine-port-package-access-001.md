---
name: International EU Rhine-Alpine Port Package Access 001
slug: international-eu-rhine-alpine-port-package-access-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-20
updated: 2026-06-20
sources:
  - tools/build_eu_rhine_alpine_port_package_access.py
  - tools/check_eu_rhine_alpine_port_package_access.py
  - data/international-eu-rhine-alpine-port-package-access-001.csv
  - docs/reviews/international-eu-rhine-alpine-road-feature-metadata-probe-001.md
---

# International EU Rhine-Alpine Port Package Access 001

## Result

EU now has bounded package-access metadata for the GISCO Ports 2013 node source
candidate.

The GDB and SHP ZIP package URLs are reachable by HTTP `HEAD` and recorded as
package metadata only. ROUTE has not downloaded, parsed, accepted geometry,
selected port nodes, replaced fixtures, or promoted internal adapter proof.

## Gate

Decision: **ports_2013_package_access_ready; node_replacement_held**

Run:

```powershell
npm run check:eu:port-package-access
```
