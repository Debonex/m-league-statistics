#[tokio::main]
async fn main() {
    crawler::start_crawl(".data").await;
}
