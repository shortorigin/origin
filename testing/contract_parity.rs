use toml::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogServiceBoundary {
    pub version: String,
    pub primary_service: String,
    pub approved_workflows: Vec<String>,
    pub owned_aggregates: Vec<String>,
}

#[must_use]
pub fn parse_service_boundary_catalog(source: &str) -> CatalogServiceBoundary {
    let value = source
        .parse::<Value>()
        .expect("service boundary catalog should be valid TOML");
    let table = value
        .as_table()
        .expect("service boundary catalog should be a TOML table");

    CatalogServiceBoundary {
        version: string_field(table, "version"),
        primary_service: string_field(table, "primary_service"),
        approved_workflows: string_array_field(table, "approved_workflows"),
        owned_aggregates: string_array_field(table, "owned_aggregates"),
    }
}

pub fn assert_service_boundary_matches_catalog(
    boundary: &contracts::ServiceBoundaryV1,
    expected_domain: &str,
    source: &str,
) {
    let catalog = parse_service_boundary_catalog(source);

    assert_eq!(catalog.version, "v1");
    assert_eq!(boundary.domain, expected_domain);
    assert_eq!(boundary.service_name, catalog.primary_service);
    assert_eq!(boundary.approved_workflows, catalog.approved_workflows);
    assert_eq!(boundary.owned_aggregates, catalog.owned_aggregates);
}

fn string_field(table: &toml::map::Map<String, Value>, key: &str) -> String {
    table
        .get(key)
        .and_then(Value::as_str)
        .unwrap_or_else(|| panic!("missing string field `{key}`"))
        .to_owned()
}

fn string_array_field(table: &toml::map::Map<String, Value>, key: &str) -> Vec<String> {
    table
        .get(key)
        .and_then(Value::as_array)
        .unwrap_or_else(|| panic!("missing string array field `{key}`"))
        .iter()
        .map(|value| {
            value
                .as_str()
                .unwrap_or_else(|| panic!("field `{key}` should contain only strings"))
                .to_owned()
        })
        .collect()
}
