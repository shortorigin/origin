use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use contracts::{
    ServiceBoundaryV1, WeatherAlertFeedV1, WeatherArtifactKindV1, WeatherAvailabilityV1,
    WeatherFeatureSliceV1, WeatherProvenanceV1, WeatherQcFlagV1,
};
use error_model::{InstitutionalError, InstitutionalResult, OperationContext};
use events::{WeatherAlertUpdatedV1, WeatherProductPublishedV1};
use serde::{Deserialize, Serialize};

const SERVICE_NAME: &str = "meteorological-service";
const DOMAIN_NAME: &str = "meteorological_intelligence";
const APPROVED_WORKFLOWS: &[&str] = &["weather_ingestion"];
const OWNED_AGGREGATES: &[&str] = &[
    "weather_dataset",
    "weather_feature_product",
    "weather_alert",
];

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawSourceAsset {
    pub asset_id: String,
    pub source_kind: contracts::WeatherSourceKindV1,
    pub source_ref: String,
    pub bytes_sha256: String,
    pub retrieved_at: DateTime<Utc>,
    pub upstream_qc_notes: Vec<String>,
    pub provenance: WeatherProvenanceV1,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NormalizedWeatherProduct {
    pub product_id: String,
    pub region_id: String,
    pub artifact_kind: WeatherArtifactKindV1,
    pub native_identifier: String,
    pub event_time: DateTime<Utc>,
    pub valid_time: DateTime<Utc>,
    pub lead_hours: u16,
    pub raw_asset_ids: Vec<String>,
    pub qc_flags: Vec<WeatherQcFlagV1>,
    pub provenance: Vec<WeatherProvenanceV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WeatherFixtureBatchV1 {
    pub batch_id: String,
    pub region_id: String,
    pub raw_assets: Vec<RawSourceAsset>,
    pub normalized_products: Vec<NormalizedWeatherProduct>,
    pub availability: WeatherAvailabilityV1,
    pub view: contracts::WeatherViewV1,
    pub feature_slices: Vec<WeatherFeatureSliceV1>,
    pub alerts: WeatherAlertFeedV1,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WeatherIngestionReport {
    pub batch_id: String,
    pub raw_asset_count: usize,
    pub normalized_product_count: usize,
    pub feature_slice_count: usize,
    pub alert_count: usize,
    pub latest_availability: WeatherAvailabilityV1,
}

#[derive(Debug, Default, Clone)]
struct MeteorologicalCatalog {
    raw_assets: Vec<RawSourceAsset>,
    normalized_products: Vec<NormalizedWeatherProduct>,
    availability_by_region: BTreeMap<String, WeatherAvailabilityV1>,
    view_by_region: BTreeMap<String, contracts::WeatherViewV1>,
    feature_slices_by_region: BTreeMap<String, Vec<WeatherFeatureSliceV1>>,
    alerts_by_region: BTreeMap<String, WeatherAlertFeedV1>,
    published_products: Vec<WeatherProductPublishedV1>,
    updated_alerts: Vec<WeatherAlertUpdatedV1>,
}

#[derive(Debug, Default, Clone)]
pub struct MeteorologicalService {
    catalog: MeteorologicalCatalog,
}

impl MeteorologicalService {
    pub fn ingest_fixture_batch(
        &mut self,
        batch: WeatherFixtureBatchV1,
    ) -> InstitutionalResult<WeatherIngestionReport> {
        if batch.raw_assets.is_empty() {
            return Err(InstitutionalError::invariant(
                OperationContext::new("services/meteorological-service", "ingest_fixture_batch"),
                "weather ingest requires at least one raw source asset",
            ));
        }
        if batch.normalized_products.is_empty() {
            return Err(InstitutionalError::invariant(
                OperationContext::new("services/meteorological-service", "ingest_fixture_batch"),
                "weather ingest requires at least one normalized weather product",
            ));
        }

        self.catalog
            .raw_assets
            .extend(batch.raw_assets.iter().cloned());
        self.catalog
            .normalized_products
            .extend(batch.normalized_products.iter().cloned());
        self.catalog
            .availability_by_region
            .insert(batch.region_id.clone(), batch.availability.clone());
        self.catalog
            .view_by_region
            .insert(batch.region_id.clone(), batch.view.clone());
        self.catalog
            .feature_slices_by_region
            .insert(batch.region_id.clone(), batch.feature_slices.clone());
        self.catalog
            .alerts_by_region
            .insert(batch.region_id.clone(), batch.alerts.clone());

        self.catalog
            .published_products
            .extend(
                batch
                    .normalized_products
                    .iter()
                    .map(|product| WeatherProductPublishedV1 {
                        product_ref: product.product_id.clone(),
                        region_id: product.region_id.clone(),
                        artifact_kind: product.artifact_kind,
                        native_identifier: product.native_identifier.clone(),
                        event_time: product.event_time,
                        valid_time: product.valid_time,
                    }),
            );
        self.catalog.updated_alerts.push(WeatherAlertUpdatedV1 {
            feed: batch.alerts.clone(),
        });

        Ok(WeatherIngestionReport {
            batch_id: batch.batch_id,
            raw_asset_count: batch.raw_assets.len(),
            normalized_product_count: batch.normalized_products.len(),
            feature_slice_count: batch.feature_slices.len(),
            alert_count: batch.alerts.alerts.len(),
            latest_availability: batch.availability,
        })
    }

    pub fn weather_availability(&self, region_id: &str) -> Option<&WeatherAvailabilityV1> {
        self.catalog.availability_by_region.get(region_id)
    }

    pub fn weather_view(&self, region_id: &str) -> Option<&contracts::WeatherViewV1> {
        self.catalog.view_by_region.get(region_id)
    }

    pub fn weather_feature_slices(&self, region_id: &str) -> &[WeatherFeatureSliceV1] {
        self.catalog
            .feature_slices_by_region
            .get(region_id)
            .map_or(&[], Vec::as_slice)
    }

    pub fn weather_alert_feed(&self, region_id: &str) -> Option<&WeatherAlertFeedV1> {
        self.catalog.alerts_by_region.get(region_id)
    }

    pub fn published_products(&self) -> &[WeatherProductPublishedV1] {
        &self.catalog.published_products
    }

    pub fn updated_alerts(&self) -> &[WeatherAlertUpdatedV1] {
        &self.catalog.updated_alerts
    }
}

pub fn service_boundary() -> ServiceBoundaryV1 {
    ServiceBoundaryV1 {
        service_name: SERVICE_NAME.to_owned(),
        domain: DOMAIN_NAME.to_owned(),
        approved_workflows: APPROVED_WORKFLOWS.iter().copied().map(Into::into).collect(),
        owned_aggregates: OWNED_AGGREGATES.iter().copied().map(Into::into).collect(),
    }
}

#[cfg(test)]
mod tests {
    mod contract_parity {
        include!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../testing/contract_parity.rs"
        ));
    }

    use contract_parity::assert_service_boundary_matches_catalog;

    use super::{service_boundary, MeteorologicalService, WeatherFixtureBatchV1, DOMAIN_NAME};

    fn load_fixture() -> WeatherFixtureBatchV1 {
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../testing/fixtures/weather/run-2026-03-10/noaa_weather_batch.json"
        )))
        .expect("weather fixture json")
    }

    #[test]
    fn service_boundary_matches_enterprise_catalog() {
        let source = include_str!(
            "../../../enterprise/domains/meteorological_intelligence/service_boundaries.toml"
        );
        let boundary = service_boundary();

        assert_service_boundary_matches_catalog(&boundary, DOMAIN_NAME, source);
    }

    #[test]
    fn ingest_fixture_populates_queryable_weather_products() {
        let fixture = load_fixture();
        let mut service = MeteorologicalService::default();

        let report = service.ingest_fixture_batch(fixture).expect("ingest");

        assert_eq!(report.raw_asset_count, 6);
        assert_eq!(report.normalized_product_count, 4);
        assert_eq!(report.feature_slice_count, 2);
        assert_eq!(report.alert_count, 1);
        assert_eq!(
            service
                .weather_availability("us-west")
                .expect("availability")
                .available_layers
                .len(),
            3
        );
        assert_eq!(service.weather_feature_slices("us-west").len(), 2);
        assert_eq!(service.published_products().len(), 4);
        assert_eq!(service.updated_alerts().len(), 1);
    }

    #[test]
    fn view_and_features_share_weather_provenance_roots() {
        let fixture = load_fixture();
        let mut service = MeteorologicalService::default();
        service.ingest_fixture_batch(fixture).expect("ingest");

        let view_hashes = service
            .weather_view("us-west")
            .expect("view")
            .layers
            .iter()
            .flat_map(|layer| {
                layer
                    .provenance
                    .iter()
                    .map(|record| record.raw_artifact_hash.clone())
            })
            .collect::<Vec<_>>();
        let feature_hashes = service
            .weather_feature_slices("us-west")
            .iter()
            .flat_map(|slice| {
                slice
                    .provenance
                    .iter()
                    .map(|record| record.raw_artifact_hash.clone())
            })
            .collect::<Vec<_>>();

        assert!(view_hashes.iter().any(|hash| feature_hashes.contains(hash)));
    }
}
