#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();

    crawler::start_crawl(".data").await;
}
