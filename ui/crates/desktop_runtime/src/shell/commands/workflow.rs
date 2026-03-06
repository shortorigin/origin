#![allow(clippy::clone_on_copy)]

use std::rc::Rc;

use contracts::{AgentActionRequestV1, Classification, ImpactTier, ReleaseApprovalRequestV1};
use desktop_app_contract::AppCommandRegistration;
use identity::{ActorRef, InstitutionalRole};
use platform_gateway::PlatformGateway;
use sdk_rs::InstitutionalPlatformRuntimeClient;
use system_shell_contract::{
    CommandArgSpec, CommandDataShape, CommandOutputShape, CommandResult, ShellError,
    ShellErrorCode, StructuredScalar, StructuredValue,
};

use crate::components::DesktopRuntimeContext;

pub(super) fn registrations(runtime: DesktopRuntimeContext) -> Vec<AppCommandRegistration> {
    vec![release_approval_submit_registration(runtime)]
}

fn release_approval_submit_registration(runtime: DesktopRuntimeContext) -> AppCommandRegistration {
    AppCommandRegistration {
        descriptor: super::super::namespaced_descriptor(
            "workflow release-approval submit",
            &[],
            "Submit a governed release approval request.",
            "workflow release-approval submit <release-id> <build-ref> <artifact-digest> <test-evidence-ref> <environment>",
            vec![
                arg("release-id", "Stable release identifier."),
                arg("build-ref", "Build provenance reference."),
                arg("artifact-digest", "Artifact digest, usually sha256."),
                arg("test-evidence-ref", "Reference to test evidence."),
                arg("environment", "Target rollout environment."),
            ],
            Vec::new(),
            system_shell_contract::CommandInputShape::none(),
            CommandOutputShape::new(CommandDataShape::Record),
        ),
        completion: None,
        handler: Rc::new(move |context| {
            let gateway = runtime.platform_gateway.get_value();
            Box::pin(async move { submit_release_approval_command(gateway, context.args).await })
        }),
    }
}

fn arg(name: &str, summary: &str) -> CommandArgSpec {
    CommandArgSpec {
        name: name.to_string(),
        summary: summary.to_string(),
        required: true,
        repeatable: false,
    }
}

async fn submit_release_approval_command(
    gateway: PlatformGateway,
    args: Vec<String>,
) -> Result<CommandResult, ShellError> {
    if args.len() != 5 {
        return Err(super::super::usage_error(
            "usage: workflow release-approval submit <release-id> <build-ref> <artifact-digest> <test-evidence-ref> <environment>",
        ));
    }

    let action = AgentActionRequestV1 {
        action_id: format!("action::release::{}", args[0]),
        actor_ref: ActorRef("human.release_operator".to_string()),
        objective: format!("Approve governed release `{}`", args[0]),
        requested_workflow: "release_approval".to_string(),
        impact_tier: ImpactTier::Tier3,
        classification: Classification::Restricted,
        required_approver_roles: vec![InstitutionalRole::Cto, InstitutionalRole::Ciso],
        policy_refs: vec!["engineering.release.approval.v1".to_string()],
    };
    let request = ReleaseApprovalRequestV1 {
        release_id: args[0].clone(),
        build_ref: args[1].clone(),
        artifact_digest: args[2].clone(),
        test_evidence_ref: args[3].clone(),
        environment: args[4].clone(),
    };

    let client = InstitutionalPlatformRuntimeClient::new(gateway.manifest(), gateway);
    let record = client
        .submit_release_approval(action.clone(), request.clone())
        .await
        .map_err(map_platform_error)?;
    let recent_events = client
        .query_recent_events(8)
        .await
        .map_err(map_platform_error)?
        .into_iter()
        .filter(|event| event.correlation_id == action.action_id)
        .map(|event| StructuredValue::Scalar(StructuredScalar::String(event.event_type)))
        .collect::<Vec<_>>();

    Ok(CommandResult {
        output: super::super::record_data(vec![
            super::super::string_field("release_id", record.release_id),
            super::super::string_field("environment", record.environment),
            super::super::string_field("release_window_ref", record.release_window_ref),
            super::super::value_field(
                "approved_by_roles",
                StructuredValue::List(
                    record
                        .approved_by_roles
                        .into_iter()
                        .map(|role| {
                            StructuredValue::Scalar(StructuredScalar::String(format!("{role:?}")))
                        })
                        .collect(),
                ),
            ),
            super::super::value_field(
                "evidence_refs",
                StructuredValue::List(
                    record
                        .evidence_refs
                        .into_iter()
                        .map(|reference| {
                            StructuredValue::Scalar(StructuredScalar::String(reference))
                        })
                        .collect(),
                ),
            ),
            super::super::value_field(
                "audit_event_ids",
                StructuredValue::List(
                    record
                        .audit_event_ids
                        .into_iter()
                        .map(|event_id| StructuredValue::Scalar(StructuredScalar::String(event_id)))
                        .collect(),
                ),
            ),
            super::super::value_field("emitted_events", StructuredValue::List(recent_events)),
        ]),
        display: system_shell_contract::DisplayPreference::Record,
        notices: Vec::new(),
        cwd: None,
        exit: system_shell_contract::ShellExit::success(),
    })
}

fn map_platform_error(error: error_model::InstitutionalError) -> ShellError {
    let code = match error {
        error_model::InstitutionalError::ApprovalMissing { .. }
        | error_model::InstitutionalError::PolicyDenied { .. } => ShellErrorCode::PermissionDenied,
        error_model::InstitutionalError::NotFound { .. }
        | error_model::InstitutionalError::InvariantViolation { .. }
        | error_model::InstitutionalError::IdentityViolation { .. }
        | error_model::InstitutionalError::ParseError { .. } => ShellErrorCode::Internal,
    };
    ShellError::new(code, error.to_string())
}

#[cfg(test)]
mod tests {
    use chrono::Utc;
    use contracts::ApprovalDecisionV1;
    use identity::{ActorRef, InstitutionalRole};
    use platform_gateway::PlatformGateway;
    use system_shell_contract::StructuredData;

    use super::submit_release_approval_command;

    #[tokio::test]
    async fn shell_release_approval_command_round_trips_through_gateway() {
        let gateway = PlatformGateway::shell_default(Vec::new());
        gateway.seed_approval(ApprovalDecisionV1 {
            action_id: "action::release::release-2026-03-06.1".to_string(),
            approver: ActorRef("human.cto".to_string()),
            approver_role: InstitutionalRole::Cto,
            approved: true,
            rationale: "Engineering signoff".to_string(),
            decided_at: Utc::now(),
        });
        gateway.seed_approval(ApprovalDecisionV1 {
            action_id: "action::release::release-2026-03-06.1".to_string(),
            approver: ActorRef("human.ciso".to_string()),
            approver_role: InstitutionalRole::Ciso,
            approved: true,
            rationale: "Security signoff".to_string(),
            decided_at: Utc::now(),
        });

        let result = submit_release_approval_command(
            gateway.clone(),
            vec![
                "release-2026-03-06.1".to_string(),
                "ghcr.io/shortorigin/release@sha256:test".to_string(),
                "sha256:test".to_string(),
                "evidence/tests/release-2026-03-06.1".to_string(),
                "prod".to_string(),
            ],
        )
        .await
        .expect("command result");

        assert!(matches!(result.output, StructuredData::Record(_)));
        assert_eq!(gateway.release_approvals().len(), 1);
        assert_eq!(gateway.audit_events().len(), 2);
    }
}
