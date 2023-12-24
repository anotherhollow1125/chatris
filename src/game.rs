use std::collections::VecDeque;

const NEXTS: u32 = 3;
// const SAFTY_FRAMES: u32 = 90;
// const INTERVAL: u32 = 20;
// const LEVEL_UP_LINE_NUM: u32 = 10;
// const MAX_GRAVITY_RECIP: u32 = 5;
// const SPEEDUP_RATE: f32 = 0.9;

const SCORE_TABLE: [u32; 5] = [0, 100, 300, 600, 900];

// const WHITE : [f32; 4] = [1.0 , 1.0 , 1.0 , 1.0];
// const GRAY  : [f32; 4] = [0.3 , 0.3 , 0.3 , 1.0];
// const TRANS : [f32; 4] = [0.0 , 0.0 , 0.0 , 0.0];
// const CYAN  : [f32; 4] = [0.0 , 1.0 , 1.0 , 1.0];
// const YELLOW: [f32; 4] = [1.0 , 1.0 , 0.0 , 1.0];
// const LIME  : [f32; 4] = [0.0 , 1.0 , 0.0 , 1.0];
// const RED   : [f32; 4] = [1.0 , 0.0 , 0.0 , 1.0];
// const BLUE  : [f32; 4] = [0.0 , 0.0 , 1.0 , 1.0];
// const ORANGE: [f32; 4] = [1.0 , 0.65, 0.0 , 1.0];
// const PURPLE: [f32; 4] = [0.5 , 0.0 , 0.5 , 1.0];

#[derive(Debug, Copy, Clone)]
pub enum MinoColors {
    WHITE,
    GRAY,
    TRANS,
    CYAN,
    YELLOW,
    LIME,
    RED,
    BLUE,
    ORANGE,
    PURPLE,
}

use MinoColors::*;

impl MinoColors {
    pub fn from_char(c: &char) -> Self {
        match c {
            'I' => CYAN,
            'O' => YELLOW,
            'S' => LIME,
            'Z' => RED,
            'J' => BLUE,
            'L' => ORANGE,
            'T' => PURPLE,
            '@' => GRAY,
            _ => WHITE,
        }
    }

    pub fn to_rgb(&self) -> [f32; 4] {
        match self {
            WHITE => [1.0, 1.0, 1.0, 1.0],
            GRAY => [0.3, 0.3, 0.3, 1.0],
            TRANS => [0.0, 0.0, 0.0, 0.0],
            CYAN => [0.0, 1.0, 1.0, 1.0],
            YELLOW => [1.0, 1.0, 0.0, 1.0],
            LIME => [0.0, 1.0, 0.0, 1.0],
            RED => [1.0, 0.0, 0.0, 1.0],
            BLUE => [0.0, 0.0, 1.0, 1.0],
            ORANGE => [1.0, 0.65, 0.0, 1.0],
            PURPLE => [0.5, 0.0, 0.5, 1.0],
        }
    }
}

use std::fmt::{self, Display, Formatter};

impl Display for MinoColors {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let c = match &self {
            CYAN => "I",
            YELLOW => "O",
            LIME => "S",
            RED => "Z",
            BLUE => "J",
            ORANGE => "L",
            PURPLE => "T",
            GRAY => "@",
            _ => ".",
        };
        write!(f, "{}", c)
    }
}

pub struct Mino {
    name: &'static str,
    shape: [[bool; 4]; 4],
    size: usize,
    color: MinoColors,
}

pub static D: Mino = Mino {
    name: "D",
    shape: [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    size: 4,
    color: TRANS,
};

pub static I: Mino = Mino {
    name: "I",
    shape: [
        [false, false, false, false],
        [true, true, true, true],
        [false, false, false, false],
        [false, false, false, false],
    ],
    size: 4,
    color: CYAN,
};
pub static O: Mino = Mino {
    name: "O",
    shape: [
        [true, true, false, false],
        [true, true, false, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    size: 2,
    color: YELLOW,
};
pub static S: Mino = Mino {
    name: "S",
    shape: [
        [false, true, true, false],
        [true, true, false, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    size: 3,
    color: LIME,
};
pub static Z: Mino = Mino {
    name: "Z",
    shape: [
        [true, true, false, false],
        [false, true, true, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    size: 3,
    color: RED,
};
pub static J: Mino = Mino {
    name: "J",
    shape: [
        [true, false, false, false],
        [true, true, true, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    size: 3,
    color: BLUE,
};
pub static L: Mino = Mino {
    name: "L",
    shape: [
        [false, false, true, false],
        [true, true, true, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    size: 3,
    color: ORANGE,
};
pub static T: Mino = Mino {
    name: "T",
    shape: [
        [false, true, false, false],
        [true, true, true, false],
        [false, false, false, false],
        [false, false, false, false],
    ],
    size: 3,
    color: PURPLE,
};

impl Mino {
    pub fn from_char(c: &char) -> Option<&'static Self> {
        match c {
            'I' => Some(&I),
            'O' => Some(&O),
            'S' => Some(&S),
            'Z' => Some(&Z),
            'J' => Some(&J),
            'L' => Some(&L),
            'T' => Some(&T),
            _ => None,
        }
    }

    pub fn get_color(&self) -> MinoColors {
        self.color
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Block {
    filled: bool,
    color: MinoColors,
}

impl Block {
    pub fn new(filled: bool, color: MinoColors) -> Block {
        Block { filled, color }
    }

    pub fn is_filled(&self) -> bool {
        self.filled
    }

    pub fn get_color(&self) -> MinoColors {
        self.color
    }

    pub fn from_char(c: &char) -> Self {
        match c {
            'I' => Self {
                color: CYAN,
                filled: true,
            },
            'O' => Self {
                color: YELLOW,
                filled: true,
            },
            'S' => Self {
                color: LIME,
                filled: true,
            },
            'Z' => Self {
                color: RED,
                filled: true,
            },
            'J' => Self {
                color: BLUE,
                filled: true,
            },
            'L' => Self {
                color: ORANGE,
                filled: true,
            },
            'T' => Self {
                color: PURPLE,
                filled: true,
            },
            '@' => Self {
                color: GRAY,
                filled: true,
            },
            _ => Self {
                color: WHITE,
                filled: false,
            },
        }
    }
}

#[derive(Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn next(&self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }

    fn pre(&self) -> Dir {
        match self {
            Dir::North => Dir::West,
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
        }
    }

    // SRS
    // url: https://tetrisch.github.io/main/srs.html
    fn next_sequences(&self, mino: &'static Mino) -> Vec<(i32, i32)> {
        match mino.size {
            4 => match self {
                Dir::East => vec![(0, -2), (0, 1), (1, -2), (-2, 1)],
                Dir::South => vec![(0, -1), (0, 2), (-2, -1), (1, 2)],
                Dir::West => vec![(0, 2), (0, -1), (-1, 2), (2, -1)],
                Dir::North => vec![(0, -2), (0, 1), (2, 1), (-1, -2)],
            },
            _ => {
                let d = match self {
                    Dir::East | Dir::South => 1,
                    _ => -1,
                };
                match self {
                    Dir::East | Dir::West => vec![(0, -d), (-1, -d), (2, 0), (2, -d)],
                    _ => vec![(0, d), (1, d), (-2, 0), (-2, d)],
                }
            }
        }
    }

    fn pre_sequences(&self, mino: &'static Mino) -> Vec<(i32, i32)> {
        match mino.size {
            4 => match self {
                Dir::West => vec![(0, -1), (0, 2), (-2, -1), (1, 2)],
                Dir::North => vec![(0, 2), (0, -1), (-1, 2), (2, -1)],
                Dir::East => vec![(0, 1), (0, -2), (2, 1), (-1, -2)],
                Dir::South => vec![(0, 1), (0, -2), (1, -2), (-2, 1)],
            },
            _ => {
                let d = match self {
                    Dir::West | Dir::North => 1,
                    _ => -1,
                };
                match self {
                    Dir::West | Dir::East => vec![(0, d), (-1, d), (2, 0), (2, d)],
                    _ => vec![(0, d), (1, d), (-2, 0), (-2, d)],
                }
            }
        }
    }
}

struct ControlledMino {
    pos: (i32, i32),
    mino: &'static Mino,
    dir: Dir,
}

pub type FieldArray = [[Block; 10]; 21];

impl ControlledMino {
    fn new(mino: &'static Mino) -> ControlledMino {
        ControlledMino {
            pos: (0, 4),
            mino: mino,
            dir: Dir::North,
        }
    }

    fn rend_mino(&self) -> Vec<Vec<bool>> {
        let size = self.mino.size;
        if size < 1 {
            return vec![];
        }
        let shape: [[bool; 4]; 4] = self.mino.shape;
        let mut method: Box<dyn FnMut(usize, usize) -> bool> = match self.dir {
            Dir::North => Box::new(|i, j| shape[i][j]),
            Dir::East => Box::new(|i, j| shape[size - 1 - j][i]),
            Dir::South => Box::new(|i, j| shape[size - 1 - i][size - 1 - j]),
            Dir::West => Box::new(|i, j| shape[j][size - 1 - i]),
        };
        (0..size)
            .map(|i| (0..size).map(|j| method(i, j)).collect())
            .collect()
    }

    fn pos_verify(&mut self, field: &FieldArray) -> bool {
        let r = self.rend_mino();
        let height = field.len() as i32;
        let width = field[0].len() as i32;

        // enumerateで書き直すところから。寝ます。

        for (i, row) in r.iter().enumerate() {
            for (j, &filled) in row.iter().enumerate() {
                if !filled {
                    continue;
                }

                let (sx, sy) = self.pos;
                // 範囲チェック
                let (x, y) = ((i as i32 + sx) as i32, (j as i32 + sy) as i32);
                if x < 0 || height <= x || y < 0 || width <= y {
                    return false;
                }
                let (x, y) = (x as usize, y as usize);
                // 設置可能チェック
                if field[x][y].filled {
                    return false;
                }
            }
        }
        true
    }

    fn move_mino(&mut self, field: &FieldArray, i: i32, j: i32) -> bool {
        let pre = self.pos;
        self.pos = (self.pos.0 + i, self.pos.1 + j);
        if !self.pos_verify(field) {
            self.pos = pre;
            return false;
        }
        true
    }

    fn spin_right(&mut self, field: &FieldArray) {
        self.dir = self.dir.next();
        let mino = self.mino;
        if !self.pos_verify(field)
            && self
                .dir
                .next_sequences(mino)
                .iter()
                .all(|(i, j)| !self.move_mino(field, *i, *j))
        {
            self.dir = self.dir.pre();
        }
    }

    fn spin_left(&mut self, field: &FieldArray) {
        self.dir = self.dir.pre();
        let mino = self.mino;
        if !self.pos_verify(field)
            && self
                .dir
                .pre_sequences(mino)
                .iter()
                .all(|(i, j)| !self.move_mino(field, *i, *j))
        {
            self.dir = self.dir.next();
        }
    }
}

pub trait MinoGenerator {
    fn generate(&mut self) -> &'static Mino;
}

pub struct WorldRuleMinoGenerator {
    dropped: Vec<(usize, u32)>,
    count: u32,
    rand_gen: Box<dyn FnMut() -> u32>,
}

impl WorldRuleMinoGenerator {
    pub fn new(rand_gen: Box<dyn FnMut() -> u32>) -> Self {
        let dropped = (0..7).map(|i| (i as usize, 0)).collect();

        Self {
            dropped,
            count: 0,
            rand_gen,
        }
    }
}

impl MinoGenerator for WorldRuleMinoGenerator {
    fn generate(&mut self) -> &'static Mino {
        if self.dropped.iter().all(|m| m.1 >= self.count) {
            self.count += 1
        };
        let cands = self
            .dropped
            .iter()
            .filter(|m| m.1 < self.count)
            .collect::<Vec<_>>();
        let m = cands[((self.rand_gen)() % (cands.len() as u32)) as usize];
        let i: usize = m.0;
        self.dropped[i].1 += 1;
        [&I, &O, &S, &Z, &J, &L, &T][i]
    }
}

enum Hold {
    Holding(&'static Mino),
    None,
}

pub struct Game {
    field: FieldArray,
    contmino: ControlledMino,
    holdmino: Hold,
    holdflag: bool,
    nextminos: VecDeque<&'static Mino>,
    minogen: Box<dyn MinoGenerator>,

    // States
    score: u32,
    clearlines: u32,
    game_over: bool,

    // for Display
    memo: String,
}

#[derive(Clone, Copy, Debug)]
pub enum InputKey {
    RSpin,
    LSpin,
    Down,
    HardDrop,
    Right,
    Left,
    Hold,
    Commit,
}
use InputKey::*;

impl Game {
    pub fn new<MG>(mut minogen: MG, memo: String) -> Game
    where
        MG: MinoGenerator + 'static,
    {
        let field: FieldArray = [[Block::new(false, TRANS); 10]; 21];

        let mut nextminos = VecDeque::new();
        for _ in 0..(NEXTS + 1) {
            nextminos.push_front(minogen.generate());
        }

        Game {
            field,
            contmino: ControlledMino::new(nextminos.pop_back().unwrap()),
            holdmino: Hold::None,
            holdflag: false,
            nextminos,
            minogen: Box::new(minogen),

            score: 0,
            clearlines: 0,
            game_over: false,

            memo,
        }
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn get_clearlines(&self) -> u32 {
        self.clearlines
    }

    pub fn get_contmino(&self) -> &'static Mino {
        self.contmino.mino
    }

    pub fn is_gameover(&self) -> bool {
        self.game_over
    }

    pub fn can_use_hold(&self) -> bool {
        !self.holdflag
    }

    pub fn rend_field(&self) -> FieldArray {
        let mut res = self.field.clone();

        let (x, y) = self.contmino.pos;
        let r = self.contmino.rend_mino();
        for (i, row) in r.iter().enumerate() {
            for (j, filled) in row.iter().enumerate() {
                if !filled {
                    continue;
                }
                let (i, j) = (i as i32, j as i32);
                let (x, y) = ((i + x) as usize, (j + y) as usize);
                res[x][y] = Block::new(true, GRAY);
            }
        }

        res
    }

    // 固定と落下開始にワンクッション入れるために分離
    fn drop_start(&mut self, mino: &'static Mino) -> bool {
        self.contmino = ControlledMino::new(mino);
        self.contmino.pos_verify(&self.field)
    }

    fn lines_clear(&mut self) {
        let mut cls: usize = 0;
        let height = self.field.len();

        for i in 0..height {
            let line_clear = self.field[i].iter().all(|b| b.filled);
            if line_clear {
                cls += 1;
                for j in (1..(i + 1)).rev() {
                    self.field[j] = self.field[j - 1];
                }
                let new_line = [Block::new(false, TRANS); 10];
                self.field[0] = new_line;
            }
        }
        self.score += SCORE_TABLE[if cls > 4 { 4 } else { cls }];
        self.clearlines += cls as u32;
        let mino = self.nextminos.pop_back().unwrap();
        self.game_over = !self.drop_start(mino);
        self.holdflag = false;
        self.nextminos.push_front(self.minogen.generate());
    }

    fn fix_mino(&mut self) {
        if self.contmino.move_mino(&self.field, 1, 0) {
            self.contmino.move_mino(&self.field, -1, 0);
            return;
        }

        let mino = self.contmino.mino;
        let size = mino.size;
        let (x, y) = self.contmino.pos;
        let r = self.contmino.rend_mino();
        for i in 0..size {
            for j in 0..size {
                if r[i][j] {
                    let (x, y) = ((i as i32 + x) as usize, (j as i32 + y) as usize);
                    self.field[x][y] = Block::new(true, mino.color);
                }
            }
        }
        // レンダリング回避のためにダミーミノを挿入
        // バグの原因かもしれない...
        self.contmino = ControlledMino::new(&D);
        self.lines_clear();
    }

    fn mino_down_with_score(&mut self) -> bool {
        let res = self.contmino.move_mino(&self.field, 1, 0);
        if res {
            self.score += 1
        }
        res
    }

    pub fn handle_input(&mut self, input_key: &InputKey) -> bool {
        if self.game_over {
            return false;
        }

        match input_key {
            &RSpin => self.contmino.spin_right(&self.field),
            &LSpin => self.contmino.spin_left(&self.field),
            &Down => {
                self.mino_down_with_score();
            }
            &HardDrop => while self.mino_down_with_score() {},
            &Right => {
                self.contmino.move_mino(&self.field, 0, 1);
            }
            &Left => {
                self.contmino.move_mino(&self.field, 0, -1);
            }
            &Hold => {
                self.hold();
            }
            &Commit => {
                while self.mino_down_with_score() {}
                self.fix_mino();
            }
        }

        !self.game_over
    }

    fn hold(&mut self) {
        if self.holdflag {
            return;
        }

        let mino = match self.holdmino {
            Hold::Holding(mino) => mino,
            Hold::None => {
                self.nextminos.push_front(self.minogen.generate());
                self.nextminos.pop_back().unwrap()
            }
        };
        self.holdmino = Hold::Holding(self.contmino.mino);
        self.game_over = !self.drop_start(mino);
        self.holdflag = true;
    }

    pub fn set_field(&mut self, new_field: FieldArray) {
        self.field = new_field;
    }

    pub fn set_minogen(&mut self, mut new_minogen: Box<dyn MinoGenerator>) {
        let mut nextminos = VecDeque::new();
        for _ in 0..(NEXTS + 1) {
            nextminos.push_front((*new_minogen).generate());
        }
        self.contmino = ControlledMino::new(nextminos.pop_back().unwrap());
        self.nextminos = nextminos;
        self.minogen = new_minogen;
    }
}

impl Game {
    pub fn field_info(&self) -> String {
        let hold = match self.holdmino {
            Hold::Holding(mino) => mino.name,
            Hold::None => "-",
        };

        let current = self.contmino.mino.name;

        let next = self
            .nextminos
            .iter()
            .rev()
            .map(|mino| mino.name)
            .collect::<Vec<_>>()
            .join(" ");

        let memo = &self.memo;
        let score = self.score;
        let lines = self.clearlines;

        let gameover_str = if self.game_over { "Game Over!" } else { "" };

        format!(
            "\
Current: {}
Hold: {}
Next: {}

{}
Score: {}
Lines: {}
feedback: {}",
            current, hold, next, memo, score, lines, gameover_str
        )
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let field_string = self
            .rend_field()
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let row_s = row
                    .iter()
                    .map(|b| b.color.to_string())
                    .collect::<Vec<_>>()
                    .join("");
                format!("{:>2}{},", i, row_s)
            })
            .collect::<Vec<_>>()
            .join("\n");

        let info = self.field_info();

        write!(
            f,
            "\
{}
==========
{}",
            field_string, info
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{Game, WorldRuleMinoGenerator};
    use rand::{thread_rng, Rng};

    #[test]
    fn new_game() {
        let mut rng = thread_rng();
        let rand_gen = Box::new(move || rng.gen::<u32>());
        let minogen = WorldRuleMinoGenerator::new(rand_gen);
        let _ = Game::new(minogen, "".to_string());
    }
}
