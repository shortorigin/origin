use runtime_security::{runtime_lab_fixture_path, validate_service_binding};
use wasmcloud_bindings::{SignedComponentRefV1, WasmComponentBindingV1};

fn main() {
    let binding = WasmComponentBindingV1::service(
        "sandboxing-demo",
        SignedComponentRefV1 {
            component_ref: "ghcr.io/shortorigin/sandboxing-demo:wasm".to_string(),
            digest: "sha256:sandboxing-demo".to_string(),
            signature_ref: None,
        },
        "lab",
        vec!["schemas/wit/v1/platform.wit".to_string()],
        Vec::new(),
    );
    validate_service_binding(&binding).expect("service world");

    println!(
        "validated service-world binding using fixture {}",
        runtime_lab_fixture_path("busy-loop.wat").display()
    );
}
