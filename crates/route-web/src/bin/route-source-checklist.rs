fn main() {
    let r = std::env::args()
        .nth(1)
        .ok_or_else(|| "Provide a checklist query file".to_string())
        .and_then(|p| std::fs::read_to_string(p).map_err(|e| e.to_string()))
        .and_then(|s| route_web::source_checklist::evaluate(&s));
    match r {
        Ok(s) => print!("{s}"),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    }
}
