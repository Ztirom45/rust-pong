extern crate sdl3;

use ball::Ball;
use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::render::FRect;
use std::time::Duration;

use crate::player::*;
use crate::config::*;

mod player;
mod config;
mod body;
mod ball;

pub fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl3 demo", SCREEN_SIZE_W, SCREEN_SIZE_H)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut player:Player = Player::new(FRect{x:0.42,y:0.9,w:0.16,h:0.05},Color::RGB(255, 0, 0));
    let mut ball:Ball = Ball::new(FRect{x:0.47,y:0.47,w:0.06,h:0.06},Color::RGB(0, 100, 100));

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        player.handle_events(&event_pump);    
        ball.update();
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
    
        player.draw(&mut canvas);
        ball.draw(&mut canvas);
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
