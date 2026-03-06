use std::sync::Arc;

use futures::future::{self, BoxFuture};
use futures::stream::{self, BoxStream};

use contracts::{
    AgentActionRequestV1, MarketDataBatchV1, PromotionRecommendationV1,
    QuantStrategyPromotionRequestV1, ReleaseApprovalRecordV1, ReleaseApprovalRequestV1,
    ServiceBoundaryV1, WorkflowBoundaryV1,
};
use error_model::{InstitutionalError, InstitutionalResult};
use events::EventEnvelopeV1;
use lattice_config::LatticeConfigV1;
use serde::{Deserialize, Serialize};

pub type TransportFuture<T> = BoxFuture<'static, InstitutionalResult<T>>;
pub type EventSubscription = BoxStream<'static, EventEnvelopeV1>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReleasedUiAppV1 {
    pub app_id: String,
    pub display_name: String,
    pub desktop_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiDashboardSnapshotV1 {
    pub client_name: String,
    pub services: Vec<ServiceBoundaryV1>,
    pub workflows: Vec<WorkflowBoundaryV1>,
    pub lattice: Option<LatticeConfigV1>,
    pub release_apps: Vec<ReleasedUiAppV1>,
    pub connected_cache: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "command_type", content = "payload", rename_all = "snake_case")]
pub enum PlatformCommandV1 {
    DispatchAgentAction(AgentActionRequestV1),
    SubmitReleaseApproval {
        action: AgentActionRequestV1,
        request: ReleaseApprovalRequestV1,
    },
    PreparePromotion {
        action: AgentActionRequestV1,
        request: QuantStrategyPromotionRequestV1,
    },
    RegisterMarketDataBatch(MarketDataBatchV1),
    SubmitPromotionRecommendation(PromotionRecommendationV1),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlatformCommandAckV1 {
    pub command_id: String,
    pub accepted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "result_type", content = "payload", rename_all = "snake_case")]
pub enum PlatformCommandResultV1 {
    Ack(PlatformCommandAckV1),
    ReleaseApproval(ReleaseApprovalRecordV1),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "query_type", content = "payload", rename_all = "snake_case")]
pub enum PlatformQueryV1 {
    Dashboard,
    SupportedWorkflows,
    RecentEvents { limit: usize },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "result_type", content = "payload", rename_all = "snake_case")]
pub enum PlatformQueryResultV1 {
    Dashboard(UiDashboardSnapshotV1),
    SupportedWorkflows(Vec<WorkflowBoundaryV1>),
    RecentEvents(Vec<EventEnvelopeV1>),
}

pub trait InstitutionalPlatformTransport: Send + Sync {
    fn execute_command(
        &self,
        command: PlatformCommandV1,
    ) -> TransportFuture<PlatformCommandResultV1>;
    fn execute_query(&self, query: PlatformQueryV1) -> TransportFuture<PlatformQueryResultV1>;
    fn subscribe_events(&self) -> EventSubscription;
}

impl<T> InstitutionalPlatformTransport for Arc<T>
where
    T: InstitutionalPlatformTransport + ?Sized,
{
    fn execute_command(
        &self,
        command: PlatformCommandV1,
    ) -> TransportFuture<PlatformCommandResultV1> {
        (**self).execute_command(command)
    }

    fn execute_query(&self, query: PlatformQueryV1) -> TransportFuture<PlatformQueryResultV1> {
        (**self).execute_query(query)
    }

    fn subscribe_events(&self) -> EventSubscription {
        (**self).subscribe_events()
    }
}

#[derive(Debug, Clone, Default)]
pub struct NoopPlatformTransport;

impl InstitutionalPlatformTransport for NoopPlatformTransport {
    fn execute_command(
        &self,
        _command: PlatformCommandV1,
    ) -> TransportFuture<PlatformCommandResultV1> {
        Box::pin(future::ready(Err(InstitutionalError::PolicyDenied {
            reason: "no platform transport configured".to_string(),
        })))
    }

    fn execute_query(&self, _query: PlatformQueryV1) -> TransportFuture<PlatformQueryResultV1> {
        Box::pin(future::ready(Err(InstitutionalError::PolicyDenied {
            reason: "no platform transport configured".to_string(),
        })))
    }

    fn subscribe_events(&self) -> EventSubscription {
        Box::pin(stream::empty())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstitutionalPlatformClientV1 {
    pub client_name: String,
    pub supported_services: Vec<ServiceBoundaryV1>,
    pub supported_workflows: Vec<WorkflowBoundaryV1>,
    pub lattice_config: Option<LatticeConfigV1>,
}

impl InstitutionalPlatformClientV1 {
    #[must_use]
    pub fn prepare_action(&self, action: AgentActionRequestV1) -> AgentActionRequestV1 {
        action
    }

    #[must_use]
    pub fn receive_event(&self, envelope: EventEnvelopeV1) -> EventEnvelopeV1 {
        envelope
    }

    #[must_use]
    pub fn prepare_quant_strategy_promotion(
        &self,
        action: AgentActionRequestV1,
        request: QuantStrategyPromotionRequestV1,
    ) -> (AgentActionRequestV1, QuantStrategyPromotionRequestV1) {
        (action, request)
    }

    #[must_use]
    pub fn register_market_data_batch(&self, batch: MarketDataBatchV1) -> MarketDataBatchV1 {
        batch
    }

    #[must_use]
    pub fn submit_promotion_recommendation(
        &self,
        recommendation: PromotionRecommendationV1,
    ) -> PromotionRecommendationV1 {
        recommendation
    }

    #[must_use]
    pub fn dashboard_snapshot(
        &self,
        release_apps: Vec<ReleasedUiAppV1>,
        connected_cache: bool,
    ) -> UiDashboardSnapshotV1 {
        UiDashboardSnapshotV1 {
            client_name: self.client_name.clone(),
            services: self.supported_services.clone(),
            workflows: self.supported_workflows.clone(),
            lattice: self.lattice_config.clone(),
            release_apps,
            connected_cache,
        }
    }
}

#[derive(Clone)]
pub struct InstitutionalPlatformRuntimeClient<T>
where
    T: InstitutionalPlatformTransport,
{
    manifest: InstitutionalPlatformClientV1,
    transport: T,
}

impl<T> InstitutionalPlatformRuntimeClient<T>
where
    T: InstitutionalPlatformTransport,
{
    #[must_use]
    pub fn new(manifest: InstitutionalPlatformClientV1, transport: T) -> Self {
        Self {
            manifest,
            transport,
        }
    }

    #[must_use]
    pub fn manifest(&self) -> &InstitutionalPlatformClientV1 {
        &self.manifest
    }

    pub async fn execute_command(
        &self,
        command: PlatformCommandV1,
    ) -> InstitutionalResult<PlatformCommandResultV1> {
        self.transport.execute_command(command).await
    }

    pub async fn execute_query(
        &self,
        query: PlatformQueryV1,
    ) -> InstitutionalResult<PlatformQueryResultV1> {
        self.transport.execute_query(query).await
    }

    pub async fn query_dashboard(&self) -> InstitutionalResult<UiDashboardSnapshotV1> {
        match self
            .transport
            .execute_query(PlatformQueryV1::Dashboard)
            .await?
        {
            PlatformQueryResultV1::Dashboard(snapshot) => Ok(snapshot),
            PlatformQueryResultV1::SupportedWorkflows(_)
            | PlatformQueryResultV1::RecentEvents(_) => {
                Err(InstitutionalError::InvariantViolation {
                    invariant: "dashboard query returned non-dashboard payload".to_string(),
                })
            }
        }
    }

    pub async fn query_supported_workflows(&self) -> InstitutionalResult<Vec<WorkflowBoundaryV1>> {
        match self
            .transport
            .execute_query(PlatformQueryV1::SupportedWorkflows)
            .await?
        {
            PlatformQueryResultV1::SupportedWorkflows(workflows) => Ok(workflows),
            PlatformQueryResultV1::Dashboard(_) | PlatformQueryResultV1::RecentEvents(_) => {
                Err(InstitutionalError::InvariantViolation {
                    invariant: "workflow query returned non-workflow payload".to_string(),
                })
            }
        }
    }

    pub async fn submit_release_approval(
        &self,
        action: AgentActionRequestV1,
        request: ReleaseApprovalRequestV1,
    ) -> InstitutionalResult<ReleaseApprovalRecordV1> {
        match self
            .execute_command(PlatformCommandV1::SubmitReleaseApproval { action, request })
            .await?
        {
            PlatformCommandResultV1::ReleaseApproval(record) => Ok(record),
            PlatformCommandResultV1::Ack(_) => Err(InstitutionalError::InvariantViolation {
                invariant: "release approval command returned non-release payload".to_string(),
            }),
        }
    }

    pub async fn query_recent_events(
        &self,
        limit: usize,
    ) -> InstitutionalResult<Vec<EventEnvelopeV1>> {
        match self
            .transport
            .execute_query(PlatformQueryV1::RecentEvents { limit })
            .await?
        {
            PlatformQueryResultV1::RecentEvents(events) => Ok(events),
            PlatformQueryResultV1::Dashboard(_) | PlatformQueryResultV1::SupportedWorkflows(_) => {
                Err(InstitutionalError::InvariantViolation {
                    invariant: "recent events query returned non-event payload".to_string(),
                })
            }
        }
    }

    #[must_use]
    pub fn subscribe_events(&self) -> EventSubscription {
        self.transport.subscribe_events()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemoryPlatformTransport {
    dashboard: UiDashboardSnapshotV1,
    recent_events: Vec<EventEnvelopeV1>,
}

impl MemoryPlatformTransport {
    #[must_use]
    pub fn new(dashboard: UiDashboardSnapshotV1, recent_events: Vec<EventEnvelopeV1>) -> Self {
        Self {
            dashboard,
            recent_events,
        }
    }
}

impl InstitutionalPlatformTransport for MemoryPlatformTransport {
    fn execute_command(
        &self,
        _command: PlatformCommandV1,
    ) -> TransportFuture<PlatformCommandResultV1> {
        Box::pin(future::ready(Ok(PlatformCommandResultV1::Ack(
            PlatformCommandAckV1 {
                command_id: "memory-ack".to_string(),
                accepted: true,
            },
        ))))
    }

    fn execute_query(&self, query: PlatformQueryV1) -> TransportFuture<PlatformQueryResultV1> {
        let result = match query {
            PlatformQueryV1::Dashboard => PlatformQueryResultV1::Dashboard(self.dashboard.clone()),
            PlatformQueryV1::SupportedWorkflows => {
                PlatformQueryResultV1::SupportedWorkflows(self.dashboard.workflows.clone())
            }
            PlatformQueryV1::RecentEvents { limit } => PlatformQueryResultV1::RecentEvents(
                self.recent_events.iter().take(limit).cloned().collect(),
            ),
        };
        Box::pin(future::ready(Ok(result)))
    }

    fn subscribe_events(&self) -> EventSubscription {
        Box::pin(stream::iter(self.recent_events.clone()))
    }
}

#[cfg(test)]
mod tests {
    use contracts::Classification;
    use futures::StreamExt;
    use identity::ActorRef;

    use super::*;

    #[tokio::test]
    async fn memory_transport_serves_dashboard_and_events() {
        let manifest = InstitutionalPlatformClientV1 {
            client_name: "ui-shell".to_string(),
            supported_services: Vec::new(),
            supported_workflows: Vec::new(),
            lattice_config: None,
        };
        let dashboard = manifest.dashboard_snapshot(
            vec![ReleasedUiAppV1 {
                app_id: "system.control-center".to_string(),
                display_name: "Control Center".to_string(),
                desktop_enabled: true,
            }],
            true,
        );
        let event = EventEnvelopeV1::new(
            "shell.started",
            ActorRef("ui-shell".to_string()),
            "corr-1",
            None,
            Classification::Internal,
            "schemas/events/v1/shell-started",
            "abc",
        );
        let transport = MemoryPlatformTransport::new(dashboard.clone(), vec![event.clone()]);
        let client = InstitutionalPlatformRuntimeClient::new(manifest, transport);

        assert_eq!(
            client.query_dashboard().await.expect("dashboard"),
            dashboard
        );
        assert_eq!(
            client
                .query_supported_workflows()
                .await
                .expect("workflows")
                .len(),
            0
        );
        assert!(matches!(
            client
                .execute_command(PlatformCommandV1::DispatchAgentAction(
                    AgentActionRequestV1 {
                        action_id: "action-1".to_string(),
                        actor_ref: ActorRef("ui-shell".to_string()),
                        objective: "noop".to_string(),
                        requested_workflow: "release_approval".to_string(),
                        impact_tier: contracts::ImpactTier::Tier0,
                        classification: Classification::Internal,
                        required_approver_roles: Vec::new(),
                        policy_refs: vec!["policy.test.v1".to_string()],
                    }
                ))
                .await
                .expect("command"),
            PlatformCommandResultV1::Ack(_)
        ));
        let events = client.subscribe_events().collect::<Vec<_>>().await;
        assert_eq!(events, vec![event]);
    }
}
