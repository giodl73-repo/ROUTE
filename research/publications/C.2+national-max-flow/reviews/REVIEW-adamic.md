---
reviewer: Lada Adamic
persona: Lada Adamic, Research Scientist, Meta AI Research; Affiliate, University of Michigan School of Information
round: 1
date: 2026-05-07
score: 2/4
---

> **Note:** This is an AI-generated simulated review, written by Claude in the voice of a named-expert persona. It is not an actual review by the named person and does not reflect their views or endorsement.

## Overall

This paper presents an ambitious and technically interesting application of max-flow analysis to the national highway network. The graph construction effort — 227 corridors, ~48,000 directed edges — is substantial, and the Edmonds-Karp implementation on a network of this scale is non-trivial. The bottleneck identification and closure simulation results are compelling as exploratory findings. However, the single-commodity max-flow formulation is not a minor limitation — it is a fundamental modeling choice that may misidentify the binding constraints in the real freight network, because the binding constraint in a multi-commodity system (where truck freight, rail-truck intermodal, and temperature-controlled freight compete for capacity on different paths) may not be the same arc that binds in a single-commodity system. The paper's investment recommendations should not be presented as conclusions of the max-flow analysis unless the authors address this gap. Score: 2/4.

## What Works

The graph construction methodology is well-documented. The choice of TIGER/Line with HPMS capacity overlays is appropriate for a national-scale analysis, and the 227-corridor, 48,000-edge specification is sufficiently large to capture the essential structure of the national network without the computational explosion of segment-level granularity. The 8 origin-destination clusters based on FAF5 economic regions are well-chosen — they correspond to established BEA economic areas and have reasonable demand data available.

The Donner closure simulation is the paper's most striking result: NE→Pacific max-flow drops 23% and I-40 V/C approaches saturation (0.52→0.84). The compound failure scenario (Donner + I-35 simultaneously) producing I-40 V/C 1.11 is a genuinely important resilience finding. These results are intuitive — Donner is a well-known geographic chokepoint — but the quantification of the network-level effect is useful.

The I-69 completion result (+18% Gulf→Chicago max-flow, Dallas V/C 1.9→1.4) is consistent with the C.1 paper's operational analysis and validates the two papers' findings against each other.

## What Doesn't Work

Single-commodity max-flow assumes all freight is fungible and can flow on any path. In reality, freight has commodity classes with different path preferences: temperature-controlled freight must use I-90 Northern Route in winter for carrier operational reasons; oversized loads cannot use certain bridges or tunnels; hazardous materials are banned from certain tunnels (e.g., Holland Tunnel). When a commodity-specific constraint binds on a particular arc, it becomes a bottleneck for that commodity class only — but in the single-commodity model, its capacity is pooled with the general freight capacity of the arc, masking the constraint.

More critically: rail-truck intermodal freight does not appear in the model at all. The paper models highway capacity only, but for the Donner Pass corridor specifically, the Union Pacific Sunset Route and the BNSF Transcon carry a substantial volume of intermodal freight. Under the Donner closure scenario, how much of the highway flow would divert to rail rather than to I-40? If rail absorbs 30% of the diversion, the I-40 V/C effect shrinks from 0.52→0.84 to something significantly lower. The model cannot answer this question because it has no rail arcs.

The paper's limitation section acknowledges single-commodity max-flow but treats it as a future extension. Given that the investment recommendations (I-70W, I-69, Donner alternatives) follow directly from the max-flow results, the single-commodity limitation should be elevated to a primary caveat that conditions all recommendations.

## The Question I'd Push On

If the authors ran a two-commodity max-flow (highway freight vs. rail-eligible intermodal), would the ranking of binding bottleneck arcs change? Specifically: is the I-95 Baltimore-Washington bottleneck (V/C 2.1+) a highway-only bottleneck, or does it reflect a constraint that would persist even with freight mode-shift to rail? And for Donner Pass, does the rail alternative provide a meaningful capacity buffer that the single-commodity model systematically ignores? Even a heuristic sensitivity analysis — e.g., removing 20% of Donner Pass demand to simulate mode shift — would substantially strengthen the paper's bottleneck claims.
