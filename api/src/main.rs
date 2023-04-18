use std::{fs::File, io::Read};

use crawler::UMDGameItem;

mod game;

#[tokio::main]
async fn main() {
    // crawler::start_crawl(".data").await;
    let files = ["L001_S001_0001_01A.json"];

    files.iter().for_each(|file| {
        let mut content = String::new();
        File::open(format!("{}/{}", ".data", file))
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();
        let items: Vec<UMDGameItem> = serde_json::from_str(content.as_str()).unwrap();

        let game = game::evaluate(&items, file).unwrap();

        println!("{:?}", game);
    });
}
