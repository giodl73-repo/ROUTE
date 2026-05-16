use crate::manifest::{Manifest, ManifestSource};
use anyhow::{Context, Result};
use fletch_core::{
    adapter_handoff_report, dry_run_flight, fetch_plan_with_kind, fetch_to_cache,
    graph_from_registry, validate_registry, CachePolicy, FetchOptions, FletchRegistry,
    FreshnessPolicy, SourceKind,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct RouteSourceFetchPolicyRow {
    pub fetch_family: String,
    pub commands: String,
    pub cache_targets: String,
    pub mutation_mode: String,
    pub preservation_contract: String,
    pub implementation_guard: String,
    pub validation_floor: String,
    pub policy_doc: String,
    pub validation_status: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct FletchSourceHandoffRow {
    pub fetch_family: String,
    pub fletch_id: String,
    pub source_kind: String,
    pub source_url: String,
    pub cache_targets: String,
    pub mutation_mode: String,
    pub acquisition_mode: String,
    pub activation_rule: String,
    pub route_validation_floor: String,
    pub dependency_count: usize,
    pub dependency_ids: String,
    pub graph_edge_count: usize,
    pub handoff_status: String,
    pub validation_status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FletchSourceHandoffReport {
    pub registry_id: String,
    pub registry_valid: bool,
    pub fletch_count: usize,
    pub source_count: usize,
    pub adapter_source_count: usize,
    pub graph_node_count: usize,
    pub graph_edge_count: usize,
    pub flight_step_count: usize,
    pub validation_finding_count: usize,
    pub policy_family_count: usize,
    pub covered_family_count: usize,
    pub missing_policy_families: Vec<String>,
    pub rows: Vec<FletchSourceHandoffRow>,
}

pub fn load_fletch_source_registry(path: &Path) -> Result<FletchRegistry> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))
}

pub fn load_route_source_fetch_policy(path: &Path) -> Result<Vec<RouteSourceFetchPolicyRow>> {
    let mut reader =
        csv::Reader::from_path(path).with_context(|| format!("reading {}", path.display()))?;
    let mut rows = Vec::new();
    for row in reader.deserialize() {
        rows.push(row?);
    }
    Ok(rows)
}

pub fn fletch_source_handoff_report(
    registry: &FletchRegistry,
    policy_rows: &[RouteSourceFetchPolicyRow],
) -> FletchSourceHandoffReport {
    let validation = validate_registry(registry);
    let requested = registry
        .fletches
        .iter()
        .map(|definition| definition.id.clone())
        .collect::<Vec<_>>();
    let handoff = adapter_handoff_report(registry, &requested);
    let flight = dry_run_flight(registry, &requested);
    let graph = graph_from_registry(registry);
    let policy_families = policy_rows
        .iter()
        .map(|row| row.fetch_family.as_str())
        .collect::<BTreeSet<_>>();
    let policy_by_family = policy_rows
        .iter()
        .map(|row| (row.fetch_family.as_str(), row))
        .collect::<BTreeMap<_, _>>();

    let mut covered_families = BTreeSet::new();
    let mut rows = Vec::new();
    for definition in &registry.fletches {
        let fetch_family = definition
            .metadata
            .get("fetch_family")
            .cloned()
            .unwrap_or_default();
        if !fetch_family.is_empty() {
            covered_families.insert(fetch_family.clone());
        }
        let policy = policy_by_family.get(fetch_family.as_str());
        let cache_targets = definition
            .metadata
            .get("cache_targets")
            .cloned()
            .or_else(|| policy.map(|row| row.cache_targets.clone()))
            .unwrap_or_default();
        let mutation_mode = definition
            .metadata
            .get("mutation_mode")
            .cloned()
            .or_else(|| policy.map(|row| row.mutation_mode.clone()))
            .unwrap_or_default();
        let route_validation_floor = definition
            .metadata
            .get("route_validation_floor")
            .cloned()
            .or_else(|| policy.map(|row| row.validation_floor.clone()))
            .unwrap_or_default();
        let activation_rule = definition
            .metadata
            .get("activation_rule")
            .cloned()
            .unwrap_or_default();
        let acquisition_mode = definition
            .metadata
            .get("acquisition_mode")
            .cloned()
            .unwrap_or_default();
        let dependency_ids = definition
            .edges
            .iter()
            .map(|edge| edge.to.clone())
            .collect::<Vec<_>>()
            .join(";");
        let source = definition.shafts.first();
        let source_kind = source
            .map(|source| source_kind_label(&source.kind).to_string())
            .unwrap_or_else(|| "none".to_string());
        let source_url = source.map(|source| source.url.clone()).unwrap_or_default();
        let graph_edge_count = graph
            .edges
            .iter()
            .filter(|edge| edge.from.ends_with(&definition.id))
            .count();
        let handoff_status = if validation.findings.iter().any(|finding| {
            finding
                .fletch_id
                .as_deref()
                .is_some_and(|id| id == definition.id)
        }) {
            "registry-blocked"
        } else if policy.is_none() {
            "policy-unmapped"
        } else if source.is_some_and(|source| source.kind == SourceKind::Adapter) {
            "adapter-required"
        } else {
            "generic-fetch-ready"
        };
        let validation_status = if policy.is_some()
            && !activation_rule.is_empty()
            && !cache_targets.is_empty()
            && !mutation_mode.is_empty()
            && !route_validation_floor.is_empty()
            && definition
                .metadata
                .get("claim_validated_by_download")
                .is_none_or(|value| value != "true")
        {
            "pass"
        } else {
            "review"
        };
        rows.push(FletchSourceHandoffRow {
            fetch_family,
            fletch_id: definition.id.clone(),
            source_kind,
            source_url,
            cache_targets,
            mutation_mode,
            acquisition_mode,
            activation_rule,
            route_validation_floor,
            dependency_count: definition.edges.len(),
            dependency_ids,
            graph_edge_count,
            handoff_status: handoff_status.to_string(),
            validation_status: validation_status.to_string(),
        });
    }
    rows.sort_by(|left, right| left.fetch_family.cmp(&right.fetch_family));

    let missing_policy_families = policy_families
        .iter()
        .filter(|family| !covered_families.contains(**family))
        .map(|family| (*family).to_string())
        .collect::<Vec<_>>();

    FletchSourceHandoffReport {
        registry_id: registry.registry_id.clone(),
        registry_valid: handoff.registry_valid,
        fletch_count: handoff.fletch_count,
        source_count: handoff.source_count,
        adapter_source_count: handoff.adapter_source_count,
        graph_node_count: handoff.graph_node_count,
        graph_edge_count: handoff.graph_edge_count,
        flight_step_count: flight.steps.len(),
        validation_finding_count: handoff.validation_finding_count,
        policy_family_count: policy_families.len(),
        covered_family_count: policy_families
            .len()
            .saturating_sub(missing_policy_families.len()),
        missing_policy_families,
        rows,
    }
}

pub fn write_fletch_source_handoff(path: &Path, report: &FletchSourceHandoffReport) -> Result<()> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut writer = csv::Writer::from_path(path)?;
    for row in &report.rows {
        writer.serialize(row)?;
    }
    writer.flush()?;
    Ok(())
}

pub fn fetch_all_manifest_sources_with_fletch(manifest: &Manifest, force: bool) -> Result<()> {
    std::fs::create_dir_all(&manifest.cache_dir).context("creating cache directory")?;
    let fletch_root = manifest.cache_dir.join(".fletch");

    for (name, source) in &manifest.sources {
        if source.url.is_empty() {
            println!("  [skip] {name} - no URL (manual source)");
            continue;
        }

        let dest = manifest.cache_dir.join(&source.filename);
        if dest.exists() && !force {
            println!(
                "  [skip] {name} - already cached ({} bytes)",
                dest.metadata().map(|m| m.len()).unwrap_or(0)
            );
            continue;
        }

        println!("  [fletch] {name}");
        println!("            {}", source.url);
        let outcome = fetch_manifest_source_to_fletch(name, source, &fletch_root, force)
            .with_context(|| format!("fetching {name} through FLETCH"))?;
        let bytes = std::fs::read(&outcome.path)
            .with_context(|| format!("reading FLETCH cache object {}", outcome.path.display()))?;
        crate::fetch::atomic_write_bytes(&dest, &bytes)
            .with_context(|| format!("writing ROUTE cache target {}", dest.display()))?;
        println!(
            "  [ok]      {} -> {} bytes",
            name,
            dest.metadata().map(|m| m.len()).unwrap_or(0)
        );
    }
    Ok(())
}

fn fetch_manifest_source_to_fletch(
    name: &str,
    source: &ManifestSource,
    cache_root: &Path,
    force: bool,
) -> Result<fletch_core::FetchOutcome> {
    let mut plan = fetch_plan_with_kind(
        format!("route.manifest.{name}"),
        source.url.clone(),
        SourceKind::Http,
    )?;
    plan.version = Some(source.year.to_string());
    plan.cache_policy = CachePolicy {
        freshness: FreshnessPolicy::Immutable,
        allow_offline: true,
        resumable: true,
    };
    plan.metadata
        .insert("route_filename".to_string(), source.filename.clone());
    plan.metadata
        .insert("route_format".to_string(), format!("{:?}", source.format));
    fetch_to_cache(
        &plan,
        FetchOptions::new(PathBuf::from(cache_root)).with_force(force),
    )
    .map_err(Into::into)
}

fn source_kind_label(kind: &SourceKind) -> &'static str {
    match kind {
        SourceKind::Http => "http",
        SourceKind::File => "file",
        SourceKind::Adapter => "adapter",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fletch_core::{
        FletchDefinition, GraphEdgeKind, GraphNodeKind, RegistryEdge, SourceSpec,
        FLETCH_REGISTRY_SCHEMA,
    };

    #[test]
    fn handoff_report_covers_policy_families_without_claim_promotion() {
        let registry = FletchRegistry {
            schema_version: FLETCH_REGISTRY_SCHEMA.to_string(),
            generated_by: "route-test".to_string(),
            registry_id: "route.sources".to_string(),
            fletches: vec![FletchDefinition {
                id: "route.hpms.national".to_string(),
                node_kind: GraphNodeKind::Fletch,
                shafts: vec![SourceSpec {
                    kind: SourceKind::Adapter,
                    url: "route-adapter://fetch-hpms".to_string(),
                    headers: BTreeMap::new(),
                }],
                edges: vec![RegistryEdge {
                    to: "route.derived.pavement".to_string(),
                    kind: GraphEdgeKind::DerivedFrom,
                    label: Some("feeds".to_string()),
                    metadata: BTreeMap::new(),
                }],
                format: None,
                tags: vec!["route-source".to_string()],
                metadata: BTreeMap::from([
                    ("fetch_family".to_string(), "hpms-national".to_string()),
                    (
                        "cache_targets".to_string(),
                        "data/cache/hpms_2018.csv".to_string(),
                    ),
                    (
                        "mutation_mode".to_string(),
                        "full-replace-after-validation".to_string(),
                    ),
                    ("acquisition_mode".to_string(), "group".to_string()),
                    (
                        "activation_rule".to_string(),
                        "non-empty parsed HPMS rows".to_string(),
                    ),
                    (
                        "route_validation_floor".to_string(),
                        "non-empty HPMS record set".to_string(),
                    ),
                ]),
            }],
        };
        let policies = vec![RouteSourceFetchPolicyRow {
            fetch_family: "hpms-national".to_string(),
            commands: "route fetch-hpms".to_string(),
            cache_targets: "data/cache/hpms_2018.csv".to_string(),
            mutation_mode: "full-replace-after-validation".to_string(),
            preservation_contract: "preserve old cache".to_string(),
            implementation_guard: "temp replace".to_string(),
            validation_floor: "non-empty HPMS record set".to_string(),
            policy_doc: "docs/source-fetch-cache-policy.md".to_string(),
            validation_status: "pass".to_string(),
        }];

        let report = fletch_source_handoff_report(&registry, &policies);

        assert!(report.registry_valid);
        assert_eq!(report.covered_family_count, 1);
        assert!(report.missing_policy_families.is_empty());
        assert_eq!(report.rows[0].handoff_status, "adapter-required");
        assert_eq!(report.rows[0].validation_status, "pass");
    }
}
