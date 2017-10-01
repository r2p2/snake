#![allow(dead_code)]
#![allow(unused_variables)]
mod game;
mod snake;
mod pos;
mod dir;

use game::Game;

fn main() {
    let game = Game::new_game(50, 30);
}
