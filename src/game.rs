use sdl3::pixels::Color;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::render::Canvas;
use sdl3::render::FRect;
use sdl3::video::Window;
use sdl3::EventPump;
use sdl3::Sdl;
use sdl3::VideoSubsystem;
use std::time::Duration;

use crate::player::*;
use crate::config::*;
use crate::ball::*;

pub struct Game{
    pub sdl_context:Sdl,
    pub video_subsystem:VideoSubsystem,
    pub canvas:Canvas<Window>,
    pub event_pump:EventPump,
    pub player:Player,
    pub ball:Ball, 
}

impl Game{
    pub fn new()->Self{
        let sdl_context = sdl3::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem.window("rust-pong", SCREEN_SIZE_W, SCREEN_SIZE_H)
        .position_centered()
        .build()
        .unwrap();

        let canvas = window.into_canvas();

        let event_pump = sdl_context.event_pump().unwrap();

        let player:Player = Player::new(FRect{x:0.42,y:0.9,w:0.16,h:0.05},Color::RGB(0xFF, 0xFF, 0xFF));
        let ball:Ball = Ball::new(FRect{x:0.47,y:0.47,w:0.06,h:0.06},Color::RGB(0xFF, 0xFF, 0xFF));

        Self{
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            player,
            ball, 
        }
    }
    
    pub fn update(&mut self){
        self.player.update(&self.event_pump);    
        self.ball.update(&mut self.player); 
    }

    pub fn draw(&mut self){
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.player.draw(&mut self.canvas);
        self.ball.draw(&mut self.canvas);
        self.canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));       

    }

    pub fn run(&mut self){
     'running: loop {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        self.update();
        self.draw();

     }
    }
}
