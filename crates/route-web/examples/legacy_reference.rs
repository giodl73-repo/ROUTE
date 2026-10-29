fn main() {
    for i in [
        route_web::Input::default(),
        route_web::Input {
            corridor: 1,
            demand_pct: 150.0,
            outage_hours: 24.0,
            reserve_pct: 0.0,
            adjacent_pct: 0.0,
            ..route_web::Input::default()
        },
        route_web::Input {
            corridor: 2,
            demand_pct: 200.0,
            capacity_pct: 25.0,
            ..route_web::Input::default()
        },
        route_web::Input {
            outage_hours: 8.0,
            capacity_pct: 75.0,
            ..route_web::Input::default()
        },
    ] {
        let input = serde_json::to_string(&i).unwrap();
        let canonical = route_web::evaluate_json(&input).unwrap();
        println!(
            "{}",
            serde_json::json!({"input":input,"canonical":canonical})
        );
    }
}
