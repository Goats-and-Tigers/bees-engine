// mod utils;
use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;

// #[cfg(feature = "wee_alloc")]
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn bee_log(msg: &str) {
    log(format!("[BEE_ENGINE:{line}] {msg}", line = file!(), msg = msg).as_str())
}

#[derive(Debug, Clone)]
pub enum PassiveTiles {
    Goat,
    Horse,
    Sloth,
}
#[derive(Debug, Clone)]
pub enum AggressiveTiles {
    Tiger,
    Bear,
    Snake,
    MantisShrimp,
}

#[derive(Debug, Clone)]
pub enum TilesTypes {
    P(PassiveTiles),
    A(AggressiveTiles),
    Bird,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Tile {
    Type: TilesTypes,
    color: Turn,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Square {
    id: String,
    tile: Option<Tile>,
}

#[wasm_bindgen]
pub fn gen_id(row: usize, col: usize) -> String {
    let row_name = match row + 1 {
        1 => "a",
        2 => "b",
        3 => "c",
        4 => "d",
        5 => "e",
        6 => "f",
        7 => "g",
        8 => "h",
        _ => "",
    };
    format!("{}{}", row_name, col + 1)
}

#[wasm_bindgen]
pub fn gen_tile_id(tile: Tile) -> String {
    let T = match tile.Type {
        TilesTypes::P(p) => match p {
            PassiveTiles::Goat => "g".to_string(),
            PassiveTiles::Horse => "h".to_string(),
            PassiveTiles::Sloth => "s".to_string(),
        },
        TilesTypes::Bird => "r".to_string(),
        TilesTypes::A(a) => match a {
            AggressiveTiles::Bear => "b".to_string(),
            AggressiveTiles::Tiger => "t".to_string(),
            AggressiveTiles::Snake => "l".to_string(),
            AggressiveTiles::MantisShrimp => "m".to_string(),
        },
    };
    if tile.color == Turn::Orange {
        return T.to_string().to_uppercase();
    }
    T.to_string()
}

fn expand_tile_id(tile: String) -> Tile {
    let mut t: Tile = Tile {
        Type: TilesTypes::Bird,
        color: Turn::Nil,
    };

    if tile.chars().collect::<Vec<char>>()[0].is_uppercase() {
        t.color = Turn::Orange
    } else {
        t.color = Turn::White
    }
    match tile.to_lowercase().as_str() {
        "g" => {
            t.Type = TilesTypes::P(PassiveTiles::Goat);
        }
        "h" => {
            t.Type = TilesTypes::P(PassiveTiles::Horse);
        }
        "r" => {
            t.Type = TilesTypes::Bird;
        }
        "s" => {
            t.Type = TilesTypes::P(PassiveTiles::Sloth);
        }
        "b" => {
            t.Type = TilesTypes::A(AggressiveTiles::Bear);
        }
        "t" => {
            t.Type = TilesTypes::A(AggressiveTiles::Tiger);
        }
        "l" => {
            t.Type = TilesTypes::A(AggressiveTiles::Snake);
        }
        "m" => {
            t.Type = TilesTypes::A(AggressiveTiles::MantisShrimp);
        }
        _ => bee_log("bad tile token"),
    };

    t
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SquareLoc {
    row: usize,
    col: usize,
    id: String,
}

#[wasm_bindgen]
pub fn expand_id(id: String) -> SquareLoc {
    let pieces = id.split("");
    let arr: Vec<&str> = pieces.filter(|str| str.len() != 0).collect();

    let row_name = match arr[0] {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        "e" => 4,
        "f" => 5,
        "g" => 6,
        "h" => 7,
        _ => 100,
    };

    SquareLoc {
        row: row_name,
        col: arr[1].parse::<usize>().unwrap() - 1,
        id,
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Debug, Clone)]
pub enum Turn {
    Nil,
    White,
    Orange,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Move {
    from: SquareLoc,
    to: SquareLoc,
    color: Turn,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Board {
    state: Vec<Vec<Square>>,
    turn: Turn,
    queue: Vec<Move>,
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Around {
    left: Option<SquareLoc>,
    right: Option<SquareLoc>,

    up: Option<SquareLoc>,
    down: Option<SquareLoc>,

    up_diag_left: Option<SquareLoc>,
    up_diag_right: Option<SquareLoc>,

    down_diag_left: Option<SquareLoc>,
    down_diag_right: Option<SquareLoc>,
}

impl Around {
    pub fn to_vec(&self) -> Vec<Option<SquareLoc>> {
        let mut arr: Vec<Option<SquareLoc>> = vec![];
        for i in 1..8 {
            match i {
                1 => arr.push(self.up.clone()),
                2 => arr.push(self.down.clone()),
                3 => arr.push(self.right.clone()),
                4 => arr.push(self.left.clone()),
                5 => arr.push(self.up_diag_left.clone()),
                6 => arr.push(self.up_diag_right.clone()),
                7 => arr.push(self.down_diag_left.clone()),
                8 => arr.push(self.down_diag_right.clone()),
                _ => break,
            }
        }
        return arr;
    }
}
#[wasm_bindgen]
impl Board {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Board {
        let mut new_board: Board = Board {
            state: vec![],
            turn: Turn::Nil,
            queue: vec![],
        };
        for row in 0..8 {
            new_board.state.push(vec![]);
            for col in 0..8 {
                let mut sq = Square {
                    id: "".to_string(),
                    tile: None,
                };
                sq.id = gen_id(row, col);
                new_board.state[row].push(sq);
            }
        }
        return new_board;
    }
    pub fn to_fen(&self) -> String {
        let mut fen_str = "".to_string();
        for (i, row) in self.state.clone().iter().enumerate() {
            let mut row_str: Vec<String> = vec!["".to_string()];
            for col in row {
                if col.tile.is_none() {
                    if row_str[row_str.len() - 1].parse::<u32>().is_ok() {
                        let num = row_str[row_str.len() - 1].parse::<u32>().unwrap() + 1;
                        let index = row_str.len() - 1;
                        let _ = std::mem::replace(&mut row_str[index], num.to_string());
                    } else {
                        row_str.push("1".to_string())
                    }
                } else {
                    row_str.push(gen_tile_id(col.tile.clone().unwrap()))
                }
            }
            if i == 7 {
                fen_str.push_str(&row_str.clone().join(""))
            } else {
                fen_str.push_str(&row_str.clone().join(""));
                fen_str.push_str("/")
            }
        }

        fen_str
    }

    pub fn start(&mut self) {
        self.turn = Turn::White;
        bee_log("starting game")
    }

    pub fn set_turn(&mut self, color: Turn) {
        self.turn = color;
        bee_log("change turn")
    }

    fn around(&self, sel: SquareLoc) -> Around {
        let mut around = Around {
            left: None,
            right: None,
            up: None,
            down: None,
            up_diag_left: None,
            up_diag_right: None,
            down_diag_left: None,
            down_diag_right: None,
        };
        let mut up;
        let mut down;
        let mut left;
        let mut right;
        match self.turn {
            Turn::Orange => {
                up = sel.row + 1;
                down = sel.row - 1;

                left = sel.row + 1;
                right = sel.row - 1;
            }
            Turn::White => {
                up = sel.row - 1;
                down = sel.row + 1;

                left = sel.row - 1;
                right = sel.row + 1;
            }
            Turn::Nil => {
                bee_log("bad nil color");
                return around;
            }
        }
        if self.state.get(up).is_some() {
            around.up = Some(SquareLoc {
                row: up,
                col: sel.col,
                id: gen_id(up, sel.col),
            })
        }
        if self.state.get(down).is_some() {
            around.down = Some(SquareLoc {
                row: down,
                col: sel.col,
                id: gen_id(down, sel.col),
            })
        }
        if self.state.get(sel.row).unwrap().get(left).is_some() {
            around.left = Some(SquareLoc {
                row: sel.row,
                col: left,
                id: gen_id(sel.row, left),
            })
        }
        if self.state.get(sel.row).unwrap().get(right).is_some() {
            around.right = Some(SquareLoc {
                row: sel.row,
                col: right,
                id: gen_id(sel.row, right),
            })
        }
        if self.state.get(up).is_some() && self.state.get(up).unwrap().get(left).is_some() {
            around.up_diag_left = Some(SquareLoc {
                row: up,
                col: left,
                id: gen_id(up, left),
            })
        }

        if self.state.get(up).is_some() && self.state.get(up).unwrap().get(right).is_some() {
            around.up_diag_right = Some(SquareLoc {
                row: up,
                col: right,
                id: gen_id(up, right),
            })
        }
        if self.state.get(down).is_some() && self.state.get(down).unwrap().get(left).is_some() {
            around.down_diag_left = Some(SquareLoc {
                row: down,
                col: left,
                id: gen_id(down, left),
            })
        }

        if self.state.get(down).is_some() && self.state.get(down).unwrap().get(right).is_some() {
            around.down_diag_right = Some(SquareLoc {
                row: down,
                col: right,
                id: gen_id(down, right),
            })
        }

        around
    }
    pub fn add_tile(&mut self, t: String, pos: String) {
        let tile = expand_tile_id(t);
        let pos: SquareLoc = expand_id(pos);

        if self.turn != Turn::Nil {
            bee_log("cannot add tile to started game");
            return;
        }
        self.state[pos.row][pos.col].tile = Some(tile)
    }
    fn exec(&mut self, mov: Move) -> bool {
        let from_row = mov.from.row;
        let from_col = mov.from.col;
        let from_sq: Square = self.state[from_row][from_col].clone();

        let to_row = mov.to.row;
        let to_col = mov.to.col;
        let to_sq: Square = self.state[to_row][to_col].clone();

        if from_sq.tile.is_some() {
            let around_from = self.around(mov.from.clone()).to_vec();
            let is_one_away = around_from.contains(&Some(mov.to.clone()));
            bee_log(format!("{:#?}", self.around(mov.from)).to_string().as_str());
            if !is_one_away {
                return false;
            }

            bee_log(
                format!(
                    "move {}-{}",
                    gen_id(from_row, from_col),
                    gen_id(to_row, to_col)
                )
                .as_str(),
            );
        } else {
            return false;
        }

        if self.turn == Turn::White {
            self.set_turn(Turn::Orange)
        } else {
            self.set_turn(Turn::White)
        }
        return true;
    }

    pub fn add_move(&mut self, from: String, to: String) {
        let from_loc = expand_id(from);
        let to_loc = expand_id(to);
        self.queue.push(Move {
            from: from_loc,
            to: to_loc,
            color: self.turn.clone(),
        })
    }

    fn proc(&mut self) {
        let current = self.turn.clone();
        for mov in self.queue.clone() {
            if mov.color == current {
                self.exec(mov.clone());
            }
        }
    }
    pub fn proc_moves(&mut self) {
        self.proc()
    }
}
