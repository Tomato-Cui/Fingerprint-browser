use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry().with(fmt::layer()).init();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5678").await?;
    let routes = server::build_root_router();
    axum::serve(listener, routes).await?;
    Ok(())
}
