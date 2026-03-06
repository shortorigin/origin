use std::collections::BTreeMap;

use contracts::ServiceBoundaryV1;
use error_model::{InstitutionalError, InstitutionalResult};
use identity::{ActorRef, ActorV1, InstitutionalRole};

#[derive(Debug, Default, Clone)]
pub struct IdentityService {
    actors: BTreeMap<String, ActorV1>,
}

impl IdentityService {
    pub fn register_actor(&mut self, actor: ActorV1) {
        self.actors.insert(actor.actor_ref.0.clone(), actor);
    }

    pub fn require_role(
        &self,
        actor_ref: &ActorRef,
        role: InstitutionalRole,
    ) -> InstitutionalResult<()> {
        let actor = self
            .actors
            .get(&actor_ref.0)
            .ok_or_else(|| InstitutionalError::NotFound {
                resource: actor_ref.0.clone(),
            })?;
        if actor.has_role(role) {
            Ok(())
        } else {
            Err(InstitutionalError::IdentityViolation {
                actor: actor_ref.0.clone(),
            })
        }
    }
}

#[must_use]
pub fn service_boundary() -> ServiceBoundaryV1 {
    contracts::service_boundary_named("identity-service")
        .expect("generated identity-service boundary")
}
