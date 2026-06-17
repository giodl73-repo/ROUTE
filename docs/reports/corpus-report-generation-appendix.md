# Corpus And Report Generation Appendix

## Purpose

This appendix explains how ROUTE's corpus and report-generation machinery
supports the communications package without turning generated markdown into a
source of truth.

The safe claim is narrow: ROUTE can produce reviewable draft corpus entries
from current graph attributes, scores, bundle identity, source labels, and
generation provenance. A generated entry is an inspection surface. It is not an
official plan, an agency decision, a proof of service readiness, or a substitute
for source-backed review.

## What Exists

| Surface | Current Role | Reviewer Use |
|---|---|---|
| `crates/route-report` | Generates corpus markdown entries and scoring ledger updates. | Inspect whether generated entries carry score dimensions, confidence, sources, and provenance. |
| `write_corpus_entry_with_provenance` | Writes a corridor corpus entry with command, manifest, and scoring-config metadata. | Check which command and source configuration produced the draft entry. |
| `write_bundle_corpus_entry_with_provenance` | Adds bundle frontmatter before the corridor block. | Confirm that a report can identify the service object by `segment_bundle_id`, not only by route label. |
| `route score` CLI surface | Scores one corridor and can write to existing or proposed corpus paths. | Distinguish current corpus regeneration from proposed or draft material. |
| `docs/briefs/bundle-identity-brief.md` | Defines why reports must preserve bundle, member, stitch, alias, and scope identity. | Pressure-test whether generated output keeps identity stable enough for review. |

## Generation Contract

Generated corpus entries should carry enough context for a reviewer to ask
where a claim came from and what still limits it.

| Field / Section | Why It Matters | Boundary |
|---|---|---|
| `generated_by` and `## Generation` command row | Shows the command context that produced the entry. | Command provenance is not correctness proof. |
| `data_manifest_version` and `data_manifest_path` | Shows which source manifest was used. | A manifest reference does not prove every source row is sufficient. |
| `scoring_config_path` | Shows which scoring configuration shaped the dimensions. | A config path does not make the scoring policy accepted by a reviewer. |
| `estimated`, `confidence`, and confidence labels | Keeps weak or estimated evidence visible. | Confidence labels do not promote an unsupported result. |
| Sixteen dimension rows and `/160` total | Makes the score structure inspectable. | The score is a review artifact, not a funding conclusion. |
| `segment_bundle_id` and `member_segment_ids` | Preserves stable service identity and physical extent. | Bundle identity makes the object reviewable; it does not prove the service claim. |
| Source labels | Names source classes used by the generated entry. | Source labels are not a complete citation audit by themselves. |

## Communications Use

Use generated corpus/report artifacts when the meeting needs to show process,
not certainty:

- "This draft entry shows the current scoring structure, confidence labels, and
  generation metadata."
- "This bundle id is the review object; the member segment ids are the physical
  evidence rows to inspect."
- "The proposed/current path tells reviewers whether the entry is a draft
  candidate or a regenerated existing corpus item."
- "Weak confidence, estimated values, missing source detail, or held identity
  fields are evidence debt, not presentation defects."

Do not use generated corpus/report artifacts to claim:

- an adopted route plan;
- a construction or funding recommendation;
- an operating service guarantee;
- a positive ROI or benefit-cost result;
- legal, grant, or program eligibility;
- agency compliance, endorsement, or stakeholder validation;
- public or release readiness.

## Review Ladder Questions

Use these questions during technical, state, regional, congressional, or DOT
pressure tests:

| Reviewer Question | Passing Answer |
|---|---|
| What command produced this entry? | The entry names the command in frontmatter/body, or the entry remains draft until provenance is added. |
| Which data manifest and scoring config were used? | The entry names both paths and version fields, or the source posture is held. |
| Is the object a route label or a bundle? | The report uses `segment_bundle_id` for service claims and expands to `member_segment_ids` for physical evidence. |
| Are all scoring dimensions visible? | The generated entry exposes each dimension and total structure for challenge. |
| Which values are estimated or low confidence? | Estimated and confidence fields remain visible, with no cleanup that hides uncertainty. |
| Is this proposed, current, or accepted? | The path/status makes the posture explicit; proposed material is not treated as adopted. |
| What closes the next blocker? | The appendix names the missing source, role review, fixture, or command evidence instead of promoting the claim. |

## Round-Specific Failure Modes

| Round | Likely Challenge | Pass Condition |
|---|---|---|
| Intra-state regional meeting | "Does this draft entry erase local objections?" | Keep generated entries paired with intake notes and held claims. |
| State meeting | "Is this a state commitment or a planning artifact?" | State clearly that corpus output is draft analysis until source-backed state review exists. |
| Multi-state regional meeting | "Can a route label mean different things across borders?" | Use bundle/member/stitch identity and state scope rather than label-only claims. |
| Congressional hearing | "Are you presenting a national build list?" | Present corpus output as an auditable method and evidence queue, not a program list. |
| DOT review | "Can we reproduce, inspect, and challenge the artifact?" | Show command provenance, manifest/config paths, confidence labels, dimension rows, and next evidence steps. |

## Current Claim Posture

The corpus/report generation capability is implemented as a draft artifact
generator. Tests cover the presence of the sixteen dimensions, `/160` total,
reproducible date handling through `ROUTE_DATE`, bundle frontmatter, parent
directory creation, and generation provenance in frontmatter/body.

Those tests support a technical story about inspectability. They do not support
policy, construction, SLA, ROI, public-release, eligibility, compliance, or
endorsement claims.

## Safe Language

| Use This | Avoid This |
|---|---|
| "Generated corpus entries make the current scoring and evidence posture inspectable." | "The generated report proves the corridor should advance." |
| "Bundle frontmatter keeps the service object stable across maps, reports, simulations, and overlays." | "The route name is enough to define the corridor." |
| "Provenance shows how this draft was generated." | "Provenance proves the result is correct." |
| "Confidence and estimated fields keep evidence debt visible." | "The score is final despite weak source posture." |
| "Proposed entries are candidates for review." | "Proposed entries are accepted recommendations." |

## Next Evidence Steps

1. Capture a small before/after generated corpus example in the Round 5 command
   bundle when a populated stakeholder fixture exists.
2. Add report-generation examples to the demo runbook only when command output,
   generated paths, and held-claim captions are recorded.
3. Pair any generated entry used in a review packet with a source pack, role
   review note, and prohibited-claim scan.
4. Keep bundle identity visible in generated report metadata before a map,
   simulation, or game overlay uses the same claim.
