extern crate rus_tetris;
use rus_tetris::tetrimino;

use std::io::{stdin, stdout, Write};
use termion::clear;
use termion::cursor;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
// use termion::screen::AlternateScreen;

const STAGE_WIDTH: usize = 10;
const STAGE_HEIGHT: usize = 20;
struct Cursor {
    row: usize,
    column: usize,
}

type Stage = [[bool; STAGE_WIDTH - 1]; STAGE_HEIGHT - 1];

struct Display {
    // ステージの状態
    stage: Stage,
    empty_block: String,
    filled_block: String,
    // 現在のカーソルの位置
    cursor: Cursor,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            stage: [[false; STAGE_WIDTH - 1]; STAGE_HEIGHT - 1],
            empty_block: String::from("･"),
            filled_block: String::from("■"),
            cursor: Cursor { row: 0, column: 0 },
        }
    }
}

impl Display {
    fn draw<T: Write>(&self, out: &mut T) {
        write!(out, "{}", clear::All);
        write!(out, "{}", cursor::Goto(1, 1));
        for line in &self.stage {
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
    fn insert_tm(&mut self, tm: tetrimino::Tetrimino) {
        // TODO:計算の前に描画可能か判定
        self.cursor.row = 0;
        self.cursor.column = 3;
        let mut new_stage = self.stage;
        for (r, line) in tm.block.iter().enumerate() {
            for (c, v) in line.iter().enumerate() {
                new_stage[r + self.cursor.row][c + self.cursor.column] = *v;
            }
        }
        self.stage = new_stage;
    }
}

fn main() {
    let mut state = Display::default();
    let stdin = stdin();
    // let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    let mut stdout = stdout().into_raw_mode().unwrap();
    state.draw(&mut stdout);
    state.insert_tm(tetrimino::Tetrimino::j_tetrimino());
    state.draw(&mut stdout);

    for evt in stdin.events() {
        let evt = evt.unwrap();

        match evt {
            Event::Key(Key::Char('u')) => {
                write!(stdout, "kaiten-h");
            }
            Event::Key(Key::Char('i')) => {
                write!(stdout, "kaiten-m");
            }
            Event::Key(Key::Char('j')) => {
                write!(stdout, "sita");
            }
            Event::Key(Key::Char('k')) => {
                write!(stdout, "ue");
            }
            Event::Key(Key::Char('l')) => {
                write!(stdout, "migi");
            }
            Event::Key(Key::Char('h')) => {
                write!(stdout, "hidari");
            }
            // Ctrl-cで終了
            Event::Key(Key::Ctrl('c')) => {
                return;
            }
            _ => {}
        }
    }
}