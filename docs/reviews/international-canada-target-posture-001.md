---
name: International Canada Target Posture 001
slug: international-canada-target-posture-001
type: review
status: draft
rubric_version: v1.0
author: codex
created: 2026-06-19
updated: 2026-06-19
sources:
  - tools/build_canada_target_posture.py
  - tools/check_canada_target_posture.py
  - data/canada_service_target_candidates.csv
  - data/international-canada-target-posture-001.csv
---

# International Canada Target Posture 001

## Result

Canada service-target rows remain held planning assumptions. This is acceptable
for internal adapter proof because the proof is about source-backed workflow,
not guaranteed service.

Decision: **held_planning_assumptions_accepted_for_internal_proof**

Blocked: guaranteed SLA, travel-time proof, delivery commitment, official
approval, construction readiness, ROI, compliance, endorsement, validation,
public readiness, and external readiness.

Run:

```powershell
npm run check:canada:target-posture
```
