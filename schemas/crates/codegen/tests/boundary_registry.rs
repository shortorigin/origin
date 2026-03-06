use codegen::embedded_boundary_catalogs;
use contracts::{all_service_boundaries, all_workflow_boundaries};

#[test]
fn boundary_registry_embeds_generated_service_and_workflow_catalogs() {
    let catalogs = embedded_boundary_catalogs().expect("boundary catalogs");

    assert_eq!(catalogs.len(), 2);
    assert!(catalogs
        .iter()
        .any(|catalog| catalog.name == "service-boundaries-v1"));
    assert!(catalogs
        .iter()
        .any(|catalog| catalog.name == "workflow-boundaries-v1"));

    let service_catalog = catalogs
        .iter()
        .find(|catalog| catalog.name == "service-boundaries-v1")
        .expect("service catalog");
    let workflow_catalog = catalogs
        .iter()
        .find(|catalog| catalog.name == "workflow-boundaries-v1")
        .expect("workflow catalog");

    assert_eq!(
        service_catalog.document["service_boundaries"]
            .as_array()
            .expect("service boundaries")
            .len(),
        all_service_boundaries().len()
    );
    assert_eq!(
        workflow_catalog.document["workflow_boundaries"]
            .as_array()
            .expect("workflow boundaries")
            .len(),
        all_workflow_boundaries().len()
    );
}
