use sdl3::{pixels::Color, render::{Canvas, FRect}, video::Window};
use crate::config::*;
use crate::body::*;

pub struct Ball{ 
    pub body:Body,
    pub speed_x:f32,
    pub speed_y:f32,
}

impl Ball{
    pub fn new(rect_rel:FRect,color:Color)->Self{
        Self { 
            body:Body::new_rel(rect_rel, color),
            speed_x:-5.0,
            speed_y:5.0,
        }
                
    }

    pub fn draw(&mut self, canvas:&mut Canvas<Window>) {
        self.body.draw(canvas);
    }

    pub fn update(&mut self){
        self.body.rect.x += self.speed_x;
        self.body.rect.y += self.speed_y;
        //screen edge collision
        if self.body.rect.x>=SCREEN_SIZE_W as f32 -self.body.rect.w as f32{
            self.speed_x=-self.speed_x;
        }
        if self.body.rect.x<=0 as f32{
            self.speed_x=-self.speed_x;
        }
        if self.body.rect.y>=SCREEN_SIZE_H as f32-self.body.rect.h as f32{
            self.speed_y=-self.speed_y;
        }
        if self.body.rect.y<=0 as f32{
            self.speed_y=-self.speed_y;
        }

        
    }
}

