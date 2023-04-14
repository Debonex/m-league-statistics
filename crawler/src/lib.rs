use futures::{stream, StreamExt};
use log::info;
use regex::Regex;
use reqwest::{
    header::{HeaderMap, ORIGIN, REFERER},
    Client, ClientBuilder,
};
use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
    sync::{Arc, Mutex},
    time::Instant,
};

mod password;

const GAME_DATA_REG_STR: &str = r"UMP_PLAYER.init\(true, true, '(.+)', autoplay\);";
const SEASON_LIST: [&str; 5] = [
    "games/2018-season",
    "games/2019-season",
    "games/2020-season",
    "games/2021-season",
    // current season
    "games",
];

pub async fn start_crawl(data_dir: &str) {
    let instant = Instant::now();
    let headers = HeaderMap::from_iter(vec![
        (ORIGIN, "https://m-league.jp".try_into().unwrap()),
        (REFERER, "https://m-league.jp/".try_into().unwrap()),
    ]);
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();
    let client = Arc::new(client);
    let progress = Arc::new(Mutex::new(Progress { count: 0, total: 0 }));
    // create directory if not exists
    create_dir_all(data_dir).unwrap();
    // tasks for get season info
    let tasks = SEASON_LIST.into_iter().map(|season_url| {
        let client = client.clone();
        let progress = progress.clone();
        async move {
            let instant = Instant::now();
            info!("Start crawling {}", season_url);
            let html = client
                .get(format!("https://m-league.jp/{}", season_url))
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            let html = scraper::Html::parse_document(&html);
            let selector = scraper::Selector::parse(".js-viewer-form").unwrap();
            let el_list = html.select(&selector);
            let game_id_list: Vec<&str> = el_list
                .filter_map(|el| el.value().attr("data-game-id"))
                .collect();
            {
                let mut progress = progress.lock().unwrap();
                progress.total += game_id_list.len() as u32;
            }
            crawl_games(data_dir, &game_id_list, client.clone(), progress.clone()).await;
            let dur = instant.elapsed().as_secs_f64();
            info!("{} finished in {} s.", season_url, dur);
        }
    });

    stream::iter(tasks)
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await;

    let dur = instant.elapsed().as_secs_f64();
    info!("Crawling finished in {} s.", dur);
}

async fn crawl_games(
    data_dir: &str,
    game_id_list: &[&str],
    client: Arc<Client>,
    progress: Arc<Mutex<Progress>>,
) {
    let password = password::gen_password();
    let form = Arc::new([("password", &password)]);
    let regex = Arc::new(Regex::new(GAME_DATA_REG_STR).unwrap());

    let tasks = game_id_list.iter().map(|game_id| {
        let form = form.clone();
        let regex = regex.clone();
        let client = client.clone();
        let progress = progress.clone();
        async move {
            let instant = Instant::now();
            let path = Path::new(data_dir).join(game_id).with_extension("json");
            if !path.exists() {
                let html = client
                    .post(format!(
                        "https://viewer.ml-log.jp/web/viewer?gameid={}",
                        game_id
                    ))
                    .form(&*form)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();

                regex
                    .captures(&html)
                    .and_then(|captures| captures.get(1))
                    .and_then(|game| {
                        File::create(path)
                            .and_then(|mut file| file.write_all(game.as_str().as_bytes()))
                            .ok()
                    });

                let mut progress = progress.lock().unwrap();
                progress.count += 1;
                let dur = instant.elapsed().as_millis();
                info!(
                    "{} finished in {} ms. ({}/{})",
                    game_id, dur, progress.count, progress.total
                );
            } else {
                let mut count = progress.lock().unwrap();
                count.count += 1;
            }
        }
    });

    stream::iter(tasks)
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await;
}

struct Progress {
    pub count: u32,
    pub total: u32,
}
