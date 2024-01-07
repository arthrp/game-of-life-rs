mod game_of_life;

use std::{time::Duration, thread};

use game_of_life::*;

fn main() {
    let mut g = GameOfLife::new();
    g.init();


    loop {
        println!("\x1B[2J"); //Clear screen
        g.print();
        g.make_turn();
        thread::sleep(Duration::from_millis(500));
    }
}

