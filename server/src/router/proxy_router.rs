

use actix_web::{
    get,
    web::{self, Data, Path},
    Scope, post,
};
use app::{doc, App};

use crate::util::{json_result, JsonResult};

#[get("")]
async fn get_proxies(app: Data<App>) -> JsonResult {
    json_result(app.db.proxy.find(doc! {}, "_id", 0).await?)
}

#[post("/{id}/check")]
async fn check_proxy(app: Data<App>, id: Path<ObjectId>) -> JsonResult {
    json_result(app.source_service.check(id.to_string()).await?)
}


pub fn scope() -> Scope {
    web::scope("/proxies").service(get_proxies)
}
