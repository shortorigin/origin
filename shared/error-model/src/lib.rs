use identity::{ServiceId, WorkflowId};
use serde::{Deserialize, Serialize};
use telemetry::CorrelationId;
use thiserror::Error;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstitutionalErrorCategory {
    Configuration,
    Transport,
    DependencyUnavailable,
    Parse,
    Validation,
    PolicyDenied,
    ApprovalDenied,
    DomainStateViolation,
    Persistence,
    Timeout,
    Cancelled,
    InvariantViolation,
    NotFound,
    IdentityViolation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperationContext {
    pub subsystem: String,
    pub service_id: Option<ServiceId>,
    pub workflow_id: Option<WorkflowId>,
    pub correlation_id: Option<CorrelationId>,
    pub operation: String,
}

impl OperationContext {
    #[must_use]
    pub fn new(subsystem: impl Into<String>, operation: impl Into<String>) -> Self {
        Self {
            subsystem: subsystem.into(),
            service_id: None,
            workflow_id: None,
            correlation_id: None,
            operation: operation.into(),
        }
    }

    #[must_use]
    pub fn with_service_id(mut self, service_id: impl Into<ServiceId>) -> Self {
        self.service_id = Some(service_id.into());
        self
    }

    #[must_use]
    pub fn with_workflow_id(mut self, workflow_id: impl Into<WorkflowId>) -> Self {
        self.workflow_id = Some(workflow_id.into());
        self
    }

    #[must_use]
    pub fn with_correlation_id(mut self, correlation_id: impl Into<CorrelationId>) -> Self {
        self.correlation_id = Some(correlation_id.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceErrorInfo {
    pub origin: String,
    pub operation: Option<String>,
    pub details: String,
}

impl SourceErrorInfo {
    #[must_use]
    pub fn new(
        origin: impl Into<String>,
        operation: impl Into<Option<String>>,
        details: impl Into<String>,
    ) -> Self {
        Self {
            origin: origin.into(),
            operation: operation.into(),
            details: details.into(),
        }
    }
}

type BoxedContext = Box<OperationContext>;
type BoxedSourceInfo = Box<SourceErrorInfo>;

#[derive(Debug, Error, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum InstitutionalError {
    #[error(
        "configuration failure in `{}` during `{}`: {message}",
        .context.subsystem,
        .context.operation
    )]
    Configuration {
        context: BoxedContext,
        message: String,
        source_info: Option<BoxedSourceInfo>,
    },
    #[error(
        "transport failure in `{}` during `{}`: {message}",
        .context.subsystem,
        .context.operation
    )]
    Transport {
        context: BoxedContext,
        message: String,
        source_info: Option<BoxedSourceInfo>,
    },
    #[error(
        "dependency unavailable in `{}` during `{}`: {message}",
        .context.subsystem,
        .context.operation
    )]
    DependencyUnavailable {
        context: BoxedContext,
        message: String,
        source_info: Option<BoxedSourceInfo>,
    },
    #[error(
        "parse failure in `{}` during `{}`: {message}",
        .context.subsystem,
        .context.operation
    )]
    Parse {
        context: BoxedContext,
        message: String,
        source_info: Option<BoxedSourceInfo>,
    },
    #[error(
        "validation failure in `{}` during `{}`: {message}",
        .context.subsystem,
        .context.operation
    )]
    Validation {
        context: BoxedContext,
        message: String,
        source_info: Option<BoxedSourceInfo>,
    },
    #[error(
        "policy denial in `{}` during `{}`: {message}",
        .context.subsystem,
        .context.operation
    )]
    PolicyDenied {
        context: BoxedContext,
        message: String,
        source_info: Option<BoxedSourceInfo>,
    },
    #[error(
        "approval denial in `{}` during `{}`: {message}",
        .context.subsystem,
        .context.operation
    )]
    ApprovalDenied {
        context: BoxedContext,
        message: String,
        source_info: Option<BoxedSourceInfo>,
    },
    #[error(
        "domain state violation in `{}` during `{}`: {message}",
        .context.subsystem,
        .context.operation
    )]
    DomainStateViolation {
        context: BoxedContext,
        message: String,
        source_info: Option<BoxedSourceInfo>,
    },
    #[error(
        "persistence failure in `{}` during `{}`: {message}",
        .context.subsystem,
        .context.operation
    )]
    Persistence {
        context: BoxedContext,
        message: String,
        source_info: Option<BoxedSourceInfo>,
    },
    #[error(
        "timeout in `{}` during `{}`: {message}",
        .context.subsystem,
        .context.operation
    )]
    Timeout {
        context: BoxedContext,
        message: String,
        source_info: Option<BoxedSourceInfo>,
    },
    #[error(
        "cancellation in `{}` during `{}`: {message}",
        .context.subsystem,
        .context.operation
    )]
    Cancelled {
        context: BoxedContext,
        message: String,
        source_info: Option<BoxedSourceInfo>,
    },
    #[error(
        "invariant violation in `{}` during `{}`: {message}",
        .context.subsystem,
        .context.operation
    )]
    InvariantViolation {
        context: BoxedContext,
        message: String,
        source_info: Option<BoxedSourceInfo>,
    },
    #[error(
        "not found in `{}` during `{}`: {message}",
        .context.subsystem,
        .context.operation
    )]
    NotFound {
        context: BoxedContext,
        message: String,
        source_info: Option<BoxedSourceInfo>,
    },
    #[error(
        "identity violation in `{}` during `{}`: {message}",
        .context.subsystem,
        .context.operation
    )]
    IdentityViolation {
        context: BoxedContext,
        message: String,
        source_info: Option<BoxedSourceInfo>,
    },
}

pub type InstitutionalResult<T> = Result<T, InstitutionalError>;

impl InstitutionalError {
    #[must_use]
    pub fn category(&self) -> InstitutionalErrorCategory {
        match self {
            Self::Configuration { .. } => InstitutionalErrorCategory::Configuration,
            Self::Transport { .. } => InstitutionalErrorCategory::Transport,
            Self::DependencyUnavailable { .. } => InstitutionalErrorCategory::DependencyUnavailable,
            Self::Parse { .. } => InstitutionalErrorCategory::Parse,
            Self::Validation { .. } => InstitutionalErrorCategory::Validation,
            Self::PolicyDenied { .. } => InstitutionalErrorCategory::PolicyDenied,
            Self::ApprovalDenied { .. } => InstitutionalErrorCategory::ApprovalDenied,
            Self::DomainStateViolation { .. } => InstitutionalErrorCategory::DomainStateViolation,
            Self::Persistence { .. } => InstitutionalErrorCategory::Persistence,
            Self::Timeout { .. } => InstitutionalErrorCategory::Timeout,
            Self::Cancelled { .. } => InstitutionalErrorCategory::Cancelled,
            Self::InvariantViolation { .. } => InstitutionalErrorCategory::InvariantViolation,
            Self::NotFound { .. } => InstitutionalErrorCategory::NotFound,
            Self::IdentityViolation { .. } => InstitutionalErrorCategory::IdentityViolation,
        }
    }

    #[must_use]
    pub fn context(&self) -> &OperationContext {
        match self {
            Self::Configuration { context, .. }
            | Self::Transport { context, .. }
            | Self::DependencyUnavailable { context, .. }
            | Self::Parse { context, .. }
            | Self::Validation { context, .. }
            | Self::PolicyDenied { context, .. }
            | Self::ApprovalDenied { context, .. }
            | Self::DomainStateViolation { context, .. }
            | Self::Persistence { context, .. }
            | Self::Timeout { context, .. }
            | Self::Cancelled { context, .. }
            | Self::InvariantViolation { context, .. }
            | Self::NotFound { context, .. }
            | Self::IdentityViolation { context, .. } => context.as_ref(),
        }
    }

    #[must_use]
    pub fn message(&self) -> &str {
        match self {
            Self::Configuration { message, .. }
            | Self::Transport { message, .. }
            | Self::DependencyUnavailable { message, .. }
            | Self::Parse { message, .. }
            | Self::Validation { message, .. }
            | Self::PolicyDenied { message, .. }
            | Self::ApprovalDenied { message, .. }
            | Self::DomainStateViolation { message, .. }
            | Self::Persistence { message, .. }
            | Self::Timeout { message, .. }
            | Self::Cancelled { message, .. }
            | Self::InvariantViolation { message, .. }
            | Self::NotFound { message, .. }
            | Self::IdentityViolation { message, .. } => message,
        }
    }

    #[must_use]
    pub fn source_info(&self) -> Option<&SourceErrorInfo> {
        match self {
            Self::Configuration { source_info, .. }
            | Self::Transport { source_info, .. }
            | Self::DependencyUnavailable { source_info, .. }
            | Self::Parse { source_info, .. }
            | Self::Validation { source_info, .. }
            | Self::PolicyDenied { source_info, .. }
            | Self::ApprovalDenied { source_info, .. }
            | Self::DomainStateViolation { source_info, .. }
            | Self::Persistence { source_info, .. }
            | Self::Timeout { source_info, .. }
            | Self::Cancelled { source_info, .. }
            | Self::InvariantViolation { source_info, .. }
            | Self::NotFound { source_info, .. }
            | Self::IdentityViolation { source_info, .. } => source_info.as_deref(),
        }
    }

    #[must_use]
    pub fn configuration(context: OperationContext, message: impl Into<String>) -> Self {
        Self::Configuration {
            context: Box::new(context),
            message: message.into(),
            source_info: None,
        }
    }

    #[must_use]
    pub fn transport(context: OperationContext, message: impl Into<String>) -> Self {
        Self::Transport {
            context: Box::new(context),
            message: message.into(),
            source_info: None,
        }
    }

    #[must_use]
    pub fn dependency_unavailable(context: OperationContext, message: impl Into<String>) -> Self {
        Self::DependencyUnavailable {
            context: Box::new(context),
            message: message.into(),
            source_info: None,
        }
    }

    #[must_use]
    pub fn validation(context: OperationContext, message: impl Into<String>) -> Self {
        Self::Validation {
            context: Box::new(context),
            message: message.into(),
            source_info: None,
        }
    }

    #[must_use]
    pub fn policy_denied(context: OperationContext, message: impl Into<String>) -> Self {
        Self::PolicyDenied {
            context: Box::new(context),
            message: message.into(),
            source_info: None,
        }
    }

    #[must_use]
    pub fn approval_denied(context: OperationContext, message: impl Into<String>) -> Self {
        Self::ApprovalDenied {
            context: Box::new(context),
            message: message.into(),
            source_info: None,
        }
    }

    #[must_use]
    pub fn not_found(context: OperationContext, message: impl Into<String>) -> Self {
        Self::NotFound {
            context: Box::new(context),
            message: message.into(),
            source_info: None,
        }
    }

    #[must_use]
    pub fn invariant(context: OperationContext, message: impl Into<String>) -> Self {
        Self::InvariantViolation {
            context: Box::new(context),
            message: message.into(),
            source_info: None,
        }
    }

    #[must_use]
    pub fn identity_violation(context: OperationContext, message: impl Into<String>) -> Self {
        Self::IdentityViolation {
            context: Box::new(context),
            message: message.into(),
            source_info: None,
        }
    }

    #[must_use]
    pub fn persistence(
        context: OperationContext,
        message: impl Into<String>,
        source: SourceErrorInfo,
    ) -> Self {
        Self::Persistence {
            context: Box::new(context),
            message: message.into(),
            source_info: Some(Box::new(source)),
        }
    }

    #[must_use]
    pub fn parse(source_name: impl Into<String>, details: impl Into<String>) -> Self {
        Self::Parse {
            context: Box::new(OperationContext::new("parser", "parse")),
            message: format!("failed to parse `{}`", source_name.into()),
            source_info: Some(Box::new(SourceErrorInfo::new("parse", None, details))),
        }
    }

    #[must_use]
    pub fn parse_with_parser(
        source_name: impl Into<String>,
        parser: impl Into<String>,
        details: impl Into<String>,
    ) -> Self {
        let source_name = source_name.into();
        let parser = parser.into();
        Self::Parse {
            context: Box::new(OperationContext::new("parser", parser.clone())),
            message: format!("failed to parse `{source_name}`"),
            source_info: Some(Box::new(SourceErrorInfo::new(
                source_name,
                Some(parser),
                details,
            ))),
        }
    }

    #[must_use]
    pub fn external(
        system: impl Into<String>,
        operation: impl Into<Option<String>>,
        details: impl Into<String>,
    ) -> Self {
        let system = system.into();
        let operation = operation.into();
        let context = OperationContext::new(
            system.clone(),
            operation.clone().unwrap_or_else(|| "external".to_string()),
        );
        Self::Transport {
            context: Box::new(context),
            message: format!("dependency `{system}` operation failed"),
            source_info: Some(Box::new(SourceErrorInfo::new(system, operation, details))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        InstitutionalError, InstitutionalErrorCategory, OperationContext, SourceErrorInfo,
    };

    #[test]
    fn parse_helper_preserves_category_and_source() {
        let error = InstitutionalError::parse_with_parser("config.toml", "toml", "bad key");
        assert_eq!(error.category(), InstitutionalErrorCategory::Parse);
        assert_eq!(error.context().operation, "toml");
        assert_eq!(
            error.source_info(),
            Some(&SourceErrorInfo::new(
                "config.toml",
                Some("toml".to_string()),
                "bad key"
            ))
        );
    }

    #[test]
    fn persistence_helper_preserves_context_and_source() {
        let error = InstitutionalError::persistence(
            OperationContext::new("shared/surrealdb-access", "store"),
            "failed to store evidence",
            SourceErrorInfo::new("surrealdb", Some("upsert".to_string()), "record missing"),
        );
        assert_eq!(error.category(), InstitutionalErrorCategory::Persistence);
        assert_eq!(error.context().subsystem, "shared/surrealdb-access");
        assert_eq!(error.message(), "failed to store evidence");
        assert_eq!(
            error.source_info(),
            Some(&SourceErrorInfo::new(
                "surrealdb",
                Some("upsert".to_string()),
                "record missing"
            ))
        );
    }
}
