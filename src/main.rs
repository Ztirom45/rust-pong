extern crate sdl3;

use ball::Ball;
use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::render::FRect;
use std::time::Duration;

use crate::player::*;
use crate::config::*;
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
