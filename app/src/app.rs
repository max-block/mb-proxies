use crate::db::Db;
use crate::service::SourceService;
use std::sync::Arc;

pub struct App {
    pub db: Arc<Db>,
    pub source_service: SourceService,
}

impl App {
    pub async fn new(database_url: &str) -> crate::Result<Self> {
        let db = Arc::new(Db::new(database_url).await?);
        let source_service = SourceService::new(db.clone());
        Ok(Self { db, source_service })
    }
}
