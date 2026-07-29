//! Helper `load_railroad_parallels`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_railroad_parallels() -> std::collections::HashMap<String, String> {
    let path = std::path::Path::new("data/railroad_parallels.csv");
    if !path.exists() {
        return std::collections::HashMap::new();
    }
    let Ok(mut rdr) = csv::Reader::from_path(path) else {
        return std::collections::HashMap::new();
    };
    let mut map = std::collections::HashMap::new();
    for result in rdr.records().filter_map(|r| r.ok()) {
        if result.len() < 5 {
            continue;
        }
        // Columns: interstate, railroad, railroad_owner, approx_parallel_miles, within_50mi, notes
        let interstate = result[0].trim().to_string();
        let railroad = result[1].trim().to_string();
        let within_50mi = result[4].trim() == "true";
        if within_50mi {
            // Normalize interstate name: "I-80" -> "I80"
            let id: String = interstate
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_uppercase();
            map.insert(id, railroad);
        }
    }
    map
}

