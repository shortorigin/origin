use tempfile::tempdir;

#[test]
fn schema_export_writes_grouped_output_layout() {
    let tempdir = tempdir().expect("tempdir");
    let written = xtask::export_schemas(tempdir.path()).expect("export schemas");

    assert!(!written.is_empty());
    assert!(tempdir.path().join("contracts/work-item-v1.json").is_file());
    assert!(tempdir
        .path()
        .join("events/signal-generated-v1.json")
        .is_file());
    assert!(tempdir
        .path()
        .join("surrealdb/record-types-v1.json")
        .is_file());
}

#[test]
fn docs_preview_plan_forwards_passthrough_arguments() {
    let workspace_root = xtask::workspace_root().expect("workspace root");
    let book_root = workspace_root.join("docs/security-rust/book");
    let passthrough = vec![
        "--hostname".to_string(),
        "127.0.0.1".to_string(),
        "--port".to_string(),
        "3001".to_string(),
    ];

    let plan = xtask::security_book::preview_plan(&book_root, &passthrough);

    assert_eq!(plan.program, "mdbook");
    assert_eq!(plan.current_dir, book_root);
    assert_eq!(
        plan.args,
        vec![
            "serve".to_string(),
            "--hostname".to_string(),
            "127.0.0.1".to_string(),
            "--port".to_string(),
            "3001".to_string(),
        ]
    );
}
