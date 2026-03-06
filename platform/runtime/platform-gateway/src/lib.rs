use std::sync::{Arc, Mutex};

use approval_service::ApprovalService;
use component_descriptors::{
    engineering_service::component_binding as engineering_component,
    release_approval::component_binding as release_approval_component,
    security_service::component_binding as security_component,
};
use contracts::{
    AgentActionRequestV1, ApprovalDecisionV1, Classification, ReleaseApprovalRecordV1,
    ReleaseApprovalRequestV1,
};
use engineering_service::EngineeringService;
use error_model::InstitutionalResult;
use events::{
    EventEnvelopeV1, ReleaseApprovalApprovedV1, ReleaseApprovalDeniedV1, ReleaseApprovalRequestedV1,
};
use evidence_service::{AuditEvent, EvidenceService};
use futures::channel::mpsc;
use futures::future::{self};
use futures::StreamExt;
use lattice_config::{LatticeConfigV1, RolloutTargetV1};
use orchestrator::WorkflowEngine;
use policy_service::PolicyService;
use sdk_rs::{
    EventSubscription, InstitutionalPlatformClientV1, InstitutionalPlatformTransport,
    PlatformCommandResultV1, PlatformCommandV1, PlatformQueryResultV1, PlatformQueryV1,
    ReleasedUiAppV1, TransportFuture, UiDashboardSnapshotV1,
};
use security_service::SecurityService;
use sha2::{Digest, Sha256};

const MAX_RECENT_EVENTS: usize = 64;

#[derive(Debug, Clone)]
pub struct PlatformRuntimeServices {
    pub policy_service: PolicyService,
    pub approval_service: ApprovalService,
    pub evidence_service: EvidenceService,
    pub engineering_service: EngineeringService,
    pub security_service: SecurityService,
    pub audit_service: EvidenceService,
}

impl Default for PlatformRuntimeServices {
    fn default() -> Self {
        Self {
            policy_service: PolicyService::institutional_default(),
            approval_service: ApprovalService::default(),
            evidence_service: EvidenceService::default(),
            engineering_service: EngineeringService::default(),
            security_service: SecurityService,
            audit_service: EvidenceService::default(),
        }
    }
}

struct PlatformGatewayState {
    manifest: InstitutionalPlatformClientV1,
    release_apps: Vec<ReleasedUiAppV1>,
    connected_cache: bool,
    recent_events: Vec<EventEnvelopeV1>,
    subscribers: Vec<mpsc::UnboundedSender<EventEnvelopeV1>>,
    services: PlatformRuntimeServices,
}

#[derive(Clone)]
pub struct PlatformGateway {
    state: Arc<Mutex<PlatformGatewayState>>,
}

impl PlatformGateway {
    #[must_use]
    pub fn new(
        manifest: InstitutionalPlatformClientV1,
        release_apps: Vec<ReleasedUiAppV1>,
        connected_cache: bool,
        services: PlatformRuntimeServices,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(PlatformGatewayState {
                manifest,
                release_apps,
                connected_cache,
                recent_events: Vec::new(),
                subscribers: Vec::new(),
                services,
            })),
        }
    }

    #[must_use]
    pub fn shell_default(release_apps: Vec<ReleasedUiAppV1>) -> Self {
        let lattice = LatticeConfigV1 {
            lattice_name: "institutional-lattice".to_string(),
            rollout: RolloutTargetV1 {
                environment: "prod".to_string(),
                namespace: "runtime".to_string(),
                policy_group: "institutional-default".to_string(),
            },
            components: vec![
                engineering_component(),
                security_component(),
                release_approval_component(),
            ],
        };
        let manifest = InstitutionalPlatformClientV1 {
            client_name: "short-origin-shell".to_string(),
            supported_services: vec![
                engineering_service::service_boundary(),
                security_service::service_boundary(),
            ],
            supported_workflows: vec![release_approval::workflow_boundary()],
            lattice_config: Some(lattice),
        };
        Self::new(
            manifest,
            release_apps,
            true,
            PlatformRuntimeServices::default(),
        )
    }

    #[must_use]
    pub fn manifest(&self) -> InstitutionalPlatformClientV1 {
        self.state
            .lock()
            .expect("platform gateway lock")
            .manifest
            .clone()
    }

    #[must_use]
    pub fn dashboard_snapshot(&self) -> UiDashboardSnapshotV1 {
        let state = self.state.lock().expect("platform gateway lock");
        state
            .manifest
            .dashboard_snapshot(state.release_apps.clone(), state.connected_cache)
    }

    pub fn seed_approval(&self, decision: ApprovalDecisionV1) {
        self.state
            .lock()
            .expect("platform gateway lock")
            .services
            .approval_service
            .record_decision(decision);
    }

    #[must_use]
    pub fn release_approvals(&self) -> Vec<ReleaseApprovalRecordV1> {
        self.state
            .lock()
            .expect("platform gateway lock")
            .services
            .engineering_service
            .release_approvals()
            .to_vec()
    }

    #[must_use]
    pub fn audit_events(&self) -> Vec<AuditEvent> {
        self.state
            .lock()
            .expect("platform gateway lock")
            .services
            .audit_service
            .audit_events()
            .to_vec()
    }

    fn emit_event(
        state: &mut PlatformGatewayState,
        event_type: &str,
        actor_ref: identity::ActorRef,
        correlation_id: &str,
        decision_ref: Option<String>,
        classification: Classification,
        schema_ref: &str,
        payload: serde_json::Value,
    ) {
        let envelope = EventEnvelopeV1::new(
            event_type,
            actor_ref,
            correlation_id.to_string(),
            decision_ref,
            classification,
            schema_ref.to_string(),
            hash_payload(&payload),
        );
        state.recent_events.push(envelope.clone());
        if state.recent_events.len() > MAX_RECENT_EVENTS {
            let overflow = state.recent_events.len() - MAX_RECENT_EVENTS;
            state.recent_events.drain(0..overflow);
        }
        state
            .subscribers
            .retain(|sender| sender.unbounded_send(envelope.clone()).is_ok());
    }

    fn execute_release_approval(
        state: &mut PlatformGatewayState,
        action: AgentActionRequestV1,
        request: ReleaseApprovalRequestV1,
    ) -> InstitutionalResult<PlatformCommandResultV1> {
        Self::emit_event(
            state,
            "release_approval_requested",
            action.actor_ref.clone(),
            &action.action_id,
            None,
            action.classification,
            "schemas/events/platform/v1/release-approval-requested-v1.json",
            serde_json::to_value(ReleaseApprovalRequestedV1 {
                action_id: action.action_id.clone(),
                request: request.clone(),
            })
            .expect("serialize requested event"),
        );

        let mut engine = WorkflowEngine::new(
            std::mem::take(&mut state.services.policy_service),
            std::mem::take(&mut state.services.approval_service),
            std::mem::take(&mut state.services.evidence_service),
        );
        let result = release_approval::execute(
            &mut engine,
            &mut state.services.engineering_service,
            &state.services.security_service,
            &mut state.services.audit_service,
            &action,
            request.clone(),
        );
        let (policy_service, approval_service, evidence_service) = engine.into_parts();
        state.services.policy_service = policy_service;
        state.services.approval_service = approval_service;
        state.services.evidence_service = evidence_service;

        match result {
            Ok(record) => {
                Self::emit_event(
                    state,
                    "release_approval_approved",
                    action.actor_ref,
                    &action.action_id,
                    Some(format!("release-approval/{}", record.release_id)),
                    action.classification,
                    "schemas/events/platform/v1/release-approval-approved-v1.json",
                    serde_json::to_value(ReleaseApprovalApprovedV1 {
                        action_id: action.action_id.clone(),
                        record: record.clone(),
                    })
                    .expect("serialize approved event"),
                );
                Ok(PlatformCommandResultV1::ReleaseApproval(record))
            }
            Err(error) => {
                Self::emit_event(
                    state,
                    "release_approval_denied",
                    action.actor_ref,
                    &action.action_id,
                    None,
                    action.classification,
                    "schemas/events/platform/v1/release-approval-denied-v1.json",
                    serde_json::to_value(ReleaseApprovalDeniedV1 {
                        action_id: action.action_id.clone(),
                        release_id: request.release_id,
                        reason: error.to_string(),
                    })
                    .expect("serialize denied event"),
                );
                Err(error)
            }
        }
    }
}

impl InstitutionalPlatformTransport for PlatformGateway {
    fn execute_command(
        &self,
        command: PlatformCommandV1,
    ) -> TransportFuture<PlatformCommandResultV1> {
        let result = {
            let mut state = self.state.lock().expect("platform gateway lock");
            match command {
                PlatformCommandV1::SubmitReleaseApproval { action, request } => {
                    Self::execute_release_approval(&mut state, action, request)
                }
                PlatformCommandV1::DispatchAgentAction(_)
                | PlatformCommandV1::PreparePromotion { .. }
                | PlatformCommandV1::RegisterMarketDataBatch(_)
                | PlatformCommandV1::SubmitPromotionRecommendation(_) => {
                    Ok(PlatformCommandResultV1::Ack(sdk_rs::PlatformCommandAckV1 {
                        command_id: "platform-gateway-ack".to_string(),
                        accepted: true,
                    }))
                }
            }
        };
        Box::pin(future::ready(result))
    }

    fn execute_query(&self, query: PlatformQueryV1) -> TransportFuture<PlatformQueryResultV1> {
        let result = {
            let state = self.state.lock().expect("platform gateway lock");
            match query {
                PlatformQueryV1::Dashboard => PlatformQueryResultV1::Dashboard(
                    state
                        .manifest
                        .dashboard_snapshot(state.release_apps.clone(), state.connected_cache),
                ),
                PlatformQueryV1::SupportedWorkflows => PlatformQueryResultV1::SupportedWorkflows(
                    state.manifest.supported_workflows.clone(),
                ),
                PlatformQueryV1::RecentEvents { limit } => PlatformQueryResultV1::RecentEvents(
                    state
                        .recent_events
                        .iter()
                        .rev()
                        .take(limit)
                        .cloned()
                        .collect(),
                ),
            }
        };
        Box::pin(future::ready(Ok(result)))
    }

    fn subscribe_events(&self) -> EventSubscription {
        let (sender, receiver) = mpsc::unbounded();
        self.state
            .lock()
            .expect("platform gateway lock")
            .subscribers
            .push(sender);
        Box::pin(receiver.map(|event| event))
    }
}

fn hash_payload(payload: &serde_json::Value) -> String {
    let mut digest = Sha256::new();
    digest.update(payload.to_string().as_bytes());
    hex::encode(digest.finalize())
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use contracts::{
        AgentActionRequestV1, ApprovalDecisionV1, Classification, ImpactTier,
        ReleaseApprovalRequestV1,
    };
    use futures::StreamExt;
    use identity::{ActorRef, InstitutionalRole};
    use sdk_rs::{InstitutionalPlatformRuntimeClient, ReleasedUiAppV1};

    use super::PlatformGateway;

    fn release_apps() -> Vec<ReleasedUiAppV1> {
        vec![ReleasedUiAppV1 {
            app_id: "system.terminal".to_string(),
            display_name: "Terminal".to_string(),
            desktop_enabled: true,
        }]
    }

    fn action(action_id: &str) -> AgentActionRequestV1 {
        AgentActionRequestV1 {
            action_id: action_id.to_string(),
            actor_ref: ActorRef("human.release_operator".to_string()),
            objective: "Approve governed production release".to_string(),
            requested_workflow: "release_approval".to_string(),
            impact_tier: ImpactTier::Tier3,
            classification: Classification::Restricted,
            required_approver_roles: vec![InstitutionalRole::Cto, InstitutionalRole::Ciso],
            policy_refs: vec!["engineering.release.approval.v1".to_string()],
        }
    }

    fn request() -> ReleaseApprovalRequestV1 {
        ReleaseApprovalRequestV1 {
            release_id: "release-2026-03-06.1".to_string(),
            build_ref: "ghcr.io/shortorigin/release@sha256:test".to_string(),
            artifact_digest: "sha256:test".to_string(),
            test_evidence_ref: "evidence/tests/release-2026-03-06.1".to_string(),
            environment: "prod".to_string(),
        }
    }

    #[tokio::test]
    async fn gateway_dispatches_release_approval_and_streams_events() {
        let gateway = PlatformGateway::shell_default(release_apps());
        gateway.seed_approval(ApprovalDecisionV1 {
            action_id: "action::release::gateway".to_string(),
            approver: ActorRef("human.cto".to_string()),
            approver_role: InstitutionalRole::Cto,
            approved: true,
            rationale: "Engineering signoff".to_string(),
            decided_at: Utc::now(),
        });
        gateway.seed_approval(ApprovalDecisionV1 {
            action_id: "action::release::gateway".to_string(),
            approver: ActorRef("human.ciso".to_string()),
            approver_role: InstitutionalRole::Ciso,
            approved: true,
            rationale: "Security signoff".to_string(),
            decided_at: Utc::now(),
        });
        let manifest = gateway.manifest();
        let client = InstitutionalPlatformRuntimeClient::new(manifest, gateway.clone());
        let mut subscription = client.subscribe_events();

        let record = client
            .submit_release_approval(action("action::release::gateway"), request())
            .await
            .expect("release approval");

        assert_eq!(record.release_id, "release-2026-03-06.1");
        assert_eq!(gateway.release_approvals().len(), 1);
        assert_eq!(gateway.audit_events().len(), 2);

        let streamed = [
            subscription.next().await.expect("requested event"),
            subscription.next().await.expect("approved event"),
        ];
        assert_eq!(
            streamed
                .iter()
                .map(|event| event.event_type.as_str())
                .collect::<Vec<_>>(),
            vec!["release_approval_requested", "release_approval_approved"]
        );

        let recent = client.query_recent_events(8).await.expect("recent events");
        assert_eq!(recent.len(), 2);
    }
}
