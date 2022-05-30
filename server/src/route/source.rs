use actix_web::web::{scope, Json};
use actix_web::{web::Data, Scope};
use actix_web::{get, post};
use app::{doc, App, ProxyProtocol, db};
use serde::Deserialize;

use crate::util::{json_result, JsonResult};


#[derive(Debug, Deserialize)]
struct CreateSourceParams {
    id: String,
    url: String,
    protocol: ProxyProtocol,
    username: String,
    password: String,
    port: u16
}

#[get("")]
async fn get_sources(app: Data<App>) -> JsonResult {
    json_result(app.db.source.find(doc! {}, "_id", 0).await?)
}

#[post("")]
async fn create_source(app: Data<App>, params: Json<CreateSourceParams>) -> JsonResult {
    dbg!(params);
    json_result(true)
    // json_result(app.db.source.insert(doc).await?)
}

pub fn sources_scope() -> Scope {
    scope("/sources").service(get_sources)
}