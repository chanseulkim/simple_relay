
mod relay;

#[tokio::main]
async fn main() {
    relay::run().await;
}
