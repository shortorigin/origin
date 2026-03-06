use std::collections::BTreeSet;

use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityGrant {
    service: String,
    actions: BTreeSet<String>,
}

impl CapabilityGrant {
    #[must_use]
    pub fn new(
        service: impl Into<String>,
        actions: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            service: service.into(),
            actions: actions.into_iter().map(Into::into).collect(),
        }
    }

    #[must_use]
    pub fn service(&self) -> &str {
        &self.service
    }

    #[must_use]
    pub fn allows(&self, action: &str) -> bool {
        self.actions.contains(action)
    }

    pub fn require(&self, action: &str) -> Result<(), CapabilityError> {
        self.allows(action)
            .then_some(())
            .ok_or_else(|| CapabilityError::Denied {
                service: self.service.clone(),
                action: action.to_string(),
            })
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CapabilityError {
    #[error("capability grant for `{service}` does not allow `{action}`")]
    Denied { service: String, action: String },
}

#[cfg(test)]
mod tests {
    use super::{CapabilityError, CapabilityGrant};

    #[test]
    fn capability_grant_allows_only_configured_actions() {
        let grant = CapabilityGrant::new("security-gateway", ["inspect_packet", "emit_alert"]);

        assert!(grant.allows("inspect_packet"));
        assert_eq!(
            grant.require("reconfigure_listener"),
            Err(CapabilityError::Denied {
                service: "security-gateway".to_string(),
                action: "reconfigure_listener".to_string(),
            })
        );
    }
}
