use app::App;

#[tokio::main]
async fn main() {
    let _app = App::new("mongodb://localhost/proxy-checker").await.unwrap();
}
