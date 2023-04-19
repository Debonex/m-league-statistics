use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Game {
    pub id: String,
    pub start_time: String,
    pub east_code: String,
    pub wind: Wind,
    pub bon: u32,
    pub richibo: u32,
    pub game_pros: Vec<GamePro>,
    pub dora: Vec<Tile>,
    pub dora_pointer: Vec<Tile>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            id: "".to_string(),
            start_time: "".to_string(),
            east_code: "".to_string(),
            wind: Wind::East,
            bon: 0,
            richibo: 0,
            game_pros: Vec::new(),
            dora: Vec::with_capacity(4),
            dora_pointer: Vec::with_capacity(4),
        }
    }

    pub fn get_pro_mut(&mut self, code: &str) -> Option<&mut GamePro> {
        self.game_pros.iter_mut().find(|pro| pro.code == code)
    }

    pub fn reset_tiles(&mut self) {
        self.dora.clear();
        self.dora_pointer.clear();
        self.game_pros.iter_mut().for_each(|pro| pro.reset_tiles())
    }
}

#[derive(Debug)]
pub struct GamePro {
    pub name: String,
    pub team_code: String,
    pub code: String,
    pub point: i32,
    pub wind: Wind,
    pub status: Status,
    pub tiles: Tiles,
    pub kyoku_infos: Vec<ProKyokuInfo>,
}

impl GamePro {
    pub fn new(name: String, team_code: String, code: String) -> Self {
        GamePro {
            name,
            team_code,
            code,
            wind: Wind::East,
            point: 0,
            status: Status::Menzen,
            tiles: Tiles::new(),
            kyoku_infos: Vec::new(),
        }
    }

    pub fn reset_tiles(&mut self) {
        self.status = Status::Menzen;
        self.tiles.reset();
    }

    pub fn tsumo(&mut self, tile: Tile) {
        self.tiles.tsumo(tile)
    }

    pub fn sute(&mut self, tile: &Tile, tsumogiri: bool) {
        self.tiles.sute(tile, tsumogiri)
    }

    pub fn furo(&mut self, tiles: Vec<Tile>) {
        self.tiles.furo(tiles);
        self.status = Status::Furo
    }
}

#[derive(Debug)]
pub struct ProKyokuInfo {
    pub agari: Option<Agari>,
    pub houjuu: Option<Houjuu>,
    pub wind: Wind,
    pub status: Status,
    pub haipai: Vec<Tile>,
}

#[derive(Debug)]
pub struct Agari {
    pub hai: Tiles,
    pub yaku: Vec<String>,
    pub point: i32,
    pub status: Status,
    pub agari_type: AgariType,
}

#[derive(Debug)]
pub struct Houjuu {
    pub hai: Tiles,
    pub yaku: Vec<String>,
    pub point: i32,
    pub status: Status,
}

#[derive(Debug)]
pub enum Status {
    Menzen,
    Furo,
    Richi,
}

#[derive(Debug)]
pub enum AgariType {
    Ron,
    Tsumo,
}

/// # Wind
#[derive(Debug)]
pub enum Wind {
    East,
    West,
    South,
    North,
}

impl TryFrom<&str> for Wind {
    type Error = TileError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "1z" | "1Z" => Ok(Wind::East),
            "2z" | "2Z" => Ok(Wind::South),
            "3z" | "3Z" => Ok(Wind::West),
            "4z" | "4Z" => Ok(Wind::North),
            _ => Err(TileError {}),
        }
    }
}

#[derive(Debug)]
pub struct Tiles {
    pub sute: Vec<SuteTile>,
    pub tehai: Vec<Tile>,
    pub furo: Vec<Vec<Tile>>,
}

impl Tiles {
    pub fn new() -> Self {
        Tiles {
            sute: Vec::with_capacity(24),
            tehai: Vec::with_capacity(14),
            furo: Vec::with_capacity(4),
        }
    }

    pub fn reset(&mut self) {
        self.sute.clear();
        self.tehai.clear();
        self.furo.clear();
    }

    pub fn tsumo(&mut self, tile: Tile) {
        self.tehai.push(tile)
    }

    fn sute(&mut self, tile: &Tile, tsumogiri: bool) {
        if let Some(index) = self.tehai.iter().position(|tehai_tile| tehai_tile.eq(tile)) {
            let removed_tile = self.tehai.remove(index);
            self.sute.push(SuteTile {
                tsumogiri,
                tile: removed_tile,
            })
        }
    }

    fn furo(&mut self, tiles: Vec<Tile>) {
        self.furo.push(tiles)
    }

    pub fn count_dora(&self, dora: &[Tile]) -> usize {
        let mut count = 0;
        for tile in self.tehai.iter() {
            if matches!(tile, Tile::M(0) | Tile::P(0) | Tile::S(0)) {
                count += 1;
            }
            count += dora.iter().filter(|dora_tile| dora_tile.eq(&tile)).count();
        }
        count
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Tile {
    Z(u8),
    M(u8),
    S(u8),
    P(u8),
}

impl TryFrom<&str> for Tile {
    type Error = TileError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(TileError {});
        }
        let n: u8 = value[0..1].parse().map_err(|_| TileError {})?;
        let t = &value[1..];
        match (n, t) {
            (1..=9, "s") => Ok(Tile::S(n)),
            (1..=9, "m") => Ok(Tile::M(n)),
            (1..=9, "p") => Ok(Tile::P(n)),
            (5, "S") => Ok(Tile::S(0)),
            (5, "M") => Ok(Tile::M(0)),
            (5, "P") => Ok(Tile::P(0)),
            (1..=7, "z") => Ok(Tile::Z(n)),
            _ => Err(TileError {}),
        }
    }
}

#[derive(Debug)]
pub struct TileError {}

impl Error for TileError {}

impl Display for TileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid tile value.")
    }
}

#[derive(Debug)]
pub struct SuteTile {
    tsumogiri: bool,
    tile: Tile,
}
