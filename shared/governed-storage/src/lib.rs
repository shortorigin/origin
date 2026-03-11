use contracts::{
    EvidenceManifestV1, KnowledgeCapsuleV1, KnowledgeChangeNotificationV1, KnowledgeEdgeV1,
    KnowledgePublicationStatusV1, KnowledgeRetrievalHitV1, KnowledgeRetrievalQueryV1,
    KnowledgeSourceV1, MacroFinancialAnalysisV1,
};
use error_model::InstitutionalResult;
use futures::{future::BoxFuture, stream::BoxStream, StreamExt};
use surrealdb_model::{EventRecordV1, KnowledgeChunkRecordV1};

pub type KnowledgeChangeStream =
    BoxStream<'static, InstitutionalResult<KnowledgeChangeNotificationV1>>;

pub trait KnowledgeStore: Send + Sync {
    fn store_sources_batch(
        &self,
        sources: Vec<KnowledgeSourceV1>,
        events: Vec<EventRecordV1>,
        notifications: Vec<KnowledgeChangeNotificationV1>,
    ) -> BoxFuture<'_, InstitutionalResult<()>>;
    fn store_publication_bundle(
        &self,
        capsule: KnowledgeCapsuleV1,
        chunks: Vec<KnowledgeChunkRecordV1>,
        events: Vec<EventRecordV1>,
        edges: Vec<KnowledgeEdgeV1>,
        notifications: Vec<KnowledgeChangeNotificationV1>,
    ) -> BoxFuture<'_, InstitutionalResult<()>>;
    fn store_analysis_bundle(
        &self,
        analysis: MacroFinancialAnalysisV1,
        evidence_id: String,
        manifest: EvidenceManifestV1,
        events: Vec<EventRecordV1>,
        edges: Vec<KnowledgeEdgeV1>,
        notifications: Vec<KnowledgeChangeNotificationV1>,
    ) -> BoxFuture<'_, InstitutionalResult<()>>;
    fn load_analysis(
        &self,
        analysis_id: &str,
    ) -> BoxFuture<'_, InstitutionalResult<Option<MacroFinancialAnalysisV1>>>;
    fn load_sources(
        &self,
        ids: &[String],
    ) -> BoxFuture<'_, InstitutionalResult<Vec<KnowledgeSourceV1>>>;
    fn load_capsule(
        &self,
        capsule_id: &str,
    ) -> BoxFuture<'_, InstitutionalResult<Option<KnowledgeCapsuleV1>>>;
    fn latest_publication_status(
        &self,
    ) -> BoxFuture<'_, InstitutionalResult<Option<KnowledgePublicationStatusV1>>>;
    fn search_capsule(
        &self,
        query: KnowledgeRetrievalQueryV1,
    ) -> BoxFuture<'_, InstitutionalResult<Vec<KnowledgeRetrievalHitV1>>>;
    fn load_change_notifications(
        &self,
        limit: usize,
    ) -> BoxFuture<'_, InstitutionalResult<Vec<KnowledgeChangeNotificationV1>>>;
    fn subscribe_change_notifications(
        &self,
    ) -> BoxFuture<'_, InstitutionalResult<KnowledgeChangeStream>>;
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
pub type DurableKnowledgeStore =
    GovernedKnowledgeStore<storage_backend::DurableKnowledgeStoreBackend>;

pub async fn connect_in_memory() -> InstitutionalResult<InMemoryKnowledgeStore> {
    Ok(GovernedKnowledgeStore::new(
        storage_backend::connect_in_memory().await?,
    ))
}

pub async fn connect_durable_from_env() -> InstitutionalResult<DurableKnowledgeStore> {
    Ok(GovernedKnowledgeStore::new(
        storage_backend::connect_durable_from_env().await?,
    ))
}

impl<C> KnowledgeStore for GovernedKnowledgeStore<storage_backend::KnowledgeStoreBackend<C>>
where
    C: storage_backend::BackendConnection + Send + Sync,
{
    fn store_sources_batch(
        &self,
        sources: Vec<KnowledgeSourceV1>,
        events: Vec<EventRecordV1>,
        notifications: Vec<KnowledgeChangeNotificationV1>,
    ) -> BoxFuture<'_, InstitutionalResult<()>> {
        Box::pin(async move {
            self.inner
                .store_sources_batch(sources, events, notifications)
                .await
        })
    }

    fn store_publication_bundle(
        &self,
        capsule: KnowledgeCapsuleV1,
        chunks: Vec<KnowledgeChunkRecordV1>,
        events: Vec<EventRecordV1>,
        edges: Vec<KnowledgeEdgeV1>,
        notifications: Vec<KnowledgeChangeNotificationV1>,
    ) -> BoxFuture<'_, InstitutionalResult<()>> {
        Box::pin(async move {
            self.inner
                .store_publication_bundle(capsule, chunks, events, edges, notifications)
                .await
        })
    }

    fn store_analysis_bundle(
        &self,
        analysis: MacroFinancialAnalysisV1,
        evidence_id: String,
        manifest: EvidenceManifestV1,
        events: Vec<EventRecordV1>,
        edges: Vec<KnowledgeEdgeV1>,
        notifications: Vec<KnowledgeChangeNotificationV1>,
    ) -> BoxFuture<'_, InstitutionalResult<()>> {
        Box::pin(async move {
            self.inner
                .store_analysis_bundle(
                    analysis,
                    evidence_id,
                    manifest,
                    events,
                    edges,
                    notifications,
                )
                .await
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
        Box::pin(async move { self.inner.publication_status().latest().await })
    }

    fn search_capsule(
        &self,
        query: KnowledgeRetrievalQueryV1,
    ) -> BoxFuture<'_, InstitutionalResult<Vec<KnowledgeRetrievalHitV1>>> {
        Box::pin(async move { self.inner.knowledge_chunks().search(query).await })
    }

    fn load_change_notifications(
        &self,
        limit: usize,
    ) -> BoxFuture<'_, InstitutionalResult<Vec<KnowledgeChangeNotificationV1>>> {
        Box::pin(async move {
            Ok(self
                .inner
                .change_notifications()
                .recent(limit)
                .await?
                .into_iter()
                .map(|record| record.as_notification())
                .collect())
        })
    }

    fn subscribe_change_notifications(
        &self,
    ) -> BoxFuture<'_, InstitutionalResult<KnowledgeChangeStream>> {
        Box::pin(async move {
            let stream = self.inner.change_notifications().subscribe().await?;
            Ok(stream
                .map(|result| result.map(|record| record.as_notification()))
                .boxed())
        })
    }
}
