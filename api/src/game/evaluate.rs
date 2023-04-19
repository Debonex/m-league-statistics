use super::models::{Game, Status, Tile};
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
                gen_tiles(&item.args[1])
                    .into_iter()
                    .for_each(|tile| pro.tsumo(tile));
            }
        } else if cmd == "tsumo" {
            if let Some(pro) = game.get_pro_mut(item.args[0].as_str()) {
                pro.tsumo(item.args[2].as_str().try_into().unwrap())
            }
        } else if cmd == "sutehai" {
            if let Some(pro) = game.get_pro_mut(item.args[0].as_str()) {
                let sute_type = item.args[2].as_str();
                pro.sute(
                    &item.args[1].as_str().try_into().unwrap(),
                    sute_type == "tsumogiri",
                );

                if sute_type == "richi" {
                    pro.status = Status::Richi;
                }
            }
        } else if cmd == "dora" {
            game.dora.push(item.args[0].as_str().try_into().unwrap());
            game.dora_pointer
                .push(item.args[1].as_str().try_into().unwrap());
        } else if cmd == "say" {
            let say_type = item.args[1].as_str();
            if say_type == "chi" || say_type == "pon" || (say_type == "kan" && item.args.len() > 2)
            {
                if let Some(pro) = game.get_pro_mut(&item.args[0]) {
                    let mut tiles = gen_tiles(&item.args[1][1..item.args[1].len() - 1]);
                    tiles.push(item.args[2].as_str().try_into().unwrap());
                    pro.furo(tiles);
                }
            }
        }
    }

    Ok(game)
}

fn gen_tiles(tiles_str: &str) -> Vec<Tile> {
    let mut res = Vec::new();
    tiles_str.as_bytes().chunks(2).for_each(|bytes| {
        if let Ok(tile_str) = std::str::from_utf8(bytes) {
            if let Ok(tile) = TryInto::<Tile>::try_into(tile_str) {
                res.push(tile)
            }
        }
    });
    res
}
