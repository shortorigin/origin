use std::collections::BTreeMap;
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct ServiceBoundaryRecord {
    service_name: String,
    domain: String,
    approved_workflows: Vec<String>,
    owned_aggregates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct WorkflowBoundaryRecord {
    workflow_name: String,
    touched_domains: Vec<String>,
    target_services: Vec<String>,
    emits_evidence: bool,
    mutation_path_only: bool,
}

#[derive(Debug, Deserialize)]
struct ServiceBoundaryManifest {
    version: String,
    primary_service: Option<String>,
    approved_workflows: Option<Vec<String>>,
    owned_aggregates: Option<Vec<String>>,
    services: Option<Vec<ServiceBoundaryEntry>>,
}

#[derive(Debug, Deserialize)]
struct ServiceBoundaryEntry {
    service_name: String,
    approved_workflows: Vec<String>,
    owned_aggregates: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct WorkflowBoundaryManifest {
    version: String,
    workflow_name: String,
    touched_domains: Vec<String>,
    target_services: Vec<String>,
    emits_evidence: bool,
    mutation_path_only: bool,
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest dir"));
    let repo_root = manifest_dir
        .ancestors()
        .nth(3)
        .expect("contracts crate should live under repo root")
        .to_path_buf();
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));

    let service_boundaries = collect_service_boundaries(&repo_root);
    let workflow_boundaries = collect_workflow_boundaries(&repo_root);
    let generated = render(&service_boundaries, &workflow_boundaries);

    let generated_path = out_dir.join("generated_boundaries.rs");
    fs::write(&generated_path, generated).expect("write generated boundaries");
}

fn collect_service_boundaries(repo_root: &Path) -> Vec<ServiceBoundaryRecord> {
    let domain_root = repo_root.join("enterprise/domains");
    println!("cargo:rerun-if-changed={}", domain_root.display());

    let mut boundaries = BTreeMap::new();
    for domain_dir in fs::read_dir(&domain_root).expect("read enterprise domains") {
        let domain_dir = domain_dir.expect("domain entry");
        let file_type = domain_dir.file_type().expect("domain file type");
        if !file_type.is_dir() {
            continue;
        }

        let domain = domain_dir.file_name().to_string_lossy().into_owned();
        let manifest_path = domain_dir.path().join("service_boundaries.toml");
        if !manifest_path.is_file() {
            continue;
        }
        println!("cargo:rerun-if-changed={}", manifest_path.display());

        let contents = fs::read_to_string(&manifest_path)
            .unwrap_or_else(|error| panic!("read `{}`: {error}", manifest_path.display()));
        let manifest: ServiceBoundaryManifest = toml::from_str(&contents)
            .unwrap_or_else(|error| panic!("parse `{}`: {error}", manifest_path.display()));

        let entries = match manifest.version.as_str() {
            "v1" => vec![ServiceBoundaryEntry {
                service_name: manifest.primary_service.unwrap_or_else(|| {
                    panic!("`{}` missing primary_service", manifest_path.display())
                }),
                approved_workflows: manifest.approved_workflows.unwrap_or_else(|| {
                    panic!("`{}` missing approved_workflows", manifest_path.display())
                }),
                owned_aggregates: manifest.owned_aggregates.unwrap_or_else(|| {
                    panic!("`{}` missing owned_aggregates", manifest_path.display())
                }),
            }],
            "v2" => manifest
                .services
                .unwrap_or_else(|| panic!("`{}` missing services", manifest_path.display())),
            other => {
                panic!(
                    "unsupported service boundary manifest version `{other}` in `{}`",
                    manifest_path.display()
                );
            }
        };

        for entry in entries {
            let record = ServiceBoundaryRecord {
                service_name: entry.service_name.clone(),
                domain: domain.clone(),
                approved_workflows: entry.approved_workflows,
                owned_aggregates: entry.owned_aggregates,
            };
            let previous = boundaries.insert(record.service_name.clone(), record);
            assert!(
                previous.is_none(),
                "duplicate service boundary `{}` declared in `{}`",
                entry.service_name,
                manifest_path.display()
            );
        }
    }

    boundaries.into_values().collect()
}

fn collect_workflow_boundaries(repo_root: &Path) -> Vec<WorkflowBoundaryRecord> {
    let workflow_root = repo_root.join("workflows");
    println!("cargo:rerun-if-changed={}", workflow_root.display());

    let mut boundaries = BTreeMap::new();
    for workflow_dir in fs::read_dir(&workflow_root).expect("read workflows") {
        let workflow_dir = workflow_dir.expect("workflow entry");
        let file_type = workflow_dir.file_type().expect("workflow file type");
        if !file_type.is_dir() {
            continue;
        }

        let manifest_path = workflow_dir.path().join("workflow.boundary.toml");
        if !manifest_path.is_file() {
            continue;
        }
        println!("cargo:rerun-if-changed={}", manifest_path.display());

        let contents = fs::read_to_string(&manifest_path)
            .unwrap_or_else(|error| panic!("read `{}`: {error}", manifest_path.display()));
        let manifest: WorkflowBoundaryManifest = toml::from_str(&contents)
            .unwrap_or_else(|error| panic!("parse `{}`: {error}", manifest_path.display()));

        assert!(
            manifest.version == "v1",
            "unsupported workflow boundary manifest version `{}` in `{}`",
            manifest.version,
            manifest_path.display()
        );

        let record = WorkflowBoundaryRecord {
            workflow_name: manifest.workflow_name.clone(),
            touched_domains: manifest.touched_domains,
            target_services: manifest.target_services,
            emits_evidence: manifest.emits_evidence,
            mutation_path_only: manifest.mutation_path_only,
        };
        let previous = boundaries.insert(record.workflow_name.clone(), record);
        assert!(
            previous.is_none(),
            "duplicate workflow boundary `{}` declared in `{}`",
            manifest.workflow_name,
            manifest_path.display()
        );
    }

    boundaries.into_values().collect()
}

fn render(
    service_boundaries: &[ServiceBoundaryRecord],
    workflow_boundaries: &[WorkflowBoundaryRecord],
) -> String {
    let mut output = String::new();
    output.push_str("// @generated by schemas/crates/contracts/build.rs\n\n");

    output.push_str("#[must_use]\n");
    output.push_str("pub fn service_boundary_named(name: &str) -> Option<ServiceBoundaryV1> {\n");
    output.push_str("    match name {\n");
    for boundary in service_boundaries {
        let _ = writeln!(
            output,
            "        {:?} => Some(ServiceBoundaryV1 {{ service_name: {:?}.to_owned(), domain: {:?}.to_owned(), approved_workflows: {}, owned_aggregates: {} }}),",
            boundary.service_name,
            boundary.service_name,
            boundary.domain,
            string_vec_expr(&boundary.approved_workflows),
            string_vec_expr(&boundary.owned_aggregates),
        );
    }
    output.push_str("        _ => None,\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");

    output.push_str("#[must_use]\n");
    output.push_str("pub fn workflow_boundary_named(name: &str) -> Option<WorkflowBoundaryV1> {\n");
    output.push_str("    match name {\n");
    for boundary in workflow_boundaries {
        let _ = writeln!(
            output,
            "        {:?} => Some(WorkflowBoundaryV1 {{ workflow_name: {:?}.to_owned(), touched_domains: {}, target_services: {}, emits_evidence: {}, mutation_path_only: {} }}),",
            boundary.workflow_name,
            boundary.workflow_name,
            string_vec_expr(&boundary.touched_domains),
            string_vec_expr(&boundary.target_services),
            boundary.emits_evidence,
            boundary.mutation_path_only,
        );
    }
    output.push_str("        _ => None,\n");
    output.push_str("    }\n");
    output.push_str("}\n\n");

    output.push_str("#[must_use]\n");
    output.push_str("pub fn all_service_boundaries() -> Vec<ServiceBoundaryV1> {\n");
    output.push_str("    vec![\n");
    for boundary in service_boundaries {
        let _ = writeln!(
            output,
            "        service_boundary_named({:?}).expect(\"generated service boundary\"),",
            boundary.service_name
        );
    }
    output.push_str("    ]\n");
    output.push_str("}\n\n");

    output.push_str("#[must_use]\n");
    output.push_str("pub fn all_workflow_boundaries() -> Vec<WorkflowBoundaryV1> {\n");
    output.push_str("    vec![\n");
    for boundary in workflow_boundaries {
        let _ = writeln!(
            output,
            "        workflow_boundary_named({:?}).expect(\"generated workflow boundary\"),",
            boundary.workflow_name
        );
    }
    output.push_str("    ]\n");
    output.push_str("}\n");

    output
}

fn string_vec_expr(values: &[String]) -> String {
    let values = values
        .iter()
        .map(|value| format!("{value:?}.to_owned()"))
        .collect::<Vec<_>>()
        .join(", ");
    format!("vec![{values}]")
}
