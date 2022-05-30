use bson::doc;
use bson::oid::ObjectId;

use app::App;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = App::new("mongodb://localhost/proxy-checker").await.unwrap();
    app.source_service.check("goldproxy_eu".into()).await.unwrap();
    // let res = app.db.proxy.find(doc! {}, "", 0).await.unwrap();
    // dbg!(res);
    // let res = app
    //     .proxy_service
    //     .check(ObjectId::parse_str("628b3ad219612a3add3c87d1").unwrap())
    //     .await;
    // dbg!(res);
    Ok(())
}
