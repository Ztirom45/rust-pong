use sdl3::{keyboard::Scancode, pixels::Color, render::{Canvas, FRect}, video::Window};
use crate::config::*;

pub struct Livebar{
    pub lives:u8,
    pub max_lives:u8
}

impl Livebar{
    pub fn draw(&mut self, canvas:&mut Canvas<Window>){
        canvas.set_draw_color(Color::RGB(100, 100, 100));
        canvas.fill_rect(FRect{
            x:SCREEN_SIZE_W as f32 *0.8,
            y:SCREEN_SIZE_H as f32 *0.1,
            w:SCREEN_SIZE_W as f32 *0.15,
            h:SCREEN_SIZE_H as f32 *0.02,
        }).unwrap(); 
        if self.lives > 0{
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            canvas.fill_rect(FRect{
                x:SCREEN_SIZE_W as f32 *0.8,
                y:SCREEN_SIZE_H as f32 *0.1,
                w:SCREEN_SIZE_W as f32 *0.15 * self.lives as f32/self.max_lives as f32,
                h:SCREEN_SIZE_H as f32 *0.02,
        }).unwrap();
        }
    }
}
