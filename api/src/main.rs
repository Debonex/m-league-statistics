#[tokio::main]
async fn main() {
    crawler::start_craw(".data").await;
}
