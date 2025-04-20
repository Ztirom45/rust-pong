use sdl3::{keyboard::Scancode, pixels::Color, render::{Canvas, FRect}, video::Window};
use crate::config::*;
use crate::body::*;

pub struct Player{ 
    pub body:Body,
    pub speed:f32,
}


impl Player{
    pub fn new(rect_rel:FRect,color:Color)->Self{
        Self { 
            body:Body::new_rel(rect_rel, color),
            speed:5.0,
        }
                
    }

    pub fn draw(&mut self, canvas:&mut Canvas<Window>) {
        self.body.draw(canvas);
    }

    pub fn handle_events(&mut self,e: &sdl3::EventPump){
        
        if (e.keyboard_state().is_scancode_pressed(Scancode::A)
        ||e.keyboard_state().is_scancode_pressed(Scancode::H))
        && self.body.rect.x >0.0{
            self.body.rect.x -= self.speed;
        }
        if (e.keyboard_state().is_scancode_pressed(Scancode::D)
        ||e.keyboard_state().is_scancode_pressed(Scancode::L))
        && self.body.rect.x < (SCREEN_SIZE_W as f32-self.body.rect.w){
            self.body.rect.x += self.speed;
        }
    }

    pub fn update(&mut self,event_pump: &sdl3::EventPump){
        self.handle_events(event_pump);
    }
}

