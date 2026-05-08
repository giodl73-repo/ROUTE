---
reviewer: Jarrett Walker
paper: F.2+intercity-bus-corridors
review_type: recheck
round: 1
date: 2026-05-07
pp_items_rechecked: [PP1.2, PP2.2]
verdict: PASS-WITH-NOTE
score: 3/4
---

> **Note:** AI-generated simulated recheck review.

## Items Rechecked

### PP2.2 — Stop-penalty model (62 mph effective speed claim)

The original concern: the paper modeled bus travel time at 62 mph effective average speed by applying a 3 mph deduction from the 65 mph managed-lane design speed. This deduction accounts for deceleration and acceleration around stops integrated over the full route, but for a route with 8 intermediate stops on a 500-mile corridor, the actual stop-time penalty is approximately 64 minutes (8 × 8 min), reducing effective speed to roughly 54–55 mph, not 62 mph. The 62 mph figure was presented without a stop-count model, making it non-replicable.

The revision adds an explicit stop-penalty model in Section 3. The model specifies $N_{\text{stops}} = \lfloor d/150 \rfloor$ intermediate T1/T2 hub stops, a dwell time of $t_{\text{stop}} = 8$ minutes per stop, and derives effective average speed $\bar{v} \approx 54$–$58$ mph for corridors in the 300–900 mile range. The formula is stated, the parameter values are defined, and the result is correctly more conservative than the prior 62 mph assumption. The 12-corridor comparison table is stated to reflect this stop-penalty model.

This is precisely the fix the original concern required. The stop-penalty is now explicit, parameterized, and consistent with the travel time model. A reviewer can now check any specific corridor against the formula. The effective speed range (54–58 mph) is more honest than the prior 62 mph and still represents a meaningful advantage over current Greyhound effective speeds (~44 mph). This concern is resolved.

Note: the paper retains the 62 mph figure in the original "effective cruising speed" paragraph (the 3 mph deduction from 65 mph) while adding the stop-penalty model as a separate paragraph below it. Readers should understand that 62 mph is the speed between stops, and 54–58 mph is the effective average over the full trip including stops. The distinction is now stated but the two paragraphs sit in mild tension; a future revision should either consolidate them or add a clarifying sentence noting that 62 mph is the between-stop cruising speed, not the trip-level average.

### PP1.2 — Local access assumption and 24M demand estimate transparency

The original concern: the gravity model produces a 24 million annual passenger estimate that assumes travelers can reach T1/T1 hub stops. For transit-dependent travelers, this assumption is unwarranted in the absence of feeder service. Additionally, the calibration corpus (18 corridors from BTS) was named but the calibration methodology was underdescribed.

The revision adds a "Local Access Assumption" paragraph stating explicitly that the 24 million figure is an upper bound for markets where local feeder service does not exist, and that FTA 5311 grantee markets are partially excepted. The upper-bound framing is the correct scientific response: it converts an implicit assumption into an explicit scope condition. The calibration methodology specificity — 18 corridors, $R^2 = 0.79$ in-sample — remains as stated in the original paper and is now easier to interpret against the upper-bound framing.

The upper-bound framing resolves the first-order concern. The calibration corpus is still identified only by size and fit statistic; a future revision should list the 18 corridors in an appendix so the calibration can be fully reproduced. This is a P3 item and does not block the current recheck.

**P3 recommendation (future work):** The paper should include a mode-split sensitivity analysis: how does the 24 million annual passenger estimate change under 50% / 25% / 10% modal capture assumptions? The gravity model produces a total potential market; the actual ridership depends on what fraction of that market the bus captures relative to driving, air, and Amtrak. A three-row sensitivity table would substantially strengthen the demand analysis.

## Verdict

Both blocking concerns from Round 1 are addressed. The stop-penalty model is now explicit and correctly derived; the 24M upper-bound framing is the appropriate scientific scope condition. The P3 items (between-stop/trip-average speed clarification, 18-corridor appendix, mode-split sensitivity) are recommended for the next revision but do not block readiness.
