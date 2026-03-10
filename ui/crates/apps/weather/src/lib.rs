//! Built-in Weather app for platform-managed meteorological intelligence surfaces.

#![warn(missing_docs, rustdoc::broken_intra_doc_links)]

use desktop_app_contract::AppServices;
use leptos::{
    component, create_effect, create_rw_signal, view, Callback, CollectView, For, IntoView, Show,
    Signal, SignalGet, SignalSet, SignalUpdate,
};
use sdk_rs::WeatherPlatformSnapshotV1;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use system_ui::components::{AppShell, Button, StatusBar, StatusBarItem, Toolbar};
use system_ui::primitives::{
    ButtonVariant, DataTable, Elevation, Grid, Heading, LayoutGap, Panel, Stack, Surface,
    SurfaceVariant, Text, TextRole, TextTone,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum WeatherSection {
    Overview,
    Features,
    Alerts,
}

impl WeatherSection {
    fn label(self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Features => "Features",
            Self::Alerts => "Alerts",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct WeatherAppState {
    active_section: WeatherSection,
}

impl Default for WeatherAppState {
    fn default() -> Self {
        Self {
            active_section: WeatherSection::Overview,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct WeatherFeatureRow {
    label: String,
    valid_time: String,
    value: String,
    qc: String,
}

fn snapshot_region(snapshot: &WeatherPlatformSnapshotV1) -> String {
    snapshot
        .availability
        .as_ref()
        .map(|availability| availability.region_id.clone())
        .or_else(|| snapshot.view.as_ref().map(|view| view.region_id.clone()))
        .unwrap_or_else(|| "unavailable".to_string())
}

fn snapshot_summary(snapshot: &WeatherPlatformSnapshotV1) -> String {
    let region = snapshot_region(snapshot);
    let layer_count = snapshot
        .availability
        .as_ref()
        .map_or(0, |availability| availability.available_layers.len());
    let alert_count = snapshot.alerts.as_ref().map_or(0, |feed| feed.alerts.len());
    format!("{region} has {layer_count} active layers and {alert_count} alert(s).")
}

fn feature_rows(snapshot: &WeatherPlatformSnapshotV1) -> Vec<WeatherFeatureRow> {
    snapshot
        .feature_slices
        .iter()
        .flat_map(|slice| {
            slice.features.iter().map(move |feature| WeatherFeatureRow {
                label: format!("{:?}", feature.feature),
                valid_time: slice.valid_time.to_rfc3339(),
                value: format!("{:.2} {}", feature.value, feature.units),
                qc: feature
                    .qc_flags
                    .iter()
                    .map(|flag| format!("{flag:?}"))
                    .collect::<Vec<_>>()
                    .join(", "),
            })
        })
        .collect()
}

fn alert_headlines(snapshot: &WeatherPlatformSnapshotV1) -> Vec<String> {
    snapshot
        .alerts
        .as_ref()
        .map(|feed| {
            feed.alerts
                .iter()
                .map(|alert| format!("{} ({})", alert.headline, alert.severity))
                .collect()
        })
        .unwrap_or_default()
}

fn latest_valid_time(snapshot: &WeatherPlatformSnapshotV1) -> String {
    snapshot
        .view
        .as_ref()
        .map(|view| view.valid_time.to_rfc3339())
        .or_else(|| {
            snapshot
                .availability
                .as_ref()
                .and_then(|availability| availability.available_layers.first())
                .map(|layer| layer.latest_valid_time.to_rfc3339())
        })
        .unwrap_or_else(|| "n/a".to_string())
}

#[component]
/// Weather window contents.
pub fn WeatherApp(
    /// Launch parameters supplied by the desktop runtime.
    launch_params: Value,
    /// Previously persisted per-window state restored by the desktop runtime.
    restored_state: Option<Value>,
    /// Capability-scoped host and platform services injected by the runtime.
    services: Option<AppServices>,
) -> impl IntoView {
    let services = services.expect("weather app requires app services");
    let app_state = create_rw_signal(WeatherAppState::default());

    if let Some(restored_state) = restored_state {
        if let Ok(restored) = serde_json::from_value::<WeatherAppState>(restored_state) {
            app_state.set(restored);
        }
    }

    if let Some(section) = launch_params.get("section").and_then(Value::as_str) {
        let parsed = match section {
            "features" => Some(WeatherSection::Features),
            "alerts" => Some(WeatherSection::Alerts),
            "overview" => Some(WeatherSection::Overview),
            _ => None,
        };
        if let Some(section) = parsed {
            app_state.update(|state| state.active_section = section);
        }
    }

    let state_service = services.state.clone();
    create_effect(move |_| {
        if let Ok(serialized) = serde_json::to_value(app_state.get()) {
            state_service.persist_window_state(serialized);
        }
    });

    let weather_snapshot = Signal::derive({
        let services = services.clone();
        move || services.platform.weather.get()
    });

    view! {
        <AppShell>
            <Toolbar aria_label="Weather sections">
                {[WeatherSection::Overview, WeatherSection::Features, WeatherSection::Alerts]
                    .into_iter()
                    .map(|section| {
                        let selected = move || app_state.get().active_section == section;
                        view! {
                            <Button
                                variant=ButtonVariant::Quiet
                                selected=Signal::derive(selected)
                                on_click=Callback::new(move |_| {
                                    app_state.update(|state| state.active_section = section);
                                })
                            >
                                {section.label()}
                            </Button>
                        }
                    })
                    .collect_view()}
            </Toolbar>

            <Panel variant=SurfaceVariant::Standard>
                <Stack gap=LayoutGap::Md>
                    <Heading>"Weather Intelligence"</Heading>
                    <Text tone=TextTone::Secondary>{move || snapshot_summary(&weather_snapshot.get())}</Text>

                    <Show
                        when=move || app_state.get().active_section == WeatherSection::Overview
                        fallback=move || {
                            view! {
                                <Show
                                    when=move || app_state.get().active_section == WeatherSection::Features
                                    fallback=move || {
                                        let alerts = Signal::derive(move || alert_headlines(&weather_snapshot.get()));
                                        view! {
                                            <Surface variant=SurfaceVariant::Muted elevation=Elevation::Raised>
                                                <Stack gap=LayoutGap::Sm>
                                                    <Heading role=TextRole::Title>"Active Alerts"</Heading>
                                                    <For
                                                        each=move || alerts.get()
                                                        key=|headline| headline.clone()
                                                        let:headline
                                                    >
                                                        <Text>{headline}</Text>
                                                    </For>
                                                </Stack>
                                            </Surface>
                                        }
                                    }
                                >
                                    <WeatherFeaturesTable rows=Signal::derive(move || feature_rows(&weather_snapshot.get())) />
                                </Show>
                            }
                        }
                    >
                        <WeatherOverview snapshot=weather_snapshot />
                    </Show>
                </Stack>
            </Panel>

            <StatusBar>
                <StatusBarItem>{move || format!("Region: {}", snapshot_region(&weather_snapshot.get()))}</StatusBarItem>
                <StatusBarItem>{move || format!("Latest valid: {}", latest_valid_time(&weather_snapshot.get()))}</StatusBarItem>
                <StatusBarItem>{move || format!("Feature slices: {}", weather_snapshot.get().feature_slices.len())}</StatusBarItem>
            </StatusBar>
        </AppShell>
    }
}

#[component]
fn WeatherOverview(snapshot: Signal<WeatherPlatformSnapshotV1>) -> impl IntoView {
    view! {
        <Grid>
            <Surface variant=SurfaceVariant::Muted elevation=Elevation::Raised>
                <Stack gap=LayoutGap::Sm>
                    <Text role=TextRole::Label>"Region"</Text>
                    <Text>{move || snapshot_region(&snapshot.get())}</Text>
                </Stack>
            </Surface>
            <Surface variant=SurfaceVariant::Muted elevation=Elevation::Raised>
                <Stack gap=LayoutGap::Sm>
                    <Text role=TextRole::Label>"Latest valid time"</Text>
                    <Text>{move || latest_valid_time(&snapshot.get())}</Text>
                </Stack>
            </Surface>
            <Surface variant=SurfaceVariant::Muted elevation=Elevation::Raised>
                <Stack gap=LayoutGap::Sm>
                    <Text role=TextRole::Label>"Available layers"</Text>
                    <Text>{move || snapshot.get().availability.map_or(0, |availability| availability.available_layers.len()).to_string()}</Text>
                </Stack>
            </Surface>
            <Surface variant=SurfaceVariant::Muted elevation=Elevation::Raised>
                <Stack gap=LayoutGap::Sm>
                    <Text role=TextRole::Label>"Alert feed"</Text>
                    <Text>{move || snapshot.get().alerts.map_or(0, |feed| feed.alerts.len()).to_string()}</Text>
                </Stack>
            </Surface>
        </Grid>
    }
}

#[component]
fn WeatherFeaturesTable(rows: Signal<Vec<WeatherFeatureRow>>) -> impl IntoView {
    view! {
        <Surface variant=SurfaceVariant::Muted elevation=Elevation::Raised>
            <Stack gap=LayoutGap::Sm>
                <Heading role=TextRole::Title>"Feature Products"</Heading>
                <DataTable aria_label="Weather feature table">
                    <thead>
                        <tr>
                            <th>"Feature"</th>
                            <th>"Valid time"</th>
                            <th>"Value"</th>
                            <th>"QC"</th>
                        </tr>
                    </thead>
                    <tbody>
                        <For each=move || rows.get() key=|row| format!("{}{}", row.label, row.valid_time) let:row>
                            <tr>
                                <td>{row.label}</td>
                                <td>{row.valid_time}</td>
                                <td>{row.value}</td>
                                <td>{row.qc}</td>
                            </tr>
                        </For>
                    </tbody>
                </DataTable>
            </Stack>
        </Surface>
    }
}

#[cfg(test)]
mod tests {
    use sdk_rs::WeatherPlatformSnapshotV1;

    use super::{alert_headlines, feature_rows, snapshot_summary};

    fn load_snapshot() -> WeatherPlatformSnapshotV1 {
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../../testing/fixtures/weather/run-2026-03-10/platform_snapshot.json"
        )))
        .expect("weather snapshot fixture")
    }

    #[test]
    fn snapshot_summary_mentions_region_and_alerts() {
        let summary = snapshot_summary(&load_snapshot());
        assert!(summary.contains("us-west"));
        assert!(summary.contains("alert"));
    }

    #[test]
    fn feature_rows_flatten_weather_feature_products() {
        let rows = feature_rows(&load_snapshot());
        assert_eq!(rows.len(), 4);
        assert!(rows
            .iter()
            .any(|row| row.label.contains("PrecipitationRate")));
    }

    #[test]
    fn alert_headlines_surface_active_alerts() {
        let headlines = alert_headlines(&load_snapshot());
        assert_eq!(headlines, vec!["Flood Watch (moderate)"]);
    }
}
