#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async {
        clients::run().await.unwrap();
    });
    views::run();
    Ok(())
}
