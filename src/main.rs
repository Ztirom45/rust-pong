extern crate sdl3;
use crate::game::*;

mod player;
mod config;
mod body;
mod ball;
mod livebar;
mod game;

pub fn main() {
   let mut game = Game::new(); 

  
   game.run();

}
