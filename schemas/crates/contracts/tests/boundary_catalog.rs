use std::collections::BTreeSet;
use std::path::PathBuf;

use contracts::{
    all_service_boundaries, all_workflow_boundaries, service_boundary_catalog_document,
    workflow_boundary_catalog_document,
};

#[test]
fn workflow_targets_are_explicitly_approved_by_target_services() {
    let services = all_service_boundaries()
        .into_iter()
        .map(|boundary| (boundary.service_name.clone(), boundary))
        .collect::<std::collections::BTreeMap<_, _>>();

    for workflow in all_workflow_boundaries() {
        for target_service in &workflow.target_services {
            let service = services
                .get(target_service)
                .unwrap_or_else(|| panic!("missing service boundary for `{target_service}`"));
            assert!(
                service.approved_workflows.contains(&workflow.workflow_name),
                "workflow `{}` targets `{target_service}` without approval",
                workflow.workflow_name
            );
        }
    }
}

#[test]
fn approved_workflows_reference_existing_workflows() {
    let workflows = all_workflow_boundaries()
        .into_iter()
        .map(|boundary| boundary.workflow_name)
        .collect::<BTreeSet<_>>();

    for service in all_service_boundaries() {
        for workflow_name in &service.approved_workflows {
            assert!(
                workflows.contains(workflow_name),
                "service `{}` references unknown workflow `{workflow_name}`",
                service.service_name
            );
        }
    }
}

#[test]
fn boundary_catalogs_reference_existing_enterprise_domains() {
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(3)
        .expect("repo root")
        .to_path_buf();

    for service in all_service_boundaries() {
        let domain_dir = repo_root.join("enterprise/domains").join(&service.domain);
        assert!(
            domain_dir.is_dir(),
            "service `{}` references unknown domain `{}`",
            service.service_name,
            service.domain
        );
    }

    for workflow in all_workflow_boundaries() {
        for domain in &workflow.touched_domains {
            let domain_dir = repo_root.join("enterprise/domains").join(domain);
            assert!(
                domain_dir.is_dir(),
                "workflow `{}` references unknown domain `{domain}`",
                workflow.workflow_name
            );
        }
    }
}

#[test]
fn boundary_catalog_documents_round_trip_through_json() {
    let service_catalog = service_boundary_catalog_document();
    let workflow_catalog = workflow_boundary_catalog_document();

    assert_eq!(
        service_catalog["service_boundaries"]
            .as_array()
            .expect("service boundaries")
            .len(),
        all_service_boundaries().len()
    );
    assert_eq!(
        workflow_catalog["workflow_boundaries"]
            .as_array()
            .expect("workflow boundaries")
            .len(),
        all_workflow_boundaries().len()
    );
}
