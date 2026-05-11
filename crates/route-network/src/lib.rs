pub mod aggregate;
pub mod build;
pub mod centrality;
pub mod connectivity;
pub mod corridor;
pub mod coverage;
pub mod diamond;
pub mod flow;
pub mod graph;
pub mod invest;
pub mod join;
pub mod strategic;
pub mod tier;

pub use aggregate::aggregate_corridor;
pub use build::{build_graph, build_graph_with_fpm};
pub use connectivity::{analyze_t1_connectivity, T1ConnectivityReport};
pub use corridor::{Corridor, CorridorAttributes};
pub use coverage::{
    compute_coverage, compute_pop_coverage, corridor_pop_within_50mi, counties_within_50mi,
    CountyGap, CoverageResult, PopCoverageResult,
};
pub use diamond::{
    analyze_diamond, find_intersection, find_t1_intersections, DiamondResult, T1Intersection,
};
pub use flow::{corridor_max_flow, FlowResult};
pub use graph::{HighwayEdge, HighwayGraph, HighwayNode};
pub use invest::{allocate_investment, InvestmentCandidate, InvestmentPlan, UpgradeType};
pub use strategic::{
    agricultural_export_score, init_designations, load_designations, military_strategic_score,
    usmca_corridor_score,
};
pub use tier::{
    analyze_tier_connectivity, endpoint_rule_label, minimum_system_contacts_for_tier,
    tier_connectivity_gate_failures, RouteTier, StopNodeClass, StopServiceClass,
    TierConnectivityRow, TierNodeClass, TierTouchNode, T1_BACKBONE_ROUTES, T1_SCORE_THRESHOLD,
    T2_SCORE_THRESHOLD, T3_SCORE_THRESHOLD,
};
