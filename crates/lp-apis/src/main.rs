#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = std::env::args().nth(1);

    lp_apis::run(port).await?;
    Ok(())
}
