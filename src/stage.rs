use super::tetrimino;
use std::io::Write;
use termion::clear;
use termion::cursor;

const STAGE_WIDTH: usize = 10;
const STAGE_HEIGHT: usize = 20;
pub struct Position {
    top: usize,
    left: usize,
}

type Area = [[bool; STAGE_WIDTH - 1]; STAGE_HEIGHT - 1];

pub enum Tm_Direction {
    head,
    right,
    bottom,
    left,
}

pub struct Stage {
    // ステージの状態
    // テトリミノは含まない
    area: Area,
    empty_block: String,
    filled_block: String,
    // 現在操作中のテトリミノ
    tm: Option<tetrimino::Tetrimino>,
    // 現在のテトリミノの位置
    tm_position: Position,
    tm_direction: Tm_Direction,
}

impl Default for Stage {
    fn default() -> Self {
        Stage {
            area: [[false; STAGE_WIDTH - 1]; STAGE_HEIGHT - 1],
            empty_block: String::from("･"),
            filled_block: String::from("■"),
            tm_position: Position { top: 0, left: 0 },
            tm: None,
            tm_direction: Tm_Direction::head,
        }
    }
}

pub enum Rotate {
    r,
    l,
}

impl Stage {
    pub fn draw<T: Write>(&mut self, out: &mut T) {
        write!(out, "{}", clear::All);
        write!(out, "{}", cursor::Goto(1, 1));
        let mut calc_stage = self.area;
        if let Some(tm) = &self.tm {
            match &self.tm_direction {
                Tm_Direction::head => {
                    for (r, line) in tm.block.iter().enumerate() {
                        for (c, v) in line.iter().enumerate() {
                            calc_stage[r + self.tm_position.top][c + self.tm_position.left] = *v;
                        }
                    }
                }
                Tm_Direction::right => {
                    // let w = &tm.w;
                    let h = &tm.h;
                    for (t, line) in tm.block.iter().enumerate() {
                        for (l, v) in line.iter().enumerate() {
                            calc_stage[self.tm_position.top + l][self.tm_position.left + h - t] =
                                *v;
                        }
                    }
                }
                Tm_Direction::bottom => {
                    let w = &tm.w;
                    let h = &tm.h;
                    for (t, line) in tm.block.iter().enumerate() {
                        for (l, v) in line.iter().enumerate() {
                            calc_stage[self.tm_position.top + h - t]
                                [self.tm_position.left + w - l] = *v;
                        }
                    }
                }
                Tm_Direction::left => {
                    let w = &tm.w;
                    // let h = &tm.h;
                    for (t, line) in tm.block.iter().enumerate() {
                        for (l, v) in line.iter().enumerate() {
                            calc_stage[self.tm_position.top + w - l][self.tm_position.left + t] =
                                *v;
                        }
                    }
                }
            }
        }

        for line in &calc_stage {
            for i in line {
                if *i {
                    write!(out, "{}", &self.filled_block);
                } else {
                    write!(out, "{}", &self.empty_block);
                }
            }
            write!(out, "\r\n");
        }
    }
    pub fn insert_tm(&mut self, tm: tetrimino::Tetrimino) {
        self.tm = Some(tm);
        self.tm_position.top = 0;
        self.tm_position.left = 3;
    }
    pub fn move_w(&mut self, w: i32) {
        let left = *&self.tm_position.left as i32;
        if 0 <= left + w && left + w <= (STAGE_WIDTH - 1) as i32 {
            self.tm_position.left = (left + w) as usize;
        }
    }
    pub fn down(&mut self) {
        if let Some(tm) = &self.tm {
            if &self.tm_position.top + tm.h < (STAGE_HEIGHT - 1) {}
            self.tm_position.top += 1;
        }
    }
    pub fn rotate_tm(&mut self, direction: Rotate) {
        let mut deg = match self.tm_direction {
            Tm_Direction::head => 0,
            Tm_Direction::right => 1,
            Tm_Direction::bottom => 2,
            Tm_Direction::left => 3,
        };
        match direction {
            Rotate::r => {
                deg -= 1;
                if &deg == &-1 {
                    deg = 3;
                }
            }
            Rotate::l => {
                deg += 1;
                if &deg == &5 {
                    deg = 0;
                }
            }
        }
        if let Some(_tm) = &self.tm {
            match deg {
                0 => {
                    self.tm_direction = Tm_Direction::head;
                }
                1 => {
                    self.tm_direction = Tm_Direction::right;
                }
                2 => {
                    self.tm_direction = Tm_Direction::bottom;
                }
                3 => {
                    self.tm_direction = Tm_Direction::left;
                }
                _ => {
                    self.tm_direction = Tm_Direction::head;
                }
            }
        }
    }
}
