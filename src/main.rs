#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate rustty;
extern crate rand;

mod log;
mod snake;
mod snake_tui;

use snake::game::Game;

fn main() {
    snake_tui::ui::Ui::new().run();
}
