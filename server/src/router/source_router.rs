use actix_web::web::{Json, Path, self};
use actix_web::{get, post};
use actix_web::{web::Data, Scope};
use app::{doc, App, ProxyProtocol, Source};
use chrono::Utc;
use serde::Deserialize;

use crate::util::{json_result, JsonResult};

#[derive(Debug, Deserialize)]
struct CreateSourceParams {
    id: String,
    url: String,
    protocol: ProxyProtocol,
    username: String,
    password: String,
    port: u16,
}

impl From<CreateSourceParams> for Source {
    fn from(params: CreateSourceParams) -> Self {
        Self {
            id: params.id,
            url: params.url,
            protocol: params.protocol,
            username: params.username,
            password: params.password,
            port: params.port,
            created_at: Utc::now(),
            checked_at: None,
        }
    }
}

#[get("")]
async fn get_sources(app: Data<App>) -> JsonResult {
    json_result(app.db.source.find(doc! {}, "_id", 0).await?)
}

#[post("")]
async fn create_source(app: Data<App>, params: Json<CreateSourceParams>) -> JsonResult {
    json_result(app.db.source.insert(&params.into_inner().into()).await?)
}

#[post("/{id}/check")]
async fn check_source(app: Data<App>, id: Path<String>) -> JsonResult {
    json_result(app.source_service.check(id.to_string()).await?)
}

pub fn scope() -> Scope {
    web::scope("/sources").service(get_sources).service(create_source).service(check_source)
}
