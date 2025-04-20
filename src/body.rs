use sdl3::{pixels::Color, rect::Rect, render::{Canvas, FRect, FPoint}, video::Window};
use crate::config::*;

pub struct Body{
    pub rect:FRect,
    pub color:Color,
}

impl Body {
    /*
    constructor with relative numbers for size and position
     */ 
    pub fn new_rel(rect_rel:FRect,color:Color)->Self{
        Self{
            rect:FRect {
                x: SCREEN_SIZE_W as f32*rect_rel.x,
                y: SCREEN_SIZE_H as f32*rect_rel.y,
                w: SCREEN_SIZE_W as f32*rect_rel.w,
                h: SCREEN_SIZE_H as f32*rect_rel.h 
            }, 
            color,
        }
    }
    pub fn draw(&mut self, canvas:&mut Canvas<Window>){
        canvas.set_draw_color(self.color);
        canvas.fill_rect(self.rect).unwrap();
    }
    pub fn get_rect(&mut self)->Rect{
        Rect::new(
            self.rect.x as i32,
            self.rect.y as i32,
            self.rect.w as u32,
            self.rect.h as u32
        )
    }

    pub fn center(&self) -> FPoint {
        FPoint::new(self.rect.x+self.rect.w/2.0,self.rect.y+self.rect.h/2.0)
    }

}

