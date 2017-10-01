#![allow(dead_code)]
#![allow(unused_variables)]

mod snake;

use snake::game::Game;

fn main() {
    let game = Game::new_game(50, 30);
}
