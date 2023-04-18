use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use crate::models::UMDGameItem;

pub fn fix_all(data_dir: &str) {
    let fix = |file_name: &str, f: fn(&mut Vec<UMDGameItem>)| {
        let path = format!("{}/{}", data_dir, file_name);
        let mut content = String::new();
        File::open(&path)
            .unwrap()
            .read_to_string(&mut content)
            .unwrap();
        let mut game: Vec<UMDGameItem> = serde_json::from_str(&content).unwrap();
        f(&mut game);
        let new_content = serde_json::to_string(&game).unwrap();
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap()
            .write_all(new_content.as_bytes())
            .unwrap();
    };

    fix("L001_S007_0010_02A.json", |game| {
        game.last_mut().unwrap().args[3] = "-0.4".to_string()
    });

    fix("L001_S010_0015_02A.json", |game| {
        let item = game.last_mut().unwrap();
        item.args[1] = "38.0".to_string();
        item.args[3] = "38.0".to_string();
    });

    fix("L001_S010_0080_02A.json", |game| {
        let item = game.last_mut().unwrap();
        item.args[1] = "76.4".to_string();
        item.args[7] = "-72.8".to_string();
    });
}
