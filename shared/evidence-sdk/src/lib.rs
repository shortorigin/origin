use std::sync::{Arc, Mutex};

use contracts::EvidenceManifestV1;
use error_model::{InstitutionalError, InstitutionalResult, OperationContext, SourceErrorInfo};
use futures::future::BoxFuture;

pub trait EvidenceSink {
    fn record(&self, manifest: EvidenceManifestV1) -> BoxFuture<'_, InstitutionalResult<()>>;
    fn recorded(&self) -> BoxFuture<'_, InstitutionalResult<Vec<EvidenceManifestV1>>>;
}

#[derive(Debug, Default, Clone)]
pub struct MemoryEvidenceSink {
    manifests: Arc<Mutex<Vec<EvidenceManifestV1>>>,
}

impl EvidenceSink for MemoryEvidenceSink {
    fn record(&self, manifest: EvidenceManifestV1) -> BoxFuture<'_, InstitutionalResult<()>> {
        Box::pin(async move {
            self.manifests
                .lock()
                .map_err(|error| {
                    InstitutionalError::persistence(
                        OperationContext::new("shared/evidence-sdk", "record"),
                        "failed to acquire evidence sink lock",
                        SourceErrorInfo::new("std::sync::Mutex", None, error.to_string()),
                    )
                })?
                .push(manifest);
            Ok(())
        })
    }

    fn recorded(&self) -> BoxFuture<'_, InstitutionalResult<Vec<EvidenceManifestV1>>> {
        Box::pin(async move {
            Ok(self
                .manifests
                .lock()
                .map_err(|error| {
                    InstitutionalError::persistence(
                        OperationContext::new("shared/evidence-sdk", "recorded"),
                        "failed to acquire evidence sink lock",
                        SourceErrorInfo::new("std::sync::Mutex", None, error.to_string()),
                    )
                })?
                .clone())
        })
    }
}

impl MemoryEvidenceSink {
    pub fn len(&self) -> InstitutionalResult<usize> {
        Ok(self
            .manifests
            .lock()
            .map_err(|error| {
                InstitutionalError::persistence(
                    OperationContext::new("shared/evidence-sdk", "len"),
                    "failed to acquire evidence sink lock",
                    SourceErrorInfo::new("std::sync::Mutex", None, error.to_string()),
                )
            })?
            .len())
    }

    pub fn is_empty(&self) -> InstitutionalResult<bool> {
        Ok(self.len()? == 0)
    }
}
