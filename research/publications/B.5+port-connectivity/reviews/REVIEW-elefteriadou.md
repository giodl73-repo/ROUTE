---
reviewer: Lily Elefteriadou
persona: Lily Elefteriadou — Director, McTrans Center, University of Florida; HCM7 contributing author; freeway operations and capacity
round: 1
date: 2026-05-08
score: 2/4
---

> **Note:** AI-generated simulated review.

## Overall

This paper addresses a genuine and under-studied problem — the short-segment, high-intensity connectors between major US port gates and the T1 interstate system — and frames it well. The V/C analysis is structured correctly, the drayage peak-hour modeling is a meaningful contribution, and the Savannah trajectory projection is defensible. However, the paper's central methodological innovation, the K_port peak hour factor of 0.12–0.14, is presented as an empirical input without the empirical backing that a factor of this consequence requires. The K_port factor governs every V/C result in the paper. If it is wrong by 20%, the comparative severity rankings change. That is not acceptable for work claiming to support investment prioritization at the $20–50B scale.

## What Works

The port connector identification methodology is clear and reproducible. The selection of the top 10 US container ports by TEU volume and the procedure for identifying the "first interstate segment from the port gate" gives reviewers enough to audit the dataset. The failure-mode taxonomy — capacity failure, classification failure, resilience failure, jurisdictional fragmentation — is a useful analytical framework that ports and FHWA planners could actually deploy.

The Savannah trajectory analysis (Section 5) is the paper's strongest section. The compound growth model applied to I-16/I-95 interchange demand, anchored to Georgia Ports Authority's verified 2020–2024 CAGR data, produces a credible V/C=2.6 projection for 2028. This is not speculative; the GPA CAGR is public and the infrastructure lag timeline (7–10 years from decision to opening) is well-established in port capital programs.

The jurisdictional fragmentation diagnosis in Section 7 is also well-argued. The multi-agency overlap at I-710 — port authority, City of Los Angeles, Caltrans, FHWA — is documented, and the paper correctly identifies this as a coordination problem that investment dollars alone cannot solve.

## What Doesn't Work

**K_port factor (critical).** The paper specifies a K_port factor of 0.12–0.14 to capture drayage peak-hour demand concentration — described as higher than the HCM7 standard K30 factor of 0.09–0.10 for freeway segments. This is plausible in concept: port drayage operates in a compressed gate window (06:00–18:00) with appointment-system demand spiking at gate opening. But the paper offers no source for 0.12–0.14. The two paths to legitimacy are: (a) empirical calibration from actual POLA/POLB gate transaction data published by hour — these data exist and are public; or (b) a derivation from HCM7 Section 12 (freeway facilities with managed demand) using gate appointment arrival headways as the input distribution. Without one of these, K_port is a number the paper invented, and the V/C analysis rests on it entirely.

**HCM7 citation gap.** HCM 6th Edition is cited throughout. HCM7 (published 2022) supersedes this and includes specific guidance on managed-demand facilities including appointment-based loading docks and terminal access roads. If K_port is to be derived analytically rather than empirically, HCM7 Section 12 is the correct starting point. Using HCM6 when HCM7 is available for a 2026 paper is a material citation deficiency.

**Queuing model absent.** The I-710 delay cost ($1.86B/year) is attributed to a queuing model, but no queuing model is described. What is the arrival distribution? Is this M/M/1, M/D/c, or a deterministic peak-period volume-delay function? The BPR function and the queuing models produce materially different delay estimates at V/C > 1.0. The reader cannot audit the $1.86B figure without knowing which model was applied.

**CBP wait time data.** The Laredo 3-hour average wait time is stated but sources are not specified to publication-year granularity. CBP publishes wait time data by crossing and direction, but these data are notoriously variable — wait times at Laredo range from under 1 hour to over 6 hours depending on time of day, day of week, and CBP staffing. Using an annual average obscures this variance. The paper should report the distribution (mean, 85th percentile, peak-day) and anchor to a specific CBP data release.

## The Question I'd Push On

If POLA/POLB gate transaction data (published hourly at pola.org) were used to directly measure peak-hour arrival rates for drayage trucks, what K_port value would emerge? My expectation, based on appointment-system dynamics at major ports, is that K_port for the first two hours after gate opening could be as high as 0.18–0.22 — substantially higher than 0.12–0.14 — meaning the paper may be *understating* peak congestion rather than overstating it. That would strengthen the investment case. But the paper needs to make this measurement rather than assume the factor. The POLA data are freely available; this is a one-week analysis, not a multi-year research program.
