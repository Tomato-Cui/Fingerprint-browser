#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    clients::run().await?;
    Ok(())
}
