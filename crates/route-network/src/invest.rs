/// Investment allocation for Interstate 2.0 corridor upgrades.
///
/// Given a budget and a set of corridors with estimated upgrade costs and
/// throughput gains, find the portfolio that maximizes total benefit.
///
/// Phase 1 — single budget constraint: LP solved with minilp (pure Rust).
/// Each x_i ∈ [0,1] is the fraction of corridor i to fund.
/// Objective: maximize Σ gain_i × x_i
/// Subject to: Σ cost_i × x_i ≤ budget
///             0 ≤ x_i ≤ 1  ∀i
///
/// For future multi-constraint problems (budget + maintenance + political
/// feasibility + climate resilience simultaneously), add rows to the LP.
use minilp::{Problem, OptimizationDirection, ComparisonOp};

/// A corridor that is a candidate for investment.
#[derive(Debug, Clone)]
pub struct InvestmentCandidate {
    pub route_id: String,
    pub designation: String,
    /// Corridor length in miles
    pub miles: f64,
    /// Estimated upgrade cost in $B (see cost_per_mile_b)
    pub upgrade_cost_b: f64,
    /// Estimated daily throughput gain (vehicles/day) at full investment
    pub throughput_gain_vpd: f64,
    /// Composite score improvement (0–120 scale) at full investment
    pub score_improvement: f64,
    /// Upgrade type — drives cost model
    pub upgrade_type: UpgradeType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpgradeType {
    /// Add managed freight lanes to existing interstate
    InterstateWidening,
    /// Upgrade US highway to interstate standard (grade separation, access control)
    UsHighwayToInterstate,
    /// Upgrade state highway to interstate standard
    StateHighwayToInterstate,
    /// New greenfield corridor
    Greenfield,
}

impl UpgradeType {
    /// Rough cost per mile in $B. Source: FHWA project cost data ranges.
    pub fn cost_per_mile_b(&self) -> f64 {
        match self {
            UpgradeType::InterstateWidening      => 0.010, // $10M/mile average
            UpgradeType::UsHighwayToInterstate   => 0.030, // $30M/mile average
            UpgradeType::StateHighwayToInterstate => 0.040, // $40M/mile average
            UpgradeType::Greenfield              => 0.075, // $75M/mile average
        }
    }

    /// Throughput gain per mile per day at full upgrade (vpd added).
    pub fn throughput_gain_per_mile_vpd(&self) -> f64 {
        match self {
            UpgradeType::InterstateWidening      => 45_600.0, // 2 added lanes × 1900 × 24
            UpgradeType::UsHighwayToInterstate   => 91_200.0, // full interstate standard
            UpgradeType::StateHighwayToInterstate => 91_200.0,
            UpgradeType::Greenfield              => 91_200.0,
        }
    }
}

impl InvestmentCandidate {
    /// Build a candidate from corridor data.
    pub fn from_corridor(
        route_id: &str,
        designation: &str,
        miles: f64,
        is_upgrade: bool,
    ) -> Self {
        let upgrade_type = if is_upgrade {
            if route_id.starts_with("US") {
                UpgradeType::UsHighwayToInterstate
            } else {
                UpgradeType::StateHighwayToInterstate
            }
        } else {
            UpgradeType::InterstateWidening
        };

        let upgrade_cost_b = miles * upgrade_type.cost_per_mile_b();
        let throughput_gain_vpd = miles * upgrade_type.throughput_gain_per_mile_vpd();

        InvestmentCandidate {
            route_id: route_id.to_string(),
            designation: designation.to_string(),
            miles,
            upgrade_cost_b,
            throughput_gain_vpd,
            score_improvement: 0.0, // filled in by caller if scores available
            upgrade_type,
        }
    }
}

/// The output investment plan.
#[derive(Debug)]
pub struct InvestmentPlan {
    pub budget_b: f64,
    pub allocated_b: f64,
    pub items: Vec<InvestmentItem>,
    pub total_throughput_gain_vpd: f64,
    pub total_score_improvement: f64,
}

#[derive(Debug, Clone)]
pub struct InvestmentItem {
    pub route_id: String,
    pub designation: String,
    pub allocation: f64,       // fraction funded (0.0–1.0)
    pub cost_b: f64,           // dollars allocated
    pub throughput_gain_vpd: f64,
    pub upgrade_type: UpgradeType,
    pub miles: f64,
}

/// Solve the investment allocation LP.
/// Returns the optimal portfolio maximising throughput within budget.
pub fn allocate_investment(
    candidates: &[InvestmentCandidate],
    budget_b: f64,
) -> InvestmentPlan {
    if candidates.is_empty() {
        return InvestmentPlan {
            budget_b, allocated_b: 0.0, items: vec![],
            total_throughput_gain_vpd: 0.0, total_score_improvement: 0.0,
        };
    }

    // LP: maximise Σ gain_i × x_i  s.t.  Σ cost_i × x_i ≤ budget, 0 ≤ x_i ≤ 1
    let mut problem = Problem::new(OptimizationDirection::Maximize);

    let vars: Vec<_> = candidates
        .iter()
        .map(|c| problem.add_var(c.throughput_gain_vpd, (0.0, 1.0)))
        .collect();

    // Budget constraint: Σ cost_i × x_i ≤ budget_b
    let budget_row: Vec<(minilp::Variable, f64)> = vars.iter()
        .zip(candidates.iter())
        .map(|(&v, c)| (v, c.upgrade_cost_b))
        .collect();
    problem.add_constraint(&budget_row, ComparisonOp::Le, budget_b);

    // Solve
    let allocations: Vec<f64> = match problem.solve() {
        Ok(solution) => vars.iter().map(|&v| solution[v].max(0.0).min(1.0)).collect(),
        Err(_) => {
            // Fallback: greedy by gain/cost ratio (optimal for single constraint)
            greedy_allocate(candidates, budget_b)
        }
    };

    // Build output
    let mut items = Vec::new();
    let mut allocated_b = 0.0;
    let mut total_gain = 0.0;
    let mut total_score = 0.0;

    for (c, &alloc) in candidates.iter().zip(allocations.iter()) {
        if alloc < 0.001 { continue; }
        let cost = c.upgrade_cost_b * alloc;
        let gain = c.throughput_gain_vpd * alloc;
        allocated_b += cost;
        total_gain += gain;
        total_score += c.score_improvement * alloc;
        items.push(InvestmentItem {
            route_id: c.route_id.clone(),
            designation: c.designation.clone(),
            allocation: alloc,
            cost_b: cost,
            throughput_gain_vpd: gain,
            upgrade_type: c.upgrade_type.clone(),
            miles: c.miles,
        });
    }

    // Sort by throughput gain descending
    items.sort_by(|a, b| b.throughput_gain_vpd.partial_cmp(&a.throughput_gain_vpd).unwrap());

    InvestmentPlan {
        budget_b,
        allocated_b,
        items,
        total_throughput_gain_vpd: total_gain,
        total_score_improvement: total_score,
    }
}

/// Greedy fallback: sort by gain/cost ratio, fill until budget exhausted.
fn greedy_allocate(candidates: &[InvestmentCandidate], budget_b: f64) -> Vec<f64> {
    let mut ratios: Vec<(usize, f64)> = candidates
        .iter()
        .enumerate()
        .map(|(i, c)| (i, c.throughput_gain_vpd / c.upgrade_cost_b.max(0.001)))
        .collect();
    ratios.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut allocations = vec![0.0f64; candidates.len()];
    let mut remaining = budget_b;

    for (i, _) in ratios {
        let c = &candidates[i];
        if remaining <= 0.0 { break; }
        let alloc = (remaining / c.upgrade_cost_b).min(1.0);
        allocations[i] = alloc;
        remaining -= c.upgrade_cost_b * alloc;
    }

    allocations
}
