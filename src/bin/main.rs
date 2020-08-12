extern crate rus_tetris;

fn main() {
    println!("Hello, RusTetris!");
    handle_display();
}

const STAGE_WIDTH: usize = 10;
const STAGE_HEIGHT: usize = 20;
fn handle_display() {
    let block_empty = String::from("･");
    let block_filled = String::from("■");
    let mut stage: [[bool; STAGE_WIDTH - 1]; STAGE_HEIGHT - 1] =
        [[false; STAGE_WIDTH - 1]; STAGE_HEIGHT - 1];

    for line in &stage {
        for i in line {
            if *i {
                print!("{}{}{}", block_filled, block_filled, block_filled);
            } else {
                print!(" {} ", block_empty);
            }
        }
        println!("")
    }
}
