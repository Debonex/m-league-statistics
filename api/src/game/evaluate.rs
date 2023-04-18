use super::models::{Game, Tile};
use crate::game::models::GamePro;
use crawler::UMDGameItem;

pub fn evaluate(items: &[UMDGameItem], game_id: &str) -> Result<Game, ()> {
    let mut game = Game::new();

    for item in items.iter() {
        let cmd = item.cmd.as_str();
        if cmd == "player" {
            let game_pro = GamePro::new(
                item.args[1].clone(),
                item.args[3].clone(),
                item.args[0].clone(),
            );
            game.game_pros.push(game_pro);
        } else if cmd == "gamestart" {
            game.id = game_id.to_string();
            game.start_time = item.time.clone()
        } else if cmd == "point" {
            let code = item.args[0].as_str();
            let operate = item.args[1].chars().next().unwrap();
            let value: i32 = item.args[1][1..].parse().unwrap();
            if let Some(pro) = game.get_pro_mut(code) {
                if operate == '+' {
                    pro.point += value;
                } else if operate == '-' {
                    pro.point -= value;
                } else if operate == '=' {
                    pro.point = value;
                }
            }
        } else if cmd == "kyokustart" {
            let new_ease_code = item.args[1].clone();
            if game.east_code == new_ease_code {
                // TODO renchan
            }
            game.east_code = new_ease_code;
            game.bon = item.args[2].parse().unwrap();
            game.richibo = item.args[3].parse().unwrap();
            game.wind = item.args[4].as_str().try_into().unwrap();
            game.reset_tiles();

            game.game_pros
                .iter_mut()
                .enumerate()
                .for_each(|(idx, pro)| {
                    pro.wind = item.args[5 + idx].as_str().try_into().unwrap();
                });
        } else if cmd == "haipai" {
            if let Some(pro) = game.get_pro_mut(item.args[0].as_str()) {
                item.args[1]
                    .as_bytes()
                    .chunks(2)
                    .map_while(|bytes| std::str::from_utf8(bytes).ok())
                    .for_each(|s| pro.tsumo(s.try_into().unwrap()));
            }
        } else if cmd == "tsumo" {
            if let Some(pro) = game.get_pro_mut(item.args[0].as_str()) {
                pro.tsumo(item.args[2].as_str().try_into().unwrap())
            }
        } else if cmd == "sutehai" {
            // TODO sutehai
        }
    }

    Ok(game)
}
