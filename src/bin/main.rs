extern crate rus_tetris;
use rus_tetris::stage;
use rus_tetris::tetrimino;

use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

fn main() {
    let mut stage = stage::Stage::default();
    let stdin = stdin();
    let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    stage.insert_tm(tetrimino::Tetrimino::l_tetrimino());
    stage.draw(&mut stdout);

    for evt in stdin.events() {
        let evt = evt.unwrap();

        match evt {
            Event::Key(Key::Char('u')) => {
                stage.rotate_tm(stage::Rotate::r);
            }
            Event::Key(Key::Char('i')) => {
                stage.rotate_tm(stage::Rotate::l);
            }
            Event::Key(Key::Char('j')) => {
                stage.down();
            }
            Event::Key(Key::Char('k')) => {
                write!(stdout, "ue");
            }
            Event::Key(Key::Char('l')) => {
                stage.move_w(1);
            }
            Event::Key(Key::Char('h')) => {
                stage.move_w(-1);
            }
            // Ctrl-cで終了
            Event::Key(Key::Ctrl('c')) => {
                return;
            }
            _ => {}
        }
        stage.draw(&mut stdout);
    }
}
