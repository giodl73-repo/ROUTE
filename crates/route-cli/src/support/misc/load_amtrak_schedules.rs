//! Helper `load_amtrak_schedules`.
#[allow(unused_imports)]
use crate::*;

pub(crate) fn load_amtrak_schedules(
    data_dir: &std::path::Path,
) -> std::collections::HashMap<String, f64> {
    let path = data_dir.join("amtrak-schedules.csv");
    let mut map = std::collections::HashMap::new();
    let Ok(file) = std::fs::File::open(&path) else {
        return map;
    };
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let Ok(record) = result else { continue };
        let slug = record.get(0).unwrap_or("").trim().to_string();
        let hours: f64 = match record.get(2).unwrap_or("").trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        if !slug.is_empty() {
            map.entry(slug).or_insert(hours);
        }
    }
    map
}
