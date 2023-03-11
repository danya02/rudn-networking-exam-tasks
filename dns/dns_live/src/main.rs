mod querying;
mod session;
mod web;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    tracing::info!("Started!");

    web::web_main().await;
}
