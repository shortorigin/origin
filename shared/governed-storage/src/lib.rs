use contracts::{
    EvidenceManifestV1, KnowledgeCapsuleV1, KnowledgeEdgeV1, KnowledgePublicationStatusV1,
    KnowledgeSourceV1, MacroFinancialAnalysisV1,
};
use error_model::InstitutionalResult;
use events::RecordedEventV1;
use futures::future::BoxFuture;

pub trait KnowledgeStore: Send + Sync {
    fn store_analysis(
        &self,
        analysis: MacroFinancialAnalysisV1,
    ) -> BoxFuture<'_, InstitutionalResult<()>>;
    fn load_analysis(
        &self,
        analysis_id: &str,
    ) -> BoxFuture<'_, InstitutionalResult<Option<MacroFinancialAnalysisV1>>>;
    fn store_evidence(
        &self,
        id: String,
        manifest: EvidenceManifestV1,
    ) -> BoxFuture<'_, InstitutionalResult<()>>;
    fn append_event(
        &self,
        id: String,
        event: RecordedEventV1,
    ) -> BoxFuture<'_, InstitutionalResult<()>>;
    fn store_source(&self, source: KnowledgeSourceV1) -> BoxFuture<'_, InstitutionalResult<()>>;
    fn load_sources(
        &self,
        ids: &[String],
    ) -> BoxFuture<'_, InstitutionalResult<Vec<KnowledgeSourceV1>>>;
    fn store_capsule(&self, capsule: KnowledgeCapsuleV1) -> BoxFuture<'_, InstitutionalResult<()>>;
    fn load_capsule(
        &self,
        capsule_id: &str,
    ) -> BoxFuture<'_, InstitutionalResult<Option<KnowledgeCapsuleV1>>>;
    fn latest_publication_status(
        &self,
    ) -> BoxFuture<'_, InstitutionalResult<Option<KnowledgePublicationStatusV1>>>;
    fn store_edge(&self, edge: KnowledgeEdgeV1) -> BoxFuture<'_, InstitutionalResult<()>>;
}

#[derive(Clone)]
pub struct GovernedKnowledgeStore<B> {
    inner: B,
}

impl<B> GovernedKnowledgeStore<B> {
    #[must_use]
    pub fn new(inner: B) -> Self {
        Self { inner }
    }
}

pub type InMemoryKnowledgeStore =
    GovernedKnowledgeStore<storage_backend::InMemoryKnowledgeStoreBackend>;

pub async fn connect_in_memory() -> InstitutionalResult<InMemoryKnowledgeStore> {
    Ok(GovernedKnowledgeStore::new(
        storage_backend::connect_in_memory().await?,
    ))
}

impl<C> KnowledgeStore for GovernedKnowledgeStore<storage_backend::KnowledgeStoreBackend<C>>
where
    C: storage_backend::BackendConnection + Send + Sync,
{
    fn store_analysis(
        &self,
        analysis: MacroFinancialAnalysisV1,
    ) -> BoxFuture<'_, InstitutionalResult<()>> {
        Box::pin(async move {
            self.inner.knowledge_analyses().store(analysis).await?;
            Ok(())
        })
    }

    fn load_analysis(
        &self,
        analysis_id: &str,
    ) -> BoxFuture<'_, InstitutionalResult<Option<MacroFinancialAnalysisV1>>> {
        let analysis_id = analysis_id.to_owned();
        Box::pin(async move {
            Ok(self
                .inner
                .knowledge_analyses()
                .load(&analysis_id)
                .await?
                .map(|record| record.analysis))
        })
    }

    fn store_evidence(
        &self,
        id: String,
        manifest: EvidenceManifestV1,
    ) -> BoxFuture<'_, InstitutionalResult<()>> {
        Box::pin(async move {
            self.inner.evidence_manifests().store(id, manifest).await?;
            Ok(())
        })
    }

    fn append_event(
        &self,
        id: String,
        event: RecordedEventV1,
    ) -> BoxFuture<'_, InstitutionalResult<()>> {
        Box::pin(async move {
            self.inner.recorded_events().append(id, event).await?;
            Ok(())
        })
    }

    fn store_source(&self, source: KnowledgeSourceV1) -> BoxFuture<'_, InstitutionalResult<()>> {
        Box::pin(async move {
            self.inner.knowledge_sources().store(source).await?;
            Ok(())
        })
    }

    fn load_sources(
        &self,
        ids: &[String],
    ) -> BoxFuture<'_, InstitutionalResult<Vec<KnowledgeSourceV1>>> {
        let ids = ids.to_vec();
        Box::pin(async move {
            Ok(self
                .inner
                .knowledge_sources()
                .load_many(&ids)
                .await?
                .into_iter()
                .map(|record| record.source)
                .collect())
        })
    }

    fn store_capsule(&self, capsule: KnowledgeCapsuleV1) -> BoxFuture<'_, InstitutionalResult<()>> {
        Box::pin(async move {
            self.inner.knowledge_capsules().store(capsule).await?;
            Ok(())
        })
    }

    fn load_capsule(
        &self,
        capsule_id: &str,
    ) -> BoxFuture<'_, InstitutionalResult<Option<KnowledgeCapsuleV1>>> {
        let capsule_id = capsule_id.to_owned();
        Box::pin(async move {
            Ok(self
                .inner
                .knowledge_capsules()
                .load(&capsule_id)
                .await?
                .map(|record| record.capsule))
        })
    }

    fn latest_publication_status(
        &self,
    ) -> BoxFuture<'_, InstitutionalResult<Option<KnowledgePublicationStatusV1>>> {
        Box::pin(async move { self.inner.knowledge_capsules().latest_status().await })
    }

    fn store_edge(&self, edge: KnowledgeEdgeV1) -> BoxFuture<'_, InstitutionalResult<()>> {
        Box::pin(async move {
            self.inner.knowledge_edges().store(edge).await?;
            Ok(())
        })
    }
}
