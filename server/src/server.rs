use actix_web::{web, App as HttpApp, HttpServer};
use actix_web::middleware::Logger;
use app::App;

use crate::route::sources_scope;

pub async fn run() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();


    let app = web::Data::new(App::new("mongodb://localhost/proxy-checker").await.unwrap());
    HttpServer::new(move || {
        println!("started");
        HttpApp::new()
            .wrap(Logger::default())
            .app_data(app.clone())
            .service(sources_scope())
            
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}