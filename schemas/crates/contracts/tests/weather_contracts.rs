use chrono::{TimeZone, Utc};
use contracts::{
    GeoBoundsV1, WeatherAlertFeedV1, WeatherAlertV1, WeatherArtifactKindV1, WeatherAvailabilityV1,
    WeatherFeatureKindV1, WeatherFeatureSliceV1, WeatherFeatureValueV1, WeatherLayerAvailabilityV1,
    WeatherLayerKindV1, WeatherProvenanceV1, WeatherQcFlagV1, WeatherSourceKindV1,
    WeatherViewLayerV1, WeatherViewV1,
};

#[test]
fn weather_contracts_round_trip_through_json() {
    let provenance = WeatherProvenanceV1 {
        source_kind: WeatherSourceKindV1::Hrrr,
        source_dataset: "hrrr".to_string(),
        source_object_ref: "s3://noaa-hrrr/example.grib2".to_string(),
        retrieved_at: Utc
            .with_ymd_and_hms(2026, 3, 10, 11, 0, 0)
            .single()
            .expect("time"),
        raw_artifact_hash: "abc123".to_string(),
        transform_version: "weather/v1".to_string(),
        config_hash: "cfg-1".to_string(),
    };
    let availability = WeatherAvailabilityV1 {
        catalog_id: "catalog-west".to_string(),
        region_id: "us-west".to_string(),
        bounds: GeoBoundsV1 {
            north: 49.0,
            south: 31.0,
            east: -109.0,
            west: -125.0,
        },
        generated_at: provenance.retrieved_at,
        source_kinds: vec![WeatherSourceKindV1::Hrrr, WeatherSourceKindV1::NwsAlert],
        available_layers: vec![WeatherLayerAvailabilityV1 {
            layer: WeatherLayerKindV1::Precipitation,
            latest_event_time: provenance.retrieved_at,
            latest_valid_time: Utc
                .with_ymd_and_hms(2026, 3, 10, 15, 0, 0)
                .single()
                .expect("time"),
            horizon_hours: 4,
            product_ref: "product-hrrr-001".to_string(),
        }],
    };
    let view = WeatherViewV1 {
        view_id: "view-west".to_string(),
        region_id: "us-west".to_string(),
        bounds: availability.bounds.clone(),
        generated_at: availability.generated_at,
        valid_time: availability.available_layers[0].latest_valid_time,
        horizon_hours: 4,
        layers: vec![WeatherViewLayerV1 {
            layer: WeatherLayerKindV1::Precipitation,
            title: "HRRR Precipitation".to_string(),
            asset_ref: "zarr://weather/views/west/precip".to_string(),
            native_identifier: "APCP".to_string(),
            event_time: availability.available_layers[0].latest_event_time,
            valid_time: availability.available_layers[0].latest_valid_time,
            qc_flags: vec![WeatherQcFlagV1::Passed],
            provenance: vec![provenance.clone()],
        }],
        alert_feed_id: Some("alerts-west".to_string()),
    };
    let feature_slice = WeatherFeatureSliceV1 {
        slice_id: "slice-west-1".to_string(),
        region_id: "us-west".to_string(),
        event_time: availability.available_layers[0].latest_event_time,
        valid_time: availability.available_layers[0].latest_valid_time,
        lead_hours: 4,
        source_product_refs: vec!["product-hrrr-001".to_string()],
        features: vec![WeatherFeatureValueV1 {
            feature: WeatherFeatureKindV1::PrecipitationRate,
            value: 2.4,
            units: "mm/hr".to_string(),
            probability: Some(0.62),
            qc_flags: vec![WeatherQcFlagV1::Passed],
        }],
        provenance: vec![provenance.clone()],
    };
    let alerts = WeatherAlertFeedV1 {
        feed_id: "alerts-west".to_string(),
        region_id: "us-west".to_string(),
        generated_at: availability.generated_at,
        alerts: vec![WeatherAlertV1 {
            alert_id: "alert-1".to_string(),
            region_id: "us-west".to_string(),
            headline: "Flood Watch".to_string(),
            severity: "moderate".to_string(),
            effective_at: availability.generated_at,
            expires_at: availability.available_layers[0].latest_valid_time,
            source: "nws".to_string(),
            bounds: availability.bounds.clone(),
            provenance: vec![provenance],
        }],
    };

    for value in [
        serde_json::to_value(&availability).expect("availability"),
        serde_json::to_value(&view).expect("view"),
        serde_json::to_value(&feature_slice).expect("feature"),
        serde_json::to_value(&alerts).expect("alerts"),
        serde_json::to_value(WeatherArtifactKindV1::ForecastGrid).expect("artifact"),
    ] {
        let encoded = serde_json::to_string(&value).expect("stringify");
        let _: serde_json::Value = serde_json::from_str(&encoded).expect("parse");
    }
}
