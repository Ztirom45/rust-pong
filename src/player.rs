use sdl3::{keyboard::Scancode, pixels::Color, render::{Canvas, FRect}, video::Window};
use crate::config::*;
use crate::body::*;
use crate::livebar::*;

pub struct Player{ 
    pub body:Body,
    pub speed:f32,
    pub livebar:Livebar,
}


impl Player{
    pub fn new(rect_rel:FRect,color:Color)->Self{
        Self { 
            body:Body::new_rel(rect_rel, color),
            speed:PLAYER_SPEED,
            livebar:Livebar{
                lives:PLAYER_LIVES,
                max_lives:PLAYER_LIVES,
            },
        }
                
    }

    pub fn draw(&mut self, canvas:&mut Canvas<Window>) {
        self.body.draw(canvas);
        self.livebar.draw(canvas);
    }

    pub fn handle_events(&mut self,e: &sdl3::EventPump, delta: f32){
        
        if (e.keyboard_state().is_scancode_pressed(Scancode::A)
        ||e.keyboard_state().is_scancode_pressed(Scancode::H))
        && self.body.rect.x >0.0{
            self.body.rect.x -= self.speed*delta;
        }
        if (e.keyboard_state().is_scancode_pressed(Scancode::D)
        ||e.keyboard_state().is_scancode_pressed(Scancode::L))
        && self.body.rect.x < (SCREEN_SIZE_W as f32-self.body.rect.w){
            self.body.rect.x += self.speed*delta;
        }
    }

    pub fn update(&mut self,delta: f32,event_pump: &sdl3::EventPump){
        self.handle_events(event_pump,delta);
    }
}

